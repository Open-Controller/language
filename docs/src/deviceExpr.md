# DeviceExpr

## Syntax

```pest
device = { "device" ~ "{" ~ (struct_param)+ ~ "}" }
```

## Proto

```proto
/** A message representing a controllable device */
message DeviceExpr {
    map<string, Expr> lambdas = 1; // The actions the Device is capable of
}
```

## Static Semantic: Early Errors

- Throws Parse Error if there is no *lambdas* field.

## Runtime Semantics: GetDeviceLambda

- Return the lambda in *lambdas* with the key *key*
