# DisplayInterfaceExpr

## Syntax

```pest
display_interface = { "displayInterface" ~ "{" ~ (struct_param)+ ~ "}" }
```

## Proto

```proto

/** A Controller interface for clients with displays */
message DisplayInterfaceExpr {
    /**
        * The Widgets that make up the controller.
        * 
    * Implementation Notes: The layout of the widgets in the array is flexible for the client.
        */
    repeated Expr widgets = 1;
}
```

## Static Semantic: Early Errors

- Throws Parse Error if there is no *widgets* field.

## Runtime Semantics: Display
