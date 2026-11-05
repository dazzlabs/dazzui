use crate::{State, Theme, Tint, Variant, styles};
use nsh_core::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum ActionAlign {
    #[default]
    Start,
    End,
    Between,
}
#[derive(Clone, PartialEq, Props)]
pub struct ActionBarProps {
    #[prop(default)]
    pub align: Prop<ActionAlign>,
    pub children: Prop<Children>,
}
#[component]
pub fn ActionBar(p: ActionBarProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .flex_row()
        .items_center()
        .gap(move || px(theme.tokens().control_small_gap))
        .style_with(move |s| match p.align.get() {
            ActionAlign::Start => s.justify_start(),
            ActionAlign::End => s.justify_end(),
            ActionAlign::Between => s.justify_between(),
        })
        .child(p.children)
}

#[derive(Clone, PartialEq, Props)]
pub struct PreferencesProps {
    pub children: Prop<Children>,
}
#[component]
pub fn Preferences(p: PreferencesProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .flex_col()
        .w(move || px(theme.tokens().preferences_width))
        .max_w(relative(1.0))
        .gap(move || px(theme.tokens().preferences_group_gap))
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct PreferenceGroupProps {
    pub title: Prop<SharedString>,
    pub description: Prop<Option<SharedString>>,
    pub action: Prop<Option<Slot>>,
    pub children: Prop<Children>,
}
#[component]
pub fn PreferenceGroup(p: PreferenceGroupProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .flex_col()
        .child(
            view()
                .flex_row()
                .items_center()
                .justify_between()
                .gap(move || px(theme.tokens().spacing_4))
                .mb(move || px(theme.tokens().preferences_title_gap))
                .child(
                    view()
                        .flex_1()
                        .min_w(px(0.0))
                        .gap(move || px(theme.tokens().spacing_0_5))
                        .child(text(p.title).style_with(move |s| {
                            styles::typography(s, theme.tokens().title_medium)
                                .text_color(theme.tokens().color_content.hsla())
                        }))
                        .child(when(
                            move || p.description.get().is_some(),
                            move || {
                                styles::paragraph(
                                    move || p.description.get().unwrap_or_default(),
                                    move || {
                                        let t = theme.tokens();
                                        crate::tokens::TypeStyle {
                                            line_height: t.caption_small.size
                                                * t.preferences_description_leading,
                                            ..t.caption_small
                                        }
                                    },
                                )
                                .text_color(move || theme.tokens().color_content_subtle.hsla())
                            },
                        )),
                )
                .child(
                    p.action
                        .place(|slot| slot.h(px(0.0)).flex_row().items_center()),
                ),
        )
        .child(
            view()
                .gap(move || px(theme.tokens().preferences_section_gap))
                .child(p.children),
        )
}
#[derive(Clone, PartialEq, Props)]
pub struct PreferenceSectionProps {
    pub label: Prop<Option<SharedString>>,
    pub action: Prop<Option<Slot>>,
    pub footer: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}
#[component]
pub fn PreferenceSection(p: PreferenceSectionProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .flex_col()
        .child(when(
            move || p.label.get().is_some() || p.action.get().is_some(),
            move || {
                view()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(move || px(theme.tokens().spacing_4))
                    .mb(move || px(theme.tokens().preferences_heading_gap))
                    .child(
                        crate::section_label()
                            .child(text(move || p.label.get().unwrap_or_default())),
                    )
                    .child(
                        p.action
                            .place(|slot| slot.h(px(0.0)).flex_row().items_center()),
                    )
            },
        ))
        .child(
            view()
                .gap(move || px(theme.tokens().preferences_row_gap))
                .child(p.children),
        )
        .child(when(
            move || p.footer.get().is_some(),
            move || {
                styles::paragraph(
                    move || p.footer.get().unwrap_or_default(),
                    move || {
                        let t = theme.tokens();
                        crate::tokens::TypeStyle {
                            line_height: t.caption_small.size * t.preferences_description_leading,
                            ..t.caption_small
                        }
                    },
                )
                .mt(move || px(theme.tokens().preferences_footer_gap))
                .text_color(move || theme.tokens().color_content_subtle.hsla())
            },
        ))
}
#[derive(Clone, PartialEq, Props)]
pub struct PreferenceRowProps {
    pub title: Prop<SharedString>,
    pub subtitle: Prop<Option<SharedString>>,
    pub icon: Prop<Option<Slot>>,
    #[prop(default)]
    pub interactive: Prop<bool>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum PreferenceRowEvent {
    Open,
}
#[component]
pub fn PreferenceRow(p: PreferenceRowProps, cx: Scope<PreferenceRowEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let content = move || {
        view()
            .flex_row()
            .items_center()
            .w_full()
            .min_h(move || px(theme.tokens().control_medium_size))
            .gap(move || px(theme.tokens().spacing_2_5))
            .child(p.icon.place(|s| s.flex_shrink_0()))
            .child(
                view()
                    .flex_1()
                    .min_w(px(0.0))
                    .gap(move || px(theme.tokens().spacing_1))
                    .py(move || px(theme.tokens().preferences_row_padding))
                    .child(text(p.title).truncate().style_with(move |s| {
                        styles::typography(s, theme.tokens().label_quiet)
                            .text_color(theme.tokens().color_content.hsla())
                    }))
                    .child(when(
                        move || p.subtitle.get().is_some(),
                        move || {
                            text(move || p.subtitle.get().unwrap_or_default())
                                .truncate()
                                .style_with(move |s| {
                                    styles::typography(s, theme.tokens().caption_small)
                                        .text_color(theme.tokens().color_content_subtle.hsla())
                                })
                        },
                    )),
            )
            .child(
                view()
                    .flex_row()
                    .items_center()
                    .flex_shrink_0()
                    .gap(move || px(theme.tokens().spacing_2_5))
                    .child(p.children)
                    .child(when(p.interactive, || crate::icon("ChevronRight12Regular"))),
            )
    };
    // A row with a trailing control stays inert. Only an explicit open action is a button.
    if p.interactive.get_untracked() {
        nsh_base::button()
            .accessibility_label(move || Some(p.title.get()))
            .py(px(0.0))
            .px(move || px(theme.tokens().spacing_2))
            .mx(move || px(-theme.tokens().spacing_2))
            .self_stretch()
            .rounded(move || px(theme.tokens().control_container_radius))
            .hover(move |s| {
                s.style_with(move |s| {
                    styles::colors(
                        s,
                        theme.tokens(),
                        Variant::Plain,
                        p.tint.get(),
                        State::Hovered,
                    )
                })
            })
            .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
            .on_press(move || cx.emit(PreferenceRowEvent::Open))
            .child(content())
            .into_element()
    } else {
        content().into_element()
    }
}
