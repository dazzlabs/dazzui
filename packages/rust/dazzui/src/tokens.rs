//! The value types the generated theme is written in.
//!
//! `generated/theme_tokens.rs` names these and nothing else, so what a token
//! *is* on GPUI is decided here, once: a colour is four bytes, a length is
//! logical pixels, a type style is a face and three numbers, and a recipe
//! cell is a [`Paint`] that may still be waiting for its tint.

use gpui::{BoxShadow, FontWeight, Hsla, point, px, rgba};

use crate::generated::ThemeTokens;

/// An sRGB colour as `0xrrggbbaa`, the order [`gpui::rgba`] reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color(pub u32);

impl Color {
    pub const TRANSPARENT: Color = Color(0);

    pub fn hsla(self) -> Hsla {
        rgba(self.0).into()
    }

    /// The colour with its alpha multiplied by `alpha`.
    pub fn alpha(self, alpha: f32) -> Hsla {
        let mut color = self.hsla();
        color.a *= alpha;
        color
    }

    /// CSS `color-mix(in srgb, self weight, other)`, including alpha.
    pub fn mix(self, other: Color, weight: f32) -> Hsla {
        let a = gpui::Rgba::from(self.hsla());
        let b = gpui::Rgba::from(other.hsla());
        let weight = weight.clamp(0., 1.);
        let alpha = a.a * weight + b.a * (1. - weight);
        if alpha == 0. {
            return gpui::transparent_black();
        }
        let channel = |a_channel, b_channel| {
            (a_channel * a.a * weight + b_channel * b.a * (1. - weight)) / alpha
        };
        gpui::Rgba {
            r: channel(a.r, b.r),
            g: channel(a.g, b.g),
            b: channel(a.b, b.b),
            a: alpha,
        }
        .into()
    }
}

impl From<Color> for Hsla {
    fn from(color: Color) -> Hsla {
        color.hsla()
    }
}

/// The eleven steps a ramp holds, as the CSS names them.
pub const SHADES: [u16; 11] = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

/// A role ladder: `color.primary.50` … `color.primary.950`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ramp(pub [Color; 11]);

impl Ramp {
    /// The step a recipe names. A shade the ladder does not have is a
    /// generator bug, not a value to guess at.
    pub fn shade(&self, shade: u16) -> Color {
        let index = SHADES
            .iter()
            .position(|step| *step == shade)
            .unwrap_or_else(|| panic!("no shade {shade} on a ramp"));
        self.0[index]
    }
}

/// A recipe cell: a shade off whichever ramp the control is tinted with, or
/// a colour of its own — each at an alpha.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Paint {
    Shade(u16, f32),
    Color(Color, f32),
}

impl Paint {
    /// The finished colour, against the ramp the control is tinted with.
    pub fn resolve(self, ramp: &Ramp) -> Hsla {
        match self {
            Paint::Shade(shade, alpha) => ramp.shade(shade).alpha(alpha),
            Paint::Color(color, alpha) => color.alpha(alpha),
        }
    }

    /// The alpha alone — a press dims by it.
    pub fn opacity(self) -> f32 {
        match self {
            Paint::Shade(_, alpha) | Paint::Color(_, alpha) => alpha,
        }
    }
}

/// Which of the design's faces a style is set in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Face {
    /// The UI face: SF Pro Text in the design, the platform's own here.
    Text,
    /// The display cut, for headings and strong labels. On a Mac the system
    /// face picks its optical size by point size, so this is the same family.
    Display,
    /// `base.font.code`.
    Mono,
}

impl Face {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    pub(crate) fn families(self) -> &'static [&'static str] {
        match self {
            Face::Text => crate::generated::FONT_UI,
            Face::Display => crate::generated::FONT_DISPLAY,
            Face::Mono => crate::generated::FONT_CODE,
        }
    }

    /// The family GPUI resolves. `.SystemUIFont` is GPUI's name for the
    /// platform's UI face, which is what `-apple-system` asks for on the web.
    pub fn family(self) -> &'static str {
        #[cfg(target_os = "macos")]
        if self == Face::Mono {
            return mac_code_family();
        }
        #[cfg(any(target_os = "linux", target_os = "windows"))]
        return crate::platform_fonts::family(self == Face::Mono);
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        match self {
            Face::Text | Face::Display => ".SystemUIFont",
            Face::Mono => MONO_FAMILY,
        }
    }
}

#[cfg(target_os = "macos")]
const MONO_FAMILY: &str = "Menlo";

#[cfg(target_os = "macos")]
fn mac_code_family() -> &'static str {
    static FAMILY: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
    FAMILY.get_or_init(|| {
        crate::generated::FONT_CODE
            .iter()
            .copied()
            .find(|name| {
                core_text::font::new_from_name(name, 12.)
                    .is_ok_and(|font| font.family_name() == *name)
            })
            .unwrap_or(MONO_FAMILY)
    })
}
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
const MONO_FAMILY: &str = "monospace";

/// A typography token: `labelMedium`, `control.small.content`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeStyle {
    pub face: Face,
    pub size: f32,
    pub line_height: f32,
    pub weight: u16,
}

impl TypeStyle {
    pub fn font_weight(self) -> FontWeight {
        FontWeight(self.weight as f32)
    }

    /// The same style set in another face — code set at a label's size.
    pub fn in_face(self, face: Face) -> TypeStyle {
        TypeStyle { face, ..self }
    }
}

/// One layer of an `effect.shadow` token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shadow {
    pub color: Color,
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
}

impl Shadow {
    pub fn box_shadow(self) -> BoxShadow {
        BoxShadow {
            color: self.color.hsla(),
            offset: point(px(self.x), px(self.y)),
            // CSS stores the blur length; GPUI's shader consumes Gaussian
            // sigma directly. CSS sigma is half that length.
            blur_radius: px(self.blur / 2.),
            spread_radius: px(self.spread),
        }
    }
}

/// A shadow token's layers, as GPUI draws them.
pub fn shadows(layers: &[Shadow]) -> Vec<BoxShadow> {
    layers.iter().map(|layer| layer.box_shadow()).collect()
}

/// A theme by the name React selects it with.
#[derive(Debug)]
pub struct ThemeEntry {
    pub name: &'static str,
    pub dark: bool,
    pub tokens: &'static ThemeTokens,
}
