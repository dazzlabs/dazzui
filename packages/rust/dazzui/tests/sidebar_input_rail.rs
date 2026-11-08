//! Sidebar fields use the editor, rails use native buttons, and shortcuts
//! follow window bubbling rather than requiring focus inside the provider.
#[allow(dead_code)]
mod support;
use dazzui::{
    ActiveTheme, Motion, Sidebar, SidebarCollapsible, SidebarHeader, SidebarInput, SidebarMenu,
    SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail, SidebarSide, Theme,
};
use gpui::{
    Context, Entity, FocusHandle, InteractiveElement, IntoElement, KeyUpEvent, Keystroke,
    Modifiers, MouseButton, ObservedPaint, ParentElement, Render, Styled, TestAppContext,
    VisualTestContext, Window, div, point, prelude::*, px,
};
use std::{cell::RefCell, rc::Rc, time::Duration};
use support::paint::PaintProbe;
gpui::actions!(sidebar_shortcut_tests, [OverrideSidebarKey]);
struct Harness {
    open: bool,
    mounted: bool,
    accept: bool,
    block: bool,
    side: SidebarSide,
    requests: Vec<bool>,
    text: String,
    submitted: String,
    blurred: usize,
    outside: FocusHandle,
    rail: Rc<RefCell<Vec<ObservedPaint>>>,
    field: Rc<RefCell<Vec<ObservedPaint>>>,
    column: Rc<RefCell<Vec<ObservedPaint>>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        for paints in [&self.rail, &self.field, &self.column] {
            paints.borrow_mut().clear();
        }
        let owner = cx.entity().downgrade();
        let change = owner.clone();
        let submit = owner.clone();
        let blur = owner.clone();
        let block = self.block;
        div()
            .size_full()
            .on_action(|_: &OverrideSidebarKey, _, _| {})
            .p(px(24.))
            .flex()
            .when(self.mounted, |root| {
                root.child(
                    SidebarProvider::new("provider")
                        .open(self.open)
                        .on_open_change(move |open, _, cx| {
                            let _ = owner.update(cx, |view, cx| {
                                view.requests.push(open);
                                if view.accept {
                                    view.open = open;
                                }
                                cx.notify();
                            });
                        })
                        .child(PaintProbe {
                            paints: self.column.clone(),
                            child: Sidebar::new()
                                .width(220.)
                                .side(self.side)
                                .collapsible(SidebarCollapsible::Icon)
                                .child(
                                    SidebarHeader::new().child(PaintProbe {
                                        paints: self.field.clone(),
                                        child: SidebarInput::new("filter")
                                            .value(self.text.clone())
                                            .placeholder("Filter")
                                            .on_change(move |text, _, cx| {
                                                let _ = change.update(cx, |view, cx| {
                                                    view.text = text.into();
                                                    cx.notify();
                                                });
                                            })
                                            .on_submit(move |text, _, cx| {
                                                let _ = submit.update(cx, |view, _| {
                                                    view.submitted = text.into()
                                                });
                                            })
                                            .on_blur(move |_, cx| {
                                                let _ =
                                                    blur.update(cx, |view, _| view.blurred += 1);
                                            })
                                            .into_any_element(),
                                    }),
                                )
                                .child(
                                    SidebarMenu::new().child(
                                        SidebarMenuItem::new().child(
                                            SidebarMenuButton::new("row").child("Documents"),
                                        ),
                                    ),
                                )
                                .child(PaintProbe {
                                    paints: self.rail.clone(),
                                    child: SidebarRail::new("rail").into_any_element(),
                                })
                                .into_any_element(),
                        }),
                )
            })
            .child(
                div()
                    .id("outside")
                    .track_focus(&self.outside)
                    .on_key_down(move |_, _, cx| {
                        if block {
                            cx.stop_propagation();
                        }
                    })
                    .child("Outside"),
            )
    }
}
fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
        Motion::freeze_at(Duration::ZERO, cx);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        open: true,
        mounted: true,
        accept: true,
        block: false,
        side: SidebarSide::Left,
        requests: vec![],
        text: "seed".into(),
        submitted: String::new(),
        blurred: 0,
        outside: cx.focus_handle().tab_stop(true),
        rail: Rc::default(),
        field: Rc::default(),
        column: Rc::default(),
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    (view, cx)
}
fn click(cx: &mut VisualTestContext, at: gpui::Point<gpui::Pixels>) {
    cx.simulate_mouse_down(at, MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_up(at, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
}
fn bounds(
    view: &Entity<Harness>,
    cx: &VisualTestContext,
    part: &str,
) -> gpui::Bounds<gpui::Pixels> {
    view.read_with(cx, |view, _| {
        let paints = match part {
            "rail" => &view.rail,
            "field" => &view.field,
            _ => &view.column,
        };
        paints
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
fn sidebar_input_keeps_small_default_and_editor_callbacks(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    assert_eq!(bounds(&view, cx, "field").size.height, px(26.));
    click(cx, bounds(&view, cx, "field").center());
    cx.simulate_keystrokes("cmd-a");
    cx.simulate_input("héllo");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.text.clone()), "héllo");
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.submitted.clone()),
        "héllo"
    );
    let outside = view.read_with(cx, |view, _| view.outside.clone());
    cx.update(|window, _| window.focus(&outside));
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.blurred), 1);
}
#[gpui::test]
fn window_shortcuts_work_outside_the_provider_respect_bubbling_and_unmount(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    let outside = view.read_with(cx, |view, _| view.outside.clone());
    cx.update(|window, _| window.focus(&outside));
    cx.run_until_parked();
    cx.simulate_keystrokes("cmd-b");
    cx.run_until_parked();
    assert!(!view.read_with(cx, |view, _| view.open));
    cx.simulate_keystrokes("ctrl-b");
    cx.run_until_parked();
    assert!(view.read_with(cx, |view, _| view.open));
    cx.simulate_keystrokes("cmd-shift-b");
    cx.run_until_parked();
    assert!(view.read_with(cx, |view, _| view.open));
    view.update(cx, |view, cx| {
        view.accept = false;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("cmd-b cmd-b");
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.requests.clone()),
        [false, true, false, false]
    );
    view.update(cx, |view, cx| {
        view.block = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("cmd-b");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.requests.len()), 4);
    view.update(cx, |view, cx| {
        view.mounted = false;
        view.block = false;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("cmd-b");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.requests.len()), 4);
}
#[gpui::test]
fn rails_straddle_either_edge_toggle_by_pointer_and_stay_out_of_tab_order(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    for side in [SidebarSide::Left, SidebarSide::Right] {
        view.update(cx, |view, cx| {
            view.side = side;
            view.open = true;
            cx.notify();
        });
        cx.run_until_parked();
        let rail = bounds(&view, cx, "rail");
        let column = bounds(&view, cx, "column");
        let width = cx.update(|_, cx| cx.theme().spacing_4);
        assert_eq!(rail.size.width, px(width));
        assert_eq!(rail.size.height, column.size.height);
        let edge = if side == SidebarSide::Left {
            column.right()
        } else {
            column.left()
        };
        assert!((f32::from(rail.center().x - edge)).abs() <= 1.);
        cx.update(|window, _| window.blur());
        cx.run_until_parked();
        cx.simulate_keystrokes("tab tab tab");
        cx.run_until_parked();
        let outside = view.read_with(cx, |view, _| view.outside.clone());
        assert!(cx.update(|window, _| outside.is_focused(window)));
        let inset = if side == SidebarSide::Left { -2. } else { 2. };
        click(cx, rail.center() + point(px(inset), px(0.)));
        assert!(!view.read_with(cx, |view, _| view.open));
        cx.simulate_keystrokes("space");
        cx.simulate_event(KeyUpEvent {
            keystroke: Keystroke::parse("space").unwrap(),
        });
        cx.run_until_parked();
        assert!(view.read_with(cx, |view, _| view.open));
    }
}

#[gpui::test]
fn a_consumed_application_binding_prevents_the_sidebar_shortcut(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    cx.update(|_, cx| cx.bind_keys([gpui::KeyBinding::new("cmd-b", OverrideSidebarKey, None)]));
    let outside = view.read_with(cx, |view, _| view.outside.clone());
    cx.update(|window, _| window.focus(&outside));
    cx.run_until_parked();
    cx.simulate_keystrokes("cmd-b");
    cx.run_until_parked();
    assert!(view.read_with(cx, |view, _| view.requests.is_empty()));
}
