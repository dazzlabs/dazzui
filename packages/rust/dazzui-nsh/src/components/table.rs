use crate::{Theme, styles};
use nsh_core::prelude::*;

pub fn table() -> nsh_base::TableElement {
    nsh_base::table().w_full()
}
pub fn table_head() -> nsh_base::TableRowElement {
    let theme = Theme::current();
    nsh_base::table_row().style_with(move |s| {
        let t = theme.tokens();
        s.flex_row()
            .px(px(t.spacing_5))
            .border_b(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
            .bg(t.color_surface_chrome.hsla())
    })
}
pub fn table_row() -> nsh_base::TableRowElement {
    let theme = Theme::current();
    nsh_base::table_row().style_with(move |s| {
        let t = theme.tokens();
        styles::typography(s, t.body_medium)
            .flex_row()
            .items_center()
            .px(px(t.spacing_5))
            .py(px(t.spacing_3))
            .border_b(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
    })
}
#[derive(Clone, PartialEq, Props)]
pub struct TableCellProps {
    #[prop(default)]
    pub head: Prop<bool>,
    #[prop(default)]
    pub align_end: Prop<bool>,
    pub children: Prop<Children>,
}
#[component]
pub fn TableCell(p: TableCellProps) -> impl IntoElement {
    let theme = Theme::current();
    view()
        .role(move || {
            if p.head.get() {
                AccessibilityRole::ColumnHeader
            } else {
                AccessibilityRole::Cell
            }
        })
        .style_with(move |s| {
            let t = theme.tokens();
            let s = s
                .flex_1()
                .min_w(px(0.0))
                .style_when(p.align_end.get(), |s| s.items_end());
            if p.head.get() {
                styles::typography(s, t.label_small)
                    .py(px(t.spacing_2))
                    .text_color(t.color_content_faint.hsla())
            } else {
                s
            }
        })
        .child(p.children)
}
