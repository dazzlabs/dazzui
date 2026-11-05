use crate::{Size, Theme, Tint, styles};
pub use nsh_base::{SelectItem, SelectValue};
use nsh_core::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
    pub items: Prop<Vec<SelectItem>>,
    #[prop(default)]
    pub value: Prop<SelectValue>,
    pub default_value: Prop<Option<SharedString>>,
    #[prop(default)]
    pub placeholder: Prop<SharedString>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub searchable: Prop<bool>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum SelectEvent {
    #[args(value)]
    Change(Option<SharedString>),
}
#[component]
pub fn Select(p: SelectProps, cx: Scope<SelectEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let field = use_context::<super::form_field::FieldContext>();
    let error = move || field.is_some_and(|f| f.error.get());
    let tint = move || {
        if error() {
            Tint::Danger
        } else {
            field.map(|f| f.tint.get()).unwrap_or_else(|| p.tint.get())
        }
    };
    let own = signal(p.default_value.get_untracked());
    let value = move || match p.value.get() {
        SelectValue::Uncontrolled => own.get(),
        SelectValue::Controlled(value) => value,
    };
    nsh_base::select()
        .items(p.items)
        .value(move || SelectValue::Controlled(value()))
        .disabled(p.disabled)
        .searchable(p.searchable)
        .accessibility_label(move || {
            p.accessibility_label
                .get()
                .or_else(|| field.map(|f| f.label.get()))
        })
        .offset(move || px(theme.tokens().spacing_1))
        .popup_style(move |s| s.style_with(move |s| styles::popup(s, theme.tokens())))
        .item_class(move || {
            Some(
                format!(
                    "focus:bg-[#{:08x}] selected:font-semibold disabled:opacity-50",
                    theme.tokens().color_surface_subtle.0
                )
                .into(),
            )
        })
        .item_style(move |s| s.style_with(move |s| styles::popup_item(s, theme.tokens())))
        .trigger(
            view()
                .hint(move || field.map(|f| f.hint.get()).unwrap_or_default())
                .when_some(field, |trigger, f| trigger.element_ref(&f.editor))
                .flex_row()
                .items_center()
                .justify_between()
                .style_with(move |s| {
                    styles::field(
                        s,
                        theme.tokens(),
                        p.size.get(),
                        tint(),
                        p.disabled.get(),
                        error(),
                    )
                    .gap(px(theme.tokens().spacing_2))
                })
                .focus(move |s| {
                    s.style_with(move |s| styles::field_focus(s, theme.tokens(), tint(), error()))
                })
                .child(
                    text(move || {
                        p.items
                            .get()
                            .iter()
                            .find(|item| Some(&item.value) == value().as_ref())
                            .map(|item| item.title.clone())
                            .unwrap_or_else(|| p.placeholder.get())
                    })
                    .flex_1()
                    .min_w(px(0.0))
                    .truncate(),
                )
                .child(
                    crate::icon("ChevronDown12Regular").size(move || px(theme.tokens().icon_small)),
                ),
        )
        .on_change(move |next: &Option<SharedString>| {
            if matches!(p.value.get_untracked(), SelectValue::Uncontrolled) {
                own.set(next.clone());
            }
            cx.emit(SelectEvent::Change(next.clone()));
        })
}
