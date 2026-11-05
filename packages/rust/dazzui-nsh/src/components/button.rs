use crate::{Size, State, Theme, Tint, Variant, styles};
use nsh_core::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    #[prop(default)]
    pub action: Prop<ButtonAction>,
    #[prop(default)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub variant: Prop<Variant>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub full_width: Prop<bool>,
    #[prop(default)]
    pub icon_only: Prop<bool>,
    #[prop(default)]
    pub leading_icon: Prop<bool>,
    #[prop(default)]
    pub trailing_icon: Prop<bool>,
    pub accessibility_label: Prop<Option<SharedString>>,
    pub shortcut: Prop<Option<Slot>>,
    /// A single-line label with exact intrinsic font advances.
    pub label: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}

#[derive(Clone, PartialEq, Event)]
pub enum ButtonEvent {
    Press,
}
#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum ButtonAction {
    #[default]
    Button,
    Submit,
    Reset,
}

/// DazzUI geometry and recipes over NSH's keyboard and accessible button.
#[component]
pub fn Button(p: ButtonProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    let theme = Theme::current();
    provide_context(super::icon::IconSize(std::rc::Rc::new(move || {
        let t = theme.tokens();
        if p.icon_only.get() && p.size.get() == Size::Tiny {
            t.icon_button_tiny_icon_size
        } else {
            p.size.get().profile(t).icon_size
                - if p.icon_only.get() {
                    0.0
                } else {
                    t.control_label_glyph_shrink
                }
        }
    })));
    let form = use_context::<super::form::FormContext>();
    nsh_base::button()
        .disabled(p.disabled)
        .accessibility_label(p.accessibility_label)
        .flex_row()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .style_with(move |s| {
            styles::control(
                s,
                theme.tokens(),
                p.size.get(),
                p.variant.get(),
                p.tint.get(),
                p.disabled.get(),
            )
            .style_when(p.leading_icon.get(), |s| {
                s.pl(px(p.size.get().profile(theme.tokens()).padding_inline
                    - theme.tokens().control_label_glyph_inset))
            })
            .style_when(p.trailing_icon.get(), |s| {
                s.pr(px(p.size.get().profile(theme.tokens()).padding_inline
                    - theme.tokens().control_label_glyph_inset))
            })
            .style_when(p.icon_only.get(), |s| {
                s.w(px(p.size.get().profile(theme.tokens()).size))
                    .p(px(0.0))
            })
        })
        .hover(move |s| {
            s.style_with(move |s| {
                styles::colors(
                    s,
                    theme.tokens(),
                    p.variant.get(),
                    p.tint.get(),
                    State::Hovered,
                )
            })
        })
        .active(move |s| {
            s.style_with(move |s| {
                styles::colors(
                    s,
                    theme.tokens(),
                    p.variant.get(),
                    p.tint.get(),
                    State::Pressed,
                )
            })
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .style_when(p.full_width, |s| s.w_full())
        .on_press(move || {
            cx.emit(ButtonEvent::Press);
            if let Some(form) = &form {
                match p.action.get() {
                    ButtonAction::Submit => form.submit(),
                    ButtonAction::Reset => form.reset(),
                    ButtonAction::Button => {}
                }
            }
        })
        .child(when(
            move || p.label.get().is_some(),
            move || {
                styles::label(
                    move || p.label.get().unwrap_or_default(),
                    move || {
                        let t = theme.tokens();
                        let mut face = p.size.get().profile(t).content;
                        if p.variant.get() == Variant::Filled && !p.disabled.get() {
                            face.weight = t.label_strong.weight;
                        }
                        face
                    },
                )
            },
        ))
        .child(p.children)
        .child(p.shortcut.place(move |slot| {
            slot.opacity(0.7)
                .font_family(theme.tokens().label_strong.face.family())
        }))
}
