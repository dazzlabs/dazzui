use super::state::Context;
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::Value;
use std::{cell::RefCell, rc::Rc};
fn items(value: &Value) -> Vec<ComboboxItem> {
    fn option(value: &Value) -> ComboboxOption {
        ComboboxOption {
            value: value["value"].as_str().unwrap_or("").into(),
            label: value["label"].as_str().unwrap_or("").into(),
            disabled: value["disabled"].as_bool().unwrap_or(false),
        }
    }
    value
        .as_array()
        .into_iter()
        .flatten()
        .map(|value| {
            if let Some(options) = value["options"].as_array() {
                ComboboxItem::Group {
                    label: value["label"].as_str().unwrap_or("").into(),
                    options: options.iter().map(option).collect(),
                }
            } else {
                ComboboxItem::Option(option(value))
            }
        })
        .collect()
}
fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
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
    let source = raw["props"].clone();
    let p = ctx.resolve(&source, &[]);
    let auto = raw["$el"] == "Autocomplete";
    let field = if auto {
        autocomplete(AutocompleteOptions {
            items: strings(&p["items"]),
            value: source
                .get("value")
                .map(|_| p["value"].as_str().unwrap_or("").into()),
            default_value: p["defaultValue"].as_str().unwrap_or("").into(),
            placeholder: p["placeholder"].as_str().unwrap_or("").into(),
            empty: p["empty"].as_str().unwrap_or("No matches").into(),
            size: size(&p),
            tint: tint(&p),
            mono: p["mono"].as_bool().unwrap_or(false),
            ..Default::default()
        })
    } else {
        Combobox::new(ComboboxOptions {
            options: items(&p["options"]),
            value: source
                .get("value")
                .map(|_| p["value"].as_str().map(str::to_owned)),
            default_value: p["defaultValue"].as_str().map(str::to_owned),
            placeholder: p["placeholder"].as_str().unwrap_or("Search…").into(),
            empty: p["empty"].as_str().unwrap_or("No matches").into(),
            size: size(&p),
            tint: tint(&p),
            ..Default::default()
        })
    };
    let previous = Rc::new(RefCell::new(
        p[if auto { "items" } else { "options" }].clone(),
    ));
    let binding = source.clone();
    ctx.bind(&field, move |field, ctx| {
        let p = ctx.resolve(&binding, &[]);
        let data = &p[if auto { "items" } else { "options" }];
        if *previous.borrow() != *data {
            previous.replace(data.clone());
            let options = if auto {
                strings(data)
                    .into_iter()
                    .map(|value| {
                        ComboboxItem::Option(ComboboxOption {
                            label: value.clone(),
                            value,
                            disabled: false,
                        })
                    })
                    .collect()
            } else {
                items(data)
            };
            field.set_options(&options);
        }
        if binding.get("value").is_some() {
            field.set_value(p["value"].as_str());
        }
        field.set_read_only(p["readOnly"].as_bool().unwrap_or(false));
        field.set_required(p["required"].as_bool().unwrap_or(false));
        field.set_sensitive(!p["disabled"].as_bool().unwrap_or(false));
        field.set_clearable(p["clearable"].as_bool().unwrap_or(true));
        field.set_empty(p["empty"].as_str().unwrap_or("No matches"));
        field
            .entry()
            .set_placeholder_text(Some(p["placeholder"].as_str().unwrap_or(if auto {
                ""
            } else {
                "Search…"
            })));
        let error = p["state"] == "error";
        field.popup().set_dazz_tint(tint(&p));
        if auto && p["mono"].as_bool().unwrap_or(false) {
            field.entry().add_css_class("dz-mono");
        } else {
            field.entry().remove_css_class("dz-mono");
        }
        field.set_dazz_size(size(&p));
        field.set_dazz_tint(if error { Tint::Danger } else { tint(&p) });
        field.set_dazz_invalid(error);
        field.entry().set_dazz_invalid(error);
    });
    let ctx = ctx.clone();
    field.connect_value_changed(move |_, edit| {
        ctx.run(
            &source["onValueChange"],
            &[edit
                .value
                .as_ref()
                .map(|value| Value::String(value.clone()))
                .unwrap_or(Value::Null)],
        );
    });
    Ok(field.upcast())
}
