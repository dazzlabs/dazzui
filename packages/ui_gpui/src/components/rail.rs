use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
};

use crate::components::SectionLabel;
use crate::prelude::*;
use crate::{Edge, hairline, rule};

/// The second navigation column, between the sidebar and the content.
///
/// Lighter than the sidebar, because it sits closer to the content.
#[derive(IntoElement)]
pub struct Rail {
    children: Vec<AnyElement>,
}

impl Rail {
    pub fn new() -> Self {
        Rail {
            children: Vec::new(),
        }
    }
}

impl Default for Rail {
    fn default() -> Self {
        Rail::new()
    }
}

impl ParentElement for Rail {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Rail {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .relative()
            .flex()
            .flex_col()
            .flex_none()
            .w(vars.frame_rail_width)
            .gap(vars.frame_nav_gap)
            .py(vars.spacing_3_5)
            .px(vars.spacing_2_5)
            .bg(vars.color_surface)
            .children(self.children)
            .child(rule(vars.color_border, Edge::Right, hairline(vars)))
    }
}

/// A run of rail rows, optionally named.
///
/// The first run usually goes unlabelled: it is what the pane is already
/// called, and naming it twice says nothing. A second run is the one that
/// needs the label, because a gap alone leaves the reader to guess what the
/// rows below it have in common.
#[derive(IntoElement)]
pub struct RailGroup {
    label: Option<SharedString>,
    /// The gap between groups does the work; the first one needs none.
    first: bool,
    children: Vec<AnyElement>,
}

impl RailGroup {
    pub fn new() -> Self {
        RailGroup {
            label: None,
            first: false,
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn first(mut self, first: bool) -> Self {
        self.first = first;
        self
    }
}

impl Default for RailGroup {
    fn default() -> Self {
        RailGroup::new()
    }
}

impl ParentElement for RailGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for RailGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.frame_nav_gap)
            .when(!self.first, |el| el.mt(vars.spacing_1_5))
            .when_some(self.label, |el, label| {
                el.child(
                    div()
                        .py(vars.spacing_1)
                        .px(vars.spacing_2_5)
                        .child(SectionLabel::new(label).faint(true)),
                )
            })
            .children(self.children)
    }
}
