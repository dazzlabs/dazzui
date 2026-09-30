//! The example's assets: the glyphs the stories name, under `glyph/`, and
//! the kit's own beneath them.

use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

use crate::lib::glyphs::GLYPHS;

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(name) = path.strip_prefix("glyph/").and_then(|p| p.strip_suffix("")) {
            if let Some((_, svg)) = GLYPHS.iter().find(|(glyph, _)| *glyph == name) {
                return Ok(Some(Cow::Borrowed(svg.as_bytes())));
            }
        }
        dazzui::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        dazzui::Assets.list(path)
    }
}
