use crate::Theme;
use nsh_core::{geometry::Axis, prelude::*};
#[derive(Clone, Copy)]
struct SplitterContext {
    extent: Signal<f32>,
}
#[derive(Clone, PartialEq, Props)]
pub struct SplitterProps {
    #[prop(default=Axis::Horizontal)]
    pub axis: Prop<Axis>,
    pub children: Prop<Children>,
}
#[component]
pub fn Splitter(p: SplitterProps) -> impl IntoElement {
    let theme = Theme::current();
    let extent = signal(0.0);
    provide_context(SplitterContext { extent });
    view()
        .w_full()
        .h_full()
        .on_layout(move |bounds: &nsh_core::Bounds<nsh_core::Points>| {
            extent.set(if p.axis.get() == Axis::Horizontal {
                bounds.size.width.0
            } else {
                bounds.size.height.0
            })
        })
        .child(
            nsh_base::resizable_panel_group()
                .axis(p.axis)
                .gap(move || px(theme.tokens().spacing_2))
                .handle_accessibility_label("Resize panes")
                .handle_style(move |s| s.bg(move || theme.tokens().color_border.hsla()))
                .handle_class(move || {
                    Some(
                        format!(
                            "group-hover:bg-[#{:08x}] group-active:bg-[#{:08x}]",
                            theme.tokens().color_primary.shade(600).0,
                            theme.tokens().color_primary.shade(600).0
                        )
                        .into(),
                    )
                })
                .child(p.children),
        )
}
#[derive(Clone, PartialEq, Props)]
pub struct SplitterPanelProps {
    #[prop(default = 50.0)]
    pub default_size: Prop<f32>,
    #[prop(default)]
    pub min_size: Prop<f32>,
    #[prop(default = 100.0)]
    pub max_size: Prop<f32>,
    pub children: Prop<Children>,
}
#[component]
pub fn SplitterPanel(p: SplitterPanelProps) -> impl IntoElement {
    let parent = use_context::<SplitterContext>().expect("SplitterPanel needs Splitter");
    let share = signal(p.default_size.get_untracked());
    nsh_base::resizable_panel()
        .size(move || {
            (parent.extent.get() > 0.0).then(|| px(parent.extent.get() * share.get() / 100.0))
        })
        .min_size(move || px(parent.extent.get() * p.min_size.get() / 100.0))
        .max_size(move || {
            (parent.extent.get() > 0.0).then(|| px(parent.extent.get() * p.max_size.get() / 100.0))
        })
        .on_resize(move |size: &nsh_core::Points| {
            let extent = parent.extent.get_untracked();
            if extent > 0.0 {
                share.set(size.0 / extent * 100.0);
            }
        })
        .child(
            nsh_core::element::scroll()
                .w_full()
                .h_full()
                .child(p.children),
        )
}
