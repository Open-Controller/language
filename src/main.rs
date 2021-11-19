extern crate pest;
#[macro_use]
extern crate pest_derive;

mod OpenControllerLib;

use std::{collections::HashMap, fs};

use OpenControllerLib::Expr;
use pest::{Parser, iterators::Pair};
use protobuf::Message;

use crate::OpenControllerLib::{CallExpr, ControllerExpr, DeviceExpr, DisplayInterfaceExpr, HouseExpr, LambdaExpr, Module, RefExpr, RoomExpr, WidgetExpr};

#[derive(Parser)]
#[grammar = "oc.pest"]
pub struct OCParser;

fn trim_string(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn parse_module(file: &str) -> Module {
    let unparsed_file = fs::read_to_string(file).expect("cannot read file");
    let file = OCParser::parse(Rule::module, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap();
    let mut module = Module::new();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::import => {
                let mut inner_rules = line.into_inner();
                let path = inner_rules.next().unwrap().as_str();
                let name = inner_rules.next().unwrap().as_str();
                module.imports.insert(name.to_owned(), parse_module(trim_string(path)));
            },
            Rule::expr => {
                module.body = Some(parse_expr(line)).into();
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    module
}

fn parse_widget(rule: Pair<Rule>) -> WidgetExpr {
    let mut widget = WidgetExpr::new();
    let mut widget_inner = rule.into_inner();
    let tag = widget_inner.next().unwrap().as_str();
    widget.set_widget_type(tag.to_owned());
    while match widget_inner.peek().unwrap().as_rule() {
        Rule::xml_param => true,
        _ => false
    } {
        let mut xml_param_inner = widget_inner.next().unwrap().into_inner();
        let key = xml_param_inner.next().unwrap().as_str();
        let inner = xml_param_inner.next().unwrap();
        match inner.as_rule() {
            Rule::expr => {
                let val = parse_expr(inner);
                widget.params.insert(key.to_owned(), val);
            }
            Rule::string => {
                let mut val = Expr::new();
                val.set_string(trim_string(inner.as_str()).to_owned());
                widget.params.insert(key.to_owned(), val);
            }
            _ => unreachable!()
        }
    }
    while match widget_inner.peek().unwrap().as_rule() {
        Rule::expr => true,
        _ => false
    } {
        widget.children.push(parse_expr(widget_inner.next().unwrap()));
    }
    while match widget_inner.peek().unwrap().as_rule() {
        Rule::widget => true,
        _ => false
    } {
        let mut expr = Expr::new();
        expr.set_widget(parse_widget(widget_inner.next().unwrap()));
        widget.children.push(expr);
    }
    widget
}

fn parse_struct_params(rule: Pair<Rule>) -> HashMap<&str, (Option<Expr>, Option<HashMap<String, Expr>>, Option<Vec<Expr>>)> {
    let params = rule.into_inner();
    let mut map = HashMap::new();
    for param in params {
        let mut param_inner = param.into_inner();
        let key = param_inner.next().unwrap().as_str();
        match param_inner.peek().unwrap().as_rule() {
            Rule::expr => {
                map.insert(key, (Some(parse_expr(param_inner.next().unwrap())), None, None));
            },
            Rule::map => {
                let mut inner_map = HashMap::new();
                for pair in param_inner.next().unwrap().into_inner() {
                    let mut pair_inner = pair.into_inner();
                    let key = pair_inner.next().unwrap().as_str();
                    let val = parse_expr(pair_inner.next().unwrap());
                    inner_map.insert(key.to_owned(), val);
                }
                map.insert(key, (None, Some(inner_map), None));
            },
            Rule::list => {
                let mut inner_vec = Vec::new();
                for pair in param_inner.next().unwrap().into_inner() {
                    inner_vec.push(parse_expr(pair));
                }
                map.insert(key, (None, None, Some(inner_vec)));
            },
            _ => unreachable!(),
        }
    }
    map
}

fn parse_expr(rule: Pair<Rule>) -> Expr {
    let expr_inner = rule.into_inner().next().unwrap();
    let mut expr = Expr::new();
    match expr_inner.as_rule() {
        Rule::string => expr.set_string(trim_string(expr_inner.as_str()).to_owned()),
        Rule::int => expr.set_int32(expr_inner.as_str().parse().unwrap()),
        Rule::float => expr.set_float(expr_inner.as_str().parse().unwrap()),
        Rule::bool => match expr_inner.as_str() {
            "true" => expr.set_bool(true),
            "false" => expr.set_bool(false),
            _ => unreachable!(),
        },
        Rule::widget => expr.set_widget(parse_widget(expr_inner)),
        Rule::house => {
            let params = parse_struct_params(expr_inner);
            let mut house = HouseExpr::new();
            house.id = Some(params.get("id").cloned().unwrap().0.unwrap()).into();
            house.display_name = Some(params.get("displayName").cloned().unwrap().0.unwrap()).into();
            house.rooms = params.get("rooms").cloned().unwrap().1.unwrap();
            expr.set_house(house);
        },
        Rule::room => {
            let params = parse_struct_params(expr_inner);
            let mut room = RoomExpr::new();
            room.icon = Some(params.get("icon").cloned().unwrap().0.unwrap()).into();
            room.display_name = Some(params.get("displayName").cloned().unwrap().0.unwrap()).into();
            room.controllers = params.get("controllers").cloned().unwrap().1.unwrap();
            expr.set_room(room);
        },
        Rule::controller => {
            let params = parse_struct_params(expr_inner);
            let mut controller = ControllerExpr::new();
            controller.brand_color = Some(params.get("brandColor").cloned().unwrap().0.unwrap()).into();
            controller.display_name = Some(params.get("displayName").cloned().unwrap().0.unwrap()).into();
            controller.display_interface = Some(params.get("displayInterface").cloned().unwrap().0.unwrap()).into();
            expr.set_controller(controller);
        },
        Rule::display_interface => {
            let params = parse_struct_params(expr_inner);
            let mut display_interface = DisplayInterfaceExpr::new();
            display_interface.widgets = params.get("widgets").cloned().unwrap().2.unwrap().into();
            expr.set_display_interface(display_interface);
        },
        Rule::device => {
            let params = parse_struct_params(expr_inner);
            let mut device = DeviceExpr::new();
            device.lambdas = params.get("lambdas").cloned().unwrap().1.unwrap();
            expr.set_device(device);
        },
        Rule::ref_expr => {
            let mut ref_expr = RefExpr::new();
            ref_expr.set_field_ref(expr_inner.as_str().to_string());
            expr.set_field_ref(ref_expr);
        },
        Rule::lambda => {
            let mut lambda = LambdaExpr::new();
            let mut lambda_inner = expr_inner.into_inner();
            let lambda_args = lambda_inner.next().unwrap().into_inner();
            for arg in lambda_args {
                lambda.args.push(arg.as_str().to_owned());
            }
            let body = parse_expr(lambda_inner.next().unwrap());
            lambda.field_return = Some(body).into();
            expr.set_lambda(lambda);
        },
        Rule::get_lambda => {
            let mut get_lambda = CallExpr::new();
            let mut get_lambda_ref_expr = Expr::new();
            let mut get_lambda_ref = RefExpr::new();
            get_lambda_ref.set_field_ref("getLambda".to_string());
            get_lambda_ref_expr.set_field_ref(get_lambda_ref);
            get_lambda.calling = Some(get_lambda_ref_expr).into();

            let mut get_lambda_inner = expr_inner.into_inner();
            let base = parse_expr(get_lambda_inner.next().unwrap());
            let mut path_expr = Expr::new();
            let path = get_lambda_inner.next().unwrap().as_str();
            path_expr.set_string(path.to_owned());
            get_lambda.args.push(base);
            get_lambda.args.push(path_expr);
            expr.set_call(get_lambda);
        },
        Rule::call => {
            let mut call = CallExpr::new();
            let mut call_inner = expr_inner.into_inner();
            call.calling = Some(parse_expr(call_inner.next().unwrap())).into();
            for arg in call_inner {
                call.args.push(parse_expr(arg));
            }
            expr.set_call(call);
        },
        Rule::if_expr => todo!(),
        _ => unreachable!(),
    }
    expr
}

fn main() {
    println!("{:#?}", parse_module("home.ocdef"));
    fs::write("./test.ocbin", parse_module("home.ocdef").write_to_bytes().unwrap()).unwrap();
}