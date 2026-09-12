use super::{Checkbox, ChoiceItem, ValueChanged};
use crate::Theme;
use inset_foundation::App;
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::*;
use std::{collections::BTreeSet, rc::Rc};
#[derive(Clone)]
pub struct CheckboxGroup {
    pub items: Vec<ChoiceItem<String>>,
    pub value: BTreeSet<String>,
    pub parent_label: Option<String>,
    pub on_changed: Option<ValueChanged<BTreeSet<String>>>,
}
impl std::fmt::Debug for CheckboxGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckboxGroup")
            .field("items", &self.items)
            .finish()
    }
}
impl CheckboxGroup {
    pub fn new(items: Vec<ChoiceItem<String>>, value: BTreeSet<String>) -> Self {
        Self {
            items,
            value,
            parent_label: None,
            on_changed: None,
        }
    }
    pub fn on_changed(mut self, callback: impl Fn(&mut App, BTreeSet<String>) + 'static) -> Self {
        self.on_changed = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for CheckboxGroup {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut children = vec![];
        if let Some(label) = &self.parent_label {
            let enabled: BTreeSet<_> = self
                .items
                .iter()
                .filter(|i| i.enabled)
                .map(|i| i.value.clone())
                .collect();
            let all = enabled.iter().all(|i| self.value.contains(i));
            let none = enabled.iter().all(|i| !self.value.contains(i));
            let mut parent = Checkbox::tristate(if all {
                Some(true)
            } else if none {
                Some(false)
            } else {
                None
            })
            .label(label);
            if let Some(callback) = self.on_changed.clone() {
                let value = self.value.clone();
                parent = parent.on_changed(move |app, _| {
                    let mut next = value.clone();
                    for item in &enabled {
                        if all {
                            next.remove(item);
                        } else {
                            next.insert(item.clone());
                        }
                    }
                    callback(app, next);
                });
            }
            children.push(parent.into_widget());
        }
        for item in &self.items {
            let mut checkbox = Checkbox::new(self.value.contains(&item.value)).label(&item.label);
            if item.enabled
                && let Some(callback) = self.on_changed.clone()
            {
                let selected = self.value.clone();
                let value = item.value.clone();
                checkbox = checkbox.on_changed(move |app, checked| {
                    let mut next = selected.clone();
                    if checked == Some(true) {
                        next.insert(value.clone());
                    } else {
                        next.remove(&value);
                    }
                    callback(app, next);
                });
            }
            children.push(checkbox.into_widget());
        }
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .spacing(v.spacing_2_5)
            .children(children)
            .into_widget()
    }
}
