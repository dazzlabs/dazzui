use super::{
    Button, ChoiceItem, MenuItem, MenuPanel, Popover, PopoverAlign, TextField, ValueChanged,
};
use crate::{ControlSize, ControlVariant};
use inset_foundation::{App, Handle};
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct Combobox {
    pub items: Vec<ChoiceItem<String>>,
    pub value: Option<String>,
    pub placeholder: String,
    pub on_changed: Option<ValueChanged<String>>,
}
impl std::fmt::Debug for Combobox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Combobox")
            .field("items", &self.items)
            .finish()
    }
}
impl Combobox {
    pub fn new(items: Vec<ChoiceItem<String>>) -> Self {
        Self {
            items,
            value: None,
            placeholder: "Select…".into(),
            on_changed: None,
        }
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, String) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
}
impl StatefulWidget for Combobox {
    type State = ComboboxState;
    fn create_state(&self) -> Self::State {
        ComboboxState {
            state: StateData::new(),
            controller: None,
            query: String::new(),
        }
    }
}
pub struct ComboboxState {
    state: StateData<Combobox>,
    controller: Option<Handle<TextEditingController>>,
    query: String,
}
impl State for ComboboxState {
    type Widget = Combobox;
    inset_widgets::state_accessors!();
    fn init_state(self: Handle<Self>, app: &mut App) {
        let label = self
            .widget(app)
            .items
            .iter()
            .find(|i| Some(&i.value) == self.widget(app).value.as_ref())
            .map(|i| i.label.clone())
            .unwrap_or_default();
        let c = TextEditingController::new(app).text(app, label);
        app.get_mut(self).controller = Some(c);
    }
    fn did_update_widget(self: Handle<Self>, app: &mut App, old: &Combobox) {
        if old.value != self.widget(app).value {
            let label = self
                .widget(app)
                .items
                .iter()
                .find(|i| Some(&i.value) == self.widget(app).value.as_ref())
                .map(|i| i.label.clone())
                .unwrap_or_default();
            app.get(self).controller.unwrap().set_text(app, label);
        }
    }
    fn dispose(self: Handle<Self>, app: &mut App) {
        let c = app.get_mut(self).controller.take().unwrap();
        app.get_mut(c).dispose();
        app.destroy(c);
    }
    fn build(self: Handle<Self>, app: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let c = app.get(self).controller.unwrap();
        let mut popup = Popover::new(
            move |_, _, open, toggle| {
                let arrow_toggle = toggle.clone();
                let mut field = TextField::new()
                    .controller(c)
                    .placeholder(&w.placeholder)
                    .disabled(w.on_changed.is_none())
                    .on_changed(move |app, query| {
                        self.set_state(app, |s| s.query = query);
                        if !open {
                            toggle(app);
                        }
                    });
                let mut arrow = Button::new("⌄")
                    .size(ControlSize::Tiny)
                    .variant(ControlVariant::Plain);
                if w.on_changed.is_some() {
                    arrow = arrow.on_pressed(move |app| arrow_toggle(app));
                }
                field.suffix = Some(arrow.into_widget());
                field.into_widget()
            },
            SizedBox::shrink(),
        );
        popup.align = PopoverAlign::Start;
        popup.content_builder = Some(Rc::new(move |app, _, close| {
            let w = self.widget(app).clone();
            let query = app.get(self).query.to_lowercase();
            let items: Vec<_> = w
                .items
                .iter()
                .filter(|item| item.label.to_lowercase().contains(&query))
                .map(|item| {
                    let value = item.value.clone();
                    let label = item.label.clone();
                    let callback = w.on_changed.clone();
                    let mut option = MenuItem::new(&item.label).on_select(move |app| {
                        c.set_text(app, &label);
                        self.set_state(app, |s| s.query.clear());
                        if let Some(callback) = &callback {
                            callback(app, value.clone());
                        }
                    });
                    option.enabled = item.enabled;
                    option.checked = Some(w.value.as_ref() == Some(&item.value));
                    option
                })
                .collect();
            if items.is_empty() {
                super::PopoverPanel::new(Text::new("No results")).into_widget()
            } else {
                MenuPanel {
                    items,
                    on_dismiss: Some(close),
                }
                .into_widget()
            }
        }));
        popup.into_widget()
    }
}
