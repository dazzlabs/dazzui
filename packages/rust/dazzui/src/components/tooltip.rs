//! The label a control surfaces on hover or keyboard focus. It never carries
//! anything essential: it is a reminder of what an icon means.

use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
    time::Duration,
};

use gpui::{
    AnyElement, App, Bounds, Div, Element, ElementId, Entity, FocusHandle, GlobalElementId,
    InspectorElementId, InteractiveElement, IntoElement, LayoutId, MouseButton, ParentElement,
    Pixels, RenderOnce, Stateful, StatefulInteractiveElement, Styled, Subscription, Visibility,
    Window, div, prelude::*, px,
};

use crate::components::collapsible::{kept_focus, part_id};
use crate::components::popup::{Align, Placement, PopupState, Side, measured, positioned};
use crate::foundation::press::{ActivationObserver, ButtonPress, OnClick};
use crate::foundation::{StyledExt, focus_visible, provide, scoped};
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
        child: Option<AnyElement>,
        focus: &FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let enabled = !self.disabled;
        self.bind_with(element, focus, window, cx, move |element, state| {
            let attached = Rc::new(Cell::new(false));
            let activated = state.clone();
            let observer = ActivationObserver {
                attached: attached.clone(),
                on_click: Rc::new(move |_, window, cx| {
                    if enabled {
                        activated.update(cx, |state, cx| state.hide(window, cx));
                    }
                }),
            };
            element
                .children(child.map(|child| provide(observer, child)))
                .when(enabled, |element| {
                    element.on_click(move |event, window, cx| {
                        if !event.is_keyboard() || !attached.get() {
                            state.update(cx, |state, cx| state.hide(window, cx));
                        }
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
        let parent = scoped::<TooltipScope>(cx).unwrap_or_default();
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
                    refresh_after_draw(window, cx);
                }
            });
            let blur_enabled = enabled.clone();
            let blurred = state.downgrade();
            let blur = window.on_focus_out(focus, cx, move |_, window, cx| {
                if blur_enabled.get() {
                    let _ = blurred.update(cx, |state, cx| state.hide(window, cx));
                    refresh_after_draw(window, cx);
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
                enabled: enabled.clone(),
                nested: Rc::new(NestedHints {
                    registrations: RefCell::default(),
                    enabled: enabled.clone(),
                }),
                _registration: parent.0.as_ref().map(|parent| parent.register(window, cx)),
                _subscriptions: [enter, blur, key],
            }
        });
        let enabled = watched.read(cx).enabled.clone();
        let nested = watched.read(cx).nested.clone();
        enabled.set(!self.disabled && !nested.any());
        if !enabled.get() {
            state.update(cx, |state, cx| state.hide(window, cx));
        }
        let anchor = state.read(cx).anchor.clone();
        let open = state.read(cx).open && enabled.get();
        let delay = self
            .delay
            .unwrap_or_else(|| Duration::from_secs_f64(f64::from(t.tooltip_open_delay) / 1000.));
        let close_delay = self
            .close_delay
            .unwrap_or_else(|| Duration::from_secs_f64(f64::from(t.tooltip_close_delay) / 1000.));

        let popup = open.then(|| {
            let hidden = nested.clone();
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
                .map_paint_style(move |style, _, _| {
                    if hidden.any() {
                        style.visibility = Visibility::Hidden;
                    }
                })
                .child(self.label);
            provide(
                parent,
                positioned(&anchor, Placement::side(self.side, self.align, t), label),
            )
            .into_any_element()
        });

        let hover = state.clone();
        let press = state.clone();
        let hover_enabled = enabled.clone();
        let element = element
            .when(!self.disabled, |this| {
                this.on_hover(move |hovered, window, cx| {
                    if hover_enabled.get() {
                        hover.update(cx, |state, cx| {
                            state.hover(0, *hovered, delay, close_delay, window, cx)
                        })
                    }
                })
                // A press is the control being used, not read about.
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    press.update(cx, |state, cx| state.hide(window, cx))
                })
            })
            .children(popup);
        let child = measured(&anchor, bind(element, state.clone())).into_any_element();
        TriggerTree {
            child: provide(TooltipScope(Some(nested.clone())), child).into_any_element(),
            nested,
            enabled,
            disabled: self.disabled,
            state,
        }
        .into_any_element()
    }
}

struct TriggerEvents {
    enabled: Rc<Cell<bool>>,
    nested: Rc<NestedHints>,
    _registration: Option<Rc<()>>,
    _subscriptions: [Subscription; 3],
}

#[derive(Clone, Default)]
struct TooltipScope(Option<Rc<NestedHints>>);

struct NestedHints {
    registrations: RefCell<Vec<Weak<()>>>,
    enabled: Rc<Cell<bool>>,
}

impl NestedHints {
    fn register(&self, window: &Window, cx: &App) -> Rc<()> {
        let registration = Rc::new(());
        self.registrations
            .borrow_mut()
            .push(Rc::downgrade(&registration));
        self.enabled.set(false);
        refresh_after_draw(window, cx);
        registration
    }

    fn any(&self) -> bool {
        let mut registrations = self.registrations.borrow_mut();
        registrations.retain(|registration| registration.strong_count() > 0);
        !registrations.is_empty()
    }
}

/// A trigger's descendants register their own hints while mounted. Scope
/// restoration around the popup keeps rich labels outside this relation.
struct TriggerTree {
    child: AnyElement,
    nested: Rc<NestedHints>,
    enabled: Rc<Cell<bool>>,
    disabled: bool,
    state: Entity<PopupState>,
}

impl IntoElement for TriggerTree {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for TriggerTree {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn transparent_child_mut(&mut self) -> Option<&mut AnyElement> {
        Some(&mut self.child)
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let layout = self.child.request_layout(window, cx);
        let nested = self.nested.any();
        let next = !self.disabled && !nested;
        let changed = self.enabled.replace(next) != next;
        if !next {
            self.state.update(cx, |state, cx| state.hide(window, cx));
        }
        if changed {
            refresh_after_draw(window, cx);
        }
        // Cached views retain their registration without laying out again.
        // Removed keyed state is dropped when this draw finishes, so also
        // check membership after that cleanup before restoring the owner.
        if nested {
            let registrations = self.nested.clone();
            let enabled = self.enabled.clone();
            let disabled = self.disabled;
            window
                .spawn(cx, async move |cx| {
                    let _ = cx.update(|window, _| {
                        let next = !disabled && !registrations.any();
                        if enabled.replace(next) != next {
                            window.refresh();
                        }
                    });
                })
                .detach();
        }
        (layout, ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.prepaint(window, cx);
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
    }
}

// Nested hints are discovered during layout, and focus changes arrive at
// the end of a draw. Refresh after that frame has finished.
fn refresh_after_draw(window: &Window, cx: &App) {
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
        let element = div().id(self.id.clone()).flex().track_focus(&focus);
        let child = self.child.take();
        self.bind(element, child, &focus, window, cx)
    }
}
