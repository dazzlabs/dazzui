use gpui::{App, IntoElement, RenderOnce, Styled, Window, div};

use crate::prelude::*;

/// How hard the rule has to work.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerTone {
    #[default]
    Default,
    /// The heavier rule, for a separator that has to hold its own against a
    /// filled surface. It is `border-strong` — ink at a tenth — and
    /// deliberately not `border-muted`: that role is the drawn outline of a
    /// checkbox or a radio, an opaque grey around twice this weight, and a
    /// separator set in it stops reading as a hairline and starts reading as
    /// a border.
    Muted,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// A rule.
#[derive(IntoElement)]
pub struct Divider {
    tone: DividerTone,
    orientation: DividerOrientation,
}

impl Divider {
    pub fn horizontal() -> Self {
        Divider {
            tone: DividerTone::Default,
            orientation: DividerOrientation::Horizontal,
        }
    }

    pub fn vertical() -> Self {
        Divider {
            tone: DividerTone::Default,
            orientation: DividerOrientation::Vertical,
        }
    }

    pub fn tone(mut self, tone: DividerTone) -> Self {
        self.tone = tone;
        self
    }
}

impl RenderOnce for Divider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let color = match self.tone {
            DividerTone::Default => vars.color_border,
            DividerTone::Muted => vars.color_border_strong,
        };

        div()
            .flex_none()
            .bg(color)
            .map(|el| match self.orientation {
                DividerOrientation::Horizontal => el.w_full().h(vars.spacing_px),
                // `align-self: stretch` on the web; a row stretches its
                // children by default here, so the rule only has to fill it.
                DividerOrientation::Vertical => el.h_full().w(vars.spacing_px),
            })
    }
}
