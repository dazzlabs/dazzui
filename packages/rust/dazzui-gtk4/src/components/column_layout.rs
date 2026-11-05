//! A passive column sums fractional text line boxes before rounding its height.
use crate::{Theme, prelude::*};
use gtk::{glib, subclass::prelude::*};
fn children(w: &gtk::Widget) -> Vec<gtk::Widget> {
    let mut result = vec![];
    let mut child = w.first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        if node.get_visible() {
            result.push(node);
        }
    }
    result
}
fn correction(w: &gtk::Widget, width: i32) -> f64 {
    if let Some(label) = w.downcast_ref::<gtk::Label>() {
        let width = width.max(label.measure(gtk::Orientation::Horizontal, -1).0);
        let layout = label.layout().copy();
        layout.set_width(if label.wraps() {
            width * gtk::pango::SCALE
        } else {
            -1
        });
        let height = f64::from(layout.size().1) / f64::from(gtk::pango::SCALE);
        // The widget's measure also includes CSS margins and padding. Keep
        // those intact; only recover the line box's fractional remainder.
        return height - height.ceil();
    }
    if let Some(column) = w.downcast_ref::<gtk::Box>() {
        let nodes = children(w);
        if column.orientation() == gtk::Orientation::Vertical {
            return nodes.iter().map(|child| correction(child, width)).sum();
        }
        let (mut native, mut ideal) = (0_f64, 0_f64);
        for child in nodes {
            let child_width = child.measure(gtk::Orientation::Horizontal, -1).1;
            let height = f64::from(child.measure(gtk::Orientation::Vertical, child_width).1);
            native = native.max(height);
            ideal = ideal.max(height + correction(&child, child_width));
        }
        return ideal - native;
    }
    0.
}
mod imp {
    use super::*;
    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::Layout)]
    pub struct Layout {
        #[property(get, set, override_interface = gtk::Orientable)]
        orientation: std::cell::Cell<gtk::Orientation>,
        pub gap: std::cell::RefCell<String>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzColumnLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
        type Interfaces = (gtk::Orientable,);
    }
    impl Default for Layout {
        fn default() -> Self {
            Self {
                orientation: std::cell::Cell::new(gtk::Orientation::Vertical),
                gap: std::cell::RefCell::default(),
            }
        }
    }
    impl OrientableImpl for Layout {}
    #[glib::derived_properties]
    impl ObjectImpl for Layout {}
    impl Layout {
        fn gap(&self, w: &gtk::Widget) -> f64 {
            Theme::of(w)
                .unwrap_or_default()
                .dimension(&self.gap.borrow())
                .unwrap()
        }
    }
    impl LayoutManagerImpl for Layout {
        fn request_mode(&self, _: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            w: &gtk::Widget,
            o: gtk::Orientation,
            width: i32,
        ) -> (i32, i32, i32, i32) {
            let nodes = children(w);
            let (mut min, mut nat) = (0_f64, 0_f64);
            for child in &nodes {
                let width = if width >= 0 {
                    width.max(child.measure(gtk::Orientation::Horizontal, -1).0)
                } else {
                    child.measure(gtk::Orientation::Horizontal, -1).1
                };
                let (a, b, _, _) = child.measure(
                    o,
                    if o == gtk::Orientation::Vertical {
                        width
                    } else {
                        -1
                    },
                );
                if o == gtk::Orientation::Vertical {
                    let delta = correction(child, width);
                    min += f64::from(a) + delta;
                    nat += f64::from(b) + delta;
                } else {
                    min = min.max(f64::from(a));
                    nat = nat.max(f64::from(b));
                }
            }
            if o == gtk::Orientation::Vertical {
                let gap = self.gap(w) * nodes.len().saturating_sub(1) as f64;
                min += gap;
                nat += gap;
            }
            (
                min.round().max(0.) as i32,
                nat.round().max(min.round()).max(0.) as i32,
                -1,
                -1,
            )
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, _: i32, _: i32) {
            let mut y = 0.;
            for child in children(w) {
                let cw = width.max(child.measure(gtk::Orientation::Horizontal, -1).0);
                let height = child.measure(gtk::Orientation::Vertical, cw).1;
                child.allocate(
                    cw,
                    height,
                    -1,
                    Some(
                        gtk::gsk::Transform::new()
                            .translate(&gtk::graphene::Point::new(0., y as f32)),
                    ),
                );
                y += f64::from(height) + correction(&child, cw) + self.gap(w);
            }
        }
    }
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager, @implements gtk::Orientable;}
pub(super) fn install(column: &gtk::Box, gap: &str) {
    let layout: Layout = glib::Object::new();
    *layout.imp().gap.borrow_mut() = gap.into();
    column.set_layout_manager(Some(layout));
}
