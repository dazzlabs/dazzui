use crate::{State, Theme, Tint, Variant, styles};
use nsh_core::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct OptionCardProps {
    pub title: Prop<SharedString>,
    pub description: Prop<Option<SharedString>>,
    #[prop(default)]
    pub selected: Prop<bool>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum OptionCardEvent {
    Press,
}
#[component]
pub fn OptionCard(p: OptionCardProps, cx: Scope<OptionCardEvent>) -> impl IntoElement {
    let theme = Theme::current();
    nsh_base::radio()
        .checked(p.selected)
        .disabled(p.disabled)
        .style_with(move |s| {
            let t = theme.tokens();
            let s = s
                .flex_1()
                .flex_col()
                .items_start()
                .gap(px(t.spacing_1_5))
                .py(px(t.spacing_3_5))
                .px(px(t.spacing_3))
                .rounded(px(t.radius_large))
                .border(px(t.stroke_hairline))
                .border_color(t.color_border.hsla())
                .bg(t.color_surface_muted.hsla());
            if p.disabled.get() {
                styles::colors(s, t, Variant::Normal, p.tint.get(), State::Disabled)
            } else if p.selected.get() {
                s.bg(p.tint.get().ramp(t).shade(600).alpha(t.wash_surface))
                    .border_color(p.tint.get().ramp(t).shade(600).hsla())
            } else {
                s
            }
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .on_change(move |_: &bool| cx.emit(OptionCardEvent::Press))
        .child(
            text(p.title).style_with(move |s| styles::typography(s, theme.tokens().label_strong)),
        )
        .child(
            styles::paragraph(
                move || p.description.get().unwrap_or_default(),
                move || {
                    let t = theme.tokens();
                    crate::tokens::TypeStyle {
                        line_height: t.label_small.size * t.option_card_description_leading,
                        ..t.caption_small
                    }
                },
            )
            .text_color(move || theme.tokens().color_content_subtle.hsla()),
        )
        .child(p.children)
}
