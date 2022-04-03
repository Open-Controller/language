# RoomExpr

## Syntax

```pest
room = { "room" ~ "{" ~ (struct_param)+ ~ "}" }
```

## Proto

```proto
/** A message representing a room. */
message RoomExpr {
    required Expr display_name = 1; // The name to be displayed by clients
    map<string, Expr> controllers = 2; // The Controllers inside the Room
    optional Expr icon = 3; // The icon of the Room to be displayed by clients
}
```

## Static Semantic: Early Errors

- Throws Parse Error if there is no *display_name* field.
- Throws Parse Error if there is no *controllers* field.

## Runtime Semantics: Display
