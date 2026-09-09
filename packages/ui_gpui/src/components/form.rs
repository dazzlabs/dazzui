use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    relative,
};

use crate::prelude::*;

/// The run of fields, and the rhythm between them.
///
/// The React component's other half — taking what a server said back and
/// putting each message under the field it belongs to — has no counterpart
/// here: a GPUI form's fields are built by the host, which already holds the
/// errors and can hand each [`FormField`](crate::FormField) its own. What no
/// single field can decide is the run between them, and that is what this is.
///
/// It resets nothing and draws nothing: a form is not a surface — whatever it
/// is inside is — so it has no paper, no border and no corner of its own.
#[derive(IntoElement)]
pub struct Form {
    children: Vec<AnyElement>,
}

impl Form {
    pub fn new() -> Self {
        Form {
            children: Vec::new(),
        }
    }
}

impl Default for Form {
    fn default() -> Self {
        Form::new()
    }
}

impl ParentElement for Form {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Form {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(vars.spacing_4)
            .children(self.children)
    }
}

/// A run of fields that are one question between them — a date's three boxes,
/// an address, a set of radios.
///
/// The legend names the group, and that is the whole point: it is announced
/// before each field inside, so "Street" is heard as "Billing address,
/// Street". A heading in a plain row looks the same and says none of that.
///
/// It is not the way to head a *section* of unrelated fields — that is a
/// [`SectionLabel`](crate::SectionLabel), or a
/// [`PreferenceSection`](crate::PreferenceSection). A fieldset claims the
/// fields inside it belong together.
#[derive(IntoElement)]
pub struct Fieldset {
    legend: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl Fieldset {
    pub fn new() -> Self {
        Fieldset {
            legend: None,
            children: Vec::new(),
        }
    }

    pub fn legend(mut self, legend: impl Into<SharedString>) -> Self {
        self.legend = Some(legend.into());
        self
    }
}

impl Default for Fieldset {
    fn default() -> Self {
        Fieldset::new()
    }
}

impl ParentElement for Fieldset {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Fieldset {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(vars.spacing_3)
            .when_some(self.legend, |el, legend| {
                el.child(
                    // The section label's voice: it names a group rather than
                    // labelling a control, so it takes the caption face and
                    // not the field label's.
                    div()
                        .type_style(vars.caption_small)
                        .font_weight(vars.label_small.weight)
                        .line_height(relative(1.0))
                        .text_color(vars.color_content_subtle)
                        .child(legend),
                )
            })
            .children(self.children)
    }
}
