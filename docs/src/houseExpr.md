# HouseExpr

## Syntax

```pest
house = { "house" ~ "{" ~ (struct_param)+ ~ "}" }
```

## Proto

```proto
/** The root message representing a house. **/
message HouseExpr {
    required Expr display_name = 1; // The name to be displayed by clients
    required Expr id = 2; // A unique identifier
    map<string, Expr> rooms = 3; // The Rooms inside the House
}
```

## Static Semantic: Early Errors

- Throws Parse Error if there is no *displayName* field.
- Throws Parse Error if there is no *id* field.
- Throws Parse Error if there is no *rooms* field.

## Runtime Semantics: Display
