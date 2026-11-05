//! Round the table's combined fractional text height once. The ListBox keeps
//! its own allocation, row hit testing, focus traversal and input machinery.
use crate::{Theme, prelude::*};
use gtk::{glib, subclass::prelude::*};
fn remainder(rows: &gtk::Widget, width: i32) -> f64 {
    let mut total = 0.;
    let mut node = rows.first_child();
    while let Some(row) = node {
        node = row.next_sibling();
        if row.get_visible()
            && row.has_css_class("dz-table-row")
            && let Some(columns) = row.first_child()
        {
            let inset = 2.
                * Theme::of(&row)
                    .unwrap_or_default()
                    .dimension("spacing-5")
                    .unwrap();
            total += super::table_columns::height_correction(
                &columns,
                (f64::from(width) - inset).round().max(0.) as i32,
            );
        }
    }
    total
}
mod imp {
    use super::*;
    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::Layout)]
    pub struct Layout {
        #[property(get, set, override_interface = gtk::Orientable)]
        orientation: std::cell::Cell<gtk::Orientation>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzTableLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
        type Interfaces = (gtk::Orientable,);
    }
    impl Default for Layout {
        fn default() -> Self {
            Self {
                orientation: std::cell::Cell::new(gtk::Orientation::Vertical),
            }
        }
    }
    impl OrientableImpl for Layout {}
    #[glib::derived_properties]
    impl ObjectImpl for Layout {}
    impl LayoutManagerImpl for Layout {
        fn request_mode(&self, _: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            w: &gtk::Widget,
            o: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let Some(rows) = w.first_child() else {
                return (0, 0, -1, -1);
            };
            if o == gtk::Orientation::Horizontal {
                return rows.measure(o, -1);
            }
            let width = if for_size >= 0 {
                for_size.max(rows.measure(gtk::Orientation::Horizontal, -1).0)
            } else {
                rows.measure(gtk::Orientation::Horizontal, -1).1
            };
            let (a, b, _, _) = rows.measure(o, width);
            let delta = remainder(&rows, width);
            (
                (f64::from(a) + delta).round().max(0.) as i32,
                (f64::from(b) + delta).round().max(0.) as i32,
                -1,
                -1,
            )
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            if let Some(rows) = w.first_child() {
                rows.allocate(width, height, -1, None);
            }
        }
    }
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager, @implements gtk::Orientable;}
pub(super) fn install(table: &gtk::Box) {
    table.set_layout_manager(Some(glib::Object::new::<Layout>()));
}
