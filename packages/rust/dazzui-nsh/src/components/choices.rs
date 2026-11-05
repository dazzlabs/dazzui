use crate::{SelectItem, Size, Theme, Tint, Variant, styles};
use nsh_core::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupProps {
    pub options: Prop<Vec<SelectItem>>,
    pub value: Prop<Option<SharedString>>,
    pub default_value: Prop<Option<SharedString>>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum RadioGroupEvent {
    #[args(value)]
    Change(SharedString),
}
#[component]
pub fn RadioGroup(p: RadioGroupProps, cx: Scope<RadioGroupEvent>) -> impl IntoElement {
    let own = signal(p.default_value.get_untracked());
    let theme = Theme::current();
    view()
        .optional_label(p.accessibility_label)
        .role(AccessibilityRole::Group)
        .child(
            nsh_base::radio_group()
                .gap(move || px(theme.tokens().spacing_2_5))
                .child(dynamic(move || {
                    p.options
                        .get()
                        .into_iter()
                        .map(|item| {
                            let selected = item.value.clone();
                            let value = item.value;
                            let event = cx;
                            crate::radio()
                                .size(p.size)
                                .tint(p.tint)
                                .checked(move || {
                                    p.value.get().or_else(|| own.get()).as_ref() == Some(&selected)
                                })
                                .disabled(item.disabled)
                                .on_change(move || {
                                    if p.value.get_untracked().is_none() {
                                        own.set(Some(value.clone()));
                                    }
                                    event.emit(RadioGroupEvent::Change(value.clone()));
                                })
                                .child(text(item.title))
                        })
                        .collect::<Vec<_>>()
                })),
        )
}
#[derive(Clone, Copy, Default, PartialEq, PropEnum)]
pub enum SegmentStyle {
    #[default]
    Raised,
    Filled,
}
#[derive(Clone, PartialEq, Props)]
pub struct SegmentedControlProps {
    #[prop(default)]
    pub icon_only: Prop<bool>,
    #[prop(default)]
    pub leading_icon: Prop<bool>,
    pub render_item: Prop<Option<nsh_core::element::Renderer<SelectItem>>>,
    pub render_icon: Prop<Option<nsh_core::element::Renderer<SelectItem>>>,
    #[prop(default)]
    pub active_style: Prop<SegmentStyle>,
    pub items: Prop<Vec<SelectItem>>,
    pub value: Prop<Option<SharedString>>,
    pub default_value: Prop<Option<SharedString>>,
    #[prop(default = Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    #[prop(default)]
    pub stretch: Prop<bool>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum SegmentedControlEvent {
    #[args(value)]
    Change(SharedString),
}
#[component]
pub fn SegmentedControl(
    p: SegmentedControlProps,
    cx: Scope<SegmentedControlEvent>,
) -> impl IntoElement {
    let theme = Theme::current();
    let own = signal(p.default_value.get_untracked());
    provide_context(super::icon::IconSize(std::rc::Rc::new(move || {
        p.size.get().profile(theme.tokens()).icon_size
            - if p.icon_only.get() {
                0.0
            } else {
                theme.tokens().control_label_glyph_shrink
            }
    })));
    view()
        .optional_label(p.accessibility_label)
        .role(AccessibilityRole::Group)
        .child(
            nsh_base::tabs()
                .flex_row()
                .style_with(move |s| {
                    let t = theme.tokens();
                    s.p(px(t.segmented_control_inset))
                        .rounded(px(if p.size.get() == Size::Medium {
                            t.radius_medium
                        } else {
                            p.size.get().profile(t).radius
                        }))
                        .bg(t.color_surface_inset.hsla())
                        .style_when(p.stretch, |s| s.w_full())
                })
                .child(dynamic(move || {
                    p.items
                        .get()
                        .into_iter()
                        .map(|item| {
                            let target = item.value.clone();
                            let selected = target.clone();
                            let event = cx;
                            let active = computed(move || {
                                p.value.get().or_else(|| own.get()).as_ref() == Some(&selected)
                            });
                            let label = item.title.clone();
                            let content = if let Some(renderer) = p.render_item.get() {
                                renderer
                                    .render(item.value.clone(), item.clone())
                                    .into_element()
                            } else {
                                view()
                                    .flex_row()
                                    .items_center()
                                    .gap(move || px(p.size.get().profile(theme.tokens()).gap))
                                    .child(p.render_icon.get().map(|r| {
                                        r.render(item.value.clone(), item.clone()).into_element()
                                    }))
                                    .child(when(
                                        move || !p.icon_only.get(),
                                        move || {
                                            let label = label.clone();
                                            styles::label(
                                                move || label.clone(),
                                                move || {
                                                    let t = theme.tokens();
                                                    let mut face = p.size.get().profile(t).content;
                                                    face.weight = if active.get() {
                                                        t.label_medium.weight
                                                    } else {
                                                        t.label_quiet.weight
                                                    };
                                                    face
                                                },
                                            )
                                        },
                                    ))
                                    .into_element()
                            };
                            let reference = element_ref();
                            nsh_base::tab()
                                .element_ref(&reference)
                                .selected(active)
                                .disabled(item.disabled)
                                .flex_row()
                                .items_center()
                                .justify_center()
                                .style_with(move |s| {
                                    let t = theme.tokens();
                                    let profile = p.size.get().profile(t);
                                    let inset = t.segmented_control_inset;
                                    let radius = if p.size.get() == Size::Medium {
                                        t.radius_medium
                                    } else {
                                        profile.radius
                                    };
                                    let mut face = profile.content;
                                    face.weight = if active.get() {
                                        t.label_medium.weight
                                    } else {
                                        t.label_quiet.weight
                                    };
                                    let s = styles::typography(s, face)
                                        .h(px(profile.size - 2.0 * inset))
                                        .px(px(t.spacing_3))
                                        .gap(px(profile.gap))
                                        .rounded(px(radius - inset))
                                        .text_color(t.color_content_secondary.hsla())
                                        .opacity(if item.disabled { 0.5 } else { 1.0 })
                                        .style_when(p.stretch, |s| s.flex_1());
                                    let s = s.style_when(
                                        p.leading_icon.get() && !p.icon_only.get(),
                                        |s| s.pl(px(t.spacing_3 - t.control_label_glyph_inset)),
                                    );
                                    if active.get() {
                                        let s = s.shadow(crate::tokens::shadows(t.shadow_2xs));
                                        if p.active_style.get() == SegmentStyle::Filled {
                                            styles::colors(
                                                s,
                                                t,
                                                Variant::Filled,
                                                p.tint.get(),
                                                crate::State::Normal,
                                            )
                                        } else {
                                            s.bg(t.color_surface_raised.hsla())
                                                .text_color(t.color_content.hsla())
                                        }
                                    } else {
                                        s
                                    }
                                })
                                .focus(move |s| {
                                    s.style_with(move |s| styles::focus(s, theme.tokens()))
                                })
                                .on_press(move || {
                                    reference.focus();
                                    if p.value.get_untracked().is_none() {
                                        own.set(Some(target.clone()));
                                    }
                                    event.emit(SegmentedControlEvent::Change(target.clone()));
                                })
                                .child(content)
                        })
                        .collect::<Vec<_>>()
                })),
        )
}
