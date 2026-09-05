use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};

use crate::prelude::*;

/// Where the row sits in the space it is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActionBarAlign {
    #[default]
    Start,
    /// Pushes the row to the trailing edge — a dialog footer's usual shape.
    End,
    Between,
}

/// A row of controls kept on one rhythm.
///
/// It owns no colour and no box — only the rhythm — so it takes the shared
/// control gap rather than a token of its own: the small profile's, whatever
/// the children's size, since the design draws every action row on that one
/// step.
#[derive(IntoElement)]
pub struct ActionBar {
    align: ActionBarAlign,
    children: Vec<AnyElement>,
}

impl ActionBar {
    pub fn new() -> Self {
        ActionBar {
            align: ActionBarAlign::Start,
            children: Vec::new(),
        }
    }

    pub fn align(mut self, align: ActionBarAlign) -> Self {
        self.align = align;
        self
    }
}

impl Default for ActionBar {
    fn default() -> Self {
        ActionBar::new()
    }
}

impl ParentElement for ActionBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ActionBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .items_center()
            .gap(vars.control_small_gap)
            .map(|el| match self.align {
                ActionBarAlign::Start => el,
                ActionBarAlign::End => el.justify_end(),
                ActionBarAlign::Between => el.justify_between(),
            })
            .children(self.children)
    }
}
