//! `Tokens/Control` — the shared vocabulary most components read instead of
//! owning tokens of their own.

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Button, ControlSize, ControlState, ControlVariant, Icon, IconButton,
    SegmentedControl, SegmentedControlSize, Table, TableAlign, TableCell, TableHead, TableRow,
    Tint, assets::icons,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act_with;
use crate::storybook::gallery::{
    DocPage, Section, Specimen, Specimens, TokenTable, capitalize, code, px_label,
};
use crate::storybook::stage::Row;
use crate::storybook::{Story, StoryPage};

const STATES: [(&str, ControlState); 4] = [
    ("normal", ControlState::Normal),
    ("hovered", ControlState::Hovered),
    ("pressed", ControlState::Pressed),
    ("disabled", ControlState::Disabled),
];

pub const PAGE: StoryPage = StoryPage::new(
    "Control",
    &[
        Story::page("Sizing", |_, _, cx| {
            let vars = cx.theme().vars;

            let mut table = Table::new().child(
                TableHead::new()
                    .child(TableCell::new().head(true).child("Size"))
                    .child(
                        TableCell::new()
                            .head(true)
                            .align(TableAlign::End)
                            .child("Height"),
                    )
                    .child(
                        TableCell::new()
                            .head(true)
                            .align(TableAlign::End)
                            .child("Pad"),
                    )
                    .child(
                        TableCell::new()
                            .head(true)
                            .align(TableAlign::End)
                            .child("Gap"),
                    )
                    .child(
                        TableCell::new()
                            .head(true)
                            .align(TableAlign::End)
                            .child("Radius"),
                    )
                    .child(
                        TableCell::new()
                            .head(true)
                            .align(TableAlign::End)
                            .child("Face"),
                    ),
            );
            for size in ControlSize::ALL {
                let metrics = vars.metrics(size);
                table = table.child(
                    TableRow::new(size.name())
                        .child(TableCell::new().child(size.name()))
                        .child(
                            TableCell::new()
                                .align(TableAlign::End)
                                .child(px_label(metrics.size)),
                        )
                        .child(TableCell::new().align(TableAlign::End).child(format!(
                            "{} · {}",
                            px_label(metrics.padding_block),
                            px_label(metrics.padding_inline)
                        )))
                        .child(
                            TableCell::new()
                                .align(TableAlign::End)
                                .child(px_label(metrics.gap)),
                        )
                        .child(
                            TableCell::new()
                                .align(TableAlign::End)
                                .child(px_label(metrics.radius)),
                        )
                        .child(
                            TableCell::new()
                                .align(TableAlign::End)
                                .child(px_label(metrics.content.size)),
                        ),
                );
            }

            let mut controls = Specimens::new();
            for size in ControlSize::ALL {
                controls = controls.child(
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
                            IconButton::new(
                                SharedString::from(format!("star-{}", size.name())),
                                "Star",
                                Icon::new(icons::STAR_16_REGULAR),
                            )
                            .size(size),
                        ),
                );
            }

            DocPage::new("Control sizing")
                .description(
                    "Four size profiles, each a height, a gap, two pads, a corner and a type \
                     face. A sized control reads the profile once rather than restating geometry \
                     per size — Button and IconButton keep no per-size block at all, because the \
                     profile carries the design's own control type.",
                )
                .child(
                    Section::new("Profiles")
                        .description("What every control on that step is built from.")
                        .child(div().max_w(px(768.)).child(table)),
                )
                .child(
                    Section::new("On the controls themselves")
                        .description(
                            "A button and an icon button on the same step are the same height, \
                             which is the whole point of the profile.",
                        )
                        .child(controls),
                )
                .into_any_element()
        }),
        Story::page("Colors", |state, this, cx| {
            let vars = cx.theme().vars;
            let tint = Tint::ALL
                .into_iter()
                .find(|tint| tint.name() == state.mode.as_ref())
                .unwrap_or(Tint::Primary);

            let mut picker = SegmentedControl::new("recipe-tint", tint.name())
                .size(SegmentedControlSize::Small)
                .on_select(act_with(this, |state, value: &SharedString, _, _| {
                    state.mode = value.clone()
                }));
            for candidate in Tint::ALL {
                picker = picker.item(candidate.name(), capitalize(candidate.name()));
            }

            let mut resolved = div().flex().flex_col().gap(vars.spacing_5);
            for variant in ControlVariant::ALL {
                let mut row = Row::new().gap(vars.spacing_3);
                for (name, control_state) in STATES {
                    let colors = vars.control_colors(variant, tint, control_state);
                    row = row.child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(vars.spacing_1)
                            .child(
                                div()
                                    .w(vars.spacing_20)
                                    .h(vars.spacing_8)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(vars.radius_small)
                                    .bg(colors.surface)
                                    .border(vars.stroke_control)
                                    .border_color(colors.border)
                                    .type_style(vars.label_small)
                                    .text_color(colors.content)
                                    .child("Aa"),
                            )
                            .child(
                                div()
                                    .type_style(code(vars.caption_small))
                                    .text_color(vars.color_content_muted)
                                    .child(name),
                            ),
                    );
                }
                resolved = resolved.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(vars.spacing_2)
                        .child(
                            div()
                                .type_style(vars.label_medium)
                                .text_color(vars.color_content)
                                .child(variant.name()),
                        )
                        .child(row),
                );
            }

            DocPage::new("Control colors")
                .description(
                    "Interactive colours are stored as *recipes* rather than finished colours: \
                     a shade to pick off a ramp plus an alpha to apply, resolved against \
                     whichever ramp the control is tinted with. Everything below is resolved \
                     live — nothing here is a written-down colour.",
                )
                .child(
                    Section::new("The tint")
                        .description(
                            "Pick a ramp and every recipe under it re-resolves. `recessed` and \
                             `normal` do not move: they are the neutral controls, reading the \
                             theme's surface and ink roles rather than a ramp.",
                        )
                        .child(picker),
                )
                .child(
                    Section::new("Resolved")
                        .description(
                            "Surface, content and border, in each of the four states. A cell \
                             most recipes never write is filled in by the expansion — `hovered` \
                             unwritten is `normal`, `pressed` unwritten is `hovered` dimmed — so \
                             a written cell is always a decision.",
                        )
                        .child(resolved),
                )
                .into_any_element()
        }),
        Story::page("Focus", |_, _, cx| {
            let vars = cx.theme().vars;

            let mut ring = Row::new();
            for tint in Tint::ALL {
                ring = ring.child(
                    Button::new(tint.name(), capitalize(tint.name()))
                        .variant(ControlVariant::Tinted)
                        .tint(tint),
                );
            }

            DocPage::new("Focus")
                .description(
                    "The focus ring is the system accent, always. It never follows the \
                     control's tint — a ring that changed colour per control would stop meaning \
                     focus — and it hugs the control with no offset, the way macOS draws it.",
                )
                .child(
                    Section::new("Try it")
                        .description(
                            "Tab through these. Each is a different tint, and every ring is the \
                             same colour.",
                        )
                        .child(ring),
                )
                .child(
                    Section::new("Tokens")
                        .description("What the ring is made of.")
                        .child(
                            TokenTable::new()
                                .length("focus.width", vars.focus_width)
                                .length("focus.offset", vars.focus_offset)
                                .row("focus.ring.shade", vars.focus_ring_shade.to_string())
                                .row("focus.ring.alpha", format!("{:.2}", vars.focus_ring_alpha))
                                .length("stroke.control", vars.stroke_control)
                                .length("stroke.hairline", vars.stroke_hairline),
                        ),
                )
                .into_any_element()
        }),
    ],
);
