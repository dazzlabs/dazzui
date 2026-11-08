//! The text controls and the form around them.

use std::collections::HashMap;

use dazzui::{
    Autocomplete, Combobox, FieldState, FieldValue, Fieldset, Form, FormField, NumberField,
    OtpField, SearchField, SelectEntry, SelectOption, ShortcutRecorder, SidebarInput, Size,
    TextField,
};
use gpui::{AnyElement, App, IntoElement, ParentElement, Window};
use serde_json::Value;

use super::BuildFn;
use crate::lib::scene::{Node, text, truthy};

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("TextField", text_field);
    map.insert("SidebarInput", sidebar_input);
    map.insert("FieldValue", field_value);
    map.insert("SearchField", search_field);
    map.insert("NumberField", number_field);
    map.insert("OtpField", otp_field);
    map.insert("ShortcutRecorder", shortcut_recorder);
    map.insert("Form", form);
    map.insert("Fieldset", fieldset);
    map.insert("FormField", form_field);
    map.insert("Combobox", combobox);
    map.insert("Autocomplete", autocomplete);
}

fn state(node: &Node) -> FieldState {
    node.option("state", FieldState::parse).unwrap_or_default()
}

fn size(node: &Node) -> Size {
    node.option("size", Size::parse).unwrap_or(Size::Medium)
}

fn text_field(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut field = TextField::new(node.id())
        .placeholder(node.string("placeholder").unwrap_or_default())
        .input_type(node.string("type").as_deref().unwrap_or("text"))
        .required(node.flag("required"))
        .state(state(node))
        .read_only(node.flag("readOnly"))
        .mono(node.flag("mono"))
        .size(size(node))
        .disabled(node.flag("disabled"))
        .multiline(node.flag("multiline"))
        .aria_label(node.string("aria-label").unwrap_or_default());
    if let Some(value) = node.string("defaultValue") {
        field = field.default_value(value);
    }
    if let Some(rows) = node.number("rows") {
        field = field.rows(rows.max(1.0) as usize);
    }
    field.into_any_element()
}

fn field_value(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    FieldValue::new(node.id())
        .size(size(node))
        .mono(node.flag("mono"))
        .children(node.kids(window, cx))
        .into_any_element()
}

fn search_field(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut field = SearchField::new(node.id(), node.string("value").unwrap_or_default())
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .aria_label(node.string("aria-label").unwrap_or_default());
    if let Some(placeholder) = node.string("placeholder") {
        field = field.placeholder(placeholder);
    }
    if let Some(change) = node.handler("onValueChange") {
        field = field.on_value_change(move |value, window, cx| {
            change(Value::String(value.to_string()), window, cx)
        });
    }
    field.into_any_element()
}

fn number_field(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut field = NumberField::new(node.id())
        .aria_label(node.string("aria-label").unwrap_or_default())
        .size(size(node))
        .state(state(node))
        .disabled(node.flag("disabled"))
        .read_only(node.flag("readOnly"))
        .allow_wheel_scrub(node.flag("allowWheelScrub"))
        .allow_out_of_range(node.flag("allowOutOfRange"))
        .snap_on_step(node.flag("snapOnStep"));
    if let Some(value) = node.option("tint", dazzui::Tint::parse) {
        field = field.tint(value);
    }
    if let Some(value) = node.string("placeholder") {
        field = field.placeholder(value);
    }
    if node.has("value") {
        field = field.value(node.number("value"));
    }
    if let Some(change) = node.handler("onValueChange") {
        field = field.on_value_change(move |value, window, cx| {
            change(value.map(Value::from).unwrap_or(Value::Null), window, cx)
        });
    }
    if let Some(commit) = node.handler("onValueCommitted") {
        field = field.on_value_committed(move |value, _, window, cx| {
            commit(value.map(Value::from).unwrap_or(Value::Null), window, cx)
        });
    }
    if let Some(value) = node.number("smallStep") {
        field = field.small_step(value);
    }
    if let Some(value) = node.number("largeStep") {
        field = field.large_step(value);
    }
    if let Some(value) = node.number("defaultValue") {
        field = field.default_value(value);
    }
    if let Some(unit) = node.element("unit", window, cx) {
        field = field.unit(unit);
    }
    if let Some(min) = node.number("min") {
        field = field.min(min);
    }
    if let Some(max) = node.number("max") {
        field = field.max(max);
    }
    if let Some(step) = node.number("step") {
        field = field.step(step);
    }
    field.into_any_element()
}

fn otp_field(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let length = node.number("length").unwrap_or(6.0).max(1.0) as usize;
    let mut field = OtpField::new(node.id(), length)
        .size(size(node))
        .state(state(node))
        .aria_label(node.string("aria-label").unwrap_or_default());
    if let Some(after) = node.number("groupAfter") {
        field = field.group_after(after as usize);
    }
    field.into_any_element()
}

fn shortcut_recorder(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut recorder = ShortcutRecorder::new(node.id(), node.string("value").unwrap_or_default())
        .state(state(node))
        .disabled(node.flag("disabled"))
        .aria_label(node.string("aria-label").unwrap_or_default());
    if let Some(change) = node.handler("onValueChange") {
        recorder = recorder.on_value_change(move |value, window, cx| {
            change(Value::String(value.to_string()), window, cx)
        });
    }
    if let Some(clear) = node.action("onClear") {
        recorder = recorder.on_clear(move |window, cx| clear(window, cx));
    }
    recorder.into_any_element()
}

fn form(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut form = Form::new().children(node.kids(window, cx));
    if let Some(submit) = node.action("onSubmit") {
        form = form.on_submit(move |window, cx| submit(window, cx));
    }
    form.into_any_element()
}

fn fieldset(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut fieldset = Fieldset::new().children(node.kids(window, cx));
    if let Some(legend) = node.element("legend", window, cx) {
        fieldset = fieldset.legend(legend);
    }
    fieldset.into_any_element()
}

fn form_field(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let label = node
        .element("label", window, cx)
        .unwrap_or_else(|| gpui::Empty.into_any_element());
    let mut field = FormField::new(label)
        .state(state(node))
        .children(node.kids(window, cx));
    if let Some(hint) = node.element("hint", window, cx) {
        field = field.hint(hint);
    }
    field.into_any_element()
}

fn combobox_option(node: &Node, value: &Value) -> SelectOption {
    let read = |key: &str| {
        value
            .get(key)
            .map(|value| node.ctx.resolve(value, &[]))
            .unwrap_or(Value::Null)
    };
    SelectOption::new(text(&read("value")), text(&read("label")))
        .disabled(truthy(&read("disabled")))
}

fn combobox(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let options: Vec<SelectEntry> = node
        .raw("options")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(
            |entry| match entry.get("options").and_then(Value::as_array) {
                Some(options) => SelectEntry::group(
                    text(&node.ctx.resolve(&entry["label"], &[])),
                    options
                        .iter()
                        .map(|option| combobox_option(node, option))
                        .collect(),
                ),
                None => SelectEntry::Option(combobox_option(node, entry)),
            },
        )
        .collect();
    let mut combobox = Combobox::new(node.id()).options(options);
    if let Some(value) = node.string("defaultValue") {
        combobox = combobox.default_value(value);
    }
    if let Some(placeholder) = node.string("placeholder") {
        combobox = combobox.placeholder(placeholder);
    }
    if let Some(empty) = node.element("empty", window, cx) {
        combobox = combobox.empty(empty);
    }
    if node.has("clearable") {
        combobox = combobox.clearable(node.flag("clearable"));
    }
    combobox.into_any_element()
}

fn autocomplete(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let items: Vec<String> = match node.get("items") {
        Value::Array(items) => items.iter().map(text).collect(),
        _ => Vec::new(),
    };
    let mut autocomplete = Autocomplete::new(node.id()).items(items).size(size(node));
    if let Some(placeholder) = node.string("placeholder") {
        autocomplete = autocomplete.placeholder(placeholder);
    }
    autocomplete.into_any_element()
}

fn sidebar_input(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut field = SidebarInput::new(node.id())
        .placeholder(node.string("placeholder").unwrap_or_default())
        .input_type(node.string("type").as_deref().unwrap_or("text"))
        .required(node.flag("required"))
        .state(state(node))
        .read_only(node.flag("readOnly"))
        .mono(node.flag("mono"))
        .disabled(node.flag("disabled"))
        .multiline(node.flag("multiline"))
        .aria_label(node.string("aria-label").unwrap_or_default());
    if let Some(size) = node.option("size", Size::parse) {
        field = field.size(size);
    }
    if let Some(tint) = node.option("tint", dazzui::Tint::parse) {
        field = field.tint(tint);
    }
    if let Some(value) = node.string("defaultValue") {
        field = field.default_value(value);
    }
    if let Some(value) = node.string("value") {
        field = field.value(value);
    }
    if let Some(rows) = node.number("rows") {
        field = field.rows(rows.max(1.) as usize);
    }
    if let Some(change) = node.handler("onValueChange") {
        field = field.on_change(move |value, window, cx| {
            change(Value::String(value.to_string()), window, cx)
        });
    }
    field.into_any_element()
}
