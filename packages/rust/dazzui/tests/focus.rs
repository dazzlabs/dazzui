//! Tab traversal and modal focus use native focus handles, including controls
//! a host supplies itself rather than only the kit's own components.

use dazzui::{Button, Dialog, DialogBody, Drawer, Select, SelectOption};
use gpui::{
    App, Context, Entity, FocusHandle, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, TestAppContext, VisualTestContext, Window, div, px,
};

struct Harness {
    opener: FocusHandle,
    outside: FocusHandle,
    first: FocusHandle,
    last: FocusHandle,
    nested_first: FocusHandle,
    nested_last: FocusHandle,
    nested: bool,
    intercept_tab: bool,
    popup: bool,
    selected: Option<String>,
    open: bool,
    drawer: bool,
    empty: bool,
}

fn target(id: &'static str, focus: &FocusHandle) -> impl IntoElement {
    div()
        .id(id)
        .track_focus(&focus.clone().tab_stop(true))
        .w(px(80.))
        .h(px(24.))
        .child(id)
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        let nested_view = view.clone();
        let select_view = view.clone();
        let change = move |open, _: &mut Window, cx: &mut App| {
            let _ = view.update(cx, |view, cx| {
                view.open = open;
                cx.notify();
            });
        };
        let content = div().flex().flex_col().when(!self.empty, |this| {
            this.child(target("first", &self.first))
                .child(Button::new("disabled").disabled(true).child("Skipped"))
                .child(target("last", &self.last))
                .when(self.popup, |this| {
                    this.child(
                        Select::new("select")
                            .options([
                                SelectOption::new("a", "Alpha"),
                                SelectOption::new("b", "Beta"),
                            ])
                            .default_value("a")
                            .on_value_change(move |value, _, cx| {
                                let _ = select_view.update(cx, |view, cx| {
                                    view.selected = Some(value.to_string());
                                    cx.notify();
                                });
                            }),
                    )
                })
                .child(
                    Dialog::new("nested")
                        .open(self.nested)
                        .on_open_change(move |open, _, cx| {
                            let _ = nested_view.update(cx, |view, cx| {
                                view.nested = open;
                                cx.notify();
                            });
                        })
                        .child(target("nested-first", &self.nested_first))
                        .child(target("nested-last", &self.nested_last)),
                )
        });
        let trigger = target("opener", &self.opener);
        let modal = if self.drawer {
            Drawer::new("drawer")
                .open(self.open)
                .on_open_change(change)
                .trigger(trigger)
                .child(content)
                .into_any_element()
        } else {
            Dialog::new("dialog")
                .open(self.open)
                .on_open_change(change)
                .trigger(trigger)
                .child(DialogBody::new().child(content))
                .into_any_element()
        };
        let intercept = self.intercept_tab;
        div()
            .size_full()
            .on_key_down(move |event, _, cx| {
                if intercept && event.keystroke.key == "tab" {
                    cx.stop_propagation();
                }
            })
            .child(modal)
            .child(target("outside", &self.outside))
    }
}

use gpui::prelude::FluentBuilder;

fn setup(
    cx: &mut TestAppContext,
    drawer: bool,
    empty: bool,
) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        dazzui::Motion::set_preference(dazzui::MotionPreference::Reduced, cx);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        opener: cx.focus_handle(),
        outside: cx.focus_handle(),
        first: cx.focus_handle(),
        last: cx.focus_handle(),
        nested_first: cx.focus_handle(),
        nested_last: cx.focus_handle(),
        nested: false,
        intercept_tab: false,
        popup: false,
        selected: None,
        open: false,
        drawer,
        empty,
    });
    cx.run_until_parked();
    (view, cx)
}

fn focused(
    view: &Entity<Harness>,
    cx: &mut VisualTestContext,
    get: fn(&Harness) -> &FocusHandle,
) -> bool {
    let handle = view.read_with(cx, |view, _| get(view).clone());
    cx.update(|window, _| handle.is_focused(window))
}

fn open(view: &Entity<Harness>, cx: &mut VisualTestContext) {
    view.update(cx, |view, _| view.open = true);
    cx.update(|window, _| window.refresh());
    cx.run_until_parked();
}

#[gpui::test]
fn tab_visits_controls_and_shift_tab_reverses(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, false, false);
    cx.simulate_keystrokes("tab");
    assert!(focused(&view, cx, |view| &view.opener));
    cx.simulate_keystrokes("tab");
    assert!(focused(&view, cx, |view| &view.outside));
    cx.simulate_keystrokes("shift-tab");
    assert!(focused(&view, cx, |view| &view.opener));
}

fn modal_focus(cx: &mut TestAppContext, drawer: bool) {
    let (view, cx) = setup(cx, drawer, false);
    cx.simulate_keystrokes("tab");
    open(&view, cx);
    if drawer {
        assert!(
            !focused(&view, cx, |view| &view.first),
            "Drawer initially focuses its panel"
        );
        cx.simulate_keystrokes("tab");
    }
    assert!(
        focused(&view, cx, |view| &view.first),
        "first control on opening"
    );
    cx.simulate_keystrokes("tab");
    assert!(
        focused(&view, cx, |view| &view.last),
        "disabled control skipped"
    );
    cx.simulate_keystrokes("tab");
    assert!(
        focused(&view, cx, |view| &view.first),
        "forward wrap stays in the modal"
    );
    cx.simulate_keystrokes("shift-tab");
    assert!(
        focused(&view, cx, |view| &view.last),
        "reverse wrap stays in the modal"
    );
    cx.simulate_keystrokes("escape");
    assert!(
        focused(&view, cx, |view| &view.opener),
        "closing restores the trigger"
    );
}

#[gpui::test]
fn dialog_contains_keyboard_focus(cx: &mut TestAppContext) {
    modal_focus(cx, false);
}

#[gpui::test]
fn drawer_contains_keyboard_focus(cx: &mut TestAppContext) {
    modal_focus(cx, true);
}

#[gpui::test]
fn an_empty_dialog_retains_focus_without_looping(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, false, true);
    cx.simulate_keystrokes("tab");
    open(&view, cx);
    let initial = cx.update(|window, cx| window.focused(cx));
    cx.simulate_keystrokes("tab shift-tab tab");
    assert_eq!(cx.update(|window, cx| window.focused(cx)), initial);
    assert!(!focused(&view, cx, |view| &view.outside));
    cx.simulate_keystrokes("escape");
    assert!(focused(&view, cx, |view| &view.opener));
}

#[gpui::test]
fn a_host_can_consume_tab_without_the_fallback_moving_focus(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, false, false);
    cx.simulate_keystrokes("tab");
    view.update(cx, |view, cx| {
        view.intercept_tab = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("tab");
    assert!(focused(&view, cx, |view| &view.opener));
}

#[gpui::test]
fn nested_dialog_restores_its_parent_focus(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, false, false);
    cx.simulate_keystrokes("tab");
    open(&view, cx);
    cx.simulate_keystrokes("tab");
    assert!(focused(&view, cx, |view| &view.last));
    view.update(cx, |view, cx| {
        view.nested = true;
        cx.notify();
    });
    cx.run_until_parked();
    assert!(focused(&view, cx, |view| &view.nested_first));
    cx.simulate_keystrokes("tab");
    assert!(focused(&view, cx, |view| &view.nested_last));
    cx.simulate_keystrokes("tab");
    assert!(focused(&view, cx, |view| &view.nested_first));
    cx.simulate_keystrokes("escape");
    assert!(view.read_with(cx, |view, _| view.open && !view.nested));
    assert!(focused(&view, cx, |view| &view.last));
    cx.simulate_keystrokes("escape");
    assert!(focused(&view, cx, |view| &view.opener));
}

#[gpui::test]
fn select_inside_a_dialog_keeps_its_layer_and_tab_boundary(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, false, false);
    view.update(cx, |view, cx| {
        view.popup = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("tab");
    open(&view, cx);
    for key in ["tab", "tab", "enter", "down", "enter"] {
        cx.simulate_keystrokes(key);
        cx.simulate_event(gpui::KeyUpEvent {
            keystroke: gpui::Keystroke::parse(key).unwrap(),
        });
        cx.run_until_parked();
    }
    assert_eq!(
        view.read_with(cx, |view, _| view.selected.clone()),
        Some("b".into())
    );
    assert!(view.read_with(cx, |view, _| view.open));
    cx.simulate_keystrokes("tab");
    assert!(
        focused(&view, cx, |view| &view.first),
        "selection returns to the last trigger before wrapping"
    );
    cx.simulate_keystrokes("shift-tab enter tab");
    assert!(
        focused(&view, cx, |view| &view.first),
        "Tab closes the last dropdown and stays in the dialog"
    );
    cx.simulate_keystrokes("escape");
    assert!(focused(&view, cx, |view| &view.opener));
}
