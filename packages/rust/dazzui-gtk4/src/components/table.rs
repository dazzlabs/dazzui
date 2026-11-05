//! Composed tables accept rich cells rather than flattening them to strings.
//! Native list rows own activation and focus; selection remains host state.
use crate::{Tint, box_, prelude::*};
use gtk::{glib, subclass::prelude::*};

mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Table {
        pub rows: std::cell::OnceCell<gtk::ListBox>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Table {
        const NAME: &'static str = "DazzTable";
        type Type = super::Table;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for Table {
        fn constructed(&self) {
            self.parent_constructed();
            let table = self.obj();
            table.set_orientation(gtk::Orientation::Vertical);
            table.add_css_class("dz-table-content");
            table.set_dazz_tint(Tint::Primary);
            table.set_accessible_role(gtk::AccessibleRole::Table);
            let rows = gtk::ListBox::new();
            rows.add_css_class("dz-table-list");
            rows.set_selection_mode(gtk::SelectionMode::None);
            rows.set_activate_on_single_click(true);
            rows.set_accessible_role(gtk::AccessibleRole::RowGroup);
            table.append(&rows);
            super::super::table_layout::install(table.upcast_ref());
            self.rows.set(rows).unwrap();
        }
    }
    impl WidgetImpl for Table {
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            if ![
                gtk::DirectionType::TabForward,
                gtk::DirectionType::TabBackward,
            ]
            .contains(&direction)
            {
                return self.parent_focus(direction);
            }
            self.obj().focus_rows(direction)
        }
        fn direction_changed(&self, previous: gtk::TextDirection) {
            self.parent_direction_changed(previous);
            if let Some(rows) = self.rows.get() {
                let direction = self.obj().direction();
                rows.set_direction(direction);
                let mut child = rows.first_child();
                while let Some(row) = child {
                    child = row.next_sibling();
                    if let Some(row) = row.downcast_ref::<gtk::ListBoxRow>() {
                        super::row_direction(row, direction);
                    }
                }
            }
        }
    }
    impl BoxImpl for Table {}
}
glib::wrapper! {
    pub struct Table(ObjectSubclass<imp::Table>) @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}
impl Default for Table {
    fn default() -> Self {
        glib::Object::new()
    }
}
impl Table {
    pub fn new(tint: Tint) -> Self {
        let table = Self::default();
        table.set_dazz_tint(tint);
        table
    }
    /// Connect `row-activated` here for both pointer and native keyboard input.
    pub fn rows(&self) -> gtk::ListBox {
        self.imp().rows.get().unwrap().clone()
    }
    pub fn append_head(&self, head: &impl IsA<gtk::Widget>) {
        let row = gtk::ListBoxRow::new();
        row.add_css_class("dz-table-head-row");
        row.set_focusable(false);
        row.set_selectable(false);
        row.set_activatable(false);
        row.set_accessible_role(gtk::AccessibleRole::Presentation);
        row.set_child(Some(head));
        row_direction(&row, self.direction());
        self.rows().append(&row);
    }
    pub fn append_row(&self, row: &gtk::ListBoxRow) {
        row_direction(row, self.direction());
        self.rows().append(row);
    }
    fn focus_rows(&self, direction: gtk::DirectionType) -> bool {
        let focused = || {
            self.root()
                .and_downcast::<gtk::Window>()
                .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w))
        };
        let mut rows = vec![];
        let mut child = self.rows().first_child();
        while let Some(row) = child {
            child = row.next_sibling();
            rows.push(row);
        }
        if direction == gtk::DirectionType::TabBackward {
            rows.reverse();
        }
        let focus = focused();
        let current = rows.iter().position(|row| {
            focus
                .as_ref()
                .is_some_and(|focus| focus == row || focus.is_ancestor(row))
        });
        if let Some(index) = current {
            let row = &rows[index];
            let moved = if focus.as_ref() == Some(row) {
                // The row precedes its controls in the DOM tab order. GTK's
                // row handler otherwise re-enters itself in an inactive window.
                direction == gtk::DirectionType::TabForward
                    && row
                        .first_child()
                        .is_some_and(|child| child.child_focus(direction))
            } else {
                row.child_focus(direction)
            };
            if moved && focused() != focus {
                return true;
            }
        }
        // GTK's default list traversal stops at a non-focusable row. Headers
        // and static rows may occur between actionable rows, so keep walking.
        for row in rows.into_iter().skip(current.map_or(0, |i| i + 1)) {
            if !row.is_visible() || !row.is_sensitive() {
                continue;
            }
            let before = focused();
            if row.child_focus(direction) && focused() != before {
                return true;
            }
        }
        false
    }
}
fn row_direction(row: &gtk::ListBoxRow, direction: gtk::TextDirection) {
    row.set_direction(direction);
    if let Some(cells) = row.child() {
        cells.set_direction(direction);
        let mut child = cells.first_child();
        while let Some(cell) = child {
            child = cell.next_sibling();
            cell.set_direction(direction);
        }
    }
}

pub fn table_content(tint: Tint) -> Table {
    Table::new(tint)
}

fn cells(cells: &[gtk::Widget], align: gtk::Align) -> gtk::Box {
    let row = box_(gtk::Orientation::Horizontal, "0");
    row.set_homogeneous(true);
    for cell in cells {
        cell.set_valign(align);
        row.append(cell);
    }
    super::table_columns::install(&row);
    row
}
pub fn table_head(content: &[gtk::Widget]) -> gtk::Box {
    let head = cells(content, gtk::Align::Fill);
    head.add_css_class("dz-table-head");
    head.set_accessible_role(gtk::AccessibleRole::Row);
    head
}
pub fn table_row(content: &[gtk::Widget], interactive: bool, active: bool) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::new();
    row.add_css_class("dz-table-row");
    row.set_accessible_role(gtk::AccessibleRole::Row);
    row.set_child(Some(&cells(content, gtk::Align::Center)));
    row.set_selectable(false);
    set_table_row_interactive(&row, interactive);
    set_table_row_active(&row, active);
    row
}
pub fn set_table_row_interactive(row: &gtk::ListBoxRow, interactive: bool) {
    row.set_activatable(interactive);
    row.set_focusable(interactive);
    row.set_cursor_from_name(interactive.then_some("pointer"));
    if interactive {
        row.add_css_class("dz-interactive");
    } else {
        row.remove_css_class("dz-interactive");
    }
    set_table_row_active(row, row.has_css_class("dz-active"));
}
pub fn set_table_row_active(row: &gtk::ListBoxRow, active: bool) {
    if active {
        row.add_css_class("dz-active");
    } else {
        row.remove_css_class("dz-active");
    }
    row.update_state(&[gtk::accessible::State::Selected(
        row.is_activatable().then_some(active),
    )]);
}
pub fn table_cell(content: &impl IsA<gtk::Widget>, head: bool, align: gtk::Align) -> gtk::Box {
    let cell = box_(gtk::Orientation::Vertical, "0");
    cell.add_css_class("dz-table-cell");
    cell.set_hexpand(true);
    cell.set_valign(if head {
        gtk::Align::Fill
    } else {
        gtk::Align::Center
    });
    cell.set_accessible_role(if head {
        gtk::AccessibleRole::ColumnHeader
    } else {
        gtk::AccessibleRole::Cell
    });
    if head {
        cell.add_css_class("dz-table-column-head");
    }
    fn align_content(content: &gtk::Widget, head: bool, align: gtk::Align, rtl: bool) {
        // React's end column uses text-align:right; body rows default to
        // text-align:left. Headings without an alignment inherit direction.
        let align = match (align, head, rtl) {
            (gtk::Align::End, _, true) => gtk::Align::Start,
            (gtk::Align::Start, false, true) => gtk::Align::End,
            _ => align,
        };
        content.set_halign(align);
    }
    align_content(
        content.as_ref(),
        head,
        align,
        cell.direction() == gtk::TextDirection::Rtl,
    );
    let weak = content.downgrade();
    cell.connect_direction_changed(move |cell, _| {
        if let Some(content) = weak.upgrade() {
            align_content(
                content.as_ref(),
                head,
                align,
                cell.direction() == gtk::TextDirection::Rtl,
            );
        }
    });
    cell.append(content);
    if !head {
        super::table_cell::install(&cell);
    }
    cell
}
