//! Sidebar actions retain the icon button's native input and original box.
#[allow(dead_code)]
mod support;
use dazzui::{
    ActiveTheme, Motion, MotionPreference, Sidebar, SidebarCollapsible, SidebarGroup,
    SidebarGroupAction, SidebarGroupLabel, SidebarMenu, SidebarMenuAction, SidebarMenuButton,
    SidebarMenuItem, SidebarProvider, Theme,
};
use gpui::{
    Context, Entity, FocusHandle, InteractiveElement, IntoElement, KeyUpEvent, Keystroke,
    Modifiers, ObservedPaint, ParentElement, Render, Styled, TestAppContext, VisualTestContext,
    Window, div, point, px,
};
use std::{cell::RefCell, rc::Rc, time::Duration};
use support::paint::PaintProbe;
struct Harness {
    open: bool,
    disabled: bool,
    popup: bool,
    show: bool,
    actions: usize,
    groups: usize,
    next: FocusHandle,
    row: Rc<RefCell<Vec<ObservedPaint>>>,
    action: Rc<RefCell<Vec<ObservedPaint>>>,
    group: Rc<RefCell<Vec<ObservedPaint>>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.row.borrow_mut().clear();
        self.action.borrow_mut().clear();
        self.group.borrow_mut().clear();
        let action = cx.entity().downgrade();
        let group = action.clone();
        let probe = |child, paints: Rc<RefCell<Vec<ObservedPaint>>>| PaintProbe { child, paints };
        div().size_full().p(px(24.)).flex().child(
            SidebarProvider::new("provider")
                .open(self.open)
                .child(
                    Sidebar::new()
                        .width(220.)
                        .collapsible(SidebarCollapsible::Icon)
                        .child(
                            SidebarGroup::new()
                                .child(SidebarGroupLabel::new().child("Library"))
                                .child(probe(
                                    SidebarGroupAction::new("add", "Add")
                                        .child("+")
                                        .on_click(move |_, _, cx| {
                                            let _ = group.update(cx, |view, _| view.groups += 1);
                                        })
                                        .into_any_element(),
                                    self.group.clone(),
                                ))
                                .child(
                                    SidebarMenu::new()
                                        .child(
                                            SidebarMenuItem::new()
                                                .id("first-item")
                                                .child(probe(
                                                    SidebarMenuButton::new("row")
                                                        .child("Documents")
                                                        .into_any_element(),
                                                    self.row.clone(),
                                                ))
                                                .action(probe(
                                                    SidebarMenuAction::new("action", "More")
                                                        .show_on_hover(self.show)
                                                        .popup_open(self.popup)
                                                        .disabled(self.disabled)
                                                        .child("X")
                                                        .on_click(move |_, _, cx| {
                                                            let _ = action.update(cx, |view, _| {
                                                                view.actions += 1
                                                            });
                                                        })
                                                        .into_any_element(),
                                                    self.action.clone(),
                                                )),
                                        )
                                        .child(
                                            SidebarMenuItem::new().child(
                                                SidebarMenuButton::new("other").child("Other"),
                                            ),
                                        ),
                                ),
                        ),
                )
                .child(div().id("next").track_focus(&self.next).child("Next")),
        )
    }
}
fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("bright-light", cx);
        Motion::freeze_at(Duration::ZERO, cx);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        open: true,
        disabled: false,
        popup: false,
        show: true,
        actions: 0,
        groups: 0,
        next: cx.focus_handle().tab_stop(true),
        row: Rc::default(),
        action: Rc::default(),
        group: Rc::default(),
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    (view, cx)
}
fn action(view: &Entity<Harness>, cx: &VisualTestContext) -> Option<gpui::ObservedSurface> {
    view.read_with(cx, |view, _| {
        view.action
            .borrow()
            .last()
            .unwrap()
            .surface
            .clone()
            .filter(|surface| {
                surface.bounds.size.width > px(0.) && surface.bounds.size.height > px(0.)
            })
    })
}
fn opacity(view: &Entity<Harness>, cx: &VisualTestContext) -> f32 {
    action(view, cx).unwrap().style.opacity.unwrap_or(1.)
}
fn leave(cx: &mut VisualTestContext) {
    cx.simulate_mouse_move(point(px(0.), px(0.)), None, Modifiers::none());
    cx.run_until_parked();
}
#[gpui::test]
fn actions_keep_position_label_clearance_and_keyboard_activation(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    assert_eq!(opacity(&view, cx), 0.);
    let row = view.read_with(cx, |view, _| {
        view.row.borrow().last().unwrap().surface.clone().unwrap()
    });
    let button = action(&view, cx).unwrap();
    let (right, padding) = cx.update(|_, cx| (cx.theme().spacing_1, cx.theme().spacing_8));
    assert_eq!(button.bounds.right(), row.bounds.right() - px(right));
    assert_eq!(button.bounds.center().y, row.bounds.center().y);
    assert_eq!(row.style.padding.right, px(padding).into());
    cx.simulate_keystrokes("tab enter");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.groups), 1);
    cx.simulate_keystrokes("tab");
    cx.run_until_parked();
    assert_eq!(opacity(&view, cx), 1., "row focus reveals its action");
    cx.simulate_keystrokes("tab enter");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.actions), 1);
    cx.simulate_keystrokes("space");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.actions), 1);
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse("space").unwrap(),
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.actions), 2);
    cx.simulate_keystrokes("tab");
    cx.run_until_parked();
    assert_eq!(
        opacity(&view, cx),
        0.,
        "another row does not reveal this action"
    );
}
#[gpui::test]
fn hover_popup_and_collapse_apply_independently(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    let row = view.read_with(cx, |view, _| {
        view.row
            .borrow()
            .last()
            .unwrap()
            .surface
            .clone()
            .unwrap()
            .bounds
    });
    cx.simulate_mouse_move(row.center(), None, Modifiers::none());
    cx.run_until_parked();
    assert_eq!(opacity(&view, cx), 1.);
    leave(cx);
    assert_eq!(opacity(&view, cx), 0.);
    view.update(cx, |view, cx| {
        view.popup = true;
        cx.notify();
    });
    cx.run_until_parked();
    assert_eq!(opacity(&view, cx), 1.);
    view.update(cx, |view, cx| {
        view.open = false;
        cx.notify();
    });
    cx.run_until_parked();
    assert!(action(&view, cx).is_none());
    assert!(view.read_with(cx, |view, _| {
        view.group
            .borrow()
            .last()
            .unwrap()
            .decorated_surface
            .is_none()
    }));
    cx.simulate_keystrokes("tab tab tab");
    cx.run_until_parked();
    let next = view.read_with(cx, |view, _| view.next.clone());
    assert!(
        cx.update(|window, _| next.is_focused(window)),
        "collapsed actions leave Tab traversal"
    );
}
#[gpui::test]
fn action_reveal_animates_and_reduced_motion_settles_it(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    cx.update(|_, cx| {
        Motion::set_preference(MotionPreference::Full, cx);
        Motion::seek(Duration::ZERO, cx);
    });
    cx.run_until_parked();
    let row = view.read_with(cx, |view, _| {
        view.row
            .borrow()
            .last()
            .unwrap()
            .surface
            .clone()
            .unwrap()
            .bounds
    });
    cx.simulate_mouse_move(row.center(), None, Modifiers::none());
    cx.run_until_parked();
    let duration = cx.update(|_, cx| cx.theme().motion_duration);
    cx.update(|_, cx| Motion::seek(Duration::from_secs_f64(f64::from(duration) / 2000.), cx));
    cx.run_until_parked();
    let halfway = opacity(&view, cx);
    assert!(halfway > 0. && halfway < 1., "{halfway}");
    cx.update(|_, cx| Motion::set_preference(MotionPreference::Reduced, cx));
    cx.run_until_parked();
    leave(cx);
    assert_eq!(opacity(&view, cx), 0.);
}
