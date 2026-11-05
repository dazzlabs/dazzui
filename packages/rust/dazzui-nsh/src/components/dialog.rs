use crate::{Theme, Tint, styles, tokens::shadows};
use nsh_core::{element::Placement, prelude::*};
use std::rc::Rc;

#[derive(Clone)]
struct CloseContext(Rc<dyn Fn()>);
#[derive(Clone, PartialEq, Props)]
pub struct DialogProps {
    pub trigger: Prop<Option<Slot>>,
    pub open: Prop<Option<bool>>,
    #[prop(default)]
    pub default_open: Prop<bool>,
    pub width: Prop<Option<f32>>,
    #[prop(default)]
    pub danger: Prop<bool>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum DialogEvent {
    #[args(open)]
    OpenChange(bool),
}
#[component]
pub fn Dialog(p: DialogProps, cx: Scope<DialogEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let own = signal(p.default_open.get_untracked());
    let trigger = element_ref();
    let set = move |open| {
        if p.open.get_untracked().is_none() {
            own.set(open);
        }
        cx.emit(DialogEvent::OpenChange(open));
        if !open {
            trigger.focus();
        }
    };
    effect(move || {
        if p.open.get().unwrap_or_else(|| own.get()) {
            nsh_core::element::on_key_with(
                "escape",
                nsh_core::element::KeyOptions {
                    when: nsh_core::keymap::When::Always,
                    repeat: false,
                },
                move |_| set(false),
            );
        }
    });
    provide_context(CloseContext(Rc::new(move || set(false))));
    view()
        .child(p.trigger.get_untracked().map(|s| {
            s.element()
                .element_ref(&trigger)
                .on_press(move || set(true))
        }))
        .child(dynamic(move || {
            p.open.get().unwrap_or_else(|| own.get()).then(|| {
                nsh_base::dialog()
                    .animation(nsh_core::ModalAnimation::Fade)
                    .on_cancel(move || set(false))
                    .child(
                        view()
                            .w_full()
                            .h_full()
                            .items_center()
                            .justify_center()
                            .p(move || px(theme.tokens().spacing_6))
                            .child(
                                view()
                                    .absolute()
                                    .inset_0()
                                    .bg(move || {
                                        theme
                                            .tokens()
                                            .dialog_scrim
                                            .resolve(&theme.tokens().color_neutral)
                                    })
                                    .on_press(move || set(false)),
                            )
                            .child(
                                view()
                                    .role(AccessibilityRole::Group)
                                    .on_press(|| {})
                                    .style_with(move |s| {
                                        let t = theme.tokens();
                                        s.w(px(p.width.get().unwrap_or(t.dialog_width)))
                                            .max_w(relative(1.0))
                                            .max_h(relative(1.0))
                                            .overflow_hidden()
                                            .rounded(px(t.frame_window_radius))
                                            .border(px(t.stroke_hairline))
                                            .border_color(if p.danger.get() {
                                                t.color_danger
                                                    .shade(600)
                                                    .alpha(t.dialog_danger_border_alpha)
                                            } else {
                                                t.color_border_strong.hsla()
                                            })
                                            .bg(t.color_surface_overlay.hsla())
                                            .text_color(t.color_content.hsla())
                                            .shadow(shadows(t.shadow_lg))
                                    })
                                    .child(p.children),
                            ),
                    )
            })
        }))
}
#[derive(Clone, PartialEq, Props)]
pub struct DialogHeaderProps {
    pub title: Prop<SharedString>,
    pub subtitle: Prop<Option<SharedString>>,
}
#[component]
pub fn DialogHeader(p: DialogHeaderProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .style_with(move |s| {
            let t = theme.tokens();
            s.flex_shrink_0()
                .min_h(px(t.spacing_11))
                .gap(px(t.spacing_1))
                .px(px(t.spacing_5))
                .py(px(t.spacing_3))
                .border_b(px(t.stroke_hairline))
                .border_color(t.color_border.hsla())
        })
        .child(
            text(p.title)
                .role(AccessibilityRole::Header)
                .style_with(move |s| {
                    styles::typography(s, theme.tokens().title_small)
                        .font_weight(theme.tokens().label_strong.font_weight())
                }),
        )
        .child(
            text(move || p.subtitle.get().unwrap_or_default()).style_with(move |s| {
                styles::typography(s, theme.tokens().caption_small)
                    .text_color(theme.tokens().color_content_subtle.hsla())
            }),
        )
}
pub fn dialog_body() -> nsh_core::element::Scroll {
    let theme = Theme::current();
    nsh_core::element::scroll()
        .min_h(px(0.0))
        .style_with(move |s| {
            let t = theme.tokens();
            s.gap(px(t.spacing_4))
                .px(px(t.spacing_5))
                .py(px(t.spacing_4 + t.spacing_0_5))
        })
}
pub fn dialog_footer() -> nsh_core::element::View {
    let theme = Theme::current();
    view()
        .flex_row()
        .items_center()
        .justify_end()
        .flex_shrink_0()
        .style_with(move |s| {
            let t = theme.tokens();
            s.gap(px(t.spacing_2))
                .px(px(t.spacing_5))
                .py(px(t.spacing_3))
                .bg(t.color_surface_chrome.hsla())
                .border_t(px(t.stroke_hairline))
                .border_color(t.color_border.hsla())
        })
}
#[derive(Clone, PartialEq, Props)]
pub struct DialogCloseProps {
    pub content: Prop<Option<Slot>>,
}
#[component]
pub fn DialogClose(p: DialogCloseProps) -> impl IntoElement {
    let context = use_context::<CloseContext>();
    p.content
        .get_untracked()
        .expect("DialogClose needs content")
        .element()
        .on_press(move || {
            if let Some(context) = &context {
                (context.0)();
            }
        })
}

#[derive(Clone, PartialEq, Props)]
pub struct PopoverPanelProps {
    pub trigger: Prop<Option<Slot>>,
    pub title: Prop<Option<SharedString>>,
    pub description: Prop<Option<SharedString>>,
    pub width: Prop<Option<f32>>,
    #[prop(default=Placement::Bottom)]
    pub side: Prop<Placement>,
    pub children: Prop<Children>,
}
#[component]
pub fn PopoverPanel(p: PopoverPanelProps) -> impl IntoElement {
    let theme = Theme::current();
    let open = signal(false);
    provide_context(CloseContext(Rc::new(move || open.set(false))));
    crate::popover()
        .open(move || Some(open.get()))
        .placement(p.side)
        .on_open_change(move |event: &nsh_base::PopoverEvent| {
            if let nsh_base::PopoverEvent::OpenChange(value, _) = event {
                open.set(*value)
            }
        })
        .when_some(p.trigger.get_untracked(), |el, slot| {
            el.trigger(slot.element())
        })
        .popup_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let s = styles::popup(s, t).p(px(t.spacing_4)).gap(px(t.spacing_3));
                if let Some(width) = p.width.get() {
                    s.w(px(width))
                } else {
                    s
                }
            })
        })
        .child(dynamic(move || {
            p.title.get().map(|title| {
                text(title).style_with(move |s| styles::typography(s, theme.tokens().label_strong))
            })
        }))
        .child(dynamic(move || {
            p.description.get().map(|description| {
                text(description).text_color(move || theme.tokens().color_content_secondary.hsla())
            })
        }))
        .child(p.children)
}

#[derive(Clone, PartialEq, Props)]
pub struct DrawerProps {
    pub trigger: Prop<Option<Slot>>,
    pub title: Prop<SharedString>,
    pub description: Prop<Option<SharedString>>,
    pub footer: Prop<Option<Renderer<bool>>>,
    #[prop(default=Placement::Right)]
    pub side: Prop<Placement>,
    pub children: Prop<Children>,
}
#[component]
pub fn Drawer(p: DrawerProps) -> impl IntoElement {
    let theme = Theme::current();
    let open = signal(false);
    provide_context(CloseContext(Rc::new(move || open.set(false))));
    view()
        .child(
            p.trigger
                .get_untracked()
                .map(|slot| slot.element().on_press(move || open.set(true))),
        )
        .child(
            nsh_base::sheet()
                .open(move || Some(open.get()))
                .side(p.side)
                .on_open_change(move |event: &nsh_base::SheetEvent| {
                    if let nsh_base::SheetEvent::OpenChange(value, _) = event {
                        open.set(*value)
                    }
                })
                .backdrop_style(move |s| {
                    s.bg(move || {
                        theme
                            .tokens()
                            .dialog_scrim
                            .resolve(Tint::Neutral.ramp(theme.tokens()))
                    })
                })
                .popup_style(move |s| {
                    s.style_with(move |s| {
                        let t = theme.tokens();
                        let s = s
                            .bg(t.color_surface_overlay.hsla())
                            .text_color(t.color_content.hsla())
                            .shadow(shadows(t.shadow_lg));
                        if matches!(p.side.get(), Placement::Left | Placement::Right) {
                            s.w(px(t.drawer_size)).h_full()
                        } else {
                            s.h(px(t.drawer_size)).w_full()
                        }
                    })
                })
                .child(dialog_header().title(p.title).subtitle(p.description))
                .child(dialog_body().flex_1().child(p.children))
                .child(p.footer.get_untracked().map(|render| {
                    dialog_footer().child(render.render("footer", open.get_untracked()))
                })),
        )
}

pub fn preview_card() -> nsh_base::HoverCardElement {
    let theme = Theme::current();
    nsh_base::hover_card()
        .open_delay(move || theme.tokens().preview_card_open_delay as f64)
        .close_delay(move || theme.tokens().preview_card_close_delay as f64)
        .offset(move || px(theme.tokens().spacing_1))
        .popup_style(move |s| {
            s.style_with(move |s| {
                styles::popup(s, theme.tokens())
                    .w(px(theme.tokens().preview_card_width))
                    .p(px(theme.tokens().spacing_4))
            })
        })
}
pub fn context_menu() -> nsh_base::ContextMenuElement {
    let theme = Theme::current();
    nsh_base::context_menu()
        .popup_style(move |s| s.style_with(move |s| styles::popup(s, theme.tokens())))
}
