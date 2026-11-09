//! Pointer extension after complete-unit selections, against real React traces.
#![cfg(any(target_os = "macos", target_os = "linux"))]
use dazzui::foundation::text_input::{InputState, TextInput};
use gpui::{
    AppContext, Context, Entity, EntityInputHandler, IntoElement, Modifiers, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement, Render, Styled, TestAppContext,
    Window, div, point, px,
};
use serde_json::{Value, json};
mod support;
use support::keys;

struct Harness {
    input: Entity<InputState>,
    width: f32,
    multiline: bool,
    masked: bool,
    read_only: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.width))
            .font_family("monospace")
            .text_size(px(12.))
            .line_height(px(19.2))
            .child(
                TextInput::new("shift")
                    .state(self.input.clone())
                    .multiline(self.multiline)
                    .masked(self.masked)
                    .read_only(self.read_only)
                    .rows(8),
            )
    }
}

#[gpui::test]
fn shift_pointer_selection_matches_browser_granularity(root: &mut TestAppContext) {
    root.update(dazzui::init);
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../js/dazzui/example/scripts/text-field-shift-selection.cases.json"
    ))
    .unwrap();
    let platform = if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };
    let traces = fixture["platforms"][platform]["results"]
        .as_array()
        .unwrap();
    assert_eq!(traces.len(), fixture["cases"].as_array().unwrap().len());
    let mut failures = Vec::new();
    for c in fixture["cases"].as_array().unwrap() {
        let matching = traces
            .iter()
            .filter(|trace| trace["id"] == c["id"])
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1);
        let trace = matching[0];
        assert_eq!(
            trace["states"].as_array().unwrap().len(),
            c["ops"].as_array().unwrap().len()
        );
        let (view, cx) = root.add_window_view(|window, cx| Harness {
            input: cx.new(|cx| {
                let mut input = InputState::new(window, cx);
                input.set_value(c["text"].as_str().unwrap(), cx);
                input
            }),
            width: c["width"].as_f64().unwrap_or(400.) as f32,
            multiline: c["multiline"] == true,
            masked: c["type"] == "password",
            read_only: c["readOnly"] == true,
        });
        cx.run_until_parked();
        let input = view.read_with(cx, |h, _| h.input.clone());
        cx.update(|window, cx| {
            window.activate_window();
            input.read(cx).focus(window);
            input.update(cx, |s, cx| s.select_all(cx));
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("left");
        let mut held = false;
        let mut shifted = false;
        for (i, op) in c["ops"].as_array().unwrap().iter().enumerate() {
            let kind = op[0].as_str().unwrap();
            if kind == "key" {
                cx.simulate_keystrokes(match op[1].as_str().unwrap() {
                    "all" => keys("cmd-a", "ctrl-a"),
                    "right" => "right",
                    "left" => "left",
                    "shift-right" => "shift-right",
                    key => panic!("Unknown key {key}"),
                });
            } else {
                let position = point(
                    px(op[1].as_f64().unwrap() as f32),
                    px(op[2].as_f64().unwrap() as f32),
                );
                let count = op[3].as_u64().unwrap_or(1) as usize;
                if kind == "click" || kind == "down" {
                    shifted = op[4].as_bool().unwrap_or(false);
                    let modifiers = Modifiers {
                        shift: shifted,
                        ..Modifiers::none()
                    };
                    cx.simulate_event(MouseDownEvent {
                        position,
                        button: MouseButton::Left,
                        click_count: count,
                        modifiers,
                        ..Default::default()
                    });
                    held = true;
                }
                if kind == "move" {
                    cx.simulate_event(MouseMoveEvent {
                        position,
                        pressed_button: held.then_some(MouseButton::Left),
                        modifiers: Modifiers {
                            shift: shifted,
                            ..Modifiers::none()
                        },
                    });
                }
                if kind == "click" || kind == "up" {
                    cx.simulate_event(MouseUpEvent {
                        position,
                        button: MouseButton::Left,
                        click_count: count,
                        modifiers: Modifiers {
                            shift: shifted,
                            ..Modifiers::none()
                        },
                    });
                    held = false;
                    shifted = false;
                }
            }
            cx.run_until_parked();
            let actual = cx.update(|window, cx| {
                input.update(cx, |input, cx| {
                    let s = input.selected_text_range(false, window, cx).unwrap();
                    json!([s.range.start, s.range.end, s.reversed])
                })
            });
            if actual != trace["states"][i] {
                failures.push(format!(
                    "{} step{i} {op}: got {actual}, expected {}",
                    c["id"], trace["states"][i]
                ));
            }
        }
        cx.update(|window, _| window.remove_window());
    }
    assert!(
        failures.is_empty(),
        "{} mismatches:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
