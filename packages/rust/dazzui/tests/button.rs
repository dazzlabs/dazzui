//! Native button key phases, cancellation and real click bounds.

use dazzui::foundation::ControlColors;
use dazzui::{ActiveTheme, Button, Code, IconButton, KeyCap, Link, State, Theme, Tint, Variant};
use gpui::{
    AnyElement, App, Bounds, ClickEvent, Context, Element, ElementId, Entity, GlobalElementId,
    InspectorElementId, InteractiveElement, IntoElement, KeyUpEvent, Keystroke, LayoutId,
    Modifiers, ObservedPaint, ParentElement, Pixels, Render, Styled, TestAppContext,
    VisualTestContext, Window, div,
};
use std::{cell::RefCell, rc::Rc};

struct Harness {
    icon: bool,
    disabled: bool,
    prevent_down: bool,
    prevent_up: bool,
    clicks: Vec<ClickEvent>,
    variant: Variant,
    typography: Option<Rc<RefCell<Vec<ObservedPaint>>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        let clicked = move |event: &ClickEvent, _: &mut Window, cx: &mut App| {
            let _ = view.update(cx, |view, cx| {
                view.clicks.push(event.clone());
                cx.notify();
            });
        };
        let button = if self.icon {
            IconButton::new("tested", "Action")
                .disabled(self.disabled)
                .on_click(clicked)
                .child("+")
                .into_any_element()
        } else {
            Button::new("tested")
                .variant(self.variant)
                .disabled(self.disabled)
                .on_click(clicked)
                .child("Action")
                .into_any_element()
        };
        let button = match self.typography.clone() {
            Some(paints) => PaintProbe {
                child: button,
                paints,
            }
            .into_any_element(),
            None => button,
        };
        let prevent_down = self.prevent_down;
        let prevent_up = self.prevent_up;
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .capture_key_down(move |event, window, _| {
                if prevent_down && matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    window.prevent_default();
                }
            })
            .capture_key_up(move |event, window, _| {
                if prevent_up && event.keystroke.key == "space" {
                    window.prevent_default();
                }
            })
            .child(
                div()
                    .flex()
                    .debug_selector(|| "tested-box".into())
                    .child(button),
            )
            .child(Button::new("next").child("Next"))
    }
}

fn setup(cx: &mut TestAppContext, icon: bool) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        icon,
        disabled: false,
        prevent_down: false,
        prevent_up: false,
        clicks: vec![],
        variant: Variant::Recessed,
        typography: None,
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("tab");
    (view, cx)
}

fn up(cx: &mut VisualTestContext, key: &str) {
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse(key).unwrap(),
    });
    cx.run_until_parked();
}

fn count(view: &Entity<Harness>, cx: &mut VisualTestContext) -> usize {
    view.read_with(cx, |view, _| view.clicks.len())
}

#[gpui::test]
fn enter_activates_on_down_and_space_only_once_on_release(cx: &mut TestAppContext) {
    for icon in [false, true] {
        let (view, cx) = setup(cx, icon);
        cx.simulate_keystrokes("enter");
        assert_eq!(count(&view, cx), 1);
        cx.simulate_keystrokes("enter");
        assert_eq!(count(&view, cx), 2, "held Enter repeats");
        up(cx, "enter");
        assert_eq!(count(&view, cx), 2, "Enter release does not click twice");
        up(cx, "space");
        assert_eq!(count(&view, cx), 2, "a release alone is not a press");
        cx.simulate_keystrokes("space space");
        assert_eq!(count(&view, cx), 2);
        up(cx, "space");
        assert_eq!(
            count(&view, cx),
            3,
            "Space repeats do not repeat the action"
        );
        let expected = cx.debug_bounds("tested-box").unwrap();
        view.read_with(cx, |view, _| {
            for event in &view.clicks {
                let ClickEvent::Keyboard(event) = event else {
                    panic!("keyboard event")
                };
                assert_eq!(
                    event.bounds, expected,
                    "event carries the actual control box"
                );
            }
        });
        cx.simulate_click(expected.center(), Modifiers::none());
        assert_eq!(count(&view, cx), 4, "mouse activation still fires once");
    }
}

#[gpui::test]
fn space_cancels_on_blur_and_when_disabled_mid_press(cx: &mut TestAppContext) {
    for icon in [false, true] {
        let (view, cx) = setup(cx, icon);
        cx.simulate_keystrokes("space tab shift-tab");
        up(cx, "space");
        assert_eq!(count(&view, cx), 0, "blur cancels even after focus returns");
        cx.simulate_keystrokes("space");
        view.update(cx, |view, cx| {
            view.disabled = true;
            cx.notify();
        });
        cx.run_until_parked();
        up(cx, "space");
        assert_eq!(count(&view, cx), 0);
        view.update(cx, |view, cx| {
            view.disabled = false;
            cx.notify();
        });
        cx.run_until_parked();
        up(cx, "space");
        assert_eq!(
            count(&view, cx),
            0,
            "re-enabling does not revive an old press"
        );
    }
}

#[gpui::test]
fn prevented_keys_cancel_native_activation(cx: &mut TestAppContext) {
    for icon in [false, true] {
        let (view, cx) = setup(cx, icon);
        view.update(cx, |view, cx| {
            view.prevent_down = true;
            cx.notify();
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("enter space");
        up(cx, "enter");
        up(cx, "space");
        assert_eq!(count(&view, cx), 0);
        view.update(cx, |view, cx| {
            view.prevent_down = false;
            view.prevent_up = true;
            cx.notify();
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("space");
        up(cx, "space");
        assert_eq!(count(&view, cx), 0);
        view.update(cx, |view, cx| {
            view.prevent_up = false;
            cx.notify();
        });
        cx.run_until_parked();
        up(cx, "space");
        assert_eq!(
            count(&view, cx),
            0,
            "prevented release also clears the press"
        );
    }
}

#[gpui::test]
fn pointer_press_keeps_the_button_type_and_dimensions(cx: &mut TestAppContext) {
    for variant in [Variant::Recessed, Variant::Outlined, Variant::Filled] {
        let (view, cx) = setup(cx, false);
        view.update(cx, |view, cx| {
            view.variant = variant;
            cx.notify();
        });
        cx.run_until_parked();
        let before = cx.debug_bounds("tested-box").unwrap();
        cx.simulate_mouse_down(before.center(), gpui::MouseButton::Left, Modifiers::none());
        cx.run_until_parked();
        assert_eq!(
            cx.debug_bounds("tested-box").unwrap().size,
            before.size,
            "{variant:?}: changing pressed ink must retain the button's typography"
        );
        cx.simulate_mouse_up(before.center(), gpui::MouseButton::Left, Modifiers::none());
        cx.run_until_parked();
        assert_eq!(cx.debug_bounds("tested-box").unwrap().size, before.size);
    }
}

#[gpui::test]
fn hovered_and_pressed_ink_keep_the_painted_font(cx: &mut TestAppContext) {
    for variant in [
        Variant::Recessed,
        Variant::Plain,
        Variant::Outlined,
        Variant::Filled,
    ] {
        let (view, cx) = setup(cx, false);
        let painted = Rc::new(RefCell::new(Vec::new()));
        view.update(cx, |view, cx| {
            view.variant = variant;
            view.typography = Some(painted.clone());
            cx.notify();
        });
        cx.run_until_parked();
        let original = painted
            .borrow()
            .last()
            .unwrap()
            .text
            .clone()
            .expect("actual glyph paint");
        // The test platform aliases native fonts; real-window collection
        // additionally verifies the unambiguous font weights.
        let bounds = cx.debug_bounds("tested-box").unwrap();
        for pressed in [false, true] {
            if pressed {
                cx.simulate_mouse_down(bounds.center(), gpui::MouseButton::Left, Modifiers::none());
            } else {
                cx.simulate_mouse_move(bounds.center(), None, Modifiers::none());
            }
            cx.run_until_parked();
            let actual = painted
                .borrow()
                .last()
                .unwrap()
                .text
                .clone()
                .expect("actual glyph paint");
            assert_eq!(actual.font_id, original.font_id);
            assert_eq!(actual.font_size, original.font_size);
            assert_eq!(actual.font_weight, original.font_weight);
            assert_eq!(actual.line_height, original.line_height);
            let expected = cx.update(|_, cx| {
                ControlColors::resolve(
                    cx.theme(),
                    variant,
                    Tint::Primary,
                    if pressed {
                        State::Pressed
                    } else {
                        State::Hovered
                    },
                )
                .content
            });
            assert_eq!(
                actual.color, expected,
                "{variant:?}: the actual glyph must use the state's ink"
            );
        }
    }
}

struct PaintProbe {
    child: AnyElement,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
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
        let (_, paint) =
            window.observe_element_paint(bounds, |window| self.child.paint(window, cx));
        self.paints.borrow_mut().push(paint);
    }
}

struct NestedPaint {
    parent: Rc<RefCell<Vec<ObservedPaint>>>,
    child: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for NestedPaint {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let button = Button::new("parent")
            .child(PaintProbe {
                child: KeyCap::new().child("K").into_any_element(),
                paints: self.child.clone(),
            })
            .child("Save");
        PaintProbe {
            child: button.into_any_element(),
            paints: self.parent.clone(),
        }
    }
}

#[gpui::test]
fn paint_observation_excludes_text_inside_nested_specimens(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let parent = Rc::new(RefCell::new(Vec::new()));
    let child = Rc::new(RefCell::new(Vec::new()));
    let (_, cx) = cx.add_window_view(|_, _| NestedPaint {
        parent: parent.clone(),
        child: child.clone(),
    });
    cx.run_until_parked();
    let parent = parent.borrow().last().unwrap().clone();
    let child = child.borrow().last().unwrap().clone();
    assert!(
        parent.text.as_ref().unwrap().font_size > child.text.as_ref().unwrap().font_size,
        "the parent's first text is Save, after the separately observed key cap"
    );
    let root = parent.surface.as_ref().unwrap();
    let nested = child.surface.as_ref().unwrap();
    assert_eq!(
        root.style.border_widths.top.to_pixels(root.rem_size),
        gpui::px(1.)
    );
    assert_eq!(
        nested.style.border_widths.top.to_pixels(nested.rem_size),
        gpui::px(0.)
    );
}

struct WrappedPaint(Rc<RefCell<Vec<ObservedPaint>>>);

impl Render for WrappedPaint {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PaintProbe {
            child: div()
                .flex()
                .child(
                    Button::new("field")
                        .variant(Variant::Outlined)
                        .child("Choose"),
                )
                .into_any_element(),
            paints: self.0.clone(),
        }
    }
}

#[gpui::test]
fn paint_observation_keeps_layout_root_and_inner_control_separate(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let paints = Rc::new(RefCell::new(Vec::new()));
    let (_, cx) = cx.add_window_view(|_, _| WrappedPaint(paints.clone()));
    cx.run_until_parked();
    let paints = paints.borrow();
    let paint = paints.last().unwrap();
    let root = paint.surface.as_ref().unwrap();
    let field = paint.decorated_surface.as_ref().unwrap();
    assert_eq!(
        root.style.border_widths.top.to_pixels(root.rem_size),
        gpui::px(0.)
    );
    assert_eq!(
        field.style.border_widths.top.to_pixels(field.rem_size),
        gpui::px(1.)
    );
    assert!(paint.text.is_some());
}

struct InlinePaint([Rc<RefCell<Vec<ObservedPaint>>>; 3]);

impl Render for InlinePaint {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let probe = |index: usize, child: AnyElement| PaintProbe {
            child,
            paints: self.0[index].clone(),
        };
        div()
            .flex()
            .flex_col()
            .items_start()
            .text_size(gpui::px(13.))
            .line_height(gpui::px(20.))
            .child(probe(
                0,
                Link::new("block").child("Read").into_any_element(),
            ))
            .child(dazzui::foundation::inline_context(probe(
                1,
                Link::new("inline").child("Read").into_any_element(),
            )))
            .child(probe(2, Code::new().child("code").into_any_element()))
    }
}

#[gpui::test]
fn inline_boxes_preserve_inherited_painted_leading(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let paints = std::array::from_fn(|_| Rc::new(RefCell::new(Vec::new())));
    let (_, cx) = cx.add_window_view(|_, _| InlinePaint(paints.clone()));
    cx.run_until_parked();
    let readings = paints.map(|paints| paints.borrow().last().unwrap().clone());
    for reading in &readings {
        assert_eq!(reading.text.as_ref().unwrap().line_height, gpui::px(20.));
    }
    let block = readings[0].surface.as_ref().unwrap().bounds.size.height;
    let inline = readings[1].surface.as_ref().unwrap().bounds.size.height;
    assert_eq!(block, gpui::px(20.));
    assert!(
        inline < block,
        "inline box height is the font's content area, not its leading"
    );
}

struct InlineBaseline(Rc<RefCell<Vec<ObservedPaint>>>);

impl Render for InlineBaseline {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let face = cx.theme().body_medium.face;
        let content = dazzui::content_area(face, 26., window);
        let drop =
            dazzui::baseline(face, 26., 40., window) - dazzui::baseline(face, 26., content, window);
        div()
            .flex()
            .items_start()
            .font_family(face.family())
            .text_size(gpui::px(26.))
            .line_height(gpui::px(40.))
            .debug_selector(|| "prose-row".into())
            .child(dazzui::foundation::text_child("Run "))
            .child(
                div()
                    .flex()
                    .mt(gpui::px(drop))
                    .child(dazzui::foundation::inline_context(PaintProbe {
                        child: Code::new().child("code").into_any_element(),
                        paints: self.0.clone(),
                    })),
            )
    }
}

#[gpui::test]
fn inline_code_aligns_its_smaller_font_with_the_surrounding_baseline(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let paints = Rc::new(RefCell::new(Vec::new()));
    let (_, cx) = cx.add_window_view(|_, _| InlineBaseline(paints.clone()));
    cx.run_until_parked();
    let code = paints.borrow().last().unwrap().surface.clone().unwrap();
    let row = cx.debug_bounds("prose-row").unwrap();
    let (parent, own) = cx.update(|window, cx| {
        let t = cx.theme();
        let size = t.caption_small.size;
        let face = dazzui::tokens::Face::Mono;
        (
            dazzui::baseline(t.body_medium.face, 26., 40., window),
            t.spacing_0_5
                + dazzui::baseline(face, size, dazzui::content_area(face, size, window), window),
        )
    });
    assert_eq!(
        code.bounds.origin.y + gpui::px(own),
        row.origin.y + gpui::px(parent)
    );
}
