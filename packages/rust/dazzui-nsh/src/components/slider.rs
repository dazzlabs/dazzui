use crate::{Size, Theme, Tint};
pub use nsh_base::SliderValue;
use nsh_core::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct SliderProps {
    #[prop(default)]
    pub show_value: Prop<bool>,
    pub value: Prop<Option<SliderValue>>,
    #[prop(default)]
    pub default_value: Prop<SliderValue>,
    #[prop(default)]
    pub min: Prop<f64>,
    #[prop(default = 100.0)]
    pub max: Prop<f64>,
    #[prop(default = 1.0)]
    pub step: Prop<f64>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum SliderEvent {
    #[args(value)]
    Change(SliderValue),
    #[args(value)]
    ChangeEnd(SliderValue),
}
#[component]
pub fn Slider(p: SliderProps, cx: Scope<SliderEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let end = cx;
    let own = signal(p.default_value.get_untracked());
    nsh_base::slider()
        .value(p.value)
        .default_value(p.default_value)
        .min(p.min)
        .max(p.max)
        .step(p.step)
        .disabled(p.disabled)
        .accessibility_label(p.accessibility_label)
        .w_full()
        .flex_row()
        .items_center()
        .gap(move || px(theme.tokens().spacing_3))
        .track_style(move |s| {
            s.flex_1()
                .min_w(px(0.0))
                .justify_center()
                .h(move || px(p.size.get().profile(theme.tokens()).size))
        })
        .indicator_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let (track, _) = dimensions(t, p.size.get());
                s.h(px(track))
                    .rounded(px(track / 2.0))
                    .bg(t.color_surface_sunken.hsla())
            })
        })
        .range_style(move |s| {
            s.h_full()
                .bg(move || p.tint.get().ramp(theme.tokens()).shade(600).hsla())
        })
        .thumb_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let (_, thumb) = dimensions(t, p.size.get());
                s.size(px(thumb))
                    .rounded(px(t.radius_full))
                    .bg(t
                        .color_surface
                        .hsla()
                        .mix(t.color_content.hsla(), t.slider_thumb_ink))
                    .border(px(t.slider_thumb_edge_width))
                    .border_color(
                        t.color_border_strong
                            .hsla()
                            .mix(t.color_surface.hsla(), t.slider_thumb_ink),
                    )
            })
        })
        .on_change(move |value: &SliderValue| {
            own.set(*value);
            cx.emit(SliderEvent::Change(*value));
        })
        .on_change_end(move |value: &SliderValue| end.emit(SliderEvent::ChangeEnd(*value)))
        .child(p.children)
        .child(dynamic(move || {
            p.show_value.get().then(|| {
                text(move || match p.value.get().unwrap_or_else(|| own.get()) {
                    SliderValue::Single(n) => format!("{n}"),
                    SliderValue::Range(a, b) => format!("{a} – {b}"),
                })
                .min_w(move || px(theme.tokens().spacing_10))
                .style_with(move |s| {
                    crate::styles::typography(s, theme.tokens().label_small)
                        .text_color(theme.tokens().color_content_muted.hsla())
                })
                .font_family(crate::tokens::Face::Mono.family())
            })
        }))
}
fn dimensions(t: &crate::ThemeTokens, size: Size) -> (f32, f32) {
    match size {
        Size::Tiny | Size::Small => (t.slider_small_track, t.slider_small_thumb),
        Size::Medium => (t.slider_medium_track, t.slider_medium_thumb),
        Size::Large => (t.slider_large_track, t.slider_large_thumb),
    }
}
