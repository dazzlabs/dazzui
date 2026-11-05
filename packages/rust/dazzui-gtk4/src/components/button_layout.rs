//! GTK keeps button activation and accessibility. Measure the caption's
//! fractional advance once and center the natural child, including when the
//! button is wider than its contents.
use crate::prelude::*;
use gtk::{glib, subclass::prelude::*};
pub(super) fn correction(widget: &gtk::Widget) -> f64 {
    if let Some(text) = widget.downcast_ref::<crate::TextBox>() {
        text.width_correction()
    } else if let Some(label) = widget.downcast_ref::<gtk::Label>() {
        if label.wraps()
            || label.ellipsize() != gtk::pango::EllipsizeMode::None
            || label.width_chars() >= 0
        {
            return 0.;
        }
        let layout = label.layout().copy();
        layout.set_width(-1);
        f64::from(layout.size().0) / f64::from(gtk::pango::SCALE)
            - f64::from(label.measure(gtk::Orientation::Horizontal, -1).1)
    } else if let Some(container) = widget.downcast_ref::<gtk::Box>()
        && container.orientation() == gtk::Orientation::Horizontal
        && !container.has_css_class("dz-key-cap")
    {
        let mut total = 0.;
        let mut child = container.first_child();
        while let Some(node) = child {
            child = node.next_sibling();
            if node.get_visible() {
                total += correction(&node);
            }
        }
        total
    } else {
        0.
    }
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Button;
    #[glib::object_subclass]
    impl ObjectSubclass for Button {
        const NAME: &'static str = "DazzButton";
        type Type = super::Button;
        type ParentType = gtk::Button;
        fn class_init(class: &mut Self::Class) {
            class.set_layout_manager_type::<super::Layout>();
        }
    }
    impl ObjectImpl for Button {}
    impl WidgetImpl for Button {}
    impl ButtonImpl for Button {}
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzButtonLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {}
    impl LayoutManagerImpl for Layout {
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            if orientation == gtk::Orientation::Vertical
                && ["dz-button", "dz-segment", "dz-pill-tab"]
                    .iter()
                    .any(|class| widget.has_css_class(class))
            {
                return (0, 0, -1, -1);
            }
            let Some(child) = widget
                .downcast_ref::<gtk::Button>()
                .unwrap()
                .child()
                .filter(|child| child.get_visible())
            else {
                return (0, 0, -1, -1);
            };
            let (min, natural, base, natural_base) = child.measure(orientation, for_size);
            if orientation == gtk::Orientation::Vertical
                && widget.has_css_class("dz-tag")
                && let Some(label) = child.downcast_ref::<gtk::Label>()
            {
                // Keep GtkButton's label property and native setter. The tag
                // rounds its line box once, while its label retains raw leading.
                let raw = super::super::text_box::line_height(label)
                    * f64::from(label.layout().line_count());
                let delta = raw - raw.ceil();
                return (
                    (f64::from(min) + delta).round() as i32,
                    (f64::from(natural) + delta).round() as i32,
                    base,
                    natural_base,
                );
            }
            if orientation != gtk::Orientation::Horizontal {
                return (min, natural, base, natural_base);
            }
            let delta = correction(&child);
            (
                ((f64::from(min) + delta).round() as i32).max(0),
                ((f64::from(natural) + delta).round() as i32).max(0),
                base,
                natural_base,
            )
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, _: i32) {
            if let Some(child) = widget
                .downcast_ref::<gtk::Button>()
                .unwrap()
                .child()
                .filter(|child| child.get_visible())
            {
                let natural_width = child.measure(gtk::Orientation::Horizontal, -1).1;
                let natural_height = child.measure(gtk::Orientation::Vertical, natural_width).1;
                child.allocate(
                    natural_width,
                    natural_height,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            (width - natural_width) as f32 / 2.,
                            (height - natural_height) as f32 / 2.,
                        )),
                    ),
                );
            }
        }
    }
}
glib::wrapper! {pub struct Button(ObjectSubclass<imp::Button>) @extends gtk::Widget,gtk::Button,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Actionable;}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager;}
pub(super) fn new(label: &str) -> gtk::Button {
    let button: Button = glib::Object::new();
    button.set_label(label);
    button.upcast()
}
pub(super) fn install(button: &impl IsA<gtk::Button>) {
    button
        .as_ref()
        .set_layout_manager(Some(glib::Object::new::<Layout>()));
}
/// Recover the fractional remainder after this layout rounded a button's
/// content. CSS padding/borders remain part of its native outer measurement.
pub(super) fn remainder(button: &gtk::Button) -> f64 {
    button.child().map_or(0., |child| {
        let width =
            f64::from(child.measure(gtk::Orientation::Horizontal, -1).1) + correction(&child);
        width - width.round()
    })
}
