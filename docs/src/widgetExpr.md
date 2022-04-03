# WidgetExpr

## Syntax

```pest
xml_param = { ident ~ "=" ~ (("{" ~ expr ~ "}") | string) }
widget = { "<" ~ ident ~ (xml_param)* ~ ">" ~ (widget | "{" ~ expr ~ "}")* ~ "</" ~ ident ~ ">" }
```

## Proto

```proto
/** A message representing any controller widget that can be displayed by clients */
message WidgetExpr {
    required string widget_type = 2; // The type of widget
    map<string, Expr> params = 3; // The parameters to the widget
    repeated Expr children = 4; // The children of the widget
}
```

## Runtime Semantics: Display
