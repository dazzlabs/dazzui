use super::state::Context;
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::Value;
fn values(value: &Value, fallback: f64) -> Vec<f64> {
    value
        .as_array()
        .map(|values| values.iter().filter_map(Value::as_f64).collect())
        .unwrap_or_else(|| vec![value.as_f64().unwrap_or(fallback)])
}
fn current(props: &Value) -> &Value {
    props
        .get("value")
        .or_else(|| props.get("defaultValue"))
        .unwrap_or(&Value::Null)
}
fn tint(props: &Value) -> Tint {
    match props["tint"].as_str().unwrap_or("primary") {
        "neutral" => Tint::Neutral,
        "info" => Tint::Info,
        "success" => Tint::Success,
        "warning" => Tint::Warning,
        "danger" => Tint::Danger,
        _ => Tint::Primary,
    }
}
fn size(props: &Value) -> Size {
    match props["size"].as_str().unwrap_or("medium") {
        "small" => Size::Small,
        "large" => Size::Large,
        _ => Size::Medium,
    }
}
pub(super) fn render(raw: &Value, ctx: &Context) -> Result<gtk::Widget, String> {
    let source = &raw["props"];
    let p = ctx.resolve(source, &[]);
    let min = p["min"].as_f64().unwrap_or(0.);
    let field = slider_field(SliderOptions {
        value: source.get("value").map(|_| values(&p["value"], min)),
        default_values: values(&p["defaultValue"], min),
        range: current(&p).is_array(),
        min,
        max: p["max"].as_f64().unwrap_or(100.),
        step: p["step"].as_f64().unwrap_or(1.),
        large_step: p["largeStep"].as_f64().unwrap_or(10.),
        min_steps_between_values: p["minStepsBetweenValues"].as_f64().unwrap_or(0.),
        collision: if p["thumbCollisionBehavior"] == "none" {
            SliderCollision::None
        } else {
            SliderCollision::Push
        },
        size: size(&p),
        tint: tint(&p),
        show_value: p["showValue"].as_bool().unwrap_or(false),
        disabled: p["disabled"].as_bool().unwrap_or(false),
    })?;
    let source = source.clone();
    let binding = source.clone();
    ctx.bind(&field, move |field, ctx| {
        let p = ctx.resolve(&binding, &[]);
        if binding.get("value").is_some() {
            let _ = field.set_values(&values(&p["value"], min));
            field.set_range(current(&p).is_array());
        }
        field.set_show_value(p["showValue"].as_bool().unwrap_or(false));
        field.set_sensitive(!p["disabled"].as_bool().unwrap_or(false));
        field.set_dazz_tint(tint(&p));
        field.set_dazz_size(size(&p));
        if let Some(label) = p["aria-label"].as_str() {
            for scale in field.scales() {
                scale.set_dazz_label(label);
            }
        }
    });
    let context = ctx.clone();
    let change = source.clone();
    field.connect_value_changed(move |_, edit| {
        let p = context.resolve(&change, &[]);
        let value = if current(&p).is_array() {
            serde_json::json!(edit.values)
        } else {
            serde_json::json!(edit.values[0])
        };
        context.run(
            &change["onValueChange"],
            &[
                value,
                serde_json::json!({"reason":edit.reason,"activeThumbIndex":edit.active_thumb}),
            ],
        );
    });
    let context = ctx.clone();
    field.connect_value_committed(move |_, values, reason| {
        let p = context.resolve(&source, &[]);
        let value = if current(&p).is_array() {
            serde_json::json!(values)
        } else {
            serde_json::json!(values[0])
        };
        context.run(
            &source["onValueCommitted"],
            &[value, serde_json::json!({"reason":reason})],
        );
    });
    Ok(field.upcast())
}
