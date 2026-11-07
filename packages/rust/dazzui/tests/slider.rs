//! Pointer push, keyboard clamping and off-centre grabs follow Base UI's
//! default Slider. These gestures exercise the actual rendered controls.

use dazzui::Slider;
use gpui::{
    Context, Entity, InteractiveElement, IntoElement, Modifiers, MouseButton, ParentElement,
    Render, Styled, TestAppContext, VisualTestContext, Window, div, point, px,
};

struct Harness {
    values: Vec<f64>,
    controlled: bool,
    accept: bool,
    changes: Vec<Vec<f64>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        let mut slider = Slider::new("range")
            .default_value(self.values.clone())
            .on_value_change(move |values, _, cx| {
                let _ = view.update(cx, |view, cx| {
                    view.changes.push(values.to_vec());
                    if view.controlled && view.accept {
                        view.values = values.to_vec();
                        cx.notify();
                    }
                });
            });
        if self.controlled {
            slider = slider.value(self.values.clone());
        }
        div().size_full().p(px(40.)).child(
            div()
                .w(px(200.))
                .debug_selector(|| "range".into())
                .child(slider),
        )
    }
}

fn setup<'a>(
    cx: &'a mut TestAppContext,
    values: &[f64],
    controlled: bool,
    accept: bool,
) -> (Entity<Harness>, &'a mut VisualTestContext) {
    cx.update(dazzui::init);
    let values = values.to_vec();
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        values,
        controlled,
        accept,
        changes: Vec::new(),
    });
    cx.run_until_parked();
    (view, cx)
}

fn move_to(cx: &mut VisualTestContext, value: f32) {
    let bounds = cx.debug_bounds("range").unwrap();
    cx.simulate_mouse_move(
        point(
            bounds.left() + bounds.size.width * (value / 100.),
            bounds.center().y,
        ),
        Some(MouseButton::Left),
        Modifiers::none(),
    );
    cx.run_until_parked();
}

fn last(view: &Entity<Harness>, cx: &mut VisualTestContext) -> Vec<f64> {
    view.read_with(cx, |view, _| {
        view.changes.last().cloned().unwrap_or_default()
    })
}

#[gpui::test]
fn pointer_pushes_neighbours_but_keyboard_stops_at_them(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &[20., 40., 60.], false, true);
    let thumb = cx.debug_bounds("dz-slider-thumb-0").unwrap();
    cx.simulate_mouse_down(thumb.center(), MouseButton::Left, Modifiers::none());
    move_to(cx, 80.);
    assert_eq!(last(&view, cx), [80., 80., 80.]);
    move_to(cx, 30.);
    assert_eq!(last(&view, cx), [30., 80., 80.]);
    cx.simulate_mouse_up(thumb.center(), MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    cx.simulate_keystrokes("end right");
    assert_eq!(last(&view, cx), [80., 80., 80.]);
    cx.simulate_keystrokes("home");
    assert_eq!(last(&view, cx), [0., 80., 80.]);
}

#[gpui::test]
fn grabbing_the_edge_does_not_jump_to_the_pointer(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &[20., 80.], false, true);
    let thumb = cx.debug_bounds("dz-slider-thumb-0").unwrap();
    let press = point(thumb.center().x + px(5.), thumb.center().y);
    cx.simulate_mouse_down(press, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    assert!(view.read_with(cx, |view, _| view.changes.is_empty()));
    cx.simulate_mouse_move(
        point(press.x + px(20.), press.y),
        Some(MouseButton::Left),
        Modifiers::none(),
    );
    cx.run_until_parked();
    assert_eq!(last(&view, cx), [30., 80.]);
}

#[gpui::test]
fn controlled_refusal_keeps_the_rendered_values(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &[20., 40., 60.], true, false);
    let thumb = cx.debug_bounds("dz-slider-thumb-0").unwrap();
    cx.simulate_mouse_down(thumb.center(), MouseButton::Left, Modifiers::none());
    move_to(cx, 90.);
    assert_eq!(last(&view, cx), [90., 90., 90.]);
    assert_eq!(cx.debug_bounds("dz-slider-thumb-0").unwrap(), thumb);
}

#[gpui::test]
fn a_track_tie_chooses_the_later_thumb(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &[20., 80.], false, true);
    let bounds = cx.debug_bounds("range").unwrap();
    cx.simulate_click(bounds.center(), Modifiers::none());
    cx.run_until_parked();
    assert_eq!(last(&view, cx), [20., 50.]);
}

#[gpui::test]
fn overlapping_thumbs_at_the_maximum_can_separate(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &[100., 100.], false, true);
    let thumb = cx.debug_bounds("dz-slider-thumb-1").unwrap();
    cx.simulate_mouse_down(thumb.center(), MouseButton::Left, Modifiers::none());
    move_to(cx, 90.);
    assert_eq!(last(&view, cx), [90., 100.]);
}

#[gpui::test]
fn a_range_with_no_values_has_no_pointer_target(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &[], false, true);
    let bounds = cx.debug_bounds("range").unwrap();
    cx.simulate_click(bounds.center(), Modifiers::none());
    cx.run_until_parked();
    assert!(view.read_with(cx, |view, _| view.changes.is_empty()));
}

#[gpui::test]
fn handled_arrows_restore_keyboard_focus_visibility_after_a_pointer_press(cx: &mut TestAppContext) {
    let (_, cx) = setup(cx, &[20.], false, true);
    let thumb = cx.debug_bounds("dz-slider-thumb-0").unwrap();
    cx.simulate_click(thumb.center(), Modifiers::none());
    cx.run_until_parked();
    let focus = cx.update(|window, cx| window.focused(cx).unwrap());
    assert!(!cx.update(|window, cx| dazzui::foundation::focus_visible(&focus, window, cx)));
    cx.simulate_keystrokes("right");
    assert!(cx.update(|window, cx| dazzui::foundation::focus_visible(&focus, window, cx)));
}
