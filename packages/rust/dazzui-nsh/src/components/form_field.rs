use crate::{Theme, Tint, styles};
use nsh_core::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct FieldContext {
    pub label: Prop<SharedString>,
    pub hint: Prop<SharedString>,
    pub error: Prop<bool>,
    pub tint: Prop<Tint>,
    pub editor: nsh_core::element::ElementRef,
}
#[derive(Clone, PartialEq, Props)]
pub struct FormFieldProps {
    pub label: Prop<SharedString>,
    #[prop(default)]
    pub hint: Prop<SharedString>,
    #[prop(default)]
    pub error: Prop<bool>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub children: Prop<Children>,
}
/// Labels and describes a TextField or Select. Clicking the label focuses its editor.
#[component]
pub fn FormField(p: FormFieldProps) -> impl IntoElement {
    let theme = Theme::current();
    let field = FieldContext {
        label: p.label,
        hint: p.hint,
        error: p.error,
        tint: p.tint,
        editor: element_ref(),
    };
    provide_context(field);
    let ink = move || {
        if p.error.get() {
            Tint::Danger.ramp(theme.tokens()).shade(700).hsla()
        } else {
            theme.tokens().color_content_subtle.hsla()
        }
    };
    view()
        .flex_col()
        .gap(move || px(theme.tokens().spacing_1_5))
        .child(
            view()
                .self_start()
                .on_press(move || field.editor.focus())
                .child(text(p.label).style_with(move |s| {
                    styles::typography(s, theme.tokens().label_small).text_color(ink())
                })),
        )
        .child(p.children)
        .child(when(
            move || !p.hint.get().is_empty(),
            move || {
                styles::paragraph(
                    move || p.hint.get(),
                    move || {
                        let t = theme.tokens();
                        crate::tokens::TypeStyle {
                            line_height: t.label_quiet.size * t.form_field_hint_leading,
                            ..t.label_quiet
                        }
                    },
                )
                .text_color(ink)
            },
        ))
}
