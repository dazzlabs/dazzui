//! `Components/Button` — the GPUI half of
//! `apps/storybook/src/stories/button.stories.tsx`.

use dazzui::{
    ActionBar, ActionBarAlign, Button, ControlSize, ControlVariant, Icon, Spinner, SpinnerSize,
    Tint, assets::icons,
};
use gpui::{AnyElement, IntoElement, ParentElement, SharedString, Styled, div};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, capitalize};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

/// The recipes that read a ramp. `recessed` and `normal` are the neutral
/// controls: they read the theme's surface and ink roles rather than a shade
/// off a ramp, so a tint does nothing to them — a neutral chip that could turn
/// red would no longer be neutral.
pub const TINTED: [ControlVariant; 4] = [
    ControlVariant::Filled,
    ControlVariant::Tinted,
    ControlVariant::Outlined,
    ControlVariant::Plain,
];

pub const PAGE: StoryPage = StoryPage::new(
    "Button",
    &[
        Story::page("Overview", |_, _, _| overview()),
        Story::new("Normal", |_, _, _| {
            Button::new("normal", "Save changes")
                .variant(ControlVariant::Normal)
                .into_any_element()
        }),
        Story::new("Recessed", |_, _, _| {
            Button::new("recessed", "Save changes").into_any_element()
        }),
        Story::new("Filled", |_, _, _| {
            Button::new("filled", "Save changes")
                .variant(ControlVariant::Filled)
                .into_any_element()
        }),
        Story::new("Tinted", |_, _, _| {
            Button::new("tinted", "Save changes")
                .variant(ControlVariant::Tinted)
                .into_any_element()
        }),
        Story::new("Outlined", |_, _, _| {
            Button::new("outlined", "Save changes")
                .variant(ControlVariant::Outlined)
                .into_any_element()
        }),
        Story::new("Plain", |_, _, _| {
            Button::new("plain", "Save changes")
                .variant(ControlVariant::Plain)
                .into_any_element()
        }),
        Story::new("All variants", |_, _, _| {
            let mut row = Row::new();
            for variant in ControlVariant::ALL {
                row = row.child(
                    Button::new(variant.name(), capitalize(variant.name())).variant(variant),
                );
            }
            row.into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            let mut row = Row::new();
            for variant in ControlVariant::ALL {
                row = row.child(
                    Button::new(variant.name(), capitalize(variant.name()))
                        .variant(variant)
                        .disabled(true),
                );
            }
            row.into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut row = Row::new();
            for size in ControlSize::ALL {
                row = row.child(
                    Button::new(size.name(), capitalize(size.name()))
                        .size(size)
                        .variant(ControlVariant::Filled),
                );
            }
            row.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new();
            for variant in TINTED {
                let mut row = Row::new();
                for tint in Tint::ALL {
                    row = row.child(
                        Button::new(
                            SharedString::from(format!("{}-{}", variant.name(), tint.name())),
                            capitalize(tint.name()),
                        )
                        .variant(variant)
                        .tint(tint),
                    );
                }
                col = col.child(row);
            }
            col.into_any_element()
        }),
        Story::new("With an icon", |_, _, _| {
            Button::new("export", "Export")
                .variant(ControlVariant::Filled)
                .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR))
                .into_any_element()
        }),
        Story::new("With a shortcut", |_, _, _| {
            Row::new()
                .child(
                    Button::new("translate", "Translate")
                        .variant(ControlVariant::Filled)
                        .shortcut("⌘S"),
                )
                .child(Button::new("translate-all", "Translate all").shortcut("⇧⌘S"))
                .into_any_element()
        }),
        Story::new("Busy", |_, _, _| {
            div()
                .flex()
                .items_center()
                .child(
                    Button::new("busy", "Translating…")
                        .variant(ControlVariant::Filled)
                        .disabled(true)
                        .icon(Icon::new(icons::STAR_16_REGULAR)),
                )
                .child(Spinner::new().size(SpinnerSize::Small))
                .into_any_element()
        }),
        Story::new("Full width", |_, _, _| {
            Col::new()
                .width(gpui::px(280.))
                .child(
                    Button::new("continue", "Continue")
                        .variant(ControlVariant::Filled)
                        .full_width(true),
                )
                .child(Button::new("not-now", "Not now").full_width(true))
                .into_any_element()
        }),
        Story::new("Destructive", |_, _, _| {
            Button::new("delete", "Delete glossary")
                .variant(ControlVariant::Filled)
                .tint(Tint::Danger)
                .into_any_element()
        }),
        Story::new("In an action bar", |_, _, _| {
            div()
                .w(gpui::px(320.))
                .child(
                    ActionBar::new()
                        .align(ActionBarAlign::End)
                        .child(Button::new("cancel", "Cancel"))
                        .child(Button::new("export", "Export").variant(ControlVariant::Filled)),
                )
                .into_any_element()
        }),
    ],
);

/// The documented page. Every story after this one is the component alone.
fn overview() -> AnyElement {
    let mut variants = Specimens::new();
    for variant in ControlVariant::ALL {
        variants = variants.child(
            Specimen::new(variant.name())
                .child(
                    Button::new(
                        SharedString::from(format!("live-{}", variant.name())),
                        "Save changes",
                    )
                    .variant(variant),
                )
                .child(
                    Button::new(
                        SharedString::from(format!("dead-{}", variant.name())),
                        "Disabled",
                    )
                    .variant(variant)
                    .disabled(true),
                ),
        );
    }

    let mut tints = Specimens::new();
    for variant in TINTED {
        let mut row = Specimen::new(variant.name());
        for tint in Tint::ALL {
            row = row.child(
                Button::new(
                    SharedString::from(format!("tint-{}-{}", variant.name(), tint.name())),
                    capitalize(tint.name()),
                )
                .variant(variant)
                .tint(tint),
            );
        }
        tints = tints.child(row);
    }

    let mut sizes = Specimens::new();
    for size in ControlSize::ALL {
        sizes = sizes.child(
            Specimen::new(size.name())
                .child(
                    Button::new(
                        SharedString::from(format!("export-{}", size.name())),
                        "Export",
                    )
                    .size(size)
                    .variant(ControlVariant::Filled),
                )
                .child(
                    Button::new(
                        SharedString::from(format!("cancel-{}", size.name())),
                        "Cancel",
                    )
                    .size(size),
                )
                .child(
                    Button::new(
                        SharedString::from(format!("learn-{}", size.name())),
                        "Learn more",
                    )
                    .size(size)
                    .variant(ControlVariant::Plain),
                ),
        );
    }

    DocPage::new("Button")
        .description(
            "The six control recipes, each on any ramp. Hover, press and disabled are not \
             written here at all — `pressable` decides the state before the subtree is built, \
             so a button is only the box.",
        )
        .child(
            Section::new("Variants")
                .description(
                    "`normal` is the neutral control on the paper, edged by a hairline; \
                     `recessed` is the same control one step into it — the workaday grey chip a \
                     toolbar or a footer is full of, and the default. `filled` is the one action \
                     a view points at, and the only one that carries the hairline lift; \
                     `outlined` is its drawn twin; `tinted` a quiet accent chip; `plain` looks \
                     like text at rest but keeps the box, because its recipe washes on hover.",
                )
                .child(variants),
        )
        .child(
            Section::new("Tints")
                .description(
                    "Nothing below sets a colour: each row is one variant read against six \
                     ramps, and the hover and press states come from the same recipe, so a \
                     danger button darkens the way a primary one does. `recessed` and `normal` \
                     are absent on purpose — they are the neutral controls, and a tint leaves \
                     them untouched.",
                )
                .child(tints),
        )
        .child(
            Section::new("Sizes")
                .description(
                    "Button keeps no per-size block at all: the size profile carries the \
                     height, the padding, the radius and the type face, and the component reads \
                     them once. `tiny` is the icon button's step — a labelled button rarely \
                     wants it.",
                )
                .child(sizes),
        )
        .child(
            Section::new("Content")
                .description(
                    "An icon is a slot rather than a second component, so it takes the \
                     button's own colour. A shortcut is the one trailing slot, set in that \
                     colour at a lowered opacity — a key cap inside a button would read as a \
                     second control. There is no `loading` flag either: work in flight is a \
                     spinner beside the label and a button that cannot be pressed.",
                )
                .child(
                    Specimens::new()
                        .child(
                            Specimen::new("icon").child(
                                Button::new("icon", "Export")
                                    .variant(ControlVariant::Filled)
                                    .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR)),
                            ),
                        )
                        .child(
                            Specimen::new("shortcut")
                                .child(
                                    Button::new("shortcut", "Translate")
                                        .variant(ControlVariant::Filled)
                                        .shortcut("⌘S"),
                                )
                                .child(
                                    Button::new("shortcut-all", "Translate all").shortcut("⇧⌘S"),
                                ),
                        )
                        .child(
                            Specimen::new("busy")
                                .child(
                                    Button::new("busy", "Translating…")
                                        .variant(ControlVariant::Filled)
                                        .disabled(true),
                                )
                                .child(Spinner::new().size(SpinnerSize::Small)),
                        ),
                ),
        )
        .child(
            Section::new("In a row")
                .description(
                    "Where more than one button sits together the row is an `ActionBar`, so \
                     the gap comes from the control scale. The confirming action goes last and \
                     is the only filled one: two filled buttons side by side ask the reader to \
                     choose which is the default.",
                )
                .child(
                    Specimens::new()
                        .child(
                            Specimen::new("confirming").child(
                                div().w(gpui::px(320.)).child(
                                    ActionBar::new()
                                        .align(ActionBarAlign::End)
                                        .child(Button::new("row-cancel", "Cancel"))
                                        .child(
                                            Button::new("row-export", "Export")
                                                .variant(ControlVariant::Filled),
                                        ),
                                ),
                            ),
                        )
                        .child(
                            Specimen::new("destructive").child(
                                div().w(gpui::px(320.)).child(
                                    ActionBar::new()
                                        .align(ActionBarAlign::End)
                                        .child(Button::new("row-keep", "Keep"))
                                        .child(
                                            Button::new("row-delete", "Delete glossary")
                                                .variant(ControlVariant::Filled)
                                                .tint(Tint::Danger),
                                        ),
                                ),
                            ),
                        ),
                ),
        )
        .into_any_element()
}
