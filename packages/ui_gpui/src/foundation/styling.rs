use gpui::{BoxShadow, Hsla, IntoElement, Pixels, Styled, Window, div};

use crate::foundation::{ControlColors, TokenShadow, TypeStyle};
use crate::generated::theme_variables::ThemeVariables;

/// The token vocabulary as element builders.
///
/// These are what a component reads instead of restating a value: a face, a
/// shadow stack, the three colours a recipe resolves to. Everything here goes
/// through the theme, so nothing in `components/` holds a design value.
pub trait StyledExt: Styled + Sized {
    /// A whole face — family, size, weight and leading — the way
    /// `font: var(--label-medium-*)` sets one.
    fn type_style(self, style: TypeStyle) -> Self {
        self.font_family(style.font_family())
            .text_size(style.size)
            .font_weight(style.weight)
            .line_height(style.line_height)
    }

    /// A `shadow` token, layers and all.
    fn token_shadow(self, shadows: &'static [TokenShadow]) -> Self {
        self.shadow(shadows.iter().map(BoxShadow::from).collect())
    }

    /// The three colours a control paints, for one state.
    fn control_colors(self, colors: ControlColors) -> Self {
        self.bg(colors.surface)
            .text_color(colors.content)
            .border_color(colors.border)
    }
}

impl<E: Styled> StyledExt for E {}

/// The keyboard focus ring, as a child to overlay on a focused control.
///
/// `:focus-visible { outline: 3px solid var(--focus-ring); outline-offset: 0 }`
/// — it hugs the control with no gap, the way macOS draws it, and it is the
/// system accent in every tint: it never follows `data-tint`.
///
/// GPUI has no outline, and a border would eat into the box, so the ring is
/// an absolutely positioned sibling inset by its own width. The control it
/// sits in has to be `relative()`, which every component that draws one is.
pub fn focus_ring(vars: &ThemeVariables, radius: Pixels) -> impl IntoElement {
    let width = vars.focus_width;

    div()
        .absolute()
        .inset(-(width + vars.focus_offset))
        .border(width)
        .border_color(vars.focus_ring())
        .rounded(radius + width + vars.focus_offset)
}

/// A separator is one *device* pixel.
///
/// A 1px line on a Retina display is twice the weight of the real thing, so
/// the token halves — the same rule `base.css` applies behind its
/// `min-resolution: 2dppx` block. Drawn control outlines read
/// `stroke_control` instead and stay put.
pub fn hairline(vars: &ThemeVariables, window: &Window) -> Pixels {
    vars.stroke_hairline * (1.0 / window.scale_factor().max(1.0))
}

/// A rule pinned to one edge, drawn at a hairline.
///
/// A border would take part in the box, and a separator between rows is
/// drawn *on* them; this is the `::after` a stylesheet would reach for.
pub fn rule(color: Hsla, edge: Edge, weight: Pixels) -> impl IntoElement {
    let line = div().absolute().bg(color);
    match edge {
        Edge::Top => line.top_0().left_0().right_0().h(weight),
        Edge::Bottom => line.bottom_0().left_0().right_0().h(weight),
        Edge::Left => line.top_0().bottom_0().left_0().w(weight),
        Edge::Right => line.top_0().bottom_0().right_0().w(weight),
    }
}

/// Which edge a [`rule`] hangs from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}
