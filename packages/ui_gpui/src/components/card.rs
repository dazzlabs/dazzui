use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};

use crate::prelude::*;
use crate::{Tint, hairline};

/// The three card densities. A card is not a control, so it has its own scale
/// rather than the control profiles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl CardSize {
    pub const ALL: [CardSize; 3] = [CardSize::Small, CardSize::Medium, CardSize::Large];
}

/// `raised` is the paper panel, `sunken` the workaday card one step off it,
/// `outlined` the drawn box, and `tinted` the washed one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Raised,
    Sunken,
    Outlined,
    Tinted,
}

impl CardVariant {
    pub const ALL: [CardVariant; 4] = [
        CardVariant::Raised,
        CardVariant::Sunken,
        CardVariant::Outlined,
        CardVariant::Tinted,
    ];
}

/// The filled or bordered box the panels in a layout are built from.
///
/// A card is not interactive, so it stays out of the shared state machine
/// entirely and reads the resting surfaces directly — a panel that lit up
/// under the pointer would read as something you can press.
#[derive(IntoElement)]
pub struct Card {
    size: CardSize,
    variant: CardVariant,
    /// Which ramp `tinted` resolves against; ignored by the other variants.
    tint: Tint,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Card {
            size: CardSize::Medium,
            variant: CardVariant::Raised,
            tint: Tint::Primary,
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

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
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        let gap = match self.size {
            CardSize::Small => vars.spacing_2,
            CardSize::Medium => vars.spacing_3,
            CardSize::Large => vars.spacing_4,
        };
        let padding_block = match self.size {
            CardSize::Small | CardSize::Medium => vars.spacing_3,
            CardSize::Large => vars.spacing_4,
        };
        // The medium card's padding is asymmetric — a hair more inline than
        // block — which keeps a one-line row inside it optically centred.
        let padding_inline = match self.size {
            CardSize::Small => vars.spacing_3,
            CardSize::Medium => vars.spacing_3_5,
            CardSize::Large => vars.spacing_4,
        };
        let radius = match self.size {
            CardSize::Small => vars.radius_medium,
            _ => vars.radius_large,
        };

        let surface = match self.variant {
            CardVariant::Raised => vars.color_surface,
            // The workaday card: one step off the paper, the surface most
            // content sits on.
            CardVariant::Sunken => vars.color_surface_muted,
            CardVariant::Outlined => crate::colors::TRANSPARENT,
            // Tinted takes the surface wash rather than the `tinted` recipe's
            // chip fill: a card is the largest thing a tint washes here, and
            // the chip's 12% fills it instead of tinting it.
            CardVariant::Tinted => vars.wash(self.tint, vars.wash_surface),
        };
        let border = match self.variant {
            // The matching wash, so the pair moves together when either is
            // retuned.
            CardVariant::Tinted => vars.wash(self.tint, vars.wash_edge),
            _ => vars.color_border,
        };

        div()
            .flex()
            .flex_col()
            .gap(gap)
            .py(padding_block)
            .px(padding_inline)
            .rounded(radius)
            // Drawn on every variant rather than only where it shows, so
            // `outlined` and `tinted` do not gain a pixel per side and sit a
            // hair larger than the filled ones.
            .border(hairline(vars, window))
            .bg(surface)
            .border_color(border)
            .children(self.children)
    }
}
