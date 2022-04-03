# Get Lambda

## Syntax

```pest
get_lambda_expr = { ref_expr | device | if_expr | call }
get_lambda = { get_lambda_expr ~ "." ~ ident }
```

## Static Semantics: Conversion

- Return *CallExpr* calling a *RefExpr* to "getLambda" with args *deviceExpr* and *lambdaName*
