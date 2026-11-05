use crate::{Size, Theme, Tint, styles};
pub use nsh_base::{TreeEntry, TreeItem};
use nsh_core::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TreeProps {
    pub items: Prop<Vec<TreeItem>>,
    pub expanded: Prop<Option<Vec<SharedString>>>,
    #[prop(default)]
    pub default_expanded: Prop<Vec<SharedString>>,
    #[prop(default)]
    pub selected: Prop<nsh_base::SelectValue>,
    pub default_selected: Prop<Option<SharedString>>,
    #[prop(default=Size::Medium)]
    pub size: Prop<Size>,
    #[prop(default)]
    pub tint: Prop<Tint>,
    pub accessibility_label: Prop<Option<SharedString>>,
    pub render_item: Prop<Option<Renderer<TreeEntry>>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum TreeEvent {
    #[args(value)]
    SelectedChange(SharedString),
    #[args(value)]
    ExpandedChange(Vec<SharedString>),
}
#[component]
pub fn Tree(p: TreeProps, cx: Scope<TreeEvent>) -> impl IntoElement {
    let theme = Theme::current();
    nsh_base::tree()
        .items(p.items)
        .expanded(p.expanded)
        .default_expanded(p.default_expanded)
        .selected(p.selected)
        .default_selected(p.default_selected)
        .accessibility_label(p.accessibility_label)
        .item_style(move |s| {
            s.style_with(move |s| {
                let t = theme.tokens();
                let small = matches!(p.size.get(), Size::Tiny | Size::Small);
                styles::typography(s, if small { t.label_small } else { t.label_quiet })
                    .h(px(if small { t.spacing_6 } else { t.spacing_7 }))
                    .gap(px(t.spacing_1))
                    .rounded(px(t.radius_small))
                    .text_color(t.color_content_nav.hsla())
            })
        })
        .item_class(move || {
            Some(
                format!(
                    "group selected:bg-[#{:08x}] selected:font-semibold focus:bg-[#{:08x}]",
                    (p.tint.get().ramp(theme.tokens()).shade(600).0 & 0xffffff00)
                        | (theme.tokens().wash_surface * 255.0).round() as u32,
                    theme.tokens().color_surface_subtle.0
                )
                .into(),
            )
        })
        .render_item(move |entry: &TreeEntry| {
            p.render_item
                .get()
                .map(|r| r.render(entry.id.clone(), entry.clone()).into_element())
                .unwrap_or_else(|| text(entry.label.clone()).into_element())
        })
        .on_selected_change(move |v: &SharedString| cx.emit(TreeEvent::SelectedChange(v.clone())))
        .on_expanded_change(move |v: &Vec<SharedString>| {
            cx.emit(TreeEvent::ExpandedChange(v.clone()))
        })
}
