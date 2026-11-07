//! A fixed rail's outer inset replaces the first and last group's padding.

use dazzui::{ActiveTheme, Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, Theme};
use gpui::{
    App, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, TestAppContext,
    Window, div, px, size,
};

struct Rails;

impl Render for Rails {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .children([true, false].map(|fixed| {
                Sidebar::new()
                    .collapsible(if fixed {
                        SidebarCollapsible::None
                    } else {
                        SidebarCollapsible::Offcanvas
                    })
                    .child(SidebarContent::new().children((0..3).map(move |index| {
                        SidebarGroup::new().child(
                            div()
                                .debug_selector(move || format!("row-{fixed}-{index}").into())
                                .h(px(10.)),
                        )
                    })))
            }))
    }
}

#[gpui::test]
fn fixed_rail_keeps_outer_insets_and_spacing_between_groups(cx: &mut TestAppContext) {
    cx.update(|cx: &mut App| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
    });
    let (_, cx) = cx.add_window_view(|_, _| Rails);
    cx.simulate_resize(size(px(600.), px(300.)));
    cx.run_until_parked();
    let (fixed_inset, normal_inset, group_padding) = cx.update(|_, cx| {
        let t = cx.theme();
        (t.spacing_2_5, t.spacing_2, t.spacing_1_5)
    });
    for (fixed, selectors) in [
        (true, ["row-true-0", "row-true-1", "row-true-2"]),
        (false, ["row-false-0", "row-false-1", "row-false-2"]),
    ] {
        for (index, selector) in selectors.into_iter().enumerate() {
            let bounds = cx.debug_bounds(selector).expect("group's content is drawn");
            let top = if fixed {
                fixed_inset
            } else {
                normal_inset + group_padding
            };
            assert_eq!(
                bounds.top(),
                px(top + index as f32 * (10. + 2. * group_padding))
            );
        }
    }
}
