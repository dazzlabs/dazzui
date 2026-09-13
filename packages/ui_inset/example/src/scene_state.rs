//! Per-mount state for shared scenes. Renderers contain no story-specific state logic.
use inset::*;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

pub type Values = BTreeMap<String, serde_json::Value>;
#[derive(Clone)]
pub struct SceneHandle {
    pub values: Entity<Values>,
    keys: Rc<RefCell<BTreeMap<String, Rc<GlobalKey>>>>,
    controllers: Rc<RefCell<BTreeMap<String, Handle<TextEditingController>>>>,
}
impl std::ops::Deref for SceneHandle {
    type Target = Entity<Values>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
impl SceneHandle {
    pub fn controller(&self, app: &mut App, key: &str) -> Handle<TextEditingController> {
        if let Some(controller) = self.controllers.borrow().get(key) {
            return *controller;
        }
        let value = app.read(&self.values)[key]
            .as_str()
            .expect("text state")
            .to_string();
        let controller = TextEditingController::new(app).text(app, value);
        self.controllers
            .borrow_mut()
            .insert(key.to_string(), controller);
        controller
    }
    fn dispose(&self, app: &mut App) {
        for (_, controller) in std::mem::take(&mut *self.controllers.borrow_mut()) {
            app.get_mut(controller).dispose();
            app.destroy(controller);
        }
    }

    pub fn key(&self, id: &str) -> Rc<GlobalKey> {
        self.keys
            .borrow_mut()
            .entry(id.to_string())
            .or_insert_with(|| Rc::new(GlobalKey::new()))
            .clone()
    }
}
pub type Observer = Rc<dyn Fn(&mut App, SceneHandle)>;
type Renderer = Rc<dyn Fn(&mut App, BuildContext, &SceneHandle) -> WidgetRef>;
#[derive(Clone)]
pub struct Scene {
    id: String,
    key: KeyRef,
    initial: Values,
    observer: Option<Observer>,
    render: Renderer,
}
impl std::fmt::Debug for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Scene").field(&self.id).finish()
    }
}
impl Scene {
    pub fn new(
        id: String,
        initial: Values,
        observer: Option<Observer>,
        render: impl Fn(&mut App, BuildContext, &SceneHandle) -> WidgetRef + 'static,
    ) -> Self {
        Self {
            key: Rc::new(ValueKey::new(id.clone())),
            id,
            initial,
            observer,
            render: Rc::new(render),
        }
    }
}
impl StatefulWidget for Scene {
    type State = SceneState;
    fn key(&self) -> Option<&KeyRef> {
        Some(&self.key)
    }
    fn create_state(&self) -> SceneState {
        SceneState {
            state: StateData::new(),
            values: None,
        }
    }
}
pub struct SceneState {
    state: StateData<Scene>,
    values: Option<SceneHandle>,
}
impl State for SceneState {
    type Widget = Scene;
    fn state_data(self: Handle<Self>, app: &App) -> &StateData<Scene> {
        &app.get(self).state
    }
    fn state_data_mut(self: Handle<Self>, app: &mut App) -> &mut StateData<Scene> {
        &mut app.get_mut(self).state
    }
    fn init_state(self: Handle<Self>, app: &mut App) {
        let initial = self.widget(app).initial.clone();
        let values = SceneHandle {
            values: app.new_entity(|_| initial),
            keys: Default::default(),
            controllers: Default::default(),
        };
        app.get_mut(self).values = Some(values.clone());
        if let Some(observer) = self.widget(app).observer.clone() {
            observer(app, values);
        }
    }
    fn dispose(self: Handle<Self>, app: &mut App) {
        if let Some(values) = app.get_mut(self).values.take() {
            values.dispose(app);
        }
    }
    fn build(self: Handle<Self>, app: &mut App, context: BuildContext) -> WidgetRef {
        let values = app.get(self).values.as_ref().unwrap().clone();
        let render = self.widget(app).render.clone();
        render(app, context, &values)
    }
}
pub fn set(app: &mut App, state: &SceneHandle, key: &str, value: impl Into<serde_json::Value>) {
    let value = value.into();
    if let Some(controller) = state.controllers.borrow().get(key).copied() {
        let text = value.as_str().expect("text state");
        if controller.text_value(app) != text {
            controller.set_text(app, text);
        }
    }
    app.update(&state.values, |values, cx| {
        *values.get_mut(key).expect("declared state") = value;
        cx.notify();
    });
}
