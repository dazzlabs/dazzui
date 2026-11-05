//! React's compiled state expressions and actions, without rebuilding native
//! widgets on each keystroke (which would discard focus and selection).
use dazzui_gtk4::{gtk, prelude::*};
use serde_json::{Map, Value};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

type Observer = Rc<dyn Fn()>;
struct State {
    slots: RefCell<Map<String, Value>>,
    observers: RefCell<Vec<Observer>>,
    notifying: Cell<bool>,
}
#[derive(Clone)]
pub struct Context(Rc<State>, Option<dazzui_gtk4::WeakToastQueue>);
impl Context {
    pub fn new(initial: &Value) -> Self {
        Self(
            Rc::new(State {
                slots: RefCell::new(initial.as_object().cloned().unwrap_or_default()),
                observers: RefCell::new(vec![]),
                notifying: Cell::new(false),
            }),
            None,
        )
    }
    pub fn with_toasts(&self, queue: &dazzui_gtk4::ToastQueue) -> Self {
        Self(self.0.clone(), Some(queue.downgrade()))
    }
    pub fn resolve(&self, value: &Value, args: &[Value]) -> Value {
        match value {
            Value::Array(items) => {
                Value::Array(items.iter().map(|v| self.resolve(v, args)).collect())
            }
            Value::Object(object) => {
                if object.contains_key("$do") {
                    return value.clone();
                }
                if let Some(slot) = object.get("$state").and_then(Value::as_str) {
                    return self.0.slots.borrow().get(slot).cloned().unwrap_or_default();
                }
                if let Some(index) = object.get("$arg").and_then(Value::as_u64) {
                    return args.get(index as usize).cloned().unwrap_or_default();
                }
                if let Some(items) = object.get("$eq").and_then(Value::as_array) {
                    return Value::Bool(equal(
                        &self.resolve(&items[0], args),
                        &self.resolve(&items[1], args),
                    ));
                }
                if let Some(test) = object.get("$not") {
                    return Value::Bool(!truthy(&self.resolve(test, args)));
                }
                if let Some(items) = object.get("$if").and_then(Value::as_array) {
                    return self.resolve(
                        &items[if truthy(&self.resolve(&items[0], args)) {
                            1
                        } else {
                            2
                        }],
                        args,
                    );
                }
                if let Some(items) = object.get("$concat").and_then(Value::as_array) {
                    return Value::String(
                        items.iter().map(|v| text(&self.resolve(v, args))).collect(),
                    );
                }
                Value::Object(
                    object
                        .iter()
                        .map(|(k, v)| (k.clone(), self.resolve(v, args)))
                        .collect(),
                )
            }
            _ => value.clone(),
        }
    }
    pub fn run(&self, action: &Value, args: &[Value]) {
        if self.0.notifying.get() {
            return;
        }
        let mut changed = false;
        for step in action["$do"].as_array().into_iter().flatten() {
            if let Some(options) = step.get("toast")
                && let Some(queue) = self.1.as_ref().and_then(|q| q.upgrade())
            {
                let options = self.resolve(options, args);
                let on_close = options.get("onClose").map(|action| {
                    let ctx = self.clone();
                    let action = action.clone();
                    Rc::new(move || ctx.run(&action, &[])) as Rc<dyn Fn()>
                });
                let action = options.get("action").map(|a| {
                    let ctx = self.clone();
                    let action = a["onClick"].clone();
                    dazzui_gtk4::ToastAction {
                        label: text(&a["label"]),
                        on_click: Rc::new(move || ctx.run(&action, &[])),
                    }
                });
                queue.add(dazzui_gtk4::ToastOptions {
                    id: options["id"].as_str().map(str::to_owned),
                    title: text(&options["title"]),
                    tint: super::scene::tint(&options, "neutral"),
                    icon: options["icon"]["$icon"]
                        .as_str()
                        .map(|name| dazzui_gtk4::icon(name).upcast()),
                    action,
                    dismissable: options["dismissable"].as_bool().unwrap_or(true),
                    dismiss_label: options["dismissLabel"].as_str().unwrap_or("Dismiss").into(),
                    timeout: options["timeout"]
                        .as_f64()
                        .map(|ms| std::time::Duration::from_secs_f64(ms / 1000.)),
                    high_priority: options["priority"].as_str().map(|p| p == "high"),
                    on_close,
                });
            }
            if let Some(slot) = step["set"].as_str() {
                let next = self.resolve(&step["to"], args);
                let mut slots = self.0.slots.borrow_mut();
                if !slots.get(slot).is_some_and(|old| equal(old, &next)) {
                    slots.insert(slot.into(), next);
                    changed = true;
                }
            }
        }
        if changed {
            let observers = self.0.observers.borrow().clone();
            self.0.notifying.set(true);
            for update in observers {
                update();
            }
            self.0.notifying.set(false);
        }
    }
    pub fn bind<W: IsA<gtk::Widget> + Clone + 'static>(
        &self,
        widget: &W,
        update: impl Fn(&W, &Context) + 'static,
    ) {
        update(widget, self);
        let weak = widget.downgrade();
        let context = Rc::downgrade(&self.0);
        let toasts = self.1.clone();
        self.0.observers.borrow_mut().push(Rc::new(move || {
            if let (Some(widget), Some(context)) = (weak.upgrade(), context.upgrade()) {
                update(&widget, &Context(context, toasts.clone()));
            }
        }));
    }
    pub fn wire(&self, kind: &str, widget: &gtk::Widget, props: &Value) {
        let change = |name: &str| {
            let ctx = self.clone();
            let action = props[name].clone();
            move |value: Value| ctx.run(&action, &[value])
        };
        match kind {
            "Button" | "IconButton" | "PreferenceRow" => {
                if let Some(button) = widget.downcast_ref::<gtk::Button>() {
                    let run = change(if kind == "PreferenceRow" {
                        "onOpen"
                    } else {
                        "onClick"
                    });
                    button.connect_clicked(move |_| run(Value::Null));
                }
            }
            "OptionCard" => {
                if let Some(card) = widget.downcast_ref::<gtk::CheckButton>() {
                    let selected = props["selected"].clone();
                    let value = selected.clone();
                    self.bind(card, move |card, ctx| {
                        card.set_active(ctx.resolve(&value, &[]).as_bool().unwrap_or(false))
                    });
                    let ctx = self.clone();
                    let action = props["onClick"].clone();
                    let restoring = Cell::new(false);
                    card.connect_toggled(move |card| {
                        if restoring.get() {
                            return;
                        }
                        ctx.run(&action, &[]);
                        restoring.set(true);
                        card.set_active(ctx.resolve(&selected, &[]).as_bool().unwrap_or(false));
                        restoring.set(false);
                    });
                }
            }
            "Checkbox" => {
                if let Some(check) = widget.downcast_ref::<gtk::CheckButton>() {
                    if let Some(value) = props.get("checked") {
                        let value = value.clone();
                        self.bind(check, move |w, ctx| {
                            w.set_active(ctx.resolve(&value, &[]).as_bool().unwrap_or(false))
                        });
                    }
                    let run = change("onCheckedChange");
                    check.connect_toggled(move |w| run(Value::Bool(w.is_active())));
                }
            }
            "Switch" => {
                if let Some(switch) = widget.downcast_ref::<gtk::Switch>() {
                    if let Some(value) = props.get("checked") {
                        let value = value.clone();
                        self.bind(switch, move |w, ctx| {
                            w.set_active(ctx.resolve(&value, &[]).as_bool().unwrap_or(false))
                        });
                    }
                    let run = change("onCheckedChange");
                    switch.connect_active_notify(move |w| run(Value::Bool(w.is_active())));
                }
            }
            "Toggle" | "Tag" => {
                if let Some(toggle) = widget.downcast_ref::<gtk::ToggleButton>() {
                    let key = if kind == "Tag" { "selected" } else { "pressed" };
                    if let Some(value) = props.get(key) {
                        let value = value.clone();
                        self.bind(toggle, move |w, ctx| {
                            w.set_active(ctx.resolve(&value, &[]).as_bool().unwrap_or(false))
                        });
                    }
                    let run = change(if kind == "Tag" {
                        "onSelectedChange"
                    } else {
                        "onPressedChange"
                    });
                    toggle.connect_toggled(move |w| run(Value::Bool(w.is_active())));
                }
            }
            "TextField" | "SearchField" => {
                if let Ok(editable) = widget.clone().dynamic_cast::<gtk::Editable>() {
                    if let Some(value) = props.get("value") {
                        let value = value.clone();
                        self.bind(widget, move |w, ctx| {
                            let editable = w.clone().dynamic_cast::<gtk::Editable>().unwrap();
                            let next = text(&ctx.resolve(&value, &[]));
                            if editable.text().as_str() != next {
                                editable.set_text(&next);
                            }
                        });
                    }
                    let run = change("onValueChange");
                    editable.connect_changed(move |w| run(Value::String(w.text().into())));
                }
            }
            "OtpField" => {
                if let Some(field) = widget.downcast_ref::<dazzui_gtk4::OtpField>() {
                    let source = props.clone();
                    self.bind(field, move |field, ctx| {
                        let p = ctx.resolve(&source, &[]);
                        if source.get("value").is_some() {
                            field.set_value(p["value"].as_str().unwrap_or(""));
                        }
                        field.set_read_only(p["readOnly"].as_bool().unwrap_or(false));
                        field.set_mask(p["mask"].as_bool().unwrap_or(false));
                        field.set_sensitive(!p["disabled"].as_bool().unwrap_or(false));
                    });
                    let action = props["onValueChange"].clone();
                    let ctx = self.clone();
                    field.connect_value_changed(move |_, edit| {
                        ctx.run(
                            &action,
                            &[
                                Value::String(edit.value.clone()),
                                serde_json::json!({"reason":edit.reason.as_str()}),
                            ],
                        )
                    });
                    let action = props["onValueComplete"].clone();
                    let ctx = self.clone();
                    field.connect_value_complete(move |_, value, reason| {
                        ctx.run(
                            &action,
                            &[
                                Value::String(value.into()),
                                serde_json::json!({"reason":reason}),
                            ],
                        )
                    });
                    let action = props["onValueInvalid"].clone();
                    let ctx = self.clone();
                    field.connect_value_invalid(move |_, value, reason| {
                        ctx.run(
                            &action,
                            &[
                                Value::String(value.into()),
                                serde_json::json!({"reason":reason}),
                            ],
                        )
                    });
                }
            }
            "ShortcutRecorder" => {
                if let Some(recorder) = widget.downcast_ref::<dazzui_gtk4::ShortcutRecorder>() {
                    let source = props.clone();
                    self.bind(recorder, move |recorder, ctx| {
                        let p = ctx.resolve(&source, &[]);
                        recorder.set_value(p["value"].as_str().unwrap_or(""));
                        recorder.set_prompts(
                            p["placeholder"].as_str().unwrap_or("Record shortcut"),
                            p["recordingLabel"].as_str().unwrap_or("Press keys…"),
                            p["clearLabel"].as_str().unwrap_or("Clear shortcut"),
                        );
                        recorder.set_invalid(p["state"] == "error");
                        recorder.set_sensitive(!p["disabled"].as_bool().unwrap_or(false));
                        recorder.set_dazz_tint(match p["tint"].as_str().unwrap_or("primary") {
                            "neutral" => dazzui_gtk4::Tint::Neutral,
                            "info" => dazzui_gtk4::Tint::Info,
                            "success" => dazzui_gtk4::Tint::Success,
                            "warning" => dazzui_gtk4::Tint::Warning,
                            "danger" => dazzui_gtk4::Tint::Danger,
                            _ => dazzui_gtk4::Tint::Primary,
                        });
                        if let Some(label) = p["aria-label"].as_str() {
                            recorder.button().set_dazz_label(label);
                        } else {
                            recorder
                                .button()
                                .reset_property(gtk::AccessibleProperty::Label);
                        }
                    });
                    let run = change("onValueChange");
                    recorder
                        .connect_value_changed(move |_, value| run(Value::String(value.into())));
                    if let Some(action) = props.get("onClear").filter(|v| !v.is_null()) {
                        let action = action.clone();
                        let ctx = self.clone();
                        recorder.connect_clear(move |_| ctx.run(&action, &[]));
                    }
                }
            }
            "CodeBlock" => {
                if let Some(block) = widget.downcast_ref::<gtk::Box>() {
                    let source = dazzui_gtk4::code_block_source(block);
                    let value = props["code"].clone();
                    self.bind(&source, move |source, ctx| {
                        let value = ctx.resolve(&value, &[]);
                        let body = value.as_str().unwrap_or("").trim_end_matches('\n');
                        if source.text().as_str() != body {
                            source.set_label(body);
                        }
                    });
                }
            }
            "NumberField" => {
                if let Some(input) = widget.first_child().and_downcast::<gtk::SpinButton>() {
                    if let Some(value) = props.get("value") {
                        let value = value.clone();
                        self.bind(&input, move |input, ctx| {
                            let next = ctx.resolve(&value, &[]).as_f64().unwrap_or(0.);
                            if input.value() != next {
                                input.set_value(next);
                            }
                        });
                    }
                    let run = change("onValueChange");
                    input.connect_value_changed(move |input| run(Value::from(input.value())));
                }
            }
            "Select" => {
                if let Some(select) = widget.downcast_ref::<gtk::DropDown>() {
                    let options = dazzui_gtk4::select_values(&super::scene::select_items(
                        &self.resolve(&props["options"], &[]),
                    ));
                    if let Some(value) = props.get("value") {
                        let value = value.clone();
                        let options = options.clone();
                        self.bind(select, move |w, ctx| {
                            let value = ctx.resolve(&value, &[]);
                            let index = options
                                .iter()
                                .position(|o| o.as_deref() == value.as_str())
                                .map(|i| i as u32)
                                .unwrap_or(0);
                            if w.selected() != index {
                                w.set_selected(index);
                            }
                        });
                    }
                    let run = change("onValueChange");
                    select.connect_selected_notify(move |w| {
                        if let Some(Some(value)) = options.get(w.selected() as usize) {
                            run(Value::String(value.clone()));
                        }
                    });
                }
            }
            "Collapsible" => {
                if let Some(disclosure) = widget.downcast_ref::<gtk::Expander>() {
                    if let Some(value) = props.get("open") {
                        let value = value.clone();
                        self.bind(disclosure, move |w, ctx| {
                            w.set_expanded(ctx.resolve(&value, &[]).as_bool().unwrap_or(false))
                        });
                    }
                    let run = change("onOpenChange");
                    disclosure.connect_expanded_notify(move |w| run(Value::Bool(w.is_expanded())));
                }
            }
            _ => (),
        }
    }
}
pub fn text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            if let Some(number) = n.as_f64().filter(|_| n.is_f64()) {
                if number == 0. {
                    "0".into()
                } else {
                    number.to_string()
                }
            } else {
                n.to_string()
            }
        }
        Value::Bool(b) => b.to_string(),
        _ => String::new(),
    }
}
fn equal(left: &Value, right: &Value) -> bool {
    // Native numeric controls report f64 even for whole numbers. Compiled
    // React expressions have one number type, so 14 and 14.0 are equal.
    match (left.as_number(), right.as_number()) {
        (Some(left), Some(right)) => left.as_f64() == right.as_f64(),
        _ => left == right,
    }
}
fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().unwrap_or(0.) != 0.,
        Value::String(s) => !s.is_empty(),
        _ => true,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn actions_resolve_arguments_negation_comparison_and_text() {
        let ctx = Context::new(&json!({"s0":false,"s1":"One"}));
        ctx.run(&json!({"$do":[{"set":"s0","to":{"$not":{"$state":"s0"}}},{"set":"s1","to":{"$arg":0}}]}), &[json!("Two")]);
        assert_eq!(ctx.resolve(&json!({"$if":[{"$eq":[{"$state":"s0"},true]},{"$concat":["Page ",{"$state":"s1"}]},"Empty"]}), &[]), json!("Page Two"));
    }
    #[test]
    fn native_numbers_keep_react_scalar_text_and_equality() {
        let ctx = Context::new(&json!({"n":14.0}));
        assert_eq!(
            ctx.resolve(&json!({"$eq":[{"$state":"n"},14]}), &[]),
            json!(true)
        );
        assert_eq!(text(&json!(14.0)), "14");
        assert_eq!(text(&json!(1.75)), "1.75");
        assert_eq!(text(&json!(-0.0)), "0");
    }
}
