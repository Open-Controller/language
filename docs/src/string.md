# String

## Syntax

```pest
// matches anything between 2 double quotes
double_quoted_string = @{ "\"" ~ (!("\"") ~ ANY)* ~ "\""}
// matches anything between 2 single quotes
single_quoted_string = @{ "\'" ~ (!("\'") ~ ANY)* ~ "\'"}

string = @{
    double_quoted_string |
    single_quoted_string
}
```

## Proto

```string```

## Runtime Semantics: Evaluation

- Returns ? *string*
