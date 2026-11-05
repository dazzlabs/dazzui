use super::{scene, state::Context};
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::{Value, json};
fn date(value: &Value) -> Option<CalendarDate> {
    value.as_str().and_then(CalendarDate::parse)
}
fn selection(value: &Value, mode: CalendarMode) -> CalendarValue {
    if mode == CalendarMode::Range {
        CalendarValue::Range(DateRange {
            start: date(&value["start"]),
            end: date(&value["end"]),
        })
    } else {
        CalendarValue::Single(date(value))
    }
}
fn encoded(date: CalendarDate) -> Value {
    Value::String(format!("{}T00:00", date.iso()))
}
fn value(value: CalendarValue) -> Value {
    match value {
        CalendarValue::Single(day) => day.map(encoded).unwrap_or(Value::Null),
        CalendarValue::Range(range) => {
            json!({"start":range.start.map(encoded),"end":range.end.map(encoded)})
        }
    }
}
pub(super) fn render(raw: &Value, ctx: &Context) -> Result<gtk::Widget, String> {
    let source = raw["props"].clone();
    let p = ctx.resolve(&source, &[]);
    let mode = if p["mode"] == "range" {
        CalendarMode::Range
    } else {
        CalendarMode::Single
    };
    let calendar = Calendar::new(CalendarOptions {
        mode,
        size: if p["size"] == "small" {
            CalendarSize::Small
        } else {
            CalendarSize::Medium
        },
        months: p["months"].as_u64().unwrap_or(1) as usize,
        value: source.get("value").map(|_| selection(&p["value"], mode)),
        default_value: source
            .get("defaultValue")
            .map(|_| selection(&p["defaultValue"], mode)),
        month: date(&p["month"]),
        default_month: date(&p["defaultMonth"]),
        min: date(&p["min"]),
        max: date(&p["max"]),
        week_starts_on: p["weekStartsOn"].as_u64().unwrap_or(1) as u8,
        locale: p["locale"].as_str().map(str::to_owned),
        today: date(&p["today"]),
        navigation: p["navigation"].as_bool().unwrap_or(true),
        tint: scene::tint(&p, "primary"),
    })?;
    let binding = source.clone();
    ctx.bind(&calendar, move |calendar, ctx| {
        let p = ctx.resolve(&binding, &[]);
        let _ = calendar
            .set_controlled_value(binding.get("value").map(|_| selection(&p["value"], mode)));
        calendar.set_controlled_month(date(&p["month"]));
        calendar.set_bounds(date(&p["min"]), date(&p["max"]));
        if let Some(today) = date(&p["today"]) {
            calendar.set_today(today);
        }
        calendar.set_size(if p["size"] == "small" {
            CalendarSize::Small
        } else {
            CalendarSize::Medium
        });
        calendar.set_dazz_tint(scene::tint(&p, "primary"));
        if let Some(label) = p["aria-label"].as_str() {
            calendar.set_dazz_label(label);
        } else {
            calendar.reset_property(gtk::AccessibleProperty::Label);
        }
    });
    let context = ctx.clone();
    let action = source["onValueChange"].clone();
    calendar.connect_value_change(move |_, next| context.run(&action, &[value(next)]));
    let context = ctx.clone();
    let action = source["onMonthChange"].clone();
    calendar.connect_month_change(move |_, next| context.run(&action, &[encoded(next)]));
    Ok(calendar.upcast())
}
