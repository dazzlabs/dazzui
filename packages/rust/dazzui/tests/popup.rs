//! The floating components, opened and driven the way a person would: a
//! press on the trigger, the keyboard inside the sheet, a press outside it.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;

use dazzui::{Menu, MenuItem, Popover, PopoverClose, PreviewCard, Select, SelectOption, Side};
use gpui::{
    AnyElement, App, Context, InteractiveElement, IntoElement, Modifiers, ParentElement, Render,
    StatefulInteractiveElement, Styled, TestAppContext, VisualTestContext, Window, canvas, div,
    point, px,
};

type Log = Rc<RefCell<Vec<String>>>;

/// The markers painted in the latest frame. `debug_bounds` alone cannot say
/// a sheet has gone: gpui 0.2.2 never clears it between frames.
type Painted = Rc<RefCell<HashSet<&'static str>>>;

/// A plain pressable box standing in for a `Button`, found by `selector`.
fn trigger(selector: &'static str, press: Rc<dyn Fn(&mut Window, &mut App)>) -> AnyElement {
    div()
        .id(selector)
        .debug_selector(move || selector.into())
        .w(px(80.))
        .h(px(24.))
        .on_click(move |_, window, cx| press(window, cx))
        .into_any_element()
}

/// A marker inside a sheet: painted exactly while the sheet is drawn.
fn marker(selector: &'static str, painted: &Painted) -> impl IntoElement {
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
}

struct Harness {
    log: Log,
    painted: Painted,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.painted.borrow_mut().clear();
        let painted = &self.painted;
        let log = self.log.clone();
        let push = move |entry: &str| {
            let log = log.clone();
            let entry = entry.to_string();
            move |_: &mut Window, _: &mut App| log.borrow_mut().push(entry.clone())
        };
        let open_log = self.log.clone();
        let value_log = self.log.clone();
        div()
            .size_full()
            .p(px(40.))
            .flex()
            .flex_col()
            .gap(px(40.))
            .child(
                Menu::new("menu")
                    .trigger(|press, _, _, _| trigger("menu-trigger", press))
                    .items([
                        MenuItem::new("Rename")
                            .icon(marker("menu-row", painted))
                            .on_select(push("rename")),
                        MenuItem::new("Delete")
                            .disabled(true)
                            .on_select(push("delete")),
                        MenuItem::new("Duplicate").on_select(push("duplicate")),
                    ])
                    .on_open_change(move |open, _, _| {
                        open_log.borrow_mut().push(format!("open:{open}"))
                    }),
            )
            .child(
                Popover::new("popover")
                    .side(Side::Right)
                    .trigger(|press, _, _, _| trigger("popover-trigger", press))
                    .title("Share")
                    .child(marker("popover-body", painted))
                    .child(PopoverClose::new(|close, _, _| {
                        trigger("popover-close", close)
                    })),
            )
            .child(
                div().w(px(200.)).debug_selector(|| "select".into()).child(
                    Select::new("select")
                        .options([
                            SelectOption::new("en", "English"),
                            SelectOption::new("zh", "Chinese"),
                            SelectOption::new("ja", "Japanese"),
                        ])
                        .default_value("en")
                        .on_value_change(move |value, _, _| {
                            value_log.borrow_mut().push(format!("value:{value}"))
                        }),
                ),
            )
            .child(
                PreviewCard::new("card")
                    .trigger(div().debug_selector(|| "card-trigger".into()).size(px(40.)))
                    .child(marker("card-body", painted)),
            )
    }
}

struct Setup<'a> {
    log: Log,
    painted: Painted,
    cx: &'a mut VisualTestContext,
}

fn setup(cx: &mut TestAppContext) -> Setup<'_> {
    cx.update(dazzui::init);
    let log: Log = Rc::default();
    let painted: Painted = Rc::default();
    let (harness_log, harness_painted) = (log.clone(), painted.clone());
    let (_, cx) = cx.add_window_view(|_, _| Harness {
        log: harness_log,
        painted: harness_painted,
    });
    cx.run_until_parked();
    Setup { log, painted, cx }
}

/// Whether the marker was painted in a fresh frame.
fn drawn(painted: &Painted, cx: &mut VisualTestContext, selector: &'static str) -> bool {
    cx.update(|window, _| window.refresh());
    cx.run_until_parked();
    painted.borrow().contains(selector)
}

fn click(cx: &mut VisualTestContext, selector: &'static str) {
    let bounds = cx.debug_bounds(selector).expect(selector);
    cx.simulate_click(bounds.center(), Modifiers::none());
    cx.run_until_parked();
}

#[gpui::test]
fn menu_opens_roves_and_closes(cx: &mut TestAppContext) {
    let Setup { log, painted, cx } = setup(cx);
    assert!(!drawn(&painted, cx, "menu-row"), "closed at first");

    click(cx, "menu-trigger");
    assert!(drawn(&painted, cx, "menu-row"), "the press opens it");
    let trigger = cx.debug_bounds("menu-trigger").unwrap();
    let row = cx.debug_bounds("menu-row").unwrap();
    assert!(row.top() > trigger.bottom(), "it opens below the trigger");

    cx.simulate_keystrokes("escape");
    assert!(!drawn(&painted, cx, "menu-row"), "Escape closes it");

    click(cx, "menu-trigger");
    // Down reaches Rename, the next Down skips the disabled Delete.
    cx.simulate_keystrokes("down down enter");
    assert!(!drawn(&painted, cx, "menu-row"), "choosing closes it");
    assert_eq!(
        *log.borrow(),
        [
            "open:true",
            "open:false",
            "open:true",
            "open:false",
            "duplicate"
        ]
    );

    // Type-ahead.
    log.borrow_mut().clear();
    click(cx, "menu-trigger");
    cx.simulate_keystrokes("r enter");
    cx.run_until_parked();
    assert_eq!(*log.borrow(), ["open:true", "open:false", "rename"]);

    // A press outside.
    click(cx, "menu-trigger");
    cx.simulate_click(point(px(900.), px(700.)), Modifiers::none());
    assert!(
        !drawn(&painted, cx, "menu-row"),
        "an outside press closes it"
    );

    // The trigger toggles rather than closing and reopening.
    click(cx, "menu-trigger");
    assert!(drawn(&painted, cx, "menu-row"));
    click(cx, "menu-trigger");
    assert!(!drawn(&painted, cx, "menu-row"), "a second press closes it");
}

#[gpui::test]
fn popover_opens_on_its_side_and_its_close_closes_it(cx: &mut TestAppContext) {
    let Setup { painted, cx, .. } = setup(cx);
    click(cx, "popover-trigger");
    assert!(drawn(&painted, cx, "popover-body"), "the press opens it");
    let trigger = cx.debug_bounds("popover-trigger").unwrap();
    let body = cx.debug_bounds("popover-body").unwrap();
    assert!(
        body.left() > trigger.right(),
        "side right puts it to the right"
    );

    click(cx, "popover-close");
    assert!(
        !drawn(&painted, cx, "popover-body"),
        "PopoverClose closes it"
    );
}

#[gpui::test]
fn select_chooses_from_the_keyboard(cx: &mut TestAppContext) {
    let Setup { log, cx, .. } = setup(cx);
    click(cx, "select");
    cx.simulate_keystrokes("j enter");
    cx.run_until_parked();
    click(cx, "select");
    cx.simulate_keystrokes("up enter");
    cx.run_until_parked();
    // It reopens on Japanese, and Up reaches Chinese.
    assert_eq!(*log.borrow(), ["value:ja", "value:zh"]);
}

#[gpui::test]
fn preview_card_opens_after_the_pointer_rests(cx: &mut TestAppContext) {
    let Setup { painted, cx, .. } = setup(cx);
    let trigger = cx.debug_bounds("card-trigger").unwrap();
    cx.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    assert!(!drawn(&painted, cx, "card-body"), "not before the delay");
    cx.executor().advance_clock(Duration::from_millis(700));
    assert!(drawn(&painted, cx, "card-body"), "open after it");

    cx.simulate_mouse_move(point(px(900.), px(700.)), None, Modifiers::none());
    cx.executor().advance_clock(Duration::from_millis(400));
    assert!(
        !drawn(&painted, cx, "card-body"),
        "closed once the pointer left"
    );
}
