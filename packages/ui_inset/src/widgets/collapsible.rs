use super::{Button, ValueChanged};
use crate::{ControlVariant, Theme};
use inset_foundation::{App, Handle};
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct Collapsible {
    pub title: String,
    pub child: WidgetRef,
    pub open: Option<bool>,
    pub initially_open: bool,
    pub on_open_changed: Option<ValueChanged<bool>>,
}
impl std::fmt::Debug for Collapsible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Collapsible")
            .field("title", &self.title)
            .finish()
    }
}
impl Collapsible {
    pub fn new<K>(title: impl Into<String>, child: impl IntoWidget<K>) -> Self {
        Self {
            title: title.into(),
            child: child.into_widget(),
            open: None,
            initially_open: false,
            on_open_changed: None,
        }
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }
    pub fn on_open_changed(mut self, callback: impl Fn(&mut App, bool) + 'static) -> Self {
        self.on_open_changed = Some(Rc::new(callback));
        self
    }
}
impl StatefulWidget for Collapsible {
    type State = CollapsibleState;
    fn create_state(&self) -> Self::State {
        CollapsibleState {
            state: StateData::new(),
            open: self.initially_open,
        }
    }
}
pub struct CollapsibleState {
    state: StateData<Collapsible>,
    open: bool,
}
impl State for CollapsibleState {
    type Widget = Collapsible;
    inset_widgets::state_accessors!();
    fn build(self: Handle<Self>, app: &mut App, context: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let v = Theme::of(app, context);
        let open = w.open.unwrap_or(app.get(self).open);
        let title = format!("{}  {}", if open { "⌄" } else { "›" }, w.title);
        let header = Button::new(title)
            .variant(ControlVariant::Plain)
            .expand(true)
            .on_pressed(move |app| {
                self.set_state(app, |s| s.open = !open);
                if let Some(callback) = &w.on_open_changed {
                    callback(app, !open);
                }
            });
        let mut children = vec![header.into_widget()];
        if open {
            children.push(w.child);
        }
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .spacing(v.spacing_2)
            .children(children)
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct AccordionItem {
    pub title: String,
    pub content: WidgetRef,
}
impl AccordionItem {
    pub fn new<K>(title: impl Into<String>, child: impl IntoWidget<K>) -> Self {
        Self {
            title: title.into(),
            content: child.into_widget(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Accordion {
    pub items: Vec<AccordionItem>,
    pub multiple: bool,
    pub collapsible: bool,
}
impl Accordion {
    pub fn new(items: Vec<AccordionItem>) -> Self {
        Self {
            items,
            multiple: false,
            collapsible: true,
        }
    }
}
impl StatefulWidget for Accordion {
    type State = AccordionState;
    fn create_state(&self) -> Self::State {
        AccordionState {
            state: StateData::new(),
            open: std::collections::BTreeSet::new(),
        }
    }
}
pub struct AccordionState {
    state: StateData<Accordion>,
    open: std::collections::BTreeSet<usize>,
}
impl State for AccordionState {
    type Widget = Accordion;
    inset_widgets::state_accessors!();
    fn build(self: Handle<Self>, app: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let children: Vec<_> = w
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                Collapsible::new(&item.title, item.content.clone())
                    .open(app.get(self).open.contains(&i))
                    .on_open_changed(move |app, value| {
                        self.set_state(app, |s| {
                            if value {
                                if !w.multiple {
                                    s.open.clear();
                                }
                                s.open.insert(i);
                            } else if w.collapsible || s.open.len() > 1 {
                                s.open.remove(&i);
                            }
                        })
                    })
                    .into_widget()
            })
            .collect();
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .children(children)
            .into_widget()
    }
}
