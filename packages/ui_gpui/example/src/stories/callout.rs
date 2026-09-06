//! `Components/Callout` — a tinted notice that stays.

use dazzui::{Button, Callout, CalloutSize, ControlVariant, Icon, Tint, assets::icons};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, capitalize, case};
use crate::storybook::stage::{Col, Frame, Pane};
use crate::storybook::{Story, StoryPage};

const SIZES: [CalloutSize; 3] = [CalloutSize::Small, CalloutSize::Medium, CalloutSize::Large];

pub const PAGE: StoryPage = StoryPage::new(
    "Callout",
    &[
        Story::page("Overview", |_, _, _| {
            let mut tints = Col::new().width(px(560.));
            for tint in Tint::ALL {
                tints = tints.child(
                    Callout::new()
                        .tint(tint)
                        .title(capitalize(tint.name()))
                        .description("The notice reads on any ramp."),
                );
            }

            let mut sizes = Col::new().width(px(560.));
            for size in SIZES {
                sizes = sizes.child(
                    Callout::new()
                        .size(size)
                        .tint(Tint::Info)
                        .title(capitalize(&case(size)))
                        .description("The box changes; the type never does."),
                );
            }

            DocPage::new("Callout")
                .description(
                    "A notice that stays on the page, where a toast passes. It always wears a \
                     tinted treatment — a notice that needed a fill louder than this would be a \
                     dialog — so the tint is the only colour axis it has.",
                )
                .child(
                    Section::new("Tints")
                        .description(
                            "The fill and the edge are washes of the tint rather than opaque \
                             steps, which is what lets the same callout sit correctly on paper, \
                             on a card and on a near-black canvas without per-theme values.",
                        )
                        .child(tints),
                )
                .child(
                    Section::new("Shapes")
                        .description(
                            "A title is optional: without one the description carries the whole \
                             notice. The leading slot takes a glyph, the action slot takes at \
                             most one action — and that action is tinted to match, so the notice \
                             reads as one object rather than as a banner with a button on it.",
                        )
                        .child(
                            Col::new()
                                .width(px(560.))
                                .child(
                                    Callout::new()
                                        .tint(Tint::Info)
                                        .description("The glossary has not been indexed yet."),
                                )
                                .child(with_an_icon())
                                .child(with_an_action("overview")),
                        ),
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "One shape, three densities. The type never changes, only the box \
                             around it — a callout that grew its type would be competing with \
                             the content it interrupts.",
                        )
                        .child(sizes),
                )
                .into_any_element()
        }),
        Story::new("Info", |_, _, _| {
            notice(
                Tint::Info,
                "A new version is available",
                "Restart to finish updating to 2.1.0.",
            )
            .into_any_element()
        }),
        Story::new("Success", |_, _, _| {
            notice(
                Tint::Success,
                "Tokens rebuilt",
                "Four platform packages were refreshed.",
            )
            .into_any_element()
        }),
        Story::new("Warning", |_, _, _| {
            notice(
                Tint::Warning,
                "Ramp moves only part-way",
                "color.warning cannot move whole under bright-dark.",
            )
            .into_any_element()
        }),
        Story::new("Danger", |_, _, _| {
            notice(
                Tint::Danger,
                "The build failed",
                "dazzui: 4 errors in sidebar.rs.",
            )
            .into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new().width(px(460.));
            for tint in Tint::ALL {
                col = col.child(
                    Callout::new()
                        .tint(tint)
                        .title(capitalize(tint.name()))
                        .description("The notice reads on any ramp."),
                );
            }
            col.into_any_element()
        }),
        Story::new("Without a title", |_, _, _| {
            div()
                .w(px(460.))
                .child(
                    Callout::new()
                        .tint(Tint::Info)
                        .description("The glossary has not been indexed yet."),
                )
                .into_any_element()
        }),
        Story::new("With an icon", |_, _, _| with_an_icon().into_any_element()),
        Story::new("With an action", |_, _, _| {
            with_an_action("story").into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut col = Col::new().width(px(460.));
            for size in SIZES {
                col = col.child(
                    Callout::new()
                        .size(size)
                        .tint(Tint::Info)
                        .title(capitalize(&case(size)))
                        .description("The box changes; the type never does."),
                );
            }
            col.into_any_element()
        }),
        Story::new("In a pane", |_, _, _| {
            Frame::new(px(560.), px(260.))
                .child(
                    Pane::new()
                        .child(
                            Callout::new()
                                .tint(Tint::Warning)
                                .title("Stale build")
                                .description("Rebuild the tokens before this page will be right."),
                        )
                        .child("Components"),
                )
                .into_any_element()
        }),
    ],
);

fn notice(tint: Tint, title: &'static str, description: &'static str) -> impl IntoElement {
    div().w(px(460.)).child(
        Callout::new()
            .tint(tint)
            .title(title)
            .description(description),
    )
}

fn with_an_icon() -> impl IntoElement {
    div().w(px(460.)).child(
        Callout::new()
            .tint(Tint::Info)
            .icon(Icon::new(icons::ALERT_16_REGULAR))
            .title("A new version is available")
            .description("Restart to finish updating to 2.1.0."),
    )
}

fn with_an_action(id: &'static str) -> impl IntoElement {
    div().w(px(460.)).child(
        Callout::new()
            .tint(Tint::Warning)
            .title("Tokens are stale")
            .description("packages/ds changed after the last build.")
            .action(
                Button::new(SharedString::from(format!("{id}-rebuild")), "Rebuild")
                    .variant(ControlVariant::Tinted)
                    .tint(Tint::Warning),
            ),
    )
}
