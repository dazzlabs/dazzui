//! The arrangements a bare specimen needs — the counterpart of the Storybook's
//! `components/stage.tsx`.
//!
//! Only a component's `Overview` is a documented page; every other story draws
//! the component and nothing else, so all it ever needs is a row or a column
//! with a gap in it, and a width where the component has none of its own.
//! Nothing here draws.

use dazzui::prelude::*;
use dazzui::{ActiveTheme, Divider};
use gpui::{AnyElement, App, IntoElement, ParentElement, Pixels, RenderOnce, Styled, Window, div};

/// A row that wraps rather than clipping, which is what a shelf of variants
/// wants at any window width.
#[derive(IntoElement)]
pub struct Row {
    gap: Option<Pixels>,
    align_top: bool,
    children: Vec<AnyElement>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            gap: None,
            align_top: false,
            children: Vec::new(),
        }
    }

    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = Some(gap);
        self
    }

    /// For a shelf of things that are not the same height — cards, dialogs.
    pub fn align_top(mut self, align_top: bool) -> Self {
        self.align_top = align_top;
        self
    }
}

impl Default for Row {
    fn default() -> Self {
        Row::new()
    }
}

impl ParentElement for Row {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Row {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_wrap()
            .gap(self.gap.unwrap_or(vars.spacing_3))
            .map(|el| {
                if self.align_top {
                    el.items_start()
                } else {
                    el.items_center()
                }
            })
            .children(self.children)
    }
}

/// A column, for the components that stack.
#[derive(IntoElement)]
pub struct Col {
    gap: Option<Pixels>,
    width: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl Col {
    pub fn new() -> Self {
        Col {
            gap: None,
            width: None,
            children: Vec::new(),
        }
    }

    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = Some(gap);
        self
    }

    /// A width for the components that have none of their own.
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

impl Default for Col {
    fn default() -> Self {
        Col::new()
    }
}

impl ParentElement for Col {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Col {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .items_start()
            .gap(self.gap.unwrap_or(vars.spacing_3))
            .when_some(self.width, |el, width| el.w(width))
            .children(self.children)
    }
}

/// A window, for the columns that only make sense with a pane beside them.
///
/// It is a frame rather than a shell: the corner and the border are the
/// design's own window metrics, but nothing here draws a titlebar, because the
/// real one belongs to the app.
#[derive(IntoElement)]
pub struct Frame {
    width: Pixels,
    height: Pixels,
    children: Vec<AnyElement>,
}

impl Frame {
    pub fn new(width: Pixels, height: Pixels) -> Self {
        Frame {
            width,
            height,
            children: Vec::new(),
        }
    }
}

impl ParentElement for Frame {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Frame {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .w(self.width)
            .h(self.height)
            .overflow_hidden()
            .rounded(vars.frame_window_radius)
            .bg(vars.color_surface)
            .border(vars.spacing_px)
            .border_color(vars.color_border)
            .children(self.children)
    }
}

/// The empty pane a navigation column is a navigation column to.
#[derive(IntoElement)]
pub struct Pane {
    children: Vec<AnyElement>,
}

impl Pane {
    pub fn new() -> Self {
        Pane {
            children: Vec::new(),
        }
    }
}

impl Default for Pane {
    fn default() -> Self {
        Pane::new()
    }
}

impl ParentElement for Pane {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Pane {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap(vars.spacing_3)
            .p(vars.spacing_5)
            .children(self.children)
    }
}

/// A vertical rule between two runs of controls, given the height of the
/// controls beside it — CSS gives the React one its height by stretching in
/// the row, which a row of intrinsically sized children cannot do.
pub fn separator(height: Pixels) -> impl IntoElement {
    div().h(height).child(Divider::vertical())
}
