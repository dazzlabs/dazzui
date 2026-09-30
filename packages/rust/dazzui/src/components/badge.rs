//! A status or a count, read at a glance and never pressed.

use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};

use crate::components::icon::icon_slot;
use crate::foundation::{Size, Tint};
use crate::theme::ActiveTheme;

/// The `control.color.*` recipes that read as a label rather than a control.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BadgeVariant {
    Filled,
    #[default]
    Tinted,
    Outlined,
    /// The badge that sits on a grey card: paper fill, quiet ink.
    Raised,
}

impl BadgeVariant {
    pub fn parse(name: &str) -> Option<BadgeVariant> {
        Some(match name {
            "filled" => BadgeVariant::Filled,
            "tinted" => BadgeVariant::Tinted,
            "outlined" => BadgeVariant::Outlined,
            "raised" => BadgeVariant::Raised,
            _ => return None,
        })
    }
}

/// A tag: the display cut at label weight in a tight chip-cornered capsule.
/// It reads its recipe at rest only — it is not a control, so nothing
/// changes under the pointer.
#[derive(IntoElement)]
pub struct Badge {
    size: Size,
    variant: BadgeVariant,
    tint: Tint,
    children: Vec<AnyElement>,
}

impl Badge {
    pub fn new() -> Self {
        Badge {
            size: Size::Medium,
            variant: BadgeVariant::Tinted,
            tint: Tint::Primary,
            children: Vec::new(),
        }
    }

    /// `small`, `medium` or `large`; a badge has no `tiny`.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Which ramp the variant resolves against; `raised` ignores it.
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl Default for Badge {
    fn default() -> Self {
        Badge::new()
    }
}

impl ParentElement for Badge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Badge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let ramp = self.tint.ramp(t);
        let (padding_block, padding_inline, font_size) = match self.size {
            Size::Tiny | Size::Small => (t.spacing_0_5, t.spacing_1_5, t.label_small.size),
            Size::Medium => (t.spacing_0_5, t.spacing_2, t.label_small.size),
            Size::Large => (t.spacing_1, t.spacing_2, t.label_medium.size),
        };
        let transparent = gpui::transparent_black();
        let (surface, content, border) = match self.variant {
            BadgeVariant::Filled => (
                t.control_color_filled_surface_normal.resolve(ramp),
                t.control_color_filled_content_normal.resolve(ramp),
                transparent,
            ),
            BadgeVariant::Tinted => (
                t.control_color_tinted_surface_normal.resolve(ramp),
                t.control_color_tinted_content_normal.resolve(ramp),
                transparent,
            ),
            BadgeVariant::Outlined => (
                transparent,
                t.control_color_outlined_content_normal.resolve(ramp),
                t.control_color_outlined_border_normal.resolve(ramp),
            ),
            BadgeVariant::Raised => (
                t.color_surface.hsla(),
                t.color_content_muted.hsla(),
                transparent,
            ),
        };
        icon_slot(
            t.icon_small,
            div()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .whitespace_nowrap()
                .border(px(t.spacing_px))
                .border_color(border)
                .bg(surface)
                .text_color(content)
                .py(px(padding_block))
                .px(px(padding_inline))
                .rounded(px(t.radius_tiny))
                .font_family(t.label_strong.face.family())
                .font_weight(t.label_medium.font_weight())
                .text_size(px(font_size))
                .line_height(px(font_size * 1.4))
                .children(self.children),
        )
    }
}
