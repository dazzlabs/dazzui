//! The glyphs the kit draws, served to GPUI's asset loader.

use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

use crate::glyphs::SVGS;

/// Serves the kit's own glyphs under `dazzui/`. An app with glyphs of its
/// own wraps this in its source and falls through to it.
pub struct Assets;

impl Assets {
    /// A kit glyph's SVG, by its asset path.
    pub fn get(path: &str) -> Option<&'static str> {
        SVGS.iter()
            .find(|(name, _)| *name == path)
            .map(|(_, svg)| *svg)
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Assets::get(path).map(|svg| Cow::Borrowed(svg.as_bytes())))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(SVGS
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}
