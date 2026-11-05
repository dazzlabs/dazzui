use crate::{Size, State, Theme, Tint, Variant, styles};
use nsh_core::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ToggleProps {
    pub pressed: Prop<Option<bool>>,
    #[prop(default)]
    pub default_pressed: Prop<bool>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default = Size::Tiny)]
    pub size: Prop<Size>,
    #[prop(default = Tint::Neutral)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub icon_only: Prop<bool>,
    #[prop(default=Variant::Plain)]
    pub variant: Prop<Variant>,
    pub accessibility_label: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum ToggleEvent {
    #[args(pressed)]
    Change(bool),
}
#[component]
pub fn Toggle(p: ToggleProps, cx: Scope<ToggleEvent>) -> impl IntoElement {
    let theme = Theme::current();
    provide_context(super::icon::IconSize(std::rc::Rc::new(move || {
        p.size.get().profile(theme.tokens()).icon_size
    })));
    let own = signal(p.default_pressed.get_untracked());
    let pressed = move || p.pressed.get().unwrap_or_else(|| own.get());
    let variant = move || {
        if pressed() {
            Variant::Tinted
        } else {
            p.variant.get()
        }
    };
    nsh_base::toggle()
        .pressed(pressed)
        .disabled(p.disabled)
        .accessibility_label(p.accessibility_label)
        .flex_row()
        .items_center()
        .justify_center()
        .style_with(move |s| {
            styles::control(
                s,
                theme.tokens(),
                p.size.get(),
                variant(),
                p.tint.get(),
                p.disabled.get(),
            )
            .style_when(p.icon_only.get(), |s| {
                s.w(px(p.size.get().profile(theme.tokens()).size))
                    .px(px(0.0))
            })
            .style_when(pressed() && !p.disabled.get(), |s| {
                s.font_weight(theme.tokens().label_strong.font_weight())
            })
        })
        .hover(move |s| {
            s.style_with(move |s| {
                styles::colors(s, theme.tokens(), variant(), p.tint.get(), State::Hovered)
            })
        })
        .active(move |s| {
            s.style_with(move |s| {
                styles::colors(s, theme.tokens(), variant(), p.tint.get(), State::Pressed)
            })
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .on_change(move |next: &bool| {
            if p.pressed.get_untracked().is_none() {
                own.set(*next);
            }
            cx.emit(ToggleEvent::Change(*next));
        })
        .child(p.children)
}
