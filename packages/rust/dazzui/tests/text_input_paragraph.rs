//! Paragraph selection through the rendered editor's actual pointer handlers.

use dazzui::foundation::text_input::{InputState, TextInput};
use gpui::{
    AppContext, Context, Entity, EntityInputHandler, IntoElement, Modifiers, MouseButton,
    MouseDownEvent, MouseUpEvent, ParentElement, Pixels, Point, Render, Styled, TestAppContext,
    VisualTestContext, Window, div, point, px,
};
use serde_json::Value;

struct Harness {
    input: Entity<InputState>,
    multiline: bool,
    masked: bool,
    read_only: bool,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(74.))
            .font_family("monospace")
            .text_size(px(12.))
            .line_height(px(19.2))
            .child(
                TextInput::new("paragraph")
                    .state(self.input.clone())
                    .multiline(self.multiline)
                    .masked(self.masked)
                    .read_only(self.read_only)
                    .rows(8),
            )
    }
}

fn fixtures() -> Value {
    serde_json::from_str(include_str!(
        "../../../js/dazzui/example/scripts/text-field-paragraph-selection.cases.json"
    ))
    .unwrap()
}

fn open<'a>(
    cx: &'a mut TestAppContext,
    fixture: &Value,
) -> (Entity<InputState>, &'a mut VisualTestContext) {
    cx.update(dazzui::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        input: cx.new(|cx| InputState::new(window, cx)),
        multiline: fixture["multiline"] == true,
        masked: fixture["type"] == "password",
        read_only: fixture["readOnly"] == true,
    });
    cx.run_until_parked();
    let input = view.read_with(cx, |view, _| view.input.clone());
    cx.update(|window, cx| input.read(cx).focus(window));
    (input, cx)
}

fn reset(input: &Entity<InputState>, fixture: &Value, cx: &mut VisualTestContext) {
    cx.update(|_, cx| {
        input.update(cx, |input, cx| {
            input.set_value(fixture["text"].as_str().unwrap(), cx);
            input.select_all(cx);
        })
    });
    cx.simulate_keystrokes("left");
    cx.run_until_parked();
}

fn position(value: &Value) -> Point<Pixels> {
    point(
        px(value[0].as_f64().unwrap() as f32),
        px(value[1].as_f64().unwrap() as f32),
    )
}

fn down(position: Point<Pixels>, count: usize, cx: &mut VisualTestContext) {
    cx.simulate_event(MouseDownEvent {
        position,
        button: MouseButton::Left,
        click_count: count,
        ..Default::default()
    });
}

fn up(position: Point<Pixels>, count: usize, cx: &mut VisualTestContext) {
    cx.simulate_event(MouseUpEvent {
        position,
        button: MouseButton::Left,
        click_count: count,
        ..Default::default()
    });
}

fn snapshot(input: &Entity<InputState>, cx: &mut VisualTestContext) -> Value {
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            let selection = input.selected_text_range(false, window, cx).unwrap();
            serde_json::json!([
                selection.range.start,
                selection.range.end,
                selection.reversed
            ])
        })
    })
}

#[gpui::test]
fn triple_click_selects_complete_paragraphs_and_their_line_separators(cx: &mut TestAppContext) {
    for fixture in fixtures()["cases"].as_array().unwrap() {
        let (input, cx) = open(cx, fixture);
        for check in fixture["clicks"].as_array().unwrap() {
            reset(&input, fixture, cx);
            let at = position(&check["point"]);
            down(at, 1, cx);
            up(at, 1, cx);
            down(at, 3, cx);
            up(at, 3, cx);
            assert_eq!(
                snapshot(&input, cx),
                check["selected"],
                "{}: {check}",
                fixture["id"]
            );
            cx.simulate_keystrokes("shift-right");
            assert_eq!(
                snapshot(&input, cx),
                check["afterShiftRight"],
                "{}: keyboard after {check}",
                fixture["id"]
            );
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[gpui::test]
fn triple_click_drag_keeps_the_original_paragraph_and_platform_direction(cx: &mut TestAppContext) {
    let platform = if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    };
    for fixture in fixtures()["cases"].as_array().unwrap() {
        let (input, cx) = open(cx, fixture);
        for check in fixture["drags"].as_array().unwrap() {
            reset(&input, fixture, cx);
            let expected = &check["expectations"][platform];
            down(position(&check["from"]), 3, cx);
            assert_eq!(
                snapshot(&input, cx),
                expected["states"][0],
                "{}: {check}",
                fixture["id"]
            );
            for (index, at) in check["to"].as_array().unwrap().iter().enumerate() {
                cx.simulate_mouse_move(position(at), Some(MouseButton::Left), Modifiers::none());
                assert_eq!(
                    snapshot(&input, cx),
                    expected["states"][index + 1],
                    "{}: move {at}",
                    fixture["id"]
                );
            }
            up(
                position(check["to"].as_array().unwrap().last().unwrap()),
                3,
                cx,
            );
            cx.simulate_keystrokes("shift-right");
            assert_eq!(
                snapshot(&input, cx),
                expected["afterShiftRight"],
                "{}: keyboard after {check}",
                fixture["id"]
            );
        }
    }
}
