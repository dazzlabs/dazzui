//! Framework state for generated scenes. Content and actions come from declarations.
use dazzui::TextBuffer;
use gpui::{AppContext, Context, Entity, Subscription};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Text(String),
    Number(f64),
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl Value {
    pub fn number(&self) -> f64 {
        match self {
            Self::Number(value) => *value,
            _ => panic!("validated numeric expression"),
        }
    }
    pub fn boolean(&self) -> bool {
        match self {
            Self::Boolean(value) => *value,
            _ => panic!("validated boolean binding"),
        }
    }
    pub fn text(&self) -> &str {
        match self {
            Self::Text(value) => value,
            _ => panic!("validated text binding"),
        }
    }
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(value) => value.fmt(f),
            Self::Text(value) => value.fmt(f),
            Self::Number(value) => value.fmt(f),
        }
    }
}

pub struct Field {
    pub key: &'static str,
    pub binding: &'static str,
    pub placeholder: &'static str,
    pub read_only: bool,
}
pub struct Timer {
    pub every: u64,
    pub update: fn(&mut crate::Playground, &mut Context<crate::Playground>),
}
#[derive(Default)]
pub struct Definition {
    pub values: BTreeMap<&'static str, Value>,
    pub fields: Vec<Field>,
    pub timers: Vec<Timer>,
}
#[derive(Default)]
pub struct SceneState {
    grid_columns: std::cell::RefCell<BTreeMap<&'static str, std::rc::Rc<std::cell::Cell<u16>>>>,
    pub disabled_values: std::cell::RefCell<BTreeMap<&'static str, bool>>,
    pub values: BTreeMap<&'static str, Value>,
    pub fields: BTreeMap<&'static str, Entity<TextBuffer>>,
    bindings: BTreeMap<&'static str, &'static str>,
    // Dropping the scene also disconnects all its native input observers.
    _subscriptions: Vec<Subscription>,
    _timers: Vec<gpui::Task<()>>,
}
impl SceneState {
    pub fn grid_columns(&self, key: &'static str) -> std::rc::Rc<std::cell::Cell<u16>> {
        self.grid_columns
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| std::rc::Rc::new(std::cell::Cell::new(1)))
            .clone()
    }
    pub fn record_disabled(&self, id: &'static str, disabled: bool) -> bool {
        self.disabled_values.borrow_mut().insert(id, disabled);
        disabled
    }

    pub fn new(definition: Definition, cx: &mut Context<crate::Playground>) -> Self {
        let mut state = Self {
            values: definition.values,
            ..Default::default()
        };
        for timer in definition.timers {
            state._timers.push(cx.spawn(async move |host, cx| {
                loop {
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(timer.every))
                        .await;
                    if host
                        .update(cx, |host, cx| (timer.update)(host, cx))
                        .is_err()
                    {
                        break;
                    }
                }
            }));
        }
        for field in definition.fields {
            let buffer = cx.new(|cx| {
                TextBuffer::new(cx, field.placeholder)
                    .with_value(state.values[field.binding].text().to_owned())
                    .read_only(field.read_only)
            });
            let subscription = cx.observe(&buffer, move |host, buffer, cx| {
                let value = Value::Text(buffer.read(cx).value().to_string());
                host.generated.assign(field.binding, value, cx);
            });
            state.fields.insert(field.key, buffer);
            state.bindings.insert(field.key, field.binding);
            state._subscriptions.push(subscription);
        }
        state
    }
    pub fn assign(&mut self, key: &'static str, value: Value, cx: &mut Context<crate::Playground>) {
        if self.values.get(key) == Some(&value) {
            return;
        }
        self.values.insert(key, value.clone());
        if let Value::Text(text) = value {
            for (field, binding) in &self.bindings {
                if *binding == key && self.fields[field].read(cx).value().as_ref() != text {
                    self.fields[field].update(cx, |buffer, cx| buffer.set_value(text.clone(), cx));
                }
            }
        }
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn replacing_a_scene_cancels_its_timers(cx: &mut gpui::TestAppContext) {
        let host = cx.new(crate::Playground::new);
        host.update(cx, |host, cx| {
            host.generated = SceneState::new(
                crate::generated_stories::definition("components-progress--live"),
                cx,
            );
        });
        cx.run_until_parked();
        cx.background_executor
            .advance_clock(std::time::Duration::from_millis(400));
        cx.run_until_parked();
        assert_eq!(
            host.read_with(cx, |host, _| host.generated.values["value"].clone()),
            Value::Number(12.0)
        );
        host.update(cx, |host, _| {
            host.generated = SceneState::default();
            host.generated.values.insert("value", Value::Number(71.0));
        });
        cx.background_executor
            .advance_clock(std::time::Duration::from_millis(400));
        cx.run_until_parked();
        assert_eq!(
            host.read_with(cx, |host, _| host.generated.values["value"].clone()),
            Value::Number(71.0)
        );
    }

    #[gpui::test]
    fn bound_inputs_share_values_and_release_old_observers(cx: &mut gpui::TestAppContext) {
        let host = cx.new(crate::Playground::new);
        host.update(cx, |host, cx| {
            host.generated = SceneState::new(
                Definition {
                    values: BTreeMap::from([("text", Value::Text("Initial".into()))]),
                    timers: vec![],
                    fields: vec![
                        Field {
                            key: "editor",
                            binding: "text",
                            placeholder: "Edit",
                            read_only: false,
                        },
                        Field {
                            key: "mirror",
                            binding: "text",
                            placeholder: "Preview",
                            read_only: true,
                        },
                    ],
                },
                cx,
            );
        });
        let (editor, mirror) = host.read_with(cx, |host, _| {
            (
                host.generated.fields["editor"].clone(),
                host.generated.fields["mirror"].clone(),
            )
        });
        assert_ne!(editor.entity_id(), mirror.entity_id());
        assert_eq!(
            editor.read_with(cx, |field, _| field.placeholder().clone()),
            "Edit"
        );
        assert_eq!(
            mirror.read_with(cx, |field, _| field.placeholder().clone()),
            "Preview"
        );
        editor.update(cx, |field, cx| field.set_value("Edited", cx));
        cx.run_until_parked();
        assert_eq!(
            host.read_with(cx, |host, _| host.generated.values["text"].clone()),
            Value::Text("Edited".into())
        );
        assert_eq!(
            mirror.read_with(cx, |field, _| field.value().clone()),
            "Edited"
        );
        host.update(cx, |host, cx| {
            host.generated
                .assign("text", Value::Text("Assigned".into()), cx)
        });
        cx.run_until_parked();
        assert_eq!(
            editor.read_with(cx, |field, _| field.value().clone()),
            "Assigned"
        );
        assert_eq!(
            mirror.read_with(cx, |field, _| field.value().clone()),
            "Assigned"
        );
        host.update(cx, |host, _| host.generated = SceneState::default());
        editor.update(cx, |field, cx| field.set_value("Old scene", cx));
        cx.run_until_parked();
        assert!(host.read_with(cx, |host, _| host.generated.values.is_empty()));
    }
}
