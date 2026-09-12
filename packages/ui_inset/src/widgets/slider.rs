use super::ValueChanged;
use crate::{ControlSize, Theme, Tint};
use inset_foundation::App;
use inset_painting::{AlignmentGeometry, BorderRadiusGeometry, BoxDecoration};
use inset_rendering::HitTestBehavior;
use inset_services::LogicalKeyboardKey;
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct Slider {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub tint: Tint,
    pub size: ControlSize,
    pub on_changed: Option<ValueChanged<f64>>,
}
impl std::fmt::Debug for Slider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slider")
            .field("value", &self.value)
            .finish()
    }
}
impl Slider {
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        assert!(min.is_finite() && max.is_finite() && max > min && value.is_finite());
        Self {
            value,
            min,
            max,
            step: (max - min) / 100.0,
            tint: Tint::Primary,
            size: ControlSize::Small,
            on_changed: None,
        }
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
    pub fn normalize(&self, value: f64) -> f64 {
        (self.min + ((value - self.min) / self.step).round() * self.step).clamp(self.min, self.max)
    }
}
impl StatelessWidget for Slider {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.clone();
        LayoutBuilder::new(move |app, context, constraints| {
            let v = Theme::of(app, context);
            let width = if constraints.max_width.is_finite() {
                constraints.max_width
            } else {
                v.spacing_20 * 2.0
            };
            let height = v.metrics(w.size).size;
            let thumb = v.spacing_3;
            let fraction = ((w.value - w.min) / (w.max - w.min)).clamp(0.0, 1.0);
            let mut fill = super::Progress::new(Some(fraction));
            fill.tint = w.tint;
            let track = Center::new().child(fill).into_widget();
            let mark = Align::new()
                .alignment(AlignmentGeometry::xy(fraction * 2.0 - 1.0, 0.0))
                .child(
                    Container::new().width(thumb).height(thumb).decoration(
                        BoxDecoration::new()
                            .color(v.color_surface_raised)
                            .border_radius(BorderRadiusGeometry::circular(v.radius_full)),
                    ),
                )
                .into_widget();
            let mut gesture = GestureDetector::new()
                .behavior(HitTestBehavior::Opaque)
                .child(
                    Container::new()
                        .width(width)
                        .height(height)
                        .child(Stack::new().children([track, mark])),
                );
            if let Some(callback) = w.on_changed.clone() {
                let spec = w.clone();
                let change = Rc::new(move |app: &mut App, x: f64| {
                    callback(
                        app,
                        spec.normalize(
                            spec.min
                                + (spec.max - spec.min)
                                    * ((x - thumb / 2.0) / (width - thumb)).clamp(0.0, 1.0),
                        ),
                    )
                });
                let drag = change.clone();
                gesture = gesture
                    .on_tap_down(Rc::new(move |app, d| change(app, d.local_position.dx())))
                    .on_horizontal_drag_update(Rc::new(move |app, d| {
                        drag(app, d.local_position.dx())
                    }));
            }
            let spec = w.clone();
            Focus::new(gesture)
                .can_request_focus(w.on_changed.is_some())
                .on_key_event(Rc::new(move |app, _, event| {
                    if matches!(event, inset_services::KeyEvent::Up(_)) {
                        return KeyEventResult::Ignored;
                    }
                    let key = event.logical_key();
                    let value = if key == LogicalKeyboardKey::ARROW_RIGHT
                        || key == LogicalKeyboardKey::ARROW_UP
                    {
                        Some(spec.value + spec.step)
                    } else if key == LogicalKeyboardKey::ARROW_LEFT
                        || key == LogicalKeyboardKey::ARROW_DOWN
                    {
                        Some(spec.value - spec.step)
                    } else if key == LogicalKeyboardKey::HOME {
                        Some(spec.min)
                    } else if key == LogicalKeyboardKey::END {
                        Some(spec.max)
                    } else {
                        None
                    };
                    if let (Some(value), Some(callback)) = (value, &spec.on_changed) {
                        callback(app, spec.normalize(value));
                        KeyEventResult::Handled
                    } else {
                        KeyEventResult::Ignored
                    }
                }))
                .into_widget()
        })
        .into_widget()
    }
}
