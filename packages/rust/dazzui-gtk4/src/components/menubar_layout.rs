//! GTK rounds each label before summing a bar. Preserve fractional text
//! advances until the bar and its item boundaries are rounded, as flex does.
//! Only geometry changes; the native items retain focus, popups and actions.
use crate::{Theme, prelude::*};
use gtk::{glib, subclass::prelude::*};

fn items(widget: &gtk::Widget) -> Vec<(gtk::Widget, f64)> {
    let mut out = vec![];
    let mut child = widget.first_child();
    while let Some(item) = child {
        child = item.next_sibling();
        if !item.should_layout() {
            continue;
        }
        let width = item.measure(gtk::Orientation::Horizontal, -1).1;
        let correction = item
            .first_child()
            .and_downcast::<gtk::Label>()
            .map(|label| {
                let layout = label.layout().copy();
                layout.set_width(-1);
                let text = f64::from(layout.size().0) / f64::from(gtk::pango::SCALE);
                text - f64::from(label.measure(gtk::Orientation::Horizontal, -1).1)
            })
            .unwrap_or_default();
        out.push((item, f64::from(width) + correction));
    }
    out
}
fn gap(widget: &gtk::Widget) -> f64 {
    Theme::of(widget)
        .unwrap_or_default()
        .dimension("spacing-0-5")
        .unwrap()
}
fn advance(label: &gtk::Label) -> f64 {
    let layout = label.layout().copy();
    layout.set_width(-1);
    f64::from(layout.size().0) / f64::from(gtk::pango::SCALE)
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzMenubarLayout";
        type Type = super::MenubarLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {}
    #[derive(Default)]
    pub struct ItemLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for ItemLayout {
        const NAME: &'static str = "DazzMenubarItemLayout";
        type Type = super::MenubarItemLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for ItemLayout {}
    impl LayoutManagerImpl for ItemLayout {
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            _: i32,
        ) -> (i32, i32, i32, i32) {
            let label = widget.first_child().and_downcast::<gtk::Label>().unwrap();
            if orientation == gtk::Orientation::Horizontal {
                let width = advance(&label);
                (width.floor() as i32, width.ceil() as i32, -1, -1)
            } else {
                label.measure(orientation, -1)
            }
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
            if let Some(label) = widget.first_child().and_downcast::<gtk::Label>() {
                let minimum = label.measure(gtk::Orientation::Horizontal, -1).0;
                label.size_allocate(
                    &gtk::Allocation::new(0, 0, width.max(minimum), height),
                    baseline,
                );
            }
        }
    }
    impl LayoutManagerImpl for Layout {
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            _: i32,
        ) -> (i32, i32, i32, i32) {
            let items = items(widget);
            let size = if orientation == gtk::Orientation::Horizontal {
                (items.iter().map(|(_, width)| width).sum::<f64>()
                    + gap(widget) * items.len().saturating_sub(1) as f64)
                    .round() as i32
            } else {
                items
                    .iter()
                    .map(|(item, _)| item.measure(orientation, -1).1)
                    .max()
                    .unwrap_or_default()
            };
            (size, size, -1, -1)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
            let mut cursor: f64 = 0.;
            for (item, natural) in items(widget) {
                let start = cursor.round() as i32;
                cursor += natural;
                let end = cursor.round() as i32;
                let x = if widget.direction() == gtk::TextDirection::Rtl {
                    width - end
                } else {
                    start
                };
                item.size_allocate(&gtk::Allocation::new(x, 0, end - start, height), baseline);
                cursor += gap(widget);
            }
        }
    }
}
glib::wrapper! { pub struct MenubarLayout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager; }
glib::wrapper! { pub struct MenubarItemLayout(ObjectSubclass<imp::ItemLayout>) @extends gtk::LayoutManager; }

pub fn install(bar: &gtk::PopoverMenuBar) {
    let mut child = bar.first_child();
    while let Some(item) = child {
        child = item.next_sibling();
        if let Some(label) = item.first_child().and_downcast::<gtk::Label>() {
            label.pango_context().set_round_glyph_positions(false);
            item.set_layout_manager(Some(glib::Object::new::<MenubarItemLayout>()));
        }
    }
    bar.set_layout_manager(Some(glib::Object::new::<MenubarLayout>()));
}
