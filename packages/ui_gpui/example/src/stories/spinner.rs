//! `Components/Spinner` — work in flight, with no end in sight.

use dazzui::{Button, Callout, Card, ControlVariant, Spinner, SpinnerSize, Tint};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, case};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

const SIZES: [SpinnerSize; 3] = [SpinnerSize::Small, SpinnerSize::Medium, SpinnerSize::Large];

pub const PAGE: StoryPage = StoryPage::new(
    "Spinner",
    &[
        Story::page("Overview", |_, _, _| {
            let mut sizes = Specimens::new();
            for size in SIZES {
                let mut row = Specimen::new(case(size));
                for tint in Tint::ALL {
                    row = row.child(Spinner::new().size(size).tint(tint));
                }
                sizes = sizes.child(row);
            }

            DocPage::new("Spinner")
                .description(
                    "The indeterminate mark. Where a `Progress` bar without a value sweeps \
                     along a track, a spinner has no track at all — it is what a label, a button \
                     or a toast carries while it waits.",
                )
                .child(
                    Section::new("Sizes and tints")
                        .description(
                            "Three sizes; the track is the tint at a quarter, which is the one \
                             value the React stylesheet holds as a literal and this port keeps \
                             as a named constant beside the same reason.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("On an accent")
                        .description(
                            "`on_accent` is the spinner drawn to sit on a filled surface rather \
                             than on paper. A spinner beside a *disabled* filled button is on \
                             the groove instead, so it takes the ordinary colours — which is why \
                             this is a flag rather than something the component guesses.",
                        )
                        .child(
                            Row::new()
                                .child(Spinner::new().on_accent(true))
                                .child(Spinner::new()),
                        ),
                )
                .child(
                    Section::new("Where it lands")
                        .description(
                            "Beside a button's label, in a callout's leading slot, in a toast, \
                             or on its own in a panel that is still filling.",
                        )
                        .child(Col::new().child(in_a_callout()).child(in_a_panel())),
                )
                .into_any_element()
        }),
        Story::new("Beside a button", |_, _, _| {
            Row::new()
                .child(
                    Button::new("busy", "Translating…")
                        .variant(ControlVariant::Filled)
                        .disabled(true),
                )
                .child(Spinner::new().size(SpinnerSize::Small))
                .into_any_element()
        }),
        Story::new("In a panel", |_, _, _| in_a_panel().into_any_element()),
    ],
);

fn in_a_callout() -> impl IntoElement {
    div().w(px(420.)).child(
        Callout::new()
            .tint(Tint::Info)
            .icon(Spinner::new().size(SpinnerSize::Small).tint(Tint::Info))
            .title("Rebuilding")
            .description("Four platform packages are being rewritten."),
    )
}

fn in_a_panel() -> impl IntoElement {
    div().w(px(320.)).child(
        Card::new().child(
            Row::new()
                .child(Spinner::new())
                .child(div().child("Resolving packages…")),
        ),
    )
}
