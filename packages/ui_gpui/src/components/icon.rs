use gpui::{App, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, px, svg};

use crate::prelude::*;

/// Draws a glyph at a given size, in the surrounding text colour.
///
/// The set is Fluent, the same one `packages/ui_react` pulls from
/// `@fluentui/react-icons` — its components name specific glyphs
/// (`Checkmark12Filled`, `ChevronDown12Regular`), so the port names the same
/// ones rather than substituting a system symbol at a different weight. The
/// paths this crate carries are in [`crate::assets::icons`]; any other SVG a
/// host's asset source can resolve works just as well, which is the
/// counterpart of React's `IconGlyph` accepting any component that renders an
/// `<svg>`.
///
/// GPUI paints an SVG as a mask in the element's own text colour, so an icon
/// with no colour of its own reads the ambient one — the same thing
/// `fill: currentColor` does on the web.
#[derive(IntoElement, Clone)]
pub struct Icon {
    path: SharedString,
    /// Rendered edge length. Fixed-size glyphs are scaled to fit.
    size: Pixels,
    color: Option<Hsla>,
}

impl Icon {
    pub fn new(path: impl Into<SharedString>) -> Self {
        Icon {
            path: path.into(),
            size: px(20.),
            color: None,
        }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    /// Paint the glyph in a colour of its own rather than the ambient one.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or_else(|| window.text_style().color);

        svg()
            .flex_none()
            .size(self.size)
            .path(self.path)
            .text_color(color)
    }
}
