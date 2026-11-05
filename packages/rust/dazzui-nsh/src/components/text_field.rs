use crate::{Size, Theme, Tint, styles, tokens::Face};
use nsh_core::{element::input, prelude::*};

#[derive(Clone, PartialEq, Props)]
pub struct TextFieldProps {
    pub value: Prop<Option<SharedString>>,
    #[prop(default)]
    pub default_value: Prop<SharedString>,
    #[prop(default)]
    pub placeholder: Prop<SharedString>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub read_only: Prop<bool>,
    #[prop(default)]
    pub mono: Prop<bool>,
    #[prop(default)]
    pub multiline: Prop<bool>,
    #[prop(default = 3)]
    pub rows: Prop<u32>,
    #[prop(default)]
    pub masked: Prop<bool>,
    #[prop(default)]
    pub required: Prop<bool>,
    #[prop(default)]
    pub email: Prop<bool>,
    #[prop(default)]
    pub error: Prop<bool>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum TextFieldEvent {
    #[args(value)]
    Change(SharedString),
}

#[component]
pub fn TextField(p: TextFieldProps, cx: Scope<TextFieldEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let field = use_context::<super::form_field::FieldContext>();
    let invalid = signal(false);
    let error = move || p.error.get() || invalid.get() || field.is_some_and(|f| f.error.get());
    let tint = move || {
        if error() {
            Tint::Danger
        } else {
            field.map(|f| f.tint.get()).unwrap_or_else(|| p.tint.get())
        }
    };
    let local = signal(p.default_value.get_untracked());
    let editor = field.map(|f| f.editor).unwrap_or_else(element_ref);
    let form = use_context::<super::form::FormContext>();
    let validate = move || {
        let value = p
            .value
            .get_untracked()
            .unwrap_or_else(|| local.get_untracked());
        let bad = !p.disabled.get_untracked()
            && ((p.required.get_untracked() && value.trim().is_empty())
                || (p.email.get_untracked()
                    && !value.is_empty()
                    && !value.split_once('@').is_some_and(|(a, b)| {
                        !a.is_empty() && !b.is_empty() && !value.contains(char::is_whitespace)
                    })));
        invalid.set(bad);
        !bad
    };
    if let Some(form) = &form {
        form.register(super::form::FieldRegistration {
            validate: std::rc::Rc::new(validate),
            focus: editor,
            reset: std::rc::Rc::new(move || {
                invalid.set(false);
                local.set(p.default_value.get_untracked());
                cx.emit(TextFieldEvent::Change(p.default_value.get_untracked()));
            }),
        });
    }
    input()
        .element_ref(&editor)
        .value(move || p.value.get().unwrap_or_else(|| local.get()).to_string())
        .placeholder(move || p.placeholder.get().to_string())
        .disabled(p.disabled)
        .readonly(p.read_only)
        .multiline(p.multiline)
        .masked(p.masked)
        .optional_label(move || {
            p.accessibility_label
                .get()
                .or_else(|| field.map(|f| f.label.get()))
        })
        .hint(move || field.map(|f| f.hint.get()).unwrap_or_default())
        .placeholder_color(move || theme.tokens().color_content_faint.hsla())
        .style_with(move |s| {
            let t = theme.tokens();
            let tint = tint();
            let s = styles::field(s, t, p.size.get(), tint, p.disabled.get(), error());
            s.style_when(p.mono.get(), |s| s.font_family(Face::Mono.family()))
                .style_when(p.multiline.get(), |s| {
                    let line =
                        p.size.get().profile(t).content.size * t.text_field_multiline_leading;
                    s.h(px((p.rows.get().max(1) as f32 * line
                        + t.spacing_2_5 * 2.0
                        + t.stroke_hairline * 2.0)
                        .max(t.spacing_20)))
                        .py(px(t.spacing_2_5))
                        .rounded(px(t.radius_large))
                        .line_height(px(line))
                })
        })
        .focus(move |s| {
            s.style_with(move |s| styles::field_focus(s, theme.tokens(), tint(), error()))
        })
        .on_change(move |value: &str| {
            invalid.set(false);
            if p.value.get_untracked().is_none() {
                local.set(SharedString::from(value.to_owned()));
            }
            cx.emit(TextFieldEvent::Change(value.to_owned().into()));
        })
        .on_submit(move |_: &str| {
            if let Some(form) = &form {
                form.submit();
            } else {
                validate();
            }
        })
}
