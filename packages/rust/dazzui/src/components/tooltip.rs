//! The label a control surfaces on hover or keyboard focus. It never carries
//! anything essential: it is a reminder of what an icon means.

use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
    time::Duration,
};

use gpui::{
    AnyElement, App, Bounds, Div, Element, ElementId, Entity, FocusHandle, GlobalElementId,
    InspectorElementId, InteractiveElement, IntoElement, LayoutId, MouseButton, MouseMoveEvent,
    ParentElement, Pixels, Point, RenderOnce, Stateful, StatefulInteractiveElement, Styled,
    Subscription, Visibility, Window, div, prelude::*, px,
};

use crate::components::collapsible::{kept_focus, part_id};
use crate::components::popup::{Align, Anchor, Placement, PopupState, Side, measured, positioned};
use crate::foundation::press::{ActivationObserver, ButtonPress, OnClick};
use crate::foundation::{StyledExt, focus_visible, provide, scoped};
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

mod group;
mod travel;
use group::DelayGroup;
pub use group::TooltipProvider;
use travel::{HoverTravel, TravelContext};

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
        let group = scoped::<DelayGroup>(cx);
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, false, window, cx);
        let watched = window.use_keyed_state(part_id(&self.id, "events"), cx, |window, cx| {
            let enabled = Rc::new(Cell::new(true));
            let travel = HoverTravel::default();
            let member = group.as_ref().map(|group| {
                Rc::new(GroupMember {
                    token: Rc::new(()),
                    group: group.clone(),
                    open: Cell::new(false),
                })
            });
            let focus_enabled = enabled.clone();
            let focus_member = member.clone();
            let focus_travel = travel.clone();
            let entered = state.downgrade();
            let enter = window.on_focus_in(focus, cx, move |window, cx| {
                if focus_enabled.get()
                    && window
                        .focused(cx)
                        .is_some_and(|focus| focus_visible(&focus, window, cx))
                {
                    focus_travel.reset();
                    let _ = entered.update(cx, |state, cx| {
                        state.cancel_hover();
                        state.open = true;
                        cx.notify();
                    });
                    if let Some((member, popup)) = focus_member.as_ref().zip(entered.upgrade()) {
                        member.sync(&popup, true, window, cx);
                    }
                    refresh_after_draw(window, cx);
                }
            });
            let blur_enabled = enabled.clone();
            let blur_member = member.clone();
            let blurred = state.downgrade();
            let blur = window.on_focus_out(focus, cx, move |_, window, cx| {
                if blur_enabled.get() {
                    let _ = blurred.update(cx, |state, cx| state.hide(window, cx));
                    if let Some((member, popup)) = blur_member.as_ref().zip(blurred.upgrade()) {
                        member.sync(&popup, true, window, cx);
                    }
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
            let observer = member.as_ref().map(|member| {
                let member = member.clone();
                cx.observe_in(
                    &state,
                    window,
                    move |this: &mut TriggerEvents, popup, window, cx| {
                        member.sync(&popup, this.enabled.get(), window, cx);
                    },
                )
            });
            TriggerEvents {
                enabled: enabled.clone(),
                nested: Rc::new(NestedHints {
                    registrations: RefCell::default(),
                    enabled: enabled.clone(),
                }),
                _registration: parent.0.as_ref().map(|parent| parent.register(window, cx)),
                _group: member,
                _group_observer: observer,
                pointer: Rc::new(Cell::new(None)),
                travel,
                surface: Anchor::default(),
                _subscriptions: [enter, blur, key],
            }
        });
        let enabled = watched.read(cx).enabled.clone();
        let nested = watched.read(cx).nested.clone();
        enabled.set(!self.disabled && !nested.any());
        if !enabled.get() {
            state.update(cx, |state, cx| state.hide(window, cx));
        }
        // Focus and disabled-state changes can happen during GPUI's draw
        // phase, where notifications invalidate views without observing them.
        let member = watched.read(cx)._group.clone();
        if let Some(member) = member {
            member.sync(&state, enabled.get(), window, cx);
        }
        let anchor = state.read(cx).anchor.clone();
        let open = state.read(cx).open && enabled.get();
        let default_delay = Duration::from_secs_f64(f64::from(t.tooltip_open_delay) / 1000.);
        let default_close = Duration::from_secs_f64(f64::from(t.tooltip_close_delay) / 1000.);
        let own_delay = self.delay;
        let own_close = self.close_delay;
        let travel = watched.read(cx).travel.clone();
        if !open {
            travel.reset();
        }
        let surface = watched.read(cx).surface.clone();
        let travel_context = TravelContext {
            state: state.clone(),
            surface: surface.clone(),
            side: self.side,
            gap: t.spacing_1,
            group: group.clone(),
            close_delay: own_close,
            default_close,
        };

        let popup = open.then(|| {
            let hidden = nested.clone();
            // `.dz-tooltip`: the raised card, a hairline, the small label.
            let label = div()
                .id(part_id(&self.id, "panel"))
                .occlude()
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
                positioned(
                    &anchor,
                    Placement::side(self.side, self.align, t),
                    measured(&surface, label),
                ),
            )
            .into_any_element()
        });

        let hover = state.clone();
        let moved = state.clone();
        let press = state.clone();
        let hover_enabled = enabled.clone();
        let move_enabled = enabled.clone();
        let pointer = watched.read(cx).pointer.clone();
        let move_pointer = pointer.clone();
        let move_group = group.clone();
        let hover_travel = travel.clone();
        let hover_context = travel_context.clone();
        // Keep GPUI's hover identity while disabled. Reattaching the listener
        // would synthesize an enter when the hint becomes enabled in place.
        let element = element
            .on_hover(move |hovered, window, cx| {
                let moved = pointer.get().is_some();
                pointer.set(hovered.then(|| window.mouse_position()));
                if hover_enabled.get() {
                    if !*hovered && hover.read(cx).open {
                        hover_travel.leave(window.mouse_position(), &hover_context, window, cx);
                        return;
                    }
                    if *hovered {
                        hover_travel.reset();
                    }
                    let delay = group.as_ref().map_or_else(
                        || own_delay.unwrap_or(default_delay),
                        |group| group.delay(own_delay, default_delay),
                    );
                    let close_delay = group.as_ref().map_or_else(
                        || own_close.unwrap_or(default_close),
                        |group| group.close_delay(own_close, default_close),
                    );
                    hover.update(cx, |state, cx| {
                        state.hover_rest(*hovered, moved, delay, close_delay, window, cx)
                    })
                }
            })
            .on_mouse_move(move |event, window, cx| {
                let previous = move_pointer.replace(Some(event.position));
                if !move_enabled.get() {
                    return;
                }
                let jitter = previous.is_some_and(|previous| {
                    let dx = f32::from(event.position.x - previous.x);
                    let dy = f32::from(event.position.y - previous.y);
                    // Base UI's rest timer tolerates one-pixel jitter.
                    dx * dx + dy * dy < 2.
                });
                let delay = move_group.as_ref().map_or_else(
                    || own_delay.unwrap_or(default_delay),
                    |group| group.delay(own_delay, default_delay),
                );
                moved.update(cx, |state, cx| {
                    state.restart_hover_open(delay, jitter, window, cx)
                });
            })
            // A press is the control being used, not read about.
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                press.update(cx, |state, cx| state.hide(window, cx))
            })
            .children(popup);
        let child = measured(&anchor, bind(element, state.clone())).into_any_element();
        TriggerTree {
            child: provide(TooltipScope(Some(nested.clone())), child).into_any_element(),
            nested,
            enabled,
            disabled: self.disabled,
            state,
            travel,
            travel_context,
        }
        .into_any_element()
    }
}

struct TriggerEvents {
    enabled: Rc<Cell<bool>>,
    nested: Rc<NestedHints>,
    _registration: Option<Rc<()>>,
    _group: Option<Rc<GroupMember>>,
    _group_observer: Option<Subscription>,
    pointer: Rc<Cell<Option<Point<Pixels>>>>,
    travel: HoverTravel,
    surface: Anchor,
    _subscriptions: [Subscription; 3],
}

struct GroupMember {
    token: Rc<()>,
    group: DelayGroup,
    open: Cell<bool>,
}

impl GroupMember {
    fn sync(&self, popup: &Entity<PopupState>, enabled: bool, window: &mut Window, cx: &mut App) {
        let open = popup.read(cx).open && enabled;
        if self.open.replace(open) != open {
            self.group
                .changed(popup, Rc::downgrade(&self.token), open, window, cx);
        }
    }
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
    travel: HoverTravel,
    travel_context: TravelContext,
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
        if self.enabled.get() && self.state.read(cx).open {
            let travel = self.travel.clone();
            let context = self.travel_context.clone();
            let enabled = self.enabled.clone();
            window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
                if !phase.bubble() && enabled.get() {
                    travel.motion(event.position, false, &context, window, cx);
                }
            });
        }
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
