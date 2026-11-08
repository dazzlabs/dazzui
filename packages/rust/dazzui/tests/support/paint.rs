use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, ObservedPaint, Pixels, Window,
};
use std::{cell::RefCell, rc::Rc};

pub struct PaintProbe {
    pub child: AnyElement,
    pub paints: Rc<RefCell<Vec<ObservedPaint>>>,
}
impl IntoElement for PaintProbe {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for PaintProbe {
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
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let (_, paint) = window
            .observe_element_paint_with_surfaces(bounds, |window| self.child.paint(window, cx));
        self.paints.borrow_mut().push(paint);
    }
}
