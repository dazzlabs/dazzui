//! Remaining KeyCap stories; shared declarations own all migrated content.
use dazzui::{KeyCap, KeyCapSize, SearchField};
use gpui::{IntoElement, ParentElement, Styled, div, px};
use crate::storybook::{Story, StoryPage};
pub const PAGE: StoryPage = StoryPage::new("KeyCap", &[
        Story::new("As a search hint", |state, _, _| {
            div()
                .w(px(320.))
                .child(
                    SearchField::new("search", &state.search)
                        .hint(KeyCap::new("⌘K").size(KeyCapSize::Small)),
                )
                .into_any_element()
        }),
]);
