//! The DazzUI design system for [GPUI] — the platform sibling of
//! `packages/ui_react`, `packages/ui_flutter` and `packages/ui_swiftui`. Same
//! tokens, same four themes, same components.
//!
//! **`packages/ui_react` is the standard.** Every name, prop and metric here
//! is read off the React source; where the two disagree, the React file wins.
//!
//! ```no_run
//! use dazzui::{Button, ControlVariant, prelude::*};
//!
//! # fn example() -> impl gpui::IntoElement {
//! Button::new("translate", "Translate").variant(ControlVariant::Filled)
//! # }
//! ```
//!
//! Names match the React export exactly, with no prefix — `Button` is
//! `Button`, `Divider` is `Divider`, `Table` is `Table`. The colour axis is
//! `tint` ([`Tint::Primary`]), never `kind` or `secondary`: the ds vocabulary
//! is the API here as much as it is in Flutter and Swift.
//!
//! [GPUI]: https://gpui.rs

mod components;
mod foundation;
mod generated;
mod theme;

pub mod assets;

pub use components::*;
pub use foundation::*;
pub use generated::colors;
pub use generated::theme_variables::{STUDIO_LIGHT_SEED, ThemeSeed, ThemeVariables};
pub use theme::*;

/// The traits a host imports to build with this crate, GPUI's own included.
///
/// ```no_run
/// use dazzui::prelude::*;
/// ```
pub mod prelude {
    pub use crate::foundation::StyledExt;
    pub use crate::theme::ActiveTheme;
    pub use gpui::prelude::*;
}
