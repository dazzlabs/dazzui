//! A month grid, in one of two modes: one day, or a span of them.
//!
//! GPUI has no calendar to wear, so this is one of the few components that
//! writes its behaviour out, and the reason is written here: a grid's
//! keyboard is not a list's. The arrow keys move by a day and by a week,
//! Page Up and Page Down by a month, Home and End to the ends of the week,
//! and the whole grid is one tab stop that lands on the day last reached.
//! Picking is a press, Enter or Space; in range mode the first press sets
//! the start and the second the end, in either order, and while the end is
//! pending the day under the pointer or the keyboard previews the span.
//!
//! Every day is a control, so it wears the shared recipes: `tinted` inside
//! a range — the system's one drawing for a selection — and `filled` for the
//! day or the two days actually picked. At rest it is drawn as a row, in ink
//! with the neutral hover wash, the way a menu row is, because an accent
//! wash on every one of forty-two days would make the month shout.
//!
//! Dates are `chrono`'s: a day is a [`NaiveDate`], civil and local, since
//! the kit draws a calendar and does not do calendars.

use std::rc::Rc;

use chrono::{Datelike, Days, Local, Months, NaiveDate, Weekday};
use gpui::{
    AnyElement, App, Div, ElementId, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent,
    MouseButton, ParentElement, RenderOnce, SharedString, Stateful, StatefulInteractiveElement,
    Styled, Window, div, prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::components::icon::Icon;
use crate::components::icon_button::IconButton;
use crate::foundation::{
    ControlColors, Size, State, StyledExt, Tint, Variant, focus_visible, pointer_down,
};
use crate::generated::ThemeTokens;
use crate::glyphs;
use crate::theme::ActiveTheme;

/// A span of days. `end` is `None` while the first day has been picked and
/// the second has not — a range mid-pick is still a value, because the
/// picker around the calendar wants to show the start while the reader
/// chooses the end.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DateRange {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

impl DateRange {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Self {
        DateRange {
            start: Some(start),
            end: Some(end),
        }
    }

    /// The first day picked, the second still to come.
    pub fn is_pending(&self) -> bool {
        self.start.is_some() && self.end.is_none()
    }
}

/// One day, or a span of them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CalendarMode {
    #[default]
    Single,
    Range,
}

impl CalendarMode {
    pub fn parse(name: &str) -> Option<CalendarMode> {
        Some(match name {
            "single" => CalendarMode::Single,
            "range" => CalendarMode::Range,
            _ => return None,
        })
    }
}

/// The words a calendar is labelled with. English unless the product says
/// otherwise; the kit carries no locale of its own, and a product hands it
/// the formatters for the one it speaks.
#[derive(Clone)]
pub struct CalendarStrings {
    /// The month's name over its grid, e.g. `October 2026`.
    pub month_title: Rc<dyn Fn(NaiveDate) -> String>,
    /// The narrow name over a column.
    pub weekday: Rc<dyn Fn(Weekday) -> String>,
    pub previous_month: SharedString,
    pub next_month: SharedString,
}

pub(crate) const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub(crate) const WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

impl Default for CalendarStrings {
    fn default() -> Self {
        CalendarStrings {
            month_title: Rc::new(|month| {
                format!("{} {}", MONTHS[month.month0() as usize], month.year())
            }),
            weekday: Rc::new(|weekday| {
                WEEKDAYS[weekday.num_days_from_monday() as usize][..1].to_string()
            }),
            previous_month: "Previous month".into(),
            next_month: "Next month".into(),
        }
    }
}

/* Day arithmetic ------------------------------------------------------------ */

/// The first of the month.
pub fn start_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap_or(date)
}

/// The first of the month `months` away, so the day never overflows.
pub fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let first = start_of_month(date);
    let moved = if months >= 0 {
        first.checked_add_months(Months::new(months as u32))
    } else {
        first.checked_sub_months(Months::new(months.unsigned_abs()))
    };
    moved.unwrap_or(first)
}

/// `day` of `month`, or the month's last day when it has no such day.
pub fn same_day_in(month: NaiveDate, day: u32) -> NaiveDate {
    let first = start_of_month(month);
    let last = add_months(first, 1)
        .pred_opt()
        .map(|d| d.day())
        .unwrap_or(28);
    first.with_day(day.min(last)).unwrap_or(first)
}

/// Days later, or earlier when negative.
pub fn add_days(date: NaiveDate, days: i64) -> NaiveDate {
    let moved = if days >= 0 {
        date.checked_add_days(Days::new(days as u64))
    } else {
        date.checked_sub_days(Days::new(days.unsigned_abs()))
    };
    moved.unwrap_or(date)
}

/// How far into its week a day is, counting from the day the week starts.
fn offset_in_week(day: NaiveDate, week_starts_on: Weekday) -> i64 {
    ((day.weekday().num_days_from_sunday() + 7 - week_starts_on.num_days_from_sunday()) % 7) as i64
}

/// The six weeks a month grid draws, from the day the week starts on —
/// always forty-two days, so the grid never changes height between months.
pub fn month_grid(month: NaiveDate, week_starts_on: Weekday) -> Vec<NaiveDate> {
    let first = start_of_month(month);
    let start = add_days(first, -offset_in_week(first, week_starts_on));
    (0..42).map(|i| add_days(start, i)).collect()
}

/// The weekday React's `weekStartsOn` numbers: `0` is Sunday.
pub fn weekday_from_sunday(index: u32) -> Weekday {
    match index % 7 {
        0 => Weekday::Sun,
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        _ => Weekday::Sat,
    }
}

fn same_month(a: NaiveDate, b: NaiveDate) -> bool {
    a.year() == b.year() && a.month() == b.month()
}

/* The component ------------------------------------------------------------- */

type OnDay = Rc<dyn Fn(NaiveDate, &mut Window, &mut App)>;
type OnRange = Rc<dyn Fn(DateRange, &mut Window, &mut App)>;

/// A calendar.
#[derive(IntoElement)]
pub struct Calendar {
    id: ElementId,
    mode: CalendarMode,
    value: Option<Option<NaiveDate>>,
    default_value: Option<NaiveDate>,
    on_value_change: Option<OnDay>,
    range: Option<DateRange>,
    default_range: DateRange,
    on_range_change: Option<OnRange>,
    size: Size,
    months: usize,
    month: Option<NaiveDate>,
    default_month: Option<NaiveDate>,
    on_month_change: Option<OnDay>,
    min: Option<NaiveDate>,
    max: Option<NaiveDate>,
    week_starts_on: Weekday,
    strings: CalendarStrings,
    today: Option<NaiveDate>,
    navigation: bool,
    tint: Tint,
}

impl Calendar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Calendar {
            id: id.into(),
            mode: CalendarMode::Single,
            value: None,
            default_value: None,
            on_value_change: None,
            range: None,
            default_range: DateRange::default(),
            on_range_change: None,
            size: Size::Medium,
            months: 1,
            month: None,
            default_month: None,
            on_month_change: None,
            min: None,
            max: None,
            // Monday, as ISO counts the week.
            week_starts_on: Weekday::Mon,
            strings: CalendarStrings::default(),
            today: None,
            navigation: true,
            tint: Tint::Primary,
        }
    }

    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }

    /// Controlled, in single mode: the day picked, which a press only
    /// reports through [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: Option<NaiveDate>) -> Self {
        self.value = Some(value);
        self
    }

    /// Uncontrolled, in single mode: where it starts.
    pub fn default_value(mut self, value: NaiveDate) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(NaiveDate, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    /// Controlled, in range mode: the span picked. Sets the mode.
    pub fn range(mut self, range: DateRange) -> Self {
        self.mode = CalendarMode::Range;
        self.range = Some(range);
        self
    }

    /// Uncontrolled, in range mode: where it starts. Sets the mode.
    pub fn default_range(mut self, range: DateRange) -> Self {
        self.mode = CalendarMode::Range;
        self.default_range = range;
        self
    }

    pub fn on_range_change(
        mut self,
        handler: impl Fn(DateRange, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_range_change = Some(Rc::new(handler));
        self
    }

    /// `small` is the small cell — the step for a panel that holds two
    /// months; every other size is the regular control height per day.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// How many months stand side by side: one or two.
    pub fn months(mut self, months: usize) -> Self {
        self.months = months.clamp(1, 2);
        self
    }

    /// Controlled: the month shown — the first of two. Any day in it will do.
    pub fn month(mut self, month: NaiveDate) -> Self {
        self.month = Some(month);
        self
    }

    pub fn default_month(mut self, month: NaiveDate) -> Self {
        self.default_month = Some(month);
        self
    }

    pub fn on_month_change(
        mut self,
        handler: impl Fn(NaiveDate, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_month_change = Some(Rc::new(handler));
        self
    }

    /// Days before `min` are drawn but cannot be picked.
    pub fn min(mut self, min: NaiveDate) -> Self {
        self.min = Some(min);
        self
    }

    /// Days after `max` are drawn but cannot be picked.
    pub fn max(mut self, max: NaiveDate) -> Self {
        self.max = Some(max);
        self
    }

    pub fn week_starts_on(mut self, weekday: Weekday) -> Self {
        self.week_starts_on = weekday;
        self
    }

    pub fn strings(mut self, strings: CalendarStrings) -> Self {
        self.strings = strings;
        self
    }

    /// What the grid counts as today. The clock's, unless a test says
    /// otherwise.
    pub fn today(mut self, today: NaiveDate) -> Self {
        self.today = Some(today);
        self
    }

    /// Whether the head carries the chevrons that move the month. Off for a
    /// calendar whose host moves it, which then shows the month's name alone.
    pub fn navigation(mut self, navigation: bool) -> Self {
        self.navigation = navigation;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// The accessible name. GPUI has no accessibility tree to give it to.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

/// What the grid keeps between frames: the value and the month when the
/// caller leaves them to it, the day its one tab stop lands on, and the day
/// under the pointer.
struct CalendarState {
    value: Option<NaiveDate>,
    range: DateRange,
    month: Option<NaiveDate>,
    focused: Option<NaiveDate>,
    hovered: Option<NaiveDate>,
}

/// What a press and a key need to know, shared by every day of the grid.
struct Wiring {
    state: gpui::Entity<CalendarState>,
    focus: FocusHandle,
    mode: CalendarMode,
    controlled_value: bool,
    controlled_range: bool,
    controlled_month: bool,
    on_value_change: Option<OnDay>,
    on_range_change: Option<OnRange>,
    on_month_change: Option<OnDay>,
    min: Option<NaiveDate>,
    max: Option<NaiveDate>,
    months: usize,
    first_month: NaiveDate,
    span: DateRange,
    focused: NaiveDate,
    week_starts_on: Weekday,
}

impl Wiring {
    fn visible(&self, day: NaiveDate) -> bool {
        (0..self.months).any(|i| same_month(add_months(self.first_month, i as i32), day))
    }

    fn disabled(&self, day: NaiveDate) -> bool {
        self.min.is_some_and(|min| day < min) || self.max.is_some_and(|max| day > max)
    }

    fn set_month(&self, next: NaiveDate, window: &mut Window, cx: &mut App) {
        let first = start_of_month(next);
        if !self.controlled_month {
            self.state.update(cx, |state, cx| {
                state.month = Some(first);
                cx.notify();
            });
        }
        if let Some(handler) = &self.on_month_change {
            handler(first, window, cx);
        }
    }

    fn set_focused(&self, day: NaiveDate, cx: &mut App) {
        self.state.update(cx, |state, cx| {
            state.focused = Some(day);
            cx.notify();
        });
    }

    fn pick(&self, day: NaiveDate, window: &mut Window, cx: &mut App) {
        if self.disabled(day) {
            return;
        }
        match self.mode {
            CalendarMode::Range => {
                let span = self.span;
                let next = match span.start {
                    Some(start) if span.end.is_none() => {
                        if day < start {
                            DateRange::new(day, start)
                        } else {
                            DateRange::new(start, day)
                        }
                    }
                    _ => DateRange {
                        start: Some(day),
                        end: None,
                    },
                };
                if !self.controlled_range {
                    self.state.update(cx, |state, cx| {
                        state.range = next;
                        cx.notify();
                    });
                }
                if let Some(handler) = &self.on_range_change {
                    handler(next, window, cx);
                }
            }
            CalendarMode::Single => {
                if !self.controlled_value {
                    self.state.update(cx, |state, cx| {
                        state.value = Some(day);
                        cx.notify();
                    });
                }
                if let Some(handler) = &self.on_value_change {
                    handler(day, window, cx);
                }
            }
        }
    }

    fn move_focus(&self, day: NaiveDate, window: &mut Window, cx: &mut App) {
        self.set_focused(day, cx);
        if !self.visible(day) {
            // Reached past the shown months: slide them so the day is in the
            // first one going back, or the last one going forward.
            let forward = start_of_month(day) > self.first_month;
            let month = if forward {
                add_months(day, 1 - self.months as i32)
            } else {
                day
            };
            self.set_month(month, window, cx);
        }
    }

    /// The grid's keys. Whether the key was one of them.
    fn key(&self, event: &KeyDownEvent, window: &mut Window, cx: &mut App) -> bool {
        let day = self.focused;
        let shift = event.keystroke.modifiers.shift;
        let offset = offset_in_week(day, self.week_starts_on);
        let next = match event.keystroke.key.as_str() {
            "left" => add_days(day, -1),
            "right" => add_days(day, 1),
            "up" => add_days(day, -7),
            "down" => add_days(day, 7),
            "pageup" => same_day_in(add_months(day, if shift { -12 } else { -1 }), day.day()),
            "pagedown" => same_day_in(add_months(day, if shift { 12 } else { 1 }), day.day()),
            "home" => add_days(day, -offset),
            "end" => add_days(day, 6 - offset),
            "enter" | "space" => {
                self.pick(day, window, cx);
                return true;
            }
            _ => return false,
        };
        self.move_focus(next, window, cx);
        true
    }
}

/// The two days of a span in order — only when both are set.
fn ordered(a: Option<NaiveDate>, b: Option<NaiveDate>) -> (Option<NaiveDate>, Option<NaiveDate>) {
    match (a, b) {
        (Some(a), Some(b)) => (Some(a.min(b)), Some(a.max(b))),
        _ => (None, None),
    }
}

/// How a day is drawn, worked out from the value, the preview and the month.
struct DayState {
    picked: bool,
    in_span: bool,
    span_start: bool,
    span_end: bool,
    previewed: bool,
    outside: bool,
    today: bool,
    disabled: bool,
    focused: bool,
}

impl RenderOnce for Calendar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let today = self.today.unwrap_or_else(|| Local::now().date_naive());
        let state = window.use_keyed_state(kit::child_id(&self.id, "state"), cx, {
            let value = self.default_value;
            let range = self.default_range;
            let month = self.default_month.map(start_of_month);
            move |_, _| CalendarState {
                value,
                range,
                month,
                focused: None,
                hovered: None,
            }
        });
        let focus = kit::focus_handle(&self.id, window, cx);
        let (held_value, held_range, held_month, held_focused, hovered) = {
            let state = state.read(cx);
            (
                state.value,
                state.range,
                state.month,
                state.focused,
                state.hovered,
            )
        };

        // The value, controlled or not, read as a range either way: a single
        // day is a range whose two ends are the same day.
        let span = match self.mode {
            CalendarMode::Range => self.range.unwrap_or(held_range),
            CalendarMode::Single => {
                let day = self.value.unwrap_or(held_value);
                DateRange {
                    start: day,
                    end: day,
                }
            }
        };
        let pending = self.mode == CalendarMode::Range && span.is_pending();
        let first_month = start_of_month(self.month.or(held_month).or(span.start).unwrap_or(today));
        let focused = held_focused.or(span.start).unwrap_or(today);

        let wiring = Rc::new(Wiring {
            state: state.clone(),
            focus: focus.clone(),
            mode: self.mode,
            controlled_value: self.value.is_some(),
            controlled_range: self.range.is_some(),
            controlled_month: self.month.is_some(),
            on_value_change: self.on_value_change.clone(),
            on_range_change: self.on_range_change.clone(),
            on_month_change: self.on_month_change.clone(),
            min: self.min,
            max: self.max,
            months: self.months,
            first_month,
            span,
            focused,
            week_starts_on: self.week_starts_on,
        });

        // What the span to draw is: the two picked ends, or the one picked
        // end and the day being previewed against it.
        let preview = pending.then(|| hovered.unwrap_or(focused));
        let (lo, hi) = match self.mode {
            CalendarMode::Range => ordered(span.start, if pending { preview } else { span.end }),
            CalendarMode::Single => (None, None),
        };
        let focus_shown = focus_visible(&focus, window, cx);

        let cell = if self.size == Size::Small {
            t.calendar_small_cell_size
        } else {
            t.calendar_medium_cell_size
        };
        let gap = t.calendar_cell_gap;
        let width = 7. * cell + 6. * gap;

        let months: Vec<AnyElement> = (0..self.months)
            .map(|index| {
                let current = add_months(first_month, index as i32);
                let days = month_grid(current, self.week_starts_on);
                let mut rows: Vec<AnyElement> = Vec::new();
                for row in 0..6 {
                    let mut cells: Vec<AnyElement> = Vec::new();
                    for column in 0..7 {
                        let day = days[row * 7 + column];
                        let day_state = DayState {
                            picked: match self.mode {
                                CalendarMode::Range => {
                                    Some(day) == span.start || Some(day) == span.end
                                }
                                CalendarMode::Single => Some(day) == span.start,
                            },
                            in_span: lo.is_some_and(|lo| day >= lo)
                                && hi.is_some_and(|hi| day <= hi),
                            span_start: Some(day) == lo,
                            span_end: Some(day) == hi,
                            previewed: pending && preview == Some(day) && Some(day) != span.start,
                            outside: !same_month(day, current),
                            today: day == today,
                            disabled: wiring.disabled(day),
                            focused: focus_shown && day == focused,
                        };
                        cells.push(
                            day_cell(
                                &self.id,
                                index * 42 + row * 7 + column,
                                day,
                                day_state,
                                cell,
                                self.tint,
                                self.mode,
                                &wiring,
                                t,
                            )
                            .into_any_element(),
                        );
                    }
                    rows.push(div().flex().gap(px(gap)).children(cells).into_any_element());
                }

                // The bar: a chevron at each end of the month's name, in the
                // tiny glyph square. A middle month of a run has neither, and
                // keeps the room.
                let spacer = || {
                    div()
                        .flex_none()
                        .size(px(t.control_tiny_size))
                        .into_any_element()
                };
                let mut head = div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(t.spacing_1));
                if self.navigation {
                    head = head.child(if index == 0 {
                        let wiring = wiring.clone();
                        IconButton::new(
                            kit::child_id(&self.id, "previous"),
                            self.strings.previous_month.clone(),
                        )
                        .on_click(move |_, window, cx| {
                            wiring.set_month(add_months(wiring.first_month, -1), window, cx)
                        })
                        .child(Icon::new(glyphs::CHEVRON_LEFT_12_REGULAR))
                        .into_any_element()
                    } else {
                        spacer()
                    });
                }
                head = head.child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_center()
                        .whitespace_nowrap()
                        .type_style(t.label_medium)
                        .tabular_nums()
                        .text_color(t.color_content.hsla())
                        .child(crate::foundation::text_child((self.strings.month_title)(
                            current,
                        ))),
                );
                if self.navigation {
                    head = head.child(if index == self.months - 1 {
                        let wiring = wiring.clone();
                        IconButton::new(
                            kit::child_id(&self.id, "next"),
                            self.strings.next_month.clone(),
                        )
                        .on_click(move |_, window, cx| {
                            wiring.set_month(add_months(wiring.first_month, 1), window, cx)
                        })
                        .child(Icon::new(glyphs::CHEVRON_RIGHT_12_REGULAR))
                        .into_any_element()
                    } else {
                        spacer()
                    });
                }

                let weekdays = div().flex().gap(px(gap)).children((0..7).map(|i| {
                    let weekday =
                        weekday_from_sunday(self.week_starts_on.num_days_from_sunday() + i);
                    div()
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .w(px(cell))
                        .h(px(t.spacing_5))
                        .text_color(t.color_content_subtle.hsla())
                        .font_family(t.caption_small.face.family())
                        .text_size(px(t.caption_small.size))
                        .font_weight(t.label_small.font_weight())
                        .line_height(px(t.caption_small.size))
                        .child(crate::foundation::text_child((self.strings.weekday)(
                            weekday,
                        )))
                }));

                div()
                    .flex()
                    .flex_col()
                    .flex_none()
                    .gap(px(t.spacing_1))
                    .w(px(width))
                    .child(head)
                    .child(weekdays)
                    .child(div().flex().flex_col().gap(px(gap)).children(rows))
                    .into_any_element()
            })
            .collect();

        let keys = wiring.clone();
        let leave = state.clone();
        div()
            .id(self.id)
            .flex()
            .items_start()
            .gap(px(t.spacing_4))
            .text_color(t.color_content.hsla())
            // One tab stop for the whole grid, which lands on the day last
            // reached.
            .track_focus(&focus.tab_stop(true))
            .on_key_down(move |event, window, cx| {
                if keys.key(event, window, cx) {
                    cx.stop_propagation();
                }
            })
            .on_hover(move |hovered, _, cx| {
                if !*hovered {
                    leave.update(cx, |state, cx| {
                        if state.hovered.take().is_some() {
                            cx.notify();
                        }
                    });
                }
            })
            .children(months)
    }
}

/// One day of the grid: a square at the cell size, drawn by its state.
#[allow(clippy::too_many_arguments)]
fn day_cell(
    id: &ElementId,
    index: usize,
    day: NaiveDate,
    state: DayState,
    cell: f32,
    tint: Tint,
    mode: CalendarMode,
    wiring: &Rc<Wiring>,
    t: &'static ThemeTokens,
) -> Stateful<Div> {
    let ramp = tint.ramp(t);
    let radius = t.radius_small;
    let off = state.disabled;

    // Colour comes off the shared recipes where the day is part of the
    // selection; at rest it is the row's drawing — ink on nothing, with the
    // neutral wash under the pointer.
    let (surface, content, hovered, pressed) = if state.picked || state.in_span {
        let variant = if state.picked {
            Variant::Filled
        } else {
            Variant::Tinted
        };
        let normal = ControlColors::resolve(t, variant, tint, State::Normal);
        let hovered = ControlColors::resolve(t, variant, tint, State::Hovered);
        let pressed = ControlColors::resolve(t, variant, tint, State::Pressed);
        (
            normal.surface,
            normal.content,
            (hovered.surface, hovered.content),
            (pressed.surface, pressed.content),
        )
    } else {
        let content = if off || state.outside {
            t.color_content_faint.hsla()
        } else if state.today {
            ramp.shade(600).hsla()
        } else {
            t.color_content.hsla()
        };
        (
            gpui::transparent_black(),
            content,
            (
                t.control_color_normal_surface_hovered.resolve(ramp),
                content,
            ),
            (
                t.control_color_normal_surface_pressed.resolve(ramp),
                content,
            ),
        )
    };
    // Today is a weight heavier, and a picked day the selection's weight —
    // the half of a selection that survives on a monochrome theme.
    let weight = if state.today {
        t.label_strong.font_weight()
    } else if state.picked {
        t.label_medium.font_weight()
    } else {
        t.label_quiet.font_weight()
    };

    // The span between the two ends is one band: the days inside square
    // their corners, and the two ends keep the corner only on the outer
    // side, so the band's edge stays concentric with the day's.
    let start_radius = if !state.in_span || state.span_start {
        radius
    } else {
        0.
    };
    let end_radius = if !state.in_span || state.span_end {
        radius
    } else {
        0.
    };
    let corners = move |element: Div| {
        element
            .rounded_tl(px(start_radius))
            .rounded_bl(px(start_radius))
            .rounded_tr(px(end_radius))
            .rounded_br(px(end_radius))
    };

    let mut cell_div = corners(div())
        .id(kit::child_id(id, &format!("day-{index}")))
        .relative()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(cell))
        .bg(surface)
        .text_color(content)
        .font_family(t.label_quiet.face.family())
        .text_size(px(t.label_quiet.size))
        .line_height(px(t.label_quiet.size))
        .font_weight(weight)
        .tabular_nums()
        .child(crate::foundation::text_child(day.day().to_string()));

    if state.today {
        // Today's dot, in the ink the recipe gave the label, so today is
        // never lost under a selection.
        cell_div = cell_div.child(
            div()
                .absolute()
                .left_0()
                .right_0()
                .bottom(px(t.spacing_0_5))
                .flex()
                .justify_center()
                .child(div().size(px(t.spacing_0_5)).rounded_full().bg(content)),
        );
    }
    if state.previewed {
        // The end a pending range is previewing against: dashed, because it
        // is where the span would end, not where it does.
        cell_div = cell_div.child(
            corners(div())
                .absolute()
                .inset_0()
                .border(px(t.stroke_hairline))
                .border_dashed()
                .border_color(ramp.shade(600).hsla()),
        );
    }
    if state.focused {
        cell_div = cell_div.child(kit::outline(t, radius));
    }

    if off {
        cell_div = cell_div.cursor_not_allowed();
    } else {
        cell_div = cell_div
            .cursor_pointer()
            .hover(move |style| style.bg(hovered.0).text_color(hovered.1))
            .active(move |style| style.bg(pressed.0).text_color(pressed.1));
    }

    let focus = wiring.focus.clone();
    let click = wiring.clone();
    let hover = wiring.state.clone();
    cell_div
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            // A press lands the grid's tab stop on the day, as a click
            // focuses a button.
            pointer_down(window, cx);
            window.focus(&focus);
        })
        .on_click(move |_, window, cx| {
            click.set_focused(day, cx);
            click.pick(day, window, cx);
        })
        .when(mode == CalendarMode::Range, |this| {
            this.on_hover(move |over, _, cx| {
                if *over {
                    hover.update(cx, |state, cx| {
                        if state.hovered != Some(day) {
                            state.hovered = Some(day);
                            cx.notify();
                        }
                    });
                }
            })
        })
}
