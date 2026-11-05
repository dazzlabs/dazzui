use crate::{Size, Theme, Variant, styles, tokens::Face};
use nsh_core::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CodeProps {
    pub children: Prop<Children>,
}
#[component]
pub fn Code(p: CodeProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .style_with(move |s| {
            let t = theme.tokens();
            styles::typography(s, t.caption_small.in_face(Face::Mono))
                .h({
                    let metrics = nsh_core::element::text_metrics(
                        &format!("{}px {}", t.caption_small.size, Face::Mono.family()),
                        "x",
                    );
                    px((metrics.ascent.0 + metrics.descent.0).ceil() + 2.0 * t.spacing_0_5)
                })
                .px(px(t.spacing_1))
                .py(px(t.spacing_0_5))
                .rounded(px(t.radius_tiny))
                .bg(t.color_surface_sunken.hsla())
                .text_color(t.color_content.hsla())
        })
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct CodeBlockProps {
    pub code: Prop<SharedString>,
    pub title: Prop<Option<SharedString>>,
    pub language: Prop<Option<SharedString>>,
    #[prop(default)]
    pub copyable: Prop<bool>,
    #[prop(default)]
    pub line_numbers: Prop<bool>,
}
#[component]
pub fn CodeBlock(p: CodeBlockProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .overflow_hidden()
        .style_with(move |s| {
            let t = theme.tokens();
            s.rounded(px(t.control_container_radius))
                .border(px(t.stroke_hairline))
                .border_color(t.color_border.hsla())
                .bg(t.color_surface_muted.hsla())
        })
        .child(
            view()
                .flex_row()
                .items_center()
                .style_with(move |s| {
                    let t = theme.tokens();
                    s.gap(px(t.spacing_2))
                        .min_h(px(t.spacing_8))
                        .px(px(t.spacing_3))
                        .py(px(t.spacing_1))
                        .border_b(px(t.stroke_hairline))
                        .border_color(t.color_border.hsla())
                        .bg(t.color_surface_chrome.hsla())
                })
                .child(text(move || p.title.get().unwrap_or_default()))
                .child(text(move || {
                    p.language.get().unwrap_or_default().to_uppercase()
                }))
                .child(view().flex_1())
                .child(dynamic(move || {
                    p.copyable.get().then(|| {
                        crate::button()
                            .size(Size::Tiny)
                            .variant(Variant::Plain)
                            .child(text("Copy"))
                            .on_press(move || {
                                nsh_core::element::set_clipboard(p.code.get().to_string())
                            })
                    })
                })),
        )
        .child(
            nsh_core::element::scroll().horizontal().child(
                view()
                    .flex_row()
                    .style_with(move |s| {
                        let t = theme.tokens();
                        styles::typography(s, t.body_small.in_face(Face::Mono))
                            .py(px(t.spacing_2_5))
                            .px(px(t.spacing_3))
                    })
                    .child(dynamic(move || {
                        p.line_numbers.get().then(|| {
                            text(move || {
                                (1..=p.code.get().lines().count())
                                    .map(|n| n.to_string())
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            })
                            .min_w(move || px(theme.tokens().spacing_10))
                            .text_color(move || theme.tokens().color_content_faint.hsla())
                        })
                    }))
                    .child(text(p.code)),
            ),
        )
}
