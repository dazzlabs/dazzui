//! Retain GTK's check/radio nodes and behaviour, rounding a complete row once.
use crate::prelude::*;
use gtk::{glib, subclass::prelude::*};
fn children(widget: &gtk::Widget) -> Vec<gtk::Widget> {
    let mut result = vec![];
    let mut child = widget.first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        if node.get_visible() {
            result.push(node);
        }
    }
    result
}
pub(super) fn correction(widget: &gtk::Widget) -> f64 {
    if let Some(caption) = widget.downcast_ref::<super::check_caption::Caption>() {
        caption.remainder()
    } else if let Some(column) = widget.downcast_ref::<gtk::Box>()
        && column.orientation() == gtk::Orientation::Vertical
    {
        let (mut native, mut ideal) = (0_f64, 0_f64);
        for child in children(widget) {
            let width = f64::from(child.measure(gtk::Orientation::Horizontal, -1).1);
            native = native.max(width);
            ideal = ideal.max(width + correction(&child));
        }
        ideal - native
    } else if let Some(label) = widget.downcast_ref::<gtk::Label>()
        && label.wraps()
        && label.width_chars() < 0
        && label.ellipsize() == gtk::pango::EllipsizeMode::None
    {
        let layout = label.layout().copy();
        layout.set_width(-1);
        f64::from(layout.size().0) / f64::from(gtk::pango::SCALE)
            - f64::from(label.measure(gtk::Orientation::Horizontal, -1).1)
    } else {
        super::button_layout::correction(widget)
    }
}
mod imp {
    use super::*;
    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::Layout)]
    pub struct Layout {
        #[property(get, set, override_interface = gtk::Orientable)]
        orientation: std::cell::Cell<gtk::Orientation>,
        pub gap: std::cell::RefCell<Option<String>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzCheckLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
        type Interfaces = (gtk::Orientable,);
    }
    impl Default for Layout {
        fn default() -> Self {
            Self {
                orientation: std::cell::Cell::new(gtk::Orientation::Horizontal),
                gap: std::cell::RefCell::default(),
            }
        }
    }
    impl OrientableImpl for Layout {}
    #[glib::derived_properties]
    impl ObjectImpl for Layout {}
    impl Layout {
        fn gap(&self, w: &gtk::Widget) -> Option<f64> {
            self.gap.borrow().as_ref().map(|key| {
                crate::Theme::of(w)
                    .unwrap_or_default()
                    .dimension(&format!("spacing-{key}"))
                    .unwrap()
            })
        }
        fn row(&self, w: &gtk::Widget, width: i32, gap: f64) -> Vec<(gtk::Widget, f64)> {
            let children = children(w);
            let mut widths: Vec<_> = children
                .iter()
                .map(|child| {
                    f64::from(child.measure(gtk::Orientation::Horizontal, -1).1) + correction(child)
                })
                .collect();
            if width >= 0 {
                let extra = f64::from(width)
                    - widths.iter().sum::<f64>()
                    - gap * widths.len().saturating_sub(1) as f64;
                let expand = children.iter().filter(|c| c.hexpands()).count();
                if expand > 0 {
                    for (child, width) in children.iter().zip(&mut widths) {
                        if child.hexpands() {
                            *width = (*width + extra / expand as f64)
                                .max(f64::from(child.measure(gtk::Orientation::Horizontal, -1).0));
                        }
                    }
                }
            }
            children.into_iter().zip(widths).collect()
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
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let children = children(w);
            if let Some(gap) = self.gap(w) {
                if o == gtk::Orientation::Horizontal {
                    let rows = self.row(w, -1, gap);
                    let natural = (rows.iter().map(|(_, width)| width).sum::<f64>()
                        + gap * rows.len().saturating_sub(1) as f64)
                        .round() as i32;
                    let minimum = (rows
                        .iter()
                        .map(|(child, _)| f64::from(child.measure(o, -1).0))
                        .sum::<f64>()
                        + gap * rows.len().saturating_sub(1) as f64)
                        .round() as i32;
                    return (minimum.min(natural), natural, -1, -1);
                }
                let height = self
                    .row(w, for_size, gap)
                    .into_iter()
                    .map(|(child, width)| child.measure(o, width.ceil() as i32).1)
                    .max()
                    .unwrap_or(0);
                return (height, height, -1, -1);
            }
            if o == gtk::Orientation::Horizontal {
                let (mut min, mut nat) = (0., 0.);
                for child in children {
                    let (a, b, _, _) = child.measure(o, for_size);
                    let delta = correction(&child);
                    min += f64::from(a) + delta;
                    nat += f64::from(b) + delta;
                }
                let natural = nat.round().max(0.) as i32;
                return ((min.round().max(0.) as i32).min(natural), natural, -1, -1);
            }
            let widths: Vec<_> = children
                .iter()
                .map(|c| c.measure(gtk::Orientation::Horizontal, -1).1)
                .collect();
            let fixed = widths.first().copied().unwrap_or(0);
            let mut min = 0;
            let mut nat = 0;
            for (i, child) in children.iter().enumerate() {
                let width = if i > 0 && for_size >= 0 {
                    (for_size - fixed).max(0)
                } else {
                    widths[i]
                };
                let minimum = child.measure(gtk::Orientation::Horizontal, -1).0;
                let (a, b, _, _) = child.measure(o, width.max(minimum));
                min = min.max(a);
                nat = nat.max(b);
            }
            (min, nat, -1, -1)
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            if let Some(gap) = self.gap(w) {
                let mut x = 0.;
                for (child, ideal) in self.row(w, width, gap) {
                    let cw = ideal.ceil() as i32;
                    let ch = child.measure(gtk::Orientation::Vertical, cw).1;
                    let cx = if w.direction() == gtk::TextDirection::Rtl {
                        f64::from(width) - x - ideal
                    } else {
                        x
                    };
                    child.allocate(
                        cw,
                        ch,
                        -1,
                        Some(
                            gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                                cx as f32,
                                (height - ch) as f32 / 2.,
                            )),
                        ),
                    );
                    x += ideal + gap;
                }
                return;
            }
            let mut x = 0.;
            for (i, child) in children(w).into_iter().enumerate() {
                let (minimum, natural, _, _) = child.measure(gtk::Orientation::Horizontal, -1);
                let ideal = f64::from(natural) + correction(&child);
                let available = if i > 0 {
                    (f64::from(width) - x).max(0.)
                } else {
                    ideal
                };
                let allocated = if ideal <= available + 0.5 {
                    natural
                } else {
                    (available.floor().max(0.) as i32).max(minimum)
                };
                let ch = child.measure(gtk::Orientation::Vertical, allocated).1;
                let cx = if w.direction() == gtk::TextDirection::Rtl {
                    f64::from(width) - x - ideal
                } else {
                    x
                };
                child.allocate(
                    allocated,
                    ch,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            cx as f32,
                            (height - ch) as f32 / 2.,
                        )),
                    ),
                );
                x += ideal;
            }
        }
    }
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager, @implements gtk::Orientable;}
pub(super) fn install(check: &gtk::CheckButton) {
    check.set_layout_manager(Some(glib::Object::new::<Layout>()));
}
/// Passive rows share the same shaped-width rounding, with their ds gap.
pub(super) fn install_row(row: &gtk::Box, gap: &str) {
    let layout: Layout = glib::Object::new();
    *layout.imp().gap.borrow_mut() = Some(gap.replace('.', "-"));
    row.set_layout_manager(Some(layout));
}
