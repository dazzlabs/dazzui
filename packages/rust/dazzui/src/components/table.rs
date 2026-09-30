//! A flex grid of rows and cells, rather than a real table: rows are often
//! clickable and column widths are set by the design.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use crate::foundation::{StyledExt, Tint, focusable, provide, scoped};
use crate::theme::ActiveTheme;

/// The ramp a table's selected row is picked out with, handed to its rows.
#[derive(Clone, Copy, Debug, PartialEq)]
struct TableTint(Tint);

#[derive(IntoElement, Default)]
pub struct Table {
    tint: Tint,
    children: Vec<AnyElement>,
}

impl Table {
    pub fn new() -> Self {
        Table::default()
    }

    /// Which ramp a selected row is picked out with.
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl ParentElement for Table {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Table {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        provide(
            TableTint(self.tint),
            div().flex().flex_col().children(self.children),
        )
    }
}

/// The heading row, on the chrome step.
#[derive(IntoElement, Default)]
pub struct TableHead {
    children: Vec<AnyElement>,
}

impl TableHead {
    pub fn new() -> Self {
        TableHead::default()
    }
}

impl ParentElement for TableHead {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableHead {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .flex()
            .px(px(t.spacing_5))
            .bg(t.color_surface_chrome.hsla())
            .border_b(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
            .children(self.children)
    }
}

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A row. Interactive once it is given a press: a neutral wash under the
/// pointer, a tab stop, and the tint's surface wash when `active`.
#[derive(IntoElement, Default)]
pub struct TableRow {
    active: bool,
    press: Option<(ElementId, OnClick)>,
    children: Vec<AnyElement>,
}

impl TableRow {
    pub fn new() -> Self {
        TableRow::default()
    }

    /// Picked out — the selection, not a press.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Makes the row selectable, and reachable from the keyboard.
    pub fn on_click(
        mut self,
        id: impl Into<ElementId>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.press = Some((id.into(), Box::new(handler)));
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
        let t = cx.theme();
        let tint = scoped::<TableTint>(cx).map_or(Tint::Primary, |tint| tint.0);
        // The surface wash, not the chip fill: a selected row is a band the
        // width of the table.
        let selected = tint.ramp(t).shade(600).alpha(t.wash_surface);
        let row = div()
            .flex()
            .items_center()
            .py(px(t.spacing_3))
            .px(px(t.spacing_5))
            .border_b(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
            .type_style(t.body_medium)
            .when(self.active, |this| this.bg(selected));
        match self.press {
            None => row.children(self.children).into_any_element(),
            Some((id, on_click)) => {
                let focus = window
                    .use_keyed_state(id.clone(), cx, |_, cx| cx.focus_handle())
                    .read(cx)
                    .clone();
                // Neutral, because the tinted selection sits one step lower
                // on the ramp and a tinted hover would read as the heavier.
                let hovered = t.control_color_normal_surface_hovered.resolve(tint.ramp(t));
                let row = row
                    .id(id)
                    .cursor_pointer()
                    .when(!self.active, |this| {
                        this.hover(move |style| style.bg(hovered))
                    })
                    .on_click(on_click);
                focusable(row, &focus, false, window, cx)
                    .children(self.children)
                    .into_any_element()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TableCellAlign {
    #[default]
    Start,
    End,
}

impl TableCellAlign {
    pub fn parse(name: &str) -> Option<TableCellAlign> {
        Some(match name {
            "start" => TableCellAlign::Start,
            "end" => TableCellAlign::End,
            _ => return None,
        })
    }
}

/// A cell: an equal share of the row. `head` sets it as a column heading.
#[derive(IntoElement, Default)]
pub struct TableCell {
    head: bool,
    align: TableCellAlign,
    children: Vec<AnyElement>,
}

impl TableCell {
    pub fn new() -> Self {
        TableCell::default()
    }

    /// Renders as a column heading.
    pub fn head(mut self, head: bool) -> Self {
        self.head = head;
        self
    }

    pub fn align(mut self, align: TableCellAlign) -> Self {
        self.align = align;
        self
    }
}

impl ParentElement for TableCell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableCell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .flex()
            .flex_row()
            .items_center()
            .flex_1()
            .flex_basis(px(0.))
            .min_w_0()
            .when(self.align == TableCellAlign::End, |this| {
                this.justify_end().text_right()
            })
            .when(self.head, |this| {
                this.py(px(t.spacing_2))
                    .text_color(t.color_content_faint.hsla())
                    .type_style(t.label_small)
            })
            .children(self.children)
    }
}
