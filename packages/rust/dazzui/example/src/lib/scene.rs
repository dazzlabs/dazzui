//! Draws a compiled scene: its slots, held in an entity, and its tree, built
//! node by node through the mapping.
//!
//! A scene is the finished tree the React page renders, with what stays live
//! kept as references — `{"$state": "open"}` reads a slot, `{"$do": [...]}`
//! writes one — so this is the whole of the interpreter: resolve a prop,
//! run an action, and hand each `$el` to its component's builder.

use std::cell::RefCell;
use std::rc::Rc;

use dazzui::{Icon, Tint};
use gpui::{
    AnyElement, App, AppContext, Context, ElementId, Entity, IntoElement, ParentElement, Render,
    SharedString, Styled, WeakEntity, Window, div,
};
use serde_json::{Map, Value};

use crate::lib::mapping;
use crate::lib::specimen;
use crate::lib::story::Scene;

/// A scene's slots — the values its `useState`s hold — and the tree drawn
/// from them.
pub struct SceneView {
    scene: &'static Scene,
    slots: Map<String, Value>,
}

impl SceneView {
    pub fn new(scene: &'static Scene, cx: &mut App) -> Entity<SceneView> {
        cx.new(|_| SceneView::of(scene))
    }

    /// The view itself, for a window that builds its root.
    pub fn of(scene: &'static Scene) -> SceneView {
        SceneView {
            scene,
            slots: scene.state.clone(),
        }
    }
}

impl Render for SceneView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let context = Ctx {
            view: cx.entity().downgrade(),
            slots: Rc::new(self.slots.clone()),
            press: None,
            toast: None,
        };
        let key = self.scene.key.clone();
        // The root stands in for the frame the React page draws a scene in —
        // a flex row centring what it holds — so a row a story sizes at
        // `100%` has a width to resolve against, as it does on the web.
        div()
            .flex()
            .w_full()
            .items_center()
            .justify_center()
            .child(draw(&self.scene.content, &context, &key, window, cx))
    }
}

/// An event with no argument.
pub type Callback = Rc<dyn Fn(&mut Window, &mut App)>;
/// An event that passes what changed.
pub type Handler = Rc<dyn Fn(Value, &mut Window, &mut App)>;

/// A toast a story raises, as `useToast().add` was called.
#[derive(Clone)]
pub struct ToastRequest {
    pub title: String,
    pub tint: Tint,
    pub action: Option<(String, Callback)>,
}

/// What a node reads beside its own props: the scene's slots, and what an
/// ancestor handed down — the press a trigger is given, the toaster a
/// provider holds.
#[derive(Clone)]
pub struct Ctx {
    view: WeakEntity<SceneView>,
    slots: Rc<Map<String, Value>>,
    pub press: Option<Callback>,
    pub toast: Option<Rc<dyn Fn(ToastRequest, &mut Window, &mut App)>>,
}

impl Ctx {
    /// The same context with a press handed down — a dialog's trigger never
    /// declares its own.
    pub fn with_press(&self, press: Callback) -> Ctx {
        Ctx {
            press: Some(press),
            ..self.clone()
        }
    }

    pub fn with_toast(&self, toast: Rc<dyn Fn(ToastRequest, &mut Window, &mut App)>) -> Ctx {
        Ctx {
            toast: Some(toast),
            ..self.clone()
        }
    }

    /// A prop's value now: a slot read, a test worked out, an argument taken.
    pub fn resolve(&self, value: &Value, args: &[Value]) -> Value {
        let Some(object) = value.as_object().filter(|_| is_live(value)) else {
            return value.clone();
        };
        if let Some(slot) = object.get("$state").and_then(Value::as_str) {
            return self.slots.get(slot).cloned().unwrap_or(Value::Null);
        }
        if let Some(index) = object.get("$arg").and_then(Value::as_u64) {
            return args.get(index as usize).cloned().unwrap_or(Value::Null);
        }
        if let Some([a, b]) = object
            .get("$eq")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
        {
            return Value::Bool(self.resolve(a, args) == self.resolve(b, args));
        }
        if let Some(test) = object.get("$not") {
            return Value::Bool(!truthy(&self.resolve(test, args)));
        }
        if let Some([test, then, otherwise]) = object
            .get("$if")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
        {
            return if truthy(&self.resolve(test, args)) {
                self.resolve(then, args)
            } else {
                self.resolve(otherwise, args)
            };
        }
        let parts = object.get("$concat").and_then(Value::as_array);
        Value::String(
            parts
                .into_iter()
                .flatten()
                .map(|part| text(&self.resolve(part, args)))
                .collect(),
        )
    }

    /// Makes the writes an event declares: slots set, toasts raised.
    pub fn run(&self, action: &Value, args: &[Value], window: &mut Window, cx: &mut App) {
        for step in action["$do"].as_array().into_iter().flatten() {
            if let Some(slot) = step["set"].as_str() {
                let value = self.resolve(&step["to"], args);
                let slot = slot.to_string();
                let _ = self.view.update(cx, |view, cx| {
                    view.slots.insert(slot, value);
                    cx.notify();
                });
            } else if step.get("toast").is_some() {
                let toast = &step["toast"];
                let Some(raise) = self.toast.clone() else {
                    continue;
                };
                let action = toast.get("action").map(|action| {
                    let context = self.clone();
                    let on_click = action["onClick"].clone();
                    let run: Callback =
                        Rc::new(move |window, cx| context.run(&on_click, &[], window, cx));
                    (text(&action["label"]), run)
                });
                raise(
                    ToastRequest {
                        title: text(&self.resolve(&toast["title"], args)),
                        tint: self
                            .resolve(&toast["tint"], args)
                            .as_str()
                            .and_then(Tint::parse)
                            .unwrap_or(Tint::Neutral),
                        action,
                    },
                    window,
                    cx,
                );
            }
        }
    }
}

/// Whether a value is only known while the scene runs.
pub fn is_live(value: &Value) -> bool {
    value.as_object().is_some_and(|object| {
        ["$state", "$eq", "$not", "$if", "$concat", "$arg"]
            .iter()
            .any(|key| object.contains_key(*key))
    })
}

/// JavaScript's truthiness, which is what a story's conditions were
/// written against.
pub fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(flag) => *flag,
        Value::Number(number) => number.as_f64().is_some_and(|n| n != 0.0),
        Value::String(text) => !text.is_empty(),
        _ => true,
    }
}

/// A value as React would print it into a text node.
pub fn text(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(text) => text.clone(),
        Value::Number(number) => match number.as_f64() {
            Some(n) if n.fract() == 0.0 && n.abs() < 1e15 => format!("{}", n as i64),
            _ => number.to_string(),
        },
        Value::Bool(_) => String::new(),
        Value::Array(items) => items.iter().map(text).collect(),
        Value::Object(_) => String::new(),
    }
}

/// The asset path an `$icon` names, served by the example's asset source.
pub fn glyph_path(value: &Value) -> SharedString {
    let name = value["$icon"].as_str().expect("an $icon node");
    format!("glyph/{name}").into()
}

/// One node of a scene, drawn: text, a glyph, or a component.
pub fn draw(value: &Value, ctx: &Ctx, path: &str, window: &mut Window, cx: &mut App) -> AnyElement {
    match value {
        Value::String(text) => SharedString::from(text.clone()).into_any_element(),
        Value::Number(_) => SharedString::from(text(value)).into_any_element(),
        Value::Object(object) if object.contains_key("$icon") => {
            specimen::wrap(object.get("$at"), Icon::new(glyph_path(value)), path)
        }
        Value::Object(object) if object.contains_key("$el") => {
            let node = Node {
                json: value,
                ctx: ctx.clone(),
                path: path.to_string(),
            };
            let element = mapping::build(&node, window, cx);
            specimen::wrap(object.get("$at"), element, path)
        }
        _ if is_live(value) => {
            SharedString::from(text(&ctx.resolve(value, &[]))).into_any_element()
        }
        _ => div().into_any_element(),
    }
}

/// A component in the tree, read the way its builder needs it.
pub struct Node<'a> {
    pub json: &'a Value,
    pub ctx: Ctx,
    /// Where the node sits in its scene — the key its state is kept under.
    pub path: String,
}

impl<'a> Node<'a> {
    pub fn component(&self) -> &str {
        self.json["$el"].as_str().unwrap_or_default()
    }

    pub fn props(&self) -> Option<&'a Map<String, Value>> {
        self.json["props"].as_object()
    }

    pub fn children(&self) -> &'a [Value] {
        self.json["children"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// The same node read in another context.
    pub fn in_ctx(&self, ctx: Ctx) -> Node<'a> {
        Node {
            json: self.json,
            ctx,
            path: self.path.clone(),
        }
    }

    /// An element id unique to this node in the scene.
    pub fn id(&self) -> ElementId {
        ElementId::Name(SharedString::from(self.path.clone()))
    }

    /// An element id for one of this node's parts.
    pub fn id_for(&self, part: impl std::fmt::Display) -> ElementId {
        ElementId::Name(SharedString::from(format!("{}#{part}", self.path)))
    }

    pub fn has(&self, name: &str) -> bool {
        self.props().is_some_and(|props| props.contains_key(name))
    }

    /// A prop as written, before it is resolved.
    pub fn raw(&self, name: &str) -> Option<&'a Value> {
        self.props()?.get(name)
    }

    /// A prop's current value.
    pub fn get(&self, name: &str) -> Value {
        self.raw(name)
            .map(|value| self.ctx.resolve(value, &[]))
            .unwrap_or(Value::Null)
    }

    pub fn string(&self, name: &str) -> Option<String> {
        match self.get(name) {
            Value::Null => None,
            value => Some(text(&value)),
        }
    }

    pub fn flag(&self, name: &str) -> bool {
        truthy(&self.get(name))
    }

    pub fn number(&self, name: &str) -> Option<f64> {
        self.get(name).as_f64()
    }

    /// A string prop read as one of a kit enum's cases.
    pub fn option<T>(&self, name: &str, parse: impl Fn(&str) -> Option<T>) -> Option<T> {
        self.string(name).and_then(|value| parse(&value))
    }

    /// A prop that holds a component, drawn.
    pub fn element(&self, name: &str, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let value = self.raw(name)?;
        Some(draw(
            value,
            &self.ctx,
            &format!("{}.{name}", self.path),
            window,
            cx,
        ))
    }

    /// A value inside a prop — an item of a list, a field of an object —
    /// drawn at a path of its own.
    pub fn draw_value(
        &self,
        value: &Value,
        part: impl std::fmt::Display,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        draw(
            value,
            &self.ctx,
            &format!("{}.{part}", self.path),
            window,
            cx,
        )
    }

    /// The children, drawn.
    pub fn kids(&self, window: &mut Window, cx: &mut App) -> Vec<AnyElement> {
        self.kids_in(&self.ctx, window, cx)
    }

    /// The children, drawn in another context.
    pub fn kids_in(&self, ctx: &Ctx, window: &mut Window, cx: &mut App) -> Vec<AnyElement> {
        self.children()
            .iter()
            .enumerate()
            .map(|(index, child)| draw(child, ctx, &format!("{}/{index}", self.path), window, cx))
            .collect()
    }

    /// An event with no argument, or `None` when the story sets none.
    pub fn action(&self, name: &str) -> Option<Callback> {
        let value = self.raw(name)?.clone();
        let ctx = self.ctx.clone();
        Some(Rc::new(move |window, cx| ctx.run(&value, &[], window, cx)))
    }

    /// An event that passes what changed.
    pub fn handler(&self, name: &str) -> Option<Handler> {
        let value = self.raw(name)?.clone();
        let ctx = self.ctx.clone();
        Some(Rc::new(move |arg, window, cx| {
            ctx.run(&value, &[arg], window, cx)
        }))
    }

    /// What a press does: a trigger's own, the story's event, or nothing — a
    /// React button with no handler is still a button.
    pub fn press(&self, event: &str) -> Option<Callback> {
        self.ctx.press.clone().or_else(|| self.action(event))
    }
}

/// A value held for the life of the window, keyed by a node — the state a
/// mapping keeps for a prop the story leaves to the component.
pub fn held<T: 'static>(
    node: &Node,
    part: &str,
    window: &mut Window,
    cx: &mut App,
    init: impl FnOnce() -> T,
) -> Rc<RefCell<T>> {
    window
        .use_keyed_state(node.id_for(part), cx, |_, _| Rc::new(RefCell::new(init())))
        .read(cx)
        .clone()
}
