//! A paint transform preserves layout while moving the complete subtree.

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, Point, Window,
};

pub(crate) fn scaled(child: impl IntoElement, factor: f32) -> impl IntoElement {
    Scale {
        child: child.into_any_element(),
        factor,
    }
}

struct Scale {
    child: AnyElement,
    factor: f32,
}

impl IntoElement for Scale {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for Scale {
    type RequestLayoutState = ();
    type PrepaintState = Point<Pixels>;

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
    ) -> Point<Pixels> {
        let origin = bounds.center();
        window.with_element_scale(origin, self.factor, |window| {
            self.child.prepaint(window, cx)
        });
        origin
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        origin: &mut Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_element_scale(*origin, self.factor, |window| self.child.paint(window, cx));
    }
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;
    use gpui::{
        AnyView, AppContext, Context, Entity, InteractiveElement, Modifiers, MouseButton,
        ParentElement, Render, StatefulInteractiveElement, StyleRefinement, Styled, TestAppContext,
        div, point, px,
    };
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    struct Target {
        clicks: Rc<Cell<usize>>,
        downs: Rc<RefCell<Vec<(Point<Pixels>, Point<Pixels>)>>>,
    }

    impl Render for Target {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let clicks = self.clicks.clone();
            let downs = self.downs.clone();
            div().relative().w(px(800.)).h(px(400.)).child(
                div()
                    .id("target")
                    .absolute()
                    .left(px(200.))
                    .top(px(100.))
                    .w(px(200.))
                    .h(px(100.))
                    .debug_selector(|| "scaled-target".into())
                    .on_mouse_down(MouseButton::Left, move |event, window, _| {
                        downs
                            .borrow_mut()
                            .push((event.position, window.mouse_position()));
                    })
                    .on_click(move |_, _, _| clicks.set(clicks.get() + 1)),
            )
        }
    }

    struct Harness {
        target: Entity<Target>,
        factor: f32,
        nested: bool,
    }
    impl Render for Harness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let mut style = StyleRefinement::default();
            style.size.width = Some(px(800.).into());
            style.size.height = Some(px(400.).into());
            let child: AnyView = self.target.clone().into();
            let child = child.cached(style);
            // Use zero as the scale origin so input mapping cannot pass by
            // accidentally clicking near an untransformed control's centre.
            OriginScale {
                child: child.into_any_element(),
                factor: self.factor,
                nested: self.nested,
            }
        }
    }

    struct OriginScale {
        child: AnyElement,
        factor: f32,
        nested: bool,
    }
    impl IntoElement for OriginScale {
        type Element = Self;
        fn into_element(self) -> Self {
            self
        }
    }
    impl Element for OriginScale {
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
            window.with_element_scale(point(px(0.), px(0.)), self.factor, |window| {
                window.with_element_scale(
                    point(px(0.), px(0.)),
                    if self.nested { 0.5 } else { 1. },
                    |window| self.child.prepaint(window, cx),
                );
            });
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
            window.with_element_scale(point(px(0.), px(0.)), self.factor, |window| {
                window.with_element_scale(
                    point(px(0.), px(0.)),
                    if self.nested { 0.5 } else { 1. },
                    |window| self.child.paint(window, cx),
                );
            });
        }
    }

    #[gpui::test]
    fn nested_scaled_controls_map_events_and_cached_hitboxes(cx: &mut TestAppContext) {
        let clicks = Rc::new(Cell::new(0));
        let downs = Rc::new(RefCell::new(Vec::new()));
        let (view, cx) = cx.add_window_view(|_, cx| Harness {
            target: cx.new(|_| Target {
                clicks: clicks.clone(),
                downs: downs.clone(),
            }),
            factor: 0.5,
            nested: true,
        });
        cx.run_until_parked();
        let bounds = cx.debug_bounds("scaled-target").unwrap();
        assert_eq!(bounds.origin, point(px(50.), px(25.)));
        assert_eq!(bounds.size.width, px(50.));
        cx.simulate_click(bounds.center(), Modifiers::none());
        assert_eq!(clicks.get(), 1);
        assert_eq!(
            downs.borrow()[0],
            (point(px(300.), px(150.)), point(px(300.), px(150.)))
        );
        cx.simulate_click(point(px(300.), px(150.)), Modifiers::none());
        assert_eq!(
            clicks.get(),
            1,
            "the old layout box is no longer a hit target"
        );

        view.update(cx, |view, cx| {
            view.factor = 1.;
            cx.notify();
        });
        cx.run_until_parked();
        let bounds = cx.debug_bounds("scaled-target").unwrap();
        assert_eq!(bounds.origin, point(px(100.), px(50.)));
        assert_eq!(bounds.size.width, px(100.));
        cx.simulate_click(bounds.center(), Modifiers::none());
        assert_eq!(
            clicks.get(),
            2,
            "cached children follow a changed parent scale"
        );
        assert_eq!(
            downs.borrow()[1],
            (point(px(300.), px(150.)), point(px(300.), px(150.)))
        );
    }
}
