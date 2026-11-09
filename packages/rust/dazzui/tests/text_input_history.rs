//! Browser-recorded edit sequences, including cross-field history and IME.
use dazzui::foundation::text_input::{InputState, TextInput};
use gpui::{
    AppContext, ClipboardItem, Context, Entity, EntityInputHandler, IntoElement, ParentElement,
    Render, Styled, TestAppContext, Window, div, px,
};
#[cfg(any(target_os = "macos", target_os = "linux"))]
use serde_json::{Value, json};

mod support;
use support::keys;

struct Harness {
    inputs: Vec<Entity<InputState>>,
    initial: Vec<String>,
    controlled: bool,
    multiline: bool,
    masked: bool,
    read_only: bool,
    max_length: Option<usize>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(400.))
            .children(self.inputs.iter().enumerate().map(|(i, input)| {
                let owner = cx.entity().downgrade();
                let controlled = self.controlled;
                let mut field = TextInput::new(i)
                    .state(input.clone())
                    .default_value(self.initial[i].clone())
                    .multiline(self.multiline)
                    .masked(self.masked)
                    .read_only(self.read_only);
                if controlled {
                    field = field
                        .value(self.initial[i].clone())
                        .on_change(move |text, _, cx| {
                            owner
                                .update(cx, |owner, cx| {
                                    owner.initial[i] = text.to_owned();
                                    cx.notify();
                                })
                                .ok();
                        });
                }
                if let Some(limit) = self.max_length {
                    field = field.max_length(limit);
                }
                field
            }))
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[gpui::test]
fn history_matches_browser_sequences(root: &mut TestAppContext) {
    root.update(dazzui::init);
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../js/dazzui/example/scripts/text-field-history.cases.json"
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
    for case in fixture["cases"].as_array().unwrap() {
        if case["platforms"]
            .as_array()
            .is_some_and(|platforms| !platforms.iter().any(|value| value == platform))
        {
            continue;
        }
        for mode in ["text", "multi", "password"] {
            if case["multiline"].as_bool().unwrap_or(false) && mode != "multi" {
                continue;
            }
            let matching = traces
                .iter()
                .filter(|trace| trace["id"] == case["id"] && trace["mode"] == mode)
                .collect::<Vec<_>>();
            assert_eq!(
                matching.len(),
                1,
                "Missing or duplicate trace: {platform}/{mode}/{}",
                case["id"]
            );
            assert_eq!(
                matching[0]["states"].as_array().unwrap().len(),
                case["ops"].as_array().unwrap().len()
            );
        }
    }
    let mut failures = Vec::new();
    for trace in traces {
        let case = fixture["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["id"] == trace["id"])
            .unwrap();
        let mode = trace["mode"].as_str().unwrap();
        let (harness, cx) = root.add_window_view(|window, cx| {
            let values = case["fields"]
                .as_array()
                .cloned()
                .unwrap_or_else(|| vec![case.get("initial").cloned().unwrap_or(json!(""))]);
            let inputs = values
                .iter()
                .map(|value| {
                    cx.new(|cx| {
                        let mut state = InputState::new(window, cx);
                        state.set_value(value.as_str().unwrap(), cx);
                        state
                    })
                })
                .collect();
            Harness {
                controlled: case["controlled"].as_bool().unwrap_or(false),
                initial: values
                    .iter()
                    .map(|value| value.as_str().unwrap().to_owned())
                    .collect(),
                inputs,
                multiline: mode == "multi",
                masked: mode == "password",
                read_only: false,
                max_length: case["limit"].as_u64().map(|n| n as usize),
            }
        });
        cx.run_until_parked();
        let inputs = harness.read_with(cx, |harness, _| harness.inputs.clone());
        let mut focused = 0;
        cx.update(|window, cx| {
            window.activate_window();
            inputs[0].read(cx).focus(window);
        });
        cx.run_until_parked();
        cx.simulate_keystrokes(keys("cmd-down", "ctrl-end"));
        for (i, op) in case["ops"].as_array().unwrap().iter().enumerate() {
            match op[0].as_str().unwrap() {
                "type" => {
                    // Playwright sends ASCII as physical keys, Unicode via
                    // insertText. Exercise those distinct native paths too.
                    for ch in op[1].as_str().unwrap().chars() {
                        let text = ch.to_string();
                        if ch.is_ascii() {
                            cx.simulate_input(&text);
                        } else {
                            cx.update(|window, cx| {
                                inputs[focused].update(cx, |state, cx| {
                                    state.replace_text_in_range(None, &text, window, cx)
                                })
                            });
                        }
                    }
                }
                "insert" | "commit" => cx.update(|window, cx| {
                    inputs[focused].update(cx, |state, cx| {
                        state.replace_text_in_range(None, op[1].as_str().unwrap(), window, cx)
                    })
                }),
                "key" => cx.simulate_keystrokes(match op[1].as_str().unwrap() {
                    "undo" => keys("cmd-z", "ctrl-z"),
                    "redo" => keys("cmd-shift-z", "ctrl-shift-z"),
                    "redoAlt" => "ctrl-y",
                    "start" => keys("cmd-up", "ctrl-home"),
                    "left" => "left",
                    "right" => "right",
                    "sleft" => "shift-left",
                    "sright" => "shift-right",
                    "backspace" => "backspace",
                    "delete" => "delete",
                    "enter" => "enter",
                    "wordBackspace" => keys("alt-backspace", "ctrl-backspace"),
                    "cut" => keys("cmd-x", "ctrl-x"),
                    key => panic!("Unknown key {key}"),
                }),
                "paste" => {
                    cx.update(|_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(
                            op[1].as_str().unwrap().to_owned(),
                        ))
                    });
                    cx.simulate_keystrokes(keys("cmd-v", "ctrl-v"));
                }
                "value" => inputs[focused]
                    .update(cx, |state, cx| state.set_value(op[1].as_str().unwrap(), cx)),
                "readonly" => harness.update(cx, |h, cx| {
                    h.read_only = op[1].as_bool().unwrap();
                    cx.notify();
                }),
                "maxLength" => harness.update(cx, |h, cx| {
                    h.max_length = Some(op[1].as_u64().unwrap() as usize);
                    cx.notify();
                }),
                "blur" => {
                    cx.update(|window, cx| {
                        window.blur();
                        let _ = window.draw(cx);
                    });
                    cx.run_until_parked();
                    cx.update(|window, cx| inputs[focused].read(cx).focus(window));
                }
                "focus" => {
                    focused = op[1].as_u64().unwrap() as usize;
                    cx.update(|window, cx| inputs[focused].read(cx).focus(window));
                    cx.run_until_parked();
                    cx.simulate_keystrokes(keys("cmd-down", "ctrl-end"));
                }
                "ime" => cx.update(|window, cx| {
                    inputs[focused].update(cx, |state, cx| {
                        state.replace_and_mark_text_in_range(
                            None,
                            op[1].as_str().unwrap(),
                            Some(
                                op[2].as_u64().unwrap() as usize..op[3].as_u64().unwrap() as usize,
                            ),
                            window,
                            cx,
                        )
                    })
                }),
                "wait" => {} // History has no wall-clock expiry.
                op => panic!("Unknown operation {op}"),
            }
            cx.run_until_parked();
            let actual = cx.update(|window, cx| {
                let values = inputs.iter().map(|input| input.read(cx).value().to_owned()).collect::<Vec<_>>();
                focused = inputs.iter().position(|input| input.read(cx).is_focused(window)).unwrap();
                let selection = inputs[focused].update(cx, |state, cx| state.selected_text_range(false, window, cx).unwrap());
                json!({"values":values, "focus":focused, "range":[selection.range.start,selection.range.end,selection.reversed]})
            });
            let mut expected = trace["states"][i].clone();
            if let Some(range) = expected.as_object_mut().unwrap().remove("sourceRange") {
                expected["range"] = range;
            }
            if actual != expected {
                failures.push(format!(
                    "{} {mode} step {i} {op}: got {actual}, expected {}",
                    trace["id"], trace["states"][i]
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

#[gpui::test]
fn history_discards_released_fields_and_bounds_undo_memory(root: &mut TestAppContext) {
    root.update(dazzui::init);
    let (harness, cx) = root.add_window_view(|window, cx| Harness {
        inputs: (0..2)
            .map(|_| cx.new(|cx| InputState::new(window, cx)))
            .collect(),
        initial: vec![String::new(), String::new()],
        controlled: false,
        multiline: false,
        masked: false,
        read_only: false,
        max_length: None,
    });
    cx.run_until_parked();
    let first = harness.read_with(cx, |h, _| h.inputs[0].clone());
    let second = harness.read_with(cx, |h, _| h.inputs[1].clone());
    cx.update(|window, cx| {
        window.activate_window();
        first.read(cx).focus(window);
    });
    cx.run_until_parked();
    cx.simulate_input("abc");
    cx.update(|window, cx| second.read(cx).focus(window));
    cx.run_until_parked();
    cx.simulate_input("xyz");
    harness.update(cx, |h, cx| {
        h.inputs.pop();
        h.initial.pop();
        cx.notify();
    });
    drop(second);
    cx.run_until_parked();
    cx.update(|window, cx| first.read(cx).focus(window));
    cx.run_until_parked();
    cx.simulate_keystrokes(keys("cmd-z", "ctrl-z"));
    assert_eq!(first.read_with(cx, |state, _| state.value().to_owned()), "");
    cx.update(|window, cx| {
        first.update(cx, |state, cx| {
            for _ in 0..1002 {
                state.replace_text_in_range(None, "x", window, cx);
            }
        });
    });
    cx.run_until_parked();
    cx.simulate_keystrokes(&vec![keys("cmd-z", "ctrl-z"); 1001].join(" "));
    assert_eq!(
        first.read_with(cx, |state, _| state.value().to_owned()),
        "xx"
    );
    cx.simulate_keystrokes(&vec![keys("cmd-shift-z", "ctrl-shift-z"); 1001].join(" "));
    assert_eq!(
        first.read_with(cx, |state, _| state.value().to_owned()),
        "x".repeat(1002)
    );
}

#[gpui::test]
fn history_is_scoped_to_its_window(root: &mut TestAppContext) {
    root.update(dazzui::init);
    let (first, first_window) = {
        let (harness, cx) = root.add_window_view(|window, cx| Harness {
            inputs: vec![cx.new(|cx| InputState::new(window, cx))],
            initial: vec![String::new()],
            controlled: false,
            multiline: false,
            masked: false,
            read_only: false,
            max_length: None,
        });
        cx.run_until_parked();
        let first = harness.read_with(cx, |h, _| h.inputs[0].clone());
        let window = cx.update(|window, cx| {
            window.activate_window();
            first.read(cx).focus(window);
            window.window_handle()
        });
        cx.run_until_parked();
        cx.simulate_input("first");
        (first, window)
    };
    {
        let (harness, cx) = root.add_window_view(|window, cx| Harness {
            inputs: vec![cx.new(|cx| InputState::new(window, cx))],
            initial: vec![String::new()],
            controlled: false,
            multiline: false,
            masked: false,
            read_only: false,
            max_length: None,
        });
        cx.run_until_parked();
        let second = harness.read_with(cx, |h, _| h.inputs[0].clone());
        cx.update(|window, cx| {
            window.activate_window();
            second.read(cx).focus(window)
        });
        cx.run_until_parked();
        cx.simulate_input("second");
        cx.simulate_keystrokes(keys("cmd-z", "ctrl-z"));
        assert_eq!(
            second.read_with(cx, |state, _| state.value().to_owned()),
            ""
        );
        assert_eq!(
            first.read_with(cx, |state, _| state.value().to_owned()),
            "first"
        );
    }
    root.simulate_keystrokes(first_window, keys("cmd-z", "ctrl-z"));
    assert_eq!(
        first.read_with(root, |state, _| state.value().to_owned()),
        ""
    );
}
