//! An explicit story width is capped by its parent, while GTK size_request
//! is only a minimum. Keep that constraint around the native layout.
use dazzui_gtk4::{gtk, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::cell::Cell;

mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Width {
        pub width: Cell<i32>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Width {
        const NAME: &'static str = "DazzGalleryWidth";
        type Type = super::Width;
        type ParentType = gtk::Widget;
    }
    impl ObjectImpl for Width {
        fn dispose(&self) {
            if let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Width {
        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let width = self.width.get();
            let Some(child) = self.obj().first_child() else {
                return (0, 0, -1, -1);
            };
            let minimum = child.measure(gtk::Orientation::Horizontal, -1).0;
            if orientation == gtk::Orientation::Horizontal {
                return (minimum.min(width), width, -1, -1);
            }
            child.measure(
                orientation,
                if for_size < 0 {
                    width.max(minimum)
                } else {
                    for_size.min(width).max(minimum)
                },
            )
        }
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            if let Some(child) = self.obj().first_child() {
                child.size_allocate(&gtk::Allocation::new(0, 0, width, height), baseline);
            }
        }
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            if let Some(child) = self.obj().first_child() {
                self.obj().snapshot_child(&child, snapshot);
            }
        }
    }
}
glib::wrapper! {
    pub struct Width(ObjectSubclass<imp::Width>) @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
impl Width {
    pub fn new(child: &impl IsA<gtk::Widget>, width: i32) -> Self {
        let w: Self = glib::Object::new();
        w.imp().width.set(width);
        w.add_css_class("dz-fixed-width");
        w.set_halign(gtk::Align::Start);
        child.set_parent(&w);
        w.queue_resize();
        w
    }
}
