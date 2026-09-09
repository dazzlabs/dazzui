//! `Components/Form` — the run of fields, and the groups inside it.

use dazzui::{
    Button, ControlVariant, Fieldset, Form, FormField, RadioGroup, TextField, TextFieldState,
};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::Playground;
use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::{Story, StoryPage};

fn plain(state: &Playground) -> Form {
    Form::new()
        .child(FormField::new("Project name").child(TextField::new(&state.name)))
        .child(FormField::new("Description").child(TextField::new(&state.notes).multiline(true)))
}

fn grouped(state: &Playground) -> Form {
    Form::new()
        .child(
            Fieldset::new()
                .legend("Billing address")
                .child(FormField::new("Street").child(TextField::new(&state.name)))
                .child(FormField::new("City").child(TextField::new(&state.email))),
        )
        .child(
            Fieldset::new().legend("Delivery").child(
                RadioGroup::new("delivery", "standard")
                    .option("standard", "Standard, 3–5 days")
                    .option("express", "Express, next day"),
            ),
        )
}

pub const PAGE: StoryPage = StoryPage::new(
    "Form",
    &[
        Story::page("Overview", |state, _, _| {
            DocPage::new("Form")
                .description(
                    "The run of fields, and the rhythm between them. The React component's \
                     other half — taking what a server said back and putting each message under \
                     the field it belongs to — has no counterpart here: a GPUI form's fields \
                     are built by the host, which already holds the errors and can hand each \
                     FormField its own. What no single field can decide is the run between \
                     them, and that is what this is.",
                )
                .child(
                    Section::new("Fieldset")
                        .description(
                            "A run of fields that are one question between them. The legend \
                             names the group, which is the whole point: it is announced before \
                             each field inside, so “Street” is heard as “Billing address, \
                             Street”.",
                        )
                        .child(div().w(px(384.)).child(grouped(state))),
                )
                .child(
                    Section::new("What it is not")
                        .description(
                            "A fieldset claims the fields inside it belong together. Heading a \
                             run of unrelated fields is a SectionLabel's job, or a \
                             PreferenceSection's — reaching for a fieldset there says something \
                             untrue.",
                        )
                        .child(div().w(px(352.)).child(plain(state))),
                )
                .into_any_element()
        }),
        Story::new("Bare", |state, _, _| {
            div().w(px(352.)).child(plain(state)).into_any_element()
        }),
        Story::new("With fieldsets", |state, _, _| {
            div().w(px(384.)).child(grouped(state)).into_any_element()
        }),
        Story::new("Server errors", |state, _, _| {
            // The host holds the messages, so a field wears its own: there is
            // no context to distribute them through, and none needed.
            div()
                .w(px(352.))
                .child(
                    Form::new()
                        .child(
                            FormField::new("Email")
                                .hint("That address is already registered")
                                .state(TextFieldState::Error)
                                .child(TextField::new(&state.email).state(TextFieldState::Error)),
                        )
                        .child(Button::new("sign-up", "Sign up").variant(ControlVariant::Filled)),
                )
                .into_any_element()
        }),
    ],
);
