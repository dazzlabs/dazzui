//! The filled or bordered box the panels in a layout are built from.

use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};

use crate::foundation::{Size, Tint};
use crate::theme::ActiveTheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CardVariant {
    #[default]
    Raised,
    /// The workaday card: one step off the paper, with the hairline.
    Sunken,
    Outlined,
    /// The tint's surface wash, edged with its matching wash.
    Tinted,
}

impl CardVariant {
    pub fn parse(name: &str) -> Option<CardVariant> {
        Some(match name {
            "raised" => CardVariant::Raised,
            "sunken" => CardVariant::Sunken,
            "outlined" => CardVariant::Outlined,
            "tinted" => CardVariant::Tinted,
            _ => return None,
        })
    }
}

/// A column with a corner, a fill and a hairline. The corner is the
/// container step at every size: a card grows with what it holds.
#[derive(IntoElement)]
pub struct Card {
    size: Size,
    variant: CardVariant,
    tint: Tint,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Card {
            size: Size::Medium,
            variant: CardVariant::Raised,
            tint: Tint::Primary,
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Which ramp `tinted` resolves against; ignored by the other variants.
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Card::new()
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let (padding_block, padding_inline, gap) = match self.size {
            Size::Tiny | Size::Small => (t.spacing_3, t.spacing_3, t.spacing_2),
            // A hair more inline than block, which keeps a one-line row
            // inside optically centred.
            Size::Medium => (t.spacing_3, t.spacing_3_5, t.spacing_3),
            Size::Large => (t.spacing_4, t.spacing_4, t.spacing_4),
        };
        let border = t.color_border.hsla();
        let (surface, edge) = match self.variant {
            CardVariant::Raised => (t.color_surface.hsla(), border),
            CardVariant::Sunken => (t.color_surface_muted.hsla(), border),
            CardVariant::Outlined => (gpui::transparent_black(), border),
            CardVariant::Tinted => {
                let tint = self.tint.ramp(t).shade(600);
                (tint.alpha(t.wash_surface), tint.alpha(t.wash_edge))
            }
        };
        div()
            .flex()
            .flex_col()
            .border(px(t.stroke_hairline))
            .border_color(edge)
            .bg(surface)
            .py(px(padding_block))
            .px(px(padding_inline))
            .gap(px(gap))
            .rounded(px(t.radius_large))
            .children(self.children)
    }
}
