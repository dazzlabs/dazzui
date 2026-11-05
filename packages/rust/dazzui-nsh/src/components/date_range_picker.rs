//! A transactional range editor. The popup owns a draft until Done.
use crate::{CalendarDate, CalendarValue, SelectItem, Size, Theme, Tint, Variant, styles};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use nsh_core::prelude::*;

#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct DateRangeValue {
    pub start: SharedString,
    pub end: SharedString,
    pub all_day: bool,
}
impl DateRangeValue {
    pub fn new(
        start: impl Into<SharedString>,
        end: impl Into<SharedString>,
        all_day: bool,
    ) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            all_day,
        }
    }
    fn parsed(&self) -> Option<(NaiveDateTime, NaiveDateTime)> {
        Some((parse_datetime(&self.start)?, parse_datetime(&self.end)?))
    }
    fn dates(&self) -> CalendarDate {
        self.parsed()
            .map(|(a, b)| CalendarDate::Range(Some(a.date()), Some(b.date())))
            .unwrap_or(CalendarDate::Range(None, None))
    }
    fn with_times(&self, start: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self::new(iso(start), iso(end), self.all_day)
    }
    fn days(&self, a: NaiveDate, b: NaiveDate) -> Self {
        let Some((start, end)) = self.parsed() else {
            return self.clone();
        };
        let start = a.and_time(start.time());
        let end = b.and_time(end.time());
        self.with_times(
            start,
            if end < start {
                start + Duration::hours(1)
            } else {
                end
            },
        )
    }
    fn time(&self, value: &str, is_start: bool) -> Self {
        let (Some((start, end)), Ok(time)) =
            (self.parsed(), NaiveTime::parse_from_str(value, "%H:%M"))
        else {
            return self.clone();
        };
        if is_start {
            let next = start.date().and_time(time);
            self.with_times(next, next + (end - start))
        } else {
            let next = end.date().and_time(time);
            self.with_times(
                if next < start {
                    next - Duration::hours(1)
                } else {
                    start
                },
                next,
            )
        }
    }
    fn duration(&self, minutes: u32) -> Self {
        let Some((start, _)) = self.parsed() else {
            return self.clone();
        };
        Self {
            all_day: false,
            ..self.with_times(start, start + Duration::minutes(i64::from(minutes)))
        }
    }
    fn summary(&self) -> Vec<(String, bool)> {
        let Some((a, b)) = self.parsed() else {
            return vec![("Choose date and time".into(), false)];
        };
        let day = |d: NaiveDateTime| d.format("%a, %b %-d").to_string();
        let time = |d: NaiveDateTime| d.format("%H:%M").to_string();
        if self.all_day {
            if a.date() == b.date() {
                vec![
                    (day(a), false),
                    ("·".into(), true),
                    ("All day".into(), false),
                ]
            } else {
                vec![(day(a), false), ("→".into(), true), (day(b), false)]
            }
        } else if a.date() == b.date() {
            vec![
                (day(a), false),
                ("·".into(), true),
                (time(a), false),
                ("→".into(), true),
                (time(b), false),
            ]
        } else {
            vec![
                (format!("{} {}", day(a), time(a)), false),
                ("→".into(), true),
                (format!("{} {}", day(b), time(b)), false),
            ]
        }
    }
}
fn parse_datetime(s: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
        .ok()
        .or_else(|| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S").ok())
}
fn iso(d: NaiveDateTime) -> String {
    d.format("%Y-%m-%dT%H:%M").to_string()
}
fn blank(today: NaiveDate) -> DateRangeValue {
    let now = Local::now();
    let hour = if today == now.date_naive() {
        (now.hour() + 1).min(23)
    } else {
        9
    };
    let start = today.and_hms_opt(hour, 0, 0).unwrap();
    DateRangeValue::new(iso(start), iso(start + Duration::hours(1)), false)
}
#[derive(Clone, PartialEq, JsObject)]
pub struct DateRangePreset {
    pub label: SharedString,
    pub days: Vec<i32>,
}
#[derive(Clone, PartialEq, JsObject)]
pub struct DateRangeDuration {
    pub label: SharedString,
    pub minutes: u32,
}
fn presets(today: NaiveDate) -> Vec<DateRangePreset> {
    let weekday = today.weekday().num_days_from_monday() as i32;
    let next_monday = 7 - weekday;
    let saturday = if weekday < 5 {
        5 - weekday
    } else {
        12 - weekday
    };
    let first = today.with_day(1).unwrap();
    let next = if today.month() == 12 {
        NaiveDate::from_ymd_opt(today.year() + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1)
    }
    .unwrap();
    [
        ("Today", 0, 0),
        ("Tomorrow", 1, 1),
        ("This weekend", saturday, saturday + 1),
        ("Next Monday", next_monday, next_monday),
        ("Next week", next_monday, next_monday + 6),
        (
            "This month",
            (first - today).num_days() as i32,
            (next - Duration::days(1) - today).num_days() as i32,
        ),
    ]
    .into_iter()
    .map(|(s, a, b)| DateRangePreset {
        label: s.into(),
        days: vec![a, b],
    })
    .collect()
}
fn times(step: u32, current: SharedString) -> Vec<SelectItem> {
    let mut times = (0..1440)
        .step_by(step.clamp(1, 1440) as usize)
        .map(|m| format!("{:02}:{:02}", m / 60, m % 60))
        .collect::<Vec<_>>();
    if !times.iter().any(|t| t == current.as_ref()) {
        times.push(current.to_string());
        times.sort();
    }
    times
        .into_iter()
        .map(|s| SelectItem::new(s.clone(), s))
        .collect()
}
#[derive(Clone, PartialEq, Props)]
pub struct DateRangePickerProps {
    pub value: Prop<Option<DateRangeValue>>,
    pub default_value: Prop<Option<DateRangeValue>>,
    pub today: Prop<Option<SharedString>>,
    pub presets: Prop<Option<Vec<DateRangePreset>>>,
    pub durations: Prop<Option<Vec<DateRangeDuration>>>,
    #[prop(default = 15)]
    pub time_step: Prop<u32>,
    #[prop(default = 1)]
    pub months: Prop<u32>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub error: Prop<bool>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum DateRangePickerEvent {
    #[args(value)]
    Change(DateRangeValue),
}
#[component]
pub fn DateRangePicker(
    p: DateRangePickerProps,
    cx: Scope<DateRangePickerEvent>,
) -> impl IntoElement {
    let theme = Theme::current();
    let field = use_context::<super::form_field::FieldContext>();
    let today = move || {
        p.today
            .get()
            .and_then(|s| super::parse_date(&s))
            .unwrap_or_else(|| Local::now().date_naive())
    };
    let own = signal(p.default_value.get_untracked());
    let committed = move || p.value.get().or_else(|| own.get());
    let draft = signal(committed().unwrap_or_else(|| blank(today())));
    let open = signal(false);
    let selected = signal(draft.get_untracked().dates());
    let begin = move || {
        let value = committed().unwrap_or_else(|| blank(today()));
        selected.set(value.dates());
        draft.set(value);
        open.set(true);
    };
    let time_value = move |start: bool| -> SharedString {
        draft
            .get()
            .parsed()
            .map(|(a, b)| if start { a } else { b })
            .map(|d| d.format("%H:%M").to_string())
            .unwrap_or_default()
            .into()
    };
    crate::popover()
        .open(move || Some(open.get()))
        .on_open_change(move |e: &nsh_base::PopoverEvent| {
            let nsh_base::PopoverEvent::OpenChange(value, _) = e else {
                return;
            };
            if *value {
                if !p.disabled.get() {
                    begin();
                }
            } else {
                open.set(false);
            }
        })
        .trigger(
            view()
                .when_some(field, |v, f| v.element_ref(&f.editor))
                .disabled(p.disabled)
                .optional_label(move || {
                    p.accessibility_label
                        .get()
                        .or_else(|| field.map(|f| f.label.get()))
                })
                .flex_row()
                .items_center()
                .gap(move || px(theme.tokens().spacing_2))
                .style_with(move |s| {
                    styles::field(
                        s,
                        theme.tokens(),
                        p.size.get(),
                        p.tint.get(),
                        p.disabled.get(),
                        p.error.get() || field.is_some_and(|f| f.error.get()),
                    )
                    .w(nsh_core::geometry::Length::Auto)
                    .min_w(px(theme.tokens().date_range_picker_trigger_min_width))
                })
                .child(
                    crate::icon("CalendarLtr16Regular").size(move || px(theme.tokens().icon_small)),
                )
                .child(
                    view()
                        .flex_row()
                        .flex_grow(1.0)
                        .items_center()
                        .child(dynamic(move || {
                            let parts = committed()
                                .map(|v| v.summary())
                                .unwrap_or_else(|| vec![("Choose date and time".into(), false)]);
                            parts
                                .into_iter()
                                .map(|(label, separator)| {
                                    styles::label(
                                        move || label.clone().into(),
                                        move || {
                                            let t = theme.tokens();
                                            let mut face = p.size.get().profile(t).content;
                                            face.weight = t.body_small.weight;
                                            face
                                        },
                                    )
                                    .style_when(
                                        separator,
                                        move |s| {
                                            s.mx(px(theme.tokens().spacing_1)).text_color(
                                                theme.tokens().color_content_faint.hsla(),
                                            )
                                        },
                                    )
                                })
                                .collect::<Vec<_>>()
                        })),
                )
                .child(
                    crate::icon("ChevronDown12Regular").size(move || px(theme.tokens().icon_small)),
                ),
        )
        .child(
            view()
                .flex_row()
                .gap(move || px(theme.tokens().spacing_4))
                .p(move || px(theme.tokens().spacing_3))
                .child(
                    view()
                        .w(move || px(theme.tokens().date_range_picker_side_width))
                        .gap(move || px(theme.tokens().spacing_1))
                        .child(
                            text("Quick dates").style_with(move |s| {
                                styles::typography(s, theme.tokens().label_small)
                            }),
                        )
                        .child(dynamic(move || {
                            p.presets
                                .get()
                                .unwrap_or_else(|| presets(today()))
                                .into_iter()
                                .map(move |preset| {
                                    crate::button()
                                        .variant(Variant::Plain)
                                        .full_width(true)
                                        .child(text(preset.label))
                                        .on_press(move || {
                                            let a = today()
                                                + Duration::days(i64::from(
                                                    *preset.days.first().unwrap_or(&0),
                                                ));
                                            let b = today()
                                                + Duration::days(i64::from(
                                                    *preset.days.get(1).unwrap_or(&0),
                                                ));
                                            draft.update(|v| *v = v.days(a.min(b), a.max(b)));
                                            selected.set(draft.get_untracked().dates());
                                        })
                                })
                                .collect::<Vec<_>>()
                        }))
                        .child(crate::divider())
                        .child(
                            crate::checkbox()
                                .checked(move || Some(draft.get().all_day))
                                .child(text("All day"))
                                .on_change(move |v: &bool| draft.update(|d| d.all_day = *v)),
                        )
                        .child(text("Start time"))
                        .child(
                            crate::select()
                                .accessibility_label("Start time")
                                .items(move || times(p.time_step.get(), time_value(true)))
                                .value(move || Some(time_value(true)))
                                .disabled(move || draft.get().all_day)
                                .on_change(move |v: &Option<SharedString>| {
                                    if let Some(v) = v {
                                        draft.update(|d| *d = d.time(v, true));
                                        selected.set(draft.get_untracked().dates());
                                    }
                                }),
                        )
                        .child(text("End time"))
                        .child(
                            crate::select()
                                .accessibility_label("End time")
                                .items(move || times(p.time_step.get(), time_value(false)))
                                .value(move || Some(time_value(false)))
                                .disabled(move || draft.get().all_day)
                                .on_change(move |v: &Option<SharedString>| {
                                    if let Some(v) = v {
                                        draft.update(|d| *d = d.time(v, false));
                                        selected.set(draft.get_untracked().dates());
                                    }
                                }),
                        )
                        .child(dynamic(move || {
                            let same_day = draft
                                .get()
                                .parsed()
                                .is_some_and(|(a, b)| a.date() == b.date());
                            same_day.then(|| {
                                view()
                                    .gap(move || px(theme.tokens().spacing_1))
                                    .child(text("Duration"))
                                    .child(
                                        p.durations
                                            .get()
                                            .unwrap_or_else(|| {
                                                [("30 min", 30), ("1 hour", 60), ("2 hours", 120)]
                                                    .into_iter()
                                                    .map(|(label, minutes)| DateRangeDuration {
                                                        label: label.into(),
                                                        minutes,
                                                    })
                                                    .collect()
                                            })
                                            .into_iter()
                                            .map(|d| {
                                                crate::button()
                                                    .size(Size::Small)
                                                    .variant(Variant::Plain)
                                                    .child(text(d.label))
                                                    .on_press(move || {
                                                        draft
                                                            .update(|v| *v = v.duration(d.minutes));
                                                        selected.set(draft.get_untracked().dates());
                                                    })
                                            })
                                            .collect::<Vec<_>>(),
                                    )
                            })
                        })),
                )
                .child(
                    view()
                        .gap(move || px(theme.tokens().spacing_3))
                        .child(
                            crate::calendar()
                                .range(true)
                                .months(p.months)
                                .size(p.size)
                                .tint(p.tint)
                                .today(p.today)
                                .value(move || CalendarValue::Controlled(selected.get()))
                                .on_change(move |v: &CalendarDate| {
                                    selected.set(*v);
                                    if let Some(a) = v.start() {
                                        let b = v.end().unwrap_or(a);
                                        draft.update(|d| *d = d.days(a, b));
                                    }
                                }),
                        )
                        .child(
                            text(move || {
                                draft
                                    .get()
                                    .summary()
                                    .into_iter()
                                    .map(|(s, _)| s)
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            })
                            .style_with(move |s| styles::typography(s, theme.tokens().label_quiet)),
                        )
                        .child(
                            view()
                                .flex_row()
                                .justify_end()
                                .gap(move || px(theme.tokens().spacing_2))
                                .child(
                                    crate::button()
                                        .variant(Variant::Plain)
                                        .child(text("Cancel"))
                                        .on_press(move || open.set(false)),
                                )
                                .child(
                                    crate::button()
                                        .variant(Variant::Filled)
                                        .child(text("Done"))
                                        .on_press(move || {
                                            let value = draft.get_untracked();
                                            if p.value.get_untracked().is_none() {
                                                own.set(Some(value.clone()));
                                            }
                                            cx.emit(DateRangePickerEvent::Change(value));
                                            open.set(false);
                                        }),
                                ),
                        ),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn editing_preserves_duration_and_handles_midnight() {
        let value = DateRangeValue::new("2026-10-16T23:30", "2026-10-17T01:00", false);
        assert_eq!(value.time("22:15", true).end, "2026-10-16T23:45");
        assert_eq!(value.time("00:15", false).start, "2026-10-16T23:30");
        let value =
            DateRangeValue::new("2026-10-16T14:30", "2026-10-16T15:30", true).time("13:00", false);
        assert_eq!(value.start, "2026-10-16T12:00");
        assert!(value.all_day);
    }
    #[test]
    fn next_presets_are_future_and_custom_time_is_retained() {
        let saturday = NaiveDate::from_ymd_opt(2026, 10, 17).unwrap();
        assert_eq!(presets(saturday)[2].days, vec![7, 8]);
        assert!(times(30, "09:17".into()).iter().any(|i| i.value == "09:17"));
        assert_eq!(times(0, "09:00".into()).len(), 1440);
    }
}
