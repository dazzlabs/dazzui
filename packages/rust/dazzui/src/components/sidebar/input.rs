//! The existing text field at the sidebar's default scale.
use super::*;
use crate::{FieldState, TextField};
use gpui::SharedString;

#[derive(IntoElement)]
pub struct SidebarInput {
    field: TextField,
}
impl SidebarInput {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            field: TextField::new(id).size(Size::Small),
        }
    }
}
macro_rules! field_property {
    ($($name:ident: $ty:ty),* $(,)?) => {impl SidebarInput {$(
        pub fn $name(mut self, value: $ty) -> Self {self.field = self.field.$name(value);self}
    )*}};
}
field_property!(size:Size, state:FieldState, tint:Tint, mono:bool, read_only:bool,
    disabled:bool, multiline:bool, rows:usize, required:bool, password:bool);
macro_rules! text_property {
    ($($name:ident),*) => {impl SidebarInput {$(
        pub fn $name(mut self, value: impl Into<SharedString>) -> Self {self.field = self.field.$name(value);self}
    )*}};
}
text_property!(value, default_value, placeholder, aria_label);
impl SidebarInput {
    pub fn input_type(mut self, value: &str) -> Self {
        self.field = self.field.input_type(value);
        self
    }
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.field = self.field.on_change(handler);
        self
    }
    pub fn on_submit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.field = self.field.on_submit(handler);
        self
    }
    pub fn on_blur(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.field = self.field.on_blur(handler);
        self
    }
}
impl RenderOnce for SidebarInput {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.field
    }
}
