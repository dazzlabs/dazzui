//! The semantic UI IR.
//!
//! A page here is what it means — a vertical stack, a text, a button that
//! sends `Save` — never what any platform calls the control that draws it.
//! Read-only values, two-way bindings and event actions are distinct types,
//! and every node carries the place in the Rust source it came from.

mod component;
mod diagnostics;
mod expr;
mod generated;
mod node;
mod source;

pub use component::*;
pub use diagnostics::*;
pub use expr::*;
pub use generated::*;
pub use node::*;
pub use source::*;
