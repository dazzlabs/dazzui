use super::{Badge, BadgeVariant, Button, Callback, Pressable, ValueChanged};
use crate::{ControlSize, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_rendering::MainAxisSize;
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct Tag {
    pub label: String,
    pub tint: Tint,
    pub size: ControlSize,
    pub selected: Option<bool>,
    pub on_selected_changed: Option<ValueChanged<bool>>,
    pub on_remove: Option<Callback>,
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tag").field("label", &self.label).finish()
    }
}
impl Tag {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            tint: Tint::Primary,
            size: ControlSize::Medium,
            selected: None,
            on_selected_changed: None,
            on_remove: None,
        }
    }
    pub fn on_remove(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.on_remove = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for Tag {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let badge = Badge::new(&self.label)
            .tint(self.tint)
            .size(self.size)
            .variant(if self.selected == Some(true) {
                BadgeVariant::Outlined
            } else {
                BadgeVariant::Tinted
            });
        let mut children = vec![];
        if let Some(selected) = self.selected {
            let mut p = Pressable::new(move |_, _, _| badge.clone().into_widget());
            if let Some(callback) = self.on_selected_changed.clone() {
                p.on_pressed = Some(Rc::new(move |app| callback(app, !selected)));
            }
            children.push(p.into_widget());
        } else {
            children.push(badge.into_widget());
        }
        if let Some(callback) = self.on_remove.clone() {
            children.push(
                Button::new("×")
                    .variant(ControlVariant::Plain)
                    .size(ControlSize::Tiny)
                    .on_pressed(move |app| callback(app))
                    .into_widget(),
            );
        }
        Row::new()
            .main_axis_size(MainAxisSize::Min)
            .spacing(v.spacing_0_5)
            .children(children)
            .into_widget()
    }
}
