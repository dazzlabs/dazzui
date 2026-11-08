//! Textarea rows size a viewport; editing and wheel input scroll its content.

#[allow(dead_code)]
mod support;

use dazzui::{
    TextField, Theme,
    foundation::text_input::{InputState, input_state},
};
use gpui::{
    Bounds, Context, ElementId, Entity, EntityInputHandler, InteractiveElement, IntoElement,
    Modifiers, ObservedPaint, ParentElement, Pixels, Render, ScrollDelta, ScrollHandle,
    ScrollWheelEvent, StatefulInteractiveElement, Styled, TestAppContext, TouchPhase,
    VisualTestContext, Window, div, point, px,
};
use std::{cell::RefCell, rc::Rc};
use support::{keys, paint::PaintProbe};

struct Harness {
    rows: usize,
    read_only: bool,
    disabled: bool,
    input: Option<Entity<InputState>>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
    parent: ScrollHandle,
}

impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.input = Some(window.with_id("page", |window| {
            window.with_id(
                ElementId::Name(std::any::type_name::<TextField>().into()),
                |window| input_state("field", window, cx),
            )
        }));
        div()
            .id("page")
            .w(px(320.))
            .h(px(160.))
            .overflow_y_scroll()
            .track_scroll(&self.parent)
            .child(PaintProbe {
                child: TextField::new("field")
                    .multiline(true)
                    .rows(self.rows)
                    .read_only(self.read_only)
                    .disabled(self.disabled)
                    .into_any_element(),
                paints: self.paints.clone(),
            })
            .child(div().h(px(800.)))
    }
}

fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, Entity<InputState>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        rows: 3,
        read_only: false,
        disabled: false,
        input: None,
        paints: Rc::default(),
        parent: ScrollHandle::new(),
    });
    cx.run_until_parked();
    let input = view.read_with(cx, |view, _| view.input.clone().unwrap());
    (view, input, cx)
}

fn surface(view: &Entity<Harness>, cx: &mut VisualTestContext) -> Bounds<Pixels> {
    view.read_with(cx, |view, _| {
        view.paints
            .borrow()
            .last()
            .unwrap()
            .decorated_surface
            .as_ref()
            .unwrap()
            .bounds
    })
}

fn caret(input: &Entity<InputState>, at: usize, cx: &mut VisualTestContext) -> Bounds<Pixels> {
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input
                .bounds_for_range(at..at, Default::default(), window, cx)
                .unwrap()
        })
    })
}

fn wheel(view: &Entity<Harness>, delta: f32, cx: &mut VisualTestContext) {
    let bounds = surface(view, cx);
    cx.simulate_event(ScrollWheelEvent {
        position: bounds.center(),
        delta: ScrollDelta::Pixels(point(px(0.), px(-delta))),
        modifiers: Modifiers::none(),
        touch_phase: TouchPhase::Moved,
    });
    cx.run_until_parked();
}

fn lines() -> String {
    (1..=20)
        .map(|i| format!("Line {i}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[gpui::test]
fn explicit_and_wrapped_lines_scroll_without_growing_and_keep_the_caret_visible(
    cx: &mut TestAppContext,
) {
    let (view, input, cx) = setup(cx);
    let before = surface(&view, cx);
    assert_eq!(before.size.height, px(80.));
    cx.update(|window, cx| input.read(cx).focus(window));
    let first = caret(&input, 0, cx);
    for text in [lines(), "word ".repeat(250)] {
        cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
        cx.simulate_input(&text);
        cx.run_until_parked();
        assert_eq!(surface(&view, cx), before);
        if text == lines() {
            assert_eq!(caret(&input, 0, cx).top(), first.top() - px(314.));
        }
        let last = caret(&input, text.len(), cx);
        assert!(last.top() >= before.top() + px(1.));
        assert!(
            last.bottom() <= before.bottom() - px(1.),
            "{last:?} vs {before:?}"
        );
        let parent = view.read_with(cx, |view, _| view.parent.offset());
        assert_eq!(parent.y, px(0.));
        cx.simulate_keystrokes(keys("cmd-up", "ctrl-home"));
        assert_eq!(caret(&input, 0, cx).top(), before.top() + px(13.));
    }
}

#[gpui::test]
fn scrolling_keeps_mouse_and_input_method_coordinates_on_the_visible_text(cx: &mut TestAppContext) {
    let (view, input, cx) = setup(cx);
    cx.update(|window, cx| input.read(cx).focus(window));
    cx.simulate_input(&lines());
    cx.simulate_keystrokes(keys("cmd-up", "ctrl-home"));
    wheel(&view, 60., cx);
    let bounds = surface(&view, cx);
    let position = bounds.origin + point(px(13.), px(13.));
    let at = cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input
                .character_index_for_point(position, window, cx)
                .unwrap()
        })
    });
    assert_eq!(at, 21, "the first visible text is the fourth line");
    cx.simulate_event(gpui::MouseDownEvent {
        position,
        button: gpui::MouseButton::Left,
        click_count: 1,
        modifiers: Modifiers::none(),
        first_mouse: false,
    });
    cx.run_until_parked();
    assert_eq!(
        input.read_with(cx, |input, _| input.selected_range()),
        21..21
    );
    // Replacing all text clamps a previous scroll position back to zero.
    cx.update(|_, cx| input.update(cx, |input, cx| input.select_all(cx)));
    cx.simulate_input("short");
    cx.run_until_parked();
    assert_eq!(caret(&input, 0, cx).top(), bounds.top() + px(13.));
}

#[gpui::test]
fn wheel_keeps_selection_and_bubbles_to_the_page_only_at_the_edge(cx: &mut TestAppContext) {
    let (view, input, cx) = setup(cx);
    cx.update(|window, cx| input.read(cx).focus(window));
    cx.simulate_input(&lines());
    cx.simulate_keystrokes(keys("cmd-up", "ctrl-home"));
    let first = caret(&input, 0, cx);
    cx.simulate_keystrokes(keys("cmd-a", "ctrl-a"));
    assert_eq!(
        caret(&input, 0, cx).top(),
        first.top(),
        "select all retains the viewport"
    );
    cx.simulate_keystrokes("left");
    wheel(&view, 60., cx);
    assert_eq!(caret(&input, 0, cx).top(), first.top() - px(60.));
    assert_eq!(input.read_with(cx, |input, _| input.selected_range()), 0..0);
    assert_eq!(view.read_with(cx, |view, _| view.parent.offset().y), px(0.));
    // An editing gesture reveals the caret again; an ordinary repaint does not.
    cx.simulate_keystrokes("right");
    assert_eq!(caret(&input, 0, cx).top(), first.top());
    wheel(&view, 7., cx);
    cx.simulate_keystrokes(keys("cmd-up", "ctrl-home"));
    assert_eq!(
        caret(&input, 0, cx).top(),
        first.top() - px(7.),
        "a visible caret does not reset scrolling"
    );
    wheel(&view, 10000., cx);
    assert_eq!(caret(&input, 0, cx).top(), first.top() - px(326.));
    assert_eq!(view.read_with(cx, |view, _| view.parent.offset().y), px(0.));
    wheel(&view, 80., cx);
    assert!(view.read_with(cx, |view, _| view.parent.offset().y) < px(0.));
}

#[gpui::test]
fn readonly_disabled_and_blurred_fields_still_scroll_without_editing(cx: &mut TestAppContext) {
    for (read_only, disabled) in [(false, false), (true, false), (false, true)] {
        let (view, input, cx) = setup(cx);
        view.update(cx, |view, cx| {
            view.read_only = read_only;
            view.disabled = disabled;
            cx.notify();
        });
        cx.update(|_, cx| input.update(cx, |input, cx| input.set_value(lines(), cx)));
        cx.run_until_parked();
        let first = caret(&input, 0, cx);
        let selection = input.read_with(cx, |input, _| input.selected_range());
        wheel(&view, 60., cx);
        assert_eq!(caret(&input, 0, cx).top(), first.top() - px(60.));
        assert_eq!(
            input.read_with(cx, |input, _| input.selected_range()),
            selection
        );
        assert_eq!(
            input.read_with(cx, |input, _| input.value().to_owned()),
            lines()
        );
    }
}
