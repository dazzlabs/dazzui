//! `Components/SearchField` — a text field that knows what it is for.

use dazzui::{EmptyState, KeyCap, KeyCapSize, NavItem, SearchField, TextFieldSize, Tint};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::Playground;
use crate::storybook::gallery::{DocPage, Section, capitalize, case};
use crate::storybook::stage::{Col, Frame, Pane};
use crate::storybook::{Story, StoryPage, component_names};

const SIZES: [TextFieldSize; 3] = [
    TextFieldSize::Small,
    TextFieldSize::Medium,
    TextFieldSize::Large,
];

pub const PAGE: StoryPage = StoryPage::new(
    "SearchField",
    &[
        Story::page("Overview", |state, _, _| {
            DocPage::new("SearchField")
                .description(
                    "The text field with a glyph in front of it, a clear button behind, and a \
                     slot for the key that focuses it. It is the same box as `TextField` — same \
                     heights, same states — so a search field in a toolbar lines up with \
                     everything else on that row.",
                )
                .child(
                    Section::new("States")
                        .description(
                            "Empty, and with a query. The clear button appears only once there \
                             is something to clear, so an empty field is not offering an action \
                             that does nothing.",
                        )
                        .child(
                            div()
                                .w(px(420.))
                                .child(SearchField::new("overview", &state.search)),
                        ),
                )
                .child(
                    Section::new("The hint")
                        .description(
                            "The trailing slot takes the key that focuses the field. It is a \
                             `KeyCap` at the small step — a hint rather than a control, so it \
                             never answers the pointer.",
                        )
                        .child(
                            div().w(px(320.)).child(
                                SearchField::new("hinted", &state.search)
                                    .hint(KeyCap::new("⌘K").size(KeyCapSize::Small)),
                            ),
                        ),
                )
                .child(
                    Section::new("Filtering")
                        .description(
                            "What a search field is actually for. This one filters the \
                             playground's own component index, and falls back to an `EmptyState` \
                             when nothing matches.",
                        )
                        .child(filtering(state)),
                )
                .into_any_element()
        }),
        Story::new("Empty", |state, _, _| {
            div()
                .w(px(320.))
                .child(SearchField::new("empty", &state.search))
                .into_any_element()
        }),
        Story::new("With a hint", |state, _, _| {
            div()
                .w(px(320.))
                .child(
                    SearchField::new("hint", &state.search)
                        .hint(KeyCap::new("⌘K").size(KeyCapSize::Small)),
                )
                .into_any_element()
        }),
        Story::new("Sizes", |state, _, _| {
            let mut col = Col::new().width(px(320.));
            for size in SIZES {
                col = col.child(
                    SearchField::new(SharedString::from(case(size)), &state.search).size(size),
                );
            }
            col.into_any_element()
        }),
        Story::new("Tints", |state, _, _| {
            let mut col = Col::new().width(px(320.));
            for tint in Tint::ALL {
                col = col.child(
                    SearchField::new(tint.name(), &state.search)
                        .tint(tint)
                        .clear_label(capitalize(tint.name())),
                );
            }
            col.into_any_element()
        }),
        Story::new("Filtering a list", |state, _, _| {
            filtering(state).into_any_element()
        }),
        Story::new("In a pane header", |state, _, _| {
            Frame::new(px(560.), px(300.))
                .child(Pane::new().child(filtering(state)))
                .into_any_element()
        }),
    ],
);

/// The field doing its job: what it filters is the playground's own index.
fn filtering(state: &Playground) -> impl IntoElement {
    let needle = state.search_needle();
    let matches: Vec<&'static str> = component_names()
        .filter(|name| needle.is_empty() || name.to_lowercase().contains(&needle))
        .take(5)
        .collect();

    let mut col = Col::new()
        .width(px(320.))
        .child(SearchField::new("filtering", &state.search));

    if matches.is_empty() {
        col = col.child(EmptyState::new("Nothing matched"));
    } else {
        for name in matches {
            col = col.child(NavItem::new(
                SharedString::from(format!("match-{name}")),
                name,
            ));
        }
    }

    col
}
