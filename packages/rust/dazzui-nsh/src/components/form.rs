//! Native editors own editing; the form coordinates validation and reset.
use crate::{Theme, styles};
use nsh_core::prelude::*;
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};
#[derive(Clone)]
pub(crate) struct FieldRegistration {
    pub validate: Rc<dyn Fn() -> bool>,
    pub reset: Rc<dyn Fn()>,
    pub focus: nsh_core::element::ElementRef,
}
#[derive(Clone)]
pub(crate) struct FormContext {
    fields: Rc<RefCell<BTreeMap<usize, FieldRegistration>>>,
    next: Rc<Cell<usize>>,
    submit: Rc<dyn Fn()>,
}
impl FormContext {
    pub fn register(&self, field: FieldRegistration) {
        let id = self.next.get();
        self.next.set(id + 1);
        self.fields.borrow_mut().insert(id, field);
        let fields = self.fields.clone();
        cleanup(move || {
            fields.borrow_mut().remove(&id);
        });
    }
    pub fn submit(&self) {
        let fields = self.fields.borrow().values().cloned().collect::<Vec<_>>();
        let invalid = fields
            .iter()
            .filter(|f| !(f.validate)())
            .collect::<Vec<_>>();
        if let Some(first) = invalid.first() {
            first.focus.focus();
        } else {
            (self.submit)();
        }
    }
    pub fn reset(&self) {
        let fields = self.fields.borrow().values().cloned().collect::<Vec<_>>();
        for field in fields {
            (field.reset)();
        }
    }
}
#[derive(Clone, PartialEq, Props)]
pub struct FormProps {
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum FormEvent {
    Submit,
}
#[component]
pub fn Form(p: FormProps, cx: Scope<FormEvent>) -> impl IntoElement {
    let theme = Theme::current();
    provide_context(FormContext {
        fields: Default::default(),
        next: Default::default(),
        submit: Rc::new(move || cx.emit(FormEvent::Submit)),
    });
    view()
        .gap(move || px(theme.tokens().spacing_4))
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct FieldsetProps {
    pub legend: Prop<SharedString>,
    pub children: Prop<Children>,
}
#[component]
pub fn Fieldset(p: FieldsetProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .role(AccessibilityRole::Group)
        .label(p.legend)
        .gap(move || px(theme.tokens().spacing_3))
        .child(
            text(p.legend).style_with(move |s| styles::typography(s, theme.tokens().label_small)),
        )
        .child(p.children)
}
