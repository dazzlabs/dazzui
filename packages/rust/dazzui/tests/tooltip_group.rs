//! Shared hint timing is exercised through real hover, focus and popup paint.

use dazzui::components::popup::Side;
use dazzui::{Button, SidebarProvider, Theme, Tooltip, TooltipProvider};
use gpui::{
    AnyElement, Context, Entity, IntoElement, Modifiers, MouseButton, ParentElement, Render,
    Styled, TestAppContext, VisualTestContext, Window, canvas, div, point, prelude::*, px,
};
use std::{cell::Cell, rc::Rc, time::Duration};

struct Harness {
    delay: Option<Duration>,
    close_delay: Option<Duration>,
    own_delay: Option<Duration>,
    own_close: Option<Duration>,
    timeout: Duration,
    nested: bool,
    sidebar: bool,
    first: bool,
    present: bool,
    side: Side,
    disabled: bool,
    painted: [Rc<Cell<bool>>; 3],
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        for painted in &self.painted {
            painted.set(false);
        }
        let hint = |index: usize, name: &'static str| {
            let painted = self.painted[index].clone();
            Tooltip::new(
                name,
                div()
                    .debug_selector(move || format!("{name}-popup").into())
                    .child(name)
                    .child(
                        canvas(|_, _, _| {}, move |_, _, _, _| painted.set(true))
                            .absolute()
                            .inset_0(),
                    ),
            )
            .side(self.side)
            .disabled(self.disabled)
            .when_some(self.own_delay, |hint, delay| hint.delay(delay))
            .when_some(self.own_close, |hint, delay| hint.close_delay(delay))
            .child(Button::new("button").child(name))
            .into_any_element()
        };
        let target = |index, name: &'static str| {
            div()
                .flex()
                .debug_selector(move || name.into())
                .child(hint(index, name))
        };
        let third = target(2, "third").into_any_element();
        let third = if self.nested {
            TooltipProvider::new("nested", third).into_any_element()
        } else {
            third
        };
        let row = div()
            .flex()
            .gap(px(60.))
            .when(self.first, |row| row.child(target(0, "first")))
            .child(target(1, "second"))
            .child(third);
        let row = div().flex().children(self.present.then_some(row));
        let content: AnyElement = if self.sidebar {
            SidebarProvider::new("sidebar")
                .child(row)
                .into_any_element()
        } else {
            TooltipProvider::new("provider", row)
                .timeout(self.timeout)
                .when_some(self.delay, |provider, delay| provider.delay(delay))
                .when_some(self.close_delay, |provider, delay| {
                    provider.close_delay(delay)
                })
                .into_any_element()
        };
        div().size_full().p(px(60.)).child(content)
    }
}

fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("bright-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        delay: None,
        close_delay: None,
        own_delay: None,
        own_close: None,
        timeout: Duration::from_millis(400),
        nested: false,
        sidebar: false,
        first: true,
        present: true,
        side: Side::Top,
        disabled: false,
        painted: std::array::from_fn(|_| Rc::default()),
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    (view, cx)
}

fn hover(cx: &mut VisualTestContext, target: &'static str) {
    let at = cx.debug_bounds(target).unwrap().center();
    cx.simulate_mouse_move(at, None, Modifiers::none());
    cx.run_until_parked();
}
fn leave(cx: &mut VisualTestContext) {
    cx.simulate_mouse_move(point(px(0.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
}
fn painted(view: &Entity<Harness>, cx: &VisualTestContext) -> [bool; 3] {
    view.read_with(cx, |view, _| {
        std::array::from_fn(|index| view.painted[index].get())
    })
}
fn ms(value: u64) -> Duration {
    Duration::from_millis(value)
}

#[gpui::test]
fn group_waits_once_then_switches_immediately_and_resets_after_close(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    hover(cx, "first");
    cx.executor().advance_clock(ms(599));
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(1));
    assert_eq!(painted(&view, cx), [true, false, false]);
    hover(cx, "second");
    assert_eq!(painted(&view, cx), [false, true, false]);
    leave(cx);
    cx.executor().advance_clock(ms(399));
    hover(cx, "third");
    assert_eq!(painted(&view, cx), [false, false, true]);
    leave(cx);
    cx.executor().advance_clock(ms(400));
    hover(cx, "first");
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(600));
    assert_eq!(painted(&view, cx), [true, false, false]);
}

#[gpui::test]
fn focus_warms_the_group_and_a_new_hint_closes_one_with_a_pending_close(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.close_delay = Some(ms(300));
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("tab");
    cx.run_until_parked();
    assert_eq!(painted(&view, cx), [true, false, false]);
    hover(cx, "second");
    assert_eq!(painted(&view, cx), [false, true, false]);
    leave(cx);
    cx.executor().advance_clock(ms(299));
    assert_eq!(painted(&view, cx), [false, true, false]);
    cx.executor().advance_clock(ms(1));
    assert_eq!(painted(&view, cx), [false; 3]);
    // A delayed blur from the old focused trigger must not end the new group.
    hover(cx, "third");
    assert_eq!(painted(&view, cx), [false, false, true]);
}

#[gpui::test]
fn providers_are_independent_and_open_unmount_releases_the_instant_phase(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.nested = true;
        cx.notify();
    });
    cx.run_until_parked();
    hover(cx, "first");
    cx.executor().advance_clock(ms(600));
    hover(cx, "third");
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(600));
    assert_eq!(painted(&view, cx), [false, false, true]);
    leave(cx);
    cx.executor().advance_clock(ms(400));
    hover(cx, "first");
    cx.executor().advance_clock(ms(600));
    view.update(cx, |view, cx| {
        view.first = false;
        cx.notify();
    });
    cx.run_until_parked();
    leave(cx);
    hover(cx, "second");
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(600));
    assert_eq!(painted(&view, cx), [false, true, false]);
}

#[gpui::test]
fn child_delays_override_provider_except_during_the_zero_delay_phase(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.delay = Some(ms(300));
        view.own_delay = Some(ms(100));
        view.close_delay = Some(ms(300));
        view.own_close = Some(ms(100));
        view.timeout = Duration::ZERO;
        cx.notify();
    });
    cx.run_until_parked();
    hover(cx, "first");
    cx.executor().advance_clock(ms(99));
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(1));
    assert_eq!(painted(&view, cx), [true, false, false]);
    leave(cx);
    cx.executor().advance_clock(ms(100));
    hover(cx, "second");
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(100));
    assert_eq!(painted(&view, cx), [false, true, false]);
    leave(cx);
    cx.executor().advance_clock(ms(100));
    view.update(cx, |view, cx| {
        view.delay = Some(Duration::ZERO);
        cx.notify();
    });
    cx.run_until_parked();
    hover(cx, "first");
    assert_eq!(painted(&view, cx), [true, false, false]);
}

#[gpui::test]
fn sidebar_provider_makes_ordinary_tooltips_immediate(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.sidebar = true;
        view.own_delay = Some(ms(900));
        cx.notify();
    });
    cx.run_until_parked();
    hover(cx, "first");
    assert_eq!(painted(&view, cx), [true, false, false]);
}

#[gpui::test]
fn changing_provider_settings_and_unmounting_a_closed_hint_preserve_the_grace_period(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    hover(cx, "first");
    cx.executor().advance_clock(ms(600));
    view.update(cx, |view, cx| {
        view.delay = Some(ms(900));
        cx.notify();
    });
    cx.run_until_parked();
    hover(cx, "second");
    assert_eq!(painted(&view, cx), [false, true, false]);
    hover(cx, "first");
    leave(cx);
    view.update(cx, |view, cx| {
        view.first = false;
        cx.notify();
    });
    cx.run_until_parked();
    cx.executor().advance_clock(ms(399));
    hover(cx, "third");
    assert_eq!(painted(&view, cx), [false, false, true]);
    leave(cx);
    cx.executor().advance_clock(ms(400));
    hover(cx, "second");
    cx.executor().advance_clock(ms(899));
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(1));
    assert_eq!(painted(&view, cx), [false, true, false]);
}

#[gpui::test]
fn a_keyboard_hint_starts_the_same_reset_timeout_when_it_blurs(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    cx.simulate_keystrokes("tab");
    cx.run_until_parked();
    assert_eq!(painted(&view, cx), [true, false, false]);
    cx.update(|window, _| window.blur());
    cx.run_until_parked();
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(400));
    hover(cx, "second");
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(600));
    assert_eq!(painted(&view, cx), [false, true, false]);
}

#[gpui::test]
fn hover_open_waits_for_rest_ignores_jitter_and_respects_explicit_dismissal(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    let at = cx.debug_bounds("first").unwrap().center();
    hover(cx, "first");
    cx.executor().advance_clock(ms(500));
    cx.simulate_mouse_move(at + point(px(8.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
    cx.executor().advance_clock(ms(599));
    assert_eq!(
        painted(&view, cx),
        [false; 3],
        "movement restarts the rest timer"
    );
    cx.simulate_mouse_move(at + point(px(9.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
    cx.executor().advance_clock(ms(1));
    assert_eq!(
        painted(&view, cx),
        [true, false, false],
        "one-pixel jitter preserves the timer"
    );
    cx.simulate_mouse_move(at - point(px(8.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
    assert_eq!(painted(&view, cx), [true, false, false]);
    cx.simulate_keystrokes("escape");
    cx.run_until_parked();
    hover(cx, "first");
    cx.executor().advance_clock(ms(700));
    assert_eq!(
        painted(&view, cx),
        [false; 3],
        "movement must not undo dismissal"
    );
    leave(cx);
    hover(cx, "first");
    cx.executor().advance_clock(ms(600));
    assert_eq!(painted(&view, cx), [true, false, false]);
}

#[gpui::test]
fn a_hint_mounted_under_the_pointer_waits_for_actual_motion(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    let at = cx.debug_bounds("first").unwrap().center();
    view.update(cx, |view, cx| {
        view.present = false;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_mouse_move(at, None, Modifiers::none());
    cx.run_until_parked();
    view.update(cx, |view, cx| {
        view.present = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.executor().advance_clock(ms(700));
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.simulate_mouse_move(at + point(px(1.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
    cx.executor().advance_clock(ms(599));
    assert_eq!(painted(&view, cx), [false; 3]);
    cx.executor().advance_clock(ms(1));
    assert_eq!(painted(&view, cx), [true, false, false]);
}

#[gpui::test]
fn activating_before_a_hover_open_blocks_further_motion_until_reentry(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    let at = cx.debug_bounds("first").unwrap().center();
    hover(cx, "first");
    cx.executor().advance_clock(ms(200));
    cx.simulate_mouse_down(at, MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_up(at, MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_move(at + point(px(8.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
    cx.executor().advance_clock(ms(1000));
    assert_eq!(painted(&view, cx), [false; 3]);
    leave(cx);
    hover(cx, "first");
    cx.executor().advance_clock(ms(600));
    assert_eq!(painted(&view, cx), [true, false, false]);
}

#[gpui::test]
fn pointer_can_cross_the_gap_and_rest_inside_a_hint_on_every_side(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    for side in [Side::Top, Side::Bottom, Side::Left, Side::Right] {
        leave(cx);
        view.update(cx, |view, cx| {
            view.side = side;
            view.delay = Some(Duration::ZERO);
            cx.notify();
        });
        cx.run_until_parked();
        hover(cx, "second");
        assert_eq!(painted(&view, cx), [false, true, false]);
        let trigger = cx.debug_bounds("second").unwrap();
        let popup = cx.debug_bounds("second-popup").unwrap();
        let gap = match side {
            Side::Top => point(trigger.center().x, trigger.top() - px(2.)),
            Side::Bottom => point(trigger.center().x, trigger.bottom() + px(2.)),
            Side::Left => point(trigger.left() - px(2.), trigger.center().y),
            Side::Right => point(trigger.right() + px(2.), trigger.center().y),
        };
        cx.simulate_mouse_move(gap, None, Modifiers::none());
        cx.run_until_parked();
        cx.executor().advance_clock(ms(80));
        assert_eq!(painted(&view, cx), [false, true, false], "gap on {side:?}");
        cx.simulate_mouse_move(popup.center(), None, Modifiers::none());
        cx.run_until_parked();
        cx.executor().advance_clock(ms(80));
        assert_eq!(
            painted(&view, cx),
            [false, true, false],
            "popup on {side:?}"
        );
        leave(cx);
        assert_eq!(painted(&view, cx), [false; 3]);
    }
}

#[gpui::test]
fn disabled_hints_require_reentry_when_enabled_again(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.disabled = true;
        cx.notify();
    });
    cx.run_until_parked();
    hover(cx, "first");
    view.update(cx, |view, cx| {
        view.disabled = false;
        cx.notify();
    });
    cx.run_until_parked();
    let at = cx.debug_bounds("first").unwrap().center();
    cx.simulate_mouse_move(at + point(px(8.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
    cx.executor().advance_clock(ms(700));
    assert_eq!(painted(&view, cx), [false; 3]);
    leave(cx);
    hover(cx, "first");
    cx.executor().advance_clock(ms(600));
    assert_eq!(painted(&view, cx), [true, false, false]);
}

#[gpui::test]
fn reentering_the_popup_cancels_its_delayed_close(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.delay = Some(Duration::ZERO);
        view.close_delay = Some(ms(150));
        cx.notify();
    });
    cx.run_until_parked();
    hover(cx, "second");
    let popup = cx.debug_bounds("second-popup").unwrap();
    cx.simulate_mouse_move(popup.center(), None, Modifiers::none());
    cx.run_until_parked();
    leave(cx);
    cx.executor().advance_clock(ms(149));
    assert_eq!(painted(&view, cx), [false, true, false]);
    cx.simulate_mouse_move(popup.center(), None, Modifiers::none());
    cx.run_until_parked();
    cx.executor().advance_clock(ms(200));
    assert_eq!(painted(&view, cx), [false, true, false]);
    leave(cx);
    cx.executor().advance_clock(ms(150));
    assert_eq!(painted(&view, cx), [false; 3]);
}

#[gpui::test]
fn diagonal_travel_intent_expires_when_the_pointer_stops_outside_the_corridor(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.delay = Some(Duration::ZERO);
        cx.notify();
    });
    cx.run_until_parked();
    hover(cx, "second");
    let trigger = cx.debug_bounds("second").unwrap();
    let outside = point(trigger.right() + px(1.), trigger.top() + px(1.));
    cx.simulate_mouse_move(outside, None, Modifiers::none());
    cx.run_until_parked();
    assert_eq!(painted(&view, cx), [false, true, false]);
    cx.executor().advance_clock(ms(39));
    assert_eq!(painted(&view, cx), [false, true, false]);
    cx.executor().advance_clock(ms(1));
    assert_eq!(painted(&view, cx), [false; 3]);
}
