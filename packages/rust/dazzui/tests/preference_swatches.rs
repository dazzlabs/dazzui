//! Swatches retain each preview's palette while selection belongs to the host.
#[allow(dead_code)]
mod support;
use dazzui::{PreferenceRow, PreferenceThemeOption, PreferenceThemeSwatches, Theme};
use gpui::{
    Context, Entity, FocusHandle, InteractiveElement, IntoElement, KeyUpEvent, Keystroke,
    Modifiers, MouseButton, ObservedPaint, ObservedSurface, ParentElement, Render, Styled,
    TestAppContext, VisualTestContext, Window, div, px,
};
use std::{cell::RefCell, rc::Rc};
use support::paint::PaintProbe;
fn options() -> Vec<PreferenceThemeOption> {
    ["bright", "studio", "frost", "graphite", "ember", "nocturne"]
        .into_iter()
        .map(|name| {
            let option = PreferenceThemeOption::new(format!("{name}-light"), name.to_string());
            if name == "bright" {
                option.theme("")
            } else {
                option
            }
        })
        .collect()
}
struct Harness {
    width: f32,
    row: bool,
    value: Option<String>,
    default: Option<String>,
    changes: Vec<String>,
    special: bool,
    next: FocusHandle,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.paints.borrow_mut().clear();
        let choices = if self.special {
            vec![
                PreferenceThemeOption::new("root", "Root").theme(""),
                PreferenceThemeOption::new("bright-light", "Inherited Bright name"),
                PreferenceThemeOption::new("unknown-theme", "Inherited unknown"),
                PreferenceThemeOption::new("alias", "Alias").theme("studio-light"),
            ]
        } else {
            options()
        };
        let owner = cx.entity().downgrade();
        let mut swatches = PreferenceThemeSwatches::new("swatches", choices).on_value_change(
            move |value, _, cx| {
                let _ = owner.update(cx, |view, _| view.changes.push(value.to_string()));
            },
        );
        if let Some(value) = &self.value {
            swatches = swatches.value(value.clone());
        }
        if let Some(value) = &self.default {
            swatches = swatches.default_value(value.clone());
        }
        let content = if self.row {
            PreferenceRow::new("row", "Theme")
                .child(swatches)
                .into_any_element()
        } else {
            swatches.into_any_element()
        };
        div()
            .size_full()
            .p(px(24.))
            .child(div().w(px(self.width)).child(PaintProbe {
                child: content,
                paints: self.paints.clone(),
            }))
            .child(div().id("next").track_focus(&self.next).child("Next"))
    }
}
fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-dark", cx);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        width: 320.,
        row: false,
        value: None,
        default: None,
        changes: vec![],
        special: false,
        next: cx.focus_handle().tab_stop(true),
        paints: Rc::default(),
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    (view, cx)
}
fn paint(view: &Entity<Harness>, cx: &VisualTestContext) -> ObservedPaint {
    view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone())
}
fn windows(paint: &ObservedPaint) -> Vec<&ObservedSurface> {
    paint
        .surfaces
        .iter()
        .filter(|surface| {
            surface.style.background.is_some()
                && surface.style.border_widths.left.to_pixels(surface.rem_size) > px(0.)
                && surface
                    .style
                    .border_widths
                    .right
                    .to_pixels(surface.rem_size)
                    > px(0.)
        })
        .collect()
}
fn selected(paint: &ObservedPaint) -> Vec<gpui::Bounds<gpui::Pixels>> {
    paint
        .surfaces
        .iter()
        .filter(|surface| surface.style.box_shadow.len() == 2)
        .map(|surface| surface.bounds)
        .collect()
}
#[gpui::test]
fn every_swatch_is_a_tab_stop_and_native_activation_always_reports_the_value(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    assert!(selected(&paint(&view, cx)).is_empty());
    cx.simulate_keystrokes("tab right enter");
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        ["bright-light"]
    );
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.len()),
        2,
        "activating the selected button still reports"
    );
    cx.simulate_keystrokes("tab space");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.changes.len()), 2);
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse("space").unwrap(),
    });
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.last().cloned()),
        Some("studio-light".into())
    );
    for name in [
        "frost-light",
        "graphite-light",
        "ember-light",
        "nocturne-light",
    ] {
        cx.simulate_keystrokes("tab enter");
        cx.run_until_parked();
        assert_eq!(
            view.read_with(cx, |view, _| view.changes.last().cloned()),
            Some(name.into())
        );
    }
    cx.simulate_keystrokes("tab");
    cx.run_until_parked();
    let next = view.read_with(cx, |view, _| view.next.clone());
    assert!(cx.update(|window, _| next.is_focused(window)));
}
#[gpui::test]
fn controlled_rejection_retains_the_internal_choice_for_later_uncontrolled_use(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.value = Some("bright-light".into());
        cx.notify();
    });
    cx.run_until_parked();
    let initial = paint(&view, cx);
    let boxes = windows(&initial);
    let at = boxes[1].bounds.center();
    cx.simulate_mouse_down(at, MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_up(at, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    assert_eq!(selected(&paint(&view, cx)), vec![boxes[0].bounds]);
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        ["studio-light"]
    );
    view.update(cx, |view, cx| {
        view.value = None;
        cx.notify();
    });
    cx.run_until_parked();
    assert_eq!(selected(&paint(&view, cx)), vec![boxes[1].bounds]);
}
#[gpui::test]
fn previews_keep_own_colours_but_empty_and_unknown_theme_names_inherit_metrics(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    for host in ["bright-light", "studio-dark", "macos27-dark"] {
        cx.update(|_, cx| Theme::set(host, cx));
        cx.run_until_parked();
        let painted = paint(&view, cx);
        let previews = windows(&painted);
        assert_eq!(previews.len(), 6);
        for (index, name) in [
            "bright-light",
            "studio-light",
            "frost-light",
            "graphite-light",
            "ember-light",
            "nocturne-light",
        ]
        .into_iter()
        .enumerate()
        {
            let palette = Theme::named(name).unwrap().tokens;
            let metrics = Theme::named(host).unwrap().tokens;
            assert_eq!(
                previews[index].bounds.size.height,
                px(metrics.control_small_size)
            );
            assert_eq!(
                previews[index]
                    .style
                    .background
                    .as_ref()
                    .unwrap()
                    .color()
                    .unwrap()
                    .solid_color(),
                Some(palette.color_surface.hsla())
            );
            assert!(
                painted.surfaces.iter().any(|part| part.bounds.origin.x
                    >= previews[index].bounds.left()
                    && part.bounds.right() <= previews[index].bounds.right()
                    && part
                        .style
                        .background
                        .as_ref()
                        .and_then(|fill| fill.color())
                        .and_then(|color| color.solid_color())
                        == Some(palette.color_primary.shade(500).hsla())),
                "accent {name} in {host}"
            );
        }
    }
    view.update(cx, |view, cx| {
        view.special = true;
        cx.notify();
    });
    cx.run_until_parked();
    let painted = paint(&view, cx);
    let previews = windows(&painted);
    assert_eq!(previews.len(), 4);
    let host = Theme::named("macos27-dark").unwrap().tokens;
    for index in 0..3 {
        assert_eq!(
            previews[index].bounds.size.height,
            px(host.control_small_size)
        );
    }
    for index in [1, 2] {
        assert_eq!(
            previews[index]
                .style
                .background
                .as_ref()
                .unwrap()
                .color()
                .unwrap()
                .solid_color(),
            Some(host.color_surface.hsla())
        );
    }
}
#[gpui::test]
fn swatches_wrap_inside_a_preference_row_without_squeezing_its_label(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    for width in [320., 200., 140.] {
        view.update(cx, |view, cx| {
            view.row = true;
            view.width = width;
            cx.notify();
        });
        cx.run_until_parked();
        let painted = paint(&view, cx);
        let previews = windows(&painted);
        assert_eq!(previews.len(), 6);
        for preview in &previews {
            assert!(preview.bounds.right() <= px(24. + width + 0.5));
            assert!(preview.bounds.left() > px(24.));
        }
        if width < 300. {
            assert!(previews.last().unwrap().bounds.top() > previews[0].bounds.top());
        }
    }
}
