# Module

## Syntax

```module = { SOI ~ (import)* ~ expr ~ EOI }```

## Proto

```proto
/** A message representing an OpenController module */
message Module {
    map<string, Module> imports = 1; // The imports of the module
    required Expr body = 2; // The body of the module
}
```

## Static Semantics: ImportEntry

### Syntax

```import = { "import" ~ string ~ "as" ~ ident }```

### Static Semantics

- Return a pair of the import name to the module at the import path

## Static Semantics: ImportEntries

- Let *importEntries* be a map of the *ImportEntry* of *imports*
- Return *importEntries*

## Static Semantics: imports

- Return the *ImportEntries* of *imports*

## Runtime Semantics: Evaluation

1. Let *importResults* be the result of evaluating the *imports* modules.
2. Return the result of evaluating *body* with *importResults* as the Module Scope.
