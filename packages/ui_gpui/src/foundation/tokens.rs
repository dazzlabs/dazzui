use gpui::{BoxShadow, FontWeight, Hsla, Pixels, point};

use crate::generated::colors;

// The hand-written half of the token vocabulary: the types `generated/` is
// written against. Nothing here holds a design value — every value lives in
// `packages/ds` and arrives through the generator.

/// The family a mono face resolves to.
///
/// `base.font.code` leads its stack with `ui-monospace`, which is a request
/// for the platform's monospaced face rather than a family to look up by
/// name. GPUI has a name for the system UI face (`.SystemUIFont`) but none
/// for this one, so the stack's next real family is named instead — Menlo
/// ships with every macOS.
const MONOSPACE_FAMILY: &str = "Menlo";

/// The family a UI face resolves to.
///
/// `.SystemUIFont` is GPUI's own name for whatever the platform's UI face is,
/// which is what a stack leading with `-apple-system` is asking for.
const SYSTEM_FAMILY: &str = ".SystemUIFont";

/// A colour ramp — a role ladder indexed by shade.
///
/// Reading it as a lightness ramp is the mistake it invites: Bright's ink
/// ladder puts a *lighter* colour at 700 than at 600, because its primary
/// action is ink navy whose hover lifts. A step only ever means the role the
/// recipe reaching for it gives it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSwatch(pub &'static [(u16, Hsla)]);

impl ColorSwatch {
    /// The step at `shade`.
    ///
    /// A ramp is generated with every step present, so a missing one is a
    /// generator bug rather than a case to design around — it resolves to
    /// transparent instead of panicking.
    pub const fn shade(&self, shade: u16) -> Hsla {
        let mut index = 0;
        while index < self.0.len() {
            if self.0[index].0 == shade {
                return self.0[index].1;
            }
            index += 1;
        }
        colors::TRANSPARENT
    }

    /// Every step, palest first — what a ramp row in a gallery draws.
    pub fn steps(&self) -> &'static [(u16, Hsla)] {
        self.0
    }
}

/// One interaction state of a recipe: a colour or a shade, plus an alpha.
///
/// A control does not know its own hue until it is tinted, so a state carries
/// either a literal colour (a semantic role, the same in every tint) or a
/// shade to pick off whichever ramp the control is tinted with.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorRecipe {
    pub color: Option<Hsla>,
    pub shade: Option<u16>,
    pub alpha: Option<f32>,
}

impl ColorRecipe {
    /// Resolve against the ramp a control is tinted with.
    pub fn resolve(&self, ramp: ColorSwatch) -> Hsla {
        let seed = match (self.color, self.shade) {
            (Some(color), _) => color,
            (None, Some(shade)) => ramp.shade(shade),
            (None, None) => colors::TRANSPARENT,
        };
        match self.alpha {
            // The alpha scales what is already there rather than replacing
            // it, which is what `color-mix(… , transparent)` does on the CSS
            // side. Ramp steps are opaque, so the two only differ where a
            // recipe names a semantic role that already carries an alpha.
            Some(alpha) => seed.opacity(alpha),
            None => seed,
        }
    }
}

/// An interactive colour, stored as a *recipe* rather than a finished colour.
///
/// [`ThemeVariables::resolve_recipe`] is where a state and a tint turn it
/// into a colour.
///
/// [`ThemeVariables::resolve_recipe`]: crate::ThemeVariables::resolve_recipe
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorDescriptor {
    pub normal: ColorRecipe,
    pub hovered: ColorRecipe,
    pub pressed: ColorRecipe,
    pub disabled: ColorRecipe,
}

/// A type face: a family stack, a size, a weight, and an absolute leading.
///
/// The leading is kept absolute rather than as a multiple because that is how
/// the design states it — a label face sets its line-height to its own size
/// so its text sits on the centre of a 26px box, and a multiple would round
/// that away.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeStyle {
    pub family: &'static [&'static str],
    pub size: Pixels,
    pub weight: FontWeight,
    pub line_height: Pixels,
}

impl TypeStyle {
    /// The family this stack resolves to.
    ///
    /// Every face in the graph leads its stack with a generic — `-apple-system`
    /// for the UI faces, `ui-monospace` for the code one — which is a request
    /// for a platform face rather than a family to look up by name. Only a
    /// face naming a real family falls through.
    pub fn font_family(&self) -> &'static str {
        match self.family.first() {
            None => SYSTEM_FAMILY,
            Some(&"-apple-system" | &"system-ui" | &"sans-serif") => SYSTEM_FAMILY,
            Some(&"ui-monospace" | &"monospace") => MONOSPACE_FAMILY,
            Some(family) => family,
        }
    }

    /// The same face at another weight — what a selected row takes.
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// The same face at another size — what a badge takes at its top size.
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }
}

/// One shadow layer.
///
/// Named against the token rather than against GPUI's [`BoxShadow`], which
/// carries a colour it has already resolved; this is the layer a `shadow`
/// token describes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TokenShadow {
    pub x: Pixels,
    pub y: Pixels,
    pub blur: Pixels,
    pub spread: Pixels,
    pub color: Hsla,
}

impl From<&TokenShadow> for BoxShadow {
    fn from(shadow: &TokenShadow) -> Self {
        BoxShadow {
            color: shadow.color,
            offset: point(shadow.x, shadow.y),
            // GPUI's `blur_radius` is the CSS blur length rather than half of
            // it, so the token passes through as it is written.
            blur_radius: shadow.blur,
            spread_radius: shadow.spread,
        }
    }
}

/// A CSS `cubic-bezier(…)`, kept as data so the motion token can name one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BezierCurve {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl BezierCurve {
    /// The curve as an easing function over a normalised time.
    ///
    /// A cubic bézier gives `y` for an `x` that has to be solved for, so this
    /// walks Newton's method the way every browser does. GPUI's animations
    /// take a plain `fn(f32) -> f32`, which is what this is for.
    pub fn ease(&self, t: f32) -> f32 {
        let curve = *self;
        let x = |u: f32| bezier(u, curve.x1, curve.x2);
        let y = |u: f32| bezier(u, curve.y1, curve.y2);

        let mut guess = t;
        for _ in 0..8 {
            let error = x(guess) - t;
            if error.abs() < 1e-4 {
                break;
            }
            let slope = bezier_slope(guess, curve.x1, curve.x2);
            if slope.abs() < 1e-6 {
                break;
            }
            guess -= error / slope;
        }
        y(guess.clamp(0.0, 1.0))
    }
}

fn bezier(t: f32, a: f32, b: f32) -> f32 {
    let inverse = 1.0 - t;
    3.0 * inverse * inverse * t * a + 3.0 * inverse * t * t * b + t * t * t
}

fn bezier_slope(t: f32, a: f32, b: f32) -> f32 {
    let inverse = 1.0 - t;
    3.0 * inverse * inverse * a + 6.0 * inverse * t * (b - a) + 3.0 * t * t * (1.0 - b)
}

/// A length that is not on the spacing scale, composed out of steps that are.
///
/// The 18px a dialog body pads by is `plus(spacing_4, spacing_0_5)` — a step
/// and a half, which says what it is where `px(18.)` only says what it
/// measures.
pub fn plus(a: Pixels, b: Pixels) -> Pixels {
    a + b
}

/// Half a step — a bar's corner, a thumb's inset.
pub fn half(a: Pixels) -> Pixels {
    gpui::Half::half(&a)
}
