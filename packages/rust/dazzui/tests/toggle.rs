//! Toggle activation follows a native button, independently of its latch.

#[allow(dead_code)]
mod support;

use dazzui::foundation::ControlColors;
use dazzui::{
    ActiveTheme, Button, Motion, MotionPreference, State, Theme, Tint, Toggle, ToggleGroup,
    ToggleItem, Variant,
};
use gpui::{
    Context, Entity, InteractiveElement, IntoElement, KeyUpEvent, Keystroke, ObservedPaint,
    ParentElement, Render, Styled, TestAppContext, VisualTestContext, Window, div, prelude::*, px,
};
use std::{cell::RefCell, rc::Rc};
use support::paint::PaintProbe;

struct Harness {
    controlled: bool,
    accept: bool,
    pressed: bool,
    disabled: bool,
    prevent_down: bool,
    prevent_up: bool,
    changes: Vec<bool>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        let toggle = Toggle::new("tested")
            .variant(Variant::Outlined)
            .disabled(self.disabled)
            .when(self.controlled, |toggle| toggle.pressed(self.pressed))
            .on_pressed_change(move |pressed, _, cx| {
                let _ = view.update(cx, |view, cx| {
                    view.changes.push(pressed);
                    if view.accept {
                        view.pressed = pressed;
                    }
                    cx.notify();
                });
            })
            .child("Wrap");
        let prevent_down = self.prevent_down;
        let prevent_up = self.prevent_up;
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .p(px(24.))
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
                    .debug_selector(|| "toggle".into())
                    .child(PaintProbe {
                        child: toggle.into_any_element(),
                        paints: self.paints.clone(),
                    }),
            )
            .child(Button::new("next").child("Next"))
    }
}

fn setup(cx: &mut TestAppContext, controlled: bool) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
        Motion::set_preference(MotionPreference::Reduced, cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        controlled,
        accept: true,
        pressed: false,
        disabled: false,
        prevent_down: false,
        prevent_up: false,
        changes: vec![],
        paints: Rc::new(RefCell::new(Vec::new())),
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

fn changes(view: &Entity<Harness>, cx: &mut VisualTestContext) -> Vec<bool> {
    view.read_with(cx, |view, _| view.changes.clone())
}

fn assert_ink(view: &Entity<Harness>, cx: &mut VisualTestContext, variant: Variant, state: State) {
    let paint = view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone());
    let expected =
        cx.update(|_, cx| ControlColors::resolve(cx.theme(), variant, Tint::Neutral, state));
    assert_eq!(paint.text.unwrap().color, expected.content);
}

#[gpui::test]
fn keyboard_activation_and_transient_press_match_the_react_toggle(cx: &mut TestAppContext) {
    for controlled in [false, true] {
        let (view, cx) = setup(cx, controlled);
        let bounds = cx.debug_bounds("toggle").unwrap();
        up(cx, "space");
        up(cx, "enter");
        assert!(
            changes(&view, cx).is_empty(),
            "release without a press is inert"
        );
        cx.simulate_keystrokes("space space");
        assert!(
            changes(&view, cx).is_empty(),
            "Space repeats must not change the latch"
        );
        assert_ink(&view, cx, Variant::Outlined, State::Pressed);
        assert_eq!(cx.debug_bounds("toggle").unwrap(), bounds);
        up(cx, "space");
        assert_eq!(changes(&view, cx), [true]);
        assert_ink(&view, cx, Variant::Tinted, State::Normal);
        cx.simulate_keystrokes("enter");
        assert_eq!(
            changes(&view, cx),
            [true, false],
            "Enter activates on keydown"
        );
        cx.simulate_keystrokes("enter");
        assert_eq!(changes(&view, cx), [true, false, true]);
        up(cx, "enter");
        assert_eq!(changes(&view, cx), [true, false, true]);
    }
}

#[gpui::test]
fn pending_space_cancels_on_blur_disabling_and_prevented_key_events(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, true);
    cx.simulate_keystrokes("space tab");
    up(cx, "space");
    assert!(changes(&view, cx).is_empty());
    cx.simulate_keystrokes("shift-tab space");
    view.update(cx, |view, cx| {
        view.disabled = true;
        cx.notify();
    });
    cx.run_until_parked();
    view.update(cx, |view, cx| {
        view.disabled = false;
        cx.notify();
    });
    cx.run_until_parked();
    up(cx, "space");
    assert!(changes(&view, cx).is_empty());
    view.update(cx, |view, cx| {
        view.prevent_down = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("enter space");
    up(cx, "space");
    assert!(changes(&view, cx).is_empty());
    view.update(cx, |view, cx| {
        view.prevent_down = false;
        view.prevent_up = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("space");
    up(cx, "space");
    assert!(changes(&view, cx).is_empty());
    assert_ink(&view, cx, Variant::Outlined, State::Normal);
}

#[gpui::test]
fn controlled_toggle_can_refuse_a_change_without_latching(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, true);
    view.update(cx, |view, _| view.accept = false);
    cx.simulate_keystrokes("enter enter");
    up(cx, "enter");
    assert_eq!(changes(&view, cx), [true, true]);
    assert_ink(&view, cx, Variant::Outlined, State::Normal);
}

struct GroupHarness {
    value: Vec<gpui::SharedString>,
    multiple: bool,
    changes: usize,
}

impl Render for GroupHarness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .child(
                ToggleGroup::new(
                    "group",
                    vec![
                        ToggleItem::new("a", "Alpha"),
                        ToggleItem::new("b", "Beta").disabled(true),
                        ToggleItem::new("c", "Gamma"),
                    ],
                )
                .value(self.value.clone())
                .multiple(self.multiple)
                .on_value_change(move |value, _, cx| {
                    let _ = view.update(cx, |view, cx| {
                        view.value = value;
                        view.changes += 1;
                        cx.notify();
                    });
                }),
            )
            .child(Button::new("next").child("Next"))
    }
}

#[gpui::test]
fn grouped_toggles_use_composite_space_timing_and_skip_disabled_members(cx: &mut TestAppContext) {
    for multiple in [false, true] {
        cx.update(|cx| {
            dazzui::init(cx);
            Motion::set_preference(MotionPreference::Reduced, cx);
        });
        let (view, cx) = cx.add_window_view(|_, _| GroupHarness {
            value: vec!["a".into()],
            multiple,
            changes: 0,
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("tab right space");
        let expected: Vec<gpui::SharedString> = if multiple {
            vec!["a".into(), "c".into()]
        } else {
            vec!["c".into()]
        };
        assert_eq!(view.read_with(cx, |view, _| view.value.clone()), expected);
        assert_eq!(
            view.read_with(cx, |view, _| view.changes),
            1,
            "a composite toggles on Space keydown"
        );
        cx.simulate_keystrokes("space");
        let expected: Vec<gpui::SharedString> = if multiple { vec!["a".into()] } else { vec![] };
        assert_eq!(view.read_with(cx, |view, _| view.value.clone()), expected);
        up(cx, "space");
        assert_eq!(
            view.read_with(cx, |view, _| view.changes),
            2,
            "Space repeats activate; releasing Space does not activate again"
        );
        cx.simulate_keystrokes("enter");
        assert_eq!(view.read_with(cx, |view, _| view.changes), 3);
        up(cx, "enter");
        assert_eq!(view.read_with(cx, |view, _| view.changes), 3);
        cx.simulate_keystrokes("left enter");
        assert_eq!(view.read_with(cx, |view, _| view.changes), 4);
    }
}
