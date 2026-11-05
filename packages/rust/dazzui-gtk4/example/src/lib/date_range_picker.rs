use super::{scene, state::Context};
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::{Value, json};
fn value(value: &Value) -> Result<Option<DateRangeValue>, String> {
    if value.is_null() {
        return Ok(None);
    }
    Ok(Some(DateRangeValue::from_local(
        value["start"]
            .as_str()
            .ok_or("Range start must be a date")?,
        value["end"].as_str().ok_or("Range end must be a date")?,
        value["allDay"].as_bool().unwrap_or(false),
    )?))
}
fn encoded(value: &DateRangeValue) -> Value {
    let date = |d: &gtk::glib::DateTime| {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            d.year(),
            d.month(),
            d.day_of_month(),
            d.hour(),
            d.minute()
        )
    };
    json!({"start":date(&value.start),"end":date(&value.end),"allDay":value.all_day})
}
fn labels(p: &Value) -> DateRangePickerLabels {
    let mut l = DateRangePickerLabels::default();
    let p = &p["labels"];
    for (key, target) in [
        ("placeholder", &mut l.placeholder),
        ("presets", &mut l.presets),
        ("start", &mut l.start),
        ("end", &mut l.end),
        ("duration", &mut l.duration),
        ("allDay", &mut l.all_day),
        ("cancel", &mut l.cancel),
        ("done", &mut l.done),
    ] {
        if let Some(v) = p[key].as_str() {
            *target = v.into();
        }
    }
    l
}
pub(super) fn render(raw: &Value, ctx: &Context) -> Result<gtk::Widget, String> {
    let source = raw["props"].clone();
    let p = ctx.resolve(&source, &[]);
    let defaults = DateRangePickerOptions::default();
    let presets = p["presets"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .map(|v| {
                    Ok(DateRangePreset {
                        label: v["label"]
                            .as_str()
                            .ok_or("Preset label must be text")?
                            .into(),
                        days: (
                            v["days"][0].as_i64().ok_or("Preset days must be offsets")? as i32,
                            v["days"][1].as_i64().ok_or("Preset days must be offsets")? as i32,
                        ),
                    })
                })
                .collect::<Result<Vec<_>, String>>()
        })
        .transpose()?;
    let durations = p["durations"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .map(|v| {
                    Ok(DateRangeDuration {
                        label: v["label"]
                            .as_str()
                            .ok_or("Duration label must be text")?
                            .into(),
                        minutes: v["minutes"].as_i64().ok_or("Duration must name minutes")? as i32,
                    })
                })
                .collect::<Result<Vec<_>, String>>()
        })
        .transpose()?
        .unwrap_or(defaults.durations);
    let picker = DateRangePicker::new(DateRangePickerOptions {
        value: source
            .get("value")
            .map(|_| value(&p["value"]))
            .transpose()?,
        default_value: value(&p["defaultValue"])?,
        presets,
        durations,
        time_step: p["timeStep"].as_u64().unwrap_or(15) as u16,
        months: p["months"].as_u64().unwrap_or(1) as usize,
        week_starts_on: p["weekStartsOn"].as_u64().unwrap_or(1) as u8,
        locale: p["locale"].as_str().map(str::to_owned),
        today: p["today"].as_str().and_then(CalendarDate::parse),
        labels: labels(&p),
        size: scene::size(&p, "medium"),
        tint: scene::tint(&p, "primary"),
        invalid: p["state"] == "error",
        disabled: p["disabled"].as_bool().unwrap_or(false),
    })?;
    let binding = source.clone();
    ctx.bind(&picker, move |picker, ctx| {
        let p = ctx.resolve(&binding, &[]);
        if let Ok(v) = binding.get("value").map(|_| value(&p["value"])).transpose() {
            picker.set_controlled_value(v);
        }
        picker.set_appearance(
            scene::size(&p, "medium"),
            scene::tint(&p, "primary"),
            p["state"] == "error",
        );
        picker.set_sensitive(!p["disabled"].as_bool().unwrap_or(false));
        if let Some(label) = p["aria-label"].as_str() {
            picker.set_dazz_label(label);
        }
    });
    let context = ctx.clone();
    picker.connect_value_change(move |_, v| context.run(&source["onValueChange"], &[encoded(v)]));
    Ok(picker.upcast())
}
