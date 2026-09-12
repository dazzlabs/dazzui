use super::{Button, ValueChanged};
use crate::{ControlSize, ControlVariant, Theme, Tint};
use inset_foundation::{App, Handle};
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
