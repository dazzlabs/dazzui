use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, Window, div,
};

use crate::prelude::*;
use crate::{
    ActionListener, ControlPart, ControlState, ControlVariant, Edge, Tint, colors, hairline,
    pressable, rule,
};

/// A flex grid rather than a table: rows are often clickable and column
/// widths are set by the design, both of which a real table fights.
#[derive(IntoElement)]
pub struct Table {
    /// Which ramp a selected row is picked out with.
    tint: Tint,
    children: Vec<AnyElement>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            tint: Tint::Primary,
            children: Vec::new(),
        }
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl Default for Table {
    fn default() -> Self {
        Table::new()
    }
}

impl ParentElement for Table {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().children(self.children)
    }
}

/// The heading row.
#[derive(IntoElement)]
pub struct TableHead {
    children: Vec<AnyElement>,
}

impl TableHead {
    pub fn new() -> Self {
        TableHead {
            children: Vec::new(),
        }
    }
}

impl Default for TableHead {
    fn default() -> Self {
        TableHead::new()
    }
}

impl ParentElement for TableHead {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableHead {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .relative()
            .flex()
            .px(vars.spacing_5)
            .bg(vars.color_surface_chrome)
            .children(self.children)
            .child(rule(
                vars.color_border,
                Edge::Bottom,
                hairline(vars, window),
            ))
    }
}

/// One row.
#[derive(IntoElement)]
pub struct TableRow {
    id: ElementId,
    active: bool,
    /// Makes the row selectable.
    interactive: bool,
    tint: Tint,
    on_click: Option<ActionListener>,
    children: Vec<AnyElement>,
}

impl TableRow {
    pub fn new(id: impl Into<ElementId>) -> Self {
        TableRow {
            id: id.into(),
            active: false,
            interactive: false,
            tint: Tint::Primary,
            on_click: None,
            children: Vec::new(),
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn on_click(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.interactive = true;
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl ParentElement for TableRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableRow {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let line = hairline(vars, window);
        let TableRow {
            id,
            active,
            interactive,
            tint,
            on_click,
            children,
        } = self;
        // Neutral, because the tinted selection sits one step lower on the
        // ramp and a tinted hover would read as the heavier of the two.
        let wash = vars.control_color(
            ControlVariant::Normal,
            ControlPart::Surface,
            tint,
            ControlState::Hovered,
        );

        pressable(id.clone(), interactive, move |state, _, _| {
            div()
                .id(id)
                .relative()
                .flex()
                .items_center()
                .py(vars.spacing_3)
                .px(vars.spacing_5)
                .type_style(vars.body_medium)
                .text_color(vars.color_content)
                .map(|el| {
                    if active {
                        // Selection is the surface wash, not the `tinted`
                        // recipe's chip fill: a selected row is a band the
                        // width of the table, and the chip's 12% reads as a
                        // block on it.
                        el.bg(vars.wash(tint, vars.wash_surface))
                    } else if state.hovered {
                        el.bg(wash)
                    } else {
                        el.bg(colors::TRANSPARENT)
                    }
                })
                .when(interactive, |el| el.cursor_pointer())
                .children(children)
                .child(rule(vars.color_border, Edge::Bottom, line))
                .when_some(on_click.filter(|_| interactive), |el, listener| {
                    el.on_click(move |_, window, cx| listener(window, cx))
                })
                .into_any_element()
        })
    }
}

/// Where a cell's content sits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableAlign {
    #[default]
    Start,
    End,
}

/// One cell.
#[derive(IntoElement)]
pub struct TableCell {
    /// Renders as a column heading.
    head: bool,
    align: TableAlign,
    /// The share of the row this column takes, where the design sets one
    /// rather than splitting the width evenly.
    share: Option<f32>,
    children: Vec<AnyElement>,
}

impl TableCell {
    pub fn new() -> Self {
        TableCell {
            head: false,
            align: TableAlign::Start,
            share: None,
            children: Vec::new(),
        }
    }

    pub fn head(mut self, head: bool) -> Self {
        self.head = head;
        self
    }

    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    pub fn share(mut self, share: f32) -> Self {
        self.share = Some(share);
        self
    }
}

impl Default for TableCell {
    fn default() -> Self {
        TableCell::new()
    }
}

impl ParentElement for TableCell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableCell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .min_w_0()
            .map(|el| match self.share {
                Some(share) => el.flex_none().w(gpui::relative(share)),
                None => el.flex_1(),
            })
            .map(|el| match self.align {
                TableAlign::Start => el.justify_start(),
                TableAlign::End => el.justify_end(),
            })
            .when(self.head, |el| {
                el.py(vars.spacing_2)
                    .type_style(vars.label_small)
                    .text_color(vars.color_content_faint)
            })
            .children(self.children)
    }
}
