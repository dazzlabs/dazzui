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
pub(super) fn size(p: &Value, fallback: &str) -> Size {
    match string(p, "size", fallback) {
        "tiny" => Size::Tiny,
        "small" => Size::Small,
        "large" => Size::Large,
        _ => Size::Medium,
    }
}
pub(super) fn tint(p: &Value, fallback: &str) -> Tint {
    match string(p, "tint", fallback) {
        "neutral" => Tint::Neutral,
        "info" => Tint::Info,
        "success" => Tint::Success,
        "warning" => Tint::Warning,
        "danger" => Tint::Danger,
        _ => Tint::Primary,
    }
}
pub(super) fn variant(p: &Value, default: Variant) -> Variant {
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
fn side(p: &Value, fallback: &str) -> gtk::PositionType {
    match string(p, "side", fallback) {
        "top" => gtk::PositionType::Top,
        "left" => gtk::PositionType::Left,
        "right" => gtk::PositionType::Right,
        _ => gtk::PositionType::Bottom,
    }
}
fn alignment(p: &Value) -> gtk::Align {
    match string(p, "align", "center") {
        "start" => gtk::Align::Start,
        "end" => gtk::Align::End,
        _ => gtk::Align::Center,
    }
}
fn activate(widget: &gtk::Widget, action: impl Fn(&gtk::Widget) + 'static) {
    if let Some(button) = widget.downcast_ref::<gtk::Button>() {
        button.connect_clicked(move |w| action(w.upcast_ref()));
    } else {
        let action = std::rc::Rc::new(action);
        widget.set_focusable(true);
        let click = gtk::GestureClick::new();
        let weak = widget.downgrade();
        let run = action.clone();
        click.connect_released(move |_, _, _, _| {
            if let Some(w) = weak.upgrade() {
                run(&w);
            }
        });
        widget.add_controller(click);
        let keys = gtk::EventControllerKey::new();
        let weak = widget.downgrade();
        keys.connect_key_pressed(move |_, key, _, _| {
            if [gtk::gdk::Key::Return, gtk::gdk::Key::space].contains(&key) {
                if let Some(w) = weak.upgrade() {
                    action(&w);
                }
                return gtk::glib::Propagation::Stop;
            }
            gtk::glib::Propagation::Proceed
        });
        widget.add_controller(keys);
    }
}
pub(super) fn popup_trigger(trigger: &gtk::Widget, popup: &gtk::Popover) {
    let weak = popup.downgrade();
    activate(trigger, move |_| {
        if let Some(popup) = weak.upgrade() {
            if popup.is_visible() {
                popup.popdown();
            } else {
                popup.popup();
            }
        }
    });
}
pub(super) fn render_with(
    raw: &Value,
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<gtk::Widget, String> {
    let resolved = ctx.resolve(raw, &[]);
    let n = &resolved;
    if n.is_string() {
        let w = TextBox::new(&text(n));
        w.add_css_class("dz-type-body-medium");
        w.add_css_class("dz-text");
        w.label().set_wrap(true);
        w.label().set_wrap_mode(gtk::pango::WrapMode::WordChar);
        if raw != n {
            let raw = raw.clone();
            ctx.bind(&w, move |w, ctx| {
                w.set_text(&super::state::text(&ctx.resolve(&raw, &[])))
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
            "ToggleGroup" => "small",
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
    // Providers lend their trigger behavior but draw no box of their own.
    let transparent = [
        "Tooltip",
        "PreviewCard",
        "Popover",
        "Dialog",
        "Drawer",
        "DrawerClose",
        "PopoverClose",
        "DialogClose",
        "Menu",
    ]
    .contains(&kind);
    let mut own = (!transparent)
        .then(|| n["$at"].as_u64().map(|n| n as usize))
        .flatten();
    let before = specimens.len();
    let mut specimen = None;
    let w: gtk::Widget = match kind {
        "ToastProvider" => {
            let provider = box_(gtk::Orientation::Vertical, "0");
            provider.add_css_class("dz-toast-provider");
            let viewport = children(raw).iter().find(|n| n["$el"] == "ToastViewport");
            let queue = ToastQueue::new(
                &provider,
                if viewport.is_some_and(|n| n["props"]["placement"] == "top") {
                    ToastPlacement::Top
                } else {
                    ToastPlacement::Bottom
                },
                number(p, "limit", 3.) as usize,
            );
            let scoped = ctx.with_toasts(&queue);
            for child in children(raw) {
                if child["$el"] == "ToastViewport" {
                    if let Some(at) = child["$at"].as_u64() {
                        specimens.push((at as usize, queue.stack().clone().upcast()));
                    }
                } else {
                    let flow = render_with(child, specimens, &scoped)?;
                    if specimen.is_none() {
                        specimen = Some(flow.clone());
                    }
                    provider.append(&flow);
                }
            }
            // Action contexts hold a weak queue, so receipts do not form a
            // cycle through their originating scene state and this provider.
            provider.connect_destroy(move |_| {
                let _ = &queue;
            });
            provider.upcast()
        }
        "Frame" => {
            let frame = box_(gtk::Orientation::Horizontal, "0");
            frame.add_css_class("dz-story-frame");
            frame.set_overflow(gtk::Overflow::Hidden);
            append(&frame, children(raw), specimens, ctx)?;
            frame.set_size_request(
                -1,
                length(string(p, "height", "16rem")).ok_or("Frame needs a fixed height")?,
            );
            if p["width"] == "100%" {
                frame.set_hexpand(true);
                frame.upcast()
            } else {
                super::width::Width::new(
                    &frame,
                    length(string(p, "width", "36rem"))
                        .ok_or("Frame needs a fixed width or 100%")?,
                )
                .upcast()
            }
        }
        "Splitter" => {
            let panels = children(raw)
                .iter()
                .map(|node| {
                    if node["$el"] != "SplitterPanel" {
                        return Err("Splitter children must be panels".into());
                    }
                    let widget = render_with(node, specimens, ctx)?;
                    let props = ctx.resolve(&node["props"], &[]);
                    Ok(SplitPane {
                        widget,
                        default_size: number(&props, "defaultSize", 1.),
                        min_size: number(&props, "minSize", 0.),
                        max_size: number(&props, "maxSize", 100.),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let view = split_view(
                if p["orientation"] == "vertical" {
                    gtk::Orientation::Vertical
                } else {
                    gtk::Orientation::Horizontal
                },
                panels,
            )?;
            let ctx = ctx.clone();
            let action = raw["props"]["onSizesChange"].clone();
            view.connect_sizes_changed(move |_, sizes| {
                ctx.run(&action, &[serde_json::json!(sizes)])
            });
            view.upcast()
        }
        "SplitterPanel" => {
            let content = box_(gtk::Orientation::Vertical, "0");
            append(&content, children(raw), specimens, ctx)?;
            splitter_panel(&content).upcast()
        }
        "OtpField" => otp_field(OtpOptions {
            length: number(p, "length", 0.) as usize,
            value: raw["props"]
                .get("value")
                .map(|_| string(p, "value", "").into()),
            default_value: string(p, "defaultValue", "").into(),
            group_after: p["groupAfter"].as_u64().map(|v| v as usize),
            size: s,
            tint: if string(p, "state", "") == "error" {
                Tint::Danger
            } else {
                t
            },
            validation: match string(p, "validationType", "numeric") {
                "alpha" => OtpValidation::Alpha,
                "alphanumeric" => OtpValidation::Alphanumeric,
                "none" => OtpValidation::None,
                _ => OtpValidation::Numeric,
            },
            mask: boolean(p, "mask"),
            read_only: boolean(p, "readOnly"),
            disabled: boolean(p, "disabled"),
            required: boolean(p, "required"),
        })?
        .upcast(),
        "ShortcutRecorder" => {
            let recorder = shortcut_recorder(ShortcutRecorderOptions {
                value: string(p, "value", "").into(),
                placeholder: string(p, "placeholder", "Record shortcut").into(),
                recording_label: string(p, "recordingLabel", "Press keys…").into(),
                clear_label: string(p, "clearLabel", "Clear shortcut").into(),
                label: p["aria-label"].as_str().map(str::to_owned),
                invalid: string(p, "state", "") == "error",
                disabled: boolean(p, "disabled"),
                tint: t,
            });
            specimen = Some(recorder.button().upcast());
            recorder.upcast()
        }
        "Tree" => super::tree::render(raw, specimens, ctx)?,
        "Table" => {
            let table = table_content(t);
            let mut actions = vec![];
            for node in children(raw) {
                let child = render_with(node, specimens, ctx)?;
                if let Some(row) = child.downcast_ref::<gtk::ListBoxRow>() {
                    table.append_row(row);
                    actions.push((row.downgrade(), node["props"]["onClick"].clone()));
                } else {
                    table.append_head(&child);
                }
            }
            let ctx = ctx.clone();
            table.rows().connect_row_activated(move |_, row| {
                if row.is_activatable()
                    && row.is_sensitive()
                    && let Some((_, action)) = actions
                        .iter()
                        .find(|(weak, _)| weak.upgrade().as_ref() == Some(row))
                {
                    ctx.run(action, &[]);
                }
            });
            table.upcast()
        }
        "TableHead" | "TableRow" => {
            let content = children(raw)
                .iter()
                .map(|node| render_with(node, specimens, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            if kind == "TableHead" {
                table_head(&content).upcast()
            } else {
                let row = table_row(&content, boolean(p, "interactive"), boolean(p, "active"));
                let source = raw["props"].clone();
                ctx.bind(&row, move |row, ctx| {
                    let props = ctx.resolve(&source, &[]);
                    set_table_row_interactive(row, boolean(&props, "interactive"));
                    set_table_row_active(row, boolean(&props, "active"));
                });
                row.upcast()
            }
        }
        "TableCell" => {
            let content = box_(gtk::Orientation::Horizontal, "0");
            append(&content, children(raw), specimens, ctx)?;
            table_cell(
                &content,
                boolean(p, "head"),
                if p["align"] == "end" {
                    gtk::Align::End
                } else {
                    gtk::Align::Start
                },
            )
            .upcast()
        }
        "Menu" | "ContextMenu" | "Menubar" => super::menu::render(raw, specimens, ctx)?,
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
                    if align != gtk::Align::Fill || !child.has_css_class("dz-fixed-width") {
                        child.set_halign(align);
                    }
                }
            }
            let result: gtk::Widget = if row && string(p, "justify", "start") != "between" {
                let wrap = super::wrap::Wrap::new(string(p, "gap", "3"), align);
                wrap.set_homogeneous(b.is_homogeneous());
                wrap.set_size_request(b.width_request(), -1);
                wrap.set_hexpand(b.hexpands());
                wrap.set_halign(b.halign());
                for child in children_widgets(&b) {
                    b.remove(&child);
                    wrap.append(&child);
                }
                wrap.upcast()
            } else {
                b.upcast()
            };
            if let Some(width) = p["width"].as_str().and_then(length) {
                super::width::Width::new(&result, width).upcast()
            } else {
                result
            }
        }
        "Text" => {
            let b = box_(gtk::Orientation::Horizontal, "0");
            for node in children(raw) {
                if node.is_string() {
                    let l = TextBox::new(&text(node));
                    l.add_css_class("dz-type-body-medium");
                    l.add_css_class("dz-text");
                    l.label().set_wrap(true);
                    l.label().set_wrap_mode(gtk::pango::WrapMode::WordChar);
                    if boolean(p, "strong") {
                        l.add_css_class("dz-text-strong");
                    }
                    if string(p, "tone", "default") == "secondary" {
                        l.add_css_class("dz-text-secondary");
                    }
                    if p.get("tint").is_some() {
                        l.set_dazz_tint(t);
                        l.add_css_class("dz-tinted-text");
                    }
                    b.append(&l);
                } else {
                    let child = render_with(node, specimens, ctx)?;
                    if child.is::<gtk::Label>() || child.is::<TextBox>() {
                        if boolean(p, "strong") {
                            child.add_css_class("dz-text-strong");
                        }
                        if string(p, "tone", "default") == "secondary" {
                            child.add_css_class("dz-text-secondary");
                        }
                        if p.get("tint").is_some() {
                            child.set_dazz_tint(t);
                            child.add_css_class("dz-tinted-text");
                        }
                    }
                    if child.has_css_class("dz-link") {
                        child.add_css_class("dz-link-inline");
                        child.set_valign(gtk::Align::Center);
                    }
                    b.append(&child);
                }
            }
            b.set_valign(gtk::Align::Center);
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
        "Tooltip" | "PreviewCard" | "Popover" => {
            let tooltip = kind == "Tooltip";
            let trigger = if tooltip {
                render_with(
                    children(raw).first().ok_or("Tooltip needs a trigger")?,
                    specimens,
                    ctx,
                )?
            } else {
                render_with(&raw["props"]["trigger"], specimens, ctx)?
            };
            let body = box_(gtk::Orientation::Vertical, "0");
            if tooltip {
                body.append(&render_with(&raw["props"]["label"], specimens, ctx)?);
                trigger.set_tooltip_text(None);
            } else {
                if let Some(title) = raw["props"].get("title") {
                    let title = render_with(title, specimens, ctx)?;
                    title.add_css_class("dz-popover-heading");
                    body.append(&title);
                }
                if let Some(description) = raw["props"].get("description") {
                    let description = render_with(description, specimens, ctx)?;
                    description.add_css_class("dz-popover-description");
                    body.append(&description);
                }
                let content = box_(gtk::Orientation::Vertical, "0");
                if body.first_child().is_some() && !children(raw).is_empty() {
                    content.add_css_class("dz-popover-content");
                }
                append(&content, children(raw), specimens, ctx)?;
                body.append(&content);
            }
            let popup = rich_popover(
                &trigger,
                &body,
                side(p, if tooltip { "top" } else { "bottom" }),
                alignment(p),
            );
            if tooltip {
                popup.add_css_class("dz-tooltip");
                popup.set_accessible_role(gtk::AccessibleRole::Tooltip);
                popup.set_dazz_label(&text(&p["label"]));
                trigger.update_relation(&[gtk::accessible::Relation::DescribedBy(&[
                    popup.upcast_ref()
                ])]);
                popup.remove_css_class("dz-rich-popover");
            }
            if let Some(width) = p["width"]
                .as_f64()
                .map(|w| w as i32)
                .or_else(|| p["width"].as_str().and_then(length))
            {
                let body = body.clone();
                popup.connect_map(move |popup| {
                    let theme = Theme::of(popup).unwrap_or_default();
                    let padding = theme.dimension("spacing-4").unwrap()
                        + theme.dimension("spacing-0-5").unwrap();
                    let border = theme.dimension("stroke-hairline").unwrap();
                    body.set_size_request((width - (2. * (padding + border)) as i32).max(1), -1);
                });
            }
            if kind == "Popover" {
                popup_trigger(&trigger, &popup);
            } else if !boolean(p, "disabled") {
                let theme = Theme::default();
                let role = if tooltip { "tooltip" } else { "preview-card" };
                let open = p["delay"]
                    .as_u64()
                    .map(std::time::Duration::from_millis)
                    .unwrap_or_else(|| theme.duration(&format!("{role}-open-delay")).unwrap());
                let close = p["closeDelay"]
                    .as_u64()
                    .map(std::time::Duration::from_millis)
                    .unwrap_or_else(|| theme.duration(&format!("{role}-close-delay")).unwrap());
                hover_popup(&trigger, &popup, open, close, !tooltip);
            }
            trigger
        }
        "PopoverClose" | "DialogClose" | "DrawerClose" => {
            let control = render_with(&raw["props"]["render"], specimens, ctx)?;
            activate(&control, dismiss_from);
            control
        }
        "Drawer" => {
            let trigger = render_with(&raw["props"]["trigger"], specimens, ctx)?;
            let body = box_(gtk::Orientation::Vertical, "0");
            append(&body, children(raw), specimens, ctx)?;
            let footer = raw["props"]
                .get("footer")
                .map(|node| render_with(node, specimens, ctx))
                .transpose()?;
            let drawer = Drawer::new(
                &trigger,
                &body,
                footer.as_ref(),
                DrawerOptions {
                    side: match string(p, "side", "right") {
                        "left" => DrawerSide::Left,
                        "top" => DrawerSide::Top,
                        "bottom" => DrawerSide::Bottom,
                        _ => DrawerSide::Right,
                    },
                    size: p["size"]
                        .as_f64()
                        .map(|n| n.round() as i32)
                        .or_else(|| p["size"].as_str().and_then(length)),
                    title: p["title"].as_str().map(str::to_owned),
                    description: p["description"].as_str().map(str::to_owned),
                },
            );
            activate(&trigger, move |_| drawer.set_open(true));
            trigger
        }
        "Dialog" => {
            let trigger = render_with(&raw["props"]["trigger"], specimens, ctx)?;
            let content = box_(gtk::Orientation::Vertical, "0");
            append(&content, children(raw), specimens, ctx)?;
            let width = p["width"]
                .as_f64()
                .map(|n| n as i32)
                .or_else(|| p["width"].as_str().and_then(length));
            let danger = string(p, "tone", "default") == "danger";
            let modal = std::rc::Rc::new(std::cell::RefCell::new(None::<gtk::Window>));
            let owner = modal.clone();
            activate(&trigger, move |trigger| {
                if owner.borrow().is_none() {
                    let Some(parent) = trigger.root().and_downcast::<gtk::Window>() else {
                        return;
                    };
                    let window = dazzui_gtk4::modal(&parent, &content, width, danger);
                    if let Some(header) = content.first_child()
                        && let Some(title) = header.first_child()
                    {
                        window.update_relation(&[gtk::accessible::Relation::LabelledBy(&[
                            title.upcast_ref()
                        ])]);
                    }
                    *owner.borrow_mut() = Some(window);
                }
                owner.borrow().as_ref().unwrap().present();
            });
            trigger.connect_destroy(move |_| {
                if let Some(window) = modal.borrow_mut().take() {
                    window.destroy();
                }
            });
            trigger
        }
        "DialogHeader" => {
            let header = box_(gtk::Orientation::Vertical, "1");
            header.add_css_class("dz-modal-header");
            let title = render_with(&raw["props"]["title"], specimens, ctx)?;
            title.add_css_class("dz-modal-title");
            header.append(&title);
            if let Some(subtitle) = raw["props"].get("subtitle") {
                let subtitle = render_with(subtitle, specimens, ctx)?;
                subtitle.add_css_class("dz-modal-subtitle");
                header.append(&subtitle);
            }
            header.upcast()
        }
        "DialogBody" => {
            let body = box_(gtk::Orientation::Vertical, "4");
            body.add_css_class("dz-modal-body");
            append(&body, children(raw), specimens, ctx)?;
            let scroll = gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Never)
                .vscrollbar_policy(gtk::PolicyType::Automatic)
                .propagate_natural_height(true)
                .child(&body)
                .build();
            scroll.upcast()
        }
        "DialogFooter" => {
            let footer = box_(gtk::Orientation::Horizontal, "2");
            footer.add_css_class("dz-modal-footer");
            let spacer = box_(gtk::Orientation::Horizontal, "0");
            spacer.set_hexpand(true);
            footer.append(&spacer);
            append(&footer, children(raw), specimens, ctx)?;
            footer.upcast()
        }
        "AvatarGroup" => {
            let group = avatar_group(s);
            let all = children(raw);
            let count = number(p, "max", all.len() as f64).max(0.) as usize;
            for node in all.iter().take(count) {
                group.append(&render_with(node, specimens, ctx)?);
            }
            if all.len() > count {
                group.append(&avatar_overflow(all.len() - count, s));
            }
            group.upcast()
        }
        "Accordion" => {
            let group = accordion(boolean(p, "multiple"), boolean(p, "divided"));
            let nodes = children(raw);
            let selected = p
                .get("value")
                .or_else(|| p.get("defaultValue"))
                .and_then(Value::as_array);
            let keys: Vec<_> = nodes
                .iter()
                .map(|node| ctx.resolve(&node["props"]["value"], &[]))
                .collect();
            for (node, key) in nodes.iter().zip(&keys) {
                let item = render_with(node, specimens, ctx)?
                    .downcast::<gtk::Expander>()
                    .map_err(|_| "Accordion needs AccordionItem children")?;
                accordion_append(&group, &item);
                item.set_expanded(selected.is_some_and(|values| values.contains(key)));
            }
            for (item, key) in children_widgets(&group).iter().zip(&keys) {
                let item = item.clone().downcast::<gtk::Expander>().unwrap();
                if let Some(source) = raw["props"].get("value") {
                    let source = source.clone();
                    let key = key.clone();
                    ctx.bind(&item, move |item, ctx| {
                        item.set_expanded(
                            ctx.resolve(&source, &[])
                                .as_array()
                                .is_some_and(|v| v.contains(&key)),
                        )
                    });
                }
                let weak = group.downgrade();
                let keys = keys.clone();
                let ctx = ctx.clone();
                let action = raw["props"]["onValueChange"].clone();
                item.connect_expanded_notify(move |_| {
                    if let Some(group) = weak.upgrade() {
                        let active = children_widgets(&group)
                            .iter()
                            .zip(&keys)
                            .filter(|(w, _)| {
                                w.downcast_ref::<gtk::Expander>()
                                    .is_some_and(|w| w.is_expanded())
                            })
                            .map(|(_, k)| k.clone())
                            .collect();
                        ctx.run(&action, &[Value::Array(active)]);
                    }
                });
            }
            group.upcast()
        }
        "ToggleGroup" => {
            let group = toggle_group(p["multiple"].as_bool().unwrap_or(true));
            let items = p["items"].as_array().ok_or("ToggleGroup needs items")?;
            let keys: Vec<_> = items.iter().map(|item| item["value"].clone()).collect();
            for item in items {
                let toggle = toggle("", t, s);
                toggle.set_dazz_variant(variant(p, Variant::Recessed));
                let face = render_with(&item["label"], specimens, ctx)?;
                if face.is::<gtk::Image>() {
                    toggle.add_css_class("dz-toggle-icon");
                }
                toggle.set_child(Some(&face));
                if let Some(hint) = item["hint"].as_str() {
                    toggle.set_dazz_label(hint);
                    toggle.set_tooltip_text(Some(hint));
                }
                toggle.set_sensitive(!boolean(item, "disabled"));
                toggle_group_append(&group, &toggle);
                toggle.set_active(
                    p["value"]
                        .as_array()
                        .is_some_and(|v| v.contains(&item["value"])),
                );
            }
            for (item, key) in children_widgets(&group).iter().zip(&keys) {
                let toggle = item.clone().downcast::<gtk::ToggleButton>().unwrap();
                let source = raw["props"]["value"].clone();
                let key = key.clone();
                ctx.bind(&toggle, move |toggle, ctx| {
                    toggle.set_active(
                        ctx.resolve(&source, &[])
                            .as_array()
                            .is_some_and(|v| v.contains(&key)),
                    )
                });
                let weak = group.downgrade();
                let keys = keys.clone();
                let ctx = ctx.clone();
                let action = raw["props"]["onValueChange"].clone();
                toggle.connect_toggled(move |_| {
                    if let Some(group) = weak.upgrade() {
                        let selected = children_widgets(&group)
                            .iter()
                            .zip(&keys)
                            .filter(|(w, _)| {
                                w.downcast_ref::<gtk::ToggleButton>()
                                    .is_some_and(|w| w.is_active())
                            })
                            .map(|(_, k)| k.clone())
                            .collect();
                        ctx.run(&action, &[Value::Array(selected)]);
                    }
                });
            }
            group.upcast()
        }
        "StepList" => {
            let group = step_list();
            append(&group, children(raw), specimens, ctx)?;
            group.upcast()
        }
        "Step" => {
            let title = render_with(&raw["props"]["label"], specimens, ctx)?;
            let meta = raw["props"]
                .get("meta")
                .map(|node| render_with(node, specimens, ctx))
                .transpose()?;
            step_with_slots(
                &title,
                meta.as_ref(),
                match string(p, "status", "idle") {
                    "done" => StepStatus::Done,
                    "active" => StepStatus::Active,
                    _ => StepStatus::Idle,
                },
                t,
            )
            .upcast()
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
                w.append(&TextBox::new(&text(n)));
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
                w.set_stretch(true);
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
            let b = if let Some(note) = p["note"].as_str() {
                checkbox_with_note(&text(n), note, t, s)
            } else {
                checkbox(&text(n), t, s)
            };
            b.set_active(boolean(p, "defaultChecked"));
            b.set_inconsistent(boolean(p, "indeterminate"));
            b.upcast()
        }
        "Switch" => {
            let b = switch(t, s);
            b.set_active(boolean(p, "defaultChecked"));
            b.upcast()
        }
        "Combobox" | "Autocomplete" => super::combobox::render(raw, ctx)?,
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
        "NumberField" => {
            let unit = raw["props"]
                .get("unit")
                .map(|v| render_with(v, specimens, ctx))
                .transpose()?;
            let (root, input) = number_field_with_unit(
                number(p, "min", f64::MIN),
                number(p, "max", f64::MAX),
                number(p, "step", 1.),
                unit.as_ref(),
                s,
            );
            root.set_dazz_tint(if string(p, "state", "") == "error" {
                Tint::Danger
            } else {
                t
            });
            root.set_dazz_invalid(string(p, "state", "") == "error");
            input.set_dazz_invalid(string(p, "state", "") == "error");
            input.set_value(number(p, "defaultValue", 0.));
            if let Some(label) = p["aria-label"].as_str() {
                input.set_dazz_label(label);
            }
            root.upcast()
        }
        "TextField" => {
            let value = string(p, "defaultValue", "");
            let w: gtk::Widget = if boolean(p, "multiline") {
                let (field, w) = text_area_with_rows(number(p, "rows", 3.).max(1.) as u32);
                w.buffer().set_text(value);
                w.set_editable(!boolean(p, "readOnly"));
                if let Some(name) = p["aria-label"].as_str() {
                    w.set_dazz_label(name);
                }
                if let Some(placeholder) = p["placeholder"].as_str() {
                    let overlay = gtk::Overlay::new();
                    overlay.add_css_class("dz-text-area-container");
                    overlay.set_child(Some(&field));
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
                    field.upcast()
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
            w.set_dazz_tint(if string(p, "state", "") == "error" {
                Tint::Danger
            } else {
                t
            });
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
        "Slider" => super::slider::render(raw, ctx)?,
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
            w.set_dazz_label(string(p, "label", "Loading"));
            if boolean(p, "onAccent") {
                w.add_css_class("dz-on-accent");
            }
            w.upcast()
        }
        "Skeleton" => {
            let pixels = |v: &Value| {
                v.as_f64()
                    .map(|n| n.round() as i32)
                    .or_else(|| v.as_str().and_then(length))
            };
            let shape = match string(p, "shape", "text") {
                "circle" => SkeletonShape::Circle,
                "rect" => SkeletonShape::Rect,
                _ => SkeletonShape::Text,
            };
            let bar = skeleton(shape, pixels(&p["width"]), pixels(&p["height"]));
            if let Some(fraction) = p["width"]
                .as_str()
                .and_then(|s| s.strip_suffix('%'))
                .and_then(|s| s.parse::<f64>().ok())
            {
                specimen = Some(bar.clone().upcast());
                proportional_width(&bar, fraction / 100.).upcast()
            } else {
                bar.upcast()
            }
        }
        "SkeletonText" => skeleton_text(number(p, "lines", 3.) as usize).upcast(),
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
        "Code" => inline_code(&text(n)).upcast(),
        "CodeBlock" => {
            let title = raw["props"]
                .get("title")
                .map(|v| render_with(v, specimens, ctx))
                .transpose()?;
            let actions = raw["props"]
                .get("actions")
                .map(|v| render_with(v, specimens, ctx))
                .transpose()?;
            code_block(
                string(p, "code", ""),
                title.as_ref(),
                p["language"].as_str(),
                actions.as_ref(),
                CodeBlockOptions {
                    line_numbers: boolean(p, "lineNumbers"),
                    wrap: boolean(p, "wrap"),
                    copyable: boolean(p, "copyable"),
                },
            )
            .upcast()
        }
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
        "Collapsible" | "AccordionItem" => {
            let title = render_with(&raw["props"]["title"], specimens, ctx)?;
            let leading = raw["props"]
                .get("icon")
                .map(|node| render_with(node, specimens, ctx))
                .transpose()?;
            let meta = raw["props"]
                .get("meta")
                .map(|node| render_with(node, specimens, ctx))
                .transpose()?;
            let b = box_(gtk::Orientation::Vertical, "2");
            append(&b, children(raw), specimens, ctx)?;
            let w = collapsible_with_slots(&title, leading.as_ref(), meta.as_ref(), &b);
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
        "Form" => super::form::render(raw, specimens, ctx)?,
        "SidebarProvider"
        | "Sidebar"
        | "SidebarHeader"
        | "SidebarContent"
        | "SidebarGroup"
        | "SidebarGroupLabel"
        | "SidebarGroupContent"
        | "SidebarInset"
        | "SidebarMenu"
        | "SidebarMenuItem"
        | "SidebarMenuButton"
        | "SidebarMenuBadge"
        | "SidebarTrigger"
        | "SidebarFooter" => super::sidebar::render(raw, specimens, ctx)?,
        "DateRangePicker" => super::date_range_picker::render(raw, ctx)?,
        "Calendar" => super::calendar::render(raw, ctx)?,
        "FormField" => super::form::field(raw, specimens, ctx)?,
        "Preferences" => {
            let b = preferences();
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
            let width = Theme::default().dimension("preferences-width").unwrap() as i32;
            b.add_css_class("dz-preferences-constrained");
            super::width::Width::new(&b, width).upcast()
        }
        "PreferenceGroup" => {
            let b = preference_group(string(p, "title", ""));
            if let Some(description) = p["description"].as_str() {
                let description = label(description, "caption-small");
                description.add_css_class("dz-preference-group-description");
                b.first_child()
                    .unwrap()
                    .downcast::<gtk::Box>()
                    .unwrap()
                    .append(&description);
            }
            let sections = box_(gtk::Orientation::Vertical, "6");
            sections.add_css_class("dz-preference-group-sections");
            append(&sections, children(raw), specimens, ctx)?;
            b.append(&sections);
            b.upcast()
        }
        "PreferenceSection" => {
            let b = preference_section(string(p, "label", ""));
            let rows = box_(gtk::Orientation::Vertical, "1");
            rows.add_css_class("dz-preference-section-rows");
            append(&rows, children(raw), specimens, ctx)?;
            b.append(&rows);
            if let Some(footer) = p["footer"].as_str() {
                let footer = label(footer, "caption-small");
                footer.add_css_class("dz-preference-section-footer");
                footer.set_wrap(true);
                b.append(&footer);
            }
            b.upcast()
        }
        "PreferenceRow" => {
            let b = box_(gtk::Orientation::Horizontal, "2-5");
            append(
                &b,
                raw["children"].as_array().map(Vec::as_slice).unwrap_or(&[]),
                specimens,
                ctx,
            )?;
            if p.get("onOpen").is_some() {
                let chevron = gtk::Label::new(Some("›"));
                chevron.set_accessible_role(gtk::AccessibleRole::Presentation);
                b.append(&chevron);
                let row = preference_row_button(string(p, "title", ""), p["subtitle"].as_str(), &b);
                row.set_dazz_tint(tint(p, "neutral"));
                row.upcast()
            } else {
                preference_row(string(p, "title", ""), p["subtitle"].as_str(), &b).upcast()
            }
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
    if kind == "ToastProvider"
        && specimen
            .as_ref()
            .is_some_and(|flow| specimens[before..].iter().any(|(_, w)| w == flow))
    {
        own = None;
    }
    if let Some(at) = own {
        specimens.insert(before, (at, specimen.unwrap_or_else(|| w.clone())));
    }
    if boolean(p, "disabled") {
        w.set_sensitive(false);
    }
    if let Some(label) = p["aria-label"].as_str() {
        w.set_dazz_label(label);
    }
    Ok(w)
}
fn children_widgets(b: &impl IsA<gtk::Widget>) -> Vec<gtk::Widget> {
    let mut items = vec![];
    let mut child = b.as_ref().first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        items.push(w);
    }
    items
}
