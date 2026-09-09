//! The glyphs the components name, and the [`AssetSource`] that serves them.
//!
//! The set is Fluent — Microsoft's, the same set `packages/ui_react` pulls
//! from `@fluentui/react-icons` and `packages/ui_swiftui` takes from the
//! official Swift package. That parity is the point: the React components
//! name specific glyphs, so a port drawing its own checkmark would be a
//! different mark at a different weight in the same box. The artwork is
//! Fluent's, under its MIT licence; only the SVG wrapper around each path is
//! this repository's.
//!
//! GPUI resolves an SVG path through the app's asset source, so a host
//! installs this one:
//!
//! ```no_run
//! use gpui::Application;
//!
//! Application::new().with_assets(dazzui::assets::Assets).run(|_cx| {});
//! ```
//!
//! A host that already has an asset source of its own chains
//! [`Assets::bytes`] into it instead, rather than replacing it.

use std::borrow::Cow;

use gpui::{AssetSource, SharedString};

/// The path each glyph is served at.
///
/// The names are Fluent's own, so a component naming `CHECKMARK_12_FILLED`
/// names the same glyph the React component imports as `Checkmark12Filled`.
pub mod icons {
    pub const ALERT_16_REGULAR: &str = "dazzui/icons/alert_16_regular.svg";
    pub const CHECKBOX_CHECKED_16_REGULAR: &str =
        "dazzui/icons/checkbox_checked_16_regular.svg";
    pub const CHECKMARK_12_FILLED: &str = "dazzui/icons/checkmark_12_filled.svg";
    pub const CHEVRON_DOWN_12_REGULAR: &str = "dazzui/icons/chevron_down_12_regular.svg";
    pub const CHEVRON_LEFT_16_REGULAR: &str = "dazzui/icons/chevron_left_16_regular.svg";
    pub const CHEVRON_RIGHT_12_REGULAR: &str = "dazzui/icons/chevron_right_12_regular.svg";
    pub const CHEVRON_RIGHT_16_REGULAR: &str = "dazzui/icons/chevron_right_16_regular.svg";
    pub const CHEVRON_UP_12_REGULAR: &str = "dazzui/icons/chevron_up_12_regular.svg";
    pub const DELETE_16_REGULAR: &str = "dazzui/icons/delete_16_regular.svg";
    pub const DISMISS_12_REGULAR: &str = "dazzui/icons/dismiss_12_regular.svg";
    pub const DISMISS_CIRCLE_12_FILLED: &str = "dazzui/icons/dismiss_circle_12_filled.svg";
    pub const PAINT_BRUSH_16_REGULAR: &str = "dazzui/icons/paint_brush_16_regular.svg";
    pub const PIN_16_REGULAR: &str = "dazzui/icons/pin_16_regular.svg";
    pub const RENAME_16_REGULAR: &str = "dazzui/icons/rename_16_regular.svg";
    pub const SEARCH_16_REGULAR: &str = "dazzui/icons/search_16_regular.svg";
    pub const SETTINGS_16_REGULAR: &str = "dazzui/icons/settings_16_regular.svg";
    pub const SQUARE_MULTIPLE_16_REGULAR: &str =
        "dazzui/icons/square_multiple_16_regular.svg";
    pub const STAR_16_REGULAR: &str = "dazzui/icons/star_16_regular.svg";
    pub const SUBTRACT_12_FILLED: &str = "dazzui/icons/subtract_12_filled.svg";
    pub const TABLE_16_REGULAR: &str = "dazzui/icons/table_16_regular.svg";
}

/// Every glyph, embedded. Nothing is read from disk at runtime, so a host
/// ships one binary rather than a binary and a resource directory.
const FILES: &[(&str, &str)] = &[
    (
        icons::ALERT_16_REGULAR,
        include_str!("../assets/icons/alert_16_regular.svg"),
    ),
    (
        icons::CHECKBOX_CHECKED_16_REGULAR,
        include_str!("../assets/icons/checkbox_checked_16_regular.svg"),
    ),
    (
        icons::CHECKMARK_12_FILLED,
        include_str!("../assets/icons/checkmark_12_filled.svg"),
    ),
    (
        icons::CHEVRON_DOWN_12_REGULAR,
        include_str!("../assets/icons/chevron_down_12_regular.svg"),
    ),
    (
        icons::CHEVRON_LEFT_16_REGULAR,
        include_str!("../assets/icons/chevron_left_16_regular.svg"),
    ),
    (
        icons::CHEVRON_RIGHT_12_REGULAR,
        include_str!("../assets/icons/chevron_right_12_regular.svg"),
    ),
    (
        icons::CHEVRON_UP_12_REGULAR,
        include_str!("../assets/icons/chevron_up_12_regular.svg"),
    ),
    (
        icons::SUBTRACT_12_FILLED,
        include_str!("../assets/icons/subtract_12_filled.svg"),
    ),
    (
        icons::CHEVRON_RIGHT_16_REGULAR,
        include_str!("../assets/icons/chevron_right_16_regular.svg"),
    ),
    (
        icons::DELETE_16_REGULAR,
        include_str!("../assets/icons/delete_16_regular.svg"),
    ),
    (
        icons::DISMISS_12_REGULAR,
        include_str!("../assets/icons/dismiss_12_regular.svg"),
    ),
    (
        icons::DISMISS_CIRCLE_12_FILLED,
        include_str!("../assets/icons/dismiss_circle_12_filled.svg"),
    ),
    (
        icons::PAINT_BRUSH_16_REGULAR,
        include_str!("../assets/icons/paint_brush_16_regular.svg"),
    ),
    (
        icons::PIN_16_REGULAR,
        include_str!("../assets/icons/pin_16_regular.svg"),
    ),
    (
        icons::RENAME_16_REGULAR,
        include_str!("../assets/icons/rename_16_regular.svg"),
    ),
    (
        icons::SEARCH_16_REGULAR,
        include_str!("../assets/icons/search_16_regular.svg"),
    ),
    (
        icons::SETTINGS_16_REGULAR,
        include_str!("../assets/icons/settings_16_regular.svg"),
    ),
    (
        icons::SQUARE_MULTIPLE_16_REGULAR,
        include_str!("../assets/icons/square_multiple_16_regular.svg"),
    ),
    (
        icons::STAR_16_REGULAR,
        include_str!("../assets/icons/star_16_regular.svg"),
    ),
    (
        icons::TABLE_16_REGULAR,
        include_str!("../assets/icons/table_16_regular.svg"),
    ),
];

/// The asset source serving this crate's glyphs.
pub struct Assets;

impl Assets {
    /// One glyph's bytes, for a host chaining this into its own source.
    pub fn bytes(path: &str) -> Option<&'static [u8]> {
        FILES
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, contents)| contents.as_bytes())
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(Assets::bytes(path).map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(FILES
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::new_static(name))
            .collect())
    }
}
