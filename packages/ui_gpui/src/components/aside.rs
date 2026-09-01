use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};

use crate::prelude::*;
use crate::{Edge, hairline, rule};

/// The trailing information column.
///
/// It carries what the current selection *is*, where the sidebar and the rail
/// carry what you can navigate to — which is why it hangs off the opposite
/// edge and takes no rows. It shares the sidebar's surface rather than the
/// rail's, so a window reads as content between two quieter edges rather than
/// as three different greys.
#[derive(IntoElement)]
pub struct Aside {
    children: Vec<AnyElement>,
}

impl Aside {
    pub fn new() -> Self {
        Aside {
            children: Vec::new(),
        }
    }
}

impl Default for Aside {
    fn default() -> Self {
        Aside::new()
    }
}

impl ParentElement for Aside {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Aside {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .relative()
            .flex()
            .flex_col()
            .flex_none()
            .w(vars.frame_aside_width)
            .gap(vars.spacing_5)
            .p(vars.spacing_4)
            .bg(vars.color_surface_muted)
            .children(self.children)
            .child(rule(vars.color_border, Edge::Left, hairline(vars, window)))
    }
}
