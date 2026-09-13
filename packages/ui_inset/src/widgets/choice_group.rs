use super::{Pressable, ValueChanged};
use crate::{ControlSize, ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_painting::{BorderRadiusGeometry, BoxDecoration, EdgeInsetsGeometry};
use inset_rendering::MainAxisSize;
use inset_widgets::*;
use std::{fmt, rc::Rc};
#[derive(Debug, Clone)]
pub struct ChoiceItem<T> {
    pub value: T,
    pub label: String,
    pub enabled: bool,
    pub count: Option<usize>,
}
impl<T> ChoiceItem<T> {
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            enabled: true,
            count: None,
        }
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub enum SegmentedActiveStyle {
    #[default]
    Raised,
    Filled,
}
#[derive(Clone)]
pub struct SegmentedControl<T> {
    pub items: Vec<ChoiceItem<T>>,
    pub value: Option<T>,
    pub on_changed: Option<ValueChanged<T>>,
    pub size: ControlSize,
    pub active_style: SegmentedActiveStyle,
    pub stretch: bool,
    /// Paints a `Filled` active segment; `Raised` keeps the neutral ink.
    pub tint: Tint,
}
impl<T: fmt::Debug> fmt::Debug for SegmentedControl<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SegmentedControl")
            .field("items", &self.items)
            .finish()
    }
}
impl<T> SegmentedControl<T> {
    pub fn new(items: Vec<ChoiceItem<T>>, value: Option<T>) -> Self {
        Self {
            items,
            value,
            on_changed: None,
            size: ControlSize::Small,
            active_style: SegmentedActiveStyle::Raised,
            stretch: false,
            tint: Tint::Primary,
        }
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, T) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
}
impl<T: Clone + PartialEq + fmt::Debug + 'static> StatelessWidget for SegmentedControl<T> {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let m = v.metrics(self.size);
        let inset = v.segmented_control_inset;
        let children: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let selected = Some(&item.value) == self.value.as_ref();
                let label = item.label.clone();
                let active_style = self.active_style;
                let tint = self.tint;
                let mut p = Pressable::new(move |app, context, phase| {
                    let v = Theme::of(app, context);
                    let filled =
                        v.control_colors(ControlVariant::Filled, tint, ControlState::Normal);
                    let (surface, content) = if selected {
                        match active_style {
                            SegmentedActiveStyle::Raised => {
                                (v.color_surface_raised, v.color_content)
                            }
                            SegmentedActiveStyle::Filled => (filled.surface, filled.content),
                        }
                    } else {
                        (
                            if phase == ControlState::Hovered {
                                v.color_surface_subtle
                            } else {
                                crate::generated::colors::TRANSPARENT
                            },
                            v.color_content_secondary,
                        )
                    };
                    let mut face = v.label_quiet;
                    face.weight = if selected {
                        v.label_medium.weight
                    } else {
                        face.weight
                    };
                    Container::new()
                        .height(m.size - 2.0 * inset)
                        .padding(EdgeInsetsGeometry::symmetric(0.0, v.spacing_3))
                        .decoration(
                            BoxDecoration::new()
                                .color(surface)
                                .box_shadow(if selected {
                                    crate::shadows(v.shadow_2xs)
                                } else {
                                    vec![]
                                })
                                .border_radius(BorderRadiusGeometry::circular(m.radius)),
                        )
                        .child(
                            Center::new()
                                .width_factor(1.0)
                                .child(Text::new(&label).style(face.text_style().color(
                                    if phase == ControlState::Disabled {
                                        v.color_content_faint
                                    } else {
                                        content
                                    },
                                ))),
                        )
                        .into_widget()
                });
                if item.enabled
                    && let Some(callback) = self.on_changed.clone()
                {
                    let value = item.value.clone();
                    p.on_pressed = Some(Rc::new(move |app| callback(app, value.clone())));
                }
                p.radius = m.radius;
                if self.stretch {
                    Expanded::new(p).into_widget()
                } else {
                    p.into_widget()
                }
            })
            .collect();
        Container::new()
            .padding(EdgeInsetsGeometry::all(inset))
            .decoration(
                BoxDecoration::new()
                    .color(v.color_surface_inset)
                    .border_radius(BorderRadiusGeometry::circular(m.radius)),
            )
            .child(
                Row::new()
                    .main_axis_size(if self.stretch {
                        MainAxisSize::Max
                    } else {
                        MainAxisSize::Min
                    })
                    .children(children),
            )
            .into_widget()
    }
}
#[derive(Clone)]
pub struct PillTabs<T> {
    pub items: Vec<ChoiceItem<T>>,
    pub value: Option<T>,
    pub tint: Tint,
    pub on_changed: Option<ValueChanged<T>>,
}
impl<T: fmt::Debug> fmt::Debug for PillTabs<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PillTabs")
            .field("items", &self.items)
            .finish()
    }
}
impl<T> PillTabs<T> {
    pub fn new(items: Vec<ChoiceItem<T>>, value: Option<T>) -> Self {
        Self {
            items,
            value,
            tint: Tint::Primary,
            on_changed: None,
        }
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, T) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
}
impl<T: Clone + PartialEq + fmt::Debug + 'static> StatelessWidget for PillTabs<T> {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let children: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let selected = Some(&item.value) == self.value.as_ref();
                let label = item
                    .count
                    .map(|n| format!("{}  {n}", item.label))
                    .unwrap_or_else(|| item.label.clone());
                let tint = self.tint;
                let mut p = Pressable::new(move |app, context, phase| {
                    let v = Theme::of(app, context);
                    let filled =
                        v.control_colors(ControlVariant::Filled, tint, ControlState::Normal);
                    let mut face = v.label_small;
                    if selected {
                        face.weight = v.label_strong.weight;
                    }
                    Container::new()
                        .height(v.control_tiny_size)
                        .padding(EdgeInsetsGeometry::symmetric(0.0, v.spacing_3))
                        .decoration(
                            BoxDecoration::new()
                                .color(if selected {
                                    filled.surface
                                } else if phase == ControlState::Hovered {
                                    v.color_surface_sunken
                                } else {
                                    v.color_surface_inset
                                })
                                .border_radius(BorderRadiusGeometry::circular(v.radius_full)),
                        )
                        .child(
                            Center::new()
                                .width_factor(1.0)
                                .child(Text::new(&label).style(face.text_style().color(
                                    if phase == ControlState::Disabled {
                                        v.color_content_faint
                                    } else if selected {
                                        filled.content
                                    } else {
                                        v.color_content_nav
                                    },
                                ))),
                        )
                        .into_widget()
                });
                p.radius = v.radius_full;
                if item.enabled
                    && let Some(callback) = self.on_changed.clone()
                {
                    let value = item.value.clone();
                    p.on_pressed = Some(Rc::new(move |app| callback(app, value.clone())));
                }
                p.into_widget()
            })
            .collect();
        Row::new()
            .main_axis_size(MainAxisSize::Min)
            .spacing(v.spacing_2)
            .children(children)
            .into_widget()
    }
}
