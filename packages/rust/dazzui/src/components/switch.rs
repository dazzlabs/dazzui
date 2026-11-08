//! A setting that is on or off, and commits the moment it is pressed.

use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce,
    SharedString, Styled, Window, div, prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::foundation::motion::background_transition;
use crate::foundation::press::ButtonPress;
use crate::foundation::{ControlColors, Part, Size, State, Tint, Variant, pointer_down, recipe};
use crate::theme::ActiveTheme;

type OnCheckedChange = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// A track that carries the state and a thumb that sits at the end saying
/// which. Off is a faint thumb in the grey groove; on, the track takes the
/// filled recipe and the thumb its content colour.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    size: Size,
    tint: Tint,
    checked: Option<bool>,
    default_checked: bool,
    disabled: bool,
    on_checked_change: Option<OnCheckedChange>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Switch {
            id: id.into(),
            size: Size::Medium,
            tint: Tint::Primary,
            checked: None,
            default_checked: false,
            disabled: false,
            on_checked_change: None,
        }
    }

    /// `small`, `medium` or `large`; `tiny` draws as `small`.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn default_checked(mut self, checked: bool) -> Self {
        self.default_checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The accessible name. GPUI has no accessibility tree to give it to.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    pub fn on_checked_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = window.use_keyed_state(kit::child_id(&self.id, "checked"), cx, {
            let default = self.default_checked;
            move |_, _| default
        });
        let focus = kit::focus_handle(&self.id, window, cx);
        let activation = ButtonPress::new(&self.id, &focus, self.disabled, window, cx).non_native();
        let checked = self.checked.unwrap_or(*state.read(cx));
        let visible = !self.disabled && crate::foundation::focus_visible(&focus, window, cx);
        let (width, height, thumb) = match self.size {
            Size::Tiny | Size::Small => (
                t.switch_small_width,
                t.switch_small_height,
                t.switch_small_thumb,
            ),
            Size::Medium => (
                t.switch_medium_width,
                t.switch_medium_height,
                t.switch_medium_thumb,
            ),
            Size::Large => (
                t.switch_large_width,
                t.switch_large_height,
                t.switch_large_thumb,
            ),
        };
        let filled = ControlColors::resolve(t, Variant::Filled, self.tint, State::Normal);
        let modern = |part| {
            let opacity = recipe(t, Variant::Filled, part, State::Normal).opacity();
            checked && opacity > 0. && opacity < 1.
        };
        let thumb_id = kit::child_id(&self.id, "thumb");
        let thumb = background_transition(
            div()
                .id(thumb_id.clone())
                .flex_none()
                .size(px(thumb))
                .rounded(px(t.radius_full))
                .bg(if checked {
                    filled.content
                } else {
                    t.color_content_faint.hsla()
                }),
            thumb_id,
            t,
            modern(Part::Content),
            window,
            cx,
        );

        let mut track = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .when(checked, |this| this.justify_end())
            .w(px(width))
            .h(px(height))
            .p(px(t.spacing_0_5))
            .rounded(px(t.radius_full))
            .bg(if checked {
                filled.surface
            } else {
                t.color_surface_sunken.hsla()
            })
            .child(thumb)
            .when(visible, |this| this.child(kit::outline(t, height / 2.)));
        track = background_transition(track, self.id, t, modern(Part::Surface), window, cx);

        if self.disabled {
            track = track.opacity(0.6).cursor_not_allowed();
        } else {
            track = track
                .cursor_pointer()
                .track_focus(&focus.tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx));
        }
        let controlled = self.checked.is_some();
        let handler = self.on_checked_change;
        let change: crate::foundation::press::OnClick = Box::new(move |_, window, cx| {
            let next = !checked;
            if !controlled {
                state.update(cx, |value, cx| {
                    *value = next;
                    cx.notify();
                });
            }
            if let Some(handler) = &handler {
                handler(next, window, cx);
            }
        });
        activation.bind(track, Some(change))
    }
}
