//! Portable story expressions, bound to NSH signals instead of remounting views.
use nsh_core::prelude::*;
use serde_json::{Map, Value};
#[derive(Clone, Copy)]
pub struct Context(Signal<Map<String, Value>>);
impl Context {
    pub fn new(initial: &Value) -> Self {
        Self(signal(initial.as_object().cloned().unwrap_or_default()))
    }
    pub fn resolve(self, v: &Value, args: &[Value]) -> Value {
        match v {
            Value::Array(values) => {
                Value::Array(values.iter().map(|v| self.resolve(v, args)).collect())
            }
            Value::Object(o) => {
                if o.contains_key("$do") {
                    return v.clone();
                }
                if let Some(key) = o.get("$state").and_then(Value::as_str) {
                    return self.0.with(|s| s.get(key).cloned().unwrap_or_default());
                }
                if let Some(i) = o.get("$arg").and_then(Value::as_u64) {
                    return args.get(i as usize).cloned().unwrap_or_default();
                }
                if let Some(v) = o.get("$not") {
                    return Value::Bool(!truthy(&self.resolve(v, args)));
                }
                if let Some(v) = o.get("$eq").and_then(Value::as_array) {
                    return Value::Bool(self.resolve(&v[0], args) == self.resolve(&v[1], args));
                }
                if let Some(v) = o.get("$if").and_then(Value::as_array) {
                    return self.resolve(
                        &v[if truthy(&self.resolve(&v[0], args)) {
                            1
                        } else {
                            2
                        }],
                        args,
                    );
                }
                if let Some(v) = o.get("$concat").and_then(Value::as_array) {
                    return Value::String(
                        v.iter().map(|v| print(&self.resolve(v, args))).collect(),
                    );
                }
                Value::Object(
                    o.iter()
                        .map(|(k, v)| (k.clone(), self.resolve(v, args)))
                        .collect(),
                )
            }
            _ => v.clone(),
        }
    }
    pub fn run(self, action: &Value, args: &[Value]) {
        batch(|| {
            for step in action["$do"].as_array().into_iter().flatten() {
                if let Some(slot) = step["set"].as_str() {
                    let next = self.resolve(&step["to"], args);
                    self.0.update(|s| {
                        s.insert(slot.to_owned(), next);
                    });
                }
                if let Some(notice) = step.get("toast")
                    && let Some(handle) = dazzui_nsh::ToastHandle::current()
                {
                    let notice = self.resolve(notice, args);
                    let mut toast = dazzui_nsh::ToastNotice::new(print(&notice["title"]));
                    toast.description = notice.get("description").map(|v| print(v).into());
                    toast.tint =
                        dazzui_nsh::Tint::parse(&print(&notice["tint"])).unwrap_or_default();
                    if let Some(action) = notice.get("action") {
                        let callback = action["onClick"].clone();
                        toast.action = Some((
                            print(&action["label"]).into(),
                            std::rc::Rc::new(move || self.run(&callback, &[])),
                        ));
                    }
                    handle.push(toast);
                }
            }
        });
    }
}
pub fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(v) => *v,
        Value::String(v) => !v.is_empty(),
        Value::Number(v) => v.as_f64() != Some(0.0),
        _ => true,
    }
}
pub fn print(v: &Value) -> String {
    match v {
        Value::String(v) => v.clone(),
        Value::Null => String::new(),
        _ => v.to_string(),
    }
}
