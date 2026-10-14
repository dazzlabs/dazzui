//! The micro-heading that organises a pane.

use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};

use crate::foundation::{StyledExt, Tint};
use crate::theme::ActiveTheme;

/// One size, sentence case. Tone is the only axis: the labelling ink, one
/// step fainter for a nav group's heading, or a tint's text grade when the
/// label carries a status.
#[derive(IntoElement, Default)]
pub struct SectionLabel {
    tint: Option<Tint>,
    faint: bool,
    children: Vec<AnyElement>,
}

impl SectionLabel {
    pub fn new() -> Self {
        SectionLabel::default()
    }

    /// Leave unset for the labelling ink.
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = Some(tint);
        self
    }

    /// One step quieter — a nav group's heading rather than a pane's.
    pub fn faint(mut self, faint: bool) -> Self {
        self.faint = faint;
        self
    }
}

impl ParentElement for SectionLabel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for SectionLabel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let color = match self.tint {
            Some(tint) => t.control_color_plain_content_normal.resolve(tint.ramp(t)),
            None if self.faint => t.color_content_faint.hsla(),
            None => t.color_content_subtle.hsla(),
        };
        div()
            .flex()
            .type_style(t.label_small)
            .text_color(color)
            .children(self.children)
    }
}
