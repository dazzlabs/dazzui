use crate::{Size, Theme, Tint};
use nsh_core::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ProgressProps {
    #[prop(default)]
    pub value: Prop<f32>,
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    #[prop(default)]
    pub gradient: Prop<bool>,
    #[prop(default = Size::Small)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[component]
pub fn Progress(p: ProgressProps) -> impl IntoElement {
    let theme = Theme::current();
    nsh_base::progress()
        .value(p.value)
        .indeterminate(p.indeterminate)
        .accessibility_label(p.accessibility_label)
        .w_full()
        .track_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let h = match p.size.get() {
                    Size::Tiny | Size::Small => t.spacing_1,
                    Size::Medium => t.spacing_1_5,
                    Size::Large => t.spacing_2,
                };
                s.h(px(h))
                    .rounded(px(h / 2.0))
                    .overflow_hidden()
                    .bg(t.color_surface_sunken.hsla())
            })
        })
        .indicator_style(move |s| {
            s.h_full()
                .bg(move || p.tint.get().ramp(theme.tokens()).shade(600).hsla())
                .style_with(move |s| {
                    s.style_when(p.gradient.get(), |s| {
                        s.bg_linear_to_r()
                            .gradient_from(theme.tokens().progress_gradient_from.hsla())
                            .gradient_to(theme.tokens().progress_gradient_to.hsla())
                    })
                })
        })
}
