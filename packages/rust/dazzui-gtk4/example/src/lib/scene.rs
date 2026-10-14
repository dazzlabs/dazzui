use super::{state::Context, story::Scene};
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
fn size(p: &Value, fallback: &str) -> Size {
    match string(p, "size", fallback) {
        "tiny" => Size::Tiny,
        "small" => Size::Small,
        "large" => Size::Large,
        _ => Size::Medium,
    }
}
fn tint(p: &Value, fallback: &str) -> Tint {
    match string(p, "tint", fallback) {
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
pub(super) fn select_items(value: &Value) -> Vec<SelectItem> {
    let option = |item: &Value| SelectOption {
        value: string(item, "value", "").to_owned(),
        label: string(item, "label", "").to_owned(),
        disabled: boolean(item, "disabled"),
    };
    value
        .as_array()
        .into_iter()
        .flatten()
        .map(|item| {
            if let Some(options) = item["options"].as_array() {
                SelectItem::Group {
                    label: string(item, "label", "").to_owned(),
                    options: options.iter().map(option).collect(),
                }
            } else {
                SelectItem::Option(option(item))
            }
        })
        .collect()
}
fn append(
    box_: &gtk::Box,
    nodes: &[Value],
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<(), String> {
    for child in nodes {
        box_.append(&render_with(child, specimens, ctx)?);
    }
    Ok(())
}
fn button_children(
    b: &gtk::Box,
    nodes: &[Value],
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<(), String> {
    for node in nodes {
        if let Some(text) = node.as_str() {
            b.append(&gtk::Label::new(Some(text)));
        } else {
            b.append(&render_with(node, specimens, ctx)?);
        }
    }
    Ok(())
}
/// Return an error for unmapped scenes; never quietly substitute a different control.
pub fn render(n: &Value, specimens: &mut Vec<(usize, gtk::Widget)>) -> Result<gtk::Widget, String> {
    let ctx = Context::new(&Value::Null);
    render_with(n, specimens, &ctx)
}
pub fn render_scene(
    scene: &Scene,
    specimens: &mut Vec<(usize, gtk::Widget)>,
) -> Result<gtk::Widget, String> {
    let ctx = Context::new(&scene.state);
    let root = render_with(&scene.content, specimens, &ctx)?;
    root.connect_destroy(move |_| {
        let _ = &ctx;
    });
    Ok(root)
}
fn render_with(
    raw: &Value,
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<gtk::Widget, String> {
    let resolved = ctx.resolve(raw, &[]);
    let n = &resolved;
    if n.is_string() {
        let w = label(&text(n), "body-small");
        if raw != n {
            let raw = raw.clone();
            ctx.bind(&w, move |w, ctx| {
                w.set_label(&super::state::text(&ctx.resolve(&raw, &[])))
            });
        }
        return Ok(w.upcast());
    }
    if let Some(name) = n["$icon"].as_str() {
        return Ok(icon(name).upcast());
    }
    let kind = n["$el"].as_str().ok_or("Expected a story node")?;
    let p = &n["props"];
    let s = size(
        p,
        match kind {
            "Button" | "Progress" => "small",
            "IconButton" | "Toggle" => "tiny",
            _ => "medium",
        },
    );
    let t = tint(
        p,
        match kind {
            "IconButton" | "Toggle" | "Callout" => "neutral",
            _ => "primary",
        },
    );
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
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
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
                let kids = children_widgets(&b);
                if !kids.is_empty()
                    && kids
                        .iter()
                        .all(|child| child.has_css_class("dz-option-card"))
                {
                    b.set_homogeneous(true);
                }
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
            for node in children(raw) {
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
                    b.append(&render_with(node, specimens, ctx)?);
                }
            }
            b.upcast()
        }
        "Button" => {
            let b = button("", variant(p, Variant::Recessed), t, s);
            let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            content.add_css_class("dz-button-content");
            // Match React's glyph-end trims without flattening nested kit nodes.
            let kids = children(raw);
            let glyph = |n: &Value| n["$el"] == "Icon" || n.get("$icon").is_some();
            let trim = kids.len() > 1;
            let leading = trim && kids.first().is_some_and(glyph);
            let trailing = trim && kids.last().is_some_and(glyph);
            if let Some(shortcut) = p.get("shortcut") {
                let hint = render_with(shortcut, specimens, ctx)?;
                hint.add_css_class("dz-button-shortcut");
                // Props precede children in the shared specimen walk.
                button_children(&content, kids, specimens, ctx)?;
                content.append(&hint);
            } else {
                button_children(&content, kids, specimens, ctx)?;
            }
            if leading {
                b.add_css_class("dz-glyph-leading");
            }
            if trailing {
                b.add_css_class("dz-glyph-trailing");
            }
            if trim {
                content.add_css_class("dz-labelled-button");
            }
            b.set_child(Some(&content));
            b.set_hexpand(boolean(p, "fullWidth"));
            b.upcast()
        }
        "Icon" => {
            let w = icon(
                p["icon"]["$icon"]
                    .as_str()
                    .ok_or("Icon needs a Fluent glyph")?,
            );
            if let Some(size) = p["size"].as_i64() {
                w.set_pixel_size(size as i32);
            }
            w.upcast()
        }
        "IconButton" => {
            let b = button("", variant(p, Variant::Plain), t, s);
            b.add_css_class("dz-icon-button");
            let face = children(n).first().ok_or("IconButton needs a glyph")?;
            b.set_child(Some(&render_with(face, specimens, ctx)?));
            b.set_dazz_label(string(p, "label", ""));
            b.set_tooltip_text(Some(string(p, "label", "")));
            b.upcast()
        }
        "KeyCap" => {
            let w = key_cap(&text(n));
            w.set_dazz_size(s);
            w.add_css_class(&format!("dz-key-cap-{}", string(p, "variant", "hint")));
            w.upcast()
        }
        "Avatar" => {
            let name = string(p, "name", "");
            let words: Vec<_> = name.split_whitespace().collect();
            let initials = words
                .first()
                .and_then(|w| w.chars().next())
                .into_iter()
                .chain(
                    (words.len() > 1)
                        .then(|| words.last().unwrap().chars().next())
                        .flatten(),
                )
                .collect::<String>()
                .to_uppercase();
            let w = avatar(string(p, "fallback", &initials), s);
            w.set_dazz_tint(t);
            w.set_dazz_label(name);
            if string(p, "shape", "circle") == "square" {
                w.add_css_class("dz-avatar-square");
            }
            w.upcast()
        }
        "Badge" => badge(&text(n), variant(p, Variant::Tinted), t, s).upcast(),
        "Tag" => {
            if p.get("selected").is_some() {
                let w = tag(&text(n), variant(p, Variant::Tinted), t, s);
                w.set_active(boolean(p, "selected"));
                w.upcast()
            } else {
                let w = box_(gtk::Orientation::Horizontal, "1");
                w.add_css_class("dz-badge");
                w.add_css_class("dz-tag");
                w.add_css_class("dz-tag-static");
                w.set_dazz_size(s);
                w.set_dazz_tint(t);
                w.set_dazz_variant(variant(p, Variant::Tinted));
                w.append(&gtk::Label::new(Some(&text(n))));
                if p.get("onRemove").is_some() {
                    let remove = gtk::Button::new();
                    remove.add_css_class("dz-tag-remove");
                    remove.set_child(Some(&icon("Dismiss12Regular")));
                    remove.set_dazz_label(string(p, "removeLabel", "Remove"));
                    let action = raw["props"]["onRemove"].clone();
                    let ctx = ctx.clone();
                    remove.connect_clicked(move |_| ctx.run(&action, &[]));
                    w.append(&remove);
                }
                w.upcast()
            }
        }
        "SegmentedControl" => {
            let items = p["items"]
                .as_array()
                .ok_or("SegmentedControl needs items")?;
            let names: Vec<_> = items
                .iter()
                .map(|i| {
                    i["icon"]["$icon"]
                        .as_str()
                        .map(|name| format!("dazzui-{name}-symbolic"))
                })
                .collect();
            let faces: Vec<_> = items
                .iter()
                .zip(&names)
                .map(|(item, icon)| (string(item, "label", ""), icon.as_deref()))
                .collect();
            let selected = items
                .iter()
                .position(|i| i["value"] == p["value"])
                .unwrap_or(0);
            let w = glyph_segmented_control(&faces, selected, boolean(p, "iconOnly"));
            w.set_dazz_size(s);
            w.set_dazz_tint(t);
            if string(p, "activeStyle", "raised") == "filled" {
                w.add_css_class("dz-segments-filled");
            }
            if boolean(p, "stretch") {
                w.set_hexpand(true);
                w.set_homogeneous(true);
            }
            for (child, item) in children_widgets(&w).iter().zip(items) {
                let toggle = child.clone().downcast::<gtk::ToggleButton>().unwrap();
                toggle.set_sensitive(!boolean(item, "disabled"));
                let key = item["value"].clone();
                let source = raw["props"]["value"].clone();
                let own_key = key.clone();
                ctx.bind(&toggle, move |toggle, ctx| {
                    toggle.set_active(ctx.resolve(&source, &[]) == own_key)
                });
                let ctx = ctx.clone();
                let action = raw["props"]["onValueChange"].clone();
                toggle.connect_toggled(move |toggle| {
                    if toggle.is_active() {
                        ctx.run(&action, std::slice::from_ref(&key));
                    }
                });
            }
            w.upcast()
        }
        "PillTabs" => {
            let items = p["items"].as_array().ok_or("PillTabs needs items")?;
            let counts: Vec<_> = items
                .iter()
                .map(|item| item.get("count").map(super::state::text))
                .collect();
            let icons: Vec<_> = items
                .iter()
                .map(|item| {
                    item["icon"]["$icon"]
                        .as_str()
                        .map(|name| format!("dazzui-{name}-symbolic"))
                })
                .collect();
            let faces: Vec<_> = items
                .iter()
                .enumerate()
                .map(|(index, item)| PillTab {
                    value: string(item, "value", ""),
                    label: string(item, "label", ""),
                    count: counts[index].as_deref(),
                    icon: icons[index].as_deref(),
                    disabled: boolean(item, "disabled"),
                })
                .collect();
            let selected = p
                .get("value")
                .or_else(|| p.get("defaultValue"))
                .and_then(Value::as_str);
            let root = pill_tabs(&faces, selected, t);
            for (child, item) in children_widgets(&root).iter().zip(items) {
                let tab = child.clone().downcast::<gtk::ToggleButton>().unwrap();
                let key = item["value"].clone();
                if let Some(source) = raw["props"].get("value") {
                    let source = source.clone();
                    let key = key.clone();
                    ctx.bind(&tab, move |tab, ctx| {
                        tab.set_active(ctx.resolve(&source, &[]) == key)
                    });
                }
                let ctx = ctx.clone();
                let action = raw["props"]["onValueChange"].clone();
                tab.connect_toggled(move |tab| {
                    if tab.is_active() {
                        ctx.run(&action, std::slice::from_ref(&key));
                    }
                });
            }
            root.upcast()
        }
        "OptionCard" => {
            let card = option_card(string(p, "title", ""), string(p, "description", ""), None);
            card.set_dazz_tint(t);
            card.set_active(boolean(p, "selected"));
            card.upcast()
        }
        "Card" => {
            let cv = match string(p, "variant", "raised") {
                "raised" => CardVariant::Raised,
                "outlined" => CardVariant::Outlined,
                "tinted" => CardVariant::Tinted,
                _ => CardVariant::Sunken,
            };
            let b = card(cv, t, s);
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
            b.upcast()
        }
        "Callout" => {
            let title = raw["props"]
                .get("title")
                .map(|title| render_with(title, specimens, ctx))
                .transpose()?;
            let leading = raw["props"]
                .get("icon")
                .map(|icon| render_with(icon, specimens, ctx))
                .transpose()?;
            let action = raw["props"]
                .get("action")
                .map(|action| render_with(action, specimens, ctx))
                .transpose()?;
            let description = label(&text(n), "body-small");
            description.set_wrap(true);
            callout_with_slots(
                title.as_ref(),
                (!text(n).is_empty()).then_some(description.upcast_ref()),
                leading.as_ref(),
                action.as_ref(),
                t,
                s,
            )
            .upcast()
        }
        "ActionBar" => {
            let b = action_bar();
            let align = string(p, "align", "start");
            if align == "end" {
                let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                spacer.set_hexpand(true);
                b.append(&spacer);
            }
            for (index, node) in children(raw).iter().enumerate() {
                if align == "between" && index > 0 {
                    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    spacer.set_hexpand(true);
                    b.append(&spacer);
                }
                b.append(&render_with(node, specimens, ctx)?);
            }
            b.upcast()
        }
        "Fieldset" => {
            let b = fieldset(p["legend"].as_str());
            append(&b, children(raw), specimens, ctx)?;
            b.upcast()
        }
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
        "SearchField" => {
            let w = search_field(string(p, "placeholder", ""), s);
            w.set_text(string(p, "value", ""));
            w.upcast()
        }
        "Toggle" => {
            let b = toggle(&text(n), t, s);
            let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            content.add_css_class("dz-button-content");
            button_children(&content, children(raw), specimens, ctx)?;
            b.set_child(Some(&content));
            if children(raw).len() == 1 && children(raw)[0]["$el"] == "Icon" {
                b.add_css_class("dz-toggle-icon");
            }
            if let Some(label) = p["label"].as_str() {
                b.set_dazz_label(label);
                b.set_tooltip_text(Some(label));
            }
            b.set_active(boolean(p, "defaultPressed"));
            b.set_dazz_variant(variant(p, Variant::Plain));
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
            let items = select_items(&p["options"]);
            let w = select_options(&items, string(p, "placeholder", ""), s);
            let values = select_values(&items);
            if let Some(value) = p
                .get("value")
                .or_else(|| p.get("defaultValue"))
                .and_then(Value::as_str)
            {
                w.set_selected(
                    values
                        .iter()
                        .position(|item| item.as_deref() == Some(value))
                        .unwrap_or(0) as u32,
                );
            }
            w.set_dazz_tint(if string(p, "state", "") == "error" {
                Tint::Danger
            } else {
                t
            });
            w.set_dazz_invalid(string(p, "state", "") == "error");
            if boolean(p, "mono") {
                w.add_css_class("dz-mono");
            }
            if boolean(p, "required") {
                w.update_property(&[gtk::accessible::Property::Required(true)]);
            }
            if boolean(p, "readOnly") {
                w.add_css_class("dz-readonly");
                w.set_focusable(true);
                if let Some(button) = w.first_child() {
                    button.set_sensitive(false);
                }
                w.update_property(&[gtk::accessible::Property::ReadOnly(true)]);
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
        "Meter" => {
            let b = box_(gtk::Orientation::Vertical, "2");
            let min = number(p, "min", 0.);
            let max = number(p, "max", 100.);
            let fraction = if max > min {
                ((number(p, "value", 0.) - min) / (max - min)).clamp(0., 1.)
            } else {
                0.
            };
            if p.get("label").is_some() || boolean(p, "showValue") {
                let head = box_(gtk::Orientation::Horizontal, "3");
                if let Some(title) = p["label"].as_str() {
                    let title = label(title, "label-small");
                    title.add_css_class("dz-meter-label");
                    title.set_hexpand(true);
                    head.append(&title);
                }
                if boolean(p, "showValue") {
                    let reading = label(&format!("{}%", (fraction * 100.).round()), "label-small");
                    reading.add_css_class("dz-meter-value");
                    head.append(&reading);
                }
                b.append(&head);
            }
            b.append(&progress(fraction, t, s));
            b.upcast()
        }
        "Spinner" => {
            let w = spinner(s);
            w.set_dazz_tint(t);
            if boolean(p, "onAccent") {
                w.add_css_class("dz-on-accent");
            }
            w.upcast()
        }
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
            w.add_css_class(&format!("dz-underline-{}", string(p, "underline", "hover")));
            if boolean(p, "external") {
                w.set_tooltip_text(Some(string(p, "href", "")));
                let face = box_(gtk::Orientation::Horizontal, "0-5");
                face.append(&gtk::Label::new(Some(&text(n))));
                let arrow = icon("ArrowUpRight12Regular");
                arrow.add_css_class("dz-link-arrow");
                face.append(&arrow);
                w.set_child(Some(&face));
            }
            w.upcast()
        }
        "Collapsible" => {
            let b = box_(gtk::Orientation::Vertical, "2");
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
            let w = collapsible(string(p, "title", ""), &b);
            w.set_expanded(boolean(p, "defaultOpen"));
            w.upcast()
        }
        "EmptyState" => {
            let action = p
                .get("action")
                .map(|a| render_with(a, specimens, ctx))
                .transpose()?;
            empty_state(string(p, "title", ""), &text(n), action.as_ref()).upcast()
        }
        "FormField" => {
            let b = box_(gtk::Orientation::Vertical, "2");
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
            b.set_dazz_invalid(string(p, "state", "") == "error");
            form_field(string(p, "label", ""), &b, p["hint"].as_str()).upcast()
        }
        "Preferences" => {
            let b = preferences();
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
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
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
            if let Some(footer) = p["footer"].as_str() {
                b.append(&label(footer, "body-small"));
            }
            b.upcast()
        }
        "PreferenceRow" => {
            let b = box_(gtk::Orientation::Horizontal, "2");
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
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
    ctx.wire(kind, &w, &raw["props"]);
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
