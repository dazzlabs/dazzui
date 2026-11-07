//! Values a subtree provides to what lays out inside it — the icon size a
//! slot binds, the press a trigger is handed — the way a CSS custom property
//! reaches every descendant.
//!
//! A component's `render` runs while its parent lays out, so a value pushed
//! for the span of a child's layout is exactly what that child's subtree
//! reads, and nothing outside it. Prepaint and paint push it again, for the
//! elements that read it there.

use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, Window,
};

#[derive(Default)]
struct Scopes {
    stacks: HashMap<TypeId, Vec<Rc<dyn std::any::Any>>>,
}

impl gpui::Global for Scopes {}

/// The innermost value of type `T` a [`provide`] put in force, if any.
pub fn scoped<T: Clone + 'static>(cx: &App) -> Option<T> {
    cx.try_global::<Scopes>()?
        .stacks
        .get(&TypeId::of::<T>())?
        .last()?
        .downcast_ref::<T>()
        .cloned()
}

/// `child`, laid out and drawn with `value` in force.
pub fn provide<T: Clone + 'static>(value: T, child: impl IntoElement) -> Provide {
    Provide {
        key: TypeId::of::<T>(),
        value: Rc::new(value),
        child: child.into_any_element(),
    }
}

/// Carry the current providers across a portal's detached layout pass.
pub(crate) fn preserve(child: impl IntoElement, cx: &App) -> AnyElement {
    let mut child = child.into_any_element();
    if let Some(scopes) = cx.try_global::<Scopes>() {
        for (&key, stack) in &scopes.stacks {
            if let Some(value) = stack.last() {
                child = Provide {
                    key,
                    value: value.clone(),
                    child,
                }
                .into_any_element();
            }
        }
    }
    child
}

pub struct Provide {
    key: TypeId,
    value: Rc<dyn std::any::Any>,
    child: AnyElement,
}

impl Provide {
    fn with<R>(&mut self, cx: &mut App, f: impl FnOnce(&mut AnyElement, &mut App) -> R) -> R {
        cx.default_global::<Scopes>()
            .stacks
            .entry(self.key)
            .or_default()
            .push(self.value.clone());
        let result = f(&mut self.child, cx);
        if let Some(stack) = cx.global_mut::<Scopes>().stacks.get_mut(&self.key) {
            stack.pop();
        }
        result
    }
}

impl IntoElement for Provide {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for Provide {
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
        (
            self.with(cx, |child, cx| child.request_layout(window, cx)),
            (),
        )
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
        self.with(cx, |child, cx| child.prepaint(window, cx));
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
        self.with(cx, |child, cx| child.paint(window, cx));
    }
}
