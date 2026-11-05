use crate::{Size, Theme, Tint, styles};
use nsh_core::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RadioProps {
    #[prop(default)]
    pub checked: Prop<bool>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub children: Prop<Children>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum RadioEvent {
    Change,
}
#[component]
pub fn Radio(p: RadioProps, cx: Scope<RadioEvent>) -> impl IntoElement {
    let theme = Theme::current();
    nsh_base::radio()
        .checked(p.checked)
        .disabled(p.disabled)
        .accessibility_label(p.accessibility_label)
        .flex_row()
        .items_center()
        .gap(move || px(theme.tokens().spacing_2_5))
        .style_with(move |s| {
            let t = theme.tokens();
            styles::typography(
                s,
                if p.checked.get() {
                    t.label_medium
                } else {
                    t.label_quiet
                },
            )
            .text_color(t.color_content.hsla())
            .opacity(if p.disabled.get() { 0.6 } else { 1.0 })
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .on_change(move |_: &bool| cx.emit(RadioEvent::Change))
        .child(
            view()
                .items_center()
                .justify_center()
                .style_with(move |s| {
                    let t = theme.tokens();
                    let size = match p.size.get() {
                        Size::Tiny | Size::Small => t.checkbox_small_box,
                        Size::Medium => t.checkbox_medium_box,
                        Size::Large => t.checkbox_large_box,
                    };
                    s.size(px(size))
                        .rounded(px(t.radius_full))
                        .border(px(t.checkbox_thickness))
                        .border_color(if p.checked.get() {
                            t.control_color_filled_surface_normal
                                .resolve(p.tint.get().ramp(t))
                        } else {
                            t.color_border_muted.hsla()
                        })
                })
                .child(view().style_with(move |s| {
                    let t = theme.tokens();
                    let size = match p.size.get() {
                        Size::Tiny | Size::Small => t.radio_small_dot,
                        Size::Medium => t.radio_medium_dot,
                        Size::Large => t.radio_large_dot,
                    };
                    s.size(px(size))
                        .rounded(px(t.radius_full))
                        .bg(t
                            .control_color_filled_surface_normal
                            .resolve(p.tint.get().ramp(t)))
                        .opacity(if p.checked.get() { 1.0 } else { 0.0 })
                })),
        )
        .child(p.children)
}
