//! `Components/Drawer` — a sheet that comes in from an edge.

use dazzui::{Button, Checkbox, ControlVariant, Drawer, DrawerSide, FormField, TextField};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::Playground;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};

const SIDES: [(DrawerSide, &str); 4] = [
    (DrawerSide::End, "end"),
    (DrawerSide::Start, "start"),
    (DrawerSide::Top, "top"),
    (DrawerSide::Bottom, "bottom"),
];

/// A drawer is absolute, so the page has to give it a box to be absolute in —
/// the counterpart of the window it covers in an app.
fn frame(width: f32, height: f32, drawer: Drawer) -> impl IntoElement {
    div()
        .relative()
        .w(px(width))
        .h(px(height))
        .overflow_hidden()
        .child(drawer)
}

fn filters() -> Drawer {
    Drawer::new()
        .title("Filters")
        .description("Narrow what the list is showing.")
        .child(
            Col::new()
                .child(Checkbox::new("open", "Open", true))
                .child(Checkbox::new("merged", "Merged", false))
                .child(Checkbox::new("closed", "Closed", false)),
        )
}

fn remote(state: &Playground) -> Drawer {
    Drawer::new()
        .title("Edit remote")
        .description("origin · git@github.com:acme/web.git")
        .footer(Button::new("save", "Save").variant(ControlVariant::Filled))
        .footer(Button::new("cancel", "Cancel"))
        .child(FormField::new("Name").child(TextField::new(&state.name)))
        .child(
            FormField::new("URL")
                .hint("SSH or HTTPS")
                .child(TextField::new(&state.email)),
        )
}

pub const PAGE: StoryPage = StoryPage::new(
    "Drawer",
    &[
        Story::page("Overview", |state, _, _| {
            let mut sides = Specimens::new();
            for (side, name) in SIDES {
                sides = sides.child(Specimen::new(name).child(frame(
                    280.,
                    200.,
                    filters().side(side).size(px(160.)),
                )));
            }

            DocPage::new("Drawer")
                .description(
                    "A sheet that comes in from an edge and can be pushed back out. It is the \
                     Dialog's sibling, and the choice between them is about where the reader's \
                     attention has to go: a dialog takes the middle of the view and asks a \
                     question; a drawer takes a side of it and holds a panel the reader dips \
                     into — filters, a detail pane — while what they were reading stays where \
                     it was.",
                )
                .child(
                    Section::new("Sides")
                        .description(
                            "It keeps a corner only on the two sides still inside the view. A \
                             sheet flush to an edge with all four corners rounded reads as \
                             floating just off the screen rather than as attached to it.",
                        )
                        .child(sides),
                )
                .child(
                    Section::new("The three rows")
                        .description(
                            "Header and footer are the dialog's own parts rather than a second \
                             set: a drawer and a sheet in the same app are set the same way. \
                             Only the body may shrink, so a long panel keeps its heading and \
                             its actions in view.",
                        )
                        .child(frame(480., 320., remote(state))),
                )
                .into_any_element()
        }),
        Story::new("Bare", |_, _, _| {
            frame(480., 320., filters()).into_any_element()
        }),
        Story::new("Sides", |_, _, _| {
            let mut col = Col::new();
            for (side, name) in SIDES {
                col = col.child(div().child(SharedString::from(name)).child(frame(
                    280.,
                    200.,
                    filters().side(side).size(px(160.)),
                )));
            }
            col.into_any_element()
        }),
        Story::new("With footer", |state, _, _| {
            frame(480., 320., remote(state)).into_any_element()
        }),
        Story::new("Controlled", |_, _, _| {
            // A GPUI drawer is an element rather than a route, so a host shows
            // it by rendering it: there is nothing to open.
            frame(480., 320., filters()).into_any_element()
        }),
        Story::new("Wide", |_, _, _| {
            frame(
                560.,
                320.,
                Drawer::new()
                    .size(px(400.))
                    .title("Review")
                    .description("A panel that needs more room than the token gives it.")
                    .child("The width is a token, and `size` is the override."),
            )
            .into_any_element()
        }),
    ],
);
