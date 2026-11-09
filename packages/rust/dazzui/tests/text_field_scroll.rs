//! Textarea rows size a viewport; editing and wheel input scroll its content.

#[allow(dead_code)]
mod support;

use dazzui::{
    Size, TextField, Theme,
    foundation::text_input::{InputState, input_state},
};
use gpui::{
    Bounds, Context, ElementId, Entity, EntityInputHandler, InteractiveElement, IntoElement,
    Modifiers, ObservedPaint, ParentElement, Pixels, Render, ScrollDelta, ScrollHandle,
    ScrollWheelEvent, StatefulInteractiveElement, Styled, TestAppContext, TouchPhase,
    VisualTestContext, Window, div, point, px,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use support::{keys, paint::PaintProbe};

struct Harness {
    rows: usize,
    size: Size,
    multiline: bool,
    viewport_height: f32,
    read_only: bool,
    disabled: bool,
    input: Option<Entity<InputState>>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
    parent: ScrollHandle,
    changes: Rc<Cell<usize>>,
}

impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.input = Some(window.with_id("page", |window| {
            window.with_id(
                ElementId::Name(std::any::type_name::<TextField>().into()),
                |window| input_state("field", window, cx),
            )
        }));
        let changes = self.changes.clone();
        div()
            .id("page")
            .w(px(320.))
            .h(px(self.viewport_height))
            .overflow_y_scroll()
            .track_scroll(&self.parent)
            .child(PaintProbe {
                child: TextField::new("field")
                    .multiline(self.multiline)
                    .size(self.size)
                    .rows(self.rows)
                    .read_only(self.read_only)
                    .disabled(self.disabled)
                    .on_change(move |_, _, _| changes.set(changes.get() + 1))
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
        size: Size::Medium,
        multiline: true,
        viewport_height: 160.,
        read_only: false,
        disabled: false,
        input: None,
        paints: Rc::default(),
        parent: ScrollHandle::new(),
        changes: Rc::default(),
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
            // This test platform has synthetic font metrics. Host-specific
            // caret scroll values are checked against native browser fields.
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
        let inset = if cfg!(target_os = "linux") { 1. } else { 13. };
        assert_eq!(caret(&input, 0, cx).top(), before.top() + px(inset));
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
    let unscrolled_top = caret(&input, 0, cx).top();
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
        if cfg!(target_os = "linux") {
            first.top()
        } else {
            first.top() - px(7.)
        },
        "navigation reveals a clipped Linux caret but retains a visible macOS caret"
    );
    wheel(&view, 10000., cx);
    assert_eq!(caret(&input, 0, cx).top(), unscrolled_top - px(326.));
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

fn grip(view: &Entity<Harness>, cx: &mut VisualTestContext) -> gpui::Point<Pixels> {
    let bounds = surface(view, cx);
    point(bounds.right() - px(3.), bounds.bottom() - px(3.))
}

fn down(position: gpui::Point<Pixels>, cx: &mut VisualTestContext) {
    cx.simulate_event(gpui::MouseMoveEvent {
        position,
        ..Default::default()
    });
    cx.simulate_event(gpui::MouseDownEvent {
        position,
        button: gpui::MouseButton::Left,
        click_count: 1,
        ..Default::default()
    });
    cx.run_until_parked();
}

fn moved(position: gpui::Point<Pixels>, pressed: bool, cx: &mut VisualTestContext) {
    cx.simulate_event(gpui::MouseMoveEvent {
        position,
        pressed_button: pressed.then_some(gpui::MouseButton::Left),
        ..Default::default()
    });
    cx.run_until_parked();
}

fn up(position: gpui::Point<Pixels>, cx: &mut VisualTestContext) {
    cx.simulate_event(gpui::MouseUpEvent {
        position,
        button: gpui::MouseButton::Left,
        ..Default::default()
    });
    cx.run_until_parked();
}

#[gpui::test]
fn resize_keeps_width_selection_and_focus_even_when_readonly_or_disabled(cx: &mut TestAppContext) {
    for (read_only, disabled) in [(false, false), (true, false), (false, true)] {
        let (view, input, cx) = setup(cx);
        view.update(cx, |view, cx| {
            view.read_only = read_only;
            view.disabled = disabled;
            cx.notify();
        });
        cx.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.set_value(lines(), cx);
                input.select_all(cx)
            });
            window.blur();
        });
        cx.run_until_parked();
        let before = surface(&view, cx);
        let selection = input.read_with(cx, |input, _| input.selected_range());
        let changes = view.read_with(cx, |view, _| view.changes.get());
        let from = grip(&view, cx);
        down(from, cx);
        let to = from + point(px(70.), px(44.));
        moved(to, true, cx);
        assert_eq!(
            surface(&view, cx).size,
            gpui::size(before.size.width, px(124.))
        );
        assert_eq!(
            input.read_with(cx, |input, _| input.selected_range()),
            selection
        );
        assert_eq!(
            input.read_with(cx, |input, _| input.value().to_owned()),
            lines()
        );
        assert!(!cx.update(|window, cx| input.read(cx).is_focused(window)));
        assert_eq!(view.read_with(cx, |view, _| view.changes.get()), changes);
        up(to, cx);
        moved(to + point(px(0.), px(30.)), false, cx);
        assert_eq!(surface(&view, cx).size.height, px(124.));
        let from = grip(&view, cx);
        down(from, cx);
        let to = from - point(px(60.), px(500.));
        moved(to, true, cx);
        assert_eq!(surface(&view, cx).size.height, px(80.));
        up(to, cx);
    }
}

#[gpui::test]
fn resized_height_survives_rows_and_size_changes_and_resets_with_element_type(
    cx: &mut TestAppContext,
) {
    let (view, _, cx) = setup(cx);
    let from = grip(&view, cx);
    down(from, cx);
    moved(from + point(px(0.), px(44.)), true, cx);
    up(from + point(px(0.), px(44.)), cx);
    view.update(cx, |view, cx| {
        view.rows = 12;
        view.size = Size::Large;
        cx.notify()
    });
    cx.run_until_parked();
    assert_eq!(surface(&view, cx).size.height, px(124.));
    view.update(cx, |view, cx| {
        view.multiline = false;
        cx.notify()
    });
    cx.run_until_parked();
    view.update(cx, |view, cx| {
        view.multiline = true;
        cx.notify()
    });
    cx.run_until_parked();
    assert!(surface(&view, cx).size.height > px(124.));
}

#[gpui::test]
fn resize_returns_to_its_origin_and_lost_capture_stops_the_drag(cx: &mut TestAppContext) {
    let (view, _, cx) = setup(cx);
    let from = grip(&view, cx);
    down(from, cx);
    moved(from + point(px(0.), px(44.)), true, cx);
    moved(from, true, cx);
    assert_eq!(surface(&view, cx).size.height, px(80.));
    moved(from + point(px(0.), px(44.)), true, cx);
    moved(from, false, cx);
    moved(from + point(px(0.), px(90.)), true, cx);
    assert_eq!(surface(&view, cx).size.height, px(124.));
}

#[gpui::test]
fn resize_preserves_manual_scrolling_instead_of_revealing_an_offscreen_caret(
    cx: &mut TestAppContext,
) {
    let (view, input, cx) = setup(cx);
    cx.update(|window, cx| input.read(cx).focus(window));
    cx.simulate_input(&lines());
    cx.simulate_keystrokes(keys("cmd-up", "ctrl-home"));
    wheel(&view, 40., cx);
    let before = caret(&input, 0, cx).top();
    let from = grip(&view, cx);
    down(from, cx);
    let to = from + point(px(70.), px(44.));
    moved(to, true, cx);
    assert_eq!(caret(&input, 0, cx).top(), before);
    assert_eq!(input.read_with(cx, |input, _| input.selected_range()), 0..0);
    assert!(cx.update(|window, cx| input.read(cx).is_focused(window)));
    up(to, cx);
    cx.simulate_keystrokes("right");
    assert_eq!(
        caret(&input, 0, cx).top(),
        surface(&view, cx).top() + px(if cfg!(target_os = "linux") { 1. } else { 13. })
    );
}

#[gpui::test]
fn window_deactivation_cancels_resizing_without_reverting_the_size(cx: &mut TestAppContext) {
    let (view, _, cx) = setup(cx);
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    assert!(cx.update(|window, _| window.is_window_active()));
    let from = grip(&view, cx);
    down(from, cx);
    moved(from + point(px(0.), px(44.)), true, cx);
    cx.deactivate_window();
    assert!(!cx.update(|window, _| window.is_window_active()));
    moved(from + point(px(0.), px(90.)), true, cx);
    assert_eq!(surface(&view, cx).size.height, px(124.));
}

#[gpui::test]
fn horizontal_motion_keeps_intrinsic_rows_and_fractional_rows_resize_on_the_pixel_grid(
    cx: &mut TestAppContext,
) {
    let (view, _, cx) = setup(cx);
    let from = grip(&view, cx);
    down(from, cx);
    moved(from + point(px(70.), px(0.)), true, cx);
    up(from + point(px(70.), px(0.)), cx);
    view.update(cx, |view, cx| {
        view.rows = 8;
        // The grip must remain visible in the parent's scroll viewport.
        view.viewport_height = 400.;
        cx.notify()
    });
    cx.run_until_parked();
    let initial = surface(&view, cx);
    assert!(
        initial.size.height > px(80.),
        "horizontal motion must not install an explicit height"
    );
    let from = grip(&view, cx);
    down(from, cx);
    moved(from + point(px(70.), px(44.)), true, cx);
    assert_eq!(
        surface(&view, cx).size.height,
        initial.size.height.round() + px(44.)
    );
    up(from + point(px(70.), px(44.)), cx);
}
