//! The few things every component's box does the same way.

use gpui::{Div, Hsla, InteractiveElement, Stateful, StatefulInteractiveElement, Styled, px};

use crate::foundation::axes::{ControlColors, Profile, State, Tint, Variant};
use crate::generated::ThemeTokens;
use crate::tokens::{Face, TypeStyle};

/// The height of an inline box set in `face` at `size`: the face's ascent
/// and descent, each whole, as a browser sizes a `<span>` in a line — the
/// box a run of type has when it is not a block with a line height of its
/// own.
pub fn content_area(face: Face, size: f32, window: &gpui::Window) -> f32 {
    let text = window.text_system();
    let id = text.resolve_font(&gpui::font(face.family()));
    let size = px(size);
    let ascent = f32::from(text.ascent(id, size)).abs().round();
    let descent = f32::from(text.descent(id, size)).abs().round();
    ascent + descent
}

/// A border's width as a browser draws it: one of a pixel or more is
/// floored to whole pixels, so `--stroke-control`'s 1.5 is a 1px edge on
/// the web, and a box it bounds is no wider than a hairline's.
pub fn border_width(width: f32) -> f32 {
    if width >= 1.0 { width.floor() } else { width }
}

/// Where the baseline of type set in `face` at `size` falls in a line
/// `line_height` tall, from its top: half the leading, then the ascent —
/// what a browser aligns when a row says `align-items: baseline`, and GPUI
/// does not measure.
pub fn baseline(face: Face, size: f32, line_height: f32, window: &gpui::Window) -> f32 {
    let text = window.text_system();
    let id = text.resolve_font(&gpui::font(face.family()));
    let ascent = f32::from(text.ascent(id, px(size))).abs().round();
    let descent = f32::from(text.descent(id, px(size))).abs().round();
    ((line_height - ascent - descent) / 2.0).floor() + ascent
}

/// Styling a box from tokens rather than from numbers.
pub trait StyledExt: Styled + Sized {
    /// A typography token: face, size, leading and weight together.
    fn type_style(self, style: TypeStyle) -> Self {
        self.font_family(style.face.family())
            .text_size(px(style.size))
            .line_height(px(style.line_height))
            .font_weight(style.font_weight())
    }

    /// The box `[data-size]` binds: height, gap, padding, corner and type.
    fn profile(self, profile: Profile) -> Self {
        self.h(px(profile.size))
            .gap(px(profile.gap))
            .py(px(profile.padding_block))
            .px(px(profile.padding_inline))
            .rounded(px(profile.radius))
            .type_style(profile.content)
    }

    /// One recipe state's colours on the box.
    fn control_colors(self, colors: ControlColors) -> Self {
        self.bg(colors.surface)
            .text_color(colors.content)
            .border_color(colors.border)
    }
}

impl<T: Styled> StyledExt for T {}

/// The shared control state machine — React's `data-control` plus
/// `data-variant`: the recipe's surface, content and border, tracking hover,
/// press and disabled on their own. Content flows to text and to every
/// [`Icon`](crate::Icon) inside, since both read the inherited colour.
pub fn control(
    element: Stateful<Div>,
    t: &ThemeTokens,
    variant: Variant,
    tint: Tint,
    disabled: bool,
) -> Stateful<Div> {
    let colors = |state| ControlColors::resolve(t, variant, tint, state);
    if disabled {
        return element.control_colors(colors(State::Disabled));
    }
    let hovered = colors(State::Hovered);
    let pressed = colors(State::Pressed);
    element
        .control_colors(colors(State::Normal))
        .hover(move |style| style.control_colors(hovered))
        .active(move |style| style.control_colors(pressed))
}

/// A colour at a fraction of its alpha.
pub fn fade(color: Hsla, alpha: f32) -> Hsla {
    Hsla {
        a: color.a * alpha,
        ..color
    }
}
