//! Equal flex columns round each cell once, and retain fractional positions.
//! GtkBox's homogeneous allocator instead gives its first cell the spare pixel.
use crate::prelude::*;
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
        const NAME: &'static str = "DazzTableColumns";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
        type Interfaces = (gtk::Orientable,);
    }
    impl Default for Layout {
        fn default() -> Self {
            Self {
                orientation: std::cell::Cell::new(gtk::Orientation::Horizontal),
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
            let nodes = children(w);
            let count = nodes.len().max(1) as i32;
            let (mut min, mut nat) = (0, 0);
            for child in nodes {
                let width = if for_size >= 0 {
                    (f64::from(for_size) / f64::from(count)).round() as i32
                } else {
                    child.measure(gtk::Orientation::Horizontal, -1).1
                };
                let width = width.max(child.measure(gtk::Orientation::Horizontal, -1).0);
                let (a, b, _, _) = child.measure(
                    o,
                    if o == gtk::Orientation::Vertical {
                        width
                    } else {
                        -1
                    },
                );
                min = min.max(a);
                nat = nat.max(b);
            }
            if o == gtk::Orientation::Horizontal {
                min *= count;
                nat *= count;
            }
            (min, nat, -1, -1)
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            let nodes = children(w);
            let ideal = f64::from(width) / nodes.len().max(1) as f64;
            for (i, child) in nodes.into_iter().enumerate() {
                let cw =
                    (ideal.round() as i32).max(child.measure(gtk::Orientation::Horizontal, -1).0);
                let ch = if child.valign() == gtk::Align::Fill {
                    height
                } else {
                    child.measure(gtk::Orientation::Vertical, cw).1
                };
                let x = if w.direction() == gtk::TextDirection::Rtl {
                    f64::from(width) - (i + 1) as f64 * ideal
                } else {
                    i as f64 * ideal
                };
                child.allocate(
                    cw,
                    ch,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            x as f32,
                            (height - ch) as f32 / 2.,
                        )),
                    ),
                );
            }
        }
    }
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager, @implements gtk::Orientable;}
pub(super) fn install(row: &gtk::Box) {
    row.set_layout_manager(Some(glib::Object::new::<Layout>()));
}
pub(super) fn height_correction(columns: &gtk::Widget, width: i32) -> f64 {
    fn remainder(widget: &gtk::Widget) -> f64 {
        if let Some(text) = widget.downcast_ref::<crate::TextBox>() {
            return text.height_correction();
        }
        if let Some(column) = widget.downcast_ref::<gtk::Box>() {
            let nodes = children(widget);
            if column.orientation() == gtk::Orientation::Vertical {
                return nodes.iter().map(remainder).sum();
            }
            let (mut native, mut ideal) = (0_f64, 0_f64);
            for child in nodes {
                let cw = child.measure(gtk::Orientation::Horizontal, -1).1;
                let height = f64::from(child.measure(gtk::Orientation::Vertical, cw).1);
                native = native.max(height);
                ideal = ideal.max(height + remainder(&child));
            }
            return ideal - native;
        }
        0.
    }
    let nodes = children(columns);
    let cell_width = (f64::from(width) / nodes.len().max(1) as f64).round() as i32;
    let (mut native, mut ideal) = (0_f64, 0_f64);
    for cell in nodes {
        let cw = cell_width.max(cell.measure(gtk::Orientation::Horizontal, -1).0);
        let height = f64::from(cell.measure(gtk::Orientation::Vertical, cw).1);
        let delta = remainder(&cell);
        native = native.max(height);
        ideal = ideal.max(height + delta);
    }
    ideal - native
}
