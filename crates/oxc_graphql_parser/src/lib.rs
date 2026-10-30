//! oxc-graphql-parser is an error-resilient GraphQL lexer and parser
//! that produces a typed AST.
//!
//! ## Basic Usage
//!
//! ```rust
//! use oxc_graphql_parser::{Allocator, Parser};
//!
//! let allocator = Allocator::default();
//! let ast = Parser::new(&allocator, "union SearchResult = Photo | Person").parse();
//!
//! // `parse` always returns an AST; check the errors before walking it.
//! assert_eq!(ast.errors().len(), 0);
//! for definition in &ast.document().definitions {
//!     println!("{definition:?}");
//! }
//! ```

pub mod ast;
mod lexer;

mod error;
mod limit;
mod parser_ast;

pub use crate::error::Error;
pub use crate::lexer::Lexer;
pub use crate::lexer::Token;
pub use crate::lexer::TokenKind;
pub use crate::limit::LimitTracker;
pub use crate::parser_ast::Parser;
pub use oxc_allocator::Allocator;
