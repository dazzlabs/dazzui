//! The `ui!` DSL: its AST and the parser that builds it.
//!
//! One parser serves two readers. The proc macro hands it the macro's
//! tokens; the CLI finds `ui!` invocations in `.rs` files with
//! `syn::parse_file` and hands it the same tokens. Two parsers would drift
//! into two languages.

mod ast;
mod parser;

pub use ast::*;

/// Parses the tokens inside `ui! { … }`.
pub fn parse_page(tokens: proc_macro2::TokenStream) -> syn::Result<PageAst> {
    syn::parse2(tokens)
}
