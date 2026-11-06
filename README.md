# json-fmt

A JSON formatter written in Rust from scratch, no dependencies. Built as a learning project.

**Status: work in progress.** Only the tokenizer exists so far; there is no parser or formatter yet.

## Progress

- [x] Structural tokens: `{ } [ ] , :`
- [x] Literals: `true`, `false`, `null`
- [x] Strings (plain, no escape handling yet)
- [x] Whitespace skipping (space, tab, `\n`, `\r`)
- [x] Unit tests for the above (28)
- [ ] Numbers (`Token::Number` is declared but never produced)
- [ ] String escape sequences (`\"`, `\n`, `\uXXXX`)
- [ ] Parser / syntax validation
- [ ] Pretty-printing, reading input from a file or stdin

## Design notes

- `tokenizer(&str) -> Result<Vec<Token>, String>` walks the input as a `Vec<char>` and returns the first error with a 1-based position.
- Token types: `LeftBrace`, `RightBrace`, `LeftBracket`, `RightBracket`, `Comma`, `Colon`, `String(String)`, `Number(String)`, `True`, `False`, `Null`.
- Literals: a literal must match exactly and be followed by a structural character, whitespace, or end of input. `truex` and `null1` are errors.
- Strings: scanned from the opening quote to the next `"`; an unclosed string is an error. A backslash is not treated specially yet.
- Error messages are currently in Turkish.

## Build and test

```
cargo build
cargo test
```

`cargo test` runs 28 unit tests, all passing at the time of writing. `cargo run` currently tokenizes a hard-coded input and prints the result.

## License

MIT
