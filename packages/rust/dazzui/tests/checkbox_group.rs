//! Parent cycling, controlled proposals and direct-child geometry follow Base UI.

#[allow(dead_code)]
mod support;

use dazzui::{Checkbox, CheckboxGroup, Motion, MotionPreference, Theme};
use gpui::{
    Context, Entity, IntoElement, KeyUpEvent, Keystroke, Modifiers, ObservedPaint, ObservedSurface,
    ParentElement, Render, SharedString, Styled, TestAppContext, VisualTestContext, Window, div,
    prelude::*, px,
};
use std::{cell::RefCell, rc::Rc};
use support::paint::PaintProbe;

struct Harness {
    initial: Vec<SharedString>,
    value: Vec<SharedString>,
    controlled: bool,
    accept: bool,
    parent: bool,
    all_values: bool,
    disabled: Vec<SharedString>,
    read_only: Vec<SharedString>,
    group_disabled: bool,
    hide_b: bool,
    nested: bool,
    changes: Vec<Vec<SharedString>>,
    events: Vec<String>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        let mut group = CheckboxGroup::new("group")
            .default_value(self.initial.clone())
            .disabled(self.group_disabled)
            .when(self.all_values, |group| group.all_values(["a", "b", "c"]))
            .when(self.controlled, |group| group.value(self.value.clone()))
            .on_value_change(move |next, _, cx| {
                let _ = view.update(cx, |view, cx| {
                    view.events.push("group".into());
                    view.changes.push(next.clone());
                    if view.accept {
                        view.value = next;
                    }
                    cx.notify();
                });
            });
        if self.parent {
            group = group.child(dazzui::foundation::provide(
                (),
                Checkbox::new("all")
                    .parent(true)
                    .child("All")
                    .into_any_element()
                    .into_any_element(),
            ));
        }
        for value in ["a", "b", "c"] {
            if value == "b" && self.hide_b {
                continue;
            }
            let view = cx.entity().downgrade();
            group = group.child(
                Checkbox::new(value)
                    .value(value)
                    .disabled(self.disabled.iter().any(|item| item == value))
                    .read_only(self.read_only.iter().any(|item| item == value))
                    .on_checked_change(move |_, _, cx| {
                        let _ = view.update(cx, |view, _| view.events.push(value.into()));
                    })
                    .child(value),
            );
        }
        if self.nested {
            group = group.child(
                CheckboxGroup::new("inner")
                    .all_values(["d", "e"])
                    .default_value(["d"])
                    .child(Checkbox::new("inner-all").parent(true).child("Inner"))
                    .child(Checkbox::new("d").value("d").child("d"))
                    .child(Checkbox::new("e").value("e").child("e")),
            );
        }
        div()
            .size_full()
            .p(px(24.))
            .child(div().w(px(240.)).child(PaintProbe {
                child: group.into_any_element(),
                paints: self.paints.clone(),
            }))
    }
}

fn setup<'a>(
    cx: &'a mut TestAppContext,
    initial: &[&str],
) -> (Entity<Harness>, &'a mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
        Motion::set_preference(MotionPreference::Reduced, cx);
    });
    let initial: Vec<SharedString> = initial
        .iter()
        .map(|value| SharedString::from(value.to_string()))
        .collect();
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        initial: initial.clone(),
        value: initial,
        controlled: false,
        accept: true,
        parent: true,
        all_values: true,
        disabled: vec![],
        read_only: vec![],
        group_disabled: false,
        hide_b: false,
        nested: false,
        changes: vec![],
        events: vec![],
        paints: Rc::new(RefCell::new(Vec::new())),
    });
    cx.run_until_parked();
    (view, cx)
}

fn paint(view: &Entity<Harness>, cx: &VisualTestContext) -> ObservedPaint {
    view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone())
}

fn rows(paint: &ObservedPaint) -> Vec<&ObservedSurface> {
    paint
        .surfaces
        .iter()
        .filter(|surface| surface.style.text.font_size.is_some())
        .collect()
}

fn states(view: &Entity<Harness>, cx: &VisualTestContext) -> Vec<&'static str> {
    let paint = paint(view, cx);
    rows(&paint)
        .iter()
        .map(|row| {
            let checked = row.style.text.font_weight == Some(gpui::FontWeight(600.));
            let marked = paint.surfaces.iter().any(|surface| {
                surface.bounds.top() >= row.bounds.top()
                    && surface.bounds.bottom() <= row.bounds.bottom()
                    && surface.bounds.size.width == px(16.)
                    && surface.style.background.is_some()
            });
            if checked {
                "true"
            } else if marked {
                "mixed"
            } else {
                "false"
            }
        })
        .collect()
}

fn click(view: &Entity<Harness>, cx: &mut VisualTestContext, row: usize) {
    let point = rows(&paint(view, cx))[row].bounds.center();
    cx.simulate_click(point, Modifiers::default());
    cx.run_until_parked();
}

fn key(cx: &mut VisualTestContext, key: &str) {
    cx.simulate_keystrokes(key);
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse(key).unwrap(),
    });
    cx.run_until_parked();
}

fn changes(view: &Entity<Harness>, cx: &VisualTestContext) -> Vec<Vec<String>> {
    view.read_with(cx, |view, _| {
        view.changes
            .iter()
            .map(|values| values.iter().map(ToString::to_string).collect())
            .collect()
    })
}

#[gpui::test]
fn mixed_parent_cycles_preserve_disabled_values_and_restore_history(cx: &mut TestAppContext) {
    for family in ["bright", "studio", "frost", "graphite", "ember", "nocturne"] {
        for mode in ["light", "dark"] {
            for (initial, expected) in [
                (vec!["a"], vec![vec!["a", "c"], vec![], vec!["a"]]),
                (
                    vec!["a", "b"],
                    vec![vec!["a", "b", "c"], vec!["b"], vec!["a", "b"]],
                ),
            ] {
                let (view, cx) = setup(cx, &initial);
                cx.update(|_, cx| Theme::set(&format!("{family}-{mode}"), cx));
                view.update(cx, |view, cx| {
                    view.disabled = vec!["b".into()];
                    cx.notify();
                });
                cx.run_until_parked();
                assert_eq!(states(&view, cx)[0], "mixed");
                for _ in 0..3 {
                    click(&view, cx, 0);
                }
                assert_eq!(changes(&view, cx), expected);
                assert_eq!(states(&view, cx)[0], "mixed");
                let paint = paint(&view, cx);
                let rows = rows(&paint);
                assert_eq!(rows[1].bounds.top() - rows[0].bounds.top(), px(26.));
                let marks: Vec<_> = paint
                    .surfaces
                    .iter()
                    .filter(|surface| {
                        surface.bounds.size.width == px(16.)
                            && surface.style.border_widths.left != px(0.).into()
                    })
                    .collect();
                assert_eq!(marks[1].bounds.left() - marks[0].bounds.left(), px(26.));
            }
        }
    }
}

#[gpui::test]
fn nested_groups_keep_their_own_selection_and_disabled_scope(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &["a"]);
    view.update(cx, |view, cx| {
        view.nested = true;
        view.group_disabled = true;
        cx.notify();
    });
    cx.run_until_parked();
    assert_eq!(
        states(&view, cx),
        ["mixed", "true", "false", "false", "mixed", "true", "false"]
    );
    click(&view, cx, 4);
    assert_eq!(
        states(&view, cx),
        ["mixed", "true", "false", "false", "true", "true", "true"]
    );
    assert!(changes(&view, cx).is_empty());
}

#[gpui::test]
fn controlled_groups_report_without_latching_and_keep_original_parent_history(
    cx: &mut TestAppContext,
) {
    let (view, cx) = setup(cx, &["a"]);
    view.update(cx, |view, cx| {
        view.controlled = true;
        view.accept = false;
        cx.notify();
    });
    cx.run_until_parked();
    click(&view, cx, 2);
    assert_eq!(states(&view, cx), ["mixed", "true", "false", "false"]);
    assert_eq!(changes(&view, cx), [vec!["a", "b"]]);
    assert_eq!(
        view.read_with(cx, |view, _| view.events.clone()),
        ["b", "group"]
    );
    // A child proposal updates the remembered mixed set even when a controlled
    // host declines it. A later external value does not overwrite that history.
    view.update(cx, |view, cx| {
        view.value = vec!["c".into()];
        view.accept = true;
        cx.notify();
    });
    cx.run_until_parked();
    for _ in 0..3 {
        click(&view, cx, 0);
    }
    assert_eq!(states(&view, cx), ["mixed", "true", "true", "false"]);
}

#[gpui::test]
fn plain_groups_honor_read_only_and_keyboard_space_without_enter(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &[]);
    view.update(cx, |view, cx| {
        view.all_values = false;
        view.read_only = vec!["a".into()];
        cx.notify();
    });
    cx.run_until_parked();
    cx.simulate_keystrokes("tab tab");
    key(cx, "space");
    assert!(changes(&view, cx).is_empty());
    cx.simulate_keystrokes("tab");
    key(cx, "enter");
    assert!(changes(&view, cx).is_empty());
    key(cx, "space");
    assert_eq!(states(&view, cx), ["false", "false", "true", "false"]);
    assert_eq!(changes(&view, cx), [vec!["b"]]);
    click(&view, cx, 0);
    assert_eq!(states(&view, cx), ["true", "false", "true", "false"]);
    assert_eq!(
        changes(&view, cx),
        [vec!["b"]],
        "a parent without allValues acts independently"
    );
}

#[gpui::test]
fn removed_disabled_children_do_not_leave_stale_group_registrations(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, &[]);
    view.update(cx, |view, cx| {
        view.disabled = vec!["b".into()];
        cx.notify();
    });
    cx.run_until_parked();
    click(&view, cx, 0);
    assert_eq!(changes(&view, cx), [vec!["a", "c"]]);
    view.update(cx, |view, cx| {
        view.hide_b = true;
        cx.notify();
    });
    cx.run_until_parked();
    click(&view, cx, 0);
    assert_eq!(changes(&view, cx).last().unwrap(), &vec!["a", "b", "c"]);
    view.update(cx, |view, cx| {
        view.group_disabled = true;
        cx.notify();
    });
    cx.run_until_parked();
    let count = changes(&view, cx).len();
    for row in 0..3 {
        click(&view, cx, row);
    }
    assert_eq!(changes(&view, cx).len(), count);
}
