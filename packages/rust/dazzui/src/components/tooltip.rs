//! The label a control surfaces on hover. It never carries anything a
//! reader has to have: a tooltip is a reminder of what an icon means.

use std::{cell::Cell, rc::Rc, time::Duration};

use gpui::{
    AnyElement, App, Div, ElementId, Entity, FocusHandle, InteractiveElement, IntoElement,
    MouseButton, ParentElement, RenderOnce, Stateful, StatefulInteractiveElement, Styled,
    Subscription, Window, div, prelude::*, px,
};

use crate::components::collapsible::{kept_focus, part_id};
use crate::components::popup::{Align, Placement, PopupState, Side, measured, positioned};
use crate::foundation::press::{ButtonPress, OnClick};
use crate::foundation::{StyledExt, focus_visible};
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

/// A tooltip over its one child.
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    label: AnyElement,
    side: Side,
    align: Align,
    delay: Option<Duration>,
    close_delay: Option<Duration>,
    disabled: bool,
    child: Option<AnyElement>,
}

impl Tooltip {
    pub fn new(id: impl Into<ElementId>, label: impl IntoElement) -> Self {
        Tooltip {
            id: id.into(),
            label: crate::foundation::text_child(label),
            side: Side::Top,
            align: Align::Center,
            delay: None,
            close_delay: None,
            disabled: false,
            child: None,
        }
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// How long the pointer rests before it opens.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// How long it stays once the pointer leaves.
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.close_delay = Some(delay);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Attach to a control without inserting another layout box. Its focus,
    /// width and clipping still belong to the original control.
    pub(crate) fn bind(
        self,
        element: Stateful<Div>,
        focus: &FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let enabled = !self.disabled;
        self.bind_with(element, focus, window, cx, move |element, state| {
            element
                .when(enabled, |element| {
                    element.on_click(move |_, window, cx| {
                        state.update(cx, |state, cx| state.hide(window, cx))
                    })
                })
                .into_any_element()
        })
    }

    /// Close from the control's actual activation, including native Enter
    /// keydown and an armed Space release, rather than GPUI's keyup click.
    pub(crate) fn bind_button(
        self,
        element: Stateful<Div>,
        focus: &FocusHandle,
        press: ButtonPress,
        on_click: Option<OnClick>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let enabled = !self.disabled;
        self.bind_with(element, focus, window, cx, move |element, state| {
            press
                .bind(
                    element,
                    Some(Box::new(move |event, window, cx| {
                        if enabled {
                            state.update(cx, |state, cx| state.hide(window, cx));
                        }
                        if let Some(on_click) = &on_click {
                            on_click(event, window, cx);
                        }
                    })),
                )
                .into_any_element()
        })
    }

    fn bind_with(
        self,
        element: Stateful<Div>,
        focus: &FocusHandle,
        window: &mut Window,
        cx: &mut App,
        bind: impl FnOnce(Stateful<Div>, Entity<PopupState>) -> AnyElement,
    ) -> AnyElement {
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, false, window, cx);
        let watched = window.use_keyed_state(part_id(&self.id, "events"), cx, |window, cx| {
            let enabled = Rc::new(Cell::new(true));
            let focus_enabled = enabled.clone();
            let entered = state.downgrade();
            let enter = window.on_focus_in(focus, cx, move |window, cx| {
                if focus_enabled.get()
                    && window
                        .focused(cx)
                        .is_some_and(|focus| focus_visible(&focus, window, cx))
                {
                    let _ = entered.update(cx, |state, cx| {
                        state.cancel_hover();
                        state.open = true;
                        cx.notify();
                    });
                    refresh_after_focus(window, cx);
                }
            });
            let blur_enabled = enabled.clone();
            let blurred = state.downgrade();
            let blur = window.on_focus_out(focus, cx, move |_, window, cx| {
                if blur_enabled.get() {
                    let _ = blurred.update(cx, |state, cx| state.hide(window, cx));
                    refresh_after_focus(window, cx);
                }
            });
            let escape_enabled = enabled.clone();
            let escape = state.downgrade();
            let window_id = window.window_handle().window_id();
            let key = cx.intercept_keystrokes(move |event, window, cx| {
                if event.keystroke.key == "escape"
                    && window.window_handle().window_id() == window_id
                    && escape_enabled.get()
                {
                    let _ = escape.update(cx, |state, cx| state.hide(window, cx));
                }
            });
            TriggerEvents {
                enabled,
                _subscriptions: [enter, blur, key],
            }
        });
        watched.read(cx).enabled.set(!self.disabled);
        if self.disabled {
            state.update(cx, |state, cx| state.hide(window, cx));
        }
        let anchor = state.read(cx).anchor.clone();
        let open = state.read(cx).open && !self.disabled;
        let delay = self
            .delay
            .unwrap_or_else(|| Duration::from_secs_f64(f64::from(t.tooltip_open_delay) / 1000.));
        let close_delay = self
            .close_delay
            .unwrap_or_else(|| Duration::from_secs_f64(f64::from(t.tooltip_close_delay) / 1000.));

        let popup = open.then(|| {
            // `.dz-tooltip`: the raised card, a hairline, the small label.
            let label = div()
                .max_w(px(t.toast_max_width))
                .py(px(t.spacing_1 + t.spacing_0_5))
                .px(px(t.spacing_2))
                .bg(t.color_surface_raised.hsla())
                .border(px(t.stroke_hairline))
                .border_color(t.color_border.hsla())
                .rounded(px(t.radius_small))
                .shadow(shadows(t.shadow_sm))
                .text_color(t.color_content.hsla())
                .type_style(t.label_small)
                .child(self.label);
            positioned(&anchor, Placement::side(self.side, self.align, t), label)
        });

        let hover = state.clone();
        let press = state.clone();
        let element = element
            .when(!self.disabled, |this| {
                this.on_hover(move |hovered, window, cx| {
                    hover.update(cx, |state, cx| {
                        state.hover(0, *hovered, delay, close_delay, window, cx)
                    })
                })
                // A press is the control being used, not read about.
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    press.update(cx, |state, cx| state.hide(window, cx))
                })
            })
            .children(popup);
        measured(&anchor, bind(element, state)).into_any_element()
    }
}

struct TriggerEvents {
    enabled: Rc<Cell<bool>>,
    _subscriptions: [Subscription; 3],
}

// Focus changes arrive at the end of a draw. A refresh requested inside
// that phase is consumed by the frame that is already finishing.
fn refresh_after_focus(window: &Window, cx: &App) {
    window
        .spawn(cx, async move |cx| {
            let _ = cx.update(|window, _| window.refresh());
        })
        .detach();
}

impl ParentElement for Tooltip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        if let Some(element) = elements.into_iter().last() {
            self.child = Some(element);
        }
    }
}

impl RenderOnce for Tooltip {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = kept_focus(part_id(&self.id, "scope-focus"), window, cx);
        let element = div()
            .id(self.id.clone())
            .flex()
            .track_focus(&focus)
            .children(self.child.take());
        self.bind(element, &focus, window, cx)
    }
}
