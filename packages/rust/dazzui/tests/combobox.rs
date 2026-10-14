//! The combobox and the autocomplete, driven from the keyboard: typing
//! narrows the list, the arrows reach a row, Return chooses it, Escape
//! closes and then clears.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

mod support;
use support::keys;

use dazzui::{Autocomplete, Combobox, SelectOption};
use gpui::{
    Context, InteractiveElement, IntoElement, Modifiers, ParentElement, Render, Styled,
    TestAppContext, VisualTestContext, Window, canvas, div, point, px,
};

type Log = Rc<RefCell<Vec<String>>>;
type Painted = Rc<RefCell<HashSet<&'static str>>>;

/// Painted exactly while the sheet it is in is drawn.
fn marker(selector: &'static str, painted: &Painted) -> impl IntoElement {
    let painted = painted.clone();
    div().size(px(8.)).child(
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
        let log = self.log.clone();
        let typed = self.log.clone();
        div()
            .size_full()
            .p(px(40.))
            .flex()
            .flex_col()
            .gap(px(200.))
            .child(
                div().w(px(240.)).debug_selector(|| "combo".into()).child(
                    Combobox::new("combo")
                        .options([
                            SelectOption::new("cn", "China"),
                            SelectOption::new("de", "Germany"),
                            SelectOption::new("jp", "Japan"),
                            SelectOption::new("nz", "New Zealand"),
                        ])
                        .empty(marker("empty", &self.painted))
                        .on_value_change(move |value, _, _| {
                            log.borrow_mut().push(format!(
                                "value:{}",
                                value.as_ref().map_or("none", |v| v.as_ref())
                            ))
                        }),
                ),
            )
            .child(
                div().w(px(240.)).debug_selector(|| "auto".into()).child(
                    Autocomplete::new("auto")
                        .items(["Inter", "IBM Plex Sans", "JetBrains Mono"])
                        .empty(marker("auto-empty", &self.painted))
                        .on_value_change(move |value, _, _| {
                            typed.borrow_mut().push(format!("text:{value}"))
                        }),
                ),
            )
    }
}

fn setup(cx: &mut TestAppContext) -> (Log, Painted, &mut VisualTestContext) {
    cx.update(dazzui::init);
    let log: Log = Rc::default();
    let painted: Painted = Rc::default();
    let (harness_log, harness_painted) = (log.clone(), painted.clone());
    let (_, cx) = cx.add_window_view(|_, _| Harness {
        log: harness_log,
        painted: harness_painted,
    });
    cx.run_until_parked();
    (log, painted, cx)
}

fn drawn(painted: &Painted, cx: &mut VisualTestContext, selector: &'static str) -> bool {
    cx.update(|window, _| window.refresh());
    cx.run_until_parked();
    painted.borrow().contains(selector)
}

/// A press in the field's text, which focuses it.
fn focus(cx: &mut VisualTestContext, selector: &'static str) {
    let bounds = cx.debug_bounds(selector).expect(selector);
    cx.simulate_click(
        point(bounds.left() + px(20.), bounds.center().y),
        Modifiers::none(),
    );
    cx.run_until_parked();
}

#[gpui::test]
fn combobox_filters_chooses_and_clears(cx: &mut TestAppContext) {
    let (log, painted, cx) = setup(cx);
    focus(cx, "combo");

    // Nothing matches: the list says so.
    cx.simulate_input("zz");
    assert!(drawn(&painted, cx, "empty"), "typing opens the list");
    cx.simulate_keystrokes("backspace backspace");
    assert!(!drawn(&painted, cx, "empty"), "everything matches again");

    // "ja" leaves only Japan; Down reaches it, Return chooses it.
    cx.simulate_input("ja");
    cx.simulate_keystrokes("down enter");
    assert_eq!(*log.borrow(), ["value:jp"]);

    // Filtering is a case-insensitive "contains": "AN" leaves Germany,
    // Japan and New Zealand; the second of them is Japan.
    cx.simulate_keystrokes(keys("cmd-a", "ctrl-a"));
    cx.simulate_input("AN");
    cx.simulate_keystrokes("down down enter");
    assert_eq!(*log.borrow(), ["value:jp", "value:jp"]);

    // Escape with the list open closes it; the query is dropped.
    cx.simulate_keystrokes(keys("cmd-a", "ctrl-a"));
    cx.simulate_input("qq");
    assert!(drawn(&painted, cx, "empty"));
    cx.simulate_keystrokes("escape");
    assert!(!drawn(&painted, cx, "empty"), "Escape closes the list");
    assert_eq!(log.borrow().len(), 2, "closing chooses nothing");

    // Escape again, the list shut: the choice is cleared.
    cx.simulate_keystrokes("escape");
    assert_eq!(*log.borrow(), ["value:jp", "value:jp", "value:none"]);

    // Down on a shut list opens it on the first row; an outside press
    // closes it.
    cx.simulate_keystrokes("down enter");
    assert_eq!(log.borrow().last().map(String::as_str), Some("value:cn"));
    cx.simulate_input("xyz");
    assert!(drawn(&painted, cx, "empty"));
    cx.simulate_click(point(px(900.), px(700.)), Modifiers::none());
    assert!(!drawn(&painted, cx, "empty"), "an outside press closes it");
}

#[gpui::test]
fn autocomplete_suggests_and_keeps_what_was_typed(cx: &mut TestAppContext) {
    let (log, painted, cx) = setup(cx);
    focus(cx, "auto");

    cx.simulate_input("mono");
    assert_eq!(log.borrow().last().map(String::as_str), Some("text:mono"));
    cx.simulate_keystrokes("down enter");
    assert_eq!(
        log.borrow().last().map(String::as_str),
        Some("text:JetBrains Mono")
    );

    // Free text is a value too; the list only says nothing matches.
    cx.simulate_keystrokes(keys("cmd-a", "ctrl-a"));
    cx.simulate_input("Futura");
    assert!(drawn(&painted, cx, "auto-empty"));
    cx.simulate_keystrokes("escape");
    assert!(!drawn(&painted, cx, "auto-empty"));
    assert_eq!(log.borrow().last().map(String::as_str), Some("text:Futura"));
}
