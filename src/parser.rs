use std::{
    collections::HashMap,
    fs,
    path::Path, convert::TryInto,
};

use anyhow::{Context, Result};
use log::info;
use pest::{
    error::{Error, ErrorVariant},
    iterators::Pair,
    Parser,
};
use relative_path::RelativePathBuf;

use crate::OpenControllerLib::{
    CallExpr, ControllerExpr, DeviceExpr, DisplayInterfaceExpr, Elif, Expr, HouseExpr, IfExpr,
    LambdaExpr, Module, RefExpr, RoomExpr, WidgetExpr, Position,
};

/// A trait for to convert to a pest error
pub trait PositionalError<T> {
    /// Converts error to a pest error
    fn pos_err<M>(self, message: M, pair: &Pair<Rule>) -> Result<T, Error<()>>
    where
        M: ToString;
}

impl<T, E> PositionalError<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    /// Converts result error to a pest error
    fn pos_err<M>(self, message: M, pair: &Pair<Rule>) -> Result<T, Error<()>>
    where
        M: ToString,
    {
        self.map_err(|_error| {
            Error::new_from_span(
                ErrorVariant::<()>::CustomError {
                    message: message.to_string(),
                },
                pair.as_span(),
            )
        })
    }
}

impl<T> PositionalError<T> for Option<T> {
    /// Converts option error to a pest error
    fn pos_err<M>(self, message: M, pair: &Pair<Rule>) -> Result<T, Error<()>>
    where
        M: ToString,
    {
        self.ok_or_else(|| {
            Error::new_from_span(
                ErrorVariant::<()>::CustomError {
                    message: message.to_string(),
                },
                pair.as_span(),
            )
        })
    }
}

#[derive(Parser)]
#[grammar = "oc.pest"]
pub struct OCParser;

/// Removes quotes around parsed string
fn trim_string(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

/// Parses a module file
pub fn parse_module<P>(input_file: P) -> Result<Module>
where
    P: AsRef<Path>,
{
    info!("Parsing module {:#?}", input_file.as_ref().canonicalize()?);
    // Load file
    let unparsed_file = fs::read_to_string(&input_file).context("Couldn't read file")?;
    let file = OCParser::parse(Rule::module, &unparsed_file)
        .context("Couldn't parse")? // unwrap the parse result
        .next()
        .context("Expected module")?;

    // Parsed result
    let mut module = Module::new();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::import => {
                let mut inner_rules = line.clone().into_inner();
                let path = inner_rules.next().pos_err("Expected path", &line)?.as_str();
                let name = inner_rules.next().pos_err("Expected name", &line)?.as_str();
                // Add parsed imported module to imports
                module.imports.insert(
                    name.to_owned(),
                    parse_module(
                        RelativePathBuf::from(trim_string(path))
                            .to_path(input_file.as_ref().parent().context("Input file needs parent")?),
                    )?,
                );
            }
            Rule::expr => {
                // TODO: If already body
                module.body = Some(parse_expr(line)?).into();
            }
            // Do nothing at end of file
            Rule::EOI => (),
            _ => unreachable!("Expected import, expr, or EOI"),
        }
    }
    Ok(module)
}

/// Parses a widget rule
fn parse_widget(rule: Pair<Rule>) -> Result<WidgetExpr> {
    let mut widget = WidgetExpr::new();
    let mut widget_inner = rule.clone().into_inner();
    // Get widget tag
    let tag = widget_inner.next().pos_err("Expected tag", &rule)?.as_str();
    widget.set_widget_type(tag.to_owned());
    // Get all params in widget inner
    while match widget_inner
        .peek()
        .pos_err("Expected something in widget", &rule)?
        .as_rule()
    {
        // Match while is xml param
        Rule::xml_param => true,
        _ => false,
    } {
        let mut xml_param_inner = widget_inner
            .next()
            .pos_err("Expected params", &rule)?
            .into_inner();
        // The param key
        let key = xml_param_inner
            .next()
            .pos_err("Expected key", &rule)?
            .as_str();
        // The param value
        let inner = xml_param_inner.next().pos_err("Expected value", &rule)?;
        match inner.as_rule() {
            Rule::expr => {
                let val = parse_expr(inner)?;
                widget.params.insert(key.to_owned(), val);
            }
            Rule::string => {
                let mut val = Expr::new();
                val.set_string(trim_string(inner.as_str()).to_owned());
                widget.params.insert(key.to_owned(), val);
            }
            _ => unreachable!(),
        }
    }
    // Get expressions inside widget
    while match widget_inner
        .peek()
        .pos_err("Expected something in widget", &rule)?
        .as_rule()
    {
        // Match while is expr
        Rule::expr => true,
        _ => false,
    } {
        widget
            .children
            .push(parse_expr(widget_inner.next().unwrap())?);
    }
    // Get widget children
    while match widget_inner
        .peek()
        .pos_err("Expected something in widget", &rule)?
        .as_rule()
    {
        Rule::widget => true,
        _ => false,
    } {
        let mut expr = Expr::new();
        expr.set_widget(parse_widget(
            widget_inner
                .next()
                .pos_err("Expected child widget", &rule)?,
        )?);
        widget.children.push(expr);
    }
    Ok(widget)
}

/// Parse struct parameters
fn parse_struct_params(
    rule: Pair<Rule>,
) -> Result<
    HashMap<
        &str,
        (
            Option<Expr>,
            Option<HashMap<String, Expr>>,
            Option<Vec<Expr>>,
        ),
    >,
> {
    let params = rule.into_inner();
    // Result
    let mut map = HashMap::new();
    for param in params {
        let mut param_inner = param.clone().into_inner();
        let key = param_inner.next().pos_err("Expected key", &param)?.as_str();
        match param_inner
            .peek()
            .pos_err("Expected param value", &param)?
            .as_rule()
        {
            Rule::expr => {
                map.insert(
                    key,
                    (
                        Some(parse_expr(
                            param_inner
                                .next()
                                .pos_err("Expected param expr value", &param)?,
                        )?),
                        None,
                        None,
                    ),
                );
            }
            Rule::map => {
                let mut inner_map = HashMap::new();
                // Convert pairs
                for pair in param_inner
                    .next()
                    .pos_err("Expected map pairs", &param)?
                    .into_inner()
                {
                    let mut pair_inner = pair.clone().into_inner();
                    let key = pair_inner.next().pos_err("Expected key", &pair)?.as_str();
                    let val = parse_expr(pair_inner.next().context("Expected value")?)?;
                    inner_map.insert(key.to_owned(), val);
                }
                map.insert(key, (None, Some(inner_map), None));
            }
            Rule::list => {
                let mut inner_vec = Vec::new();
                for pair in param_inner
                    .next()
                    .pos_err("Expected list items", &param)?
                    .into_inner()
                {
                    inner_vec.push(parse_expr(pair)?);
                }
                map.insert(key, (None, None, Some(inner_vec)));
            }
            _ => unreachable!(),
        }
    }
    Ok(map)
}

/// Parses an expression from a rule
fn parse_expr(rule: Pair<Rule>) -> Result<Expr> {
    let expr_inner = rule
        .clone()
        .into_inner()
        .next()
        .context("Expected expression inner")?;
    let mut expr = Expr::new();
    let mut pos = Position::new();
    let code_pos = rule.as_span().start_pos().line_col();
    pos.set_line(code_pos.0.try_into()?);
    pos.set_column(code_pos.1.try_into()?);
    expr.position = Some(pos).into();
    match expr_inner.as_rule() {
        Rule::string => expr.set_string(trim_string(expr_inner.as_str()).to_owned()),
        Rule::int => expr.set_int32(expr_inner.as_str().parse().unwrap()),
        Rule::float => expr.set_float(expr_inner.as_str().parse().unwrap()),
        Rule::bool => match expr_inner.as_str() {
            "true" => expr.set_bool(true),
            "false" => expr.set_bool(false),
            _ => unreachable!(),
        },
        Rule::widget => expr.set_widget(parse_widget(expr_inner)?),
        Rule::house => {
            // Get parse params
            let params = parse_struct_params(expr_inner.clone())?;
            let mut house = HouseExpr::new();
            house.id = Some(
                params
                    .get("id")
                    .cloned()
                    .pos_err("Expected id", &expr_inner)?
                    .0
                    .pos_err("Expected expr value", &expr_inner)?,
            )
            .into();
            house.display_name = Some(
                params
                    .get("displayName")
                    .cloned()
                    .pos_err("Expected display name", &expr_inner)?
                    .0
                    .pos_err("Expected expr value", &expr_inner)?,
            )
            .into();
            house.rooms = params
                .get("rooms")
                .cloned()
                .pos_err("Expected rooms", &expr_inner)?
                .1
                .pos_err("Expected map value", &expr_inner)?;
            expr.set_house(house);
        }
        Rule::room => {
            // Get parse params
            let params = parse_struct_params(expr_inner.clone())?;
            let mut room = RoomExpr::new();
            room.icon = Some(
                params
                    .get("icon")
                    .cloned()
                    .pos_err("Expected icon", &expr_inner)?
                    .0
                    .pos_err("Expected expr value", &expr_inner)?,
            )
            .into();
            room.display_name = Some(
                params
                    .get("displayName")
                    .cloned()
                    .pos_err("Expected displayName", &expr_inner)?
                    .0
                    .pos_err("Expected expr value", &expr_inner)?,
            )
            .into();
            room.controllers = params
                .get("controllers")
                .cloned()
                .pos_err("Expected controllers", &expr_inner)?
                .1
                .pos_err("Expected map value", &expr_inner)?;
            expr.set_room(room);
        }
        Rule::controller => {
            // Get parse params
            let params = parse_struct_params(expr_inner.clone())?;
            let mut controller = ControllerExpr::new();
            controller.brand_color = Some(
                params
                    .get("brandColor")
                    .cloned()
                    .pos_err("Expected brandColor", &expr_inner)?
                    .0
                    .pos_err("Expected expr value", &expr_inner)?,
            )
            .into();
            controller.display_name = Some(
                params
                    .get("displayName")
                    .cloned()
                    .pos_err("Expected displayName", &expr_inner)?
                    .0
                    .pos_err("Expected expr value", &expr_inner)?,
            )
            .into();
            controller.display_interface = Some(
                params
                    .get("displayInterface")
                    .cloned()
                    .pos_err("Expected displayInterface", &expr_inner)?
                    .0
                    .pos_err("Expected expr value", &expr_inner)?,
            )
            .into();
            expr.set_controller(controller);
        }
        Rule::display_interface => {
            // Get parse params
            let params = parse_struct_params(expr_inner.clone())?;
            let mut display_interface = DisplayInterfaceExpr::new();
            display_interface.widgets = params
                .get("widgets")
                .cloned()
                .pos_err("Expected widgets", &expr_inner)?
                .2
                .pos_err("Expected list value", &expr_inner)?
                .into();
            expr.set_display_interface(display_interface);
        }
        Rule::device => {
            // Get parse params
            let params = parse_struct_params(expr_inner.clone())?;
            let mut device = DeviceExpr::new();
            device.lambdas = params
                .get("lambdas")
                .cloned()
                .pos_err("Expected lambdas", &expr_inner)?
                .1
                .pos_err("Expected map value", &expr_inner)?;
            expr.set_device(device);
        }
        Rule::ref_expr => {
            let mut ref_expr = RefExpr::new();
            ref_expr.set_field_ref(expr_inner.as_str().to_string());
            expr.set_field_ref(ref_expr);
        }
        Rule::lambda => {
            let mut lambda = LambdaExpr::new();
            let mut lambda_inner = expr_inner.clone().into_inner();
            let lambda_args = lambda_inner
                .next()
                .pos_err("Expected args", &expr_inner)?
                .into_inner();
            for arg in lambda_args {
                lambda.args.push(arg.as_str().to_owned());
            }
            let body = parse_expr(lambda_inner.next().pos_err("Expected body", &expr_inner)?)?;
            lambda.field_return = Some(body).into();
            expr.set_lambda(lambda);
        }
        Rule::get_lambda => {
            // Create expressions
            let mut get_lambda = CallExpr::new();
            let mut get_lambda_ref_expr = Expr::new();
            let mut get_lambda_ref = RefExpr::new();
            // Reference getLambda function
            get_lambda_ref.set_field_ref("getLambda".to_string());
            // Set ref on expr
            get_lambda_ref_expr.set_field_ref(get_lambda_ref);
            // Set calling to the getLambda call
            get_lambda.calling = Some(get_lambda_ref_expr).into();

            let mut get_lambda_inner = expr_inner.clone().into_inner();
            let base = parse_expr(
                get_lambda_inner
                    .next()
                    .pos_err("Expected getLambda base", &expr_inner)?,
            )?;
            let mut path_expr = Expr::new();
            let path = get_lambda_inner
                .next()
                .pos_err("Expected getLambda path", &expr_inner.clone())?
                .as_str();
            path_expr.set_string(path.to_owned());
            get_lambda.args.push(base);
            get_lambda.args.push(path_expr);
            // Call the get_lambda expression
            expr.set_call(get_lambda);
        }
        Rule::call => {
            let mut call = CallExpr::new();
            let mut call_inner = expr_inner.clone().into_inner();
            call.calling = Some(parse_expr(
                call_inner.next().pos_err("Expected calling", &expr_inner)?,
            )?)
            .into();
            for arg in call_inner {
                call.args.push(parse_expr(arg)?);
            }
            expr.set_call(call);
        }
        Rule::if_expr => {
            let mut if_expr = IfExpr::new();
            let mut if_inner = expr_inner.clone().into_inner();
            // Get if then
            if_expr.condition = Some(parse_expr(
                if_inner.next().pos_err("Expected condition", &expr_inner)?,
            )?)
            .into();
            if_expr.then = Some(parse_expr(
                if_inner.next().pos_err("Expected then", &expr_inner)?,
            )?)
            .into();
            // Get elifs
            while let Rule::elif_expr = if_inner
                .peek()
                .pos_err("Expected something in if", &expr_inner)?
                .as_rule()
            {
                let mut elif = Elif::new();
                let mut elif_inner = if_inner
                    .next()
                    .pos_err("Expected elif", &expr_inner)?
                    .into_inner();
                elif.condition = Some(parse_expr(
                    elif_inner
                        .next()
                        .pos_err("Expected condition", &expr_inner)?,
                )?)
                .into();
                elif.then = Some(parse_expr(
                    elif_inner.next().pos_err("Expected then", &expr_inner)?,
                )?)
                .into();
                if_expr.elif.push(elif);
            }
            // Get final else
            if_expr.field_else = Some(parse_expr(
                if_inner.next().pos_err("Expected else", &expr_inner)?,
            )?)
            .into();

            expr.set_field_if(if_expr);
        }
        Rule::index => {
            // Create expressions
            let mut index = CallExpr::new();
            let mut index_ref_expr = Expr::new();
            let mut index_ref = RefExpr::new();
            // Ref index function
            index_ref.set_field_ref("index".to_string());
            index_ref_expr.set_field_ref(index_ref);
            // Set expr calling to index function
            index.calling = Some(index_ref_expr).into();

            let mut index_inner = expr_inner.clone().into_inner();
            let input = parse_expr(
                index_inner
                    .next()
                    .pos_err("Expected index input", &expr_inner)?,
            )?;
            index.args.push(input);
            for path in index_inner {
                index.args.push(parse_expr(path)?);
            }
            expr.set_call(index);
        }
        _ => unreachable!(),
    }
    Ok(expr)
}
