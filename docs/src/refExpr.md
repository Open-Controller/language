# RefExpr

## Syntax

```ref_expr = @{ ident }```

## Proto

```proto
/** A message representing a reference to the cope */
message RefExpr {
    required string ref = 1; // The name of the desired reference in the scope
}
```

## Runtime Semantics: Evaluation

- If key *ref* in Local Scope
  - Returns ? the value of *ref* in Local Scope
- Else If key *ref* in Builtins
  - Returns ? the value of *ref* in Builtins
- Else If key *ref* in Module Scope
  - Returns ? the value of *ref* in Module Scope
- Else
  - Type Panic
