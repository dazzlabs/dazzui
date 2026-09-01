use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce,
    SharedString, Styled, Window, div, px,
};

use crate::components::SectionLabel;
use crate::prelude::*;
use crate::{Edge, Tint, colors, hairline, latched, rule};

/// How far the divider may travel.
///
/// Below the floor the labels stop fitting their rows; above the ceiling the
/// sidebar starts competing with the pane it is meant to serve. Neither is a
/// design token: they are this component's own limits, the way
/// `sidebar.tsx` states them.
const MIN_WIDTH: Pixels = px(150.);
const MAX_WIDTH: Pixels = px(320.);

/// The left workspace column.
///
/// Its width is a token rather than a layout choice because the columns of a
/// desktop shell have to agree with each other: the header strip is exactly
/// the titlebar's height, so the separator under the two steps where they
/// meet.
#[derive(IntoElement)]
pub struct Sidebar {
    id: ElementId,
    /// Content for the strip above the row list, held at exactly the
    /// titlebar's height so it lines up with the toolbar in the pane beside
    /// it.
    header: Option<AnyElement>,
    /// Let the separator on the trailing edge be dragged. Off by default: a
    /// sidebar standing on its own has no pane to trade width with, and a
    /// handle that leads nowhere is worse than no handle.
    resizable: bool,
    tint: Tint,
    footer: Option<AnyElement>,
    children: Vec<AnyElement>,
}

/// The payload a resize drag carries. It exists only so the sidebar can tell
/// its own drag from anything else being dragged over it.
struct SidebarResize;

impl Sidebar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Sidebar {
            id: id.into(),
            header: None,
            resizable: false,
            tint: Tint::Primary,
            footer: None,
            children: Vec::new(),
        }
    }

    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// The card pinned to the foot of the column, below the scrolling rows.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
}

impl ParentElement for Sidebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Sidebar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let line = hairline(vars, window);
        let Sidebar {
            id,
            header,
            resizable,
            tint,
            footer,
            children,
        } = self;

        latched(id, move |width, _, _| {
            // Zero is "never dragged": the token is the width until someone
            // moves the divider.
            let current = if width.get() > 0. {
                px(width.get())
            } else {
                vars.frame_sidebar_width
            };

            div()
                .relative()
                .flex()
                .flex_col()
                .flex_none()
                .w(current)
                .bg(vars.color_surface_muted)
                .when(resizable, |el| {
                    el.on_drag_move::<SidebarResize>({
                        let width = width.clone();
                        move |event, window, _| {
                            let next = (event.event.position.x - event.bounds.left())
                                .max(MIN_WIDTH)
                                .min(MAX_WIDTH);
                            width.set(f32::from(next));
                            window.refresh();
                        }
                    })
                })
                .when_some(header, |el, header| {
                    el.child(
                        div()
                            .flex()
                            .items_center()
                            .flex_none()
                            .h(vars.frame_titlebar_size)
                            .px(vars.spacing_4)
                            .child(header),
                    )
                })
                // Only the row list scrolls, so a pinned card stays put.
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_h_0()
                        .gap(vars.frame_nav_gap)
                        .py(vars.spacing_3_5)
                        .px(vars.spacing_2_5)
                        .children(children),
                )
                .when_some(footer, |el, footer| el.child(footer))
                .child(rule(vars.color_border, Edge::Right, line))
                .when(resizable, |el| {
                    // The grab area straddles the separator and reaches a few
                    // pixels into the pane beside it, because a one-pixel
                    // target is not a target. Nothing is drawn there — the
                    // separator itself is what lights up.
                    el.child(
                        div()
                            .id("sidebar-resizer")
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .right(-(vars.spacing_2 / 2.0))
                            .w(vars.spacing_2)
                            .cursor_col_resize()
                            .on_drag(SidebarResize, |_, _, _, cx| cx.new(|_| gpui::Empty))
                            .child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .bottom_0()
                                    .right(vars.spacing_2 / 2.0)
                                    .w(vars.spacing_px)
                                    .bg(colors::TRANSPARENT)
                                    .hover(|style| {
                                        style.bg(vars.ramp(tint).shade(600).opacity(0.6))
                                    }),
                            ),
                    )
                })
                .into_any_element()
        })
    }
}

/// A labelled run of rows.
///
/// A sidebar breaks its rows into groups rather than stacking them; the gap
/// between groups does most of the work and the label only names it.
#[derive(IntoElement)]
pub struct SidebarGroup {
    label: Option<SharedString>,
    first: bool,
    children: Vec<AnyElement>,
}

impl SidebarGroup {
    pub fn new() -> Self {
        SidebarGroup {
            label: None,
            first: false,
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// The gap between groups does the work; the first one needs none.
    pub fn first(mut self, first: bool) -> Self {
        self.first = first;
        self
    }
}

impl Default for SidebarGroup {
    fn default() -> Self {
        SidebarGroup::new()
    }
}

impl ParentElement for SidebarGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.frame_nav_gap)
            .when(!self.first, |el| el.mt(vars.spacing_3_5))
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

/// The card pinned to the foot of a sidebar — a count, a version, a queue.
#[derive(IntoElement)]
pub struct SidebarCard {
    label: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl SidebarCard {
    pub fn new() -> Self {
        SidebarCard {
            label: None,
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Default for SidebarCard {
    fn default() -> Self {
        SidebarCard::new()
    }
}

impl ParentElement for SidebarCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarCard {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_2)
            .m(vars.spacing_2_5)
            .p(vars.spacing_3)
            .rounded(vars.radius_large)
            .bg(vars.color_surface)
            .border(hairline(vars, window))
            .border_color(vars.color_border)
            .when_some(self.label, |el, label| el.child(SectionLabel::new(label)))
            .children(self.children)
    }
}
