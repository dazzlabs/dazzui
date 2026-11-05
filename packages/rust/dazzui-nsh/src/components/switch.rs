use crate::{Size, State, Theme, Tint, Variant, axes::ControlColors, styles};
use nsh_core::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SwitchProps {
    pub checked: Prop<Option<bool>>,
    #[prop(default)]
    pub default_checked: Prop<bool>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum SwitchEvent {
    #[args(checked)]
    Change(bool),
}

#[component]
pub fn Switch(p: SwitchProps, cx: Scope<SwitchEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let local = signal(p.default_checked.get_untracked());
    let checked = move || p.checked.get().unwrap_or_else(|| local.get());
    nsh_base::switch()
        .checked(checked)
        .disabled(p.disabled)
        .accessibility_label(p.accessibility_label)
        .flex_row()
        .items_center()
        .style_with(move |s| {
            s.gap(px(theme.tokens().spacing_2_5))
                .opacity(if p.disabled.get() { 0.6 } else { 1.0 })
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .track_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let (w, h, _) = dimensions(t, p.size.get());
                let c = ControlColors::resolve(t, Variant::Filled, p.tint.get(), State::Normal);
                s.w(px(w))
                    .h(px(h))
                    .p(px(t.spacing_0_5))
                    .rounded(px(t.radius_full))
                    .flex_row()
                    .items_center()
                    .style_when(checked(), |s| s.justify_end())
                    .style_when(!checked(), |s| s.justify_start())
                    .bg(if checked() {
                        c.surface
                    } else {
                        t.color_surface_sunken.hsla()
                    })
            })
        })
        .thumb_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let (_, _, thumb) = dimensions(t, p.size.get());
                let c = ControlColors::resolve(t, Variant::Filled, p.tint.get(), State::Normal);
                s.size(px(thumb))
                    .rounded(px(t.radius_full))
                    .bg(if checked() {
                        c.content
                    } else {
                        t.color_content_faint.hsla()
                    })
            })
        })
        .on_change(move |value: &bool| {
            if p.checked.get_untracked().is_none() {
                local.set(*value);
            }
            cx.emit(SwitchEvent::Change(*value));
        })
        .child(p.children)
}
fn dimensions(t: &crate::ThemeTokens, size: Size) -> (f32, f32, f32) {
    match size {
        Size::Tiny | Size::Small => (
            t.switch_small_width,
            t.switch_small_height,
            t.switch_small_thumb,
        ),
        Size::Medium => (
            t.switch_medium_width,
            t.switch_medium_height,
            t.switch_medium_thumb,
        ),
        Size::Large => (
            t.switch_large_width,
            t.switch_large_height,
            t.switch_large_thumb,
        ),
    }
}
