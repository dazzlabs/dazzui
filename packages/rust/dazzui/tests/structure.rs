//! The structural components that change what is on screen, driven the way a
//! person drives them: a dialog and a drawer open from their triggers and
//! close on Escape, on the scrim and from their close controls; toasts arrive
//! along the bottom edge and leave on their own; the sidebar trigger toggles
//! its provider; the tree walks by keyboard.
//!
//! GPUI's test frame never forgets a `debug_selector` once it has been drawn,
//! so what is open is read off the host's state — every sheet here is
//! controlled — and the debug bounds only for where a sheet first appears.

use std::time::Duration;

use dazzui::{
    ActiveTheme, Button, Dialog, DialogBody, DialogClose, DialogHeader, Drawer, DrawerClose,
    Sidebar, SidebarCollapsible, SidebarInset, SidebarProvider, SidebarTrigger, Theme, Tint,
    ToastOptions, ToastProvider, ToastViewport, Toaster, Tree, TreeNode,
};
use gpui::{
    App, Context, Entity, InteractiveElement, IntoElement, Modifiers, ParentElement, Point, Render,
    SharedString, Styled, TestAppContext, VisualTestContext, Window, div, px, size,
};

struct Stage {
    toaster: Toaster,
    dialog: bool,
    drawer: bool,
    sidebar: bool,
    selected: Option<SharedString>,
}

/// A setter for one of the stage's flags, as an `on_open_change`.
fn setter(
    view: &gpui::WeakEntity<Stage>,
    field: fn(&mut Stage) -> &mut bool,
) -> impl Fn(bool, &mut Window, &mut App) + 'static {
    let view = view.clone();
    move |open, _, cx| {
        let _ = view.update(cx, |stage, cx| {
            *field(stage) = open;
            cx.notify();
        });
    }
}

impl Render for Stage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        let select = view.clone();
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(
                Dialog::new("dialog")
                    .open(self.dialog)
                    .on_open_change(setter(&view, |stage| &mut stage.dialog))
                    .trigger(
                        div()
                            .debug_selector(|| "dialog-trigger".into())
                            .child(Button::new("open-dialog").child("Rename…")),
                    )
                    .child(DialogHeader::new("Rename project"))
                    .child(DialogBody::new().child("Body"))
                    .child(DialogClose::new(
                        "dialog-close",
                        div()
                            .debug_selector(|| "dialog-close".into())
                            .child(Button::new("cancel").child("Cancel")),
                    )),
            )
            .child(
                Drawer::new("drawer")
                    .open(self.drawer)
                    .on_open_change(setter(&view, |stage| &mut stage.drawer))
                    .title("Filters")
                    .trigger(
                        div()
                            .debug_selector(|| "drawer-trigger".into())
                            .child(Button::new("open-drawer").child("Open filters")),
                    )
                    .footer(DrawerClose::new(
                        "drawer-close",
                        div()
                            .debug_selector(|| "drawer-close".into())
                            .child(Button::new("done").child("Done")),
                    ))
                    .child("Body"),
            )
            .child(ToastProvider::new(self.toaster.clone()).child(ToastViewport::new()))
            .child(
                div().h(px(120.)).w(px(600.)).flex().child(
                    SidebarProvider::new("provider")
                        .open(self.sidebar)
                        .on_open_change(setter(&view, |stage| &mut stage.sidebar))
                        .child(
                            Sidebar::new()
                                .collapsible(SidebarCollapsible::Offcanvas)
                                .child("Rows"),
                        )
                        .child(
                            SidebarInset::new().child(
                                div().flex().child(
                                    div()
                                        .debug_selector(|| "sidebar-trigger".into())
                                        .child(SidebarTrigger::new("trigger")),
                                ),
                            ),
                        ),
                ),
            )
            .child(
                Tree::new(
                    "tree",
                    [
                        TreeNode::new("src", "src").children([TreeNode::new("a", "a.rs")]),
                        TreeNode::new("b", "b.rs"),
                    ],
                )
                .selected(self.selected.clone())
                .on_selected_change(move |id, _, cx| {
                    let id = id.clone();
                    let _ = select.update(cx, |stage, cx| {
                        stage.selected = Some(id);
                        cx.notify();
                    });
                }),
            )
    }
}

fn stage(cx: &mut TestAppContext) -> (Entity<Stage>, &mut VisualTestContext) {
    cx.update(|cx: &mut App| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Stage {
        toaster: Toaster::new(cx),
        dialog: false,
        drawer: false,
        sidebar: true,
        selected: None,
    });
    cx.simulate_resize(size(px(1000.), px(800.)));
    cx.run_until_parked();
    (view, cx)
}

fn click(cx: &mut VisualTestContext, selector: &'static str) {
    let bounds = cx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} is on screen"));
    cx.simulate_click(bounds.center(), Modifiers::none());
    cx.run_until_parked();
}

fn read<T>(view: &Entity<Stage>, cx: &mut VisualTestContext, f: impl FnOnce(&Stage) -> T) -> T {
    view.read_with(cx, |stage, _| f(stage))
}

#[gpui::test]
fn dialog_opens_and_closes(cx: &mut TestAppContext) {
    let (view, cx) = stage(cx);
    assert!(cx.debug_bounds("dz-dialog").is_none());

    click(cx, "dialog-trigger");
    assert!(read(&view, cx, |s| s.dialog), "the trigger opens it");
    let sheet = cx.debug_bounds("dz-dialog").expect("the sheet is drawn");
    let width = cx.update(|_, cx| cx.theme().dialog_width);
    assert_eq!(sheet.size.width, px(width), "at dialog.width");
    assert_eq!(sheet.center(), Point::new(px(500.), px(400.)), "centred");

    // Escape reaches the sheet only if focus moved into it.
    cx.simulate_keystrokes("escape");
    assert!(!read(&view, cx, |s| s.dialog), "Escape closes it");

    click(cx, "dialog-trigger");
    assert!(read(&view, cx, |s| s.dialog));
    cx.simulate_click(Point::new(px(5.), px(790.)), Modifiers::none());
    cx.run_until_parked();
    assert!(!read(&view, cx, |s| s.dialog), "the scrim closes it");

    click(cx, "dialog-trigger");
    // A press on the sheet itself is not a press on the scrim.
    cx.simulate_click(sheet.center(), Modifiers::none());
    cx.run_until_parked();
    assert!(read(&view, cx, |s| s.dialog), "the sheet does not close it");
    click(cx, "dialog-close");
    assert!(!read(&view, cx, |s| s.dialog), "DialogClose closes it");
}

#[gpui::test]
fn drawer_opens_and_closes(cx: &mut TestAppContext) {
    let (view, cx) = stage(cx);
    click(cx, "drawer-trigger");
    assert!(read(&view, cx, |s| s.drawer), "the trigger opens it");
    let sheet = cx.debug_bounds("dz-drawer").expect("the sheet is drawn");
    let width = cx.update(|_, cx| cx.theme().drawer_size);
    assert_eq!(sheet.size.width, px(width), "at drawer.size");
    assert_eq!(sheet.origin.x + sheet.size.width, px(1000.), "flush right");
    assert_eq!(sheet.size.height, px(800.), "full height");

    click(cx, "drawer-close");
    assert!(!read(&view, cx, |s| s.drawer), "DrawerClose closes it");

    click(cx, "drawer-trigger");
    cx.simulate_keystrokes("escape");
    assert!(!read(&view, cx, |s| s.drawer), "Escape closes it");
}

#[gpui::test]
fn toasts_arrive_and_leave(cx: &mut TestAppContext) {
    let (view, cx) = stage(cx);
    let toaster = read(&view, cx, |s| s.toaster.clone());
    cx.update(|_, cx| {
        toaster.add(ToastOptions::new("Saved to Downloads"), cx);
    });
    cx.run_until_parked();
    let toast = cx.debug_bounds("dz-toast").expect("the toast is drawn");
    assert!(toast.origin.y > px(700.), "it hangs from the bottom edge");
    assert_eq!(toast.center().x, px(500.), "centred along it");

    cx.executor().advance_clock(Duration::from_millis(5100));
    cx.run_until_parked();
    assert!(
        cx.update(|_, cx| toaster.is_empty(cx)),
        "a notice goes after five seconds"
    );

    cx.update(|_, cx| {
        toaster.add(ToastOptions::new("Delete failed").tint(Tint::Danger), cx);
    });
    cx.executor().advance_clock(Duration::from_secs(30));
    cx.run_until_parked();
    assert_eq!(cx.update(|_, cx| toaster.len(cx)), 1, "a failure stays");
}

#[gpui::test]
fn sidebar_trigger_toggles_the_provider(cx: &mut TestAppContext) {
    let (view, cx) = stage(cx);
    click(cx, "sidebar-trigger");
    assert!(!read(&view, cx, |s| s.sidebar), "the trigger shuts it");
    click(cx, "sidebar-trigger");
    assert!(read(&view, cx, |s| s.sidebar), "and opens it again");
}

#[gpui::test]
fn tree_walks_by_keyboard(cx: &mut TestAppContext) {
    let (view, cx) = stage(cx);
    // A click on a shut branch selects it and opens it.
    click(cx, "dz-tree-row-src");
    assert_eq!(read(&view, cx, |s| s.selected.clone()), Some("src".into()));

    // ArrowRight on an open branch descends; Enter selects.
    cx.simulate_keystrokes("right enter");
    assert_eq!(read(&view, cx, |s| s.selected.clone()), Some("a".into()));

    // ArrowLeft climbs out of a leaf, again shuts the branch; Down then
    // reaches the next visible row.
    cx.simulate_keystrokes("left left down enter");
    assert_eq!(read(&view, cx, |s| s.selected.clone()), Some("b".into()));

    // Type-ahead over the visible rows, and Home.
    cx.simulate_keystrokes("s enter");
    assert_eq!(read(&view, cx, |s| s.selected.clone()), Some("src".into()));
}
