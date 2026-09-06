//! `Components/Aside` — the trailing information column.

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Aside, Badge, BadgeSize, Button, Icon, NavItem, SectionLabel, Sidebar,
    SidebarGroup, ThemeVariables, Tint, assets::icons,
};
use gpui::{AnyElement, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::stage::{Frame, Pane};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Aside",
    &[
        Story::page("Overview", |_, _, cx| {
            let vars = cx.theme().vars;

            DocPage::new("Aside")
                .description(
                    "The trailing information column. It carries what the current selection \
                     *is*, where the sidebar and the rail carry what you can navigate to — \
                     which is why it hangs off the opposite edge and takes no rows.",
                )
                .child(
                    Section::new("Where it sits")
                        .description(
                            "It shares the sidebar's muted surface rather than the rail's, so a \
                             window reads as content between two quieter edges rather than as \
                             three different greys. Its width is the `frame.aside` metric, not a \
                             value this component decides.",
                        )
                        .child(
                            Frame::new(px(720.), px(340.))
                                .child(Pane::new().child("The selection"))
                                .child(Aside::new().child(inspector(vars))),
                        ),
                )
                .child(
                    Section::new("What goes in it")
                        .description(
                            "Labelled facts, a badge where a fact has a state, and at most one \
                             action at the foot. A row you can select belongs in the sidebar or \
                             the rail instead.",
                        )
                        .child(
                            Frame::new(px(420.), px(320.)).child(
                                Aside::new()
                                    .child(inspector(vars))
                                    .child(Button::new("reveal", "Reveal").full_width(true)),
                            ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Beside a pane", |_, _, cx| {
            let vars = cx.theme().vars;
            Frame::new(px(640.), px(340.))
                .child(Pane::new().child("The selection"))
                .child(Aside::new().child(inspector(vars)))
                .into_any_element()
        }),
        Story::new("An inspector", |_, _, cx| {
            let vars = cx.theme().vars;
            Frame::new(px(420.), px(320.))
                .child(Aside::new().child(inspector(vars)))
                .into_any_element()
        }),
        Story::new("With an action", |_, _, cx| {
            let vars = cx.theme().vars;
            Frame::new(px(420.), px(320.))
                .child(
                    Aside::new()
                        .child(inspector(vars))
                        .child(Button::new("reveal", "Reveal").full_width(true)),
                )
                .into_any_element()
        }),
        Story::new("In a three-column window", |state, this, cx| {
            let vars = cx.theme().vars;
            let mut library = SidebarGroup::new().label("Library").first(true);
            for (id, label, icon) in [
                ("projects", "Projects", icons::SQUARE_MULTIPLE_16_REGULAR),
                ("settings", "Settings", icons::SETTINGS_16_REGULAR),
            ] {
                library = library.child(
                    NavItem::new(SharedString::from(format!("aside-{id}")), label)
                        .icon(Icon::new(icon))
                        .active(state.library == id)
                        .on_click(crate::act(this, move |state, _, _| {
                            state.library = id.into()
                        })),
                );
            }

            Frame::new(px(900.), px(380.))
                .child(Sidebar::new("aside-sidebar").child(library))
                .child(Pane::new().child("aurora"))
                .child(Aside::new().child(inspector(vars)))
                .into_any_element()
        }),
    ],
);

/// What the current selection *is* — a run of labelled facts rather than rows
/// you can navigate to.
fn inspector(vars: &'static ThemeVariables) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(vars.spacing_3)
        .child(SectionLabel::new("Details"))
        .child(fact(vars, "Branch", Some("main"), None))
        .child(fact(vars, "Commit", Some("785f949"), None))
        .child(fact(
            vars,
            "Status",
            None,
            Some(
                Badge::new("Passed")
                    .size(BadgeSize::Small)
                    .tint(Tint::Success)
                    .into_any_element(),
            ),
        ))
        .child(SectionLabel::new("Timing"))
        .child(fact(vars, "Started", Some("09:41"), None))
        .child(fact(vars, "Duration", Some("2m 14s"), None))
}

fn fact(
    vars: &'static ThemeVariables,
    name: &'static str,
    value: Option<&'static str>,
    accessory: Option<AnyElement>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(vars.spacing_0_5)
        .child(
            div()
                .type_style(vars.caption_small)
                .text_color(vars.color_content_subtle)
                .child(name),
        )
        .when_some(value, |el, value| {
            el.child(
                div()
                    .type_style(vars.label_medium)
                    .text_color(vars.color_content)
                    .child(value),
            )
        })
        .children(accessory)
}
