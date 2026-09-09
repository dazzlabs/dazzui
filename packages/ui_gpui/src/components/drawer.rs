use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
};

use crate::components::{DialogFooter, DialogHeader};
use crate::prelude::*;
use crate::{Edge, hairline, rule};

/// The edge a [`Drawer`] comes in from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    #[default]
    End,
    Start,
    Top,
    Bottom,
}

/// A sheet pinned to one edge of the view.
///
/// It is the [`Dialog`](crate::Dialog)'s sibling, and the choice between them
/// is about where the reader's attention has to go rather than about the
/// screen it is on: a dialog takes the middle of the view and asks a question;
/// a drawer takes a side of it and holds a panel the reader dips into —
/// filters, a detail pane — while what they were reading stays where it was.
///
/// The header and the footer are the dialog's own parts rather than a second
/// set: a drawer and a sheet in the same app are set the same way.
#[derive(IntoElement)]
pub struct Drawer {
    side: DrawerSide,
    /// Overrides the token width — or height, on a top or bottom drawer.
    size: Option<gpui::Pixels>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    footer: Vec<AnyElement>,
    children: Vec<AnyElement>,
}

impl Drawer {
    pub fn new() -> Self {
        Drawer {
            side: DrawerSide::End,
            size: None,
            title: None,
            description: None,
            footer: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = Some(size);
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn footer(mut self, control: impl IntoElement) -> Self {
        self.footer.push(control.into_any_element());
        self
    }
}

impl Default for Drawer {
    fn default() -> Self {
        Drawer::new()
    }
}

impl ParentElement for Drawer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Drawer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let extent = self.size.unwrap_or(vars.drawer_size);
        let corner = vars.frame_window_radius;
        let horizontal = matches!(self.side, DrawerSide::Start | DrawerSide::End);
        let line = hairline(vars);

        div()
            .absolute()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(vars.color_surface_overlay)
            // It keeps a corner only on the two sides still inside the view. A
            // sheet flush to an edge with all four corners rounded reads as
            // floating just off the screen rather than as attached to it, and
            // the rule on the flush side is what separates it from what it is
            // over.
            .map(|el| match self.side {
                DrawerSide::End => el
                    .top_0()
                    .bottom_0()
                    .right_0()
                    .w(extent)
                    .rounded_tl(corner)
                    .rounded_bl(corner)
                    .child(rule(vars.color_border_strong, Edge::Left, line)),
                DrawerSide::Start => el
                    .top_0()
                    .bottom_0()
                    .left_0()
                    .w(extent)
                    .rounded_tr(corner)
                    .rounded_br(corner)
                    .child(rule(vars.color_border_strong, Edge::Right, line)),
                DrawerSide::Top => el
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(extent)
                    .rounded_bl(corner)
                    .rounded_br(corner)
                    .child(rule(vars.color_border_strong, Edge::Bottom, line)),
                DrawerSide::Bottom => el
                    .bottom_0()
                    .left_0()
                    .right_0()
                    .h(extent)
                    .rounded_tl(corner)
                    .rounded_tr(corner)
                    .child(rule(vars.color_border_strong, Edge::Top, line)),
            })
            // The grabber, on the two edges a thumb actually pushes. It is the
            // one mark that says the sheet can be pushed away.
            .when(!horizontal, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_none()
                        .justify_center()
                        .py(vars.spacing_2)
                        .child(
                            div()
                                .w(vars.spacing_9)
                                .h(vars.spacing_1)
                                .rounded(vars.radius_full)
                                .bg(vars.color_border_strong),
                        ),
                )
            })
            .when_some(self.title.clone(), |el, title| {
                el.child(
                    DialogHeader::new(title)
                        .when_some(self.description.clone(), |header, description| {
                            header.subtitle(description)
                        }),
                )
            })
            // Only the body may shrink, so a long panel keeps its heading and
            // its actions in view.
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .gap(vars.spacing_4)
                    .py(vars.spacing_4 + vars.spacing_0_5)
                    .px(vars.spacing_5)
                    .children(self.children),
            )
            .when(!self.footer.is_empty(), |el| {
                let mut footer = DialogFooter::new();
                footer.extend(self.footer);
                el.child(footer)
            })
    }
}
