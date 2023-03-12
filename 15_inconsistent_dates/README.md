# Rust Code Challenges: Challenge 15

Your challenge is to create a parser for dates that can accept
multiple date formats. You'll need to be able to handle different
delimiters and missing data.

## Testing your solution

Use `cargo test` to evaluate yourself:

```console
$ cargo test
...
```

## Requirements

### Flexible parser

Create a single function that can parse dates from the following patterns:

2002 Feb 02
2010-12-11
1999/March/02
01.March.2021
Mar.05.2021

### Extra

* More formats
* More locales
* Ambiguous cases
