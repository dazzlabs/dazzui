//! Visual rows, caret affinity and the column retained by vertical movement.

#[allow(dead_code)]
mod support;

use dazzui::foundation::text_input::{InputState, TextInput};
use gpui::{
    AppContext, Bounds, Context, Entity, EntityInputHandler, IntoElement, ObservedPaint,
    ParentElement, Pixels, Render, Styled, TestAppContext, VisualTestContext, Window, div, px,
};
use std::{cell::RefCell, rc::Rc};
use support::{keys, paint::PaintProbe};

struct Harness {
    input: Entity<InputState>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(74.))
            .font_family("monospace")
            .text_size(px(12.))
            .line_height(px(19.2))
            .child(PaintProbe {
                child: TextInput::new("input")
                    .state(self.input.clone())
                    .multiline(true)
                    .rows(4)
                    .into_any_element(),
                paints: self.paints.clone(),
            })
    }
}
fn setup<'a>(
    cx: &'a mut TestAppContext,
    text: &str,
) -> (
    Entity<Harness>,
    Entity<InputState>,
    &'a mut VisualTestContext,
) {
    cx.update(dazzui::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        input: cx.new(|cx| InputState::new(window, cx)),
        paints: Rc::default(),
    });
    cx.run_until_parked();
    let input = view.read_with(cx, |view, _| view.input.clone());
    cx.update(|window, cx| input.read(cx).focus(window));
    cx.simulate_input(text);
    cx.run_until_parked();
    (view, input, cx)
}
fn select(input: &Entity<InputState>, offset: usize, cx: &mut VisualTestContext) {
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    cx.simulate_keystrokes("left");
    if offset > 0 {
        cx.simulate_keystrokes(&vec!["right"; offset].join(" "));
    }
}
fn at(
    input: &Entity<InputState>,
    key: &str,
    expected: std::ops::Range<usize>,
    cx: &mut VisualTestContext,
) {
    cx.simulate_keystrokes(key);
    assert_eq!(
        input.read_with(cx, |input, _| input.selected_range()),
        expected,
        "{key}"
    );
}
fn caret(input: &Entity<InputState>, cx: &mut VisualTestContext) -> Bounds<Pixels> {
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            let range = input.selected_text_range(true, window, cx).unwrap();
            let cursor = if range.reversed {
                range.range.start
            } else {
                range.range.end
            };
            input
                .bounds_for_range(cursor..cursor, Default::default(), window, cx)
                .unwrap()
        })
    })
}

#[gpui::test]
fn vertical_movement_retains_the_column_through_short_rows_and_shift_selections(
    cx: &mut TestAppContext,
) {
    let (_, input, cx) = setup(cx, "abcdefghij\nx\nabcdefghij");
    select(&input, 8, cx);
    for (key, offset) in [("down", 12), ("down", 21), ("up", 12), ("up", 8)] {
        at(&input, key, offset..offset, cx);
    }
    for (key, range) in [
        ("shift-down", 8..12),
        ("shift-down", 8..21),
        ("shift-up", 8..12),
        ("shift-up", 8..8),
    ] {
        at(&input, key, range, cx);
    }
    for (key, offset) in [
        ("down", 12),
        ("down", 21),
        ("left", 20),
        ("up", 12),
        ("up", 7),
    ] {
        at(&input, key, offset..offset, cx);
    }
    for (reversed, key, target) in [
        (false, "up", 0),
        (false, "down", 23),
        (true, "up", 0),
        (true, "down", 23),
    ] {
        select(&input, if reversed { 21 } else { 8 }, cx);
        cx.simulate_keystrokes(if reversed {
            "shift-up shift-up"
        } else {
            "shift-down shift-down"
        });
        at(&input, key, target..target, cx);
    }
}

#[gpui::test]
fn visual_home_end_and_selections_keep_the_correct_side_of_a_wrap(cx: &mut TestAppContext) {
    let (view, input, cx) = setup(cx, &"abcdefghij".repeat(4));
    select(&input, 15, cx);
    let middle = caret(&input, cx);
    at(&input, keys("cmd-left", "home"), 10..10, cx);
    assert_eq!(
        caret(&input, cx).size.width,
        px(0.),
        "a collapsed IME range has no width"
    );
    assert_eq!(
        caret(&input, cx).top(),
        middle.top(),
        "Home stays on the same visual row"
    );
    at(&input, keys("cmd-right", "end"), 20..20, cx);
    assert_eq!(
        caret(&input, cx).top(),
        middle.top(),
        "End stays on the same visual row"
    );
    at(&input, keys("cmd-left", "home"), 10..10, cx);
    select(&input, 9, cx);
    at(&input, "right", 10..10, cx);
    at(&input, keys("cmd-left", "home"), 10..10, cx);
    select(&input, 11, cx);
    at(&input, "left", 10..10, cx);
    at(&input, keys("cmd-right", "end"), 20..20, cx);
    select(&input, 15, cx);
    at(&input, keys("cmd-shift-left", "shift-home"), 10..15, cx);
    select(&input, 15, cx);
    at(&input, keys("cmd-shift-right", "shift-end"), 15..20, cx);
    // The platform's IME caret rectangle must describe the submitted caret.
    at(&input, "left", 15..15, cx);
    let rect = caret(&input, cx);
    let paint = view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone());
    let quad = paint
        .quads
        .iter()
        .find(|quad| quad.bounds.size.width == px(1.))
        .unwrap();
    assert_eq!(quad.bounds.origin, rect.origin);
    assert_eq!(quad.bounds.size.height, rect.size.height);
}

#[gpui::test]
fn visual_rows_include_wrapped_spaces_and_delete_only_to_the_current_row_start(
    cx: &mut TestAppContext,
) {
    let (_, input, cx) = setup(cx, "abc def ghi jkl mno pqr");
    select(&input, 4, cx);
    at(&input, keys("cmd-right", "end"), 8..8, cx);
    at(&input, keys("cmd-left", "home"), 0..0, cx);
    select(&input, 12, cx);
    at(&input, keys("cmd-left", "home"), 8..8, cx);
    at(&input, keys("cmd-right", "end"), 16..16, cx);
    if cfg!(target_os = "macos") {
        cx.update(|_, cx| {
            input.update(cx, |input, cx| input.set_value("abcdefghij".repeat(4), cx))
        });
        cx.run_until_parked();
        select(&input, 15, cx);
        cx.simulate_keystrokes("cmd-backspace");
        assert_eq!(
            input.read_with(cx, |input, _| input.value().to_owned()),
            "abcdefghijfghijabcdefghijabcdefghij"
        );
    }
}
