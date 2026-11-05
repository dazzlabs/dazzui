#![doc = include_str!("../README.md")]

pub mod axes;
pub mod components;
pub mod generated;
pub mod styles;
pub mod theme;
pub mod tokens;

pub use axes::{Part, Size, State, Tint, Variant};
pub use components::*;
pub use generated::ThemeTokens;
pub use theme::{Theme, theme_scope};

pub mod prelude {
    pub use crate::{Part, Size, State, Theme, Tint, Variant, components::*, theme_scope};
    pub use nsh_core::prelude::*;
}
