use crate::{Size, Theme, Tint, Variant, styles, tokens::Face};
use nsh_core::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct NumberFieldProps {
    pub value: Prop<Option<f64>>,
    pub default_value: Prop<Option<f64>>,
    pub min: Prop<Option<f64>>,
    pub max: Prop<Option<f64>>,
    #[prop(default = 1.0)]
    pub step: Prop<f64>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub read_only: Prop<bool>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub error: Prop<bool>,
    pub unit: Prop<Option<SharedString>>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum NumberFieldEvent {
    #[args(value)]
    Change(Option<f64>),
}
#[component]
pub fn NumberField(p: NumberFieldProps, cx: Scope<NumberFieldEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let field = use_context::<super::form_field::FieldContext>();
    nsh_base::number_input()
        .value(p.value)
        .default_value(p.default_value)
        .min(p.min)
        .max(p.max)
        .step(p.step)
        .disabled(p.disabled)
        .readonly(p.read_only)
        .accessibility_label(move || {
            p.accessibility_label
                .get()
                .or_else(|| field.map(|f| f.label.get()))
        })
        .controls_right(true)
        .style_with(move |s| {
            styles::field(
                s,
                theme.tokens(),
                p.size.get(),
                p.tint.get(),
                p.disabled.get(),
                p.error.get(),
            )
            .flex_row()
            .items_center()
            .gap(px(theme.tokens().spacing_1))
            .pr(px(theme.tokens().spacing_1))
        })
        .input_style(|s| s.flex_1().min_w(px(0.0)).p(px(0.0)).border(px(0.0)))
        .controls_style(move |s| {
            s.flex_col()
                .justify_center()
                .border_l(move || px(theme.tokens().stroke_hairline))
                .border_color(move || theme.tokens().color_border.hsla())
                .pl(move || px(theme.tokens().spacing_1))
        })
        .increment_button_style(move |s| {
            s.w(move || px(theme.tokens().spacing_4))
                .h(move || px(theme.tokens().spacing_3))
        })
        .decrement_button_style(move |s| {
            s.w(move || px(theme.tokens().spacing_4))
                .h(move || px(theme.tokens().spacing_3))
        })
        .increment_button(crate::icon("ChevronUp12Regular"))
        .decrement_button(crate::icon("ChevronDown12Regular"))
        .suffix(
            text(move || p.unit.get().unwrap_or_default())
                .text_color(move || theme.tokens().color_content_faint.hsla()),
        )
        .on_change(move |v: &Option<f64>| cx.emit(NumberFieldEvent::Change(*v)))
}

#[derive(Clone, PartialEq, Props)]
pub struct OtpFieldProps {
    #[prop(default = 6)]
    pub length: Prop<u32>,
    #[prop(default = 1)]
    pub groups: Prop<u32>,
    pub value: Prop<Option<SharedString>>,
    pub default_value: Prop<Option<SharedString>>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub error: Prop<bool>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum OtpFieldEvent {
    #[args(value)]
    Change(SharedString),
    #[args(value)]
    Complete(SharedString),
}
#[component]
pub fn OtpField(p: OtpFieldProps, cx: Scope<OtpFieldEvent>) -> impl IntoElement {
    let theme = Theme::current();
    nsh_base::otp_input()
        .flex_row()
        .items_center()
        .length(p.length)
        .groups(p.groups)
        .value(p.value)
        .default_value(p.default_value)
        .disabled(p.disabled)
        .accessibility_label(p.accessibility_label)
        .gap(move || px(theme.tokens().spacing_2 + theme.tokens().spacing_3))
        .group_style(move |s| s.flex_row().gap(move || px(theme.tokens().spacing_2)))
        .cell_class(move || {
            Some(
                format!(
                    "focus:border-[#{:08x}] focus:bg-[#{:08x}] data-filled:bg-[#{:08x}]",
                    p.tint.get().ramp(theme.tokens()).shade(600).0,
                    theme.tokens().color_surface.0,
                    theme.tokens().color_surface.0
                )
                .into(),
            )
        })
        .cell_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                styles::field(
                    s,
                    t,
                    p.size.get(),
                    if p.error.get() {
                        Tint::Danger
                    } else {
                        p.tint.get()
                    },
                    p.disabled.get(),
                    p.error.get(),
                )
                .w(px(p.size.get().profile(t).size))
                .p(px(0.0))
                .font_family(Face::Mono.family())
            })
        })
        .caret_style(move |s| {
            s.w(move || px(theme.tokens().stroke_control))
                .h(move || px(theme.tokens().label_medium.line_height))
                .bg(move || theme.tokens().color_content.hsla())
        })
        .on_change(move |v: &SharedString| cx.emit(OtpFieldEvent::Change(v.clone())))
        .on_complete(move |v: &nsh_base::OtpInputCompleteEvent| {
            cx.emit(OtpFieldEvent::Complete(v.value.clone()))
        })
}

#[derive(Clone, PartialEq, Props)]
pub struct ComboboxProps {
    pub items: Prop<Vec<nsh_base::SelectItem>>,
    pub value: Prop<Option<Vec<SharedString>>>,
    #[prop(default)]
    pub default_value: Prop<Vec<SharedString>>,
    pub query: Prop<Option<SharedString>>,
    pub placeholder: Prop<Option<SharedString>>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub multiple: Prop<bool>,
    #[prop(default)]
    pub clearable: Prop<bool>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub empty: Prop<Option<Slot>>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum ComboboxEvent {
    #[args(value)]
    Change(Vec<SharedString>),
    #[args(value)]
    QueryChange(SharedString),
}
#[component]
pub fn Combobox(p: ComboboxProps, cx: Scope<ComboboxEvent>) -> impl IntoElement {
    let theme = Theme::current();
    nsh_base::combobox()
        .items(p.items)
        .value(p.value)
        .default_value(p.default_value)
        .query(p.query)
        .placeholder(p.placeholder)
        .disabled(p.disabled)
        .multiple(p.multiple)
        .cleanable(p.clearable)
        .accessibility_label(p.accessibility_label)
        .style_with(move |s| {
            styles::field(
                s,
                theme.tokens(),
                p.size.get(),
                p.tint.get(),
                p.disabled.get(),
                false,
            )
            .flex_row()
            .items_center()
        })
        .input_style(|s| s.flex_1().min_w(px(0.0)).p(px(0.0)).border(px(0.0)))
        .popup_style(move |s| s.style_with(move |s| styles::popup(s, theme.tokens())))
        .item_style(move |s| s.style_with(move |s| styles::popup_item(s, theme.tokens())))
        .item_class(move || {
            Some(
                format!(
                    "focus:bg-[#{:08x}] selected:font-semibold",
                    theme.tokens().color_surface_subtle.0
                )
                .into(),
            )
        })
        .clear_button(crate::icon("Dismiss12Regular"))
        .trigger_button(crate::icon("ChevronDown12Regular"))
        .when_some(p.empty.get_untracked(), |el, slot| el.empty(slot.element()))
        .on_change(move |v: &Vec<SharedString>| cx.emit(ComboboxEvent::Change(v.clone())))
        .on_query_change(move |v: &SharedString| cx.emit(ComboboxEvent::QueryChange(v.clone())))
}

#[derive(Clone, PartialEq, Props)]
pub struct SearchFieldProps {
    pub value: Prop<Option<SharedString>>,
    #[prop(default)]
    pub default_value: Prop<SharedString>,
    #[prop(default)]
    pub placeholder: Prop<SharedString>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum SearchFieldEvent {
    #[args(value)]
    Change(SharedString),
}
#[component]
pub fn SearchField(p: SearchFieldProps, cx: Scope<SearchFieldEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let own = signal(p.default_value.get_untracked());
    let focused = signal(false);
    let field = nsh_core::element::element_ref();
    let value = move || p.value.get().unwrap_or_else(|| own.get());
    view()
        .flex_row()
        .items_center()
        .style_with(move |s| {
            let t = theme.tokens();
            let s = styles::field(s, t, p.size.get(), Tint::Primary, p.disabled.get(), false)
                .rounded(px(t.radius_medium))
                .gap(px(p.size.get().profile(t).gap));
            if focused.get() {
                styles::field_focus(s, t, Tint::Primary, false)
            } else {
                s
            }
        })
        .child(crate::icon("Search16Regular"))
        .child(
            nsh_core::element::input()
                .element_ref(&field)
                .value(move || value().to_string())
                .placeholder(move || p.placeholder.get().to_string())
                .optional_label(p.accessibility_label)
                .disabled(p.disabled)
                .flex_1()
                .min_w(px(0.0))
                .w(move || px(theme.tokens().search_field_input_width))
                .on_focus(move || focused.set(true))
                .on_blur(move || focused.set(false))
                .on_change(move |v: &str| {
                    if p.value.get_untracked().is_none() {
                        own.set(v.to_owned().into());
                    }
                    cx.emit(SearchFieldEvent::Change(v.to_owned().into()));
                }),
        )
        .child(dynamic(move || {
            (!value().is_empty()).then(|| {
                crate::button()
                    .variant(Variant::Plain)
                    .size(Size::Tiny)
                    .disabled(p.disabled)
                    .accessibility_label("Clear search")
                    .child(crate::icon("Dismiss12Regular"))
                    .on_press(move || {
                        if p.value.get_untracked().is_none() {
                            own.set(SharedString::default());
                        }
                        cx.emit(SearchFieldEvent::Change(SharedString::default()));
                        field.focus();
                    })
            })
        }))
}

#[derive(Clone, PartialEq, Props)]
pub struct AutocompleteProps {
    pub items: Prop<Vec<SharedString>>,
    pub value: Prop<Option<SharedString>>,
    pub placeholder: Prop<Option<SharedString>>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum AutocompleteEvent {
    #[args(value)]
    Change(SharedString),
}
#[component]
pub fn Autocomplete(p: AutocompleteProps, cx: Scope<AutocompleteEvent>) -> impl IntoElement {
    let own = signal(SharedString::default());
    combobox()
        .items(move || {
            p.items
                .get()
                .into_iter()
                .map(|s| nsh_base::SelectItem::new(s.clone(), s))
                .collect::<Vec<_>>()
        })
        .value(Some(vec![]))
        .query(move || Some(p.value.get().unwrap_or_else(|| own.get())))
        .placeholder(p.placeholder)
        .accessibility_label(p.accessibility_label)
        .on_query_change(move |v: &SharedString| {
            if p.value.get_untracked().is_none() {
                own.set(v.clone());
            }
            cx.emit(AutocompleteEvent::Change(v.clone()));
        })
        .on_change(move |v: &Vec<SharedString>| {
            if let Some(v) = v.first() {
                if p.value.get_untracked().is_none() {
                    own.set(v.clone());
                }
                cx.emit(AutocompleteEvent::Change(v.clone()));
            }
        })
}
