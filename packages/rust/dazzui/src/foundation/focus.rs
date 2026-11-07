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
use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
};

use crate::theme::ActiveTheme;

use crate::generated::ThemeTokens;

#[derive(Default)]
struct Modality {
    keyboard: bool,
}

impl gpui::Global for Modality {}

#[derive(Default)]
struct ModalScopes(Vec<Weak<ModalScope>>);
impl gpui::Global for ModalScopes {}

/// A modal includes its detached popup subtrees as well as its own sheet.
pub(crate) struct ModalScope {
    pub focus: FocusHandle,
    window: gpui::WindowId,
    active: Cell<bool>,
    popups: RefCell<Vec<gpui::WeakFocusHandle>>,
}

impl ModalScope {
    pub fn new(focus: FocusHandle, window: &Window) -> Rc<Self> {
        Rc::new(Self {
            focus,
            window: window.window_handle().window_id(),
            active: Cell::new(false),
            popups: RefCell::default(),
        })
    }
    pub fn set_open(self: &Rc<Self>, open: bool, cx: &mut App) {
        if self.active.replace(open) == open {
            return;
        }
        let scopes = &mut cx.default_global::<ModalScopes>().0;
        scopes.retain(|scope| {
            scope
                .upgrade()
                .is_some_and(|scope| !Rc::ptr_eq(&scope, self))
        });
        if open {
            scopes.push(Rc::downgrade(self));
        }
    }
    pub fn add_popup(&self, focus: &FocusHandle) {
        let mut popups = self.popups.borrow_mut();
        popups.retain(|popup| popup.upgrade().is_some());
        if !popups.iter().any(|popup| popup == focus) {
            popups.push(focus.downgrade());
        }
    }
    fn contains(&self, window: &Window, cx: &App) -> bool {
        self.focus.contains_focused(window, cx)
            || self.popups.borrow().iter().any(|popup| {
                popup
                    .upgrade()
                    .is_some_and(|popup| popup.contains_focused(window, cx))
            })
    }
}

fn active_modal(window: &Window, cx: &App) -> Option<Rc<ModalScope>> {
    cx.try_global::<ModalScopes>()?
        .0
        .iter()
        .rev()
        .filter_map(Weak::upgrade)
        .find(|scope| scope.active.get() && scope.window == window.window_handle().window_id())
}

pub(crate) fn tab(backwards: bool, window: &mut Window, cx: &App) {
    if let Some(scope) = active_modal(window, cx) {
        cycle(
            &scope.focus,
            |window| scope.contains(window, cx),
            backwards,
            window,
            cx,
        );
    } else if backwards {
        window.focus_prev();
    } else {
        window.focus_next();
    }
}

pub(crate) fn init(cx: &mut App) {
    if cx.has_global::<Modality>() {
        return;
    }
    cx.set_global(Modality::default());
    // A handled arrow/Space key is still keyboard input. Watching only
    // unhandled events leaves a pointer-focused control without its ring.
    cx.intercept_keystrokes(|_, window, cx| keyboard_down(window, cx))
        .detach();
    cx.observe_keystrokes(|event, window, cx| {
        let stroke = &event.keystroke;
        // GPUI keeps a tab order but does not traverse it automatically.
        // Only supply the fallback after the host/field handled its keys.
        if event.action.is_none()
            && stroke.key == "tab"
            && !window.default_prevented()
            && !stroke.modifiers.control
            && !stroke.modifiers.alt
            && !stroke.modifiers.platform
        {
            tab(stroke.modifiers.shift, window, cx);
            window.prevent_default();
        }
    })
    .detach();
}

pub(crate) fn keyboard_down(window: &mut Window, cx: &mut App) {
    if let Some(modality) = cx.try_global::<Modality>()
        && !modality.keyboard
    {
        cx.global_mut::<Modality>().keyboard = true;
        window.refresh();
    }
}

/// Native traversal restricted to a modal's focus subtree. Non-tab-stop
/// containers and disabled controls remain the native tab map's concern.
pub(crate) fn cycle_in(scope: &FocusHandle, backwards: bool, window: &mut Window, cx: &App) {
    cycle(
        scope,
        |window| scope.contains_focused(window, cx),
        backwards,
        window,
        cx,
    );
}

fn cycle(
    scope: &FocusHandle,
    contains: impl Fn(&Window) -> bool,
    backwards: bool,
    window: &mut Window,
    cx: &App,
) {
    let mut seen = Vec::new();
    loop {
        if backwards {
            window.focus_prev();
        } else {
            window.focus_next();
        }
        let next = window.focused(cx);
        if contains(window) && !scope.is_focused(window) {
            return;
        }
        if seen.contains(&next) {
            window.focus(scope);
            return;
        }
        seen.push(next);
    }
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
