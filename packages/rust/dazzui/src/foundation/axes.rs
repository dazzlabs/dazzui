//! The axes a control is drawn along — the `data-*` attributes React's
//! stylesheets select on — and the recipe table they pick a colour from.

use gpui::Hsla;

use crate::generated::ThemeTokens;
use crate::tokens::{Paint, Ramp, TypeStyle};

/// Which ramp a recipe resolves against: React's `data-tint`.
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

    pub fn ramp(self, t: &ThemeTokens) -> &Ramp {
        match self {
            Tint::Primary => &t.color_primary,
            Tint::Neutral => &t.color_neutral,
            Tint::Info => &t.color_info,
            Tint::Success => &t.color_success,
            Tint::Warning => &t.color_warning,
            Tint::Danger => &t.color_danger,
        }
    }

    /// The tint React writes, `primary` … `danger`.
    pub fn parse(name: &str) -> Option<Tint> {
        Some(match name {
            "primary" => Tint::Primary,
            "neutral" => Tint::Neutral,
            "info" => Tint::Info,
            "success" => Tint::Success,
            "warning" => Tint::Warning,
            "danger" => Tint::Danger,
            _ => return None,
        })
    }
}

/// A control size: React's `data-size`, which binds the `control.<size>.*`
/// profile.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Size {
    Tiny,
    #[default]
    Small,
    Medium,
    Large,
}

impl Size {
    pub fn parse(name: &str) -> Option<Size> {
        Some(match name {
            "tiny" => Size::Tiny,
            "small" => Size::Small,
            "medium" => Size::Medium,
            "large" => Size::Large,
            _ => return None,
        })
    }

    /// What `[data-size]` binds onto `--control-*`.
    pub fn profile(self, t: &ThemeTokens) -> Profile {
        match self {
            Size::Tiny => Profile {
                size: t.control_tiny_size,
                icon_size: t.control_tiny_icon_size,
                gap: t.control_tiny_gap,
                padding_block: t.control_tiny_padding_block,
                padding_inline: t.control_tiny_padding_inline,
                radius: t.control_tiny_radius,
                content: t.control_tiny_content,
            },
            Size::Small => Profile {
                size: t.control_small_size,
                icon_size: t.control_small_icon_size,
                gap: t.control_small_gap,
                padding_block: t.control_small_padding_block,
                padding_inline: t.control_small_padding_inline,
                radius: t.control_small_radius,
                content: t.control_small_content,
            },
            Size::Medium => Profile {
                size: t.control_medium_size,
                icon_size: t.control_medium_icon_size,
                gap: t.control_medium_gap,
                padding_block: t.control_medium_padding_block,
                padding_inline: t.control_medium_padding_inline,
                radius: t.control_medium_radius,
                content: t.control_medium_content,
            },
            Size::Large => Profile {
                size: t.control_large_size,
                icon_size: t.control_large_icon_size,
                gap: t.control_large_gap,
                padding_block: t.control_large_padding_block,
                padding_inline: t.control_large_padding_inline,
                radius: t.control_large_radius,
                content: t.control_large_content,
            },
        }
    }
}

/// A size profile, resolved: `--control-size`, `--control-gap` and the rest.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Profile {
    pub size: f32,
    pub icon_size: f32,
    pub gap: f32,
    pub padding_block: f32,
    pub padding_inline: f32,
    pub radius: f32,
    pub content: TypeStyle,
}

/// A recipe: which row of `control.color` a control reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Variant {
    #[default]
    Recessed,
    Normal,
    Filled,
    Tinted,
    Outlined,
    Plain,
}

impl Variant {
    pub fn parse(name: &str) -> Option<Variant> {
        Some(match name {
            "recessed" => Variant::Recessed,
            "normal" => Variant::Normal,
            "filled" => Variant::Filled,
            "tinted" => Variant::Tinted,
            "outlined" => Variant::Outlined,
            "plain" => Variant::Plain,
            _ => return None,
        })
    }
}

/// The part of a control a recipe cell colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Part {
    Surface,
    Content,
    Border,
}

/// The state a recipe cell is for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum State {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

impl State {
    pub const ALL: [State; 4] = [
        State::Normal,
        State::Hovered,
        State::Pressed,
        State::Disabled,
    ];
}

/// One recipe cell: `control.color.<variant>.<part>.<state>`.
pub fn recipe(t: &ThemeTokens, variant: Variant, part: Part, state: State) -> Paint {
    macro_rules! row {
        ($($part:ident $state:ident => $field:ident),* $(,)?) => {
            match (part, state) {
                $((Part::$part, State::$state) => t.$field,)*
            }
        };
    }
    match variant {
        Variant::Recessed => row! {
            Surface Normal => control_color_recessed_surface_normal,
            Surface Hovered => control_color_recessed_surface_hovered,
            Surface Pressed => control_color_recessed_surface_pressed,
            Surface Disabled => control_color_recessed_surface_disabled,
            Content Normal => control_color_recessed_content_normal,
            Content Hovered => control_color_recessed_content_hovered,
            Content Pressed => control_color_recessed_content_pressed,
            Content Disabled => control_color_recessed_content_disabled,
            Border Normal => control_color_recessed_border_normal,
            Border Hovered => control_color_recessed_border_hovered,
            Border Pressed => control_color_recessed_border_pressed,
            Border Disabled => control_color_recessed_border_disabled,
        },
        Variant::Normal => row! {
            Surface Normal => control_color_normal_surface_normal,
            Surface Hovered => control_color_normal_surface_hovered,
            Surface Pressed => control_color_normal_surface_pressed,
            Surface Disabled => control_color_normal_surface_disabled,
            Content Normal => control_color_normal_content_normal,
            Content Hovered => control_color_normal_content_hovered,
            Content Pressed => control_color_normal_content_pressed,
            Content Disabled => control_color_normal_content_disabled,
            Border Normal => control_color_normal_border_normal,
            Border Hovered => control_color_normal_border_hovered,
            Border Pressed => control_color_normal_border_pressed,
            Border Disabled => control_color_normal_border_disabled,
        },
        Variant::Filled => row! {
            Surface Normal => control_color_filled_surface_normal,
            Surface Hovered => control_color_filled_surface_hovered,
            Surface Pressed => control_color_filled_surface_pressed,
            Surface Disabled => control_color_filled_surface_disabled,
            Content Normal => control_color_filled_content_normal,
            Content Hovered => control_color_filled_content_hovered,
            Content Pressed => control_color_filled_content_pressed,
            Content Disabled => control_color_filled_content_disabled,
            Border Normal => control_color_filled_border_normal,
            Border Hovered => control_color_filled_border_hovered,
            Border Pressed => control_color_filled_border_pressed,
            Border Disabled => control_color_filled_border_disabled,
        },
        Variant::Tinted => row! {
            Surface Normal => control_color_tinted_surface_normal,
            Surface Hovered => control_color_tinted_surface_hovered,
            Surface Pressed => control_color_tinted_surface_pressed,
            Surface Disabled => control_color_tinted_surface_disabled,
            Content Normal => control_color_tinted_content_normal,
            Content Hovered => control_color_tinted_content_hovered,
            Content Pressed => control_color_tinted_content_pressed,
            Content Disabled => control_color_tinted_content_disabled,
            Border Normal => control_color_tinted_border_normal,
            Border Hovered => control_color_tinted_border_hovered,
            Border Pressed => control_color_tinted_border_pressed,
            Border Disabled => control_color_tinted_border_disabled,
        },
        Variant::Outlined => row! {
            Surface Normal => control_color_outlined_surface_normal,
            Surface Hovered => control_color_outlined_surface_hovered,
            Surface Pressed => control_color_outlined_surface_pressed,
            Surface Disabled => control_color_outlined_surface_disabled,
            Content Normal => control_color_outlined_content_normal,
            Content Hovered => control_color_outlined_content_hovered,
            Content Pressed => control_color_outlined_content_pressed,
            Content Disabled => control_color_outlined_content_disabled,
            Border Normal => control_color_outlined_border_normal,
            Border Hovered => control_color_outlined_border_hovered,
            Border Pressed => control_color_outlined_border_pressed,
            Border Disabled => control_color_outlined_border_disabled,
        },
        Variant::Plain => row! {
            Surface Normal => control_color_plain_surface_normal,
            Surface Hovered => control_color_plain_surface_hovered,
            Surface Pressed => control_color_plain_surface_pressed,
            Surface Disabled => control_color_plain_surface_disabled,
            Content Normal => control_color_plain_content_normal,
            Content Hovered => control_color_plain_content_hovered,
            Content Pressed => control_color_plain_content_pressed,
            Content Disabled => control_color_plain_content_disabled,
            Border Normal => control_color_plain_border_normal,
            Border Hovered => control_color_plain_border_hovered,
            Border Pressed => control_color_plain_border_pressed,
            Border Disabled => control_color_plain_border_disabled,
        },
    }
}

/// A control's three colours in one state: what `data-control` puts on
/// `--control-surface`, `--control-content` and `--control-border`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlColors {
    pub surface: Hsla,
    pub content: Hsla,
    pub border: Hsla,
}

impl ControlColors {
    pub fn resolve(t: &ThemeTokens, variant: Variant, tint: Tint, state: State) -> ControlColors {
        let ramp = tint.ramp(t);
        ControlColors {
            surface: recipe(t, variant, Part::Surface, state).resolve(ramp),
            content: recipe(t, variant, Part::Content, state).resolve(ramp),
            border: recipe(t, variant, Part::Border, state).resolve(ramp),
        }
    }
}
