use crate::{ColorExt, ControlSize, Theme, Tint};
use inset_animation::{Animation, AnimationBehavior, AnimationController};
use inset_embedder::{Canvas, Color, Offset, Paint, PaintStyle, PathBuilder, Size, Stroke};
use inset_foundation::{App, Handle};
use inset_painting::{AlignmentGeometry, BorderRadiusGeometry, BoxDecoration};
use inset_rendering::{CustomPainter, MainAxisSize};
use inset_scheduler::{Ticker, TickerCallback, TickerProviderObject};
use inset_widgets::*;
use std::{rc::Rc, time::Duration};

type MotionBuilder = Rc<dyn Fn(&mut App, BuildContext, f64) -> WidgetRef>;
#[derive(Clone)]
struct Motion {
    builder: MotionBuilder,
    duration: Duration,
}
impl std::fmt::Debug for Motion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Motion")
    }
}
impl StatefulWidget for Motion {
    type State = MotionState;
    fn create_state(&self) -> Self::State {
        MotionState {
            state: StateData::new(),
            ticker: Default::default(),
            controller: None,
        }
    }
}
struct MotionState {
    state: StateData<Motion>,
    ticker: SingleTickerProviderStateMixinData,
    controller: Option<Handle<AnimationController>>,
}
impl SingleTickerProviderStateMixin for MotionState {
    fn single_ticker_provider_data(
        self: Handle<Self>,
        app: &App,
    ) -> &SingleTickerProviderStateMixinData {
        &app.get(self).ticker
    }
    fn single_ticker_provider_data_mut(
        self: Handle<Self>,
        app: &mut App,
    ) -> &mut SingleTickerProviderStateMixinData {
        &mut app.get_mut(self).ticker
    }
}
impl TickerProviderObject for MotionState {
    fn create_ticker(
        self: Handle<Self>,
        app: &mut App,
        callback: TickerCallback,
    ) -> Handle<Ticker> {
        SingleTickerProviderStateMixin::create_ticker(self, app, callback)
    }
}
impl State for MotionState {
    type Widget = Motion;
    inset_widgets::state_accessors!();
    fn init_state(self: Handle<Self>, app: &mut App) {
        let duration = self.widget(app).duration;
        let c = AnimationController::create(
            app,
            None,
            Some(duration),
            None,
            0.0,
            1.0,
            AnimationBehavior::Normal,
            self,
        );
        app.get_mut(self).controller = Some(c);
        c.repeat(app, None, None, false, None, None);
    }
    fn activate(self: Handle<Self>, app: &mut App) {
        SingleTickerProviderStateMixin::activate(self, app);
    }
    fn dispose(self: Handle<Self>, app: &mut App) {
        let c = app.get_mut(self).controller.take().unwrap();
        c.dispose(app);
        SingleTickerProviderStateMixin::dispose(self, app);
        app.destroy(c);
    }
    fn build(self: Handle<Self>, app: &mut App, _: BuildContext) -> WidgetRef {
        let c = app.get(self).controller.unwrap().as_animation();
        let builder = self.widget(app).builder.clone();
        AnimatedBuilder::new(Rc::new(c), move |app, context, _| {
            let value = c.value(app);
            builder(app, context, value)
        })
        .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Spinner {
    pub size: ControlSize,
    pub tint: Tint,
    pub on_accent: bool,
}
impl Default for Spinner {
    fn default() -> Self {
        Self {
            size: ControlSize::Small,
            tint: Tint::Primary,
            on_accent: false,
        }
    }
}
impl Spinner {
    pub fn new() -> Self {
        Self::default()
    }
}
impl StatelessWidget for Spinner {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.clone();
        Motion {
            duration: Duration::from_millis(800),
            builder: Rc::new(move |app, context, value| {
                let v = Theme::of(app, context);
                let size = match w.size {
                    ControlSize::Large => v.spacing_5,
                    ControlSize::Medium => v.spacing_4,
                    _ => v.spacing_3_5,
                };
                let color = if w.on_accent {
                    v.color_on_accent
                } else {
                    v.ramp(w.tint).shade(600)
                };
                SizedBox::square(Some(size))
                    .child(CustomPaint::new().painter(Ring {
                        color,
                        width: v.spacing_0_5,
                        phase: value,
                    }))
                    .into_widget()
            }),
        }
        .into_widget()
    }
}
struct Ring {
    color: Color,
    width: f64,
    phase: f64,
}
impl CustomPainter for Ring {
    fn paint(&self, _: &mut App, canvas: &mut Canvas, size: Size) {
        let r = (size.width().min(size.height()) - self.width) / 2.0;
        for (sweep, color) in [(1.0, self.color.opacity(0.25)), (0.75, self.color)] {
            let mut path = PathBuilder::new();
            // Polygonal approximation stays below a device pixel at the largest spinner.
            for i in 0..=64 {
                let angle = std::f64::consts::TAU * (self.phase + sweep * i as f64 / 64.0);
                let p = Offset::new(
                    size.width() / 2.0 + r * angle.cos(),
                    size.height() / 2.0 + r * angle.sin(),
                );
                if i == 0 {
                    path.move_to(p);
                } else {
                    path.line_to(p);
                }
            }
            canvas.draw_path(
                &path.build(),
                inset_embedder::FillRule::NonZero,
                &Paint {
                    color: color.into(),
                    style: PaintStyle::Stroke(Stroke::new(self.width as f32)),
                    ..Default::default()
                },
            );
        }
    }
    fn should_repaint(&self, _: &App, _: &dyn CustomPainter) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[derive(Debug, Clone)]
pub struct Progress {
    pub value: Option<f64>,
    pub size: ControlSize,
    pub tint: Tint,
}
impl Progress {
    pub fn new(value: Option<f64>) -> Self {
        assert!(value.is_none_or(|v| v.is_finite() && (0.0..=1.0).contains(&v)));
        Self {
            value,
            size: ControlSize::Small,
            tint: Tint::Primary,
        }
    }
}
impl StatelessWidget for Progress {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let height = match self.size {
            ControlSize::Large => v.spacing_2,
            ControlSize::Medium => v.spacing_1_5,
            _ => v.spacing_1,
        };
        let color = v.ramp(self.tint).shade(600);
        let child = if let Some(value) = self.value {
            Align::new()
                .alignment(AlignmentGeometry::CENTER_LEFT)
                .child(
                    FractionallySizedBox::new()
                        .width_factor(value)
                        .child(Container::new().color(color)),
                )
                .into_widget()
        } else {
            Motion {
                duration: Duration::from_millis(1200),
                builder: Rc::new(move |_, _, value| {
                    Align::new()
                        .alignment(AlignmentGeometry::xy(value * 2.0 - 1.0, 0.0))
                        .child(
                            FractionallySizedBox::new()
                                .width_factor(0.4)
                                .child(Container::new().color(color)),
                        )
                        .into_widget()
                }),
            }
            .into_widget()
        };
        ClipRRect::new()
            .border_radius(BorderRadiusGeometry::circular(height / 2.0))
            .child(
                Container::new()
                    .height(height)
                    .color(v.color_surface_sunken)
                    .child(child),
            )
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Meter {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub tint: Tint,
    pub label: Option<String>,
}
impl Meter {
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        assert!(value.is_finite() && min.is_finite() && max.is_finite() && max > min);
        Self {
            value,
            min,
            max,
            tint: Tint::Primary,
            label: None,
        }
    }
    pub fn fraction(&self) -> f64 {
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }
}
impl StatelessWidget for Meter {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut progress = Progress::new(Some(self.fraction()));
        progress.tint = self.tint;
        let mut children = vec![];
        if let Some(label) = &self.label {
            children.push(
                Text::new(label)
                    .style(v.label_small.text_style())
                    .into_widget(),
            );
        }
        children.push(progress.into_widget());
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .spacing(v.spacing_2)
            .children(children)
            .into_widget()
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub enum SkeletonShape {
    #[default]
    Text,
    Circle,
    Rect,
}
#[derive(Debug, Clone, Default)]
pub struct Skeleton {
    pub shape: SkeletonShape,
    pub width: Option<f64>,
    pub height: Option<f64>,
}
impl Skeleton {
    pub fn new() -> Self {
        Self::default()
    }
}
impl StatelessWidget for Skeleton {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.clone();
        Motion {
            duration: Duration::from_millis(1500),
            builder: Rc::new(move |app, context, t| {
                let v = Theme::of(app, context);
                let radius = match w.shape {
                    SkeletonShape::Circle => v.radius_full,
                    SkeletonShape::Rect => v.radius_small,
                    _ => v.radius_tiny,
                };
                let mut c = Container::new()
                    .height(w.height.unwrap_or(v.body_small.line_height))
                    .decoration(
                        BoxDecoration::new()
                            .color(v.color_surface_sunken)
                            .border_radius(BorderRadiusGeometry::circular(radius)),
                    );
                if let Some(width) = w.width {
                    c = c.width(width);
                }
                Opacity::new(0.65 + 0.35 * (t * std::f64::consts::TAU).cos().abs())
                    .child(c)
                    .into_widget()
            }),
        }
        .into_widget()
    }
}
