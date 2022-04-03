# IfExpr

## Syntax

```pest
elif_expr = { "elif" ~ expr ~ "{" ~ expr ~ "}" }
if_expr = { "if" ~ expr ~ "{" ~ expr ~ "}" ~ (elif_expr)* ~ "else" ~ "{" ~ expr ~ "}" }
```

## Proto

```proto

/** A message representing an else-if */
message Elif {
    required Expr condition = 1; // The condition to call if
    required Expr then = 2; // The expression to call
}

/** A message representing an if expression */
message IfExpr {
    required Expr condition = 1; // The condition to call if
    required Expr then = 2; // The expression to call
    repeated Elif elif = 3; // The else-if conditions
    required Expr else = 4; // The else condition
}
```

## Runtime Semantics: Evaluation

- Let *conditionResult* be *condition* evaluated with Local Scope and Module Scope
- If *conditionResult* is true
  - Return *then* evaluated with Local Scope and Module Scope
- Else
  - For each *elif*, let *conditionResult* be *condition* evaluated with Local Scope and Module Scope
    - If *conditionResult* is true
      - Return *then* evaluated with Local Scope and Module Scope
  - Return *else* evaluated with Local Scope and Module Scope
