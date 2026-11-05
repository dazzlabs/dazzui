use crate::{Size, State, Theme, Tint, Variant, axes::ControlColors, styles};
use nsh_core::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum CardVariant {
    Raised,
    #[default]
    Sunken,
    Outlined,
    Tinted,
}
#[derive(Clone, PartialEq, Props)]
pub struct CardProps {
    #[prop(default)]
    pub variant: Prop<CardVariant>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub children: Prop<Children>,
}
#[component]
pub fn Card(p: CardProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .flex_col()
        .style_with(move |s| {
            let t = theme.tokens();
            let (x, y, gap) = match p.size.get() {
                Size::Tiny | Size::Small => (t.spacing_3, t.spacing_3, t.spacing_2),
                Size::Medium => (t.spacing_3_5, t.spacing_3, t.spacing_3),
                Size::Large => (t.spacing_4, t.spacing_4, t.spacing_4),
            };
            let c = p.tint.get().ramp(t).shade(600);
            let (fill, edge) = match p.variant.get() {
                CardVariant::Raised => (t.color_surface.hsla(), t.color_border.hsla()),
                CardVariant::Sunken => (t.color_surface_muted.hsla(), t.color_border.hsla()),
                CardVariant::Outlined => (
                    crate::tokens::Color::TRANSPARENT.hsla(),
                    t.color_border.hsla(),
                ),
                CardVariant::Tinted => (c.alpha(t.wash_surface), c.alpha(t.wash_edge)),
            };
            styles::typography(s, t.body_medium)
                .px(px(x))
                .py(px(y))
                .gap(px(gap))
                .rounded(px(t.radius_large))
                .border(px(t.stroke_hairline))
                .border_color(edge)
                .bg(fill)
                .text_color(t.color_content.hsla())
        })
        .child(p.children)
}

#[derive(Clone, PartialEq, Props)]
pub struct BadgeProps {
    #[prop(default)]
    pub raised: Prop<bool>,
    #[prop(default = Variant::Tinted)]
    pub variant: Prop<Variant>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub children: Prop<Children>,
}
#[component]
pub fn Badge(p: BadgeProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .flex_row()
        .items_center()
        .justify_center()
        .style_with(move |s| {
            let t = theme.tokens();
            let (x, y, face) = match p.size.get() {
                Size::Tiny | Size::Small => (t.spacing_1_5, t.spacing_0_5, t.label_small),
                Size::Medium => (t.spacing_2, t.spacing_0_5, t.label_small),
                Size::Large => (t.spacing_2, t.spacing_1, t.label_medium),
            };
            let c = ControlColors::resolve(t, p.variant.get(), p.tint.get(), State::Normal);
            let s = styles::typography(s, face)
                .line_height(px(face.size * t.badge_line_height))
                .h(px(
                    face.size * t.badge_line_height + 2.0 * (y + t.stroke_hairline)
                ))
                .font_weight(t.label_medium.font_weight())
                .px(px(x))
                .py(px(y))
                .rounded(px(t.radius_tiny))
                .border(px(t.stroke_hairline))
                .border_color(c.border)
                .bg(c.surface)
                .text_color(c.content);
            if p.raised.get() {
                s.bg(t.color_surface.hsla())
                    .text_color(t.color_content_muted.hsla())
            } else {
                s
            }
        })
        .child(p.children)
}

#[derive(Clone, PartialEq, Props)]
pub struct DividerProps {
    #[prop(default)]
    pub vertical: Prop<bool>,
    #[prop(default)]
    pub tone: Prop<DividerTone>,
}
#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum DividerTone {
    #[default]
    Normal,
    Muted,
    Strong,
}
#[component]
pub fn Divider(p: DividerProps) -> impl IntoElement {
    let theme = Theme::current();
    view().style_with(move |s| {
        let t = theme.tokens();
        s.bg(match p.tone.get() {
            DividerTone::Normal => t.color_border,
            DividerTone::Muted => t.color_border_muted,
            DividerTone::Strong => t.color_border_strong,
        }
        .hsla())
            .flex_shrink_0()
            .style_when(p.vertical.get(), |s| s.w(px(t.stroke_hairline)).h_full())
            .style_when(!p.vertical.get(), |s| s.h(px(t.stroke_hairline)).w_full())
    })
}
