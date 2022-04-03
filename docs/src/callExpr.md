# CallExpr

## Syntax

```call = { "(" ~ (expr)+ ~ ")" }```

## Proto

```proto
/** A message representing a function call */
message CallExpr {
    required Expr calling = 1; // The expression to call
    repeated Expr args = 2; // The arguments to pass
}
```

## Runtime Semantics: Evaluation

- Let *argsResult* be *args* evaluated with Local Scope and Module Scope
- Returns ? *FnEvaluate* with *argsResult*
