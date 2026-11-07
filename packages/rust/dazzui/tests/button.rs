//! Native button key phases, cancellation and real click bounds.

use dazzui::{Button, IconButton, Theme};
use gpui::{
    App, ClickEvent, Context, Entity, InteractiveElement, IntoElement, KeyUpEvent, Keystroke,
    Modifiers, ParentElement, Render, Styled, TestAppContext, VisualTestContext, Window, div,
};

struct Harness {
    icon: bool,
    disabled: bool,
    prevent_down: bool,
    prevent_up: bool,
    clicks: Vec<ClickEvent>,
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
                .disabled(self.disabled)
                .on_click(clicked)
                .child("Action")
                .into_any_element()
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
