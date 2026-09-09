//! `Components/Dialog` — the panel, and the scrim it is centred in.

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Button, ControlVariant, Dialog, DialogBody, DialogFooter, DialogHeader,
    DialogScrim, DialogTone, FormField, TextField, Tint,
};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::on_click;
use crate::storybook::gallery::{DocPage, Section, TokenTable, px_label};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Dialog",
    &[
        Story::page("Overview", |_, _, cx| {
            let vars = cx.theme().vars;

            DocPage::new("Dialog")
                .description(
                    "A panel that blocks: the header says what is being asked, the body says \
                     what it costs, and the footer is the answer. The scrim is the content \
                     colour at its own alpha rather than black at an opacity, so it dims the \
                     same amount in every theme.",
                )
                .child(
                    Section::new("The parts")
                        .description(
                            "`DialogHeader`, `DialogBody` and `DialogFooter` are separate \
                             elements rather than slots, so a sheet that only reads can leave \
                             the footer out and a confirmation can leave the body out. The \
                             footer is an action row: the confirming action last, and the only \
                             filled one.",
                        )
                        .child(
                            Row::new()
                                .align_top(true)
                                .child(confirmation("overview"))
                                .child(Dialog::new().width(px(320.)).child(
                                    DialogBody::new().child("A sheet with only something to read."),
                                )),
                        ),
                )
                .child(
                    Section::new("Tone")
                        .description(
                            "`danger` is the one tone: the sheet's own edge takes the danger \
                             wash so the panel reads as dangerous before the reader gets to the \
                             button. The button still has to be tinted itself — the tone colours \
                             the frame, not the actions in it.",
                        )
                        .child(danger("overview")),
                )
                .child(
                    Section::new("Tokens")
                        .description(
                            "The dialog's own; the scrim is a colour and an alpha resolved \
                             together.",
                        )
                        .child(
                            TokenTable::new()
                                .length("dialog.width", vars.dialog_width)
                                .row(
                                    "dialog.scrim.alpha",
                                    format!("{:.2}", vars.dialog_scrim_alpha),
                                )
                                .row("radius.large", px_label(vars.radius_large)),
                        ),
                )
                .into_any_element()
        }),
        Story::fullscreen("A sheet", |_, _, _| {
            DialogScrim::new()
                .child(
                    Dialog::new()
                        .child(
                            DialogHeader::new("Rebuild the token graph?").subtitle(
                                "Four platform packages will be rewritten from packages/ds.",
                            ),
                        )
                        .child(
                            DialogBody::new().child(
                                "The generated files are committed, so the diff is the review.",
                            ),
                        )
                        .child(
                            DialogFooter::new()
                                .child(Button::new("sheet-cancel", "Cancel"))
                                .child(
                                    Button::new("sheet-rebuild", "Rebuild")
                                        .variant(ControlVariant::Filled),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::fullscreen("An alert", |_, _, _| {
            // `dismissible: false` is the React axis, and it switches off
            // behaviour this port does not have: a GPUI dialog is drawn where
            // it is put, with no Escape and no outside press to disable. What
            // a reader sees is the danger sheet with no way out but an answer.
            DialogScrim::new()
                .child(
                    Dialog::new()
                        .tone(DialogTone::Danger)
                        .width(px(352.))
                        .child(DialogHeader::new("Delete this glossary?"))
                        .child(DialogBody::new().child(
                            "412 terms will be removed from every project using it. There is no \
                             way out of this sheet but an answer.",
                        ))
                        .child(
                            DialogFooter::new()
                                .child(Button::new("keep", "Keep it"))
                                .child(
                                    Button::new("delete", "Delete")
                                        .variant(ControlVariant::Filled)
                                        .tint(Tint::Danger),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("A confirmation", |_, _, _| {
            confirmation("story").into_any_element()
        }),
        Story::new("Danger", |_, _, _| danger("story").into_any_element()),
        Story::new("A form", |state, _, _| {
            Dialog::new()
                .child(DialogHeader::new("New workspace").subtitle("It can be renamed later."))
                .child(
                    DialogBody::new()
                        .child(FormField::new("Name").child(TextField::new(&state.name))),
                )
                .child(
                    DialogFooter::new()
                        .child(Button::new("form-cancel", "Cancel"))
                        .child(
                            Button::new("form-create", "Create").variant(ControlVariant::Filled),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Widths", |_, _, _| {
            Col::new()
                .child(
                    Dialog::new()
                        .width(px(320.))
                        .child(DialogHeader::new("320"))
                        .child(DialogBody::new().child("A narrow sheet.")),
                )
                .child(
                    Dialog::new()
                        .child(DialogHeader::new("dialog.width"))
                        .child(DialogBody::new().child("The token, which is the default.")),
                )
                .into_any_element()
        }),
        Story::new("Body only", |_, _, _| {
            Dialog::new()
                .child(
                    DialogBody::new()
                        .child("A sheet with nothing to confirm — only something to read."),
                )
                .into_any_element()
        }),
        Story::new("Without a subtitle", |_, _, _| {
            Dialog::new()
                .child(DialogHeader::new("Rename this project"))
                .child(
                    DialogFooter::new()
                        .child(Button::new("rename-cancel", "Cancel"))
                        .child(Button::new("rename-ok", "Rename").variant(ControlVariant::Filled)),
                )
                .into_any_element()
        }),
        Story::fullscreen("Behind a trigger", |state, this, _| {
            div()
                .relative()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Button::new("open", "Show a dialog")
                        .variant(ControlVariant::Filled)
                        .on_click(on_click(this, |state| state.dialog = true)),
                )
                .when(state.dialog, |el| {
                    el.child(
                        DialogScrim::new().child(
                            Dialog::new()
                                .child(DialogHeader::new("Discard these changes?").subtitle(
                                    "The three files you edited will go back to their last \
                                     saved state.",
                                ))
                                .child(DialogBody::new().child("This cannot be undone."))
                                .child(
                                    DialogFooter::new()
                                        .child(
                                            Button::new("keep", "Keep editing")
                                                .on_click(on_click(this, |state| {
                                                    state.dialog = false
                                                })),
                                        )
                                        .child(
                                            Button::new("discard", "Discard")
                                                .variant(ControlVariant::Filled)
                                                .tint(Tint::Danger)
                                                .on_click(on_click(this, |state| {
                                                    state.dialog = false
                                                })),
                                        ),
                                ),
                        ),
                    )
                })
                .into_any_element()
        }),
    ],
);

fn confirmation(id: &'static str) -> impl IntoElement {
    Dialog::new()
        .width(px(320.))
        .child(DialogHeader::new("Discard these changes?"))
        .child(
            DialogBody::new()
                .child("The three files you edited go back to their last saved state."),
        )
        .child(
            DialogFooter::new()
                .child(Button::new(
                    gpui::SharedString::from(format!("{id}-keep")),
                    "Keep editing",
                ))
                .child(
                    Button::new(gpui::SharedString::from(format!("{id}-discard")), "Discard")
                        .variant(ControlVariant::Filled),
                ),
        )
}

fn danger(id: &'static str) -> impl IntoElement {
    Dialog::new()
        .tone(DialogTone::Danger)
        .width(px(380.))
        .child(
            DialogHeader::new("Delete the glossary?")
                .subtitle("1,204 entries. This cannot be undone."),
        )
        .child(
            DialogFooter::new()
                .child(Button::new(
                    gpui::SharedString::from(format!("{id}-danger-keep")),
                    "Keep",
                ))
                .child(
                    Button::new(
                        gpui::SharedString::from(format!("{id}-danger-delete")),
                        "Delete",
                    )
                    .variant(ControlVariant::Filled)
                    .tint(Tint::Danger),
                ),
        )
}
