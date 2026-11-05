//! A wrapped hint rounds its complete native line box once. GtkLabel remains
//! responsible for shaping, wrapping and accessibility inside that box.
use crate::prelude::*;
use gtk::{glib, subclass::prelude::*};
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzHintLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {}
    impl LayoutManagerImpl for Layout {
        fn request_mode(&self, _: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let label = widget.first_child().and_downcast::<gtk::Label>().unwrap();
            if orientation == gtk::Orientation::Horizontal {
                return label.measure(orientation, for_size);
            }
            let layout = label.layout().copy();
            layout.set_width(if for_size >= 0 {
                for_size * gtk::pango::SCALE
            } else {
                -1
            });
            let height = (f64::from(layout.size().1) / f64::from(gtk::pango::SCALE)).round() as i32;
            (height, height, -1, -1)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, _: i32) {
            let label = widget.first_child().and_downcast::<gtk::Label>().unwrap();
            let natural = label.measure(gtk::Orientation::Vertical, width).1;
            label.allocate(
                width,
                natural,
                -1,
                Some(
                    gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                        0.,
                        (height - natural) as f32 / 2.,
                    )),
                ),
            );
        }
    }
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager;}
pub(super) fn wrap(label: &gtk::Label) -> gtk::Box {
    let slot = gtk::Box::new(gtk::Orientation::Vertical, 0);
    slot.set_accessible_role(gtk::AccessibleRole::Presentation);
    slot.set_layout_manager(Some(glib::Object::new::<Layout>()));
    slot.append(label);
    slot
}
