//! An inline code chip must not inherit GtkLabel's taller selection line as
//! its painted box. The label keeps native selection inside the ds line box.
use crate::prelude::*;
use gtk::{glib, subclass::prelude::*};

mod imp {
    use super::*;
    #[derive(Default)]
    pub struct InlineCode;
    #[glib::object_subclass]
    impl ObjectSubclass for InlineCode {
        const NAME: &'static str = "DazzInlineCode";
        type Type = super::InlineCode;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<super::CodeLayout>();
        }
    }
    impl ObjectImpl for InlineCode {
        fn dispose(&self) {
            if let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for InlineCode {
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
        const NAME: &'static str = "DazzInlineCodeLayout";
        type Type = super::CodeLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {}
    impl LayoutManagerImpl for Layout {
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            _: i32,
        ) -> (i32, i32, i32, i32) {
            let code = widget.downcast_ref::<super::InlineCode>().unwrap();
            let size = if orientation == gtk::Orientation::Horizontal {
                (f64::from(code.label().layout().size().0) / f64::from(gtk::pango::SCALE)).round()
                    as i32
            } else {
                // CSS normal leading rounds the font's ascent and descent
                // independently, rather than adding an arbitrary size inset.
                let label = code.label();
                let layout = label.layout();
                let metrics = label
                    .pango_context()
                    .metrics(layout.font_description().as_ref(), None);
                let scale = f64::from(gtk::pango::SCALE);
                ((f64::from(metrics.ascent()) / scale).round()
                    + (f64::from(metrics.descent()) / scale).round()) as i32
            };
            (size, size, -1, -1)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, _: i32) {
            let label = widget.downcast_ref::<super::InlineCode>().unwrap().label();
            let min_width = label.measure(gtk::Orientation::Horizontal, -1).0;
            let natural_height = label.measure(gtk::Orientation::Vertical, -1).1;
            label.size_allocate(
                &gtk::Allocation::new(
                    0,
                    (height - natural_height) / 2,
                    width.max(min_width),
                    natural_height,
                ),
                -1,
            );
        }
    }
}
glib::wrapper! {
    pub struct InlineCode(ObjectSubclass<imp::InlineCode>) @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
glib::wrapper! { pub struct CodeLayout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager; }
impl InlineCode {
    pub fn new(text: &str) -> Self {
        let w: Self = glib::Object::new();
        w.add_css_class("dz-code");
        w.set_valign(gtk::Align::Center);
        let label = gtk::Label::new(Some(text));
        label.add_css_class("dz-inline-code-text");
        label.set_selectable(true);
        label.set_xalign(0.);
        label.set_parent(&w);
        w
    }
    /// The selectable GTK label, for selection and native text APIs.
    pub fn label(&self) -> gtk::Label {
        self.first_child().and_downcast::<gtk::Label>().unwrap()
    }
}
pub fn inline_code(text: &str) -> InlineCode {
    InlineCode::new(text)
}
