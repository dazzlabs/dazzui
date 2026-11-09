//! Anchor activation phases and focus, compared with the React Link.
use dazzui::{Button, Link, Theme};
use gpui::{
    App, ClickEvent, Context, Entity, InteractiveElement, IntoElement, KeyDownEvent, KeyUpEvent,
    KeyboardButton, Keystroke, Modifiers, ParentElement, Render, Styled, TestAppContext,
    VisualTestContext, Window, div,
};

struct Harness {
    clicks: Vec<ClickEvent>,
    next_clicks: usize,
    prevent_down: bool,
    prevent_up: bool,
    has_href: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let link_view = cx.entity().downgrade();
        let next_view = link_view.clone();
        let prevent_down = self.prevent_down;
        let prevent_up = self.prevent_up;
        let mut link = Link::new("tested")
            .child("Read")
            .on_click(move |event, _, cx: &mut App| {
                let _ = link_view.update(cx, |view, cx| {
                    view.clicks.push(event.clone());
                    cx.notify();
                });
            });
        if self.has_href {
            link = link.href("#/read");
        }
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .capture_key_down(move |event, window, _| {
                if prevent_down && event.keystroke.key == "enter" {
                    window.prevent_default();
                }
            })
            .capture_key_up(move |event, window, _| {
                if prevent_up && event.keystroke.key == "enter" {
                    window.prevent_default();
                }
            })
            .child(
                div()
                    .flex()
                    .debug_selector(|| "link-box".into())
                    .child(link),
            )
            .child(Button::new("next").child("Next").on_click(move |_, _, cx| {
                let _ = next_view.update(cx, |view, _| view.next_clicks += 1);
            }))
    }
}
fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        clicks: vec![],
        next_clicks: 0,
        prevent_down: false,
        prevent_up: false,
        has_href: true,
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
fn enter_activates_on_down_with_modifiers_but_not_autorepeat(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    for (i, key) in [
        "enter",
        "shift-enter",
        "ctrl-enter",
        "alt-enter",
        "cmd-enter",
    ]
    .iter()
    .enumerate()
    {
        cx.simulate_keystrokes(key);
        assert_eq!(count(&view, cx), i + 1, "{key} activates on keydown");
        up(cx, key);
        assert_eq!(count(&view, cx), i + 1, "keyup must not activate again");
    }
    cx.simulate_keystrokes("enter");
    for _ in 0..2 {
        cx.simulate_event(KeyDownEvent {
            keystroke: Keystroke::parse("enter").unwrap(),
            is_held: true,
        });
    }
    assert_eq!(
        count(&view, cx),
        6,
        "held Enter does not repeat link activation"
    );
    up(cx, "enter");
    let bounds = cx.debug_bounds("link-box").unwrap();
    view.read_with(cx, |view, _| {
        for event in &view.clicks {
            let ClickEvent::Keyboard(event) = event else {
                panic!("keyboard activation")
            };
            assert_eq!(event.button, KeyboardButton::Enter);
            assert_eq!(event.bounds, bounds);
        }
    });
    cx.simulate_click(bounds.center(), Modifiers::none());
    assert_eq!(count(&view, cx), 7, "pointer activation still fires once");
}

#[gpui::test]
fn space_does_not_activate_a_link_or_arm_the_next_button(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    up(cx, "space");
    cx.simulate_keystrokes("space space");
    up(cx, "space");
    assert_eq!(count(&view, cx), 0);
    cx.simulate_keystrokes("space tab");
    up(cx, "space");
    assert_eq!(count(&view, cx), 0);
    assert_eq!(view.read_with(cx, |view, _| view.next_clicks), 0);
    cx.simulate_keystrokes("enter");
    assert_eq!(view.read_with(cx, |view, _| view.next_clicks), 1);
}

#[gpui::test]
fn canceled_enter_down_does_not_turn_into_a_keyup_activation(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.prevent_down = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("enter");
    up(cx, "enter");
    assert_eq!(count(&view, cx), 0);
    view.update(cx, |view, cx| {
        view.prevent_down = false;
        view.prevent_up = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("enter");
    assert_eq!(count(&view, cx), 1);
    up(cx, "enter");
    assert_eq!(
        count(&view, cx),
        1,
        "canceling keyup does not undo the click"
    );
}

#[gpui::test]
fn removing_href_removes_keyboard_focus_but_keeps_pointer_clicks(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.has_href = false;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("enter");
    up(cx, "enter");
    assert_eq!(count(&view, cx), 0);
    cx.simulate_keystrokes("tab");
    cx.simulate_keystrokes("enter");
    assert_eq!(view.read_with(cx, |view, _| view.next_clicks), 1);
    let bounds = cx.debug_bounds("link-box").unwrap();
    cx.simulate_click(bounds.center(), Modifiers::none());
    assert_eq!(
        count(&view, cx),
        1,
        "non-navigating anchors still receive mouse clicks"
    );
}
