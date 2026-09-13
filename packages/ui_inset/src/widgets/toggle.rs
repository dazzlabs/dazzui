use super::{Button, ValueChanged};
use crate::{ControlSize, ControlState, ControlVariant, Theme, Tint};
use inset_foundation::{App, Handle};
use inset_painting::{Border, BorderRadiusGeometry, BorderStyle, BoxDecoration};
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct Toggle {
    pub child: WidgetRef,
    pub pressed: Option<bool>,
    pub initially_pressed: bool,
    pub enabled: bool,
    pub on_changed: Option<ValueChanged<bool>>,
    pub size: ControlSize,
    pub tint: Tint,
    pub variant: ControlVariant,
}
impl std::fmt::Debug for Toggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Toggle")
            .field("pressed", &self.pressed)
            .finish()
    }
}
impl Toggle {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            child: Text::new(label).into_widget(),
            pressed: None,
            initially_pressed: false,
            enabled: true,
            on_changed: None,
            size: ControlSize::Tiny,
            tint: Tint::Neutral,
            variant: ControlVariant::Plain,
        }
    }
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, bool) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
}
impl StatefulWidget for Toggle {
    type State = ToggleState;
    fn create_state(&self) -> ToggleState {
        ToggleState {
            state: StateData::new(),
            pressed: self.initially_pressed,
        }
    }
}
pub struct ToggleState {
    state: StateData<Toggle>,
    pressed: bool,
}
impl State for ToggleState {
    type Widget = Toggle;
    inset_widgets::state_accessors!();
    fn build(self: Handle<Self>, app: &mut App, context: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let pressed = w.pressed.unwrap_or(app.get(self).pressed);
        let v = Theme::of(app, context);
        let child = if pressed {
            DefaultTextStyle::new(
                DefaultTextStyle::of(app, context)
                    .style
                    .clone()
                    .font_weight(v.label_strong.weight),
                w.child,
            )
            .into_widget()
        } else {
            w.child
        };
        let mut b = Button::child(child)
            .size(w.size)
            .tint(w.tint)
            .variant(if pressed {
                ControlVariant::Tinted
            } else {
                w.variant
            });
        if w.enabled {
            b = b.on_pressed(move |app| {
                self.set_state(app, |s| s.pressed = !pressed);
                if let Some(callback) = &w.on_changed {
                    callback(app, !pressed);
                }
            });
        }
        b.into_widget()
    }
}
/// One member of a [`ToggleGroup`].
#[derive(Debug, Clone)]
pub struct ToggleItem {
    pub value: String,
    pub child: WidgetRef,
    pub enabled: bool,
}
impl ToggleItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            child: Text::new(label).into_widget(),
            enabled: true,
        }
    }
}
/// Several toggles joined into one control: the corners are rounded at the ends
/// of the run only and a pair shares a single rule. Several can be on at once,
/// or none; a group that must always have exactly one is a `SegmentedControl`.
#[derive(Clone)]
pub struct ToggleGroup {
    pub items: Vec<ToggleItem>,
    pub value: Vec<String>,
    pub multiple: bool,
    pub size: ControlSize,
    pub variant: ControlVariant,
    pub tint: Tint,
    pub on_changed: Option<ValueChanged<Vec<String>>>,
}
impl std::fmt::Debug for ToggleGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToggleGroup")
            .field("value", &self.value)
            .finish()
    }
}
impl ToggleGroup {
    pub fn new(items: Vec<ToggleItem>, value: Vec<String>) -> Self {
        Self {
            items,
            value,
            multiple: true,
            size: ControlSize::Small,
            variant: ControlVariant::Recessed,
            tint: Tint::Primary,
            on_changed: None,
        }
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, Vec<String>) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for ToggleGroup {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let edge = v
            .control_colors(self.variant, self.tint, ControlState::Normal)
            .border;
        let radius = v.metrics(self.size).radius;
        let mut children = Vec::new();
        for (index, item) in self.items.iter().enumerate() {
            if index > 0 {
                children.push(
                    Container::new()
                        .width(v.spacing_px)
                        .color(edge)
                        .into_widget(),
                );
            }
            let mut toggle = Toggle::new("").pressed(self.value.contains(&item.value));
            toggle.child = item.child.clone();
            toggle.enabled = item.enabled;
            toggle.size = self.size;
            toggle.variant = self.variant;
            toggle.tint = self.tint;
            if let Some(callback) = self.on_changed.clone() {
                let (value, key, multiple) =
                    (self.value.clone(), item.value.clone(), self.multiple);
                toggle = toggle.on_changed(move |app, _| {
                    let on = value.contains(&key);
                    let next = match (multiple, on) {
                        (true, true) => value.iter().filter(|v| **v != key).cloned().collect(),
                        (true, false) => value.iter().cloned().chain([key.clone()]).collect(),
                        (false, true) => vec![],
                        (false, false) => vec![key.clone()],
                    };
                    callback(app, next)
                });
            }
            children.push(toggle.into_widget());
        }
        Container::new()
            .decoration(
                BoxDecoration::new()
                    .border(Border::all(edge, v.spacing_px, BorderStyle::Solid, -1.0))
                    .border_radius(BorderRadiusGeometry::circular(radius)),
            )
            .child(
                ClipRRect::new()
                    .border_radius(BorderRadiusGeometry::circular(radius))
                    .child(
                        IntrinsicHeight::new().child(
                            Row::new()
                                .main_axis_size(MainAxisSize::Min)
                                .cross_axis_alignment(CrossAxisAlignment::Stretch)
                                .children(children),
                        ),
                    ),
            )
            .into_widget()
    }
}
