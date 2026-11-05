//! A text box rounds shaped advances once. GtkLabel keeps shaping and
//! accessibility, without its rounded minimum defining the painted box.
use crate::prelude::*;
use gtk::{glib, subclass::prelude::*};

fn advance(label: &gtk::Label) -> f64 {
    let layout = label.layout().copy();
    layout.set_width(-1);
    f64::from(layout.size().0) / f64::from(gtk::pango::SCALE)
}
pub(super) fn line_height(label: &gtk::Label) -> f64 {
    let layout = label.layout();
    // Keep fallback glyph overflow inside the native label rather than
    // increasing the solid CSS line box around it.
    layout
        .attributes()
        .and_then(|attrs| {
            attrs
                .iterator()
                .get(gtk::pango::AttrType::AbsoluteLineHeight)
        })
        .and_then(|attr| attr.downcast::<gtk::pango::AttrInt>().ok())
        .map_or_else(
            || f64::from(layout.size().1),
            |height| f64::from(height.value()),
        )
        / f64::from(gtk::pango::SCALE)
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct TextBox {
        pub(super) fixed_height: std::cell::Cell<bool>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for TextBox {
        const NAME: &'static str = "DazzTextBox";
        type Type = super::TextBox;
        type ParentType = gtk::Widget;
        fn class_init(class: &mut Self::Class) {
            class.set_layout_manager_type::<super::Layout>();
        }
    }
    impl ObjectImpl for TextBox {
        fn dispose(&self) {
            if let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for TextBox {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            if let Some(child) = self.obj().first_child() {
                self.obj().snapshot_child(&child, snapshot);
            }
        }
        fn grab_focus(&self) -> bool {
            self.obj().label().grab_focus()
        }
    }
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzTextBoxLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {}
    impl LayoutManagerImpl for Layout {
        fn request_mode(&self, w: &gtk::Widget) -> gtk::SizeRequestMode {
            if w.downcast_ref::<super::TextBox>().unwrap().label().wraps() {
                gtk::SizeRequestMode::HeightForWidth
            } else {
                gtk::SizeRequestMode::ConstantSize
            }
        }
        fn measure(
            &self,
            w: &gtk::Widget,
            o: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let label = w.downcast_ref::<super::TextBox>().unwrap().label();
            let size = if label.text().is_empty()
                || (o == gtk::Orientation::Vertical
                    && w.downcast_ref::<super::TextBox>()
                        .unwrap()
                        .imp()
                        .fixed_height
                        .get())
            {
                0
            } else if o == gtk::Orientation::Horizontal {
                advance(&label).round() as i32
            } else {
                let lines = if label.wraps()
                    && for_size >= 0
                    && advance(&label) > f64::from(for_size) + 0.5
                {
                    let layout = label.layout().copy();
                    layout.set_width(
                        for_size.max(label.measure(gtk::Orientation::Horizontal, -1).0)
                            * gtk::pango::SCALE,
                    );
                    layout.line_count()
                } else {
                    1
                };
                (line_height(&label) * f64::from(lines)).round() as i32
            };
            let min = if o == gtk::Orientation::Horizontal && label.wraps() {
                label.measure(o, -1).0.min(size)
            } else {
                size
            };
            (min, size, -1, -1)
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            let label = w.downcast_ref::<super::TextBox>().unwrap().label();
            let natural_width = label.measure(gtk::Orientation::Horizontal, -1).1;
            let label_width = if label.wraps() && advance(&label) > f64::from(width) + 0.5 {
                width.max(label.measure(gtk::Orientation::Horizontal, -1).0)
            } else {
                natural_width.max(width)
            };
            let natural_height = label.measure(gtk::Orientation::Vertical, label_width).1;
            label.allocate(
                label_width,
                natural_height,
                -1,
                Some(
                    gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                        0.,
                        (height - natural_height) as f32 / 2.,
                    )),
                ),
            );
        }
    }
}
glib::wrapper! {
    pub struct TextBox(ObjectSubclass<imp::TextBox>) @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager;}
impl TextBox {
    pub(crate) fn fixed_height(text: &str) -> Self {
        let w = Self::new(text);
        w.imp().fixed_height.set(true);
        w
    }
    pub fn new(text: &str) -> Self {
        let w: Self = glib::Object::new();
        w.add_css_class("dz-text-box");
        w.set_accessible_role(gtk::AccessibleRole::Presentation);
        let label = gtk::Label::new(Some(text));
        label.add_css_class("dz-text-box-label");
        label.set_xalign(0.);
        label.set_parent(&w);
        w
    }
    /// The actual GTK label, including its text and native selection APIs.
    pub fn label(&self) -> gtk::Label {
        self.first_child().and_downcast::<gtk::Label>().unwrap()
    }
    pub fn set_text(&self, text: &str) {
        self.label().set_text(text);
    }
    pub fn text(&self) -> glib::GString {
        self.label().text()
    }
    pub(super) fn width_correction(&self) -> f64 {
        let width = advance(&self.label());
        width - width.round()
    }
    pub(super) fn height_correction(&self) -> f64 {
        if self.imp().fixed_height.get() {
            return 0.;
        }
        let label = self.label();
        let height = line_height(&label) * f64::from(label.layout().line_count());
        height - height.round()
    }
}
