//! Shared field chrome animates without changing editing geometry or ink timing.

#[allow(dead_code)]
mod support;

use dazzui::foundation::text_input::{InputState, input_state};
use dazzui::{
    Autocomplete, Button, Combobox, DateRangePicker, FieldState, Motion, MotionPreference,
    NumberField, OtpField, SearchField, Select, SelectOption, TextField, Theme,
};
use gpui::{
    AnyElement, Context, ElementId, Entity, EntityInputHandler, IntoElement, ObservedPaint,
    ParentElement, Render, Styled, TestAppContext, VisualTestContext, Window, div, px,
};
use std::{cell::RefCell, rc::Rc, time::Duration};
use support::paint::PaintProbe;

struct Harness {
    kind: usize,
    error: bool,
    value: String,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
    input: Option<Entity<InputState>>,
}

impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = if self.error {
            FieldState::Error
        } else {
            FieldState::Default
        };
        self.input = match self.kind {
            0 | 1 | 2 => Some(input_state("tested", window, cx)),
            4 | 5 => Some(input_state(
                ElementId::NamedChild(Box::new("tested".into()), "text".into()),
                window,
                cx,
            )),
            7 => Some(input_state(
                ElementId::NamedChild(Box::new("tested".into()), "cell-0".into()),
                window,
                cx,
            )),
            _ => None,
        };
        let field: AnyElement = match self.kind {
            0 => TextField::new("tested")
                .default_value("Value")
                .state(state)
                .into_any_element(),
            1 => {
                let view = cx.entity().downgrade();
                SearchField::new("tested", self.value.clone())
                    .on_value_change(move |value, _, cx| {
                        let _ = view.update(cx, |view, cx| {
                            view.value = value.into();
                            cx.notify();
                        });
                    })
                    .into_any_element()
            }
            2 => NumberField::new("tested")
                .default_value(42.)
                .state(state)
                .into_any_element(),
            3 => Select::new("tested")
                .options([SelectOption::new("a", "Alpha")])
                .default_value("a")
                .invalid(self.error)
                .into_any_element(),
            4 => Combobox::new("tested")
                .options([SelectOption::new("a", "Alpha")])
                .default_value("a")
                .state(state)
                .into_any_element(),
            5 => Autocomplete::new("tested")
                .default_value("Value")
                .state(state)
                .into_any_element(),
            6 => DateRangePicker::new("tested")
                .state(state)
                .into_any_element(),
            _ => OtpField::new("tested", 6).state(state).into_any_element(),
        };
        div()
            .size_full()
            .p(px(24.))
            .child(div().w(px(320.)).child(PaintProbe {
                child: field,
                paints: self.paints.clone(),
            }))
            .child(Button::new("next").child("Next"))
    }
}

fn setup<'a>(
    cx: &'a mut TestAppContext,
    theme: &str,
    kind: usize,
) -> (Entity<Harness>, &'a mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set(theme, cx);
        Motion::set_preference(MotionPreference::Full, cx);
        Motion::seek(Duration::ZERO, cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        kind,
        error: false,
        value: "Value".into(),
        paints: Rc::new(RefCell::new(Vec::new())),
        input: None,
    });
    cx.run_until_parked();
    (view, cx)
}

fn sample(view: &Entity<Harness>, cx: &mut VisualTestContext) -> ObservedPaint {
    view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone())
}

fn seek(cx: &mut VisualTestContext, ms: u64) {
    cx.update(|_, cx| Motion::seek(Duration::from_millis(ms), cx));
    cx.run_until_parked();
}

fn caret(view: &Entity<Harness>, cx: &mut VisualTestContext) -> Option<gpui::Bounds<gpui::Pixels>> {
    let input = view.read_with(cx, |view, _| view.input.clone());
    input.and_then(|input| {
        cx.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.bounds_for_range(0..0, Default::default(), window, cx)
            })
        })
    })
}

fn assert_bytes(actual: gpui::Hsla, expected: u32) {
    let color = gpui::Rgba::from(actual);
    for (actual, expected) in [color.r, color.g, color.b, color.a]
        .into_iter()
        .zip(expected.to_be_bytes())
    {
        assert!(
            (actual * 255. - expected as f32).abs() < 1.1,
            "{} vs {expected}",
            actual * 255.
        );
    }
}

#[gpui::test]
fn shared_field_focus_transitions_keep_text_and_caret_geometry(cx: &mut TestAppContext) {
    for kind in 0..8 {
        let (view, cx) = setup(cx, "studio-light", kind);
        let rest = sample(&view, cx).decorated_surface.unwrap();
        cx.simulate_keystrokes("tab");
        let initial_caret = caret(&view, cx);
        let first = sample(&view, cx);
        assert_eq!(
            first.decorated_surface.as_ref().unwrap().style.border_color,
            rest.style.border_color,
            "field {kind}: first focus frame must retain the previous border"
        );
        seek(cx, 75);
        let middle = sample(&view, cx).decorated_surface.unwrap();
        assert_eq!(
            middle.bounds, rest.bounds,
            "field {kind}: focus must not change layout"
        );
        assert!(
            !middle.style.box_shadow.is_empty(),
            "field {kind}: focus glow is missing"
        );
        assert!((f32::from(middle.style.box_shadow[0].spread_radius) - 2.326683).abs() < 0.001);
        assert_eq!(
            caret(&view, cx),
            initial_caret,
            "field {kind}: caret moved during focus"
        );
        seek(cx, 150);
        let end = sample(&view, cx).decorated_surface.unwrap();
        assert_eq!(end.bounds, rest.bounds);
        assert_eq!(end.style.box_shadow[0].spread_radius, px(3.));
    }
}

#[gpui::test]
fn focused_field_error_retargets_the_whole_shadow_and_changes_ink_immediately(
    cx: &mut TestAppContext,
) {
    for (theme, focus_border, focus_fill, error_border, error_fill) in [
        (
            "bright-light",
            0x111c2ecb,
            0xfcfbf9ff,
            0xb1393df3,
            0xfdf4f4ff,
        ),
        (
            "studio-dark",
            0x6f52ffcb,
            0x0e1015ff,
            0xc34364f3,
            0xc7bfc0ff,
        ),
    ] {
        let (view, cx) = setup(cx, theme, 0);
        cx.simulate_keystrokes("tab");
        let initial_caret = caret(&view, cx);
        seek(cx, 75);
        let before = sample(&view, cx);
        let surface = before.decorated_surface.as_ref().unwrap();
        assert_bytes(surface.style.border_color.unwrap(), focus_border);
        assert_bytes(
            surface
                .style
                .background
                .as_ref()
                .unwrap()
                .color()
                .unwrap()
                .solid_color()
                .unwrap(),
            focus_fill,
        );
        view.update(cx, |view, cx| {
            view.error = true;
            cx.notify();
        });
        cx.run_until_parked();
        let first = sample(&view, cx);
        assert_eq!(
            first.decorated_surface.as_ref().unwrap().style.box_shadow,
            surface.style.box_shadow
        );
        assert_bytes(first.text.as_ref().unwrap().color, 0x7e1f1cff);
        seek(cx, 150);
        let middle = sample(&view, cx).decorated_surface.unwrap();
        assert_bytes(middle.style.border_color.unwrap(), error_border);
        assert_bytes(
            middle
                .style
                .background
                .as_ref()
                .unwrap()
                .color()
                .unwrap()
                .solid_color()
                .unwrap(),
            error_fill,
        );
        assert!(
            (f32::from(middle.style.box_shadow[0].spread_radius) - 2.848884).abs() < 0.001,
            "a tint change restarts the shadow property, including its spread"
        );
        assert_eq!(caret(&view, cx), initial_caret);
    }
}

#[gpui::test]
fn search_clear_focus_retains_the_field_glow_until_the_control_leaves(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, "studio-light", 1);
    cx.simulate_keystrokes("tab");
    seek(cx, 150);
    cx.simulate_keystrokes("tab");
    seek(cx, 300);
    assert_eq!(
        sample(&view, cx)
            .decorated_surface
            .unwrap()
            .style
            .box_shadow[0]
            .spread_radius,
        px(3.)
    );
    cx.simulate_keystrokes("enter");
    seek(cx, 450);
    assert!(
        sample(&view, cx)
            .decorated_surface
            .unwrap()
            .style
            .box_shadow
            .is_empty()
    );
}

#[gpui::test]
fn invalid_otp_focus_uses_the_danger_glow(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, "bright-light", 7);
    view.update(cx, |view, cx| {
        view.error = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("tab");
    seek(cx, 75);
    let field = sample(&view, cx).decorated_surface.unwrap();
    assert_bytes(field.style.box_shadow[0].color, 0xd640401c);
    assert_bytes(field.style.text.color.unwrap(), 0x7e1f1cff);
}
