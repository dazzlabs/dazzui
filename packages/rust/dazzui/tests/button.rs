//! Native button key phases, cancellation and real click bounds.

#[allow(dead_code)]
mod support;
use support::paint::PaintProbe;

use dazzui::foundation::ControlColors;
use dazzui::{ActiveTheme, Button, Code, IconButton, KeyCap, Link, State, Theme, Tint, Variant};
use gpui::{
    AnyElement, App, ClickEvent, Context, Entity, InteractiveElement, IntoElement, KeyUpEvent,
    Keystroke, Modifiers, ObservedPaint, ParentElement, Render, Styled, TestAppContext,
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
fn prevented_keyup_retains_native_space_until_an_allowed_release(cx: &mut TestAppContext) {
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
            1,
            "an allowed release completes the retained native press"
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
        // This assertion checks the final state's typography and ink. Motion
        // timing is exercised separately with the deterministic seek clock.
        cx.update(|_, cx| dazzui::Motion::freeze_at(std::time::Duration::ZERO, cx));
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let face = cx.theme().body_medium.face;
        div()
            .flex()
            .items_baseline()
            .font_family(face.family())
            .text_size(gpui::px(26.))
            .line_height(gpui::px(40.))
            .debug_selector(|| "prose-row".into())
            .child(dazzui::foundation::text_child("Run "))
            .child(dazzui::foundation::inline_context(PaintProbe {
                child: Code::new().child("code").into_any_element(),
                paints: self.0.clone(),
            }))
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
    let submitted = paints
        .borrow()
        .last()
        .unwrap()
        .text
        .as_ref()
        .unwrap()
        .baseline
        .y;
    assert!(
        (f32::from(submitted - row.origin.y - gpui::px(parent))).abs() < 0.0001,
        "the glyph painter must use the baseline advertised to layout"
    );
}

struct FractionalLeading {
    paints: [Rc<RefCell<Vec<ObservedPaint>>>; 2],
    ascent: Rc<RefCell<gpui::Pixels>>,
}

impl Render for FractionalLeading {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let family = cx.theme().body_medium.face.family();
        let mut font = gpui::font(family);
        font.weight = gpui::FontWeight::SEMIBOLD;
        let font = window.text_system().resolve_font(&font);
        let size = gpui::px(12.);
        let ascent = window.text_system().ascent(font, size).abs().round();
        let descent = window.text_system().descent(font, size).abs().round();
        *self.ascent.borrow_mut() = ascent;
        // There is less than one pixel above the font content. Rounding the
        // requested leading first would incorrectly add a whole pixel there.
        let leading = ascent + descent + gpui::px(1.8);
        div()
            .flex()
            .flex_col()
            .items_start()
            .font_family(family)
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_size(size)
            .line_height(leading)
            .children(self.paints.iter().enumerate().map(|(index, paints)| {
                PaintProbe {
                    child: div()
                        .child(if index == 0 {
                            "SDK glyphs".into_any_element()
                        } else {
                            dazzui::foundation::text_child("Kit glyphs")
                        })
                        .into_any_element(),
                    paints: paints.clone(),
                }
            }))
    }
}

#[gpui::test]
fn fractional_leading_does_not_round_the_painted_baseline_up(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let paints = std::array::from_fn(|_| Rc::new(RefCell::new(Vec::new())));
    let ascent = Rc::new(RefCell::new(gpui::px(0.)));
    let (_, cx) = cx.add_window_view(|_, _| FractionalLeading {
        paints: paints.clone(),
        ascent: ascent.clone(),
    });
    cx.run_until_parked();
    for paints in paints {
        let paints = paints.borrow();
        let paint = paints.last().unwrap();
        let top = paint.surface.as_ref().unwrap().bounds.origin.y;
        let baseline = paint.text.as_ref().unwrap().baseline.y;
        assert!(
            (f32::from(baseline - top - *ascent.borrow())).abs() < 0.0001,
            "fractional leading changed the baseline: {baseline:?}, top {top:?}"
        );
    }
}

struct MotionControl {
    kind: usize,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for MotionControl {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let child = match self.kind {
            0 => Button::new("motion")
                .variant(Variant::Plain)
                .child("Run")
                .into_any_element(),
            1 => IconButton::new("motion", "Action")
                .child("+")
                .into_any_element(),
            _ => dazzui::Toggle::new("motion")
                .variant(Variant::Outlined)
                .pressed(false)
                .child("Run")
                .into_any_element(),
        };
        div()
            .size_full()
            .p(gpui::px(24.))
            .flex()
            .items_start()
            .child(
                div()
                    .flex()
                    .debug_selector(|| "motion-control".into())
                    .child(PaintProbe {
                        child,
                        paints: self.paints.clone(),
                    }),
            )
    }
}

fn paint_colors(paints: &Rc<RefCell<Vec<ObservedPaint>>>) -> [gpui::Hsla; 3] {
    let paints = paints.borrow();
    let paint = paints.last().unwrap();
    let style = &paint.surface.as_ref().unwrap().style;
    [
        style
            .background
            .as_ref()
            .and_then(|fill| fill.color())
            .and_then(|background| background.solid_color())
            .unwrap_or_else(gpui::transparent_black),
        style.border_color.unwrap_or_else(gpui::transparent_black),
        paint.text.as_ref().unwrap().color,
    ]
}

fn assert_color_bytes(actual: [gpui::Hsla; 3], expected: [u32; 3]) {
    for (actual, expected) in actual.into_iter().zip(expected) {
        let actual = gpui::Rgba::from(actual);
        let expected = expected.to_be_bytes();
        let transparent = expected[3] == 0;
        for (index, (actual, expected)) in [actual.r, actual.g, actual.b, actual.a]
            .into_iter()
            .zip(expected)
            .enumerate()
        {
            if index < 3 && transparent {
                continue;
            }
            assert!(
                (actual * 255. - expected as f32).abs() <= 1.1,
                "painted channel {index}: {} vs {expected}",
                actual * 255.,
            );
        }
    }
}

#[gpui::test]
fn button_icon_button_and_toggle_paint_chromiums_transition_midpoints(cx: &mut TestAppContext) {
    use dazzui::{Motion, MotionPreference};
    use std::time::Duration;
    // Actual computed colors at 75ms in Chromium, from the React Button/0,
    // IconButton preview and outlined Toggle/0. Alpha is rounded to one byte.
    let cases = [
        (
            "bright-light",
            0,
            [0x111c2e10, 0, 0x243247ff],
            [0x111c2e1c, 0, 0x263449eb],
        ),
        (
            "bright-light",
            1,
            [0x111c2e0c, 0, 0x111c2ee0],
            [0x111c2e0f, 0, 0x111c2eff],
        ),
        (
            "bright-light",
            2,
            [0x52525210, 0x444444ff, 0x2c2c2cff],
            [0x5252521c, 0x404040eb, 0x262626eb],
        ),
        (
            "studio-dark",
            0,
            [0x6b4dff10, 0, 0xc9bbffff],
            [0x6b4dff1c, 0, 0xcdc0ffeb],
        ),
        (
            "studio-dark",
            1,
            [0xffffff0e, 0, 0xdbdde9ff],
            [0xffffff12, 0, 0xf2f3faff],
        ),
        (
            "studio-dark",
            2,
            [0x52525210, 0xc9c9c9ff, 0xe1e1e1ff],
            [0x5252521c, 0xd4d4d4eb, 0xe5e5e5eb],
        ),
    ];
    for (theme, kind, hover, press) in cases {
        cx.update(|cx| {
            dazzui::init(cx);
            Theme::set(theme, cx);
            Motion::set_preference(MotionPreference::Full, cx);
            Motion::seek(Duration::ZERO, cx);
        });
        let paints = Rc::new(RefCell::new(Vec::new()));
        let (_, cx) = cx.add_window_view(|_, _| MotionControl {
            kind,
            paints: paints.clone(),
        });
        cx.run_until_parked();
        let initial = paint_colors(&paints);
        let bounds = cx.debug_bounds("motion-control").unwrap();
        cx.simulate_mouse_move(bounds.center(), None, Modifiers::none());
        cx.run_until_parked();
        assert_eq!(
            paint_colors(&paints),
            initial,
            "the first hover frame must not jump"
        );
        cx.update(|_, cx| Motion::seek(Duration::from_millis(75), cx));
        cx.run_until_parked();
        assert_color_bytes(paint_colors(&paints), hover);
        cx.update(|_, cx| Motion::seek(Duration::from_millis(150), cx));
        cx.run_until_parked();
        cx.simulate_mouse_down(bounds.center(), gpui::MouseButton::Left, Modifiers::none());
        cx.run_until_parked();
        cx.update(|_, cx| Motion::seek(Duration::from_millis(225), cx));
        cx.run_until_parked();
        assert_color_bytes(paint_colors(&paints), press);
        assert_eq!(cx.debug_bounds("motion-control").unwrap(), bounds);
    }
}
