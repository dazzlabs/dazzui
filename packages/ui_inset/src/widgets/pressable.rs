//! Port of Flutter's DazzUI Pressable: interaction is resolved before drawing.
use crate::{ControlState, Theme};
use inset_foundation::{App, Handle};
use inset_painting::{Border, BorderRadiusGeometry, BorderStyle, BoxDecoration};
use inset_rendering::{DecorationPosition, HitTestBehavior};
use inset_services::LogicalKeyboardKey;
use inset_widgets::*;
use std::{any::TypeId, collections::HashMap, rc::Rc};

pub type Callback = Rc<dyn Fn(&mut App)>;
pub type PressableBuilder = Rc<dyn Fn(&mut App, BuildContext, ControlState) -> WidgetRef>;
#[derive(Clone)]
pub struct Pressable {
    pub on_pressed: Option<Callback>,
    pub builder: PressableBuilder,
    pub radius: f64,
    pub autofocus: bool,
}
impl std::fmt::Debug for Pressable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pressable")
            .field("enabled", &self.on_pressed.is_some())
            .finish()
    }
}
impl Pressable {
    pub fn new(
        builder: impl Fn(&mut App, BuildContext, ControlState) -> WidgetRef + 'static,
    ) -> Self {
        Self {
            on_pressed: None,
            builder: Rc::new(builder),
            radius: 0.0,
            autofocus: false,
        }
    }
    pub fn on_pressed(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.on_pressed = Some(Rc::new(callback));
        self
    }
}
impl StatefulWidget for Pressable {
    type State = PressableState;
    fn create_state(&self) -> Self::State {
        PressableState {
            state: StateData::new(),
            hovered: false,
            pressed: false,
            focused: false,
            actions: HashMap::new(),
        }
    }
}
pub struct PressableState {
    state: StateData<Pressable>,
    hovered: bool,
    pressed: bool,
    focused: bool,
    actions: HashMap<TypeId, AnyAction>,
}
impl PressableState {
    fn activate(self: Handle<Self>, app: &mut App) {
        if let Some(callback) = self.widget(app).on_pressed.clone() {
            callback(app);
        }
    }
}
impl State for PressableState {
    type Widget = Pressable;
    inset_widgets::state_accessors!();
    fn init_state(self: Handle<Self>, app: &mut App) {
        let action = CallbackAction::<ActivateIntent>::new(
            app,
            Rc::new(move |app, _| {
                self.activate(app);
                None
            }),
        );
        app.get_mut(self)
            .actions
            .insert(TypeId::of::<ActivateIntent>(), action.as_action());
    }
    fn build(self: Handle<Self>, app: &mut App, context: BuildContext) -> WidgetRef {
        let widget = self.widget(app).clone();
        let enabled = widget.on_pressed.is_some();
        let state = app.get(self);
        let phase = if !enabled {
            ControlState::Disabled
        } else if state.pressed {
            ControlState::Pressed
        } else if state.hovered {
            ControlState::Hovered
        } else {
            ControlState::Normal
        };
        let focused = state.focused && enabled;
        let mut child = (widget.builder)(app, context, phase);
        if focused {
            let vars = Theme::of(app, context);
            child = DecoratedBox::new(
                BoxDecoration::new()
                    .border(Border::all(
                        vars.focus_ring(),
                        vars.focus_width,
                        BorderStyle::Solid,
                        1.0,
                    ))
                    .border_radius(BorderRadiusGeometry::circular(widget.radius)),
            )
            .position(DecorationPosition::Foreground)
            .child(child)
            .into_widget();
        }
        let mut gesture = GestureDetector::new()
            .behavior(HitTestBehavior::Opaque)
            .child(child);
        if enabled {
            gesture = gesture
                .on_tap(inset_foundation::Listener::new(move |app| {
                    self.activate(app)
                }))
                .on_tap_down(Rc::new(move |app, _| {
                    self.set_state(app, |s| s.pressed = true)
                }))
                .on_tap_up(Rc::new(move |app, _| {
                    self.set_state(app, |s| s.pressed = false)
                }))
                .on_tap_cancel(inset_foundation::Listener::new(move |app| {
                    self.set_state(app, |s| s.pressed = false)
                }));
        }
        FocusableActionDetector::new(gesture)
            .enabled(enabled)
            .autofocus(widget.autofocus)
            .mouse_cursor(if enabled {
                inset_services::SystemMouseCursors::CLICK.into()
            } else {
                inset_services::SystemMouseCursors::FORBIDDEN.into()
            })
            .actions(app.get(self).actions.clone())
            .shortcuts(vec![
                (
                    Rc::new(SingleActivator::new(LogicalKeyboardKey::ENTER)),
                    Rc::new(ActivateIntent),
                ),
                (
                    Rc::new(SingleActivator::new(LogicalKeyboardKey::SPACE)),
                    Rc::new(ActivateIntent),
                ),
            ])
            .on_show_focus_highlight(move |app, value| self.set_state(app, |s| s.focused = value))
            .on_show_hover_highlight(move |app, value| self.set_state(app, |s| s.hovered = value))
            .into_widget()
    }
}
