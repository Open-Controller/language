# LambdaExpr

## Syntax

```lambda = { "(" ~ lambda_args ~ ")" ~ "=>" ~ expr }```

## Proto

```proto
/** A message representing a lambda expression */
message LambdaExpr {
    repeated string args = 1; // The args to the lambda
    required Expr return = 2; // The return of the lambda
}
```

## Runtime Semantics: Evaluation

- Return Fn that evaluates *return* with captured Module Scope and an intersection of the captured Local Scope and the Fn arguments, with the arguments taking precedence
