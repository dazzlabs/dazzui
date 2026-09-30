//! DazzUI for [GPUI](https://gpui.rs): the design system's tokens, themes
//! and components, drawn to the geometry `@dazzlabs/dazzui` draws.
//!
//! `@dazzlabs/dazzui` is the standard. Every name, prop and length here is
//! read off the React source; where the two disagree, React wins.

mod assets;
pub mod components;
pub mod foundation;
mod generated;
pub mod glyphs;
pub mod theme;
pub mod tokens;

pub use assets::Assets;
pub use components::*;
pub use foundation::{
    Length, Profile, Size, State, StyledExt, Tint, Variant, baseline, content_area,
};
pub use generated::{THEMES, ThemeTokens};
pub use theme::{ActiveTheme, Theme, themed};

/// Installs what the kit needs app-wide: the theme, Studio Light until one
/// is set, the keyboard watch that decides when focus is shown, and the keys
/// a text field edits with.
pub fn init(cx: &mut gpui::App) {
    if !cx.has_global::<Theme>() {
        Theme::set("studio-light", cx);
    }
    foundation::focus::init(cx);
    foundation::text_input::init(cx);
}
