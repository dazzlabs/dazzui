//! Loading silhouettes have no input behaviour. Native GTK layout and CSS
//! keep their shape and animation in the current theme.
use crate::{Theme, box_, prelude::*};

#[derive(Clone, Copy, Debug, Default)]
pub enum SkeletonShape {
    #[default]
    Text,
    Circle,
    Rect,
}

/// A loading silhouette in logical pixels. An unspecified text width fills
/// its parent; an unspecified circle height follows its width.
pub fn skeleton(shape: SkeletonShape, width: Option<i32>, height: Option<i32>) -> gtk::Box {
    let w = gtk::Box::new(gtk::Orientation::Vertical, 0);
    w.add_css_class("dz-skeleton");
    w.add_css_class(match shape {
        SkeletonShape::Text => "dz-skeleton-text-bar",
        SkeletonShape::Circle => "dz-skeleton-circle",
        SkeletonShape::Rect => "dz-skeleton-rect",
    });
    let height = height.or_else(|| {
        matches!(shape, SkeletonShape::Circle)
            .then_some(width)
            .flatten()
    });
    if height.is_some() {
        w.add_css_class("dz-skeleton-height");
    }
    w.set_size_request(width.unwrap_or(-1), height.unwrap_or(-1));
    w.set_halign(if width.is_some() {
        gtk::Align::Start
    } else {
        gtk::Align::Fill
    });
    w.set_hexpand(width.is_none());
    w.set_can_target(false);
    w.set_accessible_role(gtk::AccessibleRole::Presentation);
    w.update_state(&[gtk::accessible::State::Hidden(true)]);
    w
}

/// Reserve a line and give a child a fraction of its current allocation.
/// Round the whole fraction once, rather than each column in a grid.
pub fn proportional_width(child: &impl IsA<gtk::Widget>, fraction: f64) -> gtk::Widget {
    assert!(fraction >= 0. && fraction.is_finite());
    fraction::Fraction::new(child, fraction).upcast()
}

/// A paragraph of loading bars, with the last line shortened by ds.
pub fn skeleton_text(lines: usize) -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "1-5");
    w.add_css_class("dz-skeleton-paragraph");
    for index in 0..lines {
        let bar = skeleton(SkeletonShape::Text, None, None);
        if index + 1 == lines {
            let fraction = Theme::default().number("skeleton-last-line-width").unwrap();
            w.append(&proportional_width(&bar, fraction));
        } else {
            w.append(&bar);
        }
    }
    w
}

mod fraction {
    use gtk::{glib, prelude::*, subclass::prelude::*};
    use std::cell::Cell;
    mod imp {
        use super::*;
        #[derive(Default)]
        pub struct Fraction {
            pub fraction: Cell<f64>,
        }
        #[glib::object_subclass]
        impl ObjectSubclass for Fraction {
            const NAME: &'static str = "DazzProportionalWidth";
            type Type = super::Fraction;
            type ParentType = gtk::Widget;
            fn class_init(klass: &mut Self::Class) {
                klass.set_layout_manager_type::<super::Layout>();
            }
        }
        impl ObjectImpl for Fraction {
            fn dispose(&self) {
                if let Some(child) = self.obj().first_child() {
                    child.unparent();
                }
            }
        }
        impl WidgetImpl for Fraction {
            fn snapshot(&self, snapshot: &gtk::Snapshot) {
                if let Some(child) = self.obj().first_child() {
                    self.obj().snapshot_child(&child, snapshot);
                }
            }
        }
        #[derive(Default)]
        pub struct Layout;
        #[glib::object_subclass]
        impl ObjectSubclass for Layout {
            const NAME: &'static str = "DazzProportionalWidthLayout";
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
                let fraction = widget
                    .downcast_ref::<super::Fraction>()
                    .unwrap()
                    .imp()
                    .fraction
                    .get();
                let Some(child) = widget.first_child() else {
                    return (0, 0, -1, -1);
                };
                if orientation == gtk::Orientation::Horizontal {
                    let (_, natural, _, _) = child.measure(orientation, -1);
                    (
                        0,
                        if fraction == 0. {
                            0
                        } else {
                            (f64::from(natural) / fraction).ceil() as i32
                        },
                        -1,
                        -1,
                    )
                } else {
                    child.measure(
                        orientation,
                        if for_size < 0 {
                            -1
                        } else {
                            (f64::from(for_size) * fraction).round() as i32
                        },
                    )
                }
            }
            fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
                let fraction = widget
                    .downcast_ref::<super::Fraction>()
                    .unwrap()
                    .imp()
                    .fraction
                    .get();
                if let Some(child) = widget.first_child() {
                    let child_width = (f64::from(width) * fraction).round() as i32;
                    let x = if widget.direction() == gtk::TextDirection::Rtl {
                        width - child_width
                    } else {
                        0
                    };
                    child.size_allocate(&gtk::Allocation::new(x, 0, child_width, height), baseline);
                }
            }
        }
    }
    glib::wrapper! { pub struct Fraction(ObjectSubclass<imp::Fraction>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget; }
    glib::wrapper! { pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager; }
    impl Fraction {
        pub fn new(child: &impl IsA<gtk::Widget>, fraction: f64) -> Self {
            let w: Self = glib::Object::new();
            w.imp().fraction.set(fraction);
            w.set_hexpand(true);
            child.set_halign(gtk::Align::Fill);
            child.set_parent(&w);
            w
        }
    }
}
