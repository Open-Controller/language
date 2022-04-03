# Float

## Syntax

```pest
float = @{
    "-" ? ~
    (
        "0" ~ "." ~ '0'..'9' + |
        '1'..'9' ~ '0'..'9' * ~ "." ~ '0'..'9' +
    )
}
```

## Proto

```float```

## Runtime Semantics: Evaluation

- Returns ? *float*
