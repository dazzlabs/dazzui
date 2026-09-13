//! Controlled selection widgets. Values belong to the host, as in Flutter.
use super::Pressable;
use crate::{ControlSize, ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_painting::{
    AlignmentGeometry, Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_rendering::MainAxisSize;
use inset_widgets::*;
use std::rc::Rc;
pub type ValueChanged<T> = Rc<dyn Fn(&mut App, T)>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectionKind {
    Checkbox,
    Radio,
    Switch,
}
#[derive(Clone)]
struct Selection {
    kind: SelectionKind,
    value: Option<bool>,
    label: String,
    tint: Tint,
    size: ControlSize,
    on_changed: Option<ValueChanged<Option<bool>>>,
    tristate: bool,
}
impl std::fmt::Debug for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Selection")
            .field("kind", &self.kind)
            .finish()
    }
}
impl StatelessWidget for Selection {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let radius = if self.kind == SelectionKind::Checkbox {
            v.checkbox_radius
        } else {
            v.radius_full
        };
        let widget = self.clone();
        let mut pressable = Pressable::new(move |app, context, phase| {
            let v = Theme::of(app, context);
            let checked = widget.value == Some(true);
            let marked = widget.value != Some(false);
            let c = v.control_colors(ControlVariant::Filled, widget.tint, ControlState::Normal);
            let box_size = match (widget.kind, widget.size) {
                (SelectionKind::Checkbox, ControlSize::Large) => v.checkbox_large_box,
                (SelectionKind::Checkbox, ControlSize::Medium) => v.checkbox_medium_box,
                (SelectionKind::Checkbox, _) => v.checkbox_small_box,
                (SelectionKind::Radio, ControlSize::Large) => v.radio_large_box,
                (SelectionKind::Radio, ControlSize::Medium) => v.radio_medium_box,
                (SelectionKind::Radio, _) => v.radio_small_box,
                (SelectionKind::Switch, ControlSize::Large) => v.switch_large_height,
                (SelectionKind::Switch, ControlSize::Medium) => v.switch_medium_height,
                (SelectionKind::Switch, _) => v.switch_small_height,
            };
            let mut control = Container::new().width(box_size).height(box_size);
            let mut decoration =
                BoxDecoration::new().border_radius(BorderRadiusGeometry::circular(radius));
            match widget.kind {
                SelectionKind::Checkbox => {
                    if marked {
                        decoration = decoration.color(c.surface);
                        control = control.child(Center::new().child(
                            SizedBox::square(Some(v.label_small.size)).child(
                                CustomPaint::new().painter(CheckMark {
                                    color: v.color_on_accent,
                                    dash: !checked,
                                }),
                            ),
                        ));
                    } else {
                        decoration = decoration.border(Border::all(
                            v.color_border_muted,
                            v.checkbox_thickness,
                            BorderStyle::Solid,
                            -1.0,
                        ));
                    }
                }
                SelectionKind::Radio => {
                    decoration = decoration.border(Border::all(
                        if checked {
                            c.surface
                        } else {
                            v.color_border_muted
                        },
                        v.radio_thickness,
                        BorderStyle::Solid,
                        -1.0,
                    ));
                    if checked {
                        let dot = match widget.size {
                            ControlSize::Large => v.radio_large_dot,
                            ControlSize::Medium => v.radio_medium_dot,
                            _ => v.radio_small_dot,
                        };
                        control =
                            control.child(Center::new().child(
                                Container::new().width(dot).height(dot).decoration(
                                    BoxDecoration::new().color(c.surface).border_radius(
                                        BorderRadiusGeometry::circular(v.radius_full),
                                    ),
                                ),
                            ));
                    }
                }
                SelectionKind::Switch => {
                    let (width, thumb) = match widget.size {
                        ControlSize::Large => (v.switch_large_width, v.switch_large_thumb),
                        ControlSize::Medium => (v.switch_medium_width, v.switch_medium_thumb),
                        _ => (v.switch_small_width, v.switch_small_thumb),
                    };
                    decoration = decoration.color(if checked {
                        c.surface
                    } else {
                        v.color_surface_sunken
                    });
                    control = control
                        .width(width)
                        .padding(EdgeInsetsGeometry::all((box_size - thumb) / 2.0))
                        .child(
                            Align::new()
                                .alignment(if checked {
                                    AlignmentGeometry::CENTER_RIGHT
                                } else {
                                    AlignmentGeometry::CENTER_LEFT
                                })
                                .child(
                                    Container::new().width(thumb).height(thumb).decoration(
                                        BoxDecoration::new()
                                            .color(if checked {
                                                c.content
                                            } else {
                                                v.color_content_faint
                                            })
                                            .border_radius(BorderRadiusGeometry::circular(
                                                v.radius_full,
                                            )),
                                    ),
                                ),
                        );
                }
            }
            let mut face = v.label_quiet;
            if checked {
                face.weight = v.label_medium.weight;
            }
            let mut children = vec![control.decoration(decoration).into_widget()];
            if !widget.label.is_empty() {
                children.push(SizedBox::new().width(v.spacing_2_5).into_widget());
                children.push(
                    Text::new(&widget.label)
                        .style(face.text_style().color(if checked {
                            v.color_content
                        } else {
                            v.color_content_muted
                        }))
                        .into_widget(),
                );
            }
            Opacity::new(if phase == ControlState::Disabled {
                0.6
            } else {
                1.0
            })
            .child(
                Row::new()
                    .main_axis_size(MainAxisSize::Min)
                    .children(children),
            )
            .into_widget()
        });
        pressable.radius = radius;
        if let Some(callback) = self.on_changed.clone() {
            let next = if self.kind == SelectionKind::Radio {
                Some(true)
            } else if self.tristate {
                match self.value {
                    Some(false) => Some(true),
                    Some(true) => None,
                    None => Some(false),
                }
            } else {
                Some(self.value != Some(true))
            };
            pressable.on_pressed = Some(Rc::new(move |app| callback(app, next)));
        }
        pressable.into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Checkbox(Selection);
impl Checkbox {
    pub fn new(value: bool) -> Self {
        Self(Selection {
            kind: SelectionKind::Checkbox,
            value: Some(value),
            label: String::new(),
            tint: Tint::Primary,
            size: ControlSize::Small,
            on_changed: None,
            tristate: false,
        })
    }
    pub fn tristate(value: Option<bool>) -> Self {
        let mut w = Self::new(false);
        w.0.value = value;
        w.0.tristate = true;
        w
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.0.label = label.into();
        self
    }
    pub fn tint(mut self, tint: Tint) -> Self {
        self.0.tint = tint;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.0.size = size;
        self
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, Option<bool>) + 'static) -> Self {
        self.0.on_changed = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for Checkbox {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        self.0.clone().into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Radio(Selection);
impl Radio {
    pub fn new(selected: bool) -> Self {
        let mut w = Checkbox::new(selected).0;
        w.kind = SelectionKind::Radio;
        Self(w)
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.0.label = label.into();
        self
    }
    pub fn tint(mut self, tint: Tint) -> Self {
        self.0.tint = tint;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.0.size = size;
        self
    }
    pub fn on_selected(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.0.on_changed = Some(Rc::new(move |app, _| callback(app)));
        self
    }
}
impl StatelessWidget for Radio {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        self.0.clone().into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Switch(Selection);
impl Switch {
    pub fn new(value: bool) -> Self {
        let mut w = Checkbox::new(value).0;
        w.kind = SelectionKind::Switch;
        Self(w)
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.0.label = label.into();
        self
    }
    pub fn tint(mut self, tint: Tint) -> Self {
        self.0.tint = tint;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.0.size = size;
        self
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, bool) + 'static) -> Self {
        self.0.on_changed = Some(Rc::new(move |app, value| {
            callback(app, value == Some(true))
        }));
        self
    }
}
impl StatelessWidget for Switch {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        self.0.clone().into_widget()
    }
}

struct CheckMark {
    color: inset_embedder::Color,
    dash: bool,
}
impl inset_rendering::CustomPainter for CheckMark {
    fn paint(&self, _: &mut App, canvas: &mut inset_embedder::Canvas, size: inset_embedder::Size) {
        use inset_embedder::{FillRule, Offset, Paint, PaintStyle, PathBuilder, Stroke};
        let w = size.width();
        let mut path = PathBuilder::new();
        if self.dash {
            path.move_to(Offset::new(w * 0.24, w * 0.5));
            path.line_to(Offset::new(w * 0.76, w * 0.5));
        } else {
            path.move_to(Offset::new(w * 0.22, w * 0.52));
            path.line_to(Offset::new(w * 0.42, w * 0.72));
            path.line_to(Offset::new(w * 0.78, w * 0.28));
        }
        let mut stroke = Stroke::new((w * 0.18) as f32);
        stroke.cap = inset_embedder::valo::Cap::Round;
        stroke.join = inset_embedder::valo::Join::Round;
        canvas.draw_path(
            &path.build(),
            FillRule::NonZero,
            &Paint {
                color: self.color.into(),
                style: PaintStyle::Stroke(stroke),
                ..Default::default()
            },
        );
    }
    fn should_repaint(&self, _: &App, old: &dyn inset_rendering::CustomPainter) -> bool {
        old.as_any()
            .downcast_ref::<Self>()
            .is_none_or(|old| old.color != self.color || old.dash != self.dash)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
