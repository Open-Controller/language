# ControllerExpr

## Syntax

```pest
controller = { "controller" ~ "{" ~ (struct_param)+ ~ "}" }
```

## Proto

```proto
/** A message representing a controller */
message ControllerExpr {
    required Expr display_name = 1; // The name to be displayed by clients
    optional Expr brand_color = 2; // A brand color to be displayed by clients
    optional Expr display_interface = 3; // The interface for display clients for the Controller
}
```

## Static Semantic: Early Errors

- Throws Parse Error if there is no *displayName* field.

## Runtime Semantics: Display
