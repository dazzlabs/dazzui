use super::*;
use nsh_core::element::Placement;

pub(super) fn side(value: &Value) -> Placement {
    match value.as_str() {
        Some("top") => Placement::Top,
        Some("left") => Placement::Left,
        Some("right") => Placement::Right,
        _ => Placement::Bottom,
    }
}
pub(super) fn trigger(v: Value, cx: Context, id: &str) -> AnyElement {
    render(v, cx, id)
}

pub(super) fn strings(value: &Value) -> Vec<SharedString> {
    match value {
        Value::Array(a) => a.iter().map(|v| print(v).into()).collect(),
        Value::String(s) => vec![s.clone().into()],
        _ => vec![],
    }
}
pub(super) fn select_items(v: &Value) -> Vec<SelectItem> {
    let mut items = vec![];
    for o in v.as_array().into_iter().flatten() {
        if o.get("options").is_some() {
            items.extend(
                select_items(&o["options"])
                    .into_iter()
                    .map(|item| item.group(print(&o["label"]))),
            );
        } else {
            let label = if o["label"].is_string() {
                print(&o["label"])
            } else {
                print(&o["hint"])
            };
            items.push(SelectItem::new(print(&o["value"]), label).disabled(truthy(&o["disabled"])));
        }
    }
    items
}
pub(super) fn choice_icon_renderer(
    items: Value,
    cx: Context,
    id: &str,
) -> impl Fn(&SelectItem) -> AnyElement + use<> {
    let id = id.to_owned();
    move |item: &SelectItem| {
        items
            .as_array()
            .into_iter()
            .flatten()
            .find(|v| v["value"].as_str() == Some(item.value.as_ref()))
            .and_then(|v| v.get("icon"))
            .map(|icon| render(icon.clone(), cx, &id))
            .unwrap_or_else(|| view().into_element())
    }
}
pub(super) fn choice_renderer(
    items: Value,
    cx: Context,
    id: &str,
    icon_only: bool,
) -> impl Fn(&SelectItem) -> AnyElement + use<> {
    let id = id.to_owned();
    move |item: &SelectItem| {
        let raw = items
            .as_array()
            .into_iter()
            .flatten()
            .find(|v| v["value"].as_str() == Some(item.value.as_ref()));
        let Some(raw) = raw else {
            return text(item.title.clone()).into_element();
        };
        let theme = Theme::current();
        view()
            .flex_row()
            .items_center()
            .gap(move || px(theme.tokens().spacing_1))
            .child(raw.get("icon").map(|v| render(v.clone(), cx, &id)))
            .child(
                (!icon_only || raw.get("icon").is_none())
                    .then(|| render(raw["label"].clone(), cx, &id)),
            )
            .child(raw.get("count").map(|v| render(v.clone(), cx, &id)))
            .into_element()
    }
}
pub(super) fn extend(
    kind: &str,
    p: &Props,
    v: &Value,
    cx: Context,
    id: &str,
) -> Option<AnyElement> {
    let theme = Theme::current();
    let kids = || children(&v["children"], cx, id);
    Some(match kind {
        "Calendar" => {
            let own = p.clone();
            let change = p.action("onValueChange");
            let range = p.val("mode").as_str() == Some("range");
            calendar()
                .range(range)
                .default_value(calendar_value(&p.val("defaultValue"), range))
                .value(move || {
                    if own.json.get("value").is_some() {
                        dazzui_nsh::CalendarValue::Controlled(calendar_value(
                            &own.val("value"),
                            range,
                        ))
                    } else {
                        dazzui_nsh::CalendarValue::Uncontrolled
                    }
                })
                .months(p.number("months", 1.0)() as u32)
                .week_starts_on(p.number("weekStartsOn", 1.0)() as u32)
                .today(p.optional_string("today"))
                .min(p.optional_string("min"))
                .max(p.optional_string("max"))
                .size(p.size(Size::Medium))
                .tint(p.tint())
                .on_change(move |v: &dazzui_nsh::CalendarDate| change(calendar_json(v)))
                .into_element()
        }
        "DateRangePicker" => {
            let own = p.clone();
            let change = p.action("onValueChange");
            date_range_picker().value(move ||date_range(&own.val("value"))).default_value(date_range(&p.val("defaultValue")))
                .today(p.optional_string("today")).months(p.number("months",1.0)() as u32).time_step(p.number("timeStep",15.0)() as u32)
                .presets(p.val("presets").as_array().map(|items|items.iter().map(|v|dazzui_nsh::DateRangePreset{label:print(&v["label"]).into(),days:v["days"].as_array().into_iter().flatten().filter_map(|v|v.as_i64().map(|n|n as i32)).collect()}).collect::<Vec<_>>()))
                .durations(p.val("durations").as_array().map(|items|items.iter().map(|v|dazzui_nsh::DateRangeDuration{label:print(&v["label"]).into(),minutes:v["minutes"].as_u64().unwrap_or(60) as u32}).collect::<Vec<_>>()))
                .disabled(p.bool("disabled")).error(p.val("state").as_str()==Some("error")).size(p.size(Size::Medium)).tint(p.tint()).accessibility_label(p.optional_string("aria-label"))
                .on_change(move |v:&dazzui_nsh::DateRangeValue|change(serde_json::json!({"start":v.start.as_ref(),"end":v.end.as_ref(),"allDay":v.all_day}))).into_element()
        }
        "ToastProvider" => toast_provider().child(kids()).into_element(),
        "ToastViewport" => toast_viewport()
            .top(p.val("placement").as_str() == Some("top"))
            .into_element(),
        "Form" => {
            let action = p.action("onSubmit");
            form()
                .on_submit(move || action(Value::Null))
                .child(kids())
                .into_element()
        }
        "Fieldset" => fieldset()
            .legend(p.string("legend"))
            .child(kids())
            .into_element(),
        "Autocomplete" => {
            let action = p.action("onValueChange");
            autocomplete()
                .items(strings(&p.val("items")))
                .placeholder(p.optional_string("placeholder"))
                .on_change(move |v: &SharedString| action(Value::String(v.to_string())))
                .into_element()
        }
        "ShortcutRecorder" => {
            let change = p.action("onValueChange");
            let clear = p.action("onClear");
            let fallback = p.action("onValueChange");
            let has_clear = p.json.get("onClear").is_some();
            shortcut_recorder()
                .value(p.string("value"))
                .disabled(p.bool("disabled"))
                .error(p.val("state").as_str() == Some("error"))
                .accessibility_label(p.optional_string("aria-label"))
                .on_change(move |v: &SharedString| change(Value::String(v.to_string())))
                .on_clear(move || {
                    if has_clear {
                        clear(Value::Null)
                    } else {
                        fallback(Value::String(String::new()))
                    }
                })
                .into_element()
        }
        "Splitter" => splitter()
            .axis(if p.val("orientation").as_str() == Some("vertical") {
                nsh_core::geometry::Axis::Vertical
            } else {
                nsh_core::geometry::Axis::Horizontal
            })
            .child(kids())
            .into_element(),
        "SplitterPanel" => splitter_panel()
            .default_size(p.number("defaultSize", 50.0)() as f32)
            .min_size(p.number("minSize", 0.0)() as f32)
            .max_size(p.number("maxSize", 100.0)() as f32)
            .child(kids())
            .into_element(),
        "Tree" => {
            let action = p.action("onSelectedChange");
            let selected = p.clone();
            let raw = p.val("items");
            let render_items = raw.clone();
            let id = id.to_owned();
            tree()
                .items(tree_items(&raw))
                .default_expanded(strings(&p.val("defaultExpanded")))
                .selected(move || {
                    if selected.json.get("selected").is_some() {
                        SelectValue::Controlled(
                            selected
                                .val("selected")
                                .as_str()
                                .map(|s| s.to_owned().into()),
                        )
                    } else {
                        SelectValue::Uncontrolled
                    }
                })
                .size(p.size(Size::Medium))
                .accessibility_label(p.optional_string("aria-label"))
                .on_selected_change(move |v: &SharedString| action(Value::String(v.to_string())))
                .render_item(move |entry: &dazzui_nsh::TreeEntry| {
                    let item = find_tree(&render_items, entry.id.as_ref());
                    view()
                        .flex_row()
                        .items_center()
                        .flex_1()
                        .pl(px(entry.depth as f32 * theme.tokens().spacing_4))
                        .gap(px(theme.tokens().spacing_2))
                        .child(if entry.folder {
                            icon("ChevronRight12Regular")
                                .app_class(Some(SharedString::from(
                                    "group-aria-expanded:rotate-90",
                                )))
                                .into_element()
                        } else {
                            view().w(px(theme.tokens().icon_small)).into_element()
                        })
                        .child(
                            item.and_then(|v| v.get("icon"))
                                .map(|v| render(v.clone(), cx, &id)),
                        )
                        .child(text(entry.label.clone()))
                        .child(view().flex_1())
                        .child(
                            item.and_then(|v| v.get("meta"))
                                .map(|v| render(v.clone(), cx, &id)),
                        )
                })
                .into_element()
        }
        "ContextMenu" => context_menu()
            .trigger(view().child(kids()))
            .child(menu_items(&p.raw("items"), cx, id))
            .into_element(),
        "Menubar" => menubar()
            .child(
                p.raw("menus")
                    .as_array()
                    .into_iter()
                    .flatten()
                    .map(|m| {
                        menubar_menu()
                            .value(print(&m["label"]))
                            .label(print(&m["label"]))
                            .child(menu_items(&m["items"], cx, id))
                    })
                    .collect::<Vec<_>>(),
            )
            .into_element(),
        "SidebarProvider" => sidebar_provider().child(kids()).into_element(),
        "Sidebar" => sidebar()
            .collapsible(match p.val("collapsible").as_str() {
                Some("none") => dazzui_nsh::SidebarCollapse::None,
                Some("icon") => dazzui_nsh::SidebarCollapse::Icon,
                _ => dazzui_nsh::SidebarCollapse::Offcanvas,
            })
            .variant(match p.val("variant").as_str() {
                Some("inset") => dazzui_nsh::SidebarVariant::Inset,
                Some("floating") => dazzui_nsh::SidebarVariant::Floating,
                _ => dazzui_nsh::SidebarVariant::Sidebar,
            })
            .child(kids())
            .into_element(),
        "SidebarHeader" => sidebar_header().child(kids()).into_element(),
        "SidebarContent" => sidebar_content().child(kids()).into_element(),
        "SidebarGroup" => sidebar_group().child(kids()).into_element(),
        "SidebarGroupContent" => view().child(kids()).into_element(),
        "SidebarGroupLabel" => sidebar_group_label().child(kids()).into_element(),
        "SidebarMenu" => sidebar_menu().child(kids()).into_element(),
        "SidebarMenuItem" => sidebar_menu_item().child(kids()).into_element(),
        "SidebarMenuBadge" => sidebar_menu_badge().child(kids()).into_element(),
        "SidebarInset" => sidebar_inset().child(kids()).into_element(),
        "SidebarTrigger" => sidebar_trigger().into_element(),
        "SidebarMenuButton" => {
            let action = p.action("onClick");
            sidebar_menu_button()
                .active(p.bool("active"))
                .tooltip(p.optional_string("tooltip"))
                .when_some(optional_slot(p, "icon", cx, id), |el, slot| el.icon(slot))
                .on_press(move || action(Value::Null))
                .child(kids())
                .into_element()
        }
        "Accordion" => {
            let action = p.action("onValueChange");
            let own = p.clone();
            accordion()
                .default_value(strings(&p.val("defaultValue")))
                .multiple(p.bool("multiple"))
                .divided(p.bool("divided"))
                .value(move || own.json.get("value").map(|_| strings(&own.val("value"))))
                .on_change(move |v: &Vec<SharedString>| {
                    action(serde_json::json!(
                        v.iter().map(|s| s.as_ref()).collect::<Vec<_>>()
                    ))
                })
                .child(kids())
                .into_element()
        }
        "AccordionItem" => accordion_item()
            .value(p.string("value"))
            .title(p.string("title"))
            .child(kids())
            .into_element(),
        "Code" => code().child(kids()).into_element(),
        "CodeBlock" => code_block()
            .code(p.string("code"))
            .title(p.optional_string("title"))
            .language(p.optional_string("language"))
            .copyable(p.bool("copyable"))
            .line_numbers(p.bool("lineNumbers"))
            .into_element(),
        "NumberField" => {
            let action = p.action("onValueChange");
            number_field()
                .default_value(p.val("defaultValue").as_f64())
                .min(p.val("min").as_f64())
                .max(p.val("max").as_f64())
                .step(p.number("step", 1.0))
                .unit(p.optional_string("unit"))
                .accessibility_label(p.optional_string("aria-label"))
                .on_change(move |v: &Option<f64>| action(serde_json::json!(v)))
                .into_element()
        }
        "OtpField" => {
            let action = p.action("onValueChange");
            let count = p.number("length", 6.0)() as u32;
            let group = p.val("groupAfter").as_u64().unwrap_or(0) as u32;
            otp_field()
                .length(count)
                .groups(if group > 0 { count.div_ceil(group) } else { 1 })
                .size(p.size(Size::Medium))
                .error(p.val("state").as_str() == Some("error"))
                .accessibility_label(p.optional_string("aria-label"))
                .on_change(move |v: &SharedString| action(Value::String(v.to_string())))
                .into_element()
        }
        "SearchField" => {
            let action = p.action("onValueChange");
            search_field()
                .value(p.optional_string("value"))
                .placeholder(p.string("placeholder"))
                .accessibility_label(p.optional_string("aria-label"))
                .on_change(move |v: &SharedString| action(Value::String(v.to_string())))
                .into_element()
        }
        "Combobox" => {
            let action = p.action("onValueChange");
            combobox()
                .items(select_items(&p.val("options")))
                .default_value(strings(&p.val("defaultValue")))
                .placeholder(p.optional_string("placeholder"))
                .clearable(p.bool("clearable"))
                .when_some(optional_slot(p, "empty", cx, id), |el, slot| el.empty(slot))
                .on_change(move |v: &Vec<SharedString>| {
                    action(
                        v.first()
                            .map(|s| Value::String(s.to_string()))
                            .unwrap_or_default(),
                    )
                })
                .into_element()
        }
        "Skeleton" => skeleton()
            .style_with({
                let p = p.clone();
                move |s| {
                    let value = p.val("width");
                    if let Some(percent) = value
                        .as_str()
                        .and_then(|v| v.strip_suffix('%'))
                        .and_then(|v| v.parse::<f32>().ok())
                    {
                        s.w(relative(percent / 100.0))
                    } else {
                        s
                    }
                }
            })
            .width(length(&p.val("width")))
            .height(length(&p.val("height")))
            .shape(match p.val("shape").as_str() {
                Some("circle") => dazzui_nsh::SkeletonShape::Circle,
                Some("rect") => dazzui_nsh::SkeletonShape::Rect,
                _ => dazzui_nsh::SkeletonShape::Text,
            })
            .into_element(),
        "SkeletonText" => skeleton_text()
            .lines(p.number("lines", 3.0)() as u32)
            .into_element(),
        "Spinner" => spinner()
            .size(p.size(Size::Medium))
            .on_accent(p.bool("onAccent"))
            .into_element(),
        "Meter" => {
            let value = p.number("value", 0.0);
            meter()
                .value(move || value() as f32)
                .label(p.optional_string("label"))
                .show_value(p.bool("showValue"))
                .size(p.size(Size::Medium))
                .tint(p.tint())
                .accessibility_label(p.optional_string("aria-label"))
                .into_element()
        }
        "StepList" => step_list().child(kids()).into_element(),
        "Step" => step()
            .label(p.string("label"))
            .meta(p.optional_string("meta"))
            .status(match p.val("status").as_str() {
                Some("done") => dazzui_nsh::StepStatus::Done,
                Some("active") => dazzui_nsh::StepStatus::Active,
                _ => dazzui_nsh::StepStatus::Idle,
            })
            .into_element(),
        "Table" => table().child(kids()).into_element(),
        "TableHead" => table_head().child(kids()).into_element(),
        "TableRow" => table_row().child(kids()).into_element(),
        "TableCell" => table_cell()
            .head(p.bool("head"))
            .align_end(p.val("align").as_str() == Some("end"))
            .child(kids())
            .into_element(),
        "OptionCard" => {
            let action = p.action("onClick");
            option_card()
                .title(p.string("title"))
                .description(p.optional_string("description"))
                .selected(p.bool("selected"))
                .disabled(p.bool("disabled"))
                .on_press(move || action(Value::Null))
                .child(kids())
                .into_element()
        }
        "AvatarGroup" => {
            let count = v["children"].as_array().map_or(0, Vec::len) as u32;
            let limit = p.number("max", count as f64)() as u32;
            avatar_group()
                .limit(limit)
                .overflow(text(format!("+{}", count.saturating_sub(limit))))
                .child(kids())
                .into_element()
        }
        "Link" => link()
            .href(p.optional_string("href"))
            .external(p.bool("external"))
            .inline(use_context::<super::InlineText>().is_some())
            .tint(p.tint())
            .underline(match p.val("underline").as_str() {
                Some("none") => dazzui_nsh::LinkUnderline::None,
                Some("hover") => dazzui_nsh::LinkUnderline::Hover,
                _ => dazzui_nsh::LinkUnderline::Always,
            })
            .child(kids())
            .into_element(),
        "PillTabs" => {
            let action = p.action("onValueChange");
            pill_tabs()
                .items(select_items(&p.val("items")))
                .value(p.optional_string("value"))
                .default_value(p.optional_string("defaultValue"))
                .accessibility_label(p.optional_string("aria-label"))
                .render_item(choice_renderer(p.raw("items"), cx, id, false))
                .on_change(move |v: &SharedString| action(Value::String(v.to_string())))
                .into_element()
        }
        "ToggleGroup" => {
            let action = p.action("onValueChange");
            let own = p.clone();
            let multiple = p.json.get("multiple").is_none() || p.bool("multiple")();
            toggle_group()
                .items(select_items(&p.val("items")))
                .icon_only_items(
                    p.val("items")
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter(|item| {
                            item["label"]["$el"] == "Icon" || item["label"].get("$icon").is_some()
                        })
                        .map(|item| SharedString::from(print(&item["value"])))
                        .collect::<Vec<_>>(),
                )
                .tint(p.tint())
                .variant(p.variant(Variant::Recessed))
                .multiple(multiple)
                .size(p.size(Size::Small))
                .value(move || own.json.get("value").map(|_| strings(&own.val("value"))))
                .accessibility_label(p.optional_string("aria-label"))
                .render_item(choice_renderer(p.raw("items"), cx, id, false))
                .on_change(move |v: &Vec<SharedString>| {
                    action(if multiple {
                        serde_json::json!(v.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
                    } else {
                        v.first()
                            .map(|s| Value::String(s.to_string()))
                            .unwrap_or_default()
                    })
                })
                .into_element()
        }
        "Tag" => {
            let selected = p.action("onSelectedChange");
            let remove = p.action("onRemove");
            tag()
                .tint(p.tint())
                .selected(p.optional_bool("selected"))
                .removable(p.json.get("onRemove").is_some())
                .on_selected_change(move |v: &bool| selected(Value::Bool(*v)))
                .on_remove(move || remove(Value::Null))
                .child(kids())
                .into_element()
        }
        "Dialog" => dazzui_nsh::dialog()
            .trigger(trigger(p.raw("trigger"), cx, id))
            .width(length(&p.val("width")))
            .danger(p.val("tone").as_str() == Some("danger"))
            .child(kids())
            .into_element(),
        "DialogHeader" => dialog_header()
            .title(p.string("title"))
            .subtitle(p.optional_string("subtitle"))
            .into_element(),
        "DialogBody" => dialog_body().child(kids()).into_element(),
        "DialogFooter" => dialog_footer().child(kids()).into_element(),
        "DialogClose" | "DrawerClose" | "PopoverClose" => dialog_close()
            .content(trigger(p.raw("render"), cx, id))
            .into_element(),
        "Popover" => popover_panel()
            .trigger(trigger(p.raw("trigger"), cx, id))
            .title(p.optional_string("title"))
            .description(p.optional_string("description"))
            .width(length(&p.val("width")))
            .side(side(&p.val("side")))
            .child(kids())
            .into_element(),
        "Drawer" => drawer()
            .trigger(trigger(p.raw("trigger"), cx, id))
            .title(p.string("title"))
            .description(p.optional_string("description"))
            .side(if p.raw("side").is_null() {
                Placement::Right
            } else {
                side(&p.val("side"))
            })
            .when_some(p.json.get("footer").cloned(), |el, v| {
                let id = id.to_owned();
                el.footer(move |_: &bool| render(v.clone(), cx, &id))
            })
            .child(kids())
            .into_element(),
        "PreviewCard" => {
            let width = length(&p.val("width"));
            preview_card()
                .trigger(trigger(p.raw("trigger"), cx, id))
                .popup_style(move |s| {
                    s.style_with(move |s| {
                        dazzui_nsh::styles::popup(s, theme.tokens())
                            .p(px(theme.tokens().spacing_4))
                            .w(px(width.unwrap_or(theme.tokens().preview_card_width)))
                    })
                })
                .child(kids())
                .into_element()
        }
        "Tooltip" => view()
            .child(kids())
            .tooltip(
                tooltip()
                    .placement(if p.raw("side").is_null() {
                        Placement::Top
                    } else {
                        side(&p.val("side"))
                    })
                    .child(text(p.string("label"))),
            )
            .into_element(),
        _ => return None,
    })
}

fn tree_items(v: &Value) -> Vec<dazzui_nsh::TreeItem> {
    v.as_array()
        .into_iter()
        .flatten()
        .map(|i| {
            dazzui_nsh::TreeItem::new(print(&i["id"]), print(&i["label"]))
                .disabled(truthy(&i["disabled"]))
                .children(tree_items(&i["children"]))
        })
        .collect()
}
fn find_tree<'a>(v: &'a Value, id: &str) -> Option<&'a Value> {
    for i in v.as_array().into_iter().flatten() {
        if i["id"].as_str() == Some(id) {
            return Some(i);
        }
        if let Some(found) = find_tree(&i["children"], id) {
            return Some(found);
        }
    }
    None
}
pub(super) fn menu_items(v: &Value, cx: Context, id: &str) -> Vec<AnyElement> {
    v.as_array()
        .into_iter()
        .flatten()
        .map(|i| {
            if i["type"].as_str() == Some("separator") {
                return divider().into_element();
            }
            if let Some(items) = i.get("items") {
                return nsh_base::submenu()
                    .trigger(menu_item().child(text(print(&i["label"]))))
                    .child(menu_items(items, cx, id))
                    .into_element();
            }
            let p = Props::new(i.clone(), cx);
            let action = p.action("onSelect");
            menu_item()
                .disabled(p.bool("disabled"))
                .checked(p.optional_bool("checked"))
                .on_select(move || action(Value::Null))
                .child(optional_slot(&p, "icon", cx, id))
                .child(text(p.string("label")))
                .child(view().flex_1())
                .child(optional_slot(&p, "shortcut", cx, id))
                .into_element()
        })
        .collect()
}

fn calendar_value(v: &Value, range: bool) -> dazzui_nsh::CalendarDate {
    let date = |v: &Value| v.as_str().and_then(dazzui_nsh::parse_date);
    if range {
        dazzui_nsh::CalendarDate::Range(date(&v["start"]), date(&v["end"]))
    } else {
        dazzui_nsh::CalendarDate::Single(date(v))
    }
}
fn calendar_json(v: &dazzui_nsh::CalendarDate) -> Value {
    match v {
        dazzui_nsh::CalendarDate::Single(d) => d
            .map(|d| Value::String(d.format("%Y-%m-%dT00:00").to_string()))
            .unwrap_or_default(),
        dazzui_nsh::CalendarDate::Range(a, b) => {
            serde_json::json!({"start":a.map(|d|d.format("%Y-%m-%dT00:00").to_string()),"end":b.map(|d|d.format("%Y-%m-%dT00:00").to_string())})
        }
    }
}
fn date_range(v: &Value) -> Option<dazzui_nsh::DateRangeValue> {
    Some(dazzui_nsh::DateRangeValue::new(
        v["start"].as_str()?.to_owned(),
        v["end"].as_str()?.to_owned(),
        truthy(&v["allDay"]),
    ))
}
