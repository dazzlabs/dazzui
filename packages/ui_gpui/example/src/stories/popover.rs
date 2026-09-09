//! `Components/Popover` — a panel of anything, anchored to what opened it.

use dazzui::{
    Button, Checkbox, ControlVariant, FormField, Icon, IconButton, Popover, PopoverAlign,
    PopoverSide, TextField, assets::icons,
};
use gpui::{IntoElement, ParentElement, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

const SIDES: [PopoverSide; 4] = [
    PopoverSide::Top,
    PopoverSide::Bottom,
    PopoverSide::Left,
    PopoverSide::Right,
];

fn about(id: &'static str, side: PopoverSide, label: &'static str) -> Popover {
    Popover::new(id)
        .side(side)
        .title("Build 4.2.1")
        .description("Published 3 days ago from main.")
        .trigger(move |_, _| Button::new("about-trigger", label).into_any_element())
}

fn filter(id: &'static str) -> Popover {
    Popover::new(id)
        .align(PopoverAlign::Start)
        .width(px(256.))
        .title("Filter")
        .trigger(|_, _| {
            IconButton::new(
                "filter-trigger",
                "Filter",
                Icon::new(icons::SEARCH_16_REGULAR),
            )
            .into_any_element()
        })
        .child(
            Col::new()
                .child(Checkbox::new("open", "Open", true))
                .child(Checkbox::new("merged", "Merged", false))
                .child(Checkbox::new("closed", "Closed", false)),
        )
}

pub const PAGE: StoryPage = StoryPage::new(
    "Popover",
    &[
        Story::page("Overview", |_, _, _| {
            let mut sides = Specimens::new();
            for side in SIDES {
                let name: &'static str = match side {
                    PopoverSide::Top => "top",
                    PopoverSide::Bottom => "bottom",
                    PopoverSide::Left => "left",
                    PopoverSide::Right => "right",
                };
                sides = sides.child(Specimen::new(name).child(about("doc-side", side, name)));
            }

            DocPage::new("Popover")
                .description(
                    "A panel of anything, anchored to what opened it. It is the sibling of Menu \
                     rather than of Dialog: it wears the same kind of floating sheet and is \
                     positioned against its trigger, but it holds content rather than rows. \
                     Reach for a dialog instead when the reader has to finish before doing \
                     anything else — a popover closes on an outside press, which makes it the \
                     wrong home for something half-typed.",
                )
                .child(
                    Section::new("A heading and a line")
                        .description(
                            "The title names the panel and the description describes it. The \
                             gap between the panel and its trigger is a token rather than a \
                             number passed in.",
                        )
                        .child(about("doc-about", PopoverSide::Bottom, "About this build")),
                )
                .child(
                    Section::new("Sides")
                        .description("Anchored to whichever edge of the trigger it is told.")
                        .child(sides),
                )
                .child(
                    Section::new("A panel with controls in it")
                        .description(
                            "This is what separates it from a Tooltip: a popover can be reached \
                             and pressed. A tooltip must carry nothing a reader needs, because \
                             a pointer never gets there.",
                        )
                        .child(filter("doc-filter")),
                )
                .into_any_element()
        }),
        Story::new("Bare", |_, _, _| {
            about("bare", PopoverSide::Bottom, "About this build").into_any_element()
        }),
        Story::new("Sides", |_, _, _| {
            Row::new()
                .child(about("side-top", PopoverSide::Top, "top"))
                .child(about("side-bottom", PopoverSide::Bottom, "bottom"))
                .child(about("side-left", PopoverSide::Left, "left"))
                .child(about("side-right", PopoverSide::Right, "right"))
                .into_any_element()
        }),
        Story::new("With controls", |_, _, _| {
            filter("with-controls").into_any_element()
        }),
        Story::new("A form", |state, _, _| {
            Popover::new("a-form")
                .width(px(320.))
                .title("Rename branch")
                .description("The remote keeps its own name until you push.")
                .trigger(|_, _| {
                    Button::new("rename-trigger", "Rename branch")
                        .variant(ControlVariant::Filled)
                        .into_any_element()
                })
                .child(
                    FormField::new("Branch name").child(TextField::new(&state.branch).mono(true)),
                )
                .into_any_element()
        }),
        Story::new("In a toolbar", |_, _, _| {
            Row::new()
                .child(filter("toolbar-filter"))
                .child(
                    Popover::new("toolbar-about")
                        .align(PopoverAlign::End)
                        .title("About")
                        .description("42 pull requests, 8 of them yours.")
                        .trigger(|_, _| {
                            IconButton::new(
                                "about-icon",
                                "About",
                                Icon::new(icons::ALERT_16_REGULAR),
                            )
                            .into_any_element()
                        }),
                )
                .into_any_element()
        }),
    ],
);
