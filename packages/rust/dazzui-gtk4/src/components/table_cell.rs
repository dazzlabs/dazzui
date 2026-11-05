//! A cell is a native inline paragraph: its strut also reserves baseline space
//! around an inline control. GTK still owns each child control and its input.
use crate::{Theme, prelude::*};
use gtk::{glib, subclass::prelude::*};
fn label(w: &gtk::Widget) -> Option<gtk::Label> {
    if let Some(label) = w.downcast_ref::<gtk::Label>() {
        return Some(label.clone());
    }
    let mut child = w.first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        if let Some(found) = label(&node) {
            return Some(found);
        }
    }
    None
}
fn leading(w: &gtk::Widget, child: &gtk::Widget, width: i32) -> f64 {
    let Some(label) = label(child) else { return 0. };
    let theme = Theme::of(w).unwrap_or_default();
    let line = theme.dimension("body-medium-line-height").unwrap();
    let metrics = w.pango_context().metrics(None, None);
    let scale = f64::from(gtk::pango::SCALE);
    let baseline =
        (line + (f64::from(metrics.ascent()) - f64::from(metrics.descent())) / scale) / 2.;
    let height = child.measure(gtk::Orientation::Vertical, width).1;
    let label_height = label.measure(gtk::Orientation::Vertical, -1).1;
    let child_baseline =
        f64::from(height - label_height) / 2. + f64::from(label.layout().baseline()) / scale;
    (baseline - child_baseline).max(0.)
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
        const NAME: &'static str = "DazzTableCell";
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
            let Some(child) = w.first_child() else {
                return (0, 0, -1, -1);
            };
            if o == gtk::Orientation::Horizontal {
                return child.measure(o, -1);
            }
            let width = if for_size >= 0 {
                for_size.max(child.measure(gtk::Orientation::Horizontal, -1).0)
            } else {
                child.measure(gtk::Orientation::Horizontal, -1).1
            };
            let (a, b, _, _) = child.measure(o, width);
            let gap = leading(w, &child, width).ceil() as i32;
            let strut = Theme::of(w)
                .unwrap_or_default()
                .dimension("body-medium-line-height")
                .unwrap()
                .round() as i32;
            ((a + gap).max(strut), (b + gap).max(strut), -1, -1)
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, _: i32, _: i32) {
            let Some(child) = w.first_child() else {
                return;
            };
            let (min, nat, _, _) = child.measure(gtk::Orientation::Horizontal, -1);
            let cw = if child.halign() == gtk::Align::Fill {
                width.max(min)
            } else {
                nat.min(width).max(min)
            };
            let ch = child.measure(gtk::Orientation::Vertical, cw).1;
            let align = child.halign();
            let x = if align == gtk::Align::Center {
                (width - cw) as f32 / 2.
            } else if (align == gtk::Align::End) != (w.direction() == gtk::TextDirection::Rtl) {
                (width - cw) as f32
            } else {
                0.
            };
            child.allocate(
                cw,
                ch,
                -1,
                Some(
                    gtk::gsk::Transform::new()
                        .translate(&gtk::graphene::Point::new(x, leading(w, &child, cw) as f32)),
                ),
            );
        }
    }
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager, @implements gtk::Orientable;}
pub(super) fn install(cell: &gtk::Box) {
    cell.set_layout_manager(Some(glib::Object::new::<Layout>()));
}
