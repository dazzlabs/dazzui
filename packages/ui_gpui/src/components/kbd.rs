use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, relative,
};

use crate::prelude::*;
use crate::{colors, plus};

/// A key hint keeps its own three sizes: it annotates a row rather than
/// standing in one, so it never reaches control height.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KbdSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// `hint` is bare faint text — the default, because most keys shown in an
/// interface are asides. `strong` darkens it a step. `key` alone draws the
/// recessed cap.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KbdVariant {
    #[default]
    Hint,
    Strong,
    Key,
}

impl KbdVariant {
    pub const ALL: [KbdVariant; 3] = [KbdVariant::Hint, KbdVariant::Strong, KbdVariant::Key];
}

/// One key hint or cap. A chord is several of these, composed by the caller.
///
/// Three renderings of a key, quietest first. `hint` is the `⌘F` beside an
/// empty search field; `strong` is the same a step darker, for a hint that
/// has to survive on a busy row; `key` draws the cap — a recessed
/// chip-cornered fill with no border, because a key is pressed *into* the
/// surface rather than raised off it.
///
/// Its colours are already in the token graph, so it takes no tint, and it is
/// not interactive, so it stays out of the shared state machine.
#[derive(IntoElement)]
pub struct Kbd {
    keys: SharedString,
    size: KbdSize,
    variant: KbdVariant,
}

impl Kbd {
    pub fn new(keys: impl Into<SharedString>) -> Self {
        Kbd {
            keys: keys.into(),
            size: KbdSize::Medium,
            variant: KbdVariant::Hint,
        }
    }

    pub fn size(mut self, size: KbdSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: KbdVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let cap = self.variant == KbdVariant::Key;

        let font_size = match self.size {
            KbdSize::Large => vars.label_medium.size,
            _ => vars.label_small.size,
        };
        let weight = if cap {
            vars.label_strong.weight
        } else {
            vars.label_medium.weight
        };
        let content = match self.variant {
            KbdVariant::Hint => vars.color_content_faint,
            KbdVariant::Strong => vars.color_content_subtle,
            KbdVariant::Key => vars.color_content,
        };

        div()
            .flex()
            .flex_none()
            .items_center()
            // The display cut whatever the variant — a key is a glyph, not
            // copy.
            .type_style(vars.label_strong.size(font_size).weight(weight))
            .line_height(relative(1.0))
            .text_color(content)
            .when(cap, |el| {
                el.justify_center()
                    .h(plus(vars.spacing_5, vars.spacing_0_5))
                    .px(vars.spacing_2_5)
                    .rounded(vars.radius_tiny)
                    .bg(vars.color_surface_inset)
            })
            .when(!cap, |el| el.bg(colors::TRANSPARENT))
            .child(self.keys)
    }
}
