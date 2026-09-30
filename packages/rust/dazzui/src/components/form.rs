//! The run of fields, and a group of them that is one question.

use std::rc::Rc;

use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};

use crate::foundation::provide;
use crate::foundation::text_input::SubmitScope;
use crate::theme::ActiveTheme;

/// The run of fields. It owns the rhythm between them, which no single field
/// can decide, and it is what Return in any field inside submits.
///
/// A form is not a surface — whatever it is inside is — so it draws no
/// paper, border or corner of its own.
#[derive(IntoElement)]
pub struct Form {
    on_submit: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}

impl Form {
    pub fn new() -> Self {
        Form {
            on_submit: None,
            children: Vec::new(),
        }
    }

    /// Called when a field inside submits it — Return in a single-line field.
    pub fn on_submit(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Rc::new(handler));
        self
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let run = div()
            .flex()
            .flex_col()
            .gap(px(t.spacing_4))
            .children(self.children);
        // A form with no handler still submits — to nothing, as a `<form>`
        // without an action does — so Return in its fields is still taken.
        let submit = self
            .on_submit
            .unwrap_or_else(|| Rc::new(|_: &mut Window, _: &mut App| {}));
        provide(SubmitScope(submit), run)
    }
}

/// A run of fields that are one question between them — a date's three
/// boxes, an address, a set of radios — named by its legend.
///
/// Not the way to head a section of unrelated fields: that is a
/// `SectionLabel`. A fieldset claims the fields inside belong together.
#[derive(IntoElement)]
pub struct Fieldset {
    legend: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl Fieldset {
    pub fn new() -> Self {
        Fieldset {
            legend: None,
            children: Vec::new(),
        }
    }

    /// Names the group.
    pub fn legend(mut self, legend: impl IntoElement) -> Self {
        self.legend = Some(legend.into_any_element());
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        // The legend takes the section label's voice: the caption face at
        // the small label's weight, set solid.
        let caption = t.caption_small;
        let legend = self.legend.map(|legend| {
            div()
                .text_color(t.color_content_subtle.hsla())
                .font_family(caption.face.family())
                .text_size(px(caption.size))
                .font_weight(t.label_small.font_weight())
                .line_height(px(caption.size))
                .child(legend)
        });
        // A browser lays a rendered legend outside the fieldset's content
        // box, which is the flex container — so the run's gap is between the
        // fields, not between the legend and the first of them.
        div().flex().flex_col().min_w_0().children(legend).child(
            div()
                .flex()
                .flex_col()
                .gap(px(t.spacing_3))
                .children(self.children),
        )
    }
}
