use std::cell::Cell;
use std::panic::Location;
use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, DispatchPhase, Element, ElementId, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Window,
};

use crate::foundation::ControlState;

/// The interaction state a [`Pressable`] hands its builder.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PressableState {
    pub hovered: bool,
    pub pressed: bool,
    pub enabled: bool,
}

impl PressableState {
    /// What the shared state machine resolves recipes for. Disabled outranks
    /// everything, which is what flattens a control to the groove.
    pub fn control_state(self) -> ControlState {
        if !self.enabled {
            ControlState::Disabled
        } else if self.pressed {
            ControlState::Pressed
        } else if self.hovered {
            ControlState::Hovered
        } else {
            ControlState::Normal
        }
    }
}

/// The substrate every clickable component is built on.
///
/// It gives a component what CSS gets from `:hover` and `:active` — the two
/// pseudo-classes the shared control state machine is written against — and
/// it has to be an element of its own rather than GPUI's `hover()` and
/// `active()` refinements because of *when* those apply. GPUI bakes a text
/// run's colour when the text is laid out, and a refinement is only resolved
/// a phase later, so a recipe whose content darkens a grade under the pointer
/// would move its surface and leave its label behind. Here the state is known
/// before the subtree is built, so every part of it follows the recipe.
///
/// The state is read from the previous frame's bounds — the same thing GPUI's
/// own hit testing does — so a press is tracked without the component having
/// to own any.
pub fn pressable(
    id: impl Into<ElementId>,
    enabled: bool,
    builder: impl FnOnce(PressableState, &mut Window, &mut App) -> AnyElement + 'static,
) -> Pressable {
    Pressable {
        id: id.into(),
        enabled,
        builder: Some(Box::new(builder)),
    }
}

/// What a pressable builds once it knows its state.
type PressableBuilder = Box<dyn FnOnce(PressableState, &mut Window, &mut App) -> AnyElement>;

/// The element [`pressable`] builds.
pub struct Pressable {
    id: ElementId,
    enabled: bool,
    builder: Option<PressableBuilder>,
}

/// What one pressable remembers between frames.
#[derive(Clone, Default)]
struct PressableFrame {
    bounds: Bounds<Pixels>,
    pressed: Rc<Cell<bool>>,
}

impl Element for Pressable {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, AnyElement) {
        let enabled = self.enabled;
        let builder = self.builder.take().expect("a pressable is built once");

        window.with_element_state::<PressableFrame, _>(
            global_id.expect("a pressable carries an id"),
            |state, window| {
                let state = state.unwrap_or_default();
                let hovered = enabled && state.bounds.contains(&window.mouse_position());
                let pressed = hovered && state.pressed.get();

                let mut element = builder(
                    PressableState {
                        hovered,
                        pressed,
                        enabled,
                    },
                    window,
                    cx,
                );
                let layout_id = element.request_layout(window, cx);
                ((layout_id, element), state)
            },
        )
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        element: &mut AnyElement,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.prepaint(window, cx);
        window.with_element_state::<PressableFrame, _>(
            global_id.expect("a pressable carries an id"),
            |state, _| {
                let mut state: PressableFrame = state.unwrap_or_default();
                state.bounds = bounds;
                ((), state)
            },
        );
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        element: &mut AnyElement,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        element.paint(window, cx);

        if !self.enabled {
            return;
        }

        let pressed = window.with_element_state::<PressableFrame, _>(
            global_id.expect("a pressable carries an id"),
            |state, _| {
                let state: PressableFrame = state.unwrap_or_default();
                (state.pressed.clone(), state)
            },
        );

        let held = pressed.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, _| {
            if phase == DispatchPhase::Bubble && bounds.contains(&event.position) {
                held.set(true);
                window.refresh();
            }
        });

        let held = pressed;
        window.on_mouse_event(move |_: &MouseUpEvent, phase, window, _| {
            // Captured rather than bubbled, and released wherever the pointer
            // ended up: a press that wanders off the control still has to
            // let go of it.
            if phase == DispatchPhase::Capture && held.replace(false) {
                window.refresh();
            }
        });

        // Nothing else asks for a frame when the pointer merely moves, and
        // the state this element reads is the previous frame's — so crossing
        // the edge has to ask for one.
        let was_hovered = bounds.contains(&window.mouse_position());
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, _| {
            if phase == DispatchPhase::Capture && bounds.contains(&event.position) != was_hovered {
                window.refresh();
            }
        });
    }
}

impl IntoElement for Pressable {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// A flag a stateless component keeps between frames.
///
/// A `RenderOnce` component owns nothing, and some of them have a state that
/// is nobody else's business — whether a menu is open, how far a divider has
/// been dragged. GPUI already keeps per-element state across frames, keyed by
/// element id; this hands a component a cell in it, so the state lives
/// exactly as long as the element does and no host has to hold it.
pub fn latched<T: Copy + Default + 'static>(
    id: impl Into<ElementId>,
    builder: impl FnOnce(Rc<Cell<T>>, &mut Window, &mut App) -> AnyElement + 'static,
) -> Latched<T> {
    Latched {
        id: id.into(),
        builder: Some(Box::new(builder)),
    }
}

/// What a latch builds once it has its cell.
type LatchedBuilder<T> = Box<dyn FnOnce(Rc<Cell<T>>, &mut Window, &mut App) -> AnyElement>;

/// The element [`latched`] builds.
pub struct Latched<T: Copy + Default + 'static> {
    id: ElementId,
    builder: Option<LatchedBuilder<T>>,
}

impl<T: Copy + Default + 'static> Element for Latched<T> {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, AnyElement) {
        let builder = self.builder.take().expect("a latch is built once");

        window.with_element_state::<Rc<Cell<T>>, _>(
            global_id.expect("a latch carries an id"),
            |state, window| {
                let state = state.unwrap_or_default();
                let mut element = builder(state.clone(), window, cx);
                let layout_id = element.request_layout(window, cx);
                ((layout_id, element), state)
            },
        )
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        element: &mut AnyElement,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        element: &mut AnyElement,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        element.paint(window, cx);
    }
}

impl<T: Copy + Default + 'static> IntoElement for Latched<T> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// What a component calls when it is activated.
///
/// The three shapes cover every callback in the system: one that carries
/// GPUI's click event, one that carries nothing because the component's own
/// state says everything, and one that hands back the value that was chosen.
pub type ClickListener = Box<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>;

/// A callback with nothing to hand back — a nav row, a menu item, a card.
pub type ActionListener = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// A callback that hands back what was chosen or typed.
pub type ValueListener<T> = Box<dyn Fn(&T, &mut Window, &mut App) + 'static>;

/// The same, shared: a group hands one listener to every row it builds.
pub type SharedActionListener = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// The same, shared and carrying a value.
pub type SharedValueListener<T> = Rc<dyn Fn(&T, &mut Window, &mut App) + 'static>;
