use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::Value;

fn text(v: &Value) -> String {
    if let Some(s) = v.as_str() {
        return s.to_owned();
    }
    if let Some(children) = v
        .get("children")
        .and_then(Value::as_array)
        .or_else(|| v.as_array())
    {
        return children.iter().map(text).collect();
    }
    String::new()
}
fn string<'a>(p: &'a Value, k: &str, fallback: &'a str) -> &'a str {
    p[k].as_str().unwrap_or(fallback)
}
fn boolean(p: &Value, k: &str) -> bool {
    p[k].as_bool().unwrap_or(false)
}
fn number(p: &Value, k: &str, fallback: f64) -> f64 {
    p[k].as_f64().unwrap_or(fallback)
}
fn size(p: &Value) -> Size {
    match string(p, "size", "medium") {
        "tiny" => Size::Tiny,
        "small" => Size::Small,
        "large" => Size::Large,
        _ => Size::Medium,
    }
}
fn tint(p: &Value) -> Tint {
    match string(p, "tint", "primary") {
        "neutral" => Tint::Neutral,
        "info" => Tint::Info,
        "success" => Tint::Success,
        "warning" => Tint::Warning,
        "danger" => Tint::Danger,
        _ => Tint::Primary,
    }
}
fn variant(p: &Value, default: Variant) -> Variant {
    match string(p, "variant", "") {
        "filled" => Variant::Filled,
        "tinted" => Variant::Tinted,
        "outlined" => Variant::Outlined,
        "normal" => Variant::Normal,
        "raised" => Variant::Normal,
        "recessed" => Variant::Recessed,
        "plain" => Variant::Plain,
        _ => default,
    }
}
fn children(n: &Value) -> &[Value] {
    n["children"].as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn length(v: &str) -> Option<i32> {
    if let Some(v) = v.strip_suffix("rem") {
        v.parse::<f64>().ok().map(|v| (v * 16.).round() as i32)
    } else if let Some(v) = v.strip_suffix("px") {
        v.parse::<f64>().ok().map(|v| v.round() as i32)
    } else {
        None
    }
}
fn append(
    box_: &gtk::Box,
    nodes: &[Value],
    specimens: &mut Vec<(usize, gtk::Widget)>,
) -> Result<(), String> {
    for child in nodes {
        box_.append(&render(child, specimens)?);
    }
    Ok(())
}
/// Return an error for unmapped scenes; never quietly substitute a different control.
pub fn render(n: &Value, specimens: &mut Vec<(usize, gtk::Widget)>) -> Result<gtk::Widget, String> {
    if n.is_string() {
        return Ok(label(&text(n), "body-small").upcast());
    }
    let kind = n["$el"].as_str().ok_or("Expected a story node")?;
    let p = &n["props"];
    let s = size(p);
    let t = tint(p);
    let own = n["$at"].as_u64().map(|n| n as usize);
    let before = specimens.len();
    let w: gtk::Widget = match kind {
        "Row" | "Col" => {
            let row = kind == "Row";
            let b = box_(
                if row {
                    gtk::Orientation::Horizontal
                } else {
                    gtk::Orientation::Vertical
                },
                string(p, "gap", "3"),
            );
            append(&b, children(n), specimens)?;
            if let Some(width) = p["width"].as_str().and_then(length) {
                b.set_size_request(width, -1);
            }
            if p["width"] == "100%" {
                b.set_hexpand(true);
            }
            let align = match string(p, "align", if row { "center" } else { "stretch" }) {
                "center" => gtk::Align::Center,
                "end" => gtk::Align::End,
                "stretch" => gtk::Align::Fill,
                _ => gtk::Align::Start,
            };
            if row {
                for child in children_widgets(&b) {
                    child.set_valign(align);
                }
                match string(p, "justify", "start") {
                    "center" => b.set_halign(gtk::Align::Center),
                    "end" => b.set_halign(gtk::Align::End),
                    "between" => {
                        if let Some(first) = b.first_child() {
                            first.set_hexpand(true);
                            first.set_halign(gtk::Align::Fill);
                        }
                    }
                    _ => (),
                }
            } else {
                for child in children_widgets(&b) {
                    child.set_halign(align);
                }
            }
            b.upcast()
        }
        "Text" => {
            let b = box_(gtk::Orientation::Horizontal, "0");
            for node in children(n) {
                if node.is_string() {
                    let l = label(
                        &text(node),
                        if boolean(p, "strong") {
                            "label-strong"
                        } else {
                            "body-small"
                        },
                    );
                    if string(p, "tone", "primary") != "primary" {
                        l.add_css_class("dz-muted");
                    }
                    if p.get("tint").is_some() {
                        l.set_dazz_tint(t);
                        l.add_css_class("dz-tinted-text");
                    }
                    b.append(&l);
                } else {
                    b.append(&render(node, specimens)?);
                }
            }
            b.upcast()
        }
        "Button" => {
            let b = button(&text(n), variant(p, Variant::Normal), t, s);
            b.set_hexpand(boolean(p, "fullWidth"));
            b.upcast()
        }
        "Badge" => badge(&text(n), variant(p, Variant::Tinted), t, s).upcast(),
        "Card" => {
            let cv = match string(p, "variant", "sunken") {
                "raised" => CardVariant::Raised,
                "outlined" => CardVariant::Outlined,
                "tinted" => CardVariant::Tinted,
                _ => CardVariant::Sunken,
            };
            let b = card(cv, t, s);
            append(&b, children(n), specimens)?;
            b.upcast()
        }
        "Callout" => callout(string(p, "title", ""), &text(n), t).upcast(),
        "Checkbox" => {
            let b = checkbox(&text(n), t, s);
            b.set_active(boolean(p, "defaultChecked"));
            b.set_inconsistent(boolean(p, "indeterminate"));
            if let Some(note) = p["note"].as_str() {
                b.set_label(Some(&format!("{} {note}", text(n))));
            }
            b.upcast()
        }
        "Switch" => {
            let b = switch(t, s);
            b.set_active(boolean(p, "defaultChecked"));
            b.upcast()
        }
        "Toggle" => {
            let b = toggle(string(p, "label", ""), t, s);
            b.set_active(boolean(p, "defaultPressed"));
            b.set_dazz_variant(variant(p, Variant::Normal));
            b.upcast()
        }
        "TextField" => {
            let value = string(p, "defaultValue", "");
            let w: gtk::Widget = if boolean(p, "multiline") {
                let w = text_area();
                w.buffer().set_text(value);
                w.set_editable(!boolean(p, "readOnly"));
                let rows = number(p, "rows", 3.0);
                let metrics = w.pango_context().metrics(None, None);
                let leading =
                    (metrics.ascent() + metrics.descent()) as f64 / gtk::pango::SCALE as f64;
                w.set_size_request(-1, (rows * leading).ceil() as i32);
                if let Some(name) = p["aria-label"].as_str() {
                    w.set_dazz_label(name);
                }
                if let Some(placeholder) = p["placeholder"].as_str() {
                    let overlay = gtk::Overlay::new();
                    overlay.add_css_class("dz-text-area-container");
                    overlay.set_child(Some(&w));
                    let hint = label(placeholder, "body-small");
                    hint.add_css_class("dz-text-area-placeholder");
                    hint.set_can_target(false);
                    hint.set_halign(gtk::Align::Start);
                    hint.set_valign(gtk::Align::Start);
                    hint.set_visible(value.is_empty());
                    overlay.add_overlay(&hint);
                    let weak = hint.downgrade();
                    w.buffer().connect_changed(move |buffer| {
                        if let Some(hint) = weak.upgrade() {
                            hint.set_visible(buffer.char_count() == 0);
                        }
                    });
                    overlay.upcast()
                } else {
                    w.upcast()
                }
            } else {
                let w = text_field(string(p, "placeholder", ""), s);
                w.set_text(value);
                w.set_editable(!boolean(p, "readOnly"));
                if string(p, "type", "text") == "password" {
                    w.set_visibility(false);
                }
                match string(p, "type", "text") {
                    "email" => w.set_input_purpose(gtk::InputPurpose::Email),
                    "url" => w.set_input_purpose(gtk::InputPurpose::Url),
                    _ => (),
                }
                w.upcast()
            };
            w.set_dazz_invalid(string(p, "state", "") == "error");
            if boolean(p, "required") {
                w.update_property(&[gtk::accessible::Property::Required(true)]);
            }
            if boolean(p, "mono") {
                w.add_css_class("dz-mono");
            }
            w
        }
        "Select" => {
            let options = p["options"].as_array().ok_or("Select needs options")?;
            let labels: Vec<_> = options.iter().map(|o| string(o, "label", "")).collect();
            let w = select(&labels, s);
            if let Some(value) = p["defaultValue"].as_str() {
                if let Some(index) = options.iter().position(|o| o["value"] == value) {
                    w.set_selected(index as u32);
                }
            } else {
                w.set_selected(gtk::INVALID_LIST_POSITION);
            }
            w.upcast()
        }
        "Slider" => {
            let w = slider(0., 100., 1., t, s);
            w.set_value(number(p, "defaultValue", 0.));
            w.set_draw_value(boolean(p, "showValue"));
            w.upcast()
        }
        "Progress" => {
            let w = progress(number(p, "value", 0.) / 100., t, s);
            if boolean(p, "gradient") {
                w.add_css_class("dz-progress-gradient");
            }
            w.upcast()
        }
        "Spinner" => spinner(s).upcast(),
        "Divider" => {
            let w = divider(if string(p, "orientation", "horizontal") == "vertical" {
                gtk::Orientation::Vertical
            } else {
                gtk::Orientation::Horizontal
            });
            if let Some(tone) = p["tone"].as_str() {
                w.add_css_class(&format!("dz-divider-{tone}"));
            }
            w.upcast()
        }
        "Code" => code(&text(n), false).upcast(),
        "CodeBlock" => code(string(p, "code", ""), true).upcast(),
        "SectionLabel" => {
            let w = section_label(&text(n));
            if boolean(p, "faint") {
                w.add_css_class("dz-faint");
            }
            if p.get("tint").is_some() {
                w.set_dazz_tint(t);
                w.add_css_class("dz-tinted-text");
            }
            w.upcast()
        }
        "Link" => {
            let w = link(&text(n), string(p, "href", ""));
            w.set_dazz_tint(t);
            if boolean(p, "external") {
                w.set_tooltip_text(Some(string(p, "href", "")));
            }
            w.upcast()
        }
        "Collapsible" => {
            let b = box_(gtk::Orientation::Vertical, "2");
            append(&b, children(n), specimens)?;
            let w = collapsible(string(p, "title", ""), &b);
            w.set_expanded(boolean(p, "defaultOpen"));
            w.upcast()
        }
        "EmptyState" => {
            let action = p.get("action").map(|a| render(a, specimens)).transpose()?;
            empty_state(string(p, "title", ""), &text(n), action.as_ref()).upcast()
        }
        "FormField" => {
            let b = box_(gtk::Orientation::Vertical, "2");
            append(&b, children(n), specimens)?;
            b.set_dazz_invalid(string(p, "state", "") == "error");
            form_field(string(p, "label", ""), &b, p["hint"].as_str()).upcast()
        }
        "Preferences" => {
            let b = preferences();
            append(&b, children(n), specimens)?;
            b.upcast()
        }
        "PreferenceGroup" | "PreferenceSection" => {
            let b = preference_group(string(
                p,
                if kind == "PreferenceGroup" {
                    "title"
                } else {
                    "label"
                },
                "",
            ));
            if let Some(description) = p["description"].as_str() {
                b.append(&label(description, "body-small"));
            }
            append(&b, children(n), specimens)?;
            if let Some(footer) = p["footer"].as_str() {
                b.append(&label(footer, "body-small"));
            }
            b.upcast()
        }
        "PreferenceRow" => {
            let b = box_(gtk::Orientation::Horizontal, "2");
            append(&b, children(n), specimens)?;
            preference_row(string(p, "title", ""), p["subtitle"].as_str(), &b).upcast()
        }
        "RadioGroup" => {
            let b = box_(gtk::Orientation::Vertical, "2-5");
            let mut first = None;
            for option in p["options"].as_array().ok_or("RadioGroup needs options")? {
                let r = radio(string(option, "label", ""), first.as_ref(), t, s);
                r.set_active(option["value"] == p["defaultValue"]);
                r.set_sensitive(!boolean(option, "disabled"));
                if first.is_none() {
                    first = Some(r.clone());
                }
                b.append(&r);
            }
            b.upcast()
        }
        other => return Err(format!("Unmapped GTK4 component: {other}")),
    };
    if let Some(at) = own {
        specimens.insert(before, (at, w.clone()));
    }
    if boolean(p, "disabled") {
        w.set_sensitive(false);
    }
    if let Some(label) = p["aria-label"].as_str() {
        w.set_dazz_label(label);
    }
    Ok(w)
}
fn children_widgets(b: &gtk::Box) -> Vec<gtk::Widget> {
    let mut items = vec![];
    let mut child = b.first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        items.push(w);
    }
    items
}
