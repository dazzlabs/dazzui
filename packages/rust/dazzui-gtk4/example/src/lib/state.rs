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
pub struct Context(Rc<State>);
impl Context {
    pub fn new(initial: &Value) -> Self {
        Self(Rc::new(State {
            slots: RefCell::new(initial.as_object().cloned().unwrap_or_default()),
            observers: RefCell::new(vec![]),
            notifying: Cell::new(false),
        }))
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
                    return Value::Bool(
                        self.resolve(&items[0], args) == self.resolve(&items[1], args),
                    );
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
            if let Some(slot) = step["set"].as_str() {
                let next = self.resolve(&step["to"], args);
                let mut slots = self.0.slots.borrow_mut();
                if slots.get(slot) != Some(&next) {
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
        self.0.observers.borrow_mut().push(Rc::new(move || {
            if let (Some(widget), Some(context)) = (weak.upgrade(), context.upgrade()) {
                update(&widget, &Context(context));
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
            "Button" | "IconButton" => {
                if let Some(button) = widget.downcast_ref::<gtk::Button>() {
                    let run = change("onClick");
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
            "Slider" => {
                if let Some(slider) = widget.downcast_ref::<gtk::Scale>() {
                    if let Some(value) = props.get("value") {
                        let value = value.clone();
                        self.bind(slider, move |w, ctx| {
                            w.set_value(ctx.resolve(&value, &[]).as_f64().unwrap_or(0.))
                        });
                    }
                    let run = change("onValueChange");
                    slider.connect_value_changed(move |w| run(Value::from(w.value())));
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
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => String::new(),
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
}
