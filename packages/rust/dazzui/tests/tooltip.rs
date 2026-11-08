//! Hints remain anchored to the original control and never take its focus.

#[allow(dead_code)]
mod support;

use dazzui::{
    Button, Sidebar, SidebarCollapsible, SidebarMenu, SidebarMenuButton, SidebarMenuItem,
    SidebarProvider, Switch, Theme, Toggle, Tooltip,
};
use gpui::{
    AnyElement, Bounds, Context, Entity, InteractiveElement, IntoElement, KeyUpEvent, Keystroke,
    Modifiers, MouseButton, ObservedPaint, ParentElement, Pixels, Render, Styled, TestAppContext,
    VisualTestContext, Window, canvas, div, point, prelude::*, px,
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
    control: usize,
    has_handler: bool,
    hint: bool,
    disabled: bool,
    prevent_down: bool,
    prevent_up: bool,
    clicks: usize,
    painted: Rc<Cell<bool>>,
    label_bounds: Rc<Cell<Bounds<Pixels>>>,
    rows: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.painted.set(false);
        self.rows.borrow_mut().clear();
        let painted = self.painted.clone();
        let bounds = self.label_bounds.clone();
        let view = cx.entity().downgrade();
        let control_view = view.clone();
        let activate = move |_: &mut Window, cx: &mut gpui::App| {
            let _ = control_view.update(cx, |view, cx| {
                view.clicks += 1;
                cx.notify();
            });
        };
        let prevent_down = self.prevent_down;
        let prevent_up = self.prevent_up;
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
            let control = match self.control {
                1 => Toggle::new("target")
                    .disabled(self.disabled)
                    .when(self.has_handler, |toggle| {
                        toggle.on_pressed_change(move |_, window, cx| activate(window, cx))
                    })
                    .child("Target")
                    .into_any_element(),
                2 => Switch::new("target")
                    .disabled(self.disabled)
                    .when(self.has_handler, |switch| {
                        switch.on_checked_change(move |_, window, cx| activate(window, cx))
                    })
                    .into_any_element(),
                _ => Button::new("target")
                    .disabled(self.disabled)
                    .when(self.has_handler, |button| {
                        button.on_click(move |_, window, cx| activate(window, cx))
                    })
                    .child("Target")
                    .into_any_element(),
            };
            Tooltip::new("hint", label)
                .child(control)
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
                                        .disabled(self.disabled)
                                        .when(self.hint, |row| row.tooltip(label))
                                        .on_click(move |_, _, cx| {
                                            let _ = view.update(cx, |view, cx| {
                                                view.clicks += 1;
                                                cx.notify();
                                            });
                                        })
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
        control: 0,
        has_handler: true,
        hint: true,
        disabled: false,
        prevent_down: false,
        prevent_up: false,
        clicks: 0,
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

fn up(cx: &mut VisualTestContext, key: &str) {
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse(key).unwrap(),
    });
    cx.run_until_parked();
}

fn clicks(view: &Entity<Harness>, cx: &VisualTestContext) -> usize {
    view.read_with(cx, |view, _| view.clicks)
}

#[gpui::test]
fn ordinary_hints_follow_button_toggle_and_switch_activation(cx: &mut TestAppContext) {
    for control in 0..3 {
        let (view, cx) = setup(cx, true);
        view.update(cx, |view, cx| {
            view.control = control;
            cx.notify();
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("tab");
        assert!(drawn(&view, cx));
        up(cx, "enter");
        assert_eq!(clicks(&view, cx), 0);
        assert!(drawn(&view, cx), "a stray Enter release is inert");
        up(cx, "space");
        let stray = usize::from(control == 2);
        assert_eq!(clicks(&view, cx), stray);
        assert_eq!(drawn(&view, cx), stray == 0);
        if stray != 0 {
            cx.simulate_keystrokes("tab shift-tab");
        }
        cx.simulate_keystrokes("enter");
        assert_eq!(clicks(&view, cx), stray + 1);
        assert!(
            !drawn(&view, cx),
            "Enter closes on keydown for control {control}"
        );
        up(cx, "enter");
        assert_eq!(clicks(&view, cx), stray + 1);
        cx.simulate_keystrokes("tab shift-tab space");
        assert_eq!(clicks(&view, cx), stray + 1);
        assert!(drawn(&view, cx), "Space down is not activation");
        up(cx, "space");
        assert_eq!(clicks(&view, cx), stray + 2);
        assert!(!drawn(&view, cx));
        cx.simulate_keystrokes("tab shift-tab");
        view.update(cx, |view, cx| {
            view.prevent_down = true;
            cx.notify();
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("enter");
        up(cx, "enter");
        assert_eq!(clicks(&view, cx), stray + 2);
        assert!(
            drawn(&view, cx),
            "a prevented Enter must not dismiss on keyup"
        );
        cx.simulate_keystrokes("space");
        up(cx, "space");
        let base = stray + 2 + usize::from(control == 2);
        assert_eq!(clicks(&view, cx), base);
        assert_eq!(drawn(&view, cx), control != 2);
        if control == 2 {
            cx.simulate_keystrokes("tab shift-tab");
        }
        view.update(cx, |view, cx| {
            view.prevent_down = false;
            view.prevent_up = true;
            cx.notify();
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("space");
        up(cx, "space");
        assert_eq!(clicks(&view, cx), base);
        assert!(drawn(&view, cx));
        view.update(cx, |view, cx| {
            view.prevent_up = false;
            cx.notify();
        });
        cx.run_until_parked();
        up(cx, "space");
        assert_eq!(clicks(&view, cx), base + 1);
        assert!(!drawn(&view, cx));
        view.update(cx, |view, cx| {
            view.has_handler = false;
            cx.notify();
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("tab shift-tab");
        assert!(drawn(&view, cx));
        cx.simulate_keystrokes("enter");
        assert_eq!(clicks(&view, cx), base + 1);
        assert!(
            !drawn(&view, cx),
            "a control without a callback still activates"
        );
        up(cx, "enter");
    }
}

#[gpui::test]
fn sidebar_rows_activate_on_native_key_phases_and_close_hints_on_activation(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx, false);
    cx.simulate_keystrokes("tab");
    for hint in [true, false] {
        view.update(cx, |view, cx| {
            view.hint = hint;
            view.clicks = 0;
            cx.notify();
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("tab shift-tab");
        up(cx, "enter");
        up(cx, "space");
        assert_eq!(clicks(&view, cx), 0);
        assert_eq!(drawn(&view, cx), hint);
        cx.simulate_keystrokes("enter");
        assert_eq!(clicks(&view, cx), 1, "Enter activates on keydown");
        assert!(
            !drawn(&view, cx),
            "activation closes the hint on that same phase"
        );
        cx.simulate_keystrokes("enter");
        assert_eq!(clicks(&view, cx), 2, "held Enter repeats");
        up(cx, "enter");
        assert_eq!(clicks(&view, cx), 2);
        cx.simulate_keystrokes("tab shift-tab space space");
        assert_eq!(
            clicks(&view, cx),
            2,
            "Space repeats only arm one activation"
        );
        assert_eq!(drawn(&view, cx), hint);
        up(cx, "space");
        assert_eq!(clicks(&view, cx), 3);
        assert!(!drawn(&view, cx));
        up(cx, "space");
        assert_eq!(clicks(&view, cx), 3, "a second release is inert");
        let target = row(&view, cx);
        cx.simulate_mouse_down(target.center(), MouseButton::Left, Modifiers::none());
        cx.simulate_mouse_up(target.center(), MouseButton::Left, Modifiers::none());
        assert_eq!(clicks(&view, cx), 4, "the pointer still activates once");
    }
}

#[gpui::test]
fn sidebar_rows_preserve_prevented_release_but_cancel_on_blur_and_disabling(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx, false);
    cx.simulate_keystrokes("tab");
    view.update(cx, |view, cx| {
        view.prevent_down = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("enter space");
    up(cx, "enter");
    up(cx, "space");
    assert_eq!(clicks(&view, cx), 0);
    assert!(drawn(&view, cx), "prevented keys do not dismiss the hint");
    view.update(cx, |view, cx| {
        view.prevent_down = false;
        view.prevent_up = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("space");
    up(cx, "space");
    assert_eq!(clicks(&view, cx), 0);
    view.update(cx, |view, cx| {
        view.prevent_up = false;
        cx.notify();
    });
    cx.run_until_parked();
    up(cx, "space");
    assert_eq!(
        clicks(&view, cx),
        1,
        "an allowed release completes the retained press"
    );
    assert!(!drawn(&view, cx));
    cx.simulate_keystrokes("space tab");
    up(cx, "space");
    cx.simulate_keystrokes("shift-tab");
    up(cx, "space");
    assert_eq!(clicks(&view, cx), 1, "blur cancels a pending Space");
    cx.simulate_keystrokes("space");
    view.update(cx, |view, cx| {
        view.disabled = true;
        cx.notify();
    });
    cx.run_until_parked();
    up(cx, "space");
    view.update(cx, |view, cx| {
        view.disabled = false;
        cx.notify();
    });
    cx.run_until_parked();
    up(cx, "space");
    assert_eq!(clicks(&view, cx), 1, "disabling cancels a pending Space");
}
