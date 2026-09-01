//! The hand-written half of the token vocabulary.
//!
//! `tokens` holds the types `generated/` is written against, `axes` the axes
//! a component varies along and the resolution every interactive one reads,
//! `interaction` the substrate that turns a pointer into one of those states,
//! and `styling` the element builders that put a token on the screen.

mod axes;
mod interaction;
mod styling;
mod tokens;

pub use axes::*;
pub use interaction::*;
pub use styling::*;
pub use tokens::*;
