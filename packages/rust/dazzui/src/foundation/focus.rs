//! The focus ring, drawn only when the keyboard put focus there — CSS's
//! `:focus-visible`, which GPUI does not have.
//!
//! The rule the browser uses is the one kept here: a key press makes focus
//! visible, a pointer press hides it again. [`init`](crate::init) watches the
//! keyboard; every pressable part reports its pointer through
//! [`pointer_down`].

use gpui::{
    AbsoluteLength, App, BoxShadow, Div, FocusHandle, Hsla, InteractiveElement, MouseButton,
    ParentElement, Stateful, Styled, Window, div, point, prelude::FluentBuilder, px,
};

use crate::theme::ActiveTheme;

use crate::generated::ThemeTokens;

#[derive(Default)]
struct Modality {
    keyboard: bool,
}

impl gpui::Global for Modality {}

pub(crate) fn init(cx: &mut App) {
    cx.set_global(Modality::default());
    cx.observe_keystrokes(|_, window, cx| {
        if !cx.global::<Modality>().keyboard {
            cx.global_mut::<Modality>().keyboard = true;
            window.refresh();
        }
    })
    .detach();
}

/// A pointer went down: focus it moves is not shown.
pub fn pointer_down(window: &mut Window, cx: &mut App) {
    if let Some(modality) = cx.try_global::<Modality>()
        && modality.keyboard
    {
        cx.global_mut::<Modality>().keyboard = false;
        window.refresh();
    }
}

/// Whether `handle` holds focus that the keyboard put there.
pub fn focus_visible(handle: &FocusHandle, window: &Window, cx: &App) -> bool {
    handle.is_focused(window) && cx.try_global::<Modality>().is_some_and(|m| m.keyboard)
}

/// `--focus-ring`: the system accent at the theme's alpha. It never follows
/// a control's tint.
pub fn ring_color(t: &ThemeTokens) -> Hsla {
    t.focus_ring.resolve(&t.color_primary)
}

/// The ring as a shadow hugging the box — `outline` at offset 0.
pub fn ring(t: &ThemeTokens) -> BoxShadow {
    BoxShadow {
        color: ring_color(t),
        offset: point(px(0.), px(0.)),
        blur_radius: px(0.),
        spread_radius: px(t.focus_width + t.focus_offset),
    }
}

/// A pressable part's focus: a tab stop, the pointer hiding the ring, and
/// the ring when the keyboard put focus there.
pub fn focusable(
    element: Stateful<Div>,
    focus: &FocusHandle,
    disabled: bool,
    window: &Window,
    cx: &App,
) -> Stateful<Div> {
    if disabled {
        return element;
    }
    let visible = focus_visible(focus, window, cx);
    let t = cx.theme();
    element
        .track_focus(&focus.clone().tab_stop(true))
        .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
        .when(visible, |this| with_outline(this, t))
}

/// CSS's `outline` at `--focus-offset`: a ring drawn around the box rather
/// than a shadow under it — a shadow fills whatever the box does not, so a
/// part with no fill of its own would fill with the ring's colour. The
/// parent must be `relative`.
pub fn outline(t: &ThemeTokens, radius: f32) -> Div {
    let out = -(t.focus_width + t.focus_offset);
    div()
        .absolute()
        .top(px(out))
        .left(px(out))
        .right(px(out))
        .bottom(px(out))
        .border(px(t.focus_width))
        .border_color(ring_color(t))
        .rounded(px(radius + t.focus_width + t.focus_offset))
}

/// The outline on a box, following the corner the box was already given.
pub fn with_outline<E: Styled + ParentElement>(mut element: E, t: &ThemeTokens) -> E {
    let radius = match element.style().corner_radii.top_left {
        Some(AbsoluteLength::Pixels(radius)) => f32::from(radius),
        Some(AbsoluteLength::Rems(rems)) => rems.0 * 16.0,
        None => 0.0,
    };
    element.relative().child(outline(t, radius))
}
