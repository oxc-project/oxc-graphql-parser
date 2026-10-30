# AGENTS.md - AI Assistant Guide for oxc-graphql-parser

A fork of [apollo-rs](https://github.com/apollographql/apollo-rs)'s `apollo-parser`:
a hand-written, recursive-descent GraphQL lexer and parser producing a typed AST plus a list of errors.
It is the parsing layer for oxc's GraphQL formatter (`oxc_formatter_graphql`).

## Syntax

The reference is graphql-js v17.x (Prettier's GraphQL parser), not one spec edition.
Upstream follows spec editions and stops at October 2021; what graphql-js parses beyond that is added here.

- Executable descriptions, directives on directives and `extend directive` are always on.
- Fragment arguments (`...F(size: $size)`, and the legacy `fragment F($x: Int) on T`) are behind `Parser::experimental_fragment_arguments`.
  Parser options follow graphql-js's; this is the only one kept.

## Scope

- `parse` always returns an AST, even when errors are reported.
- No lossless syntax tree: comments, whitespace and commas are not AST nodes.
  Comment spans: `Ast::comments`.

## Architecture

- `src/lexer/`: an iterator over `Result<Token, Error>`, also public as `Lexer`.
  Lexing errors are collected and lexing continues.
- `src/parser_ast.rs`: consumes significant tokens, skipping ignored ones (comments, whitespace, commas), and builds the AST directly.
  - Entry points: `parse` (a document), `parse_selection_set`, `parse_type`.
  - `token_limit` / `recursion_limit` bound the work (`src/limit.rs`).
- `src/ast.rs`: arena-allocated nodes (`oxc_allocator`) with byte spans, one per grammar production.

## Testing

| Command | Covers |
| --- | --- |
| `cargo test --workspace` | lexer and parser tests (`tests/lexer.rs`, `tests/integration.rs`), doctests |
| `OXC_GRAPHQL_ECOSYSTEM_REPOS=<dir> cargo test -- --ignored` | every `.graphql` file under an ecosystem-ci repos directory parses without errors |
| `cargo +nightly fuzz run <lexer\|parser\|parser_limited> -- -max_total_time=60` | panic and hang fuzzing (`fuzz/`), needs `cargo-fuzz` |
| `cargo bench -p oxc-graphql-parser` | parse speed (`benches/`) |

CI (`.github/workflows`) runs check, test, clippy, doc and typos.

- `test_data/{lexer,parser}/{ok,err}/*.graphql` is a corpus:
  each file must lex back to its source when it lexes without errors, and must parse without panicking.
  The `.txt` files beside them are not compared; they serve as garbage input.
