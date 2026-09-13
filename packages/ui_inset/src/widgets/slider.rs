use super::ValueChanged;
use crate::{ControlSize, Theme, Tint};
use inset_foundation::App;
use inset_painting::{BorderRadiusGeometry, BoxDecoration};
use inset_rendering::{CrossAxisAlignment, HitTestBehavior, MainAxisSize};
use inset_services::LogicalKeyboardKey;
use inset_widgets::*;
use std::rc::Rc;
/// A value picked along a range. Set `end` for a range: a thumb per value,
/// and a drag moves whichever thumb is nearer the pointer.
#[derive(Clone)]
pub struct Slider {
    pub value: f64,
    pub end: Option<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub tint: Tint,
    pub size: ControlSize,
    pub enabled: bool,
    /// Printed at the trailing edge, for a value the reader has to say.
    pub value_label: Option<String>,
    pub on_changed: Option<ValueChanged<f64>>,
    pub on_end_changed: Option<ValueChanged<f64>>,
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
            end: None,
            min,
            max,
            step: (max - min) / 100.0,
            tint: Tint::Primary,
            size: ControlSize::Small,
            enabled: true,
            value_label: None,
            on_changed: None,
            on_end_changed: None,
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
    pub fn on_end_changed(mut self, callback: impl Fn(&mut App, f64) + 'static) -> Self {
        self.on_end_changed = Some(Rc::new(callback));
        self
    }
    pub fn normalize(&self, value: f64) -> f64 {
        (self.min + ((value - self.min) / self.step).round() * self.step).clamp(self.min, self.max)
    }
    fn fraction(&self, value: f64) -> f64 {
        ((value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }
    fn live(&self) -> bool {
        self.enabled && self.on_changed.is_some()
    }
}
impl StatelessWidget for Slider {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.clone();
        let groove = LayoutBuilder::new(move |app, context, constraints| {
            let v = Theme::of(app, context);
            let width = if constraints.max_width.is_finite() {
                constraints.max_width
            } else {
                v.spacing_20 * 2.0
            };
            let height = v.metrics(w.size).size;
            let thumb = v.spacing_3;
            let track_height = match w.size {
                ControlSize::Large => v.spacing_2,
                ControlSize::Medium => v.spacing_1_5,
                _ => v.spacing_1,
            };
            let span = width - thumb;
            let start = w.end.map_or(0.0, |_| w.fraction(w.value));
            let stop = w.fraction(w.end.unwrap_or(w.value));
            let fill = if w.live() {
                v.ramp(w.tint).shade(600)
            } else {
                v.color_content_faint
            };
            let track = Positioned {
                left: Some(thumb / 2.0),
                right: Some(thumb / 2.0),
                top: Some((height - track_height) / 2.0),
                height: Some(track_height),
                ..Positioned::new(
                    Container::new().decoration(
                        BoxDecoration::new()
                            .color(v.color_surface_sunken)
                            .border_radius(BorderRadiusGeometry::circular(track_height / 2.0)),
                    ),
                )
            }
            .into_widget();
            let range = Positioned {
                left: Some(thumb / 2.0 + span * start),
                top: Some((height - track_height) / 2.0),
                width: Some(span * (stop - start)),
                height: Some(track_height),
                ..Positioned::new(
                    Container::new().decoration(
                        BoxDecoration::new()
                            .color(fill)
                            .border_radius(BorderRadiusGeometry::circular(track_height / 2.0)),
                    ),
                )
            }
            .into_widget();
            let mark = |fraction: f64| {
                Positioned {
                    left: Some(span * fraction),
                    top: Some((height - thumb) / 2.0),
                    width: Some(thumb),
                    height: Some(thumb),
                    ..Positioned::new(
                        Container::new().decoration(
                            BoxDecoration::new()
                                .color(v.color_surface_raised)
                                .border_radius(BorderRadiusGeometry::circular(v.radius_full)),
                        ),
                    )
                }
                .into_widget()
            };
            let mut layers = vec![track, range, mark(stop)];
            if w.end.is_some() {
                layers.push(mark(start));
            }
            let mut gesture = GestureDetector::new()
                .behavior(HitTestBehavior::Opaque)
                .child(
                    SizedBox::new()
                        .width(width)
                        .height(height)
                        .child(Stack::new().children(layers)),
                );
            if w.live() {
                let spec = w.clone();
                let change = Rc::new(move |app: &mut App, x: f64| {
                    let value = spec.normalize(
                        spec.min
                            + (spec.max - spec.min) * ((x - thumb / 2.0) / span).clamp(0.0, 1.0),
                    );
                    match (spec.end, &spec.on_end_changed) {
                        // The nearer thumb takes the pointer; a tie goes to the end
                        // once the pointer is past it.
                        (Some(end), Some(callback))
                            if (value - end).abs() < (value - spec.value).abs()
                                || (value > end && end == spec.value) =>
                        {
                            callback(app, value)
                        }
                        _ => {
                            if let Some(callback) = &spec.on_changed {
                                callback(app, value)
                            }
                        }
                    }
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
                .can_request_focus(w.live())
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
                    match (value, &spec.on_changed) {
                        (Some(value), Some(callback)) if spec.live() => {
                            callback(app, spec.normalize(value));
                            KeyEventResult::Handled
                        }
                        _ => KeyEventResult::Ignored,
                    }
                }))
                .into_widget()
        })
        .into_widget();
        let Some(label) = self.value_label.clone() else {
            return groove;
        };
        // A slider fills the width it is offered; where none is, it keeps the
        // groove's own length and the reading sits after it.
        LayoutBuilder::new(move |app, context, constraints| {
            let v = Theme::of(app, context);
            let reading = Text::new(&label)
                .style(v.label_small.text_style().color(v.color_content_muted))
                .into_widget();
            let groove = if constraints.max_width.is_finite() {
                Expanded::new(groove.clone()).into_widget()
            } else {
                groove.clone()
            };
            Row::new()
                .main_axis_size(if constraints.max_width.is_finite() {
                    MainAxisSize::Max
                } else {
                    MainAxisSize::Min
                })
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .spacing(v.spacing_3)
                .children([groove, reading])
                .into_widget()
        })
        .into_widget()
    }
}
