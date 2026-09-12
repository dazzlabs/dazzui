use super::{Button, TextField, ValueChanged};
use crate::{ControlSize, ControlVariant, Theme};
use inset_foundation::{App, Handle};
use inset_rendering::MainAxisSize;
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct NumberField {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub on_changed: Option<ValueChanged<f64>>,
    pub size: ControlSize,
}
impl std::fmt::Debug for NumberField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NumberField")
            .field("value", &self.value)
            .finish()
    }
}
impl NumberField {
    pub fn new(value: f64) -> Self {
        assert!(value.is_finite());
        Self {
            value,
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            step: 1.0,
            on_changed: None,
            size: ControlSize::Small,
        }
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        assert!(min <= max);
        self.min = min;
        self.max = max;
        self
    }
    pub fn step(mut self, step: f64) -> Self {
        assert!(step.is_finite() && step > 0.0);
        self.step = step;
        self
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, f64) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
}
impl StatefulWidget for NumberField {
    type State = NumberFieldState;
    fn create_state(&self) -> Self::State {
        NumberFieldState {
            state: StateData::new(),
            controller: None,
        }
    }
}
pub struct NumberFieldState {
    state: StateData<NumberField>,
    controller: Option<Handle<TextEditingController>>,
}
impl State for NumberFieldState {
    type Widget = NumberField;
    inset_widgets::state_accessors!();
    fn init_state(self: Handle<Self>, app: &mut App) {
        let value = self.widget(app).value.to_string();
        let controller = TextEditingController::new(app).text(app, value);
        app.get_mut(self).controller = Some(controller);
    }
    fn did_update_widget(self: Handle<Self>, app: &mut App, old: &NumberField) {
        if old.value != self.widget(app).value {
            let value = self.widget(app).value.to_string();
            app.get(self).controller.unwrap().set_text(app, value);
        }
    }
    fn dispose(self: Handle<Self>, app: &mut App) {
        let c = app.get_mut(self).controller.take().unwrap();
        app.get_mut(c).dispose();
        app.destroy(c);
    }
    fn build(self: Handle<Self>, app: &mut App, context: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let v = Theme::of(app, context);
        let c = app.get(self).controller.unwrap();
        let mut field = TextField::new()
            .controller(c)
            .size(w.size)
            .disabled(w.on_changed.is_none());
        if let Some(callback) = w.on_changed.clone() {
            field = field.on_submitted(move |app, text| {
                if let Ok(value) = text.parse::<f64>()
                    && value.is_finite()
                {
                    callback(app, value.clamp(w.min, w.max));
                    return;
                }
                c.set_text(app, w.value.to_string());
            });
        }
        let mut down = Button::new("−").size(w.size).variant(ControlVariant::Plain);
        let mut up = Button::new("+").size(w.size).variant(ControlVariant::Plain);
        if let Some(callback) = w.on_changed {
            if w.value > w.min {
                let callback = callback.clone();
                down = down
                    .on_pressed(move |app| callback(app, (w.value - w.step).clamp(w.min, w.max)));
            }
            if w.value < w.max {
                up =
                    up.on_pressed(move |app| callback(app, (w.value + w.step).clamp(w.min, w.max)));
            }
        }
        Row::new()
            .main_axis_size(MainAxisSize::Min)
            .spacing(v.spacing_1)
            .children([
                down.into_widget(),
                SizedBox::new()
                    .width(v.spacing_20)
                    .child(field)
                    .into_widget(),
                up.into_widget(),
            ])
            .into_widget()
    }
}
