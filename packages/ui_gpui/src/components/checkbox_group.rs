use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};

use crate::components::Checkbox;
use crate::prelude::*;

/// Several checkboxes that are one answer, and the parent that turns them all
/// on or off.
///
/// The parent is a slot of its own rather than a flag on a child, and that is
/// what makes the indent honest: the group knows whether it has one, so it
/// knows whether the rest are *under* it. What the parent shows —
/// [`Checkbox::indeterminate`] when only some are checked — and what pressing
/// it does are the host's, which already holds the set.
///
/// The gap is tighter than a form's run between fields: these are the options
/// of a single question, and air between them would read as separate
/// questions.
#[derive(IntoElement)]
pub struct CheckboxGroup {
    parent: Option<Checkbox>,
    children: Vec<AnyElement>,
}

impl CheckboxGroup {
    pub fn new() -> Self {
        CheckboxGroup {
            parent: None,
            children: Vec::new(),
        }
    }

    /// The box that reads the group: checked when every child is, mixed when
    /// only some are.
    pub fn parent(mut self, parent: Checkbox) -> Self {
        self.parent = Some(parent);
        self
    }
}

impl Default for CheckboxGroup {
    fn default() -> Self {
        CheckboxGroup::new()
    }
}

impl ParentElement for CheckboxGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CheckboxGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let indented = self.parent.is_some();

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_2_5)
            .when_some(self.parent, |el, parent| el.child(parent))
            .children(self.children.into_iter().map(|child| {
                // Where the group has a parent, its children are indented to
                // where the parent's label starts — the box plus the row's
                // gap. The indent is what says these are under it; the wiring
                // alone is invisible. The medium box is the measure because a
                // parent and its children are one list, and a mixed-size list
                // is not a case this indent has to serve.
                div()
                    .when(indented, |el| {
                        el.pl(vars.checkbox_medium_box + vars.spacing_2_5)
                    })
                    .child(child)
            }))
    }
}
