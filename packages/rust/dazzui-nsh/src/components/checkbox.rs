use crate::{Part, Size, State, Theme, Tint, Variant, axes::recipe, styles};
use nsh_core::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
    pub checked: Prop<Option<bool>>,
    #[prop(default)]
    pub default_checked: Prop<bool>,
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
    pub note: Prop<Option<Slot>>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum CheckboxEvent {
    #[args(checked)]
    Change(bool),
}

#[component]
pub fn Checkbox(p: CheckboxProps, cx: Scope<CheckboxEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let local = signal(p.default_checked.get_untracked());
    let checked = move || p.checked.get().unwrap_or_else(|| local.get());
    let marked = move || checked() || p.indeterminate.get();
    nsh_base::checkbox()
        .checked(checked)
        .indeterminate(p.indeterminate)
        .disabled(p.disabled)
        .accessibility_label(p.accessibility_label)
        .flex_row()
        .items_center()
        .style_with(move |s| {
            let t = theme.tokens();
            styles::typography(
                s,
                if checked() {
                    t.label_medium
                } else {
                    t.label_quiet
                },
            )
            .gap(px(t.spacing_2_5))
            .text_color(if checked() {
                t.color_content.hsla()
            } else {
                t.color_content_muted.hsla()
            })
            .opacity(if p.disabled.get() { 0.6 } else { 1.0 })
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .indicator_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let size = match p.size.get() {
                    Size::Tiny | Size::Small => t.checkbox_small_box,
                    Size::Medium => t.checkbox_medium_box,
                    Size::Large => t.checkbox_large_box,
                };
                let color = recipe(t, Variant::Filled, Part::Surface, State::Normal)
                    .resolve(p.tint.get().ramp(t));
                s.size(px(size))
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .rounded(px(t.checkbox_radius))
                    .border(px(t.checkbox_thickness))
                    .border_color(if marked() {
                        color
                    } else {
                        t.color_border_muted.hsla()
                    })
                    .bg(if marked() {
                        color
                    } else {
                        crate::tokens::Color::TRANSPARENT.hsla()
                    })
                    .text_color(
                        recipe(t, Variant::Filled, Part::Content, State::Normal)
                            .resolve(p.tint.get().ramp(t)),
                    )
            })
        })
        .indicator(dynamic(move || {
            crate::icon(if p.indeterminate.get() {
                "Subtract12Filled"
            } else {
                "Checkmark12Filled"
            })
            .size(move || px(theme.tokens().icon_small))
            .opacity(if marked() { 1.0 } else { 0.0 })
        }))
        .on_change(move |value: &bool| {
            if p.checked.get_untracked().is_none() {
                local.set(*value);
            }
            cx.emit(CheckboxEvent::Change(*value));
        })
        .child(
            view()
                .flex_row()
                .child(p.children)
                .child(p.note.place(move |slot| {
                    slot.ml(move || {
                        measure_text(
                            &format!(
                                "{} {}px {}",
                                theme.tokens().label_medium.weight,
                                theme.tokens().label_medium.size,
                                theme.tokens().label_medium.face.family()
                            ),
                            " ",
                        )
                    })
                    .text_color(move || theme.tokens().color_content_faint.hsla())
                })),
        )
}
