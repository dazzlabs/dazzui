//! Shared selection and the parent checkbox's three-state cycle.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gpui::{
    AnyElement, App, Component, ElementId, Entity, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, px,
};

use super::{Checkbox, kit};
use crate::{
    ActiveTheme,
    foundation::{provide, scoped},
};

type OnValueChange = Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App)>;

#[derive(Clone, Copy, Default)]
enum ParentStatus {
    #[default]
    Mixed,
    On,
    Off,
}

struct GroupState {
    value: Vec<SharedString>,
    previous: Vec<SharedString>,
    status: ParentStatus,
}

#[derive(Clone)]
pub(super) struct GroupScope {
    pub value: Vec<SharedString>,
    pub all_values: Option<Vec<SharedString>>,
    pub disabled: bool,
    pub disabled_states: Rc<RefCell<HashMap<SharedString, bool>>>,
    state: Entity<GroupState>,
    controlled: bool,
    on_value_change: Option<OnValueChange>,
}

impl GroupScope {
    pub fn current(cx: &App) -> Option<Self> {
        scoped(cx)
    }

    fn change(&self, next: Vec<SharedString>, window: &mut Window, cx: &mut App) {
        if let Some(handler) = &self.on_value_change {
            handler(next.clone(), window, cx);
        }
        if !self.controlled {
            self.state.update(cx, |state, cx| {
                state.value = next;
                cx.notify();
            });
        }
    }

    pub fn child_change(
        &self,
        value: &SharedString,
        checked: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut next = self.value.clone();
        if checked {
            next.push(value.clone());
        } else if self.all_values.is_some() {
            if let Some(index) = next.iter().position(|item| item == value) {
                next.remove(index);
            }
        } else {
            next.retain(|item| item != value);
        }
        self.change(next.clone(), window, cx);
        if self.all_values.is_some() {
            self.state.update(cx, |state, cx| {
                state.previous = next;
                state.status = ParentStatus::Mixed;
                cx.notify();
            });
        }
    }

    pub fn parent_change(&self, window: &mut Window, cx: &mut App) {
        let Some(values) = &self.all_values else {
            return;
        };
        let state = self.state.read(cx);
        let previous = state.previous.clone();
        let status = state.status;
        let disabled = self.disabled_states.borrow();
        let none: Vec<_> = values
            .iter()
            .filter(|value| disabled.get(*value) == Some(&true) && previous.contains(value))
            .cloned()
            .collect();
        let all: Vec<_> = values
            .iter()
            .filter(|value| disabled.get(*value) != Some(&true) || previous.contains(value))
            .cloned()
            .collect();
        drop(disabled);
        if previous.is_empty() || previous.len() == all.len() {
            let next = if self.value.len() == all.len() {
                none
            } else {
                all
            };
            self.change(next, window, cx);
        } else {
            let (next, status) = match status {
                ParentStatus::Mixed => (all, ParentStatus::On),
                ParentStatus::On => (none, ParentStatus::Off),
                ParentStatus::Off => (previous, ParentStatus::Mixed),
            };
            self.change(next, window, cx);
            self.state.update(cx, |state, cx| {
                state.status = status;
                cx.notify();
            });
        }
    }
}

/// Checkboxes that answer one question. Value-bearing children share the
/// selection; `.all_values(...)` also enables a `.parent(true)` checkbox.
#[derive(IntoElement)]
pub struct CheckboxGroup {
    id: ElementId,
    value: Option<Vec<SharedString>>,
    default_value: Vec<SharedString>,
    all_values: Option<Vec<SharedString>>,
    disabled: bool,
    children: Vec<AnyElement>,
    on_value_change: Option<OnValueChange>,
}

impl CheckboxGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
            default_value: Vec::new(),
            all_values: None,
            disabled: false,
            children: Vec::new(),
            on_value_change: None,
        }
    }

    pub fn value(mut self, values: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.value = Some(values.into_iter().map(Into::into).collect());
        self
    }

    pub fn default_value(
        mut self,
        values: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.default_value = values.into_iter().map(Into::into).collect();
        self
    }

    pub fn all_values(mut self, values: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.all_values = Some(values.into_iter().map(Into::into).collect());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for CheckboxGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

// Type erasure can be repeated by scene interpreters. It does not introduce
// a layout wrapper or make a checkbox cease to be a direct child.
fn direct_checkbox(element: &mut AnyElement) -> Option<&mut Checkbox> {
    if element.downcast_mut::<AnyElement>().is_some() {
        return direct_checkbox(element.downcast_mut::<AnyElement>().unwrap());
    }
    if element.transparent_child_mut().is_some() {
        return direct_checkbox(element.transparent_child_mut().unwrap());
    }
    element
        .downcast_mut::<Component<Checkbox>>()?
        .component_mut()
}

impl RenderOnce for CheckboxGroup {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = window.use_keyed_state(kit::child_id(&self.id, "group-value"), cx, {
            let initial = self.value.clone().unwrap_or(self.default_value);
            move |_, _| GroupState {
                value: initial.clone(),
                previous: initial,
                status: ParentStatus::Mixed,
            }
        });
        let value = self
            .value
            .clone()
            .unwrap_or_else(|| state.read(cx).value.clone());
        let scope = GroupScope {
            value,
            all_values: self.all_values,
            disabled: self.disabled,
            disabled_states: Rc::new(RefCell::new(HashMap::new())),
            state,
            controlled: self.value.is_some(),
            on_value_change: self.on_value_change,
        };
        let has_parent = self
            .children
            .iter_mut()
            .any(|child| direct_checkbox(child).is_some_and(|checkbox| checkbox.parent));
        for child in &mut self.children {
            if let Some(checkbox) = direct_checkbox(child) {
                checkbox.indented = has_parent && !checkbox.parent;
            }
        }
        provide(
            scope,
            div()
                .id(self.id)
                .flex()
                .flex_col()
                .gap(px(t.spacing_2_5))
                .children(self.children),
        )
    }
}
