//! `Components/Avatar` — the picture that stands for a person or a project.

use dazzui::{ActiveTheme, Avatar, AvatarGroup, AvatarShape, AvatarSize, Card, Tint};
use gpui::{IntoElement, ParentElement, SharedString, div};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

const SIZES: [AvatarSize; 3] = [AvatarSize::Small, AvatarSize::Medium, AvatarSize::Large];

const PEOPLE: [(&str, Tint); 5] = [
    ("Ada Lovelace", Tint::Primary),
    ("Grace Hopper", Tint::Info),
    ("Alan Turing", Tint::Success),
    ("Katherine Johnson", Tint::Warning),
    ("Barbara Liskov", Tint::Danger),
];

fn group(size: AvatarSize, ring: Option<gpui::Hsla>) -> AvatarGroup {
    let mut run = AvatarGroup::new().size(size).max(3);
    for (name, tint) in PEOPLE {
        run = run.avatar(Avatar::new(name).tint(tint).size(size));
    }
    match ring {
        Some(ring) => run.ring(ring),
        None => run,
    }
}

pub const PAGE: StoryPage = StoryPage::new(
    "Avatar",
    &[
        Story::page("Overview", |_, _, cx| {
            let vars = cx.theme().vars;

            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(
                    Specimen::new(SharedString::from(format!("{size:?}").to_lowercase()))
                        .child(Avatar::new("Ada Lovelace").size(size))
                        .child(
                            Avatar::new("DazzUI")
                                .size(size)
                                .shape(AvatarShape::Square),
                        ),
                );
            }

            DocPage::new("Avatar")
                .description(
                    "The picture that stands for a person or a project. What it is really for \
                     is the case where there is no picture: the initials are what is drawn \
                     until one has decoded and instead of it when it never does. It is not \
                     interactive and stays off the shared state machine — an avatar that lit \
                     up under the pointer would read as something you can press.",
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "Three steps off the 4px grid rather than off the control profile — \
                             an avatar is not a control, and these are the sizes a row, a \
                             header and a card actually ask for. The type moves one step of the \
                             scale per size rather than tracking the box, which grows nearly \
                             twice as fast.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Circle and square")
                        .description(
                            "A person is a circle and a thing is a square — a project, a \
                             repository. The square takes the container corner rather than the \
                             control corner: under Bright, where controls are pills, a \
                             control-cornered square would go round and stop being the shape \
                             that says “project”.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("circle")
                                        .child(Avatar::new("Ada Lovelace"))
                                        .child(Avatar::new("Grace Hopper").tint(Tint::Info)),
                                )
                                .child(
                                    Specimen::new("square")
                                        .child(
                                            Avatar::new("DazzUI").shape(AvatarShape::Square),
                                        )
                                        .child(
                                            Avatar::new("Acme Web")
                                                .shape(AvatarShape::Square)
                                                .tint(Tint::Info),
                                        ),
                                ),
                        ),
                )
                .child(
                    Section::new("Groups")
                        .description(
                            "Overlapped by a third of the box and cut out of each other by a \
                             ring in the ground colour. The ring is drawn outside the box \
                             rather than as a border on it, so it costs the run nothing.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("on paper")
                                        .child(group(AvatarSize::Medium, None)),
                                )
                                .child(Specimen::new("on a card").child(Card::new().child(group(
                                    AvatarSize::Medium,
                                    Some(vars.color_surface_muted),
                                )))),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Initials", |_, _, _| {
            let mut row = Row::new();
            for (name, tint) in PEOPLE.iter().take(4) {
                row = row.child(Avatar::new(*name).tint(*tint));
            }
            row.into_any_element()
        }),
        Story::new("Image", |_, _, _| {
            // GPUI decodes no image in a headless test and this playground
            // reaches no network, so what an image would fall back to is what
            // is drawn. It is the same element either way.
            let mut row = Row::new();
            for size in SIZES {
                row = row.child(Avatar::new("Ada Lovelace").size(size));
            }
            row.into_any_element()
        }),
        Story::new("Square", |_, _, _| {
            Row::new()
                .child(Avatar::new("DazzUI").shape(AvatarShape::Square))
                .child(
                    Avatar::new("Acme Web")
                        .shape(AvatarShape::Square)
                        .tint(Tint::Info),
                )
                .child(
                    Avatar::new("Design Tokens")
                        .shape(AvatarShape::Square)
                        .tint(Tint::Neutral),
                )
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut col = Col::new();
            for size in SIZES {
                col = col.child(
                    Row::new()
                        .child(Avatar::new("Ada Lovelace").size(size))
                        .child(
                            Avatar::new("DazzUI")
                                .size(size)
                                .shape(AvatarShape::Square),
                        ),
                );
            }
            col.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut row = Row::new();
            for tint in Tint::ALL {
                row = row.child(Avatar::new(tint.name()).tint(tint));
            }
            row.into_any_element()
        }),
        Story::new("Groups", |_, _, _| {
            let mut col = Col::new();
            for size in SIZES {
                col = col.child(div().child(group(size, None)));
            }
            col.into_any_element()
        }),
    ],
);
