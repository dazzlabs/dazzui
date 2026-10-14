//! The editing every field is built on, driven by keystrokes.

use std::cell::RefCell;
use std::rc::Rc;

mod support;
use support::keys;

use dazzui::foundation::text_input::{InputState, TextInput};
use gpui::{
    AppContext, ClipboardItem, Context, Entity, EntityInputHandler, IntoElement, ParentElement,
    Render, Styled, TestAppContext, VisualTestContext, Window, div, prelude::FluentBuilder, px,
};

struct Harness {
    input: Entity<InputState>,
    multiline: bool,
    masked: bool,
    max_length: Option<usize>,
    line_height: Option<f32>,
    changes: Rc<RefCell<Vec<String>>>,
    submits: Rc<RefCell<Vec<String>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let changes = self.changes.clone();
        let submits = self.submits.clone();
        let mut input = TextInput::new("input")
            .state(self.input.clone())
            .placeholder("Type here")
            .multiline(self.multiline)
            .masked(self.masked)
            .on_change(move |text, _, _| changes.borrow_mut().push(text.to_string()))
            .on_submit(move |text, _, _| submits.borrow_mut().push(text.to_string()));
        if let Some(max) = self.max_length {
            input = input.max_length(max);
        }
        div()
            .w(px(240.))
            .text_size(px(13.))
            .when_some(self.line_height, |this, height| {
                this.line_height(px(height))
            })
            .child(input)
    }
}

struct Setup {
    multiline: bool,
    masked: bool,
    max_length: Option<usize>,
    line_height: Option<f32>,
}

fn open(
    cx: &mut TestAppContext,
    setup: Setup,
) -> (Entity<Harness>, Entity<InputState>, &mut VisualTestContext) {
    cx.update(dazzui::init);
    let (harness, cx) = cx.add_window_view(|window, cx| Harness {
        input: cx.new(|cx| InputState::new(window, cx)),
        multiline: setup.multiline,
        masked: setup.masked,
        max_length: setup.max_length,
        line_height: setup.line_height,
        changes: Rc::default(),
        submits: Rc::default(),
    });
    let input = harness.read_with(cx, |harness, _| harness.input.clone());
    cx.run_until_parked();
    cx.update(|window, cx| input.read(cx).focus(window));
    cx.run_until_parked();
    (harness, input, cx)
}

fn single() -> Setup {
    Setup {
        multiline: false,
        masked: false,
        max_length: None,
        line_height: None,
    }
}

fn value(input: &Entity<InputState>, cx: &mut VisualTestContext) -> String {
    input.read_with(cx, |input, _| input.value().to_string())
}

#[gpui::test]
fn multiline_ime_rectangles_keep_fractional_leading(cx: &mut TestAppContext) {
    let (_, input, cx) = open(
        cx,
        Setup {
            multiline: true,
            line_height: Some(20.8),
            ..single()
        },
    );
    cx.simulate_input("a\nb\nc");
    cx.run_until_parked();
    let (first, second) = cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            (
                input
                    .bounds_for_range(0..0, Default::default(), window, cx)
                    .unwrap(),
                input
                    .bounds_for_range(2..2, Default::default(), window, cx)
                    .unwrap(),
            )
        })
    });
    assert!((f32::from(second.origin.y - first.origin.y) - 20.8).abs() < 0.001);
    assert!((f32::from(second.size.height) - 20.8).abs() < 0.001);
}

#[gpui::test]
fn typing_edits_the_value(cx: &mut TestAppContext) {
    let (harness, input, cx) = open(cx, single());
    cx.simulate_keystrokes("h e l l o");
    assert_eq!(value(&input, cx), "hello");

    cx.simulate_keystrokes("backspace backspace");
    assert_eq!(value(&input, cx), "hel");

    cx.simulate_keystrokes("left left x");
    assert_eq!(value(&input, cx), "hxel");

    let changes = harness.read_with(cx, |harness, _| harness.changes.borrow().clone());
    assert_eq!(changes.first().map(String::as_str), Some("h"));
    assert_eq!(changes.last().map(String::as_str), Some("hxel"));
}

#[gpui::test]
fn input_method_text_arrives_through_the_handler(cx: &mut TestAppContext) {
    let (_, input, cx) = open(cx, single());
    cx.simulate_input("héllo wörld");
    assert_eq!(value(&input, cx), "héllo wörld");
}

#[gpui::test]
fn selection_replaces_and_words_move(cx: &mut TestAppContext) {
    let (_, input, cx) = open(cx, single());
    cx.simulate_input("one two three");
    cx.simulate_keystrokes(keys("alt-left alt-left", "ctrl-left ctrl-left"));
    assert_eq!(input.read_with(cx, |input, _| input.selected_range()), 4..4);

    cx.simulate_keystrokes(keys("alt-shift-right", "ctrl-shift-right"));
    assert_eq!(input.read_with(cx, |input, _| input.selected_range()), 4..7);
    cx.simulate_input("2");
    assert_eq!(value(&input, cx), "one 2 three");

    cx.simulate_keystrokes(keys("cmd-a", "ctrl-a"));
    cx.simulate_input("x");
    assert_eq!(value(&input, cx), "x");

    cx.simulate_input(" y z");
    cx.simulate_keystrokes(keys("alt-backspace", "ctrl-backspace"));
    assert_eq!(value(&input, cx), "x y ");
    cx.simulate_keystrokes(keys("cmd-backspace", "shift-home backspace"));
    assert_eq!(value(&input, cx), "");
}

#[gpui::test]
fn clipboard_copies_cuts_and_pastes(cx: &mut TestAppContext) {
    let (_, input, cx) = open(cx, single());
    cx.simulate_input("copy me");
    cx.simulate_keystrokes(keys("cmd-a cmd-c", "ctrl-a ctrl-c"));
    assert_eq!(
        cx.read_from_clipboard().and_then(|item| item.text()),
        Some("copy me".into())
    );
    cx.simulate_keystrokes(keys("cmd-x", "ctrl-x"));
    assert_eq!(value(&input, cx), "");

    cx.write_to_clipboard(ClipboardItem::new_string("two\nlines".into()));
    cx.simulate_keystrokes(keys("cmd-v", "ctrl-v"));
    // A single line takes a pasted break as a space.
    assert_eq!(value(&input, cx), "two lines");
}

#[gpui::test]
fn return_submits_a_line(cx: &mut TestAppContext) {
    let (harness, input, cx) = open(cx, single());
    cx.simulate_input("done");
    cx.simulate_keystrokes("enter");
    let submits = harness.read_with(cx, |harness, _| harness.submits.borrow().clone());
    assert_eq!(submits, vec!["done".to_string()]);
    assert_eq!(value(&input, cx), "done");
}

#[gpui::test]
fn return_breaks_a_multiline(cx: &mut TestAppContext) {
    let (_, input, cx) = open(
        cx,
        Setup {
            multiline: true,
            ..single()
        },
    );
    cx.simulate_input("one");
    cx.simulate_keystrokes("enter");
    cx.simulate_input("two");
    assert_eq!(value(&input, cx), "one\ntwo");
    cx.simulate_keystrokes("up");
    cx.simulate_input("!");
    assert_eq!(value(&input, cx), "one!\ntwo");
}

#[gpui::test]
fn a_password_is_masked_and_kept_off_the_clipboard(cx: &mut TestAppContext) {
    let (_, input, cx) = open(
        cx,
        Setup {
            masked: true,
            ..single()
        },
    );
    cx.write_to_clipboard(ClipboardItem::new_string("before".into()));
    cx.simulate_input("secret");
    cx.simulate_keystrokes(keys("cmd-a cmd-c", "ctrl-a ctrl-c"));
    assert_eq!(value(&input, cx), "secret");
    assert_eq!(
        cx.read_from_clipboard().and_then(|item| item.text()),
        Some("before".into())
    );
    cx.simulate_keystrokes("left backspace");
    assert_eq!(value(&input, cx), "secret");
    cx.simulate_keystrokes(keys("cmd-right backspace", "end backspace"));
    assert_eq!(value(&input, cx), "secre");
}

#[gpui::test]
fn max_length_caps_what_is_typed(cx: &mut TestAppContext) {
    let (_, input, cx) = open(
        cx,
        Setup {
            max_length: Some(3),
            ..single()
        },
    );
    cx.simulate_input("12345");
    assert_eq!(value(&input, cx), "123");
}
