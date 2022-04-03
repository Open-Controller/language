# Int

## Syntax

```pest
int = @{ "-" ? ~ ("0" | '1'..'9' ~ '0'..'9' * ) }
```

## Proto

```int32```

## Runtime Semantics: Evaluation

- Returns ? *int*
