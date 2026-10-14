//! A row of controls kept on one rhythm.

use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};

use crate::theme::ActiveTheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ActionBarAlign {
    #[default]
    Start,
    /// The row pushed to the trailing edge — a dialog footer's usual shape.
    End,
    Between,
}

impl ActionBarAlign {
    pub fn parse(name: &str) -> Option<ActionBarAlign> {
        Some(match name {
            "start" => ActionBarAlign::Start,
            "end" => ActionBarAlign::End,
            "between" => ActionBarAlign::Between,
            _ => return None,
        })
    }
}

/// It owns no colour and no box — only the rhythm, the small profile's gap
/// whatever the size of the controls in it.
#[derive(IntoElement, Default)]
pub struct ActionBar {
    align: ActionBarAlign,
    children: Vec<AnyElement>,
}

impl ActionBar {
    pub fn new() -> Self {
        ActionBar::default()
    }

    pub fn align(mut self, align: ActionBarAlign) -> Self {
        self.align = align;
        self
    }
}

impl ParentElement for ActionBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for ActionBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let row = div().flex().items_center().gap(px(t.control_small_gap));
        match self.align {
            ActionBarAlign::Start => row,
            ActionBarAlign::End => row.justify_end(),
            ActionBarAlign::Between => row.justify_between(),
        }
        .children(self.children)
    }
}
