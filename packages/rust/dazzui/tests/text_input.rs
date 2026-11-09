//! The editing every field is built on, driven by keystrokes.

use std::cell::RefCell;
use std::rc::Rc;

mod support;
use support::keys;

use dazzui::foundation::text_input::{InputState, TextInput};
use gpui::{
    AppContext, ClipboardItem, Context, Entity, EntityInputHandler, IntoElement, Modifiers,
    MouseButton, MouseDownEvent, MouseUpEvent, ObservedPaint, ParentElement, Render, Styled,
    TestAppContext, VisualTestContext, Window, div, point, prelude::FluentBuilder, px,
};
use support::paint::PaintProbe;

struct Harness {
    input: Entity<InputState>,
    multiline: bool,
    centered: bool,
    offset_y: f32,
    masked: bool,
    read_only: bool,
    max_length: Option<usize>,
    line_height: Option<f32>,
    changes: Rc<RefCell<Vec<String>>>,
    submits: Rc<RefCell<Vec<String>>>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let changes = self.changes.clone();
        let submits = self.submits.clone();
        let mut input = TextInput::new("input")
            .state(self.input.clone())
            .placeholder(if self.centered { "" } else { "Type here" })
            .multiline(self.multiline)
            .centered(self.centered)
            .masked(self.masked)
            .read_only(self.read_only)
            .on_change(move |text, _, _| changes.borrow_mut().push(text.to_string()))
            .on_submit(move |text, _, _| submits.borrow_mut().push(text.to_string()));
        if let Some(max) = self.max_length {
            input = input.max_length(max);
        }
        div()
            .w(px(if self.centered { 19. } else { 240. }))
            .pt(px(self.offset_y))
            .text_size(px(13.))
            .when_some(self.line_height, |this, height| {
                this.line_height(px(height))
            })
            .child(PaintProbe {
                child: input.into_any_element(),
                paints: self.paints.clone(),
            })
    }
}

struct Setup {
    multiline: bool,
    centered: bool,
    offset_y: f32,
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
        centered: setup.centered,
        offset_y: setup.offset_y,
        masked: setup.masked,
        read_only: false,
        max_length: setup.max_length,
        line_height: setup.line_height,
        changes: Rc::default(),
        submits: Rc::default(),
        paints: Rc::default(),
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
        centered: false,
        offset_y: 0.,
        masked: false,
        max_length: None,
        line_height: None,
    }
}

fn value(input: &Entity<InputState>, cx: &mut VisualTestContext) -> String {
    input.read_with(cx, |input, _| input.value().to_string())
}

fn set_selection(
    input: &Entity<InputState>,
    text: &str,
    range: std::ops::Range<usize>,
    cx: &mut VisualTestContext,
) {
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input.set_value("", cx);
            input.replace_and_mark_text_in_range(Some(0..0), text, Some(range), window, cx);
            input.unmark_text(window, cx);
        })
    });
    cx.run_until_parked();
}

#[gpui::test]
fn editing_boundaries_match_browser_grapheme_and_backspace_traces(cx: &mut TestAppContext) {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../js/dazzui/example/scripts/text-field-graphemes.cases.json"
    ))
    .unwrap();
    for setup in [
        single(),
        Setup {
            multiline: true,
            ..single()
        },
        Setup {
            masked: true,
            ..single()
        },
    ] {
        let (_, input, cx) = open(cx, setup);
        for fixture in fixtures["cases"].as_array().unwrap() {
            let cluster = fixture["cluster"].as_str().unwrap();
            let text = format!("X{cluster}Y");
            let select = |offset: usize, cx: &mut VisualTestContext| {
                set_selection(&input, &text, offset..offset, cx);
            };
            let at = |key: &str, expected: &serde_json::Value, cx: &mut VisualTestContext| {
                cx.simulate_keystrokes(key);
                let actual = cx.update(|window, cx| {
                    input.update(cx, |input, cx| {
                        let selection = input.selected_text_range(false, window, cx).unwrap();
                        serde_json::json!({
                            "value": input.value(),
                            "start": selection.range.start,
                            "end": selection.range.end,
                            "reversed": selection.reversed,
                        })
                    })
                });
                assert_eq!(&actual, expected, "{} / {key}", fixture["id"]);
            };
            select(0, cx);
            for expected in fixture["right"].as_array().unwrap() {
                at("right", expected, cx);
            }
            select(text.encode_utf16().count(), cx);
            for expected in fixture["left"].as_array().unwrap() {
                at("left", expected, cx);
            }
            select(1 + cluster.encode_utf16().count(), cx);
            for expected in fixture["backspace"].as_array().unwrap() {
                at("backspace", expected, cx);
            }
            select(1, cx);
            at("delete", &fixture["forward"], cx);
            select(1 + cluster.encode_utf16().count(), cx);
            at("shift-left", &fixture["selection"], cx);
            select(1, cx);
            at("shift-right", &fixture["selectionRight"], cx);
        }
        for fixture in fixtures["rangeCases"].as_array().unwrap() {
            let range = fixture["selection"][0].as_u64().unwrap() as usize
                ..fixture["selection"][1].as_u64().unwrap() as usize;
            set_selection(&input, fixture["text"].as_str().unwrap(), range, cx);
            cx.simulate_keystrokes(&fixture["key"].as_str().unwrap().to_ascii_lowercase());
            let actual = cx.update(|window, cx| input.update(cx, |input, cx| {
                let range = input.selected_text_range(false, window, cx).unwrap();
                serde_json::json!({"value":input.value(),"start":range.range.start,"end":range.range.end,"reversed":range.reversed})
            }));
            assert_eq!(actual, fixture["expected"], "{}", fixture["id"]);
        }
    }
}

#[gpui::test]
fn read_only_deletion_preserves_value_and_selection(cx: &mut TestAppContext) {
    let (view, input, cx) = open(cx, single());
    for range in [3..3, 1..2] {
        view.update(cx, |view, cx| {
            view.read_only = false;
            cx.notify();
        });
        cx.run_until_parked();
        set_selection(&input, "Xe\u{0301}Y", range.clone(), cx);
        view.update(cx, |view, cx| {
            view.read_only = true;
            cx.notify();
        });
        cx.run_until_parked();
        for key in ["backspace", "delete"] {
            cx.simulate_keystrokes(key);
            let actual = cx.update(|window, cx| {
                input.update(cx, |input, cx| {
                    input.selected_text_range(false, window, cx).unwrap().range
                })
            });
            assert_eq!(actual, range);
            assert_eq!(value(&input, cx), "Xe\u{0301}Y");
        }
    }
}

#[gpui::test]
fn passwords_map_graphemes_to_one_mask_and_keep_original_input_offsets(cx: &mut TestAppContext) {
    let (_, input, cx) = open(
        cx,
        Setup {
            masked: true,
            ..single()
        },
    );
    let bounds = |offset: usize, cx: &mut VisualTestContext| {
        cx.update(|window, cx| {
            input.update(cx, |input, cx| {
                input
                    .bounds_for_range(offset..offset, Default::default(), window, cx)
                    .unwrap()
            })
        })
    };
    for (cluster, masks) in [
        ("e\u{0301}", 1),
        ("👨‍👩‍👧‍👦", 1),
        ("👩🏽‍💻", 1),
        ("क्‍षि", 1),
        ("a‍👨", 2),
    ] {
        let ascii = "x".repeat(masks + 2);
        input.update(cx, |input, cx| input.set_value(ascii, cx));
        cx.run_until_parked();
        let expected = bounds(1 + masks, cx);
        let text = format!("X{cluster}Y");
        input.update(cx, |input, cx| input.set_value(text, cx));
        cx.run_until_parked();
        let end = 1 + cluster.encode_utf16().count();
        assert_eq!(bounds(end, cx), expected, "{cluster}");
        let actual = cx.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.character_index_for_point(expected.origin, window, cx)
            })
        });
        assert_eq!(actual, Some(end), "{cluster}");
        if cluster == "क्‍षि" {
            // Blink permits this tailored editing stop, while its Unicode
            // password masking keeps the whole conjunct under one bullet.
            assert_eq!(bounds(4, cx), expected);
        }
    }
}

#[gpui::test]
fn multiline_ime_rectangles_keep_fractional_leading(cx: &mut TestAppContext) {
    let (harness, input, cx) = open(
        cx,
        Setup {
            multiline: true,
            line_height: Some(20.8),
            offset_y: 0.5,
            ..single()
        },
    );
    cx.simulate_input("a\nb\nc");
    cx.simulate_keystrokes("left left left");
    cx.run_until_parked();
    let (first, second, cursor) = cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            (
                input
                    .bounds_for_range(0..1, Default::default(), window, cx)
                    .unwrap(),
                input
                    .bounds_for_range(2..3, Default::default(), window, cx)
                    .unwrap(),
                input
                    .bounds_for_range(2..2, Default::default(), window, cx)
                    .unwrap(),
            )
        })
    });
    assert_eq!(second.origin.y - first.origin.y, px(20.796875));
    // NoopTextSystem supplies a 17px content box at 13px, independent of
    // leading. Only the collapsed caret snaps to a whole logical pixel.
    assert_eq!(second.size.height, px(17.));
    assert_eq!(cursor.origin.y, second.origin.y.round());
    assert_eq!(cursor.size.height, second.size.height);
    let paint = harness.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone());
    let quad = paint
        .quads
        .iter()
        .find(|quad| quad.bounds.size.width == px(1.))
        .unwrap();
    assert_eq!(quad.bounds.origin, cursor.origin);
    assert_eq!(quad.bounds.size.height, cursor.size.height);
}

#[gpui::test]
fn centered_caret_and_ime_rectangles_snap_an_odd_width(cx: &mut TestAppContext) {
    let (harness, input, cx) = open(
        cx,
        Setup {
            centered: true,
            ..single()
        },
    );
    let caret = cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input
                .bounds_for_range(0..0, Default::default(), window, cx)
                .unwrap()
        })
    });
    assert_eq!(caret.origin.x, px(10.));
    let paint = harness.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone());
    let quad = paint
        .quads
        .iter()
        .find(|quad| quad.bounds.size.width == px(1.))
        .unwrap();
    assert_eq!(quad.bounds.origin, caret.origin);
}

#[gpui::test]
fn typed_caret_and_ime_snap_without_moving_the_shaped_text(cx: &mut TestAppContext) {
    let (harness, input, cx) = open(cx, single());
    cx.simulate_input("a");
    let caret = cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input
                .bounds_for_range(1..1, Default::default(), window, cx)
                .unwrap()
        })
    });
    // The synthetic glyph advances 7.8px at 13px; the caret uses its pixel.
    assert_eq!(caret.origin.x, px(7.));
    let paint = harness.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone());
    let quad = paint
        .quads
        .iter()
        .find(|quad| quad.bounds.size.width == px(1.))
        .unwrap();
    assert_eq!(quad.bounds.origin, caret.origin);
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
    assert_eq!(
        input.read_with(cx, |input, _| input.selected_range()),
        4..if cfg!(target_os = "windows") { 8 } else { 7 }
    );
    cx.simulate_input("2");
    assert_eq!(
        value(&input, cx),
        if cfg!(target_os = "windows") {
            "one 2three"
        } else {
            "one 2 three"
        }
    );

    cx.simulate_keystrokes(keys("cmd-a", "ctrl-a"));
    cx.simulate_input("x");
    assert_eq!(value(&input, cx), "x");

    cx.simulate_input(" y z");
    cx.simulate_keystrokes(keys("alt-backspace", "ctrl-backspace"));
    assert_eq!(value(&input, cx), "x y ");
    cx.simulate_keystrokes(keys("cmd-backspace", "shift-home backspace"));
    assert_eq!(value(&input, cx), "");
}

#[cfg(not(target_os = "windows"))]
#[gpui::test]
fn word_editing_matches_browser_unicode_traces(cx: &mut TestAppContext) {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../js/dazzui/example/scripts/text-field-words.cases.json"
    ))
    .unwrap();
    for setup in [
        single(),
        Setup {
            multiline: true,
            ..single()
        },
    ] {
        let multiline = setup.multiline;
        let (_, input, cx) = open(cx, setup);
        for fixture in fixtures["cases"].as_array().unwrap() {
            if fixture["multiline"] == true && !multiline {
                continue;
            }
            let text = fixture["text"].as_str().unwrap();
            for check in fixture["checks"].as_array().unwrap() {
                let at = check[0].as_u64().unwrap() as usize;
                for (index, key) in [
                    keys("alt-left", "ctrl-left"),
                    keys("alt-right", "ctrl-right"),
                    keys("alt-shift-left", "ctrl-shift-left"),
                    keys("alt-shift-right", "ctrl-shift-right"),
                    keys("alt-backspace", "ctrl-backspace"),
                    keys("alt-delete", "ctrl-delete"),
                ]
                .into_iter()
                .enumerate()
                {
                    set_selection(&input, text, at..at, cx);
                    cx.simulate_keystrokes(key);
                    let actual = cx.update(|window, cx| {
                        input.update(cx, |input, cx| {
                            let selection = input.selected_text_range(false, window, cx).unwrap();
                            serde_json::json!([
                                input.value(),
                                selection.range.start,
                                selection.range.end,
                                selection.reversed,
                            ])
                        })
                    });
                    let expected = if index < 4 {
                        let stop = check[index + 1].as_u64().unwrap() as usize;
                        if index < 2 {
                            serde_json::json!([text, stop, stop, false])
                        } else {
                            serde_json::json!([text, at.min(stop), at.max(stop), stop < at])
                        }
                    } else {
                        let deleted = &check[index + 1];
                        serde_json::json!([deleted[0], deleted[1], deleted[1], false])
                    };
                    assert_eq!(actual, expected, "{}/{at}/{key}", fixture["id"]);
                }
            }
        }
    }
}

#[gpui::test]
fn double_click_drags_whole_words_and_preserves_platform_direction(cx: &mut TestAppContext) {
    let (_, input, cx) = open(cx, single());
    let text = "one two three four";
    set_selection(&input, text, 0..0, cx);
    let positions: Vec<_> = (0..=text.len())
        .map(|at| {
            cx.update(|window, cx| {
                input.update(cx, |input, cx| {
                    input
                        .bounds_for_range(at..at, Default::default(), window, cx)
                        .unwrap()
                        .center()
                })
            })
        })
        .collect();
    let at = |index: usize| {
        point(
            positions[index].x + (positions[index + 1].x - positions[index].x) / 4.,
            positions[index].y,
        )
    };
    cx.simulate_event(MouseDownEvent {
        position: at(5),
        button: MouseButton::Left,
        click_count: 2,
        ..Default::default()
    });
    assert_eq!(input.read_with(cx, |input, _| input.selected_range()), 4..7);
    for (index, expected, reversed) in [
        (11, 4..13, false),
        (1, 0..7, !cfg!(target_os = "macos")),
        (5, 4..7, !cfg!(target_os = "macos")),
    ] {
        cx.simulate_mouse_move(at(index), Some(MouseButton::Left), Modifiers::none());
        let selected = cx.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.selected_text_range(false, window, cx).unwrap()
            })
        });
        assert_eq!(selected.range, expected);
        assert_eq!(selected.reversed, reversed);
    }
    cx.simulate_event(MouseUpEvent {
        position: at(5),
        button: MouseButton::Left,
        click_count: 2,
        ..Default::default()
    });
    cx.simulate_keystrokes("shift-right");
    assert_eq!(
        input.read_with(cx, |input, _| input.selected_range()),
        if cfg!(target_os = "macos") {
            4..8
        } else {
            5..7
        },
    );
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
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../js/dazzui/example/scripts/text-field-graphemes.cases.json"
    ))
    .unwrap();
    for fixture in fixtures["maxLengthCases"].as_array().unwrap() {
        for (multiline, masked) in [(false, false), (true, false), (false, true)] {
            let (_, input, cx) = open(
                cx,
                Setup {
                    max_length: Some(fixture["limit"].as_u64().unwrap() as usize),
                    multiline,
                    masked,
                    ..single()
                },
            );
            // One insertText operation, rather than separate keystrokes.
            cx.update(|window, cx| {
                input.update(cx, |input, cx| {
                    input.replace_text_in_range(
                        None,
                        fixture["input"].as_str().unwrap(),
                        window,
                        cx,
                    )
                })
            });
            let actual = cx.update(|window, cx| input.update(cx, |input, cx| {
                let selection = input.selected_text_range(false, window, cx).unwrap();
                serde_json::json!({"value":input.value(),"start":selection.range.start,"end":selection.range.end,"reversed":selection.reversed})
            }));
            assert_eq!(actual, fixture["expected"], "{fixture}");
            cx.simulate_keystrokes("backspace");
            let actual = cx.update(|window, cx| input.update(cx, |input, cx| {
                let selection = input.selected_text_range(false, window, cx).unwrap();
                serde_json::json!({"value":input.value(),"start":selection.range.start,"end":selection.range.end,"reversed":selection.reversed})
            }));
            assert_eq!(actual, fixture["afterBackspace"], "backspace / {fixture}");
        }
    }
    for masked in [false, true] {
        let (_, input, cx) = open(
            cx,
            Setup {
                max_length: Some(3),
                masked,
                ..single()
            },
        );
        set_selection(&input, "A😀", 1..3, cx);
        cx.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.replace_text_in_range(None, "BCDE", window, cx)
            })
        });
        assert_eq!(value(&input, cx), if masked { "AB" } else { "ABC" });
    }
}
