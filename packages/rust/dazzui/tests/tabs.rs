//! Tabs use native buttons and retain the active label during width changes.

#[allow(dead_code)]
mod support;
use dazzui::{Icon, IconButton, Motion, MotionPreference, TabItem, TabStatus, Tabs, Theme, glyphs};
use gpui::{
    Context, Entity, IntoElement, KeyUpEvent, Keystroke, Modifiers, ObservedPaint, ObservedSurface,
    ParentElement, Render, Styled, TestAppContext, VisualTestContext, Window, div, prelude::*, px,
};
use std::{cell::RefCell, rc::Rc};
use support::paint::PaintProbe;

struct Harness {
    width: f32,
    value: String,
    accept: bool,
    documents: bool,
    close: bool,
    busy: bool,
    changes: Vec<String>,
    closed: Vec<String>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let select = cx.entity().downgrade();
        let close = select.clone();
        let items = ["Pull Requests", "Issues", "Actions", "Releases", "History"]
            .into_iter()
            .enumerate()
            .map(|(index, label)| {
                let item = TabItem::new(index.to_string(), label)
                    .icon(glyphs::SEARCH_16_REGULAR)
                    .disabled(index == 2)
                    .closable(self.documents)
                    .busy(self.busy);
                if self.documents {
                    item.status(TabStatus::Success)
                } else {
                    item
                }
            })
            .collect();
        let tabs = Tabs::new("tabs", items)
            .value(self.value.clone())
            .on_value_change(move |value, _, cx| {
                let _ = select.update(cx, |view, cx| {
                    view.changes.push(value.to_string());
                    if view.accept {
                        view.value = value.to_string();
                    }
                    cx.notify();
                });
            })
            .when(self.close, |tabs| {
                tabs.on_close(move |value, _, cx| {
                    let _ = close.update(cx, |view, cx| {
                        view.closed.push(value.to_string());
                        cx.notify();
                    });
                })
            })
            .when(self.documents, |tabs| {
                tabs.trailing(
                    IconButton::new("new", "New")
                        .child(Icon::new(glyphs::MORE_HORIZONTAL_REGULAR).size(14.)),
                )
            });
        div()
            .size_full()
            .p(px(24.))
            .child(div().flex().w(px(self.width)).h(px(35.)).child(PaintProbe {
                child: tabs.into_any_element(),
                paints: self.paints.clone(),
            }))
    }
}
fn setup(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
        Motion::set_preference(MotionPreference::Reduced, cx);
    });
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        width: 700.,
        value: "0".into(),
        accept: true,
        documents: false,
        close: true,
        busy: false,
        changes: vec![],
        closed: vec![],
        paints: Rc::new(RefCell::new(Vec::new())),
    });
    cx.run_until_parked();
    (view, cx)
}
fn paint(view: &Entity<Harness>, cx: &VisualTestContext) -> ObservedPaint {
    view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone())
}
fn tabs(paint: &ObservedPaint) -> Vec<&ObservedSurface> {
    paint
        .surfaces
        .iter()
        .filter(|surface| {
            surface.style.padding.left == px(10.).into() && surface.style.text.font_size.is_some()
        })
        .collect()
}
fn key(cx: &mut VisualTestContext, key: &str) {
    cx.simulate_keystrokes(key);
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse(key).unwrap(),
    });
    cx.run_until_parked();
}
#[gpui::test]
fn arrows_select_from_the_current_value_and_skip_disabled_tabs(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    assert_eq!(tabs(&paint(&view, cx)).len(), 5);
    cx.simulate_keystrokes("tab right right end home");
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        ["1", "3", "4", "0"]
    );
    view.update(cx, |view, cx| {
        view.accept = false;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("right right");
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.clone()),
        ["1", "3", "4", "0", "1", "1"]
    );
    key(cx, "enter");
    assert_eq!(
        view.read_with(cx, |view, _| view.changes.last().cloned()),
        Some("1".into())
    );
}
#[gpui::test]
fn narrowing_promotes_the_active_tab_and_restoring_width_restores_its_peers(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx);
    let initial = tabs(&paint(&view, cx))
        .iter()
        .map(|tab| tab.bounds.size.width)
        .collect::<Vec<_>>();
    view.update(cx, |view, cx| {
        view.value = "4".into();
        cx.notify();
    });
    cx.run_until_parked();
    let active_width = tabs(&paint(&view, cx))[4].bounds.size.width;
    view.update(cx, |view, cx| {
        view.width = 180.;
        cx.notify();
    });
    cx.run_until_parked();
    let narrow = paint(&view, cx);
    let visible = tabs(&narrow);
    assert!(visible.len() < 5);
    assert_eq!(
        visible.last().unwrap().style.text.font_weight,
        Some(gpui::FontWeight(700.))
    );
    // Endpoint snapping can change a painted width by one display pixel
    // when promotion changes its origin; its intrinsic label stays whole.
    assert!(f32::from(visible.last().unwrap().bounds.size.width - active_width).abs() <= 0.5);
    view.update(cx, |view, cx| {
        view.width = 700.;
        view.value = "0".into();
        cx.notify();
    });
    cx.run_until_parked();
    assert_eq!(
        tabs(&paint(&view, cx))
            .iter()
            .map(|tab| tab.bounds.size.width)
            .collect::<Vec<_>>(),
        initial
    );
}
#[gpui::test]
fn close_is_a_separate_button_with_native_key_timing(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.documents = true;
        view.width = 900.;
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("tab tab space space");
    assert!(view.read_with(cx, |view, _| view.closed.is_empty()));
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse("space").unwrap(),
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.closed.clone()), ["0"]);
    cx.simulate_keystrokes("enter enter");
    assert_eq!(
        view.read_with(cx, |view, _| view.closed.clone()),
        ["0", "0", "0"]
    );
    assert!(view.read_with(cx, |view, _| view.changes.is_empty()));
}
#[gpui::test]
fn the_overflow_menu_selects_a_hidden_tab_without_hiding_its_new_label(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    view.update(cx, |view, cx| {
        view.width = 180.;
        cx.notify();
    });
    cx.run_until_parked();
    let before = paint(&view, cx);
    let more = before
        .surfaces
        .iter()
        .find(|surface| surface.bounds.size == gpui::size(px(24.), px(24.)))
        .unwrap();
    cx.simulate_click(more.bounds.center(), Modifiers::default());
    cx.run_until_parked();
    key(cx, "end");
    key(cx, "enter");
    assert_eq!(view.read_with(cx, |view, _| view.value.clone()), "4");
    assert_eq!(
        tabs(&paint(&view, cx))
            .last()
            .unwrap()
            .style
            .text
            .font_weight,
        Some(gpui::FontWeight(700.))
    );
}

#[gpui::test]
fn busy_status_uses_two_forward_css_easing_intervals(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    cx.update(|_, cx| {
        Motion::set_preference(MotionPreference::Full, cx);
        Motion::seek(std::time::Duration::ZERO, cx);
    });
    view.update(cx, |view, cx| {
        view.documents = true;
        view.busy = true;
        cx.notify();
    });
    cx.run_until_parked();
    for (time, expected) in [
        (0, 1.),
        (350, 0.495885),
        (700, 0.35),
        (1050, 0.854115),
        (1400, 1.),
    ] {
        cx.update(|_, cx| Motion::seek(std::time::Duration::from_millis(time), cx));
        cx.run_until_parked();
        let paint = paint(&view, cx);
        let dot = paint
            .surfaces
            .iter()
            .find(|surface| surface.bounds.size == gpui::size(px(6.), px(6.)))
            .unwrap();
        assert!((dot.style.opacity.unwrap_or(1.) - expected).abs() < 0.00001);
    }
    cx.update(|_, cx| Motion::set_preference(MotionPreference::Reduced, cx));
    cx.run_until_parked();
    let paint = paint(&view, cx);
    let dot = paint
        .surfaces
        .iter()
        .find(|surface| surface.bounds.size == gpui::size(px(6.), px(6.)))
        .unwrap();
    assert_eq!(dot.style.opacity, Some(1.));
}

#[gpui::test]
fn restarting_busy_status_starts_a_new_cycle_at_the_current_time(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx);
    cx.update(|_, cx| {
        Motion::set_preference(MotionPreference::Full, cx);
        Motion::seek(std::time::Duration::ZERO, cx);
    });
    view.update(cx, |view, cx| {
        view.documents = true;
        view.busy = true;
        cx.notify();
    });
    cx.run_until_parked();
    cx.update(|_, cx| Motion::seek(std::time::Duration::from_millis(350), cx));
    cx.run_until_parked();
    view.update(cx, |view, cx| {
        view.busy = false;
        cx.notify();
    });
    cx.run_until_parked();
    cx.update(|_, cx| Motion::seek(std::time::Duration::from_millis(1050), cx));
    view.update(cx, |view, cx| {
        view.busy = true;
        cx.notify();
    });
    cx.run_until_parked();
    for (time, expected) in [(1050, 1.), (1400, 0.495885)] {
        cx.update(|_, cx| Motion::seek(std::time::Duration::from_millis(time), cx));
        cx.run_until_parked();
        let paint = paint(&view, cx);
        let dot = paint
            .surfaces
            .iter()
            .find(|surface| surface.bounds.size == gpui::size(px(6.), px(6.)))
            .unwrap();
        assert!((dot.style.opacity.unwrap_or(1.) - expected).abs() < 0.00001);
    }
}
