use crate::{Theme, styles};
use nsh_core::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AccordionProps {
    pub value: Prop<Option<Vec<SharedString>>>,
    #[prop(default)]
    pub default_value: Prop<Vec<SharedString>>,
    #[prop(default)]
    pub multiple: Prop<bool>,
    #[prop(default)]
    pub divided: Prop<bool>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum AccordionEvent {
    #[args(value)]
    Change(Vec<SharedString>),
}
#[derive(Clone, Copy)]
struct AccordionStyle {
    divided: Prop<bool>,
}
#[component]
pub fn Accordion(p: AccordionProps, cx: Scope<AccordionEvent>) -> impl IntoElement {
    let own = signal(p.default_value.get_untracked());
    provide_context(AccordionStyle { divided: p.divided });
    nsh_base::accordion()
        .multiple(p.multiple)
        .value(move || p.value.get().unwrap_or_else(|| own.get()))
        .on_change(move |v: &Vec<SharedString>| {
            if p.value.get_untracked().is_none() {
                own.set(v.clone());
            }
            cx.emit(AccordionEvent::Change(v.clone()));
        })
        .child(p.children)
}
#[derive(Clone, PartialEq, Props)]
pub struct AccordionItemProps {
    pub value: Prop<SharedString>,
    pub title: Prop<SharedString>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    pub children: Prop<Children>,
}
#[component]
pub fn AccordionItem(p: AccordionItemProps) -> impl IntoElement {
    let theme = Theme::current();
    let group = use_context::<AccordionStyle>();
    nsh_base::accordion_item()
        .value(p.value)
        .app_class(Some(SharedString::from("first:border-t-0")))
        .disabled(p.disabled)
        .style_with(move |s| {
            s.style_when(group.is_some_and(|g| g.divided.get()), |s| {
                s.border_t(px(theme.tokens().stroke_hairline))
                    .border_color(theme.tokens().color_border.hsla())
            })
        })
        .child(
            nsh_base::accordion_header().child(
                nsh_base::accordion_trigger()
                    .app_class(Some(SharedString::from("group")))
                    .style_with(move |s| {
                        styles::typography(s, theme.tokens().label_medium)
                            .flex_row()
                            .items_center()
                            .gap(px(theme.tokens().spacing_2))
                            .p(px(theme.tokens().spacing_2))
                            .rounded(px(theme.tokens().control_container_radius))
                            .w_full()
                    })
                    .focus(move |s| s.style_with(move |s| styles::focus(s, theme.tokens())))
                    .child(
                        crate::icon("ChevronRight12Regular")
                            .size(move || px(theme.tokens().icon_small))
                            .text_color(move || theme.tokens().color_content_subtle.hsla())
                            .app_class(Some(SharedString::from("group-aria-expanded:rotate-90"))),
                    )
                    .child(text(p.title)),
            ),
        )
        .child(
            nsh_base::accordion_panel()
                .style_with(move |s| {
                    let t = theme.tokens();
                    styles::typography(s, t.body_small)
                        .pt(px(t.spacing_1))
                        .pb(px(t.spacing_3))
                        .pl(px(t.spacing_2 + t.icon_small + t.spacing_2))
                        .pr(px(t.spacing_2))
                        .text_color(t.color_content_secondary.hsla())
                })
                .child(p.children),
        )
}
