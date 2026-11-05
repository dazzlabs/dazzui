use crate::{Size, State, Theme, Tint, Variant, styles};
use nsh_core::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum LinkUnderline {
    #[default]
    Always,
    Hover,
    None,
}
#[derive(Clone, PartialEq, Props)]
pub struct LinkProps {
    pub href: Prop<Option<SharedString>>,
    #[prop(default)]
    pub external: Prop<bool>,
    #[prop(default)]
    pub inline: Prop<bool>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub underline: Prop<LinkUnderline>,
    pub children: Prop<Children>,
}
#[component]
pub fn Link(p: LinkProps) -> impl IntoElement {
    let theme = Theme::current();
    nsh_base::link()
        .href(p.href)
        .disabled(p.disabled)
        .flex_row()
        .items_center()
        .style_when(p.inline, move |s| {
            let face = theme.tokens().body_medium;
            let metrics = nsh_core::element::text_metrics(
                &format!("{} {}px {}", face.weight, face.size, face.face.family()),
                "x",
            );
            s.h(px((metrics.ascent.0 + metrics.descent.0).ceil()))
        })
        .style_with(move |s| {
            s.text_color(
                crate::axes::ControlColors::resolve(
                    theme.tokens(),
                    Variant::Plain,
                    p.tint.get(),
                    if p.disabled.get() {
                        State::Disabled
                    } else {
                        State::Normal
                    },
                )
                .content,
            )
            .style_when(p.underline.get() == LinkUnderline::Always, |s| {
                s.underline()
            })
        })
        .hover(move |s| {
            s.style_with(move |s| {
                s.style_when(p.underline.get() != LinkUnderline::None, |s| s.underline())
            })
        })
        .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
        .child(p.children)
        .child(dynamic(move || {
            p.external.get().then(|| {
                crate::icon("ArrowUpRight12Regular")
                    .size(move || px(theme.tokens().icon_small))
                    .ml(move || px(theme.tokens().spacing_0_5))
            })
        }))
}

#[derive(Clone, PartialEq, Props)]
pub struct PillTabsProps {
    pub items: Prop<Vec<nsh_base::SelectItem>>,
    pub value: Prop<Option<SharedString>>,
    pub default_value: Prop<Option<SharedString>>,
    pub accessibility_label: Prop<Option<SharedString>>,
    pub render_item: Prop<Option<nsh_core::element::Renderer<nsh_base::SelectItem>>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum PillTabsEvent {
    #[args(value)]
    Change(SharedString),
}
#[component]
pub fn PillTabs(p: PillTabsProps, cx: Scope<PillTabsEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let own = signal(
        p.default_value
            .get_untracked()
            .or_else(|| p.items.get_untracked().first().map(|i| i.value.clone())),
    );
    view().optional_label(p.accessibility_label).child(
        nsh_base::tabs()
            .flex_row()
            .items_center()
            .gap(move || px(theme.tokens().spacing_2))
            .child(dynamic(move || {
                p.items
                    .get()
                    .into_iter()
                    .map(|item| {
                        let target = item.value.clone();
                        let selected = target.clone();
                        let active =
                            move || p.value.get().or_else(|| own.get()).as_ref() == Some(&selected);
                        let content = if let Some(renderer) = p.render_item.get() {
                            renderer
                                .render(item.value.clone(), item.clone())
                                .into_element()
                        } else {
                            text(item.title).into_element()
                        };
                        let reference = element_ref();
                        nsh_base::tab()
                            .element_ref(&reference)
                            .selected(active.clone())
                            .disabled(item.disabled)
                            .style_with(move |s| {
                                let t = theme.tokens();
                                let s = styles::typography(s, t.label_small)
                                    .flex_row()
                                    .items_center()
                                    .gap(px(t.control_tiny_gap))
                                    .h(px(t.control_tiny_size))
                                    .px(px(t.spacing_3))
                                    .rounded(px(t.radius_full));
                                if active() {
                                    styles::colors(
                                        s,
                                        t,
                                        Variant::Filled,
                                        Tint::Primary,
                                        State::Normal,
                                    )
                                } else {
                                    s.bg(t.color_surface_inset.hsla())
                                        .text_color(t.color_content_nav.hsla())
                                }
                            })
                            .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
                            .on_press(move || {
                                reference.focus();
                                if p.value.get_untracked().is_none() {
                                    own.set(Some(target.clone()));
                                }
                                cx.emit(PillTabsEvent::Change(target.clone()));
                            })
                            .child(content)
                    })
                    .collect::<Vec<_>>()
            })),
    )
}

#[derive(Clone, PartialEq, Props)]
pub struct ToggleGroupProps {
    #[prop(default)]
    pub icon_only_items: Prop<Vec<SharedString>>,
    pub items: Prop<Vec<nsh_base::SelectItem>>,
    pub value: Prop<Option<Vec<SharedString>>>,
    #[prop(default)]
    pub default_value: Prop<Vec<SharedString>>,
    #[prop(default = true)]
    pub multiple: Prop<bool>,
    #[prop(default = Variant::Recessed)]
    pub variant: Prop<Variant>,
    #[prop(default)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
    pub render_item: Prop<Option<nsh_core::element::Renderer<nsh_base::SelectItem>>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum ToggleGroupEvent {
    #[args(value)]
    Change(Vec<SharedString>),
}
#[component]
pub fn ToggleGroup(p: ToggleGroupProps, cx: Scope<ToggleGroupEvent>) -> impl IntoElement {
    let own = signal(p.default_value.get_untracked());
    let theme = Theme::current();
    view()
        .optional_label(p.accessibility_label)
        .child(nsh_base::toggle_group().flex_row().child(dynamic(move || {
            let items = p.items.get();
            let count = items.len();
            items
                .into_iter()
                .enumerate()
                .map(|(index, item)| {
                    let icon_only = p.icon_only_items.get().contains(&item.value);
                    let target = item.value.clone();
                    let selected = target.clone();
                    let content = if let Some(renderer) = p.render_item.get() {
                        renderer
                            .render(item.value.clone(), item.clone())
                            .into_element()
                    } else {
                        text(item.title).into_element()
                    };
                    crate::toggle()
                        .size(p.size)
                        .variant(p.variant)
                        .icon_only(icon_only)
                        .style_with(move |s| {
                            s.style_when(index > 0, |s| {
                                s.ml(px(-theme.tokens().stroke_hairline)).rounded_s(px(0.0))
                            })
                            .style_when(index + 1 < count, |s| s.rounded_e(px(0.0)))
                        })
                        .tint(p.tint)
                        .disabled(item.disabled)
                        .pressed(move || {
                            Some(
                                p.value
                                    .get()
                                    .unwrap_or_else(|| own.get())
                                    .contains(&selected),
                            )
                        })
                        .on_change(move |on: &bool| {
                            let mut next = if p.multiple.get() {
                                p.value.get().unwrap_or_else(|| own.get())
                            } else {
                                vec![]
                            };
                            next.retain(|v| v != &target);
                            if *on {
                                next.push(target.clone());
                            }
                            if p.value.get_untracked().is_none() {
                                own.set(next.clone());
                            }
                            cx.emit(ToggleGroupEvent::Change(next));
                        })
                        .child(content)
                })
                .collect::<Vec<_>>()
        })))
}

#[derive(Clone, PartialEq, Props)]
pub struct TagProps {
    pub selected: Prop<Option<bool>>,
    #[prop(default)]
    pub removable: Prop<bool>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum TagEvent {
    #[args(selected)]
    SelectedChange(bool),
    Remove,
}
#[component]
pub fn Tag(p: TagProps, cx: Scope<TagEvent>) -> impl IntoElement {
    let theme = Theme::current();
    crate::badge().tint(p.tint).child(
        view()
            .flex_row()
            .items_center()
            .child(dynamic(move || {
                if p.selected.get().is_some() {
                    nsh_base::toggle()
                        .pressed(move || p.selected.get().unwrap_or(false))
                        .disabled(p.disabled)
                        .on_change(move |v: &bool| cx.emit(TagEvent::SelectedChange(*v)))
                        .child(p.children)
                        .into_element()
                } else {
                    view().child(p.children).into_element()
                }
            }))
            .child(dynamic(move || {
                p.removable.get().then(|| {
                    nsh_base::button()
                        .disabled(p.disabled)
                        .accessibility_label("Remove tag")
                        .ml(move || px(theme.tokens().spacing_1))
                        .on_press(move || cx.emit(TagEvent::Remove))
                        .child(
                            crate::icon("Dismiss12Regular")
                                .size(move || px(theme.tokens().icon_small)),
                        )
                })
            })),
    )
}

pub fn avatar_group() -> nsh_base::AvatarGroupElement {
    let theme = Theme::current();
    nsh_base::avatar_group()
        .flex_row()
        .items_center()
        .gap(move || px(-theme.tokens().spacing_2))
        .overflow_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                styles::typography(s, t.label_small)
                    .w(px(t.spacing_7))
                    .h(px(t.spacing_7))
                    .rounded(px(t.radius_full))
                    .bg(t.color_surface_sunken.hsla())
                    .items_center()
                    .justify_center()
            })
        })
}
