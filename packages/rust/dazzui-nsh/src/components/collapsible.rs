use crate::{Theme, styles};
use nsh_core::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleProps {
    pub title: Prop<SharedString>,
    pub meta: Prop<Option<Slot>>,
    pub open: Prop<Option<bool>>,
    #[prop(default)]
    pub default_open: Prop<bool>,
    pub children: Prop<Children>,
}
#[derive(Clone, PartialEq, Event)]
pub enum CollapsibleEvent {
    #[args(open)]
    Change(bool),
}
#[component]
pub fn Collapsible(p: CollapsibleProps, cx: Scope<CollapsibleEvent>) -> impl IntoElement {
    let theme = Theme::current();
    let own = signal(p.default_open.get_untracked());
    let open = move || p.open.get().unwrap_or_else(|| own.get());
    nsh_base::collapsible()
        .open(open)
        .content(
            view()
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
        .child(
            nsh_base::button()
                .w_full()
                .flex_row()
                .items_center()
                .style_with(move |s| {
                    styles::typography(s, theme.tokens().label_medium)
                        .justify_start()
                        .gap(px(theme.tokens().spacing_2))
                        .p(px(theme.tokens().spacing_2))
                        .rounded(px(theme.tokens().control_container_radius))
                })
                .on_press(move || {
                    let next = !open();
                    if p.open.get_untracked().is_none() {
                        own.set(next);
                    }
                    cx.emit(CollapsibleEvent::Change(next));
                })
                .child(
                    crate::icon("ChevronRight12Regular")
                        .size(move || px(theme.tokens().icon_small))
                        .rotate(move || nsh_core::geometry::deg(if open() { 90.0 } else { 0.0 })),
                )
                .child(text(p.title))
                .child(view().flex_1())
                .child(p.meta.get_untracked().map(|slot| slot.element())),
        )
}
