use super::{scene, state::Context};
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::Value;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
fn mode(value: &Value) -> ValidationMode {
    match value.as_str() {
        Some("onBlur") => ValidationMode::OnBlur,
        Some("onChange") => ValidationMode::OnChange,
        _ => ValidationMode::OnSubmit,
    }
}
fn tint(value: &Value) -> Tint {
    match value.as_str() {
        Some("neutral") => Tint::Neutral,
        Some("danger") => Tint::Danger,
        Some("success") => Tint::Success,
        Some("warning") => Tint::Warning,
        Some("info") => Tint::Info,
        _ => Tint::Primary,
    }
}
fn text_input(raw: &Value) -> Option<&Value> {
    if raw["$el"] == "TextField" {
        return Some(raw);
    }
    let mut found = None;
    for child in raw["children"].as_array().into_iter().flatten() {
        if let Some(input) = text_input(child) {
            if found.is_some() {
                return None;
            }
            found = Some(input);
        }
    }
    found
}
pub(super) fn field(
    raw: &Value,
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<gtk::Widget, String> {
    let control = box_(gtk::Orientation::Vertical, "2");
    for child in raw["children"].as_array().into_iter().flatten() {
        control.append(&scene::render_with(child, specimens, ctx)?);
    }
    let p = ctx.resolve(&raw["props"], &[]);
    let field = FormField::new(
        p["label"].as_str().unwrap_or(""),
        &control,
        p["hint"].as_str(),
        FieldRules::default(),
    );
    let input = text_input(raw)
        .map(|input| input["props"].clone())
        .unwrap_or_default();
    let source = raw["props"].clone();
    ctx.bind(&field, move |field, ctx| {
        let p = ctx.resolve(&source, &[]);
        let input = ctx.resolve(&input, &[]);
        field.set_rules(FieldRules {
            name: p["name"]
                .as_str()
                .or_else(|| input["name"].as_str())
                .map(str::to_owned),
            required: input["required"].as_bool().unwrap_or(false),
            constraint: match input["type"].as_str() {
                Some("email") => TextConstraint::Email,
                Some("url") => TextConstraint::Url,
                _ => TextConstraint::Text,
            },
            validation_mode: p.get("validationMode").map(mode),
        });
        field.set_label(p["label"].as_str().unwrap_or(""));
        field.set_hint(p["hint"].as_str());
        field.set_dazz_tint(if p["state"] == "error" {
            Tint::Danger
        } else {
            tint(&p["tint"])
        });
        field.set_invalid(p["state"] == "error");
        field.set_sensitive(!p["disabled"].as_bool().unwrap_or(false));
    });
    Ok(field.upcast())
}
pub(super) fn render(
    raw: &Value,
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<gtk::Widget, String> {
    let form = Form::new();
    for child in raw["children"].as_array().into_iter().flatten() {
        form.append(&scene::render_with(child, specimens, ctx)?);
    }
    fn fields(form: &Form, widget: &gtk::Widget) -> Result<(), String> {
        if let Some(field) = widget.downcast_ref::<FormField>() {
            form.register_field(field)?;
            return Ok(());
        }
        let mut child = widget.first_child();
        while let Some(node) = child {
            child = node.next_sibling();
            fields(form, &node)?;
        }
        Ok(())
    }
    fields(&form, form.upcast_ref())?;
    fn buttons(raw: &Value, form: &Form, specimens: &[(usize, gtk::Widget)]) {
        if raw["$el"] == "Button"
            && let Some(at) = raw["$at"].as_u64()
            && let Some(button) = specimens
                .iter()
                .find(|(index, _)| *index == at as usize)
                .and_then(|(_, widget)| widget.downcast_ref::<gtk::Button>())
        {
            match raw["props"]["type"].as_str() {
                Some("submit") => form.submit_button(button),
                Some("reset") => form.reset_button(button),
                _ => (),
            }
        }
        for child in raw["children"].as_array().into_iter().flatten() {
            buttons(child, form, specimens);
        }
    }
    buttons(raw, &form, specimens);
    form.capture_defaults();
    let source = raw["props"].clone();
    let binding = source.clone();
    let previous = Rc::new(RefCell::new(Value::Null));
    ctx.bind(&form, move |form, ctx| {
        let p = ctx.resolve(&binding, &[]);
        form.set_validation_mode(mode(&p["validationMode"]));
        if *previous.borrow() != p["errors"] {
            previous.replace(p["errors"].clone());
            let mut errors = BTreeMap::new();
            for (name, value) in p["errors"].as_object().into_iter().flatten() {
                let messages = if let Some(message) = value.as_str() {
                    if message.is_empty() {
                        vec![]
                    } else {
                        vec![message.into()]
                    }
                } else {
                    value
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                };
                errors.insert(name.clone(), messages);
            }
            form.set_errors(errors);
        }
    });
    let ctx = ctx.clone();
    form.connect_submit(move |_, values| {
        ctx.run(&source["onSubmit"], &[]);
        ctx.run(
            &source["onFormSubmit"],
            &[serde_json::to_value(values).unwrap()],
        );
    });
    Ok(form.upcast())
}
