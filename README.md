# oxc-graphql-parser

This is a fork of [apollographql/apollo-rs](https://github.com/apollographql/apollo-rs) for Oxfmt; only the parser remains.

`oxc-graphql-parser` is an error-resilient GraphQL lexer and parser that produces a typed AST.

How it parses, and why, is in [AGENTS.md](https://github.com/oxc-project/oxc-graphql-parser/blob/main/AGENTS.md).

## Example

```rust
use oxc_graphql_parser::{Allocator, Parser};

let allocator = Allocator::default();
let ast = Parser::new(&allocator, "union SearchResult = Photo | Person").parse();
assert_eq!(ast.errors().len(), 0);
println!("{:#?}", ast.document());
```

More examples are available in [`examples`](https://github.com/oxc-project/oxc-graphql-parser/tree/main/crates/oxc_graphql_parser/examples).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/oxc-project/oxc-graphql-parser/blob/main/LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/oxc-project/oxc-graphql-parser/blob/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

# [Sponsored By](https://oxc.rs/sponsor)

<p align="center">
  <a href="https://oxc.rs/sponsor">
    <img src="https://raw.githubusercontent.com/oxc-project/sponsors/main/sponsors.svg" alt="Our sponsors" />
  </a>
</p>
