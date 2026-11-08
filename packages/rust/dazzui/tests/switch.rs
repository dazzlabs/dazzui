//! Switch colors animate while its layout and non-native activation follow React.

#[allow(dead_code)]
mod support;

use dazzui::{ActiveTheme, Button, Motion, MotionPreference, Switch, Theme};
use gpui::{
    Context, Entity, InteractiveElement, IntoElement, KeyUpEvent, Keystroke, ObservedPaint,
    ParentElement, Pixels, Render, Styled, TestAppContext, VisualTestContext, Window, div,
    prelude::*, px,
};
use std::{cell::RefCell, rc::Rc, time::Duration};
use support::paint::PaintProbe;

struct Harness {
    checked: bool,
    controlled: bool,
    disabled: bool,
    prevent_down: bool,
    prevent_up: bool,
    changes: Vec<bool>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        let switch = Switch::new("tested")
            .disabled(self.disabled)
            .when(self.controlled, |switch| switch.checked(self.checked))
            .on_checked_change(move |checked, _, cx| {
                let _ = view.update(cx, |view, cx| {
                    view.checked = checked;
                    view.changes.push(checked);
                    cx.notify();
                });
            });
        let prevent_down = self.prevent_down;
        let prevent_up = self.prevent_up;
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .p(px(24.))
            .capture_key_down(move |event, window, _| {
                if prevent_down && matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    window.prevent_default();
                }
            })
            .capture_key_up(move |event, window, _| {
                if prevent_up && event.keystroke.key == "space" {
                    window.prevent_default();
                }
            })
            .child(PaintProbe {
                child: switch.into_any_element(),
                paints: self.paints.clone(),
            })
            .child(Button::new("next").child("Next"))
    }
}

fn setup<'a>(
    cx: &'a mut TestAppContext,
    theme: &str,
    controlled: bool,
) -> (Entity<Harness>, &'a mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set(theme, cx);
        Motion::set_preference(MotionPreference::Full, cx);
        Motion::seek(Duration::ZERO, cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        checked: false,
        controlled,
        disabled: false,
        prevent_down: false,
        prevent_up: false,
        changes: vec![],
        paints: Rc::new(RefCell::new(Vec::new())),
    });
    cx.run_until_parked();
    (view, cx)
}

fn up(cx: &mut VisualTestContext, key: &str) {
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse(key).unwrap(),
    });
    cx.run_until_parked();
}

fn changes(view: &Entity<Harness>, cx: &mut VisualTestContext) -> Vec<bool> {
    view.read_with(cx, |view, _| view.changes.clone())
}

fn sample(view: &Entity<Harness>, cx: &mut VisualTestContext) -> ([gpui::Hsla; 2], Pixels) {
    let paint = view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone());
    let track = paint.surface.as_ref().unwrap();
    let thumb_size = cx.update(|_, cx| px(cx.theme().switch_medium_thumb));
    let thumb = paint
        .surfaces
        .iter()
        .find(|s| s.bounds.size.width == thumb_size)
        .unwrap();
    let fill = |surface: &gpui::ObservedSurface| {
        surface
            .style
            .background
            .as_ref()
            .unwrap()
            .color()
            .unwrap()
            .solid_color()
            .unwrap()
    };
    (
        [fill(track), fill(thumb)],
        thumb.bounds.left() - track.bounds.left(),
    )
}

fn assert_bytes(actual: [gpui::Hsla; 2], expected: [u32; 2]) {
    for (actual, expected) in actual.into_iter().zip(expected) {
        let color = gpui::Rgba::from(actual);
        for (actual, expected) in [color.r, color.g, color.b, color.a]
            .into_iter()
            .zip(expected.to_be_bytes())
        {
            assert!(
                (actual * 255. - expected as f32).abs() < 1.1,
                "painted channel {} vs {expected}",
                actual * 255.
            );
        }
    }
}

#[gpui::test]
fn switch_backgrounds_match_chromium_midpoints_without_moving_the_thumb(cx: &mut TestAppContext) {
    // Paused 75ms CSS transition samples from React's Switch preview.
    for (theme, middle, end) in [
        (
            "bright-light",
            [0x414955ff, 0xc2e83ddc],
            [0x111c2eff, 0xd6ff3fff],
        ),
        (
            "studio-dark",
            [0x6951d1ff, 0xdedfe6ff],
            [0x7c5cffff, 0xffffffff],
        ),
    ] {
        let (view, cx) = setup(cx, theme, true);
        let (off, offset) = sample(&view, cx);
        assert_eq!(offset, px(2.));
        view.update(cx, |view, cx| {
            view.checked = true;
            cx.notify();
        });
        cx.run_until_parked();
        let (first, on_offset) = sample(&view, cx);
        assert_eq!(
            first, off,
            "colors retain the previous paint at transition start"
        );
        let expected_offset = cx.update(|_, cx| {
            let t = cx.theme();
            px(t.switch_medium_width - t.spacing_0_5 - t.switch_medium_thumb)
        });
        assert_eq!(
            on_offset, expected_offset,
            "justify-content switches immediately in React"
        );
        cx.update(|_, cx| Motion::seek(Duration::from_millis(75), cx));
        cx.run_until_parked();
        let (middle_paint, offset) = sample(&view, cx);
        assert_bytes(middle_paint, middle);
        assert_eq!(offset, on_offset);
        cx.update(|_, cx| Motion::seek(Duration::from_millis(150), cx));
        cx.run_until_parked();
        assert_bytes(sample(&view, cx).0, end);
        view.update(cx, |view, cx| {
            view.checked = false;
            cx.notify();
        });
        cx.run_until_parked();
        assert_eq!(sample(&view, cx).1, px(2.));
        cx.update(|_, cx| Motion::seek(Duration::from_millis(225), cx));
        cx.run_until_parked();
        assert_ne!(sample(&view, cx).0, off);
        cx.update(|_, cx| Motion::set_preference(MotionPreference::Reduced, cx));
        cx.run_until_parked();
        assert_eq!(sample(&view, cx).0, off);
    }
}

#[gpui::test]
fn switch_keeps_base_uis_non_native_button_key_phases(cx: &mut TestAppContext) {
    for controlled in [false, true] {
        let (view, cx) = setup(cx, "studio-light", controlled);
        cx.simulate_keystrokes("tab");
        // Base UI's span has no keydown latch: an unpaired Space keyup
        // activates it, unlike the native button used for Toggle.
        up(cx, "space");
        assert_eq!(changes(&view, cx), [true]);
        cx.simulate_keystrokes("enter");
        assert_eq!(changes(&view, cx), [true, false]);
        cx.simulate_keystrokes("enter");
        up(cx, "enter");
        assert_eq!(changes(&view, cx), [true, false, true]);
        cx.simulate_keystrokes("space space");
        assert_eq!(changes(&view, cx), [true, false, true]);
        up(cx, "space");
        assert_eq!(changes(&view, cx), [true, false, true, false]);
    }
}

#[gpui::test]
fn switch_honors_disabled_and_prevented_events_with_non_native_space_semantics(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx, "studio-light", true);
    cx.simulate_keystrokes("tab");
    view.update(cx, |view, cx| {
        view.prevent_down = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("enter space");
    assert!(changes(&view, cx).is_empty());
    up(cx, "space");
    assert_eq!(
        changes(&view, cx),
        [true],
        "Base UI retains no prevented-keydown state"
    );
    view.update(cx, |view, cx| {
        view.prevent_down = false;
        view.prevent_up = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("space");
    up(cx, "space");
    assert_eq!(changes(&view, cx), [true]);
    view.update(cx, |view, cx| {
        view.prevent_up = false;
        view.disabled = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("enter space");
    up(cx, "space");
    assert_eq!(changes(&view, cx), [true]);
}
