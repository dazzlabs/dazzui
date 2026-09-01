use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div};

use crate::prelude::*;
use crate::{ControlPart, ControlState, ControlVariant, Tint};

/// The micro-heading that organises a pane.
///
/// One size, sentence case, no added tracking: tone is the only axis worth
/// varying — the labelling ink by default, one step fainter for a nav group's
/// heading, and a tint's *text* grade when the label carries a status,
/// because a label is read, not pressed.
#[derive(IntoElement)]
pub struct SectionLabel {
    label: SharedString,
    /// Leave unset for the labelling ink. Set, it resolves the quiet accent
    /// recipe against that ramp — which is why it is optional rather than
    /// defaulted: an untinted label must keep the labelling ink rather than
    /// resolve against a ramp it never chose.
    tint: Option<Tint>,
    /// One step quieter — a nav group's heading rather than a pane's.
    faint: bool,
}

impl SectionLabel {
    pub fn new(label: impl Into<SharedString>) -> Self {
        SectionLabel {
            label: label.into(),
            tint: None,
            faint: false,
        }
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = Some(tint);
        self
    }

    pub fn faint(mut self, faint: bool) -> Self {
        self.faint = faint;
        self
    }
}

impl RenderOnce for SectionLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        // A tint outranks the faint tone, the way `[data-tint]` outranks
        // `[data-tone='faint']` further down the stylesheet.
        let color = match self.tint {
            Some(tint) => vars.control_color(
                ControlVariant::Plain,
                ControlPart::Content,
                tint,
                ControlState::Normal,
            ),
            None if self.faint => vars.color_content_faint,
            None => vars.color_content_subtle,
        };

        div()
            .flex_none()
            .type_style(vars.label_small)
            .text_color(color)
            .child(self.label)
    }
}
