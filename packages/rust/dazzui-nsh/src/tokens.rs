//! The value types the generated theme is written in.
//!
//! `generated/theme_tokens.rs` names these and nothing else, so what a token
//! *is* on Native Shell is decided here, once: a colour is four bytes, a length is
//! logical pixels, a type style is a face and three numbers, and a recipe
//! cell is a [`Paint`] that may still be waiting for its tint.

use nsh_core::{
    BoxShadow,
    color::{Hsla, rgba},
    element::FontWeight,
    geometry::px,
};

use crate::generated::ThemeTokens;

/// An sRGB colour as `0xrrggbbaa`, the order [`nsh_core::color::rgba`] reads.
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
    pub fn family(self) -> &'static str {
        match self {
            Face::Text | Face::Display => "system-ui",
            // Blink's macOS ui-monospace resolves to Menlo. NSH's generic
            // monospace instead chooses the system monospaced design.
            Face::Mono => {
                if cfg!(target_os = "macos") {
                    "Menlo"
                } else {
                    "ui-monospace, Menlo, monospace"
                }
            }
        }
    }
}

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
        FontWeight(self.weight)
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
            color: self.color.hsla().into(),
            offset_x: px(self.x),
            offset_y: px(self.y),
            blur: px(self.blur),
            spread: px(self.spread),
            inset: false,
        }
    }
}

/// A shadow token's layers, as Native Shell draws them.
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
