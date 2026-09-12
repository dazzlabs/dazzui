use inset_embedder::{Color, FontWeight};
use inset_painting::TextStyle;

pub const fn px(value: f64) -> f64 {
    value
}
pub trait ColorExt {
    fn opacity(self, alpha: f64) -> Self;
}
impl ColorExt for Color {
    fn opacity(self, alpha: f64) -> Self {
        Self {
            a: self.a * alpha,
            ..self
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSwatch(pub &'static [(u16, Color)]);
impl ColorSwatch {
    pub const fn shade(&self, shade: u16) -> Color {
        let mut i = 0;
        while i < self.0.len() {
            if self.0[i].0 == shade {
                return self.0[i].1;
            }
            i += 1;
        }
        panic!("missing generated ramp shade")
    }
    pub fn steps(&self) -> &'static [(u16, Color)] {
        self.0
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorRecipe {
    pub color: Option<Color>,
    pub shade: Option<u16>,
    pub alpha: Option<f64>,
}
impl ColorRecipe {
    pub fn resolve(self, ramp: ColorSwatch) -> Color {
        let color = self.color.unwrap_or_else(|| {
            self.shade
                .map(|s| ramp.shade(s))
                .unwrap_or(Color::from_argb(0, 0, 0, 0))
        });
        color.opacity(self.alpha.unwrap_or(1.0))
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorDescriptor {
    pub normal: ColorRecipe,
    pub hovered: ColorRecipe,
    pub pressed: ColorRecipe,
    pub disabled: ColorRecipe,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeStyle {
    pub family: &'static [&'static str],
    pub size: f64,
    pub weight: FontWeight,
    pub line_height: f64,
}
impl TypeStyle {
    pub fn text_style(self) -> TextStyle {
        let mut style = TextStyle::new()
            .font_size(self.size)
            .font_weight(self.weight)
            .height(self.line_height / self.size);
        // Leave the UI family unset so DefaultTextStyle supplies the host face.
        if let Some(family) = self.family.first() {
            if *family == "ui-monospace" {
                style = style.font_family("monospace");
            } else if !["-apple-system", "system-ui", "sans-serif"].contains(family) {
                style = style.font_family(*family);
            }
        }
        style
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TokenShadow {
    pub x: f64,
    pub y: f64,
    pub blur: f64,
    pub spread: f64,
    pub color: Color,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BezierCurve {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl From<&TokenShadow> for inset_painting::BoxShadow {
    fn from(s: &TokenShadow) -> Self {
        Self::new(
            s.color,
            inset_embedder::Offset::new(s.x, s.y),
            s.blur,
            s.spread,
            inset_embedder::BlurStyle::Normal,
        )
    }
}
pub fn shadows(layers: &[TokenShadow]) -> Vec<inset_painting::BoxShadow> {
    layers.iter().map(Into::into).collect()
}
