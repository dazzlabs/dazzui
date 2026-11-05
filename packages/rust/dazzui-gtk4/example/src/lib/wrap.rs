//! The story Row uses flex-wrap. GTK Box does not wrap; FlowBox reserves
//! columns, so unequal buttons get different gaps. This layout places native
//! children at their own natural widths and measures height for that width.
use dazzui_gtk4::{Theme, gtk, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::cell::{Cell, RefCell};

mod imp {
    use super::*;
    pub struct Wrap {
        pub gap: RefCell<String>,
        pub align: Cell<gtk::Align>,
        pub homogeneous: Cell<bool>,
    }
    impl Default for Wrap {
        fn default() -> Self {
            Self {
                gap: RefCell::new("3".into()),
                align: Cell::new(gtk::Align::Center),
                homogeneous: Cell::new(false),
            }
        }
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Wrap {
        const NAME: &'static str = "DazzGalleryWrap";
        type Type = super::Wrap;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<super::WrapLayout>();
        }
    }
    impl ObjectImpl for Wrap {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl Wrap {
        fn children(&self) -> Vec<gtk::Widget> {
            let mut out = vec![];
            let mut child = self.obj().first_child();
            while let Some(w) = child {
                child = w.next_sibling();
                if w.is_visible() {
                    out.push(w);
                }
            }
            out
        }
        fn gap(&self) -> i32 {
            Theme::of(&*self.obj())
                .unwrap_or_default()
                .dimension(&format!("spacing-{}", self.gap.borrow()))
                .expect("A story uses a ds spacing key")
                .round() as i32
        }
        fn rows(&self, width: i32) -> Vec<Vec<(gtk::Widget, i32, i32)>> {
            if self.homogeneous.get() {
                let children = self.children();
                let count = children.len().max(1) as i32;
                let w = ((f64::from(width) - f64::from(self.gap() * (count - 1)))
                    / f64::from(count))
                .round()
                .max(0.) as i32;
                return vec![
                    children
                        .into_iter()
                        .map(|child| {
                            let w = w.max(child.measure(gtk::Orientation::Horizontal, -1).0);
                            let h = child.measure(gtk::Orientation::Vertical, w).1;
                            (child, w, h)
                        })
                        .collect(),
                ];
            }
            let mut rows = vec![];
            let mut row = vec![];
            let mut used = 0;
            let gap = self.gap();
            for child in self.children() {
                let (_, natural, _, _) = child.measure(gtk::Orientation::Horizontal, -1);
                let w = if child.hexpands() {
                    width
                } else {
                    natural.min(width)
                };
                if !row.is_empty() && used + gap + w > width {
                    rows.push(std::mem::take(&mut row));
                    used = 0;
                }
                if !row.is_empty() {
                    used += gap;
                }
                let (_, h, _, _) = child.measure(gtk::Orientation::Vertical, w);
                row.push((child, w, h));
                used += w;
            }
            if !row.is_empty() {
                rows.push(row);
            }
            rows
        }
    }
    impl Wrap {
        pub fn measure_layout(
            &self,
            orientation: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            if orientation == gtk::Orientation::Horizontal {
                let children = self.children();
                let (mut min, mut natural) = (0, 0);
                for child in &children {
                    let (a, b, _, _) = child.measure(orientation, -1);
                    min = min.max(a);
                    if self.homogeneous.get() {
                        natural = natural.max(b);
                    } else {
                        natural += b;
                    }
                }
                if self.homogeneous.get() {
                    min *= children.len() as i32;
                    natural *= children.len() as i32;
                }
                natural += self.gap() * children.len().saturating_sub(1) as i32;
                (min, natural.max(min), -1, -1)
            } else {
                let rows = self.rows(if for_size < 0 {
                    self.measure_layout(gtk::Orientation::Horizontal, -1).1
                } else {
                    for_size.max(1)
                });
                let height = rows
                    .iter()
                    .map(|row| row.iter().map(|(_, _, h)| *h).max().unwrap_or(0))
                    .sum::<i32>()
                    + self.gap() * rows.len().saturating_sub(1) as i32;
                (height, height, -1, -1)
            }
        }
        pub fn allocate_layout(&self, width: i32) {
            let gap = self.gap();
            let mut y = 0;
            for row in self.rows(width) {
                let height = row.iter().map(|(_, _, h)| *h).max().unwrap_or(0);
                let mut x = 0.;
                let column = (f64::from(width)
                    - f64::from(gap) * row.len().saturating_sub(1) as f64)
                    / row.len().max(1) as f64;
                for (child, w, h) in row {
                    let offset = match self.align.get() {
                        gtk::Align::Center => (height - h) / 2,
                        gtk::Align::End => height - h,
                        _ => 0,
                    };
                    let h = if self.align.get() == gtk::Align::Fill {
                        height
                    } else {
                        h
                    };
                    let child_x = if self.obj().direction() == gtk::TextDirection::Rtl {
                        f64::from(width)
                            - x
                            - if self.homogeneous.get() {
                                column
                            } else {
                                f64::from(w)
                            }
                    } else {
                        x
                    };
                    child.allocate(
                        w,
                        h,
                        -1,
                        Some(
                            gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                                child_x as f32,
                                (y + offset) as f32,
                            )),
                        ),
                    );
                    x += if self.homogeneous.get() {
                        column + f64::from(gap)
                    } else {
                        f64::from(w + gap)
                    };
                }
                y += height + gap;
            }
        }
    }
    impl WidgetImpl for Wrap {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            for child in self.children() {
                self.obj().snapshot_child(&child, snapshot);
            }
        }
    }
}
glib::wrapper! {
    pub struct Wrap(ObjectSubclass<imp::Wrap>) @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
impl Wrap {
    pub fn new(gap: &str, align: gtk::Align) -> Self {
        let w: Self = glib::Object::new();
        *w.imp().gap.borrow_mut() = gap.replace('.', "-");
        w.imp().align.set(align);
        w
    }
    pub fn append(&self, child: &gtk::Widget) {
        child.set_parent(self);
        self.queue_resize();
    }
    pub fn set_homogeneous(&self, value: bool) {
        self.imp().homogeneous.set(value);
        self.queue_resize();
    }
}

mod layout_imp {
    use super::*;
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzGalleryWrapLayout";
        type Type = super::WrapLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {}
    impl LayoutManagerImpl for Layout {
        fn request_mode(&self, _widget: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            widget
                .downcast_ref::<Wrap>()
                .unwrap()
                .imp()
                .measure_layout(orientation, for_size)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, _height: i32, _baseline: i32) {
            widget
                .downcast_ref::<Wrap>()
                .unwrap()
                .imp()
                .allocate_layout(width)
        }
    }
}
glib::wrapper! { pub struct WrapLayout(ObjectSubclass<layout_imp::Layout>) @extends gtk::LayoutManager; }
