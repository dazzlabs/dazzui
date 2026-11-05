//! One native paragraph keeps the note's leading space, colour and wrapping.
use crate::{Theme, prelude::*};
use gtk::{glib, subclass::prelude::*};
type Handler = (glib::WeakRef<gtk::Widget>, glib::SignalHandlerId);
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Caption {
        pub start: std::cell::Cell<u32>,
        pub handlers: std::cell::RefCell<Vec<Handler>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Caption {
        const NAME: &'static str = "DazzCheckCaption";
        type Type = super::Caption;
        type ParentType = gtk::Widget;
        fn class_init(class: &mut Self::Class) {
            class.set_layout_manager_type::<super::Layout>();
        }
    }
    impl ObjectImpl for Caption {
        fn dispose(&self) {
            self.obj().unwatch();
            if let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Caption {
        fn root(&self) {
            self.parent_root();
            self.obj().watch();
        }
        fn unroot(&self) {
            self.obj().unwatch();
            self.parent_unroot();
        }
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            self.obj().snapshot_child(&self.obj().label(), snapshot);
        }
    }
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzCheckCaptionLayout";
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
            w: &gtk::Widget,
            o: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let caption = w.downcast_ref::<super::Caption>().unwrap();
            let label = caption.label();
            if o == gtk::Orientation::Horizontal {
                let natural = caption.advance().round() as i32;
                return (label.measure(o, -1).0.min(natural), natural, -1, -1);
            }
            let width = if for_size >= 0 && caption.advance() > f64::from(for_size) + 0.5 {
                for_size
            } else {
                label.measure(gtk::Orientation::Horizontal, -1).1
            };
            label.measure(o, width)
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            let caption = w.downcast_ref::<super::Caption>().unwrap();
            let label = caption.label();
            let width = if caption.advance() <= f64::from(width) + 0.5 {
                label.measure(gtk::Orientation::Horizontal, -1).1.max(width)
            } else {
                width
            };
            let h = label.measure(gtk::Orientation::Vertical, width).1;
            label.allocate(
                width,
                h,
                -1,
                Some(
                    gtk::gsk::Transform::new()
                        .translate(&gtk::graphene::Point::new(0., (height - h) as f32 / 2.)),
                ),
            );
        }
    }
}
glib::wrapper! {pub struct Caption(ObjectSubclass<imp::Caption>) @extends gtk::Widget,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget;}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager;}
impl Caption {
    pub(super) fn new(main: &str, note: &str) -> Self {
        let w: Self = glib::Object::new();
        w.add_css_class("dz-check-caption");
        w.set_accessible_role(gtk::AccessibleRole::Presentation);
        w.imp().start.set(main.len() as u32);
        let label = gtk::Label::new(Some(&format!("{main} {note}")));
        label.set_wrap(true);
        label.set_wrap_mode(gtk::pango::WrapMode::Word);
        label.set_xalign(0.);
        label.set_parent(&w);
        w.sync_ink();
        w
    }
    pub(super) fn label(&self) -> gtk::Label {
        self.first_child().and_downcast().unwrap()
    }
    fn advance(&self) -> f64 {
        let layout = self.label().layout().copy();
        layout.set_width(-1);
        f64::from(layout.size().0) / f64::from(gtk::pango::SCALE)
    }
    pub(super) fn remainder(&self) -> f64 {
        let width = self.advance();
        width - width.round()
    }
    fn sync_ink(&self) {
        let theme = Theme::of(self).unwrap_or_default();
        let color = theme.color("color-content-faint").unwrap();
        let attrs = gtk::pango::AttrList::new();
        let mut ink = gtk::pango::AttrColor::new_foreground(
            (color.red() * 65535.).round() as u16,
            (color.green() * 65535.).round() as u16,
            (color.blue() * 65535.).round() as u16,
        );
        ink.set_start_index(self.imp().start.get());
        ink.set_end_index(u32::MAX);
        attrs.insert(ink);
        let mut alpha =
            gtk::pango::AttrInt::new_foreground_alpha((color.alpha() * 65535.).round() as u16);
        alpha.set_start_index(self.imp().start.get());
        alpha.set_end_index(u32::MAX);
        attrs.insert(alpha);
        self.label().set_attributes(Some(&attrs));
    }
    fn unwatch(&self) {
        for (w, id) in self.imp().handlers.take() {
            if let Some(w) = w.upgrade() {
                w.disconnect(id);
            }
        }
    }
    fn watch(&self) {
        self.unwatch();
        let mut w = Some(self.clone().upcast::<gtk::Widget>());
        while let Some(node) = w {
            w = node.parent();
            let weak = self.downgrade();
            let id = node.connect_css_classes_notify(move |_| {
                if let Some(w) = weak.upgrade() {
                    w.sync_ink();
                }
            });
            self.imp()
                .handlers
                .borrow_mut()
                .push((node.downgrade(), id));
        }
        self.sync_ink();
    }
}
