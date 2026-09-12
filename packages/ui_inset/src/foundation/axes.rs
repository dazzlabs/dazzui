use super::ColorExt;
use inset_embedder::Color;

use crate::foundation::{ColorDescriptor, ColorRecipe, ColorSwatch, TypeStyle};
use crate::generated::theme_variables::ThemeVariables;

// The axes a component varies along. Each is the Rust spelling of a `data-*`
// attribute in `packages/ui_react`, and the vocabulary is the ds one: the
// colour axis is `tint`, never `kind` or `secondary`.

/// The ramps a control can be tinted with — one per semantic role in
/// `tokens/global/color.yaml`.
///
/// Interactive colours are stored as recipes rather than finished colours, so
/// a control does not know its own hue until it is tinted. This is the choice
/// of ramp those recipes are resolved against.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Tint {
    #[default]
    Primary,
    Neutral,
    Info,
    Success,
    Warning,
    Danger,
}

impl Tint {
    pub const ALL: [Tint; 6] = [
        Tint::Primary,
        Tint::Neutral,
        Tint::Info,
        Tint::Success,
        Tint::Warning,
        Tint::Danger,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Tint::Primary => "primary",
            Tint::Neutral => "neutral",
            Tint::Info => "info",
            Tint::Success => "success",
            Tint::Warning => "warning",
            Tint::Danger => "danger",
        }
    }
}

/// The `control.color.*` recipes, which is the whole set: `recessed` is the
/// workaday grey chip and the default, `filled` the one action a view points
/// at, `normal` the raised white neighbour, `tinted` a quiet accent chip,
/// `outlined` its drawn twin, and `plain` text alone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ControlVariant {
    #[default]
    Recessed,
    Normal,
    Filled,
    Tinted,
    Outlined,
    Plain,
}

impl ControlVariant {
    pub const ALL: [ControlVariant; 6] = [
        ControlVariant::Recessed,
        ControlVariant::Normal,
        ControlVariant::Filled,
        ControlVariant::Tinted,
        ControlVariant::Outlined,
        ControlVariant::Plain,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            ControlVariant::Recessed => "recessed",
            ControlVariant::Normal => "normal",
            ControlVariant::Filled => "filled",
            ControlVariant::Tinted => "tinted",
            ControlVariant::Outlined => "outlined",
            ControlVariant::Plain => "plain",
        }
    }
}

/// The four control sizes, each a profile of geometry and type in
/// `tokens/global/control.yaml`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ControlSize {
    Tiny,
    #[default]
    Small,
    Medium,
    Large,
}

impl ControlSize {
    pub const ALL: [ControlSize; 4] = [
        ControlSize::Tiny,
        ControlSize::Small,
        ControlSize::Medium,
        ControlSize::Large,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            ControlSize::Tiny => "tiny",
            ControlSize::Small => "small",
            ControlSize::Medium => "medium",
            ControlSize::Large => "large",
        }
    }
}

/// The interaction states the shared state machine resolves a recipe for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ControlState {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

/// The three parts a recipe paints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ControlPart {
    Surface,
    Content,
    Border,
}

/// A control's three resolved colours for one state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlColors {
    pub surface: Color,
    pub content: Color,
    pub border: Color,
}

/// The geometry and type one `[data-size]` profile binds.
///
/// A sized control reads this once instead of restating geometry per size —
/// Button and IconButton keep no per-size branch at all, because the profile
/// carries the design's own control type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlMetrics {
    pub size: f64,
    pub gap: f64,
    pub padding_block: f64,
    pub padding_inline: f64,
    pub radius: f64,
    pub content: TypeStyle,
}

impl ThemeVariables {
    /// The ramp a tint selects — the Rust half of `control.css`'s
    /// `[data-tint] { --tint-600: var(--color-primary-600) }`.
    pub fn ramp(&self, tint: Tint) -> ColorSwatch {
        match tint {
            Tint::Primary => self.color_primary,
            Tint::Neutral => self.color_neutral,
            Tint::Info => self.color_info,
            Tint::Success => self.color_success,
            Tint::Warning => self.color_warning,
            Tint::Danger => self.color_danger,
        }
    }

    /// The size profile `[data-size]` binds.
    pub fn metrics(&self, size: ControlSize) -> ControlMetrics {
        match size {
            ControlSize::Tiny => ControlMetrics {
                size: self.control_tiny_size,
                gap: self.control_tiny_gap,
                padding_block: self.control_tiny_padding_block,
                padding_inline: self.control_tiny_padding_inline,
                radius: self.control_tiny_radius,
                content: self.control_tiny_content,
            },
            ControlSize::Small => ControlMetrics {
                size: self.control_small_size,
                gap: self.control_small_gap,
                padding_block: self.control_small_padding_block,
                padding_inline: self.control_small_padding_inline,
                radius: self.control_small_radius,
                content: self.control_small_content,
            },
            ControlSize::Medium => ControlMetrics {
                size: self.control_medium_size,
                gap: self.control_medium_gap,
                padding_block: self.control_medium_padding_block,
                padding_inline: self.control_medium_padding_inline,
                radius: self.control_medium_radius,
                content: self.control_medium_content,
            },
            ControlSize::Large => ControlMetrics {
                size: self.control_large_size,
                gap: self.control_large_gap,
                padding_block: self.control_large_padding_block,
                padding_inline: self.control_large_padding_inline,
                radius: self.control_large_radius,
                content: self.control_large_content,
            },
        }
    }

    /// The recipe for one variant and part.
    pub fn descriptor(&self, variant: ControlVariant, part: ControlPart) -> ColorDescriptor {
        match (variant, part) {
            (ControlVariant::Normal, ControlPart::Surface) => self.control_color_normal_surface,
            (ControlVariant::Normal, ControlPart::Content) => self.control_color_normal_content,
            (ControlVariant::Normal, ControlPart::Border) => self.control_color_normal_border,
            (ControlVariant::Recessed, ControlPart::Surface) => self.control_color_recessed_surface,
            (ControlVariant::Recessed, ControlPart::Content) => self.control_color_recessed_content,
            (ControlVariant::Recessed, ControlPart::Border) => self.control_color_recessed_border,
            (ControlVariant::Filled, ControlPart::Surface) => self.control_color_filled_surface,
            (ControlVariant::Filled, ControlPart::Content) => self.control_color_filled_content,
            (ControlVariant::Filled, ControlPart::Border) => self.control_color_filled_border,
            (ControlVariant::Tinted, ControlPart::Surface) => self.control_color_tinted_surface,
            (ControlVariant::Tinted, ControlPart::Content) => self.control_color_tinted_content,
            (ControlVariant::Tinted, ControlPart::Border) => self.control_color_tinted_border,
            (ControlVariant::Outlined, ControlPart::Surface) => self.control_color_outlined_surface,
            (ControlVariant::Outlined, ControlPart::Content) => self.control_color_outlined_content,
            (ControlVariant::Outlined, ControlPart::Border) => self.control_color_outlined_border,
            (ControlVariant::Plain, ControlPart::Surface) => self.control_color_plain_surface,
            (ControlVariant::Plain, ControlPart::Content) => self.control_color_plain_content,
            (ControlVariant::Plain, ControlPart::Border) => self.control_color_plain_border,
        }
    }

    /// One state of a recipe.
    pub fn state_recipe(&self, descriptor: ColorDescriptor, state: ControlState) -> ColorRecipe {
        match state {
            ControlState::Normal => descriptor.normal,
            ControlState::Hovered => descriptor.hovered,
            ControlState::Pressed => descriptor.pressed,
            ControlState::Disabled => descriptor.disabled,
        }
    }

    /// Resolve one recipe against a ramp and a state.
    ///
    /// This is what the token build does for CSS in `plugins/control.mjs`,
    /// done at read time instead: a state names either a semantic colour,
    /// which is the same in every tint, or a shade off the tinted ramp — then
    /// the alpha applies to whichever it landed on.
    pub fn resolve_recipe(
        &self,
        descriptor: ColorDescriptor,
        tint: Tint,
        state: ControlState,
    ) -> Color {
        self.state_recipe(descriptor, state)
            .resolve(self.ramp(tint))
    }

    /// The colour one variant paints one part in, in one state — the Rust
    /// half of `--control-surface` / `--control-content` / `--control-border`.
    pub fn control_color(
        &self,
        variant: ControlVariant,
        part: ControlPart,
        tint: Tint,
        state: ControlState,
    ) -> Color {
        self.resolve_recipe(self.descriptor(variant, part), tint, state)
    }

    /// The three colours a control paints in one state.
    pub fn control_colors(
        &self,
        variant: ControlVariant,
        tint: Tint,
        state: ControlState,
    ) -> ControlColors {
        ControlColors {
            surface: self.control_color(variant, ControlPart::Surface, tint, state),
            content: self.control_color(variant, ControlPart::Content, tint, state),
            border: self.control_color(variant, ControlPart::Border, tint, state),
        }
    }

    /// A tint washed over a surface — the alpha of the ramp's 600 that the
    /// design reaches for wherever a tint has to tint something rather than
    /// become it.
    pub fn wash(&self, tint: Tint, alpha: f64) -> Color {
        self.ramp(tint).shade(600).opacity(alpha)
    }

    /// Keyboard focus: the system accent at a per-theme alpha, and never the
    /// control's own tint. macOS draws it hugging the control, offset 0.
    pub fn focus_ring(&self) -> Color {
        self.color_primary
            .shade(self.focus_ring_shade)
            .opacity(self.focus_ring_alpha)
    }
}
