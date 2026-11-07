//! Advance actual modals through opening, interrupted exits and unmounting.

use dazzui::{
    Button, Dialog, DialogBody, DialogClose, Drawer, DrawerSide, Length, Motion, MotionPreference,
    Theme,
};
use gpui::{
    App, Context, Entity, FocusHandle, InteractiveElement, IntoElement, Modifiers, ParentElement,
    Render, Styled, TestAppContext, VisualTestContext, Window, canvas, div, px, size,
};
use std::{cell::Cell, rc::Rc, time::Duration};

struct Harness {
    open: bool,
    side: Option<DrawerSide>,
    mounted: Rc<Cell<bool>>,
    changes: usize,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.mounted.set(false);
        let mounted = self.mounted.clone();
        let marker = canvas(move |_, _, _| mounted.set(true), |_, _, _, _| {}).size(px(1.));
        let weak = cx.entity().downgrade();
        let change = move |open, _: &mut Window, cx: &mut App| {
            let _ = weak.update(cx, |view, cx| {
                view.open = open;
                view.changes += 1;
                cx.notify();
            });
        };
        let close = DialogClose::new(
            "close-wrapper",
            div()
                .flex()
                .debug_selector(|| "motion-close".into())
                .child(Button::new("close").child("Close")),
        );
        let trigger = Button::new("opener").child("Open");
        let modal = if let Some(side) = self.side {
            Drawer::new("modal")
                .open(self.open)
                .side(side)
                .size(Length::Px(200.))
                .on_open_change(change)
                .trigger(trigger)
                .child(marker)
                .footer(close)
                .into_any_element()
        } else {
            Dialog::new("modal")
                .open(self.open)
                .on_open_change(change)
                .trigger(trigger)
                .child(DialogBody::new().child(marker).child(close))
                .into_any_element()
        };
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .child(modal)
            .child(Button::new("outside").child("Outside"))
    }
}

fn setup(
    cx: &mut TestAppContext,
    side: Option<DrawerSide>,
) -> (Entity<Harness>, FocusHandle, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
        Motion::set_preference(MotionPreference::Full, cx);
        Motion::seek(Duration::ZERO, cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        open: false,
        side,
        mounted: Rc::default(),
        changes: 0,
    });
    cx.simulate_resize(size(px(1000.), px(800.)));
    cx.run_until_parked();
    cx.simulate_keystrokes("tab");
    let opener = cx.update(|window, cx| window.focused(cx).unwrap());
    cx.simulate_keystrokes("enter");
    (view, opener, cx)
}

fn seek(cx: &mut VisualTestContext, ms: u64) {
    cx.update(|_, cx| Motion::seek(Duration::from_millis(ms), cx));
    cx.run_until_parked();
}

fn mounted(view: &Entity<Harness>, cx: &mut VisualTestContext) -> bool {
    view.read_with(cx, |view, _| view.mounted.get())
}

fn focused(cx: &mut VisualTestContext, focus: &FocusHandle) -> bool {
    cx.update(|window, _| focus.is_focused(window))
}

#[gpui::test]
fn exit_retains_the_sheet_and_focus_until_its_last_frame(cx: &mut TestAppContext) {
    for side in [None, Some(DrawerSide::Right)] {
        let (view, opener, cx) = setup(cx, side);
        assert!(mounted(&view, cx), "the first transparent frame is mounted");
        seek(cx, 150);
        cx.simulate_keystrokes("escape");
        assert!(!view.read_with(cx, |view, _| view.open));
        assert!(mounted(&view, cx));
        assert!(!focused(cx, &opener));
        seek(cx, 225);
        assert!(mounted(&view, cx));
        assert!(!focused(cx, &opener), "focus remains in the closing sheet");
        seek(cx, 300);
        assert!(!mounted(&view, cx));
        assert!(focused(cx, &opener));
        assert_eq!(view.read_with(cx, |view, _| view.changes), 2);
    }
}

#[gpui::test]
fn drawers_slide_by_their_own_measured_width_or_height(cx: &mut TestAppContext) {
    for side in [
        DrawerSide::Left,
        DrawerSide::Right,
        DrawerSide::Top,
        DrawerSide::Bottom,
    ] {
        let (_, _, cx) = setup(cx, Some(side));
        let first = cx.debug_bounds("dz-drawer").unwrap();
        let start = match side {
            DrawerSide::Left => first.origin.x,
            DrawerSide::Right => first.origin.x,
            _ => first.origin.y,
        };
        let expected = match side {
            DrawerSide::Left | DrawerSide::Top => -200.,
            DrawerSide::Right => 1000.,
            DrawerSide::Bottom => 800.,
        };
        assert!((f32::from(start) - expected).abs() < 0.6);
        seek(cx, 75);
        let halfway = cx.debug_bounds("dz-drawer").unwrap();
        let coordinate = match side {
            DrawerSide::Left | DrawerSide::Right => halfway.origin.x,
            _ => halfway.origin.y,
        };
        // Chromium's cubic-bezier(.4,0,.2,1) is .775561 at half time.
        let remaining = 200. * (1. - 0.775561);
        let expected = match side {
            DrawerSide::Left | DrawerSide::Top => -remaining,
            DrawerSide::Right => 800. + remaining,
            DrawerSide::Bottom => 600. + remaining,
        };
        assert!(
            (f32::from(coordinate) - expected).abs() < 0.6,
            "{side:?}: {coordinate:?}"
        );
    }
}

#[gpui::test]
fn moving_controls_receive_pointer_events_at_their_painted_position(cx: &mut TestAppContext) {
    let (view, _, cx) = setup(cx, Some(DrawerSide::Right));
    seek(cx, 75);
    let close = cx.debug_bounds("motion-close").unwrap();
    cx.simulate_click(close.center(), Modifiers::none());
    assert!(!view.read_with(cx, |view, _| view.open));
    assert_eq!(view.read_with(cx, |view, _| view.changes), 2);
}

#[gpui::test]
fn an_interrupted_entry_uses_css_reversal_duration(cx: &mut TestAppContext) {
    let (view, opener, cx) = setup(cx, None);
    seek(cx, 75);
    cx.simulate_keystrokes("escape");
    seek(cx, 190);
    assert!(mounted(&view, cx));
    assert!(!focused(cx, &opener));
    // 75 + 150*.775561 = 191.334ms, rather than a fresh full 150ms exit.
    seek(cx, 192);
    assert!(!mounted(&view, cx));
    assert!(focused(cx, &opener));
}

#[gpui::test]
fn reopening_during_exit_keeps_the_original_return_focus(cx: &mut TestAppContext) {
    let (view, opener, cx) = setup(cx, None);
    seek(cx, 150);
    cx.simulate_keystrokes("escape");
    seek(cx, 200);
    view.update(cx, |view, cx| {
        view.open = true;
        cx.notify();
    });
    cx.run_until_parked();
    assert!(mounted(&view, cx));
    assert!(!focused(cx, &opener));
    seek(cx, 400);
    cx.simulate_keystrokes("escape");
    seek(cx, 550);
    assert!(!mounted(&view, cx));
    assert!(focused(cx, &opener));
}
