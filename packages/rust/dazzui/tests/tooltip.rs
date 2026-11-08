//! Hints remain anchored to the original control and never take its focus.

#[allow(dead_code)]
mod support;

use dazzui::{
    Button, Sidebar, SidebarCollapsible, SidebarMenu, SidebarMenuButton, SidebarMenuItem,
    SidebarProvider, Theme, Tooltip,
};
use gpui::{
    AnyElement, Bounds, Context, Entity, IntoElement, Modifiers, MouseButton, ObservedPaint,
    ParentElement, Pixels, Render, Styled, TestAppContext, VisualTestContext, Window, canvas, div,
    point, prelude::*, px,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};
use support::paint::PaintProbe;

struct Harness {
    open: bool,
    collapsed: Option<bool>,
    standalone: bool,
    painted: Rc<Cell<bool>>,
    label_bounds: Rc<Cell<Bounds<Pixels>>>,
    rows: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.painted.set(false);
        self.rows.borrow_mut().clear();
        let painted = self.painted.clone();
        let bounds = self.label_bounds.clone();
        let label = div()
            .relative()
            .child(dazzui::foundation::text_child("Documents"))
            .child(
                canvas(
                    |_, _, _| {},
                    move |box_, _, _, _| {
                        painted.set(true);
                        bounds.set(box_);
                    },
                )
                .absolute()
                .inset_0(),
            );
        let row: AnyElement = if self.standalone {
            Tooltip::new("hint", label)
                .child(Button::new("target").child("Target"))
                .into_any_element()
        } else {
            SidebarProvider::new("provider")
                .open(self.open)
                .child(
                    Sidebar::new()
                        .collapsible(SidebarCollapsible::Icon)
                        .when_some(self.collapsed, |sidebar, collapsed| {
                            sidebar.collapsed(collapsed)
                        })
                        .child(
                            SidebarMenu::new().child(
                                SidebarMenuItem::new().child(PaintProbe {
                                    child: SidebarMenuButton::new("target")
                                        .tooltip(label)
                                        .child("Documents")
                                        .into_any_element(),
                                    paints: self.rows.clone(),
                                }),
                            ),
                        ),
                )
                .into_any_element()
        };
        let row = if self.standalone {
            PaintProbe {
                child: row,
                paints: self.rows.clone(),
            }
            .into_any_element()
        } else {
            row
        };
        div()
            .size_full()
            .p(px(24.))
            .flex()
            .flex_col()
            .child(
                div()
                    .w(px(420.))
                    .h(px(180.))
                    .flex()
                    .items_start()
                    .child(row),
            )
            .child(Button::new("next").child("Next"))
    }
}

fn setup(cx: &mut TestAppContext, standalone: bool) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("bright-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        open: false,
        collapsed: None,
        standalone,
        painted: Rc::default(),
        label_bounds: Rc::default(),
        rows: Rc::default(),
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    (view, cx)
}

fn drawn(view: &Entity<Harness>, cx: &VisualTestContext) -> bool {
    view.read_with(cx, |view, _| view.painted.get())
}

fn row(view: &Entity<Harness>, cx: &VisualTestContext) -> Bounds<Pixels> {
    view.read_with(cx, |view, _| {
        view.rows
            .borrow()
            .last()
            .unwrap()
            .surface
            .as_ref()
            .unwrap()
            .bounds
    })
}

#[gpui::test]
fn sidebar_hint_uses_provider_state_and_preserves_the_original_row(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, false);
    for (open, collapsed, enabled) in [
        (false, None, true),
        (true, None, false),
        (true, Some(true), false),
        (false, Some(false), true),
    ] {
        cx.simulate_mouse_move(point(px(0.), px(0.)), None, Modifiers::none());
        view.update(cx, |view, cx| {
            view.open = open;
            view.collapsed = collapsed;
            cx.notify();
        });
        cx.run_until_parked();
        let before = row(&view, cx);
        cx.simulate_mouse_move(before.center(), None, Modifiers::none());
        cx.run_until_parked();
        assert_eq!(drawn(&view, cx), enabled);
        assert_eq!(row(&view, cx), before);
        if enabled {
            let label = view.read_with(cx, |view, _| view.label_bounds.get());
            // Four pixels of positioner gap, then eight of padding and a hairline.
            assert_eq!(label.left() - before.right(), px(13.));
            assert!((f32::from(label.center().y - before.center().y)).abs() <= 0.5);
            cx.simulate_keystrokes("escape");
            cx.run_until_parked();
            assert!(
                !drawn(&view, cx),
                "Escape also dismisses an unfocused hover hint"
            );
        }
        cx.simulate_mouse_move(point(px(0.), px(0.)), None, Modifiers::none());
        cx.run_until_parked();
        assert!(!drawn(&view, cx));
    }
}

#[gpui::test]
fn sidebar_hint_handles_focus_escape_click_and_reenable(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, false);
    cx.simulate_keystrokes("tab");
    cx.run_until_parked();
    assert!(drawn(&view, cx));
    cx.simulate_keystrokes("escape");
    cx.run_until_parked();
    assert!(!drawn(&view, cx));
    cx.simulate_keystrokes("tab shift-tab");
    cx.run_until_parked();
    assert!(drawn(&view, cx));
    let target = row(&view, cx);
    cx.simulate_mouse_down(target.center(), MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_up(target.center(), MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    assert!(!drawn(&view, cx));
    cx.simulate_mouse_move(point(px(0.), px(0.)), None, Modifiers::none());
    cx.simulate_mouse_move(target.center(), None, Modifiers::none());
    cx.run_until_parked();
    assert!(drawn(&view, cx));
    view.update(cx, |view, cx| {
        view.open = true;
        cx.notify();
    });
    cx.run_until_parked();
    assert!(!drawn(&view, cx));
    view.update(cx, |view, cx| {
        view.open = false;
        cx.notify();
    });
    cx.run_until_parked();
    assert!(!drawn(&view, cx));
    cx.simulate_mouse_move(point(px(0.), px(0.)), None, Modifiers::none());
    cx.simulate_mouse_move(row(&view, cx).center(), None, Modifiers::none());
    cx.run_until_parked();
    assert!(drawn(&view, cx));
}

#[gpui::test]
fn delayed_hint_cancels_before_open_and_focus_opens_without_delay(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, true);
    let target = row(&view, cx);
    cx.simulate_mouse_move(target.center(), None, Modifiers::none());
    cx.executor().advance_clock(Duration::from_millis(600));
    assert!(drawn(&view, cx));
    cx.simulate_mouse_move(point(px(0.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
    assert!(!drawn(&view, cx));
    cx.simulate_mouse_move(target.center(), None, Modifiers::none());
    cx.run_until_parked();
    assert!(!drawn(&view, cx));
    cx.executor().advance_clock(Duration::from_millis(200));
    cx.simulate_mouse_down(target.center(), MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_up(target.center(), MouseButton::Left, Modifiers::none());
    cx.executor().advance_clock(Duration::from_secs(1));
    assert!(
        !drawn(&view, cx),
        "a cancelled timer must not reopen after a press"
    );
    cx.simulate_keystrokes("tab shift-tab");
    cx.run_until_parked();
    assert!(drawn(&view, cx), "keyboard focus has no hover delay");
}
