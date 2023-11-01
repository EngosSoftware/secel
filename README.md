# Simply Enough Condition Expression Language Parser

# WORK IN PROGRESS**

## Simply Enough Condition Expression Language Grammar

```
      statement = if_expression
                ;
  if_expression = `if` `(` condition `;` expression `;` expression `)`
                ;
      condition = disjunction { `or` disjunction }
                ;
    disjunction = conjunction { `and` conjunction }
                ;
   
    conjunction = `(` condition `)`
                | comparison
                ;
     comparison = value (`=` | `<>` | `>` | `<` | `>=` | `<=`) value
                ;
     expression = value
                | if_expression
                ;
          value = NUMBER
                | NULL
                ;
```

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
