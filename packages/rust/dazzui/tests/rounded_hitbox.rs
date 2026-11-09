//! CSS border and overflow curves must govern pointer and wheel targeting.
use gpui::{
    AnyElement, AnyView, App, AppContext, Bounds, Context, Element, ElementId, Entity,
    GlobalElementId, InspectorElementId, InteractiveElement, IntoElement, LayoutId, Modifiers,
    Overflow, ParentElement, Pixels, Point, Render, ScrollDelta, ScrollWheelEvent,
    StatefulInteractiveElement, StyleRefinement, Styled, TestAppContext, TouchPhase,
    VisualTestContext, WeakEntity, Window, div, point, px, rgb, size,
};

#[derive(Clone)]
struct Case {
    name: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    border: [f32; 4],
    overflow: String,
    nested: bool,
    factor: f32,
    child_factor: f32,
    samples: Vec<(f32, f32, String, Option<String>)>,
}
struct Harness {
    case: Case,
    events: Vec<&'static str>,
    moves: Vec<&'static str>,
    cached: bool,
    leaf: Option<Entity<CachedLeaf>>,
    leaf_renders: std::rc::Rc<std::cell::Cell<usize>>,
}
struct CachedLeaf {
    owner: WeakEntity<Harness>,
    renders: std::rc::Rc<std::cell::Cell<usize>>,
}
impl Render for CachedLeaf {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.renders.set(self.renders.get() + 1);
        let movement = self.owner.clone();
        let click = self.owner.clone();
        let wheel = self.owner.clone();
        div()
            .id("cached-child")
            .size_full()
            .occlude()
            .bg(rgb(0x00ff00))
            .on_mouse_move(move |_, _, cx| {
                movement
                    .update(cx, |view, _| view.moves.push("child"))
                    .unwrap();
                cx.stop_propagation();
            })
            .on_click(move |_, _, cx| {
                click
                    .update(cx, |view, _| view.events.push("child"))
                    .unwrap();
                cx.stop_propagation();
            })
            .on_scroll_wheel(move |_, _, cx| {
                wheel
                    .update(cx, |view, _| view.events.push("child"))
                    .unwrap();
                cx.stop_propagation();
            })
    }
}

struct Scale {
    child: AnyElement,
    factor: f32,
    origin: Option<Point<Pixels>>,
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
        let origin = self.origin.unwrap_or(bounds.origin);
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

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cached = if self.cached {
            let owner = cx.entity().downgrade();
            let renders = self.leaf_renders.clone();
            let leaf = self
                .leaf
                .get_or_insert_with(|| cx.new(|_| CachedLeaf { owner, renders }))
                .clone();
            let [top, right, bottom, left] = self.case.border;
            let mut style = StyleRefinement::default();
            style.size.width = Some(px(self.case.width - left - right).into());
            style.size.height = Some(px(self.case.height - top - bottom).into());
            Some(AnyView::from(leaf).cached(style).into_any_element())
        } else {
            None
        };
        let target = |name, color| {
            div()
                .id(name)
                .occlude()
                .bg(rgb(color))
                .on_mouse_move(cx.listener(move |view, _, _, cx| {
                    view.moves.push(name);
                    cx.stop_propagation();
                }))
                .on_click(cx.listener(move |view, _, _, cx| {
                    view.events.push(name);
                    cx.stop_propagation();
                }))
                .on_scroll_wheel(cx.listener(move |view, _, _, cx| {
                    view.events.push(name);
                    cx.stop_propagation();
                }))
        };
        let [top, right, bottom, left] = self.case.border;
        let child = if self.case.overflow == "clip-x" {
            target("child", 0x00ff00)
                .absolute()
                .left(px(-24.))
                .top(px(-24.))
                .size(px(128.))
        } else {
            target("child", 0x00ff00).absolute().inset_0()
        };
        let child = Scale {
            child: cached.unwrap_or_else(|| child.into_any_element()),
            factor: self.case.child_factor,
            origin: None,
        };
        let mut host = target("host", 0xffffff)
            .absolute()
            .left(px(self.case.x))
            .top(px(self.case.y))
            .w(px(self.case.width))
            .h(px(self.case.height))
            .rounded(px(self.case.radius))
            .border_t(px(top))
            .border_r(px(right))
            .border_b(px(bottom))
            .border_l(px(left))
            .border_color(gpui::transparent_black());
        host.style().overflow.x = Some(match self.case.overflow.as_str() {
            "hidden" => Overflow::Hidden,
            "clip-x" => Overflow::Clip,
            _ => Overflow::Visible,
        });
        host.style().overflow.y = Some(if self.case.overflow == "hidden" {
            Overflow::Hidden
        } else {
            Overflow::Visible
        });
        let host = if self.case.nested {
            host.child(
                target("inner", 0xffffff)
                    .absolute()
                    .left(px(8.))
                    .top(px(8.))
                    .size(px(64.))
                    .rounded(px(20.))
                    .overflow_hidden()
                    .child(child),
            )
        } else {
            host.child(child)
        };
        Scale {
            child: div()
                .size_full()
                .child(target("behind", 0x0000ff).absolute().inset_0())
                .child(host)
                .into_any_element(),
            factor: self.case.factor,
            origin: Some(Point::default()),
        }
    }
}
fn cases() -> Vec<Case> {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../js/dazzui/example/scripts/rounded-hitbox.cases.json"
    ))
    .unwrap();
    data.as_array()
        .unwrap()
        .iter()
        .map(|c| Case {
            name: c["name"].as_str().unwrap().to_owned(),
            x: c["x"].as_f64().unwrap_or(24.) as f32,
            y: c["y"].as_f64().unwrap_or(24.) as f32,
            width: c["width"].as_f64().unwrap() as f32,
            height: c["height"]
                .as_f64()
                .unwrap_or_else(|| c["width"].as_f64().unwrap()) as f32,
            radius: c["radius"].as_f64().unwrap() as f32,
            border: std::array::from_fn(|i| c["border"][i].as_f64().unwrap() as f32),
            overflow: c["overflow"].as_str().unwrap().to_owned(),
            nested: c["nested"].as_bool().unwrap_or(false),
            factor: c["factor"].as_f64().unwrap_or(1.) as f32,
            child_factor: c["childFactor"].as_f64().unwrap_or(1.) as f32,
            samples: c["samples"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| {
                    (
                        p[0].as_f64().unwrap() as f32,
                        p[1].as_f64().unwrap() as f32,
                        p[2].as_str().unwrap().to_owned(),
                        p.get(3).and_then(|v| v.as_str()).map(str::to_owned),
                    )
                })
                .collect(),
        })
        .collect()
}

fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        case: cases().remove(0),
        events: Vec::new(),
        moves: Vec::new(),
        cached: false,
        leaf: None,
        leaf_renders: Default::default(),
    });
    cx.simulate_resize(size(px(160.), px(160.)));
    cx.run_until_parked();
    (view, cx)
}
#[gpui::test]
fn clipped_children_and_the_containers_own_border_target_the_same_layers_as_css(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    for scale in [1., 2.] {
        cx.simulate_scale_factor(scale);
        cx.run_until_parked();
        cx.update(|window, _| assert_eq!(window.scale_factor(), scale));
        for cached in [false, true] {
            for case in cases()
                .into_iter()
                .filter(|case| !cached || (!case.nested && case.overflow != "clip-x"))
            {
                view.update(cx, |view, cx| {
                    view.cached = cached;
                    view.case = case.clone();
                    cx.notify();
                });
                cx.run_until_parked();
                for (x, y, expected, _) in &case.samples {
                    view.update(cx, |view, _| view.events.clear());
                    cx.simulate_click(point(px(*x), px(*y)), Modifiers::none());
                    assert_eq!(
                        view.read_with(cx, |view, _| view.events.clone()),
                        vec![expected.as_str()],
                        "{} at ({x},{y})",
                        case.name
                    );
                }
            }
        }
    }
}
#[gpui::test]
fn wheel_targeting_obeys_the_same_round_and_elliptical_clips(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    for scale in [1., 2.] {
        cx.simulate_scale_factor(scale);
        cx.run_until_parked();
        cx.update(|window, _| assert_eq!(window.scale_factor(), scale));
        for cached in [false, true] {
            for case in cases()
                .into_iter()
                .filter(|case| !cached || (!case.nested && case.overflow != "clip-x"))
            {
                view.update(cx, |view, cx| {
                    view.cached = cached;
                    view.case = case.clone();
                    cx.notify();
                });
                cx.run_until_parked();
                for (x, y, expected, wheel) in &case.samples {
                    let expected = wheel.as_ref().unwrap_or(expected);
                    view.update(cx, |view, _| view.events.clear());
                    cx.simulate_event(ScrollWheelEvent {
                        position: point(px(*x), px(*y)),
                        delta: ScrollDelta::Pixels(point(px(0.), px(-20.))),
                        modifiers: Modifiers::none(),
                        touch_phase: TouchPhase::Moved,
                    });
                    cx.run_until_parked();
                    assert_eq!(
                        view.read_with(cx, |view, _| view.events.clone()),
                        vec![expected.as_str()],
                        "{} at ({x},{y})",
                        case.name
                    );
                }
            }
        }
    }
}

#[gpui::test]
fn cached_children_reuse_unchanged_clips_and_invalidate_when_only_the_radius_changes(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.cached = true;
        view.case.radius = 0.;
        view.case.border = [0.; 4];
        cx.notify();
    });
    cx.run_until_parked();
    let count = view.read_with(cx, |view, _| view.leaf_renders.get());
    assert!(count > 0);
    view.update(cx, |_, cx| cx.notify());
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.leaf_renders.get()),
        count,
        "unchanged clip must really reuse the cached view"
    );
    for (radius, expected) in [(24., "behind"), (2., "child"), (0., "child")] {
        let before = view.read_with(cx, |view, _| view.leaf_renders.get());
        view.update(cx, |view, cx| {
            view.case.radius = radius;
            view.events.clear();
            cx.notify();
        });
        cx.run_until_parked();
        assert!(
            view.read_with(cx, |view, _| view.leaf_renders.get()) > before,
            "changed rounded clip must invalidate cached hitboxes"
        );
        let count = view.read_with(cx, |view, _| view.leaf_renders.get());
        view.update(cx, |_, cx| cx.notify());
        cx.run_until_parked();
        assert_eq!(
            view.read_with(cx, |view, _| view.leaf_renders.get()),
            count,
            "equal nonzero clip geometry must still reuse the cache"
        );
        cx.simulate_click(point(px(25.), px(25.)), Modifiers::none());
        assert_eq!(
            view.read_with(cx, |view, _| view.events.clone()),
            vec![expected]
        );
    }
}

#[gpui::test]
fn hover_moves_follow_rounded_clips_including_fractional_edge_queries(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    for scale in [1., 2.] {
        cx.simulate_scale_factor(scale);
        cx.run_until_parked();
        cx.update(|window, _| assert_eq!(window.scale_factor(), scale));
        for cached in [false, true] {
            for case in cases()
                .into_iter()
                .filter(|case| !cached || (!case.nested && case.overflow != "clip-x"))
            {
                view.update(cx, |view, cx| {
                    view.cached = cached;
                    view.case = case.clone();
                    cx.notify();
                });
                cx.run_until_parked();
                for (x, y, expected, _) in &case.samples {
                    view.update(cx, |view, _| view.moves.clear());
                    cx.simulate_mouse_move(point(px(*x), px(*y)), None, Modifiers::none());
                    cx.run_until_parked();
                    assert_eq!(
                        view.read_with(cx, |view, _| view.moves.clone()),
                        vec![expected.as_str()],
                        "{} at ({x},{y})",
                        case.name
                    );
                }
            }
        }
    }
}
