use crate::{Size, State, Theme, Tint, Variant, axes::ControlColors, styles};
use nsh_core::{element::ImageSrc, prelude::*};

#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum KeyCapVariant {
    #[default]
    Hint,
    Strong,
    Key,
}
#[derive(Clone, PartialEq, Props)]
pub struct KeyCapProps {
    pub value: Prop<Option<SharedString>>,
    #[prop(default)]
    pub variant: Prop<KeyCapVariant>,
    #[prop(default)]
    pub size: Prop<Size>,
    pub children: Prop<Children>,
}
#[component]
pub fn KeyCap(p: KeyCapProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .flex_row()
        .items_center()
        .style_with(move |s| {
            let t = theme.tokens();
            let face = if p.size.get() == Size::Large {
                t.label_medium
            } else {
                t.label_small
            };
            let s = styles::typography(s, face);
            match p.variant.get() {
                KeyCapVariant::Hint => s.text_color(t.color_content_faint.hsla()),
                KeyCapVariant::Strong => s.text_color(t.color_content_subtle.hsla()),
                KeyCapVariant::Key => s
                    .justify_center()
                    .h(px(t.spacing_5 + t.spacing_0_5))
                    .px(px(t.spacing_2_5))
                    .rounded(px(t.radius_tiny))
                    .bg(t.color_surface_inset.hsla())
                    .text_color(t.color_content.hsla())
                    .font_weight(t.label_strong.font_weight()),
            }
        })
        .child(when(
            move || p.value.get().is_some(),
            move || {
                styles::label(
                    move || p.value.get().unwrap_or_default(),
                    move || {
                        let t = theme.tokens();
                        let mut face = if p.size.get() == Size::Large {
                            t.label_medium
                        } else {
                            t.label_small
                        };
                        if p.variant.get() == KeyCapVariant::Key {
                            face.weight = t.label_strong.weight;
                        }
                        face
                    },
                )
            },
        ))
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct SectionLabelProps {
    #[prop(default)]
    pub faint: Prop<bool>,
    pub tint: Prop<Option<Tint>>,
    pub children: Prop<Children>,
}
#[component]
pub fn SectionLabel(p: SectionLabelProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .style_with(move |s| {
            let t = theme.tokens();
            styles::typography(s, t.label_small).text_color(
                p.tint
                    .get()
                    .map(|tint| t.control_color_plain_content_normal.resolve(tint.ramp(t)))
                    .unwrap_or_else(|| {
                        if p.faint.get() {
                            t.color_content_faint.hsla()
                        } else {
                            t.color_content_subtle.hsla()
                        }
                    }),
            )
        })
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct EmptyStateProps {
    pub title: Prop<SharedString>,
    pub action: Prop<Option<Slot>>,
    pub children: Prop<Children>,
}
#[component]
pub fn EmptyState(p: EmptyStateProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .items_center()
        .justify_center()
        .flex_grow(1.0)
        .flex_shrink_0()
        .style_with(move |s| {
            let t = theme.tokens();
            styles::typography(s, t.body_medium)
                .py(px(t.spacing_14))
                .px(px(t.spacing_8))
                .gap(px(t.spacing_3))
                .text_color(t.color_content_subtle.hsla())
        })
        .child(text(p.title))
        .child(p.children)
        .child(p.action.place(move |slot| {
            slot.flex_row()
                .gap(move || px(theme.tokens().control_medium_gap))
        }))
}
#[derive(Clone, PartialEq, Props)]
pub struct AvatarProps {
    pub name: Prop<SharedString>,
    pub src: Prop<Option<ImageSrc>>,
    pub fallback: Prop<Option<Slot>>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub square: Prop<bool>,
}
#[component]
pub fn Avatar(p: AvatarProps) -> impl IntoElement {
    let theme = Theme::current();
    nsh_base::avatar()
        .name(move || Some(p.name.get()))
        .src(p.src)
        .fallback(p.fallback.place(move |slot| slot))
        .style_with(move |s| {
            let t = theme.tokens();
            let size = match p.size.get() {
                Size::Tiny | Size::Small => t.spacing_5,
                Size::Medium => t.spacing_7,
                Size::Large => t.spacing_9,
            };
            s.size(px(size))
                .overflow_hidden()
                .flex_shrink_0()
                .rounded(px(if p.square.get() {
                    t.radius_medium
                } else {
                    t.radius_full
                }))
        })
        .image_style(|s| s.size_full())
        .fallback_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let face = match p.size.get() {
                    Size::Tiny | Size::Small => t.caption_small,
                    Size::Medium => t.label_medium,
                    Size::Large => t.body_medium,
                };
                let c = ControlColors::resolve(t, Variant::Tinted, p.tint.get(), State::Normal);
                styles::typography(s, face)
                    .font_weight(t.label_strong.font_weight())
                    .size_full()
                    .items_center()
                    .justify_center()
                    .bg(c.surface)
                    .text_color(c.content)
            })
        })
}
#[derive(Clone, PartialEq, Props)]
pub struct CalloutProps {
    pub title: Prop<SharedString>,
    pub icon: Prop<Option<Slot>>,
    pub action: Prop<Option<Slot>>,
    pub children: Prop<Children>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
}
#[component]
pub fn Callout(p: CalloutProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .flex_row()
        .items_start()
        .style_with(move |s| {
            let t = theme.tokens();
            let color = p.tint.get().ramp(t).shade(600);
            let (x, y, radius) = match p.size.get() {
                Size::Tiny | Size::Small => (t.spacing_3, t.spacing_2, t.control_container_radius),
                Size::Medium => (t.spacing_3_5, t.spacing_3, t.radius_large),
                Size::Large => (t.spacing_4, t.spacing_4, t.radius_large),
            };
            s.px(px(x))
                .py(px(y))
                .gap(px(t.spacing_2_5))
                .rounded(px(radius))
                .border(px(t.stroke_hairline))
                .border_color(color.alpha(t.wash_edge))
                .bg(color.alpha(t.wash_surface))
        })
        .child(p.icon.place(move |slot| {
            slot.flex_shrink_0()
                .text_color(move || p.tint.get().ramp(theme.tokens()).shade(600).hsla())
        }))
        .child(
            view()
                .flex_1()
                .gap(move || px(theme.tokens().spacing_1))
                .child(text(p.title).style_with(move |s| {
                    let t = theme.tokens();
                    styles::typography(s, t.label_medium)
                        .line_height(px(t.label_medium.size * t.callout_title_leading))
                        .text_color(
                            t.control_color_tinted_content_hovered
                                .resolve(p.tint.get().ramp(t)),
                        )
                }))
                .child(
                    view()
                        .style_with(move |s| {
                            let t = theme.tokens();
                            styles::typography(s, t.label_quiet)
                                .line_height(px(t.label_quiet.size * t.callout_description_leading))
                                .font_weight(t.caption_small.font_weight())
                                .text_color(
                                    t.control_color_tinted_content_normal
                                        .resolve(p.tint.get().ramp(t)),
                                )
                        })
                        .child(p.children),
                ),
        )
        .child(p.action.place(move |slot| slot.flex_shrink_0()))
}
