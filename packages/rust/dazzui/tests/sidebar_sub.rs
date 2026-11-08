//! Nested rows keep native activation and release their focus stops on collapse.
#[allow(dead_code)]
mod support;
use dazzui::{
    ActiveTheme, Sidebar, SidebarCollapsible, SidebarMenu, SidebarMenuBadge, SidebarMenuButton,
    SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubButtonSize,
    SidebarMenuSubItem, SidebarProvider, Theme, Tint, Variant,
};
use gpui::{
    Context, Entity, FocusHandle, InteractiveElement, IntoElement, KeyUpEvent, Keystroke,
    ObservedPaint, ParentElement, Render, Styled, TestAppContext, Window, canvas, div, px,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use support::paint::PaintProbe;

struct Harness {
    open: bool,
    clicks: Vec<&'static str>,
    next: FocusHandle,
    badge: Rc<RefCell<Vec<ObservedPaint>>>,
    nested: Rc<Cell<bool>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.nested.set(false);
        self.badge.borrow_mut().clear();
        let small = cx.entity().downgrade();
        let medium = small.clone();
        let disabled = small.clone();
        let nested = self.nested.clone();
        div().size_full().flex().child(
            SidebarProvider::new("provider")
                .open(self.open)
                .child(
                    Sidebar::new()
                        .width(220.)
                        .collapsible(SidebarCollapsible::Icon)
                        .child(
                            SidebarMenu::new().child(
                                SidebarMenuItem::new()
                                    .child(
                                        SidebarMenuButton::new("parent")
                                            .active(true)
                                            .child("Parent"),
                                    )
                                    .child(
                                        SidebarMenuSub::new()
                                            .child(
                                                SidebarMenuSubItem::new().child(
                                                    SidebarMenuSubButton::new("small")
                                                        .variant(Variant::Tinted)
                                                        .tint(Tint::Success)
                                                        .active(true)
                                                        .on_click(move |_, _, cx| {
                                                            let _ = small.update(cx, |view, _| {
                                                                view.clicks.push("small")
                                                            });
                                                        })
                                                        .child(
                                                            div().child("Small").child(
                                                                canvas(
                                                                    |_, _, _| {},
                                                                    move |_, _, _, _| {
                                                                        nested.set(true)
                                                                    },
                                                                )
                                                                .absolute()
                                                                .inset_0(),
                                                            ),
                                                        ),
                                                ),
                                            )
                                            .child(
                                                SidebarMenuSubItem::new().child(
                                                    SidebarMenuSubButton::new("medium")
                                                        .size(SidebarMenuSubButtonSize::Medium)
                                                        .on_click(move |_, _, cx| {
                                                            let _ = medium.update(cx, |view, _| {
                                                                view.clicks.push("medium")
                                                            });
                                                        })
                                                        .child("Medium"),
                                                ),
                                            )
                                            .child(
                                                SidebarMenuSubItem::new().child(
                                                    SidebarMenuSubButton::new("disabled")
                                                        .disabled(true)
                                                        .on_click(move |_, _, cx| {
                                                            let _ =
                                                                disabled.update(cx, |view, _| {
                                                                    view.clicks.push("disabled")
                                                                });
                                                        })
                                                        .child("Disabled"),
                                                ),
                                            ),
                                    )
                                    .badge(PaintProbe {
                                        child: SidebarMenuBadge::new()
                                            .child("3")
                                            .into_any_element(),
                                        paints: self.badge.clone(),
                                    }),
                            ),
                        ),
                )
                .child(
                    div()
                        .id("next")
                        .track_focus(&self.next)
                        .w(px(60.))
                        .child("Next"),
                ),
        )
    }
}
fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, &mut gpui::VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("bright-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        open: true,
        clicks: vec![],
        next: cx.focus_handle().tab_stop(true),
        badge: Rc::default(),
        nested: Rc::default(),
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    (view, cx)
}
#[gpui::test]
fn sub_rows_activate_on_native_key_phases_and_skip_disabled_rows(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    cx.simulate_keystrokes("tab tab enter");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.clicks.clone()), ["small"]);
    cx.simulate_keystrokes("tab space");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.clicks.clone()), ["small"]);
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse("space").unwrap(),
    });
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.clicks.clone()),
        ["small", "medium"]
    );
    cx.simulate_keystrokes("tab");
    cx.run_until_parked();
    let next = view.read_with(cx, |view, _| view.next.clone());
    assert!(cx.update(|window, _| next.is_focused(window)));
    view.update(cx, |view, cx| {
        view.open = false;
        cx.notify();
    });
    cx.update(|window, _| window.blur());
    cx.run_until_parked();
    assert!(!view.read_with(cx, |view, _| view.nested.get()));
    cx.simulate_keystrokes("tab tab");
    cx.run_until_parked();
    assert!(
        cx.update(|window, _| next.is_focused(window)),
        "collapsed descendants leave the tab order"
    );
}
#[gpui::test]
fn nested_selection_does_not_recolour_a_parent_badge(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    for theme in [
        "bright-light",
        "bright-dark",
        "studio-light",
        "studio-dark",
        "frost-light",
        "frost-dark",
        "graphite-light",
        "graphite-dark",
        "ember-light",
        "ember-dark",
        "nocturne-light",
        "nocturne-dark",
    ] {
        cx.update(|_, cx| Theme::set(theme, cx));
        cx.run_until_parked();
        let paint = view.read_with(cx, |view, _| view.badge.borrow().last().unwrap().clone());
        let expected = cx.update(|_, cx| {
            cx.theme()
                .control_color_filled_content_normal
                .resolve(Tint::Primary.ramp(cx.theme()))
        });
        assert_eq!(paint.text.unwrap().color, expected, "{theme}");
        assert!(view.read_with(cx, |view, _| view.nested.get()));
    }
}
