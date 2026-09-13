use super::{Button, TextField, ValueChanged};
use crate::{ControlSize, ControlVariant, Theme, Tint};
use inset_foundation::{App, Handle};
use inset_rendering::MainAxisSize;
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct NumberField {
    /// The value, or `None` while the field is empty.
    pub value: Option<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub on_changed: Option<ValueChanged<f64>>,
    /// Called when the reader empties the field.
    pub on_cleared: Option<super::Callback>,
    pub size: ControlSize,
    pub placeholder: String,
    /// A trailing unit inside the box — a hint, never part of the value.
    pub unit: Option<String>,
    pub invalid: bool,
    pub tint: Tint,
    pub enabled: bool,
}
impl std::fmt::Debug for NumberField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NumberField")
            .field("value", &self.value)
            .finish()
    }
}
impl NumberField {
    pub fn new(value: impl Into<Option<f64>>) -> Self {
        let value = value.into();
        assert!(value.is_none_or(f64::is_finite));
        Self {
            value,
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            step: 1.0,
            on_changed: None,
            on_cleared: None,
            size: ControlSize::Small,
            placeholder: String::new(),
            unit: None,
            invalid: false,
            tint: Tint::Primary,
            enabled: true,
        }
    }
    fn text(value: Option<f64>) -> String {
        value.map(|value| value.to_string()).unwrap_or_default()
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
impl NumberFieldState {
    fn text(value: Option<f64>) -> String {
        NumberField::text(value)
    }
}
impl State for NumberFieldState {
    type Widget = NumberField;
    inset_widgets::state_accessors!();
    fn init_state(self: Handle<Self>, app: &mut App) {
        let value = Self::text(self.widget(app).value);
        let controller = TextEditingController::new(app).text(app, value);
        app.get_mut(self).controller = Some(controller);
    }
    fn did_update_widget(self: Handle<Self>, app: &mut App, old: &NumberField) {
        if old.value != self.widget(app).value {
            let value = Self::text(self.widget(app).value);
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
        let live = w.enabled && w.on_changed.is_some();
        let mut field = TextField::new()
            .controller(c)
            .size(w.size)
            .placeholder(&w.placeholder)
            .invalid(w.invalid)
            .tint(w.tint)
            .disabled(!live);
        if let Some(unit) = &w.unit {
            field.suffix = Some(
                Padding::new(inset_painting::EdgeInsetsGeometry::only(
                    0.0,
                    0.0,
                    v.spacing_2_5,
                    0.0,
                ))
                .child(
                    Text::new(unit).style(v.label_small.text_style().color(v.color_content_faint)),
                )
                .into_widget(),
            );
        }
        if let Some(callback) = w.on_changed.clone().filter(|_| live) {
            let spec = w.clone();
            field = field.on_submitted(move |app, text| {
                if text.trim().is_empty()
                    && let Some(cleared) = &spec.on_cleared
                {
                    cleared(app);
                    return;
                }
                if let Ok(value) = text.trim().parse::<f64>()
                    && value.is_finite()
                {
                    callback(app, value.clamp(spec.min, spec.max));
                    return;
                }
                c.set_text(app, NumberField::text(spec.value));
            });
        }
        let current = w
            .value
            .unwrap_or(if w.min.is_finite() { w.min } else { 0.0 });
        let mut down = Button::new("−").size(w.size).variant(ControlVariant::Plain);
        let mut up = Button::new("+").size(w.size).variant(ControlVariant::Plain);
        if let Some(callback) = w.on_changed.filter(|_| live) {
            if current > w.min {
                let callback = callback.clone();
                down = down
                    .on_pressed(move |app| callback(app, (current - w.step).clamp(w.min, w.max)));
            }
            if current < w.max {
                up =
                    up.on_pressed(move |app| callback(app, (current + w.step).clamp(w.min, w.max)));
            }
        }
        Row::new()
            .main_axis_size(MainAxisSize::Min)
            .spacing(v.spacing_1)
            .children([
                down.into_widget(),
                SizedBox::new()
                    .width(v.spacing_20 + v.spacing_8)
                    .child(field)
                    .into_widget(),
                up.into_widget(),
            ])
            .into_widget()
    }
}
