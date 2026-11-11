//! Native dropdowns, driven over a fake backend: the preference scopes, the
//! OS gets the flattened list instead of the sheet, a dismissal runs the
//! choice first and closes second, and everything unsupported falls back to
//! the sheet.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use dazzui::{
    ContextMenu, Menu, MenuItem, MenuPresentation, Menubar, MenubarMenu, NativeMenuBackend,
    NativeMenuEntry, NativeMenuEvent, NativeMenuRequest, Select, SelectEntry, SelectOption,
    provide_menu_presentation, set_debug_native_menu_backend,
};
use gpui::{
    AnyElement, App, Bounds, Context, InteractiveElement, IntoElement, Modifiers, ParentElement,
    Pixels, Point, Render, StatefulInteractiveElement, Styled, TestAppContext, VisualTestContext,
    Window, canvas, div, px,
};

type Log = Rc<RefCell<Vec<String>>>;
type Painted = Rc<RefCell<HashSet<&'static str>>>;

/// The backend the tests install: it records what it is asked to present and
/// hands the clicks and dismissals back through the queue it was given, the
/// way an OS popup would.
#[derive(Default)]
struct Fake {
    inner: RefCell<FakeInner>,
}

#[derive(Default)]
struct FakeInner {
    supported: bool,
    presents: bool,
    requests: Vec<Captured>,
    closes: usize,
    /// The queue and rows of the menu that is out, for poking it.
    live: Option<(dazzui::EventQueue, Vec<NativeMenuEntry>)>,
    pointer: Option<Point<Pixels>>,
}

#[derive(Clone)]
struct Captured {
    rows: Vec<(String, bool, Option<bool>)>,
    /// Each row's glyph, in the same walk.
    icons: Vec<Option<String>>,
    anchor: Bounds<Pixels>,
    align_end: bool,
}

impl Fake {
    fn new() -> Rc<Self> {
        let fake = Rc::new(Fake::default());
        set_debug_native_menu_backend(Some(fake.clone()));
        fake
    }

    fn release(&self) {
        set_debug_native_menu_backend(None);
    }

    fn set_supported(&self, supported: bool) {
        self.inner.borrow_mut().supported = supported;
    }

    fn set_present_result(&self, presents: bool) {
        self.inner.borrow_mut().presents = presents;
    }

    fn set_pointer(&self, pointer: Option<Point<Pixels>>) {
        self.inner.borrow_mut().pointer = pointer;
    }

    fn request_count(&self) -> usize {
        self.inner.borrow().requests.len()
    }

    fn request(&self, at: usize) -> Captured {
        self.inner.borrow().requests[at].clone()
    }

    fn close_count(&self) -> usize {
        self.inner.borrow().closes
    }

    /// Chooses a row of the menu that is out.
    fn click(&self, index: usize) {
        if let Some((queue, _)) = &self.inner.borrow().live {
            queue.push(NativeMenuEvent::Clicked(index));
        }
    }

    /// Reports the menu closed — before the click, the order AppKit and
    /// TrackPopupMenu deliver.
    fn dismiss_before(&self, index: usize) {
        if let Some((queue, _)) = &self.inner.borrow().live {
            queue.push(NativeMenuEvent::Dismissed);
            queue.push(NativeMenuEvent::Clicked(index));
        }
    }
}

impl NativeMenuBackend for Fake {
    fn supported(&self) -> bool {
        self.inner.borrow().supported
    }

    fn present(&self, request: &NativeMenuRequest, events: &dazzui::EventQueue) -> bool {
        let mut inner = self.inner.borrow_mut();
        // The rows as the OS wears them: the depth-first walk the click
        // dispatch reads, a submenu's rows following their branch.
        let mut rows = Vec::new();
        let mut icons = Vec::new();
        for entry in &request.entries {
            entry.walk(&mut |row| {
                rows.push((row.label.clone(), row.enabled, row.checked));
                icons.push(row.icon.as_ref().map(|icon| icon.to_string()));
            });
        }
        inner.requests.push(Captured {
            rows,
            icons,
            anchor: request.anchor,
            align_end: request.align_end,
        });
        if !inner.presents {
            return false;
        }
        inner.live = Some((events.clone(), request.entries.clone()));
        true
    }

    fn close(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.closes += 1;
        inner.live = None;
    }

    fn pointer_position(&self, _: &NativeMenuRequest) -> Option<Point<Pixels>> {
        self.inner.borrow().pointer
    }
}

/// What a test stages: one dropdown of a kind, wrapped in the presentation a
/// field flips.
struct Stage {
    kind: Kind,
    native: bool,
    log: Log,
    painted: Painted,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Menu,
    Nested,
    Menubar,
    Select,
    Context,
}

impl Render for Stage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.painted.borrow_mut().clear();
        let log = self.log.clone();
        let push = move |entry: &str| {
            let log = log.clone();
            let entry = entry.to_string();
            move |_: &mut Window, _: &mut App| log.borrow_mut().push(entry.clone())
        };
        let painted = &self.painted;
        let marker = |selector: &'static str| {
            let painted = painted.clone();
            div()
                .debug_selector(move || selector.into())
                .size(px(8.))
                .child(
                    canvas(
                        |_, _, _| {},
                        move |_, _, _, _| {
                            painted.borrow_mut().insert(selector);
                        },
                    )
                    .size_full(),
                )
        };
        let presentation = if self.native {
            MenuPresentation::Native
        } else {
            MenuPresentation::Custom
        };
        let content = match self.kind {
            Kind::Menu => div()
                .child(
                    Menu::new("menu")
                        .trigger(|press, _, _, _| {
                            div()
                                .id("menu-trigger")
                                .debug_selector(|| "menu-trigger".into())
                                .on_click(move |_, window, cx| press(window, cx))
                                .child("Open")
                                .into_any_element()
                        })
                        .items([
                            MenuItem::new("Rename")
                                .icon(marker("menu-row"))
                                .glyph("glyph/Edit20Regular")
                                .shortcut_hint("cmd R")
                                .on_select(push("rename")),
                            MenuItem::new("Delete")
                                .disabled(true)
                                .shortcut_hint("backspace")
                                .on_select(push("delete")),
                            MenuItem::new("Duplicate")
                                .checked(false)
                                .on_select(push("duplicate")),
                        ])
                        .on_open_change({
                            let log = self.log.clone();
                            move |open, _, _| log.borrow_mut().push(format!("open:{open}"))
                        }),
                )
                .into_any_element(),
            Kind::Nested => div()
                .child(
                    Menu::new("nested")
                        .trigger(|press, _, _, _| {
                            div()
                                .id("menu-trigger")
                                .debug_selector(|| "menu-trigger".into())
                                .on_click(move |_, window, cx| press(window, cx))
                                .child("Open")
                                .into_any_element()
                        })
                        .items([
                            MenuItem::new("Rename").on_select(push("rename")),
                            MenuItem::new("Move to").submenu([
                                MenuItem::new("Projects").submenu([
                                    MenuItem::new("DazzUI").on_select(push("dazzui")),
                                    MenuItem::new("Shell").disabled(true),
                                ]),
                                MenuItem::new("Archive").on_select(push("archive")),
                            ]),
                            MenuItem::new("Delete")
                                .disabled(true)
                                .on_select(push("delete")),
                        ])
                        .on_open_change({
                            let log = self.log.clone();
                            move |open, _, _| log.borrow_mut().push(format!("open:{open}"))
                        }),
                )
                .into_any_element(),
            Kind::Menubar => div()
                .child(Menubar::new("bar").menus([
                    MenubarMenu::new("File").items([
                        MenuItem::new("New").on_select(push("new")),
                        MenuItem::new("Close").on_select(push("close")),
                    ]),
                    MenubarMenu::new("View").items([
                        MenuItem::new("Zoom In").on_select(push("zoom-in")),
                        MenuItem::new("Zoom Out").disabled(true),
                    ]),
                ]))
                .into_any_element(),
            Kind::Select => div()
                .w(px(200.))
                .debug_selector(|| "select".into())
                .child(
                    Select::new("select")
                        .options([
                            SelectEntry::Option(SelectOption::new("en", "English")),
                            SelectEntry::group(
                                "More",
                                vec![
                                    SelectOption::new("zh", "Chinese"),
                                    SelectOption::new("ja", "Japanese"),
                                ],
                            ),
                        ])
                        .default_value("en")
                        .on_value_change({
                            let log = self.log.clone();
                            move |value: gpui::SharedString, _, _| {
                                log.borrow_mut().push(format!("value:{value}"))
                            }
                        }),
                )
                .into_any_element(),
            Kind::Context => ContextMenu::new("context")
                .items([MenuItem::new("Copy").on_select(push("copy"))])
                .child(
                    div()
                        .debug_selector(|| "context-region".into())
                        .size(px(120.))
                        .child("Region"),
                )
                .into_any_element(),
        };
        provide_menu_presentation(presentation, content)
    }
}

struct Setup<'a> {
    fake: Rc<Fake>,
    log: Log,
    painted: Painted,
    view: gpui::Entity<Stage>,
    cx: &'a mut VisualTestContext,
}

fn setup(kind: Kind, cx: &mut TestAppContext) -> Setup<'_> {
    let fake = Fake::new();
    fake.set_supported(true);
    fake.set_present_result(true);
    cx.update(dazzui::init);
    let log: Log = Rc::default();
    let painted: Painted = Rc::default();
    let (view, cx) = cx.add_window_view(|_, _| Stage {
        kind,
        native: true,
        log: log.clone(),
        painted: painted.clone(),
    });
    cx.run_until_parked();
    Setup {
        fake,
        log,
        painted,
        view,
        cx,
    }
}

impl Drop for Setup<'_> {
    fn drop(&mut self) {
        self.fake.release();
    }
}

/// Whether the sheet drew in a fresh frame — its rows' icon slots are
/// markers, and native rows carry no icons.
fn sheet_drawn(setup: &mut Setup, selector: &'static str) -> bool {
    setup.cx.update(|window, _| window.refresh());
    setup.cx.run_until_parked();
    setup.painted.borrow().contains(selector)
}

/// Lets the drain task run: the test clock only moves when a test says so,
/// and the drain looks at the queue on a timer.
fn drain(setup: &mut Setup) {
    setup
        .cx
        .executor()
        .advance_clock(std::time::Duration::from_millis(500));
    setup.cx.run_until_parked();
}

fn click(setup: &mut Setup, selector: &'static str) {
    let bounds = setup.cx.debug_bounds(selector).expect(selector);
    setup.cx.simulate_click(bounds.center(), Modifiers::none());
    drain(setup);
}

fn flip_native(setup: &mut Setup, native: bool) {
    setup.view.update(setup.cx, |stage, cx| {
        stage.native = native;
        cx.notify();
    });
    setup.cx.run_until_parked();
}

#[gpui::test]
fn a_native_menu_flattens_and_runs_the_choice_before_the_close(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Menu, cx);
    assert!(
        !sheet_drawn(&mut setup, "menu-row"),
        "no sheet while the OS owns the list"
    );
    click(&mut setup, "menu-trigger");
    setup.cx.run_until_parked();

    assert_eq!(
        setup.fake.request_count(),
        1,
        "one present for one open, sheet still undrawn"
    );
    assert!(!sheet_drawn(&mut setup, "menu-row"));
    let captured = setup.fake.request(0);
    assert_eq!(
        captured.rows,
        vec![
            ("Rename".to_string(), true, None),
            ("Delete".to_string(), false, None),
            ("Duplicate".to_string(), true, Some(false)),
        ],
        "labels, enabled and check state in order; icons and hints stay behind"
    );
    assert_eq!(
        captured.icons,
        vec![Some("glyph/Edit20Regular".to_string()), None, None],
        "the glyph goes to the OS; the drawn icon element does not"
    );
    assert!(captured.align_end, "Menu's default Align::End asks for it");
    assert_eq!(
        *setup.log.borrow(),
        ["open:true"],
        "the trigger reported it"
    );
    assert_eq!(setup.fake.close_count(), 0, "opening never closes");

    // The OS reports the close before the chosen item's action.
    setup.fake.dismiss_before(0);
    drain(&mut setup);
    assert_eq!(
        *setup.log.borrow(),
        ["open:true", "rename", "open:false"],
        "the choice runs, then the close is heard"
    );
    assert_eq!(
        setup.fake.close_count(),
        1,
        "the session is torn down once the OS has had its say"
    );
}

#[gpui::test]
fn align_end_puts_the_right_edge_at_the_trigger(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Menu, cx);
    click(&mut setup, "menu-trigger");
    setup.cx.run_until_parked();
    let captured = setup.fake.request(0);
    assert!(
        captured.align_end,
        "Menu's default Align::End asks for the right edge"
    );
    assert_eq!(
        captured.anchor,
        setup.cx.debug_bounds("menu-trigger").unwrap(),
        "the anchor is the trigger as the frame painted it"
    );
}

#[gpui::test]
fn a_native_menu_walks_its_submenus_and_dispatches_their_rows(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Nested, cx);
    click(&mut setup, "menu-trigger");
    setup.cx.run_until_parked();

    assert_eq!(setup.fake.request_count(), 1);
    assert_eq!(
        setup.fake.request(0).rows,
        vec![
            ("Rename".to_string(), true, None),
            ("Move to".to_string(), true, None),
            ("Projects".to_string(), true, None),
            ("DazzUI".to_string(), true, None),
            ("Shell".to_string(), false, None),
            ("Archive".to_string(), true, None),
            ("Delete".to_string(), false, None),
        ],
        "depth-first: a submenu's rows follow their branch"
    );

    // The OS reports a nested row's choice by its depth-first slot; the
    // session closes with everything else.
    setup.fake.dismiss_before(3);
    drain(&mut setup);
    assert_eq!(
        *setup.log.borrow(),
        ["open:true", "dazzui", "open:false"],
        "the choice three levels down runs, then the close is heard"
    );
}

#[gpui::test]
fn a_scope_overrides_its_root_and_the_default_is_custom(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Menu, cx);
    flip_native(&mut setup, false);
    click(&mut setup, "menu-trigger");
    setup.cx.run_until_parked();
    assert_eq!(
        setup.fake.request_count(),
        0,
        "custom draws the sheet; the backend never hears of it"
    );
    assert!(sheet_drawn(&mut setup, "menu-row"));
    assert_eq!(*setup.log.borrow(), ["open:true"]);
}

#[gpui::test]
fn an_unsupported_backend_falls_back_to_the_sheet(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Menu, cx);
    setup.fake.set_supported(false);
    click(&mut setup, "menu-trigger");
    setup.cx.run_until_parked();
    assert_eq!(setup.fake.request_count(), 0);
    assert!(
        sheet_drawn(&mut setup, "menu-row"),
        "the sheet is the answer"
    );
}

#[gpui::test]
fn a_refused_present_falls_back_to_the_sheet(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Menu, cx);
    setup.fake.set_present_result(false);
    click(&mut setup, "menu-trigger");
    setup.cx.run_until_parked();
    assert_eq!(
        setup.fake.request_count(),
        1,
        "the OS was asked and said no"
    );
    assert!(sheet_drawn(&mut setup, "menu-row"), "so the sheet draws");
}

#[gpui::test]
fn changing_the_presentation_dismisses_an_open_menu(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Menu, cx);
    click(&mut setup, "menu-trigger");
    setup.cx.run_until_parked();
    assert_eq!(setup.fake.request_count(), 1);
    flip_native(&mut setup, false);
    assert_eq!(
        setup.fake.close_count(),
        1,
        "the scope change takes the OS popup with it"
    );
}

#[gpui::test]
fn a_native_select_lists_groups_and_checks(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Select, cx);
    click(&mut setup, "select");
    setup.cx.run_until_parked();
    let captured = setup.fake.request(0);
    assert_eq!(
        captured.rows,
        vec![
            ("English".to_string(), true, Some(true)),
            ("More".to_string(), false, None),
            ("Chinese".to_string(), true, Some(false)),
            ("Japanese".to_string(), true, Some(false)),
        ],
        "the current value wears the check; the group caption presents disabled"
    );
    // Choosing the row after the caption.
    setup.fake.click(2);
    drain(&mut setup);
    assert_eq!(*setup.log.borrow(), ["value:zh"]);
}

#[gpui::test]
fn a_native_context_menu_opens_at_the_point(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Context, cx);
    let region = setup.cx.debug_bounds("context-region").unwrap();
    setup
        .cx
        .simulate_mouse_down(region.center(), gpui::MouseButton::Right, Modifiers::none());
    setup.cx.run_until_parked();
    let captured = setup.fake.request(0);
    assert_eq!(captured.rows, vec![("Copy".to_string(), true, None)],);
    assert!(
        captured.anchor.contains(&region.center()),
        "the anchor is where the pointer came down"
    );
    assert!(!captured.align_end);
}

#[gpui::test]
fn a_menubar_hands_the_bar_to_the_name_the_pointer_rests_on(cx: &mut TestAppContext) {
    let mut setup = setup(Kind::Menubar, cx);
    click(&mut setup, "dz-menubar-name-0");
    setup.cx.run_until_parked();
    assert_eq!(setup.fake.request_count(), 1);
    assert_eq!(setup.fake.request(0).rows.len(), 2, "File's two rows");

    // The pointer came to rest on View while File was up.
    let view_name = setup.cx.debug_bounds("dz-menubar-name-1").unwrap();
    setup.fake.set_pointer(Some(view_name.center()));
    setup.fake.dismiss_before(0);
    drain(&mut setup);
    assert_eq!(
        setup.fake.request_count(),
        2,
        "the closed menu hands the bar to View"
    );
    let captured = setup.fake.request(1);
    assert_eq!(captured.rows.len(), 2);
    assert_eq!(
        captured.rows[0].0, "Zoom In",
        "View's rows are the ones presented"
    );

    // Choosing from View, then dismissing for good.
    setup.fake.click(0);
    drain(&mut setup);
    assert_eq!(
        setup.log.borrow().last().map(String::as_str),
        Some("zoom-in")
    );
}
