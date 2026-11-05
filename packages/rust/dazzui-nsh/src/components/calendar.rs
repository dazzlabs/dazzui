use crate::{Size, Theme, Tint, styles};
use chrono::{Datelike, Duration, Months, NaiveDate};
pub use nsh_base::{CalendarValue, Date as CalendarDate};
use nsh_core::prelude::*;

pub fn parse_date(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value.get(..10)?, "%Y-%m-%d").ok()
}
#[derive(Clone, PartialEq, Props)]
pub struct CalendarProps {
    #[prop(default)]
    pub value: Prop<CalendarValue>,
    #[prop(default)]
    pub default_value: Prop<CalendarDate>,
    #[prop(default)]
    pub range: Prop<bool>,
    #[prop(default = 1)]
    pub months: Prop<u32>,
    #[prop(default = 1)]
    pub week_starts_on: Prop<u32>,
    pub today: Prop<Option<SharedString>>,
    pub min: Prop<Option<SharedString>>,
    pub max: Prop<Option<SharedString>>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
}
#[derive(Clone, PartialEq, Event)]
pub enum CalendarEvent {
    #[args(value)]
    Change(CalendarDate),
}

fn month_start(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap()
}
fn shift_month(date: NaiveDate, delta: i32) -> NaiveDate {
    if delta < 0 {
        date.checked_sub_months(Months::new(delta.unsigned_abs()))
    } else {
        date.checked_add_months(Months::new(delta as u32))
    }
    .unwrap_or(date)
}

#[derive(Clone, Copy)]
struct CalendarState {
    cursor: Signal<NaiveDate>,
    request: Signal<Option<NaiveDate>>,
    focused: Signal<bool>,
    hovered: Signal<Option<NaiveDate>>,
}

/// Six complete weeks, with a single roving tab stop and bounded date navigation.
/// Native Shell owns focus and activation; this grid follows the desktop design's
/// fixed six-week layout instead of NSH Calendar's variable-height month picker.
#[component]
pub fn Calendar(p: CalendarProps, cx: Scope<CalendarEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let own = signal(p.default_value.get_untracked());
    let selection = move || match p.value.get() {
        CalendarValue::Controlled(v) => v,
        CalendarValue::Uncontrolled => own.get(),
    };
    let today = move || {
        p.today
            .get()
            .and_then(|s| parse_date(&s))
            .unwrap_or_else(|| chrono::Local::now().date_naive())
    };
    let initial = selection().start().unwrap_or_else(today);
    let page = signal(month_start(initial));
    let cursor = signal(initial);
    let request = signal(None::<NaiveDate>);
    let focused = signal(false);
    let hovered = signal(None::<NaiveDate>);
    provide_context(CalendarState {
        cursor,
        request,
        focused,
        hovered,
    });
    let enabled = move |date: NaiveDate| {
        !p.min
            .get()
            .and_then(|s| parse_date(&s))
            .is_some_and(|min| date < min)
            && !p
                .max
                .get()
                .and_then(|s| parse_date(&s))
                .is_some_and(|max| date > max)
    };
    let pick = move |date: NaiveDate| {
        if !enabled(date) {
            return;
        }
        let next = if p.range.get() {
            match selection() {
                CalendarDate::Range(Some(a), None) => {
                    CalendarDate::Range(Some(a.min(date)), Some(a.max(date)))
                }
                _ => CalendarDate::Range(Some(date), None),
            }
        } else {
            CalendarDate::Single(Some(date))
        };
        if matches!(p.value.get_untracked(), CalendarValue::Uncontrolled) {
            own.set(next);
        }
        cursor.set(date);
        cx.emit(CalendarEvent::Change(next));
    };
    effect(move || {
        if !focused.get() {
            return;
        }
        for key in [
            "left",
            "right",
            "up",
            "down",
            "home",
            "end",
            "pageup",
            "pagedown",
            "shift+pageup",
            "shift+pagedown",
            "enter",
            "space",
        ] {
            on_key_with(
                key,
                nsh_core::element::KeyOptions {
                    when: nsh_core::keymap::When::NotEditing,
                    repeat: key != "enter" && key != "space",
                },
                move |_| {
                    let from = cursor.get_untracked();
                    if key == "enter" || key == "space" {
                        pick(from);
                        return;
                    }
                    let weekday = (from.weekday().num_days_from_sunday() + 7
                        - p.week_starts_on.get() % 7)
                        % 7;
                    let mut next = match key {
                        "left" => from - Duration::days(1),
                        "right" => from + Duration::days(1),
                        "up" => from - Duration::days(7),
                        "down" => from + Duration::days(7),
                        "home" => from - Duration::days(i64::from(weekday)),
                        "end" => from + Duration::days(i64::from(6 - weekday)),
                        "pageup" => shift_month(from, -1),
                        "pagedown" => shift_month(from, 1),
                        "shift+pageup" => shift_month(from, -12),
                        _ => shift_month(from, 12),
                    };
                    if let Some(min) = p.min.get().and_then(|s| parse_date(&s)) {
                        next = next.max(min);
                    }
                    if let Some(max) = p.max.get().and_then(|s| parse_date(&s)) {
                        next = next.min(max);
                    }
                    if !enabled(next) {
                        return;
                    }
                    cursor.set(next);
                    request.set(Some(next));
                    let last = shift_month(page.get_untracked(), p.months.get().max(1) as i32);
                    if next < page.get_untracked() || next >= last {
                        page.set(month_start(next));
                    }
                },
            );
        }
    });
    let cell = move || {
        if matches!(p.size.get(), Size::Tiny | Size::Small) {
            theme.tokens().calendar_small_cell_size
        } else {
            theme.tokens().calendar_medium_cell_size
        }
    };
    view()
        .flex_row()
        .flex_shrink_0()
        .gap(move || px(theme.tokens().spacing_4))
        .child(dynamic(move || {
            (0..p.months.get().max(1))
                .map(|index| {
                    let month = shift_month(page.get(), index as i32);
                    let offset = (month.weekday().num_days_from_sunday() + 7
                        - p.week_starts_on.get() % 7)
                        % 7;
                    let start = month - Duration::days(i64::from(offset));
                    let width = move || px(cell() * 7.0 + theme.tokens().calendar_cell_gap * 6.0);
                    view()
                        .w(width)
                        .gap(move || px(theme.tokens().spacing_1))
                        .child(
                            view()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .h(move || px(theme.tokens().control_tiny_size))
                                .child(
                                    crate::button()
                                        .icon_only(true)
                                        .size(Size::Tiny)
                                        .variant(crate::Variant::Plain)
                                        .tint(Tint::Neutral)
                                        .accessibility_label("Previous month")
                                        .style_when(index != 0, |s| s.opacity(0.0))
                                        .disabled(index != 0)
                                        .on_press(move || page.update(|d| *d = shift_month(*d, -1)))
                                        .child(crate::icon("ChevronLeft12Regular")),
                                )
                                .child(text(month.format("%B %Y").to_string()).style_with(
                                    move |s| styles::typography(s, theme.tokens().label_medium),
                                ))
                                .child(
                                    crate::button()
                                        .icon_only(true)
                                        .size(Size::Tiny)
                                        .variant(crate::Variant::Plain)
                                        .tint(Tint::Neutral)
                                        .accessibility_label("Next month")
                                        .style_when(index + 1 != p.months.get(), |s| s.opacity(0.0))
                                        .disabled(index + 1 != p.months.get())
                                        .on_press(move || page.update(|d| *d = shift_month(*d, 1)))
                                        .child(crate::icon("ChevronRight12Regular")),
                                ),
                        )
                        .child(
                            view()
                                .flex_row()
                                .gap(move || px(theme.tokens().calendar_cell_gap))
                                .child(
                                    (0..7)
                                        .map(move |d| {
                                            view()
                                                .w(move || px(cell()))
                                                .h(move || px(theme.tokens().spacing_5))
                                                .items_center()
                                                .justify_center()
                                                .style_with(move |s| {
                                                    styles::typography(
                                                        s,
                                                        theme.tokens().caption_small,
                                                    )
                                                    .font_weight(
                                                        theme.tokens().label_small.font_weight(),
                                                    )
                                                    .text_color(
                                                        theme.tokens().color_content_subtle.hsla(),
                                                    )
                                                })
                                                .child(text(
                                                    ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"][((d
                                                        + p.week_starts_on.get())
                                                        % 7)
                                                        as usize],
                                                ))
                                        })
                                        .collect::<Vec<_>>(),
                                ),
                        )
                        .child(
                            view()
                                .role(AccessibilityRole::Grid)
                                .label(month.format("%B %Y").to_string())
                                .gap(move || px(theme.tokens().calendar_cell_gap))
                                .child(
                                    (0..6)
                                        .map(move |week| {
                                            view()
                                                .role(AccessibilityRole::Row)
                                                .flex_row()
                                                .gap(move || px(theme.tokens().calendar_cell_gap))
                                                .child(
                                                    (0..7)
                                                        .map(move |day| {
                                                            let date = start
                                                                + Duration::days(week * 7 + day);
                                                            let outside =
                                                                date.month() != month.month();
                                                            calendar_day()
                                                                .date(
                                                                    date.format("%Y-%m-%d")
                                                                        .to_string(),
                                                                )
                                                                .outside(outside)
                                                                .value(selection)
                                                                .range(p.range)
                                                                .today(
                                                                    today()
                                                                        .format("%Y-%m-%d")
                                                                        .to_string(),
                                                                )
                                                                .cell(cell)
                                                                .tint(p.tint)
                                                                .disabled(move || !enabled(date))
                                                                .on_pick(move || pick(date))
                                                        })
                                                        .collect::<Vec<_>>(),
                                                )
                                        })
                                        .collect::<Vec<_>>(),
                                ),
                        )
                })
                .collect::<Vec<_>>()
        }))
}

#[derive(Clone, PartialEq, Props)]
struct CalendarDayProps {
    date: Prop<SharedString>,
    outside: Prop<bool>,

    value: Prop<CalendarDate>,
    range: Prop<bool>,
    today: Prop<SharedString>,
    cell: Prop<f32>,
    tint: Prop<Tint>,
    disabled: Prop<bool>,
}
#[derive(Clone, PartialEq, Event)]
enum CalendarDayEvent {
    Pick,
}
#[component]
fn CalendarDay(p: CalendarDayProps, cx: Scope<CalendarDayEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let state = use_context::<CalendarState>().expect("CalendarDay needs Calendar");
    let date = parse_date(&p.date.get_untracked()).unwrap();
    let reference = element_ref();
    let ready = signal(false);
    effect(move || {
        if ready.get() && !p.outside.get() && state.request.get() == Some(date) {
            state.request.set(None);
            reference.focus();
        }
    });
    view()
        .element_ref(&reference)
        .role(AccessibilityRole::Button)
        .label(p.date)
        .disabled(p.disabled)
        .accessibility_state(move || {
            let selected = match p.value.get() {
                CalendarDate::Single(a) => a == Some(date),
                CalendarDate::Range(Some(a), Some(b)) => date >= a.min(b) && date <= a.max(b),
                CalendarDate::Range(a, _) => a == Some(date),
            };
            nsh_core::AccessibilityState {
                selected,
                disabled: p.disabled.get(),
                ..Default::default()
            }
        })
        .focusable(move || !p.disabled.get() && !p.outside.get() && state.cursor.get() == date)
        .on_layout(move |_: &nsh_core::Bounds<nsh_core::Points>| ready.set(true))
        .on_focus(move || {
            state.cursor.set(date);
            state.focused.set(true);
        })
        .on_blur(move || state.focused.set(false))
        .on_press(move || {
            if !p.disabled.get() {
                reference.focus();
                cx.emit(CalendarDayEvent::Pick);
            }
        })
        .on_mouse_enter(move |_: &nsh_core::MouseEvent| state.hovered.set(Some(date)))
        .on_mouse_leave(move |_: &nsh_core::MouseEvent| state.hovered.set(None))
        .style_with(move |s| {
            let t = theme.tokens();
            let ramp = p.tint.get().ramp(t);
            let (a, b) = match p.value.get() {
                CalendarDate::Range(a, b) => (a, b),
                CalendarDate::Single(a) => (a, a),
            };
            let selected = a == Some(date) || b == Some(date);
            let end = b.or_else(|| state.hovered.get()).or(a);
            let within = p.range.get()
                && a.zip(end)
                    .is_some_and(|(a, b)| date >= a.min(b) && date <= a.max(b));
            let s = styles::typography(s, t.label_quiet)
                .size(px(p.cell.get()))
                .items_center()
                .justify_center()
                .rounded(px(t.radius_small))
                .text_color(if p.outside.get() {
                    t.color_content_faint.hsla()
                } else {
                    t.color_content.hsla()
                })
                .opacity(if p.disabled.get() { 0.4 } else { 1.0 });
            let s = if within {
                let (start, end) = a.zip(end).unwrap();
                s.rounded_none()
                    .style_when(date == start.min(end), |s| s.rounded_s(px(t.radius_small)))
                    .style_when(date == start.max(end), |s| s.rounded_e(px(t.radius_small)))
                    .style_when(b.is_none() && date == end && date != start, |s| {
                        s.outline(px(t.stroke_hairline))
                            .outline_color(ramp.shade(600).hsla())
                            .outline_dashed()
                            .outline_offset(px(-t.stroke_hairline))
                    })
                    .bg(ramp.shade(600).alpha(t.wash_surface))
            } else {
                s
            };
            if selected {
                s.bg(ramp.shade(600).hsla())
                    .text_color(t.color_on_accent.hsla())
                    .font_weight(t.label_medium.font_weight())
            } else if p.today.get().as_ref() == date.format("%Y-%m-%d").to_string() {
                s.text_color(ramp.shade(600).hsla())
                    .font_weight(t.label_strong.font_weight())
            } else {
                s
            }
        })
        .hover(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let value = p.value.get();
                if value.start() == Some(date) || value.end() == Some(date) {
                    styles::colors(
                        s,
                        t,
                        crate::Variant::Filled,
                        p.tint.get(),
                        crate::State::Hovered,
                    )
                } else {
                    s.bg(t.color_surface_subtle.hsla())
                }
            })
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .child(text(date.day().to_string()))
        .child(when(
            move || parse_date(&p.today.get()) == Some(date),
            move || {
                view()
                    .absolute()
                    .bottom(move || px(theme.tokens().spacing_0_5))
                    .size(move || px(theme.tokens().spacing_0_5))
                    .rounded(move || px(theme.tokens().radius_full))
                    .bg(move || {
                        let t = theme.tokens();
                        let value = p.value.get();
                        if value.start() == Some(date) || value.end() == Some(date) {
                            t.color_on_accent.hsla()
                        } else {
                            p.tint.get().ramp(t).shade(600).hsla()
                        }
                    })
            },
        ))
}
