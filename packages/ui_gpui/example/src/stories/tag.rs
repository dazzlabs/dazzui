//! `Components/Tag` — a Badge you can act on.

use dazzui::{Badge, BadgeSize, Tag, TagVariant, Tint};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::{Story, StoryPage};

pub const LABELS: [&str; 4] = ["bug", "good first issue", "needs design", "blocked"];

pub const PAGE: StoryPage =
    StoryPage::new(
        "Tag",
        &[
            Story::page("Overview", |_, _, _| {
                DocPage::new("Tag")
                .description(
                    "A Badge you can act on. The two are one drawing and two components rather \
                     than one component with an interactive flag, because the difference is not \
                     cosmetic: a badge reads its recipes at rest and stays out of the shared \
                     state machine, since a label that moves under the pointer reads as \
                     something you can press. A tag is something you can press — and taking the \
                     state machine is the whole reason it exists.",
                )
                .child(
                    Section::new("Beside a Badge")
                        .description(
                            "They share the capsule — the same box at the same three densities \
                             — and nothing else. A tag that is neither selectable nor removable \
                             should be a Badge.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("Badge").child(Badge::new("v1.7.0")))
                                .child(Specimen::new("Tag, selectable").child(
                                    Tag::new("doc-selected", "bug")
                                        .variant(TagVariant::Outlined)
                                        .selected(true),
                                ))
                                .child(Specimen::new("Tag, removable").child(
                                    Tag::new("doc-removable", "needs design")
                                        .on_remove(|_, _| {}),
                                )),
                        ),
                )
                .child(
                    Section::new("Selected")
                        .description(
                            "Drawn twice over: the tinted fill the component swaps the variant \
                             to, and the tint's edge. One alone is not enough — a tinted tag \
                             that was already tinted would look untouched, and an edge alone is \
                             a pixel to spot in a row of forty.",
                        )
                        .child(
                            div()
                                .flex()
                                .gap(px(6.))
                                .children(LABELS.iter().enumerate().map(|(index, label)| {
                                    Tag::new(
                                        SharedString::from(format!("doc-filter-{index}")),
                                        *label,
                                    )
                                    .variant(TagVariant::Outlined)
                                    .selected(index == 0)
                                })),
                        ),
                )
                .child(
                    Section::new("Dismissing")
                        .description(
                            "The dismiss button is its own control rather than a second job for \
                             the label: a chip that removes itself when clicked has no way to be \
                             selected, and one that does both from the same target removes \
                             things by accident. It is faint at rest and only resolves under the \
                             pointer.",
                        )
                        .child(
                            div()
                                .flex()
                                .gap(px(6.))
                                .children(LABELS.iter().enumerate().map(|(index, label)| {
                                    Tag::new(
                                        SharedString::from(format!("doc-remove-{index}")),
                                        *label,
                                    )
                                    .on_remove(|_, _| {})
                                })),
                        ),
                )
                .into_any_element()
            }),
            Story::new("Selectable", |state, this, _| {
                let on = state.filters.clone();
                div()
                    .flex()
                    .gap(px(6.))
                    .children(LABELS.iter().enumerate().map(|(index, label)| {
                        let key = SharedString::from(*label);
                        let picked = on.contains(&key);
                        Tag::new(SharedString::from(format!("filter-{index}")), *label)
                            .variant(TagVariant::Outlined)
                            .selected(picked)
                            .on_select(act(this, move |state, _, _| {
                                if picked {
                                    state.filters.retain(|one| one != &key);
                                } else {
                                    state.filters.push(key.clone());
                                }
                            }))
                    }))
                    .into_any_element()
            }),
            Story::new("Removable", |state, this, _| {
                let labels = state.labels.clone();
                div()
                    .flex()
                    .gap(px(6.))
                    .children(labels.into_iter().enumerate().map(|(index, label)| {
                        let key = label.clone();
                        Tag::new(SharedString::from(format!("remove-{index}")), label).on_remove(
                            act(this, move |state, _, _| {
                                state.labels.retain(|one| one != &key);
                            }),
                        )
                    }))
                    .into_any_element()
            }),
            Story::new("Sizes", |_, _, _| {
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.))
                    .children(BadgeSize::ALL.iter().enumerate().map(|(index, size)| {
                        Tag::new(
                            SharedString::from(format!("size-{index}")),
                            match size {
                                BadgeSize::Small => "small",
                                BadgeSize::Medium => "medium",
                                BadgeSize::Large => "large",
                            },
                        )
                        .size(*size)
                        .on_remove(|_, _| {})
                    }))
                    .into_any_element()
            }),
            Story::new("Variants", |_, _, _| {
                div()
                    .flex()
                    .gap(px(6.))
                    .children(TagVariant::ALL.iter().enumerate().map(|(index, variant)| {
                        Tag::new(
                            SharedString::from(format!("variant-{index}")),
                            match variant {
                                TagVariant::Filled => "filled",
                                TagVariant::Tinted => "tinted",
                                TagVariant::Outlined => "outlined",
                                TagVariant::Raised => "raised",
                            },
                        )
                        .variant(*variant)
                    }))
                    .into_any_element()
            }),
            Story::new("Tints", |_, _, _| {
                div()
                    .flex()
                    .gap(px(6.))
                    .children(Tint::ALL.iter().enumerate().map(|(index, tint)| {
                        Tag::new(SharedString::from(format!("tint-{index}")), tint.name())
                            .tint(*tint)
                            .on_remove(|_, _| {})
                    }))
                    .into_any_element()
            }),
            Story::new("Disabled", |_, _, _| {
                div()
                    .flex()
                    .gap(px(6.))
                    .child(
                        Tag::new("dead-select", "unavailable")
                            .selected(false)
                            .disabled(true),
                    )
                    .child(
                        Tag::new("dead-remove", "pinned")
                            .disabled(true)
                            .on_remove(|_, _| {}),
                    )
                    .into_any_element()
            }),
            Story::new("A filter bar", |state, this, _| {
                let on = state.filters.clone();
                let labels = state.labels.clone();
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(
                        div()
                            .flex()
                            .gap(px(6.))
                            .children(LABELS.iter().enumerate().map(|(index, label)| {
                                let key = SharedString::from(*label);
                                let picked = on.contains(&key);
                                Tag::new(SharedString::from(format!("bar-filter-{index}")), *label)
                                    .variant(TagVariant::Outlined)
                                    .selected(picked)
                                    .on_select(act(this, move |state, _, _| {
                                        if picked {
                                            state.filters.retain(|one| one != &key);
                                        } else {
                                            state.filters.push(key.clone());
                                        }
                                    }))
                            })),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(6.))
                            .children(labels.into_iter().enumerate().map(|(index, label)| {
                                let key = label.clone();
                                Tag::new(SharedString::from(format!("bar-remove-{index}")), label)
                                    .on_remove(act(this, move |state, _, _| {
                                        state.labels.retain(|one| one != &key);
                                    }))
                            })),
                    )
                    .into_any_element()
            }),
        ],
    );
