use crate::{Size, Theme, Tint, styles, tokens::Face};
use nsh_core::{
    animation::Easing,
    element::{Animation, Timing},
    prelude::*,
};

#[derive(Clone, PartialEq, Props)]
pub struct SpinnerProps {
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub on_accent: Prop<bool>,
}
#[component]
pub fn Spinner(p: SpinnerProps) -> impl IntoElement {
    let theme = Theme::current();
    let rotation = nsh_core::element::animated_value(0.0);
    let animation = Animation::timing(
        &rotation,
        std::f32::consts::TAU,
        Timing::new(
            if nsh_core::element::reduce_motion() {
                theme.tokens().spinner_reduced_duration
            } else {
                theme.tokens().spinner_duration
            },
            Easing::Linear,
        ),
    )
    .looped(None);
    animation.start();
    cleanup(move || animation.stop());
    view()
        .role(AccessibilityRole::Progressbar)
        .label("Loading")
        .bind_rotate(&rotation)
        .style_with(move |s| {
            let t = theme.tokens();
            let size = match p.size.get() {
                Size::Tiny | Size::Small => t.spacing_3_5,
                Size::Medium => t.spacing_4,
                Size::Large => t.spacing_5,
            };
            let color = if p.on_accent.get() {
                t.color_on_accent
            } else {
                p.tint.get().ramp(t).shade(600)
            };
            s.w(px(size))
                .h(px(size))
                .flex_shrink_0()
                .rounded(px(t.radius_full))
                .border(px(t.spacing_0_5))
                .border_color(color.alpha(if p.on_accent.get() {
                    t.spinner_on_accent_track_alpha
                } else {
                    t.spinner_track_alpha
                }))
                .border_t_color(color.hsla())
        })
}

#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum SkeletonShape {
    #[default]
    Text,
    Rect,
    Circle,
}
#[derive(Clone, PartialEq, Props)]
pub struct SkeletonProps {
    #[prop(default)]
    pub shape: Prop<SkeletonShape>,
    pub width: Prop<Option<f32>>,
    pub height: Prop<Option<f32>>,
}
#[component]
pub fn Skeleton(p: SkeletonProps) -> impl IntoElement {
    let theme = Theme::current();
    let width = signal(0.0);
    let translation = nsh_core::element::animated_value(0.0);
    effect(move || {
        let width = width.get();
        if width <= 0.0 || nsh_core::element::reduce_motion() {
            return;
        }
        translation.set(-width * 2.0);
        let animation = Animation::timing(
            &translation,
            width,
            Timing::new(theme.tokens().skeleton_duration, Easing::EaseInOut),
        )
        .looped(None);
        animation.start();
        cleanup(move || animation.stop());
    });
    view()
        .overflow_hidden()
        .on_layout(move |bounds: &nsh_core::Bounds<nsh_core::Points>| {
            width.set(bounds.size.width.0)
        })
        .style_with(move |s| {
            let t = theme.tokens();
            let mut s = s
                .w_full()
                .h(px(p.height.get().unwrap_or(t.body_small.line_height)))
                .bg(t.color_surface_sunken.hsla())
                .rounded(px(match p.shape.get() {
                    SkeletonShape::Text => t.radius_tiny,
                    SkeletonShape::Rect => t.radius_small,
                    SkeletonShape::Circle => t.radius_full,
                }));
            if let Some(w) = p.width.get() {
                s = s.w(px(w));
            }
            if p.shape.get() == SkeletonShape::Circle {
                s = s.h(px(p
                    .height
                    .get()
                    .or_else(|| p.width.get())
                    .unwrap_or(t.body_small.line_height)));
            }
            s
        })
        .child(when(
            move || !nsh_core::element::reduce_motion(),
            move || {
                view()
                    .absolute()
                    .top(px(0.0))
                    .bottom(px(0.0))
                    .w(relative(2.0))
                    .bind_translate_x(&translation)
                    .bg_linear_to_r()
                    .gradient_from(move || theme.tokens().color_surface_subtle.alpha(0.0))
                    .gradient_via(move || theme.tokens().color_surface_subtle.hsla())
                    .gradient_to(move || theme.tokens().color_surface_subtle.alpha(0.0))
            },
        ))
}
#[derive(Clone, PartialEq, Props)]
pub struct SkeletonTextProps {
    #[prop(default = 3)]
    pub lines: Prop<u32>,
}
#[component]
pub fn SkeletonText(p: SkeletonTextProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .w_full()
        .gap(move || px(theme.tokens().spacing_1_5))
        .child(dynamic(move || {
            (0..p.lines.get())
                .map(|i| {
                    skeleton().style_with(move |s| {
                        if i + 1 == p.lines.get() {
                            s.w(relative(theme.tokens().skeleton_last_line_width))
                        } else {
                            s
                        }
                    })
                })
                .collect::<Vec<_>>()
        }))
}

#[derive(Clone, PartialEq, Props)]
pub struct MeterProps {
    #[prop(default)]
    pub value: Prop<f32>,
    #[prop(default)]
    pub show_value: Prop<bool>,
    pub label: Prop<Option<SharedString>>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[component]
pub fn Meter(p: MeterProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .w_full()
        .gap(move || px(theme.tokens().spacing_2))
        .child(dynamic(move || {
            (p.label.get().is_some() || p.show_value.get()).then(|| {
                view()
                    .flex_row()
                    .justify_between()
                    .items_start()
                    .style_with(move |s| styles::typography(s, theme.tokens().label_small))
                    .child(
                        text(move || p.label.get().unwrap_or_default()).style_with(move |s| {
                            let t = theme.tokens();
                            let value = crate::tokens::TypeStyle {
                                face: Face::Mono,
                                weight: t.body_small.weight,
                                ..t.label_small
                            };
                            s.h(px(t.label_small.line_height)).mt(px(
                                if p.show_value.get() && p.label.get().is_some() {
                                    (styles::baseline(value) - styles::baseline(t.label_small))
                                        .max(0.0)
                                } else {
                                    0.0
                                },
                            ))
                        }),
                    )
                    .child(
                        text(move || {
                            if p.show_value.get() {
                                format!("{}%", p.value.get().clamp(0.0, 100.0).round())
                            } else {
                                String::new()
                            }
                        })
                        .font_family(Face::Mono.family())
                        .style_with(move |s| {
                            let t = theme.tokens();
                            let value = crate::tokens::TypeStyle {
                                face: Face::Mono,
                                weight: t.body_small.weight,
                                ..t.label_small
                            };
                            s.font_weight(value.font_weight())
                                .h(px(value.line_height))
                                .mt(px(if p.show_value.get() && p.label.get().is_some() {
                                    (styles::baseline(t.label_small) - styles::baseline(value))
                                        .max(0.0)
                                } else {
                                    0.0
                                }))
                        })
                        .text_color(move || theme.tokens().color_content_muted.hsla()),
                    )
            })
        }))
        .child(
            crate::progress()
                .value(p.value)
                .size(p.size)
                .tint(p.tint)
                .accessibility_label(p.accessibility_label),
        )
}

#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum StepStatus {
    Done,
    Active,
    #[default]
    Idle,
}
#[derive(Clone, PartialEq, Props)]
pub struct StepProps {
    pub label: Prop<SharedString>,
    pub meta: Prop<Option<SharedString>>,
    #[prop(default)]
    pub status: Prop<StepStatus>,
}
pub fn step_list() -> nsh_core::element::View {
    let theme = Theme::current();
    view()
        .role(AccessibilityRole::List)
        .gap(move || px(theme.tokens().spacing_2))
}
#[component]
pub fn Step(p: StepProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .role(AccessibilityRole::Group)
        .flex_row()
        .items_center()
        .style_with(move |s| {
            let t = theme.tokens();
            styles::typography(
                s,
                if p.status.get() == StepStatus::Active {
                    t.label_medium
                } else {
                    t.label_quiet
                },
            )
            .gap(px(t.spacing_2_5))
            .text_color(
                match p.status.get() {
                    StepStatus::Done => t.color_content_muted,
                    StepStatus::Active => t.color_content,
                    StepStatus::Idle => t.color_content_subtle,
                }
                .hsla(),
            )
        })
        .child(
            view()
                .size(move || px(theme.tokens().icon_medium))
                .items_center()
                .justify_center()
                .child(dynamic(move || match p.status.get() {
                    StepStatus::Active => spinner().size(Size::Small).into_element(),
                    StepStatus::Done => crate::icon("Checkmark12Filled")
                        .size(move || px(theme.tokens().icon_small))
                        .text_color(move || theme.tokens().color_success.shade(600).hsla())
                        .into_element(),
                    StepStatus::Idle => view()
                        .w(move || px(theme.tokens().icon_medium))
                        .h(move || px(theme.tokens().icon_medium))
                        .rounded(move || px(theme.tokens().radius_full))
                        .border(move || px(theme.tokens().stroke_control))
                        .border_color(move || theme.tokens().color_border_muted.hsla())
                        .into_element(),
                })),
        )
        .child(styles::label(
            move || p.label.get(),
            move || {
                if p.status.get() == StepStatus::Active {
                    theme.tokens().label_medium
                } else {
                    theme.tokens().label_quiet
                }
            },
        ))
        .child(when(
            move || p.meta.get().is_some(),
            move || {
                text(move || p.meta.get().unwrap_or_default())
                    .ml(nsh_core::geometry::Length::Auto)
                    .style_with(move |s| {
                        let t = theme.tokens();
                        styles::typography(s, t.caption_small)
                            .line_height(px(t.label_small.size))
                            .text_color(if p.status.get() == StepStatus::Active {
                                t.control_color_plain_content_normal
                                    .resolve(Tint::Primary.ramp(t))
                            } else {
                                t.color_content_faint.hsla()
                            })
                    })
            },
        ))
}
