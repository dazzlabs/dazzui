//! A span of days and times, chosen in one sheet.
//!
//! The trigger is a field — it wears the select's box — and what it opens
//! is a sheet: a month to pick the span on, two for a product whose spans
//! mostly cross one, and beside it a column that opens with a menu of the
//! spans a calendar reaches for most, then the start and end with their
//! times and a row of durations. Nothing in the sheet reaches the value
//! until Done; an outside press or Escape puts it back. That is the one way
//! this differs from every other popup here, and it is deliberate: a span
//! is several decisions, and a reader who has made two of them should not
//! lose both to a stray click.
//!
//! The sheet is composition rather than behaviour: the kit's own
//! [`Calendar`], [`Select`], [`Menu`] and [`Toggle`] own the parts, and the
//! floating sheet (popup.rs) owns the outside press and the Escape. The two
//! rules of its own are the ones a calendar app keeps — moving the start
//! carries the end with it, and an end set before its start pulls the start
//! back an hour ahead of it — and they are [`Draft`]'s, apart from the view.

use std::rc::Rc;

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Timelike, Weekday};
use gpui::{
    App, Div, ElementId, Entity, FocusHandle, InteractiveElement, IntoElement, MouseButton,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::*, px,
};

use crate::components::button::Button;
use crate::components::calendar::{
    Calendar, CalendarStrings, DateRange, MONTHS, WEEKDAYS, add_days, add_months, start_of_month,
};
use crate::components::checkbox::kit;
use crate::components::icon::{Icon, icon_slot};
use crate::components::menu::{Menu, MenuItem};
use crate::components::popup::{
    Align, Placement, PopupState, Press, Side, dismissable, measured, positioned, sheet,
    toggle_press,
};
use crate::components::select::{Select, SelectOption};
use crate::components::text_field::{FieldBox, FieldState};
use crate::components::toggle::Toggle;
use crate::foundation::{Size, StyledExt, Tint, Variant, focus_visible, pointer_down};
use crate::generated::ThemeTokens;
use crate::glyphs;
use crate::theme::ActiveTheme;

/// What the picker chooses: when something starts, when it ends, and
/// whether the times matter. An all-day span keeps its times — switching
/// all-day off again brings them back — and a consumer reads `all_day`
/// before them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DateRangeValue {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub all_day: bool,
}

impl DateRangeValue {
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        DateRangeValue {
            start,
            end,
            all_day: false,
        }
    }

    pub fn all_day(mut self, all_day: bool) -> Self {
        self.all_day = all_day;
        self
    }
}

/// A named span the reader can pick in one press, counted from today.
#[derive(Clone)]
pub struct DateRangePreset {
    pub label: SharedString,
    range: Rc<dyn Fn(NaiveDate) -> (NaiveDate, NaiveDate)>,
}

impl DateRangePreset {
    /// A span as a function of today.
    pub fn new(
        label: impl Into<SharedString>,
        range: impl Fn(NaiveDate) -> (NaiveDate, NaiveDate) + 'static,
    ) -> Self {
        DateRangePreset {
            label: label.into(),
            range: Rc::new(range),
        }
    }

    /// A span as whole days from today — the form a preset takes as data:
    /// `days("Today", 0, 0)`, `days("Next two weeks", 0, 13)`.
    pub fn days(label: impl Into<SharedString>, from: i64, to: i64) -> Self {
        DateRangePreset::new(label, move |today| {
            (add_days(today, from), add_days(today, to))
        })
    }

    /// The span, counted from `today`.
    pub fn range(&self, today: NaiveDate) -> (NaiveDate, NaiveDate) {
        (self.range)(today)
    }
}

/// A length the end is set to, from the start.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DateRangeDuration {
    pub label: SharedString,
    pub minutes: i64,
}

impl DateRangeDuration {
    pub fn new(label: impl Into<SharedString>, minutes: i64) -> Self {
        DateRangeDuration {
            label: label.into(),
            minutes,
        }
    }
}

/// The week's Saturday, or next week's if today is one.
fn weekend(today: NaiveDate) -> NaiveDate {
    let until = (6 + 7 - today.weekday().num_days_from_sunday() as i64) % 7;
    add_days(today, if until == 0 { 7 } else { until })
}

/// Next Monday, never today.
fn next_monday(today: NaiveDate) -> NaiveDate {
    let until = (8 - today.weekday().num_days_from_sunday() as i64) % 7;
    add_days(today, if until == 0 { 7 } else { until })
}

/// The spans a calendar reaches for most.
pub fn date_range_presets() -> Vec<DateRangePreset> {
    vec![
        DateRangePreset::new("Today", |t| (t, t)),
        DateRangePreset::new("Tomorrow", |t| (add_days(t, 1), add_days(t, 1))),
        DateRangePreset::new("This weekend", |t| (weekend(t), add_days(weekend(t), 1))),
        DateRangePreset::new("Next Monday", |t| (next_monday(t), next_monday(t))),
        DateRangePreset::new("Next week", |t| {
            (next_monday(t), add_days(next_monday(t), 6))
        }),
        DateRangePreset::new("This month", |t| {
            (start_of_month(t), add_days(add_months(t, 1), -1))
        }),
    ]
}

pub fn date_range_durations() -> Vec<DateRangeDuration> {
    vec![
        DateRangeDuration::new("30 min", 30),
        DateRangeDuration::new("1 hour", 60),
        DateRangeDuration::new("2 hours", 120),
    ]
}

/// The strings the sheet is labelled with, for a product in another
/// language, and the way it writes a day.
#[derive(Clone)]
pub struct DateRangePickerStrings {
    /// What the trigger says while nothing is chosen.
    pub placeholder: SharedString,
    pub presets: SharedString,
    pub start: SharedString,
    pub end: SharedString,
    pub duration: SharedString,
    pub all_day: SharedString,
    pub cancel: SharedString,
    pub done: SharedString,
    /// A day in the trigger and the column, e.g. `Fri, Oct 16`.
    pub day: Rc<dyn Fn(NaiveDate) -> String>,
    pub calendar: CalendarStrings,
}

impl Default for DateRangePickerStrings {
    fn default() -> Self {
        DateRangePickerStrings {
            placeholder: "Choose dates".into(),
            presets: "Quick pick".into(),
            start: "Start".into(),
            end: "End".into(),
            duration: "Duration".into(),
            all_day: "All day".into(),
            cancel: "Cancel".into(),
            done: "Done".into(),
            day: Rc::new(|day| {
                format!(
                    "{}, {} {}",
                    &WEEKDAYS[day.weekday().num_days_from_monday() as usize][..3],
                    &MONTHS[day.month0() as usize][..3],
                    day.day()
                )
            }),
            calendar: CalendarStrings::default(),
        }
    }
}

const HOUR: TimeDelta = TimeDelta::hours(1);

fn clock(at: NaiveDateTime) -> String {
    format!("{:02}:{:02}", at.hour(), at.minute())
}

/// `day` at `time`'s hours and minutes.
fn with_time(day: NaiveDate, time: NaiveDateTime) -> NaiveDateTime {
    day.and_time(NaiveTime::from_hms_opt(time.hour(), time.minute(), 0).unwrap_or_default())
}

fn set_time(at: NaiveDateTime, hour: u32, minute: u32) -> NaiveDateTime {
    at.date()
        .and_time(NaiveTime::from_hms_opt(hour, minute, 0).unwrap_or(at.time()))
}

/// The sheet's own rules, apart from the view so they can be read and
/// tested as data: moving the start carries the end with it, an end set
/// before its start pulls the start back an hour ahead of it, and the days
/// the calendar holds stay apart from the draft while a range is mid-pick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Draft {
    pub value: DateRangeValue,
    /// The days as the calendar holds them: between the first press and the
    /// second the calendar's end is unset, and the draft reads that as a
    /// one-day span until the reader finishes.
    pub days: DateRange,
    /// The month the sheet shows — the first of two.
    pub month: NaiveDate,
}

impl Draft {
    pub fn new(value: DateRangeValue) -> Self {
        Draft {
            value,
            days: DateRange::new(value.start.date(), value.end.date()),
            month: start_of_month(value.start.date()),
        }
    }

    pub fn set_days(&mut self, range: DateRange) {
        self.days = range;
        let start = range.start.unwrap_or(self.value.start.date());
        let end = range.end.unwrap_or(start);
        let next_start = with_time(start, self.value.start);
        let mut next_end = with_time(end, self.value.end);
        if next_end < next_start {
            next_end = next_start + HOUR;
        }
        self.value.start = next_start;
        self.value.end = next_end;
    }

    pub fn set_start_time(&mut self, hour: u32, minute: u32) {
        let length = self.value.end - self.value.start;
        let start = set_time(self.value.start, hour, minute);
        self.value.start = start;
        self.value.end = start + length;
    }

    pub fn set_end_time(&mut self, hour: u32, minute: u32) {
        let end = set_time(self.value.end, hour, minute);
        if end < self.value.start {
            self.value.start = end - HOUR;
        }
        self.value.end = end;
    }

    pub fn set_duration(&mut self, minutes: i64) {
        self.value.all_day = false;
        self.value.end = self.value.start + TimeDelta::minutes(minutes);
    }

    /// A duration is a length within a day; a span across days has none.
    pub fn one_day(&self) -> bool {
        self.value.start.date() == self.value.end.date()
    }

    pub fn matches_preset(&self, preset: &DateRangePreset, today: NaiveDate) -> bool {
        let (start, end) = preset.range(today);
        start == self.value.start.date() && end == self.value.end.date()
    }

    pub fn matches_duration(&self, duration: &DateRangeDuration) -> bool {
        !self.value.all_day
            && self.value.end - self.value.start == TimeDelta::minutes(duration.minutes)
    }
}

/// What the picker keeps between frames: the value when the caller leaves
/// it to the picker, and the draft while the sheet is open.
struct PickerState {
    own: Option<DateRangeValue>,
    draft: Draft,
}

type OnValueChange = Rc<dyn Fn(DateRangeValue, &mut Window, &mut App)>;
/// A time of day chosen in one of the lists, as hours and minutes.
type OnTime = Rc<dyn Fn(u32, u32, &mut App)>;

/// A date range picker.
#[derive(IntoElement)]
pub struct DateRangePicker {
    id: ElementId,
    value: Option<Option<DateRangeValue>>,
    default_value: Option<DateRangeValue>,
    on_value_change: Option<OnValueChange>,
    presets: Vec<DateRangePreset>,
    durations: Vec<DateRangeDuration>,
    time_step: u32,
    months: usize,
    week_starts_on: Weekday,
    strings: DateRangePickerStrings,
    today: Option<NaiveDate>,
    size: Size,
    state: FieldState,
    tint: Tint,
    disabled: bool,
}

impl DateRangePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        DateRangePicker {
            id: id.into(),
            value: None,
            default_value: None,
            on_value_change: None,
            presets: date_range_presets(),
            durations: date_range_durations(),
            time_step: 15,
            months: 1,
            week_starts_on: Weekday::Mon,
            strings: DateRangePickerStrings::default(),
            today: None,
            size: Size::Medium,
            state: FieldState::Default,
            tint: Tint::Primary,
            disabled: false,
        }
    }

    /// Controlled: the span chosen, which Done only reports through
    /// [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: Option<DateRangeValue>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: DateRangeValue) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(DateRangeValue, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    /// The quick-pick menu. [`date_range_presets`] unless the product has
    /// its own.
    pub fn presets(mut self, presets: Vec<DateRangePreset>) -> Self {
        self.presets = presets;
        self
    }

    /// The duration chips. [`date_range_durations`] unless the product has
    /// its own.
    pub fn durations(mut self, durations: Vec<DateRangeDuration>) -> Self {
        self.durations = durations;
        self
    }

    /// Minutes between the times the lists offer.
    pub fn time_step(mut self, minutes: u32) -> Self {
        self.time_step = minutes.max(1);
        self
    }

    /// Months side by side in the sheet: one, or two for a sheet that
    /// mostly picks spans across them.
    pub fn months(mut self, months: usize) -> Self {
        self.months = months.clamp(1, 2);
        self
    }

    pub fn week_starts_on(mut self, weekday: Weekday) -> Self {
        self.week_starts_on = weekday;
        self
    }

    pub fn strings(mut self, strings: DateRangePickerStrings) -> Self {
        self.strings = strings;
        self
    }

    /// What the grid counts as today.
    pub fn today(mut self, today: NaiveDate) -> Self {
        self.today = Some(today);
        self
    }

    /// `small`, `medium` or `large` — a text field's sizes.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// React's `state="error"`: the box re-tinted to danger.
    pub fn state(mut self, state: FieldState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The accessible name. GPUI has no accessibility tree to give it to.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

/// The next whole hour, for an hour — what a new event defaults to.
fn blank(today: NaiveDate) -> DateRangeValue {
    let now = Local::now().naive_local();
    let hour = if now.date() == today {
        (now.hour() + 1).min(23)
    } else {
        9
    };
    let start = today.and_time(NaiveTime::from_hms_opt(hour, 0, 0).unwrap_or_default());
    DateRangeValue::new(start, start + HOUR)
}

/// The caption-voiced key over a date, a time or the durations.
fn key(t: &'static ThemeTokens, text: &SharedString) -> Div {
    div()
        .text_color(t.color_content_subtle.hsla())
        .font_family(t.caption_small.face.family())
        .text_size(px(t.caption_small.size))
        .font_weight(t.label_small.font_weight())
        .line_height(px(t.caption_small.size))
        .whitespace_nowrap()
        .child(crate::foundation::text_child(text.to_uppercase()))
}

/// A span in one line: the day once when both ends share it, the times
/// unless it is all day, and the joins in the faint ink so the dates and
/// times read as the values and the arrow as punctuation.
fn summary(
    t: &'static ThemeTokens,
    value: &DateRangeValue,
    strings: &DateRangePickerStrings,
) -> Div {
    let one_day = value.start.date() == value.end.date();
    let day = |at: NaiveDateTime| (strings.day)(at.date());
    let sep = |glyph: &str| {
        div()
            .mx(px(t.spacing_1))
            .text_color(t.color_content_faint.hsla())
            .child(crate::foundation::text_child(glyph.to_string()))
    };
    let word = |text: String| crate::foundation::text_child(text);
    let line = div().flex().items_center().whitespace_nowrap();
    if value.all_day {
        if one_day {
            line.child(word(day(value.start)))
                .child(sep("·"))
                .child(word(strings.all_day.to_string()))
        } else {
            line.child(word(day(value.start)))
                .child(sep("→"))
                .child(word(day(value.end)))
        }
    } else if one_day {
        line.child(word(day(value.start)))
            .child(sep("·"))
            .child(word(clock(value.start)))
            .child(sep("→"))
            .child(word(clock(value.end)))
    } else {
        line.child(word(format!("{} {}", day(value.start), clock(value.start))))
            .child(sep("→"))
            .child(word(format!("{} {}", day(value.end), clock(value.end))))
    }
}

impl RenderOnce for DateRangePicker {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let id = self.id.clone();
        let today = self.today.unwrap_or_else(|| Local::now().date_naive());
        let picker: Entity<PickerState> =
            window.use_keyed_state(kit::child_id(&id, "state"), cx, {
                let own = self.default_value;
                move |_, _| PickerState {
                    own,
                    draft: Draft::new(own.unwrap_or_else(|| blank(today))),
                }
            });
        let focus: FocusHandle = kit::focus_handle(&id, window, cx);
        let value = self.value.unwrap_or(picker.read(cx).own);
        let draft = picker.read(cx).draft;

        let popup = PopupState::keyed(&id, false, window, cx);
        let open = popup.read(cx).open && !self.disabled;
        let anchor = popup.read(cx).anchor.clone();

        // Opening seeds the draft from the value; closing drops it, and only
        // Done writes it back.
        let toggle: Press = {
            let toggle = toggle_press(&popup, None, None);
            let popup = popup.clone();
            let picker = picker.clone();
            Rc::new(move |window, cx| {
                if !popup.read(cx).open {
                    picker.update(cx, |state, cx| {
                        state.draft = Draft::new(value.unwrap_or_else(|| blank(today)));
                        cx.notify();
                    });
                }
                toggle(window, cx);
            })
        };
        let close: Press = {
            let popup = popup.clone();
            Rc::new(move |window, cx| {
                popup.update(cx, |state, cx| state.hide(window, cx));
            })
        };
        let commit: Press = {
            let picker = picker.clone();
            let close = close.clone();
            let controlled = self.value.is_some();
            let notify = self.on_value_change.clone();
            Rc::new(move |window, cx| {
                let next = picker.read(cx).draft.value;
                if !controlled {
                    picker.update(cx, |state, cx| {
                        state.own = Some(next);
                        cx.notify();
                    });
                }
                close(window, cx);
                if let Some(notify) = &notify {
                    notify(next, window, cx);
                }
            })
        };

        // The trigger: the select's box, with the calendar glyph at its
        // leading edge and the span where the value goes.
        let lifted = open || focus_visible(&focus, window, cx);
        let glyph_ink = if self.disabled {
            t.color_content_faint.hsla()
        } else {
            t.color_content_subtle.hsla()
        };
        let mut trigger = FieldBox::new(self.size)
            .tint(self.tint)
            .state(self.state)
            .focused(lifted)
            .disabled(self.disabled)
            .style(div().id("trigger"), t)
            .w_auto()
            .min_w(px(16. * 16.))
            .gap(px(t.spacing_2))
            .tabular_nums()
            .child(icon_slot(
                t.icon_small,
                div()
                    .flex()
                    .flex_none()
                    .text_color(glyph_ink)
                    .child(Icon::new(glyphs::CALENDAR_LTR_16_REGULAR)),
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .map(|this| match &value {
                        Some(value) => this.child(summary(t, value, &self.strings)),
                        None => this
                            .text_color(t.color_content_faint.hsla())
                            .text_ellipsis()
                            .child(crate::foundation::text_child(
                                self.strings.placeholder.clone(),
                            )),
                    }),
            )
            .child(icon_slot(
                t.icon_small,
                div()
                    .flex()
                    .flex_none()
                    .text_color(glyph_ink)
                    .child(Icon::new(glyphs::CHEVRON_DOWN_12_REGULAR)),
            ));
        if self.disabled {
            trigger = trigger.cursor_not_allowed();
        } else {
            let click = toggle.clone();
            trigger = trigger
                .cursor_pointer()
                .track_focus(&focus.clone().tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .on_click(move |_, window, cx| click(window, cx));
        }

        // The sheet: the shared popup holding the months, the column beside
        // them and a footer, hairlined off each other. It drops the popup's
        // own padding and lets the parts pad themselves.
        let sheet = open.then(|| {
            let hairline = px(t.stroke_hairline);

            let calendar = {
                let picker = picker.clone();
                let month_picker = picker.clone();
                Calendar::new(kit::child_id(&id, "calendar"))
                    .range(draft.days)
                    .on_range_change(move |range, _, cx| {
                        picker.update(cx, |state, cx| {
                            state.draft.set_days(range);
                            cx.notify();
                        })
                    })
                    .size(Size::Small)
                    .months(self.months)
                    .month(draft.month)
                    .on_month_change(move |month, _, cx| {
                        month_picker.update(cx, |state, cx| {
                            state.draft.month = month;
                            cx.notify();
                        })
                    })
                    .week_starts_on(self.week_starts_on)
                    .strings(self.strings.calendar.clone())
                    .today(today)
                    .tint(self.tint)
            };

            // The column: the quick-pick menu's name on the first line, set
            // flush with the keys under it, then a key over a date and its
            // time, twice, then the durations.
            let quick = {
                let label = self.strings.presets.clone();
                let items = self.presets.iter().map(|preset| {
                    let picker = picker.clone();
                    let preset = preset.clone();
                    MenuItem::new(preset.label.clone())
                        .checked(draft.matches_preset(&preset, today))
                        .on_select(move |_, cx| {
                            let (start, end) = preset.range(today);
                            picker.update(cx, |state, cx| {
                                state.draft.month = start_of_month(start);
                                state.draft.set_days(DateRange::new(start, end));
                                cx.notify();
                            });
                        })
                });
                div()
                    .flex()
                    // A plain button pads its label by the profile's inline
                    // step; pulling the row back by it sets the name on the
                    // column's own edge.
                    .ml(px(-t.control_tiny_padding_inline))
                    .child(
                        Menu::new(kit::child_id(&id, "presets"))
                            .align(Align::Start)
                            .tint(self.tint)
                            .trigger(move |press, _, _, _| {
                                Button::new("quick")
                                    .size(Size::Tiny)
                                    .variant(Variant::Plain)
                                    .tint(Tint::Neutral)
                                    .on_click(move |_, window, cx| press(window, cx))
                                    .child(label)
                                    .glyph(Icon::new(glyphs::CHEVRON_DOWN_12_REGULAR))
                                    .into_any_element()
                            })
                            .items(items),
                    )
            };

            let end =
                |part: &'static str, label: &SharedString, at: NaiveDateTime, on_time: OnTime| {
                    let current = clock(at);
                    // A time list that starts on the step but still shows a
                    // value off it — a span handed in at 14:07 keeps 14:07
                    // until the reader moves it.
                    let mut options: Vec<SelectOption> = Vec::new();
                    if !(at.hour() * 60 + at.minute()).is_multiple_of(self.time_step) {
                        options.push(SelectOption::new(current.clone(), current.clone()));
                    }
                    for minutes in (0..24 * 60).step_by(self.time_step as usize) {
                        let label = format!("{:02}:{:02}", minutes / 60, minutes % 60);
                        options.push(SelectOption::new(label.clone(), label));
                    }
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(t.spacing_1))
                        .child(key(t, label))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(t.spacing_2))
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .overflow_hidden()
                                        .whitespace_nowrap()
                                        .type_style(t.label_quiet)
                                        .line_height(px(t.label_quiet.size))
                                        .tabular_nums()
                                        .child(crate::foundation::text_child((self.strings.day)(
                                            at.date(),
                                        ))),
                                )
                                .child(
                                    div().flex_none().min_w(px(5. * 16.)).child(
                                        Select::new(kit::child_id(&id, part))
                                            .size(Size::Small)
                                            .tint(self.tint)
                                            .options(options)
                                            .value(current)
                                            .disabled(draft.value.all_day)
                                            .on_value_change(move |time, _, cx| {
                                                let mut parts =
                                                    time.split(':').map(|n| n.parse::<u32>());
                                                if let (Some(Ok(h)), Some(Ok(m))) =
                                                    (parts.next(), parts.next())
                                                {
                                                    on_time(h, m, cx);
                                                }
                                            }),
                                    ),
                                ),
                        )
                };
            let start_time: OnTime = {
                let picker = picker.clone();
                Rc::new(move |h, m, cx| {
                    picker.update(cx, |state, cx| {
                        state.draft.set_start_time(h, m);
                        cx.notify();
                    })
                })
            };
            let end_time: OnTime = {
                let picker = picker.clone();
                Rc::new(move |h, m, cx| {
                    picker.update(cx, |state, cx| {
                        state.draft.set_end_time(h, m);
                        cx.notify();
                    })
                })
            };

            let chips = self.durations.iter().enumerate().map(|(index, duration)| {
                let picker = picker.clone();
                let minutes = duration.minutes;
                Toggle::new(kit::child_id(&id, &format!("duration-{index}")))
                    .size(Size::Tiny)
                    .variant(Variant::Outlined)
                    .tint(self.tint)
                    .pressed(draft.matches_duration(duration))
                    // A duration is a length within a day; a span across
                    // days has none.
                    .disabled(!draft.one_day())
                    .on_pressed_change(move |_, _, cx| {
                        picker.update(cx, |state, cx| {
                            state.draft.set_duration(minutes);
                            cx.notify();
                        })
                    })
                    .child(crate::foundation::text_child(duration.label.clone()))
                    .into_any_element()
            });
            let all_day = {
                let picker = picker.clone();
                Toggle::new(kit::child_id(&id, "all-day"))
                    .size(Size::Tiny)
                    .variant(Variant::Outlined)
                    .tint(self.tint)
                    .pressed(draft.value.all_day)
                    .on_pressed_change(move |all_day, _, cx| {
                        picker.update(cx, |state, cx| {
                            state.draft.value.all_day = all_day;
                            cx.notify();
                        })
                    })
                    .child(crate::foundation::text_child(self.strings.all_day.clone()))
                    .into_any_element()
            };
            let durations = div()
                .flex()
                .flex_col()
                .gap(px(t.spacing_1_5))
                .child(key(t, &self.strings.duration))
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(t.spacing_1))
                        .children(chips)
                        .child(all_day),
                );

            let side = div()
                .flex()
                .flex_col()
                .flex_none()
                .gap(px(t.spacing_3))
                .w(px(t.date_range_picker_side_width))
                .pt(px(t.spacing_2))
                .px(px(t.spacing_3))
                .pb(px(t.spacing_3))
                .border_l(hairline)
                .border_color(t.color_border.hsla())
                .child(quick)
                .child(end(
                    "start-time",
                    &self.strings.start,
                    draft.value.start,
                    start_time,
                ))
                .child(end(
                    "end-time",
                    &self.strings.end,
                    draft.value.end,
                    end_time,
                ))
                .child(durations);

            // A read-only line of what the sheet holds, and the two actions
            // at the trailing edge, as a dialog sets its own.
            let cancel = close.clone();
            let done = commit.clone();
            let foot = div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(t.spacing_3))
                .pt(px(t.spacing_2))
                .pr(px(t.spacing_2))
                .pb(px(t.spacing_2))
                .pl(px(t.spacing_3))
                .border_t(hairline)
                .border_color(t.color_border.hsla())
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .text_color(t.color_content_secondary.hsla())
                        .type_style(t.label_small)
                        .tabular_nums()
                        .child(summary(t, &draft.value, &self.strings)),
                )
                .child(
                    div()
                        .flex()
                        .flex_none()
                        .gap(px(t.spacing_2))
                        .child(
                            Button::new("cancel")
                                .variant(Variant::Normal)
                                .on_click(move |_, window, cx| cancel(window, cx))
                                .child(crate::foundation::text_child(self.strings.cancel.clone())),
                        )
                        .child(
                            Button::new("done")
                                .variant(Variant::Filled)
                                .tint(self.tint)
                                .on_click(move |_, window, cx| done(window, cx))
                                .child(crate::foundation::text_child(self.strings.done.clone())),
                        ),
                );

            let panel = sheet(t)
                .id("sheet")
                .p_0()
                .min_w_0()
                .type_style(t.body_small)
                .tabular_nums()
                .child(
                    div()
                        .flex()
                        .child(
                            div()
                                .flex_none()
                                .pt(px(t.spacing_1))
                                .px(px(t.spacing_2))
                                .pb(px(t.spacing_2))
                                .child(calendar),
                        )
                        .child(side),
                )
                .child(foot);
            let panel = dismissable(panel, &popup, &anchor, None, cx);
            positioned(
                &anchor,
                Placement::side(Side::Bottom, Align::Start, t),
                panel,
            )
        });

        div()
            .id(id)
            .flex()
            .flex_col()
            .child(measured(&anchor, trigger))
            .children(sheet)
    }
}
