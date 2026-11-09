//! Button and anchor key phases, preserving native activation and cancellation.

use std::{cell::Cell, rc::Rc};

use gpui::{
    AnyElement, App, Bounds, ClickEvent, Div, Element, ElementId, Entity, FocusHandle,
    GlobalElementId, InspectorElementId, InteractiveElement, IntoElement, KeyboardButton,
    KeyboardClickEvent, LayoutId, Pixels, Stateful, StatefulInteractiveElement, Subscription,
    Window,
};

use crate::foundation::scoped;

pub(crate) type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A trigger wrapper can observe the control's real activation without
/// depending on the SDK's synthetic keyboard click or adding a handler to
/// the control's public callback.
#[derive(Clone)]
pub(crate) struct ActivationObserver {
    pub attached: Rc<Cell<bool>>,
    pub on_click: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
}

struct PressState {
    armed: bool,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    _blur: Subscription,
}

pub(crate) struct ButtonPress {
    state: Entity<PressState>,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    disabled: bool,
    native: bool,
    composite: bool,
    link: bool,
    observer: Option<ActivationObserver>,
}

impl ButtonPress {
    pub fn new(
        id: &ElementId,
        focus: &FocusHandle,
        disabled: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let key = ElementId::NamedChild(Box::new(id.clone()), "keyboard-press".into());
        let state = window.use_keyed_state(key, cx, |window, cx| {
            let weak = cx.weak_entity();
            let blur = window.on_focus_out(focus, cx, move |_, _, cx| {
                let _ = weak.update(cx, |state: &mut PressState, cx| {
                    if std::mem::take(&mut state.armed) {
                        cx.notify();
                    }
                });
            });
            PressState {
                armed: false,
                bounds: Rc::default(),
                _blur: blur,
            }
        });
        if disabled || !focus.contains_focused(window, cx) {
            state.update(cx, |state, _| state.armed = false);
        }
        let bounds = state.read(cx).bounds.clone();
        let observer = scoped::<ActivationObserver>(cx);
        if let Some(observer) = &observer {
            observer.attached.set(true);
        }
        Self {
            state,
            bounds,
            disabled,
            native: true,
            composite: false,
            link: false,
            observer,
        }
    }

    /// Base UI's non-native button path dispatches Enter on keydown and
    /// Space on keyup without retaining a preceding Space keydown.
    pub fn non_native(mut self) -> Self {
        self.native = false;
        self
    }

    /// Composite buttons activate Space on keydown and suppress keyup.
    pub fn composite(mut self) -> Self {
        self.composite = true;
        self
    }

    /// Anchors activate Enter on the initial keydown and leave Space to the page.
    pub fn link(mut self) -> Self {
        self.link = true;
        self
    }

    pub fn pressed(&self, cx: &App) -> bool {
        self.native && !self.composite && !self.link && !self.disabled && self.state.read(cx).armed
    }

    pub fn bind(self, element: Stateful<Div>, on_click: Option<OnClick>) -> impl IntoElement {
        let bounds = self.bounds;
        let native = self.native;
        let composite = self.composite;
        let link = self.link;
        let element = if self.disabled {
            element.on_click(|_, _, cx| cx.stop_propagation())
        } else {
            let on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>> =
                on_click.map(Rc::from);
            let on_click = match self.observer {
                Some(observer) => Some(Rc::new(
                    move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                        if let Some(on_click) = &on_click {
                            on_click(event, window, cx);
                        }
                        (observer.on_click)(event, window, cx);
                    },
                )
                    as Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>),
                None => on_click,
            };
            let mouse = on_click.clone();
            let down = on_click.clone();
            let armed = self.state.clone();
            let release = self.state;
            let down_bounds = bounds.clone();
            let up_bounds = bounds.clone();
            element
                // The SDK synthesizes both keys on keyup, without remembering
                // a preceding keydown. Keep its mouse path, supply the keys.
                .on_click(move |event, window, cx| {
                    if !event.is_keyboard() {
                        if let Some(click) = &mouse {
                            click(event, window, cx);
                        }
                    }
                })
                .on_key_down(move |event, window, cx| {
                    if window.default_prevented() {
                        return;
                    }
                    let stroke = &event.keystroke;
                    if stroke.key == "space" {
                        if link {
                            return;
                        }
                        if composite {
                            window.prevent_default();
                            if let Some(click) = &down {
                                click(
                                    &keyboard(KeyboardButton::Space, down_bounds.get()),
                                    window,
                                    cx,
                                );
                            }
                            return;
                        }
                        if !native {
                            window.prevent_default();
                            return;
                        }
                        armed.update(cx, |state, cx| {
                            if !state.armed {
                                state.armed = true;
                                cx.notify();
                            }
                        });
                    } else if stroke.key == "enter"
                        && (!link || !event.is_held)
                        && (link
                            || !native
                            || (!stroke.modifiers.control
                                && !stroke.modifiers.alt
                                && !stroke.modifiers.platform))
                    {
                        if !native {
                            window.prevent_default();
                        }
                        if let Some(click) = &down {
                            click(
                                &keyboard(KeyboardButton::Enter, down_bounds.get()),
                                window,
                                cx,
                            );
                        }
                    }
                })
                .on_key_up(move |event, window, cx| {
                    if event.keystroke.key != "space" || composite || link {
                        return;
                    }
                    // The browser's native default action also releases
                    // its armed state. Preventing that action retains the
                    // press until an unprevented release or blur.
                    if window.default_prevented() {
                        return;
                    }
                    let armed = release.update(cx, |state, cx| {
                        let armed = std::mem::take(&mut state.armed);
                        if armed {
                            cx.notify();
                        }
                        armed
                    });
                    if armed || !native {
                        if let Some(click) = &on_click {
                            click(
                                &keyboard(KeyboardButton::Space, up_bounds.get()),
                                window,
                                cx,
                            );
                        }
                    }
                })
        };
        PressBounds {
            child: element.into_any_element(),
            bounds,
        }
    }
}

fn keyboard(button: KeyboardButton, bounds: Bounds<Pixels>) -> ClickEvent {
    ClickEvent::Keyboard(KeyboardClickEvent { button, bounds })
}

struct PressBounds {
    child: AnyElement,
    bounds: Rc<Cell<Bounds<Pixels>>>,
}

impl IntoElement for PressBounds {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for PressBounds {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.bounds.set(bounds);
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
