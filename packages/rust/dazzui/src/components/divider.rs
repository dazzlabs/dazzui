//! A hairline between two things.

use gpui::{AlignSelf, App, IntoElement, RenderOnce, Styled, Window, div, px};

use crate::theme::ActiveTheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DividerTone {
    #[default]
    Default,
    /// The heavier rule, `border-strong`, for a separator that has to hold
    /// its own against a filled surface.
    Muted,
}

impl DividerTone {
    pub fn parse(name: &str) -> Option<DividerTone> {
        Some(match name {
            "default" => DividerTone::Default,
            "muted" => DividerTone::Muted,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl DividerOrientation {
    pub fn parse(name: &str) -> Option<DividerOrientation> {
        Some(match name {
            "horizontal" => DividerOrientation::Horizontal,
            "vertical" => DividerOrientation::Vertical,
            _ => return None,
        })
    }
}

/// A rule: the full width when horizontal, the full height of the row it
/// stands in when vertical.
#[derive(IntoElement, Default)]
pub struct Divider {
    tone: DividerTone,
    orientation: DividerOrientation,
}

impl Divider {
    pub fn new() -> Self {
        Divider::default()
    }

    pub fn tone(mut self, tone: DividerTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn orientation(mut self, orientation: DividerOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn vertical(self) -> Self {
        self.orientation(DividerOrientation::Vertical)
    }
}

impl RenderOnce for Divider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let color = match self.tone {
            DividerTone::Default => t.color_border,
            DividerTone::Muted => t.color_border_strong,
        };
        let rule = div().flex_none().bg(color.hsla());
        match self.orientation {
            DividerOrientation::Horizontal => rule.w_full().h(px(t.stroke_hairline)),
            DividerOrientation::Vertical => {
                let mut rule = rule.w(px(t.stroke_hairline));
                rule.style().align_self = Some(AlignSelf::Stretch);
                rule
            }
        }
    }
}
