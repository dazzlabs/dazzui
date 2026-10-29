//! The calendar family: a month grid and the picker built around one.
//!
//! A story writes a date as local `YYYY-MM-DDTHH:MM`, the way codegen writes
//! `new Date(2026, 9, 16, 14, 30)`, and a slot a calendar writes back takes
//! the same text, so a round trip through a scene's state reads as it was
//! written.

use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use dazzui::{
    Calendar, CalendarMode, DateRange, DateRangeDuration, DateRangePicker, DateRangePreset,
    DateRangeValue, FieldState, Size, Tint, weekday_from_sunday,
};
use gpui::{AnyElement, App, IntoElement, Window};
use serde_json::{Map, Value};

use super::BuildFn;
use crate::lib::scene::{Node, text};

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("Calendar", calendar);
    map.insert("DateRangePicker", date_range_picker);
}

const FORMAT: &str = "%Y-%m-%dT%H:%M";

/// A date as a story wrote it.
fn read_moment(value: &Value) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(value.as_str()?, FORMAT).ok()
}

fn read_day(value: &Value) -> Option<NaiveDate> {
    read_moment(value).map(|at| at.date())
}

/// A date as a story reads one back.
fn write_moment(at: NaiveDateTime) -> Value {
    Value::String(at.format(FORMAT).to_string())
}

fn write_day(day: NaiveDate) -> Value {
    write_moment(day.and_hms_opt(0, 0, 0).unwrap_or_default())
}

fn read_range(value: &Value) -> DateRange {
    DateRange {
        start: read_day(&value["start"]),
        end: read_day(&value["end"]),
    }
}

fn write_range(range: DateRange) -> Value {
    let mut object = Map::new();
    object.insert(
        "start".into(),
        range.start.map(write_day).unwrap_or(Value::Null),
    );
    object.insert(
        "end".into(),
        range.end.map(write_day).unwrap_or(Value::Null),
    );
    Value::Object(object)
}

fn read_value(value: &Value) -> Option<DateRangeValue> {
    Some(
        DateRangeValue::new(read_moment(&value["start"])?, read_moment(&value["end"])?)
            .all_day(value["allDay"].as_bool().unwrap_or(false)),
    )
}

fn write_value(value: DateRangeValue) -> Value {
    let mut object = Map::new();
    object.insert("start".into(), write_moment(value.start));
    object.insert("end".into(), write_moment(value.end));
    object.insert("allDay".into(), Value::Bool(value.all_day));
    Value::Object(object)
}

fn calendar(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mode = node.option("mode", CalendarMode::parse).unwrap_or_default();
    let mut calendar = Calendar::new(node.id())
        .mode(mode)
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .months(node.number("months").unwrap_or(1.0) as usize)
        .navigation(!node.has("navigation") || node.flag("navigation"))
        .tint(node.option("tint", Tint::parse).unwrap_or_default());
    if let Some(today) = read_day(&node.get("today")) {
        calendar = calendar.today(today);
    }
    if let Some(min) = read_day(&node.get("min")) {
        calendar = calendar.min(min);
    }
    if let Some(max) = read_day(&node.get("max")) {
        calendar = calendar.max(max);
    }
    if let Some(start) = node.number("weekStartsOn") {
        calendar = calendar.week_starts_on(weekday_from_sunday(start as u32));
    }
    if let Some(label) = node.string("aria-label") {
        calendar = calendar.aria_label(label);
    }
    match mode {
        CalendarMode::Single => {
            if node.has("value") {
                calendar = calendar.value(read_day(&node.get("value")));
            } else if let Some(day) = read_day(&node.get("defaultValue")) {
                calendar = calendar.default_value(day);
            }
            if let Some(handler) = node.handler("onValueChange") {
                calendar = calendar
                    .on_value_change(move |day, window, cx| handler(write_day(day), window, cx));
            }
        }
        CalendarMode::Range => {
            if node.has("value") {
                calendar = calendar.range(read_range(&node.get("value")));
            } else if node.has("defaultValue") {
                calendar = calendar.default_range(read_range(&node.get("defaultValue")));
            }
            if let Some(handler) = node.handler("onValueChange") {
                calendar = calendar.on_range_change(move |range, window, cx| {
                    handler(write_range(range), window, cx)
                });
            }
        }
    }
    calendar.into_any_element()
}

fn date_range_picker(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut picker = DateRangePicker::new(node.id())
        .months(node.number("months").unwrap_or(1.0) as usize)
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .state(node.option("state", FieldState::parse).unwrap_or_default())
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .disabled(node.flag("disabled"));
    if node.has("value") {
        picker = picker.value(read_value(&node.get("value")));
    } else if let Some(value) = read_value(&node.get("defaultValue")) {
        picker = picker.default_value(value);
    }
    if let Some(handler) = node.handler("onValueChange") {
        picker = picker
            .on_value_change(move |value, window, cx| handler(write_value(value), window, cx));
    }
    // A preset is data here: a label and two whole-day offsets from today.
    if let Some(presets) = node.get("presets").as_array() {
        picker = picker.presets(
            presets
                .iter()
                .map(|preset| {
                    let days = preset["days"].as_array();
                    let offset = |index: usize| {
                        days.and_then(|days| days.get(index))
                            .and_then(Value::as_i64)
                            .unwrap_or(0)
                    };
                    DateRangePreset::days(text(&preset["label"]), offset(0), offset(1))
                })
                .collect(),
        );
    }
    if let Some(durations) = node.get("durations").as_array() {
        picker = picker.durations(
            durations
                .iter()
                .map(|duration| {
                    DateRangeDuration::new(
                        text(&duration["label"]),
                        duration["minutes"].as_i64().unwrap_or(60),
                    )
                })
                .collect(),
        );
    }
    if let Some(step) = node.number("timeStep") {
        picker = picker.time_step(step as u32);
    }
    if let Some(start) = node.number("weekStartsOn") {
        picker = picker.week_starts_on(weekday_from_sunday(start as u32));
    }
    if let Some(today) = read_day(&node.get("today")) {
        picker = picker.today(today);
    }
    if let Some(label) = node.string("aria-label") {
        picker = picker.aria_label(label);
    }
    picker.into_any_element()
}
