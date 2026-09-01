use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, relative,
};

use crate::prelude::*;
use crate::{ControlPart, ControlState, ControlVariant, Tint, colors};

/// The `control.color.*` recipes that read as a label rather than a control,
/// plus the one a badge owns: `raised`, the paper chip that sits on a grey
/// card.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    Filled,
    #[default]
    Tinted,
    Outlined,
    Raised,
}

impl BadgeVariant {
    pub const ALL: [BadgeVariant; 4] = [
        BadgeVariant::Filled,
        BadgeVariant::Tinted,
        BadgeVariant::Outlined,
        BadgeVariant::Raised,
    ];

    /// The control recipe this variant wears, where it wears one.
    fn recipe(self) -> Option<ControlVariant> {
        match self {
            BadgeVariant::Filled => Some(ControlVariant::Filled),
            BadgeVariant::Tinted => Some(ControlVariant::Tinted),
            BadgeVariant::Outlined => Some(ControlVariant::Outlined),
            BadgeVariant::Raised => None,
        }
    }
}

/// A badge is an annotation, not a control, so it has its own three sizes and
/// never reaches control height.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl BadgeSize {
    pub const ALL: [BadgeSize; 3] = [BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large];
}

/// A tag: the display cut at label weight in a tight chip-cornered capsule.
///
/// Its label has to read as an annotation rather than as body copy, which is
/// why the face and weight never soften whatever the size. A badge is not
/// interactive, so it deliberately skips the shared state machine and reads
/// the resting recipes directly — a label that changed colour under the
/// pointer would read as something you can press.
#[derive(IntoElement)]
pub struct Badge {
    label: SharedString,
    size: BadgeSize,
    variant: BadgeVariant,
    /// Which colour ramp the variant resolves against.
    tint: Tint,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Badge {
            label: label.into(),
            size: BadgeSize::Medium,
            variant: BadgeVariant::Tinted,
            tint: Tint::Primary,
        }
    }

    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let resting = |part: ControlPart| {
            self.variant
                .recipe()
                .map(|variant| vars.control_color(variant, part, self.tint, ControlState::Normal))
        };

        // Two capsule densities on the chip corner; the type moves one step
        // at the top.
        let padding_block = match self.size {
            BadgeSize::Large => vars.spacing_1,
            _ => vars.spacing_0_5,
        };
        let padding_inline = match self.size {
            BadgeSize::Small => vars.spacing_1_5,
            _ => vars.spacing_2,
        };
        let font_size = match self.size {
            BadgeSize::Large => vars.label_medium.size,
            _ => vars.label_small.size,
        };

        let surface = match self.variant {
            // The badge that sits on a grey card: paper fill, quiet ink.
            BadgeVariant::Raised => vars.color_surface,
            BadgeVariant::Outlined => colors::TRANSPARENT,
            _ => resting(ControlPart::Surface).unwrap_or(colors::TRANSPARENT),
        };
        let content = match self.variant {
            BadgeVariant::Raised => vars.color_content_muted,
            _ => resting(ControlPart::Content).unwrap_or(vars.color_content),
        };
        let border = match self.variant {
            BadgeVariant::Outlined => resting(ControlPart::Border).unwrap_or(colors::TRANSPARENT),
            _ => colors::TRANSPARENT,
        };

        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .py(padding_block)
            .px(padding_inline)
            .rounded(vars.radius_tiny)
            // A border on every variant rather than only on `outlined`, so
            // that one does not gain a pixel per side and fall off the
            // heights the others share.
            .border(vars.spacing_px)
            .bg(surface)
            .text_color(content)
            .border_color(border)
            .type_style(
                vars.label_strong
                    .size(font_size)
                    .weight(vars.label_medium.weight),
            )
            .line_height(relative(1.4))
            .child(self.label)
    }
}
