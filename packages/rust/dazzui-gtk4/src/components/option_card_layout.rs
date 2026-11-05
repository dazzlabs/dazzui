//! GTK allocates integer content heights. Carry this card's native paragraph
//! height in fractional CSS padding, so its painted border retains that height.
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
fn body_height(w: &gtk::Widget, width: i32) -> f64 {
    let Some(column) = w.downcast_ref::<gtk::CheckButton>().and_then(|c| c.child()) else {
        return 0.;
    };
    let width = if width >= 0 {
        width.max(column.measure(gtk::Orientation::Horizontal, -1).0)
    } else {
        column.measure(gtk::Orientation::Horizontal, -1).1
    };
    let labels = children(&column);
    if !labels.iter().all(|child| child.is::<gtk::Label>()) {
        return f64::from(column.measure(gtk::Orientation::Vertical, width).1);
    }
    let mut height = 0.;
    for child in &labels {
        let label = child.downcast_ref::<gtk::Label>().unwrap();
        if label.text().is_empty() {
            continue;
        }
        let layout = label.layout().copy();
        layout.set_width(if label.wraps() {
            width * gtk::pango::SCALE
        } else {
            -1
        });
        height += super::text_box::line_height(label) * f64::from(layout.line_count());
    }
    height
        + Theme::of(w)
            .unwrap_or_default()
            .dimension("spacing-1-5")
            .unwrap()
            * labels.len().saturating_sub(1) as f64
}
mod imp {
    use super::*;
    pub struct Layout {
        pub provider: std::cell::OnceCell<gtk::CssProvider>,
        pub selector: std::cell::RefCell<String>,
        pub height: std::cell::Cell<f64>,
        pub pending: std::cell::Cell<Option<f64>>,
        pub idle: std::cell::RefCell<Option<glib::SourceId>>,
    }
    impl Default for Layout {
        fn default() -> Self {
            Self {
                provider: Default::default(),
                selector: Default::default(),
                height: std::cell::Cell::new(f64::NAN),
                pending: Default::default(),
                idle: Default::default(),
            }
        }
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzOptionCardLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {
        fn dispose(&self) {
            if let Some(id) = self.idle.take() {
                id.remove();
            }
        }
    }
    impl Layout {
        pub fn request(&self, height: f64) {
            if self.height.get() == height {
                self.pending.set(None);
                if let Some(id) = self.idle.take() {
                    id.remove();
                }
                return;
            }
            self.pending.set(Some(height));
            if self.idle.borrow().is_some() {
                return;
            }
            let weak = self.obj().downgrade();
            *self.idle.borrow_mut() = Some(glib::idle_add_local_once(move || {
                let Some(layout) = weak.upgrade() else {
                    return;
                };
                let imp = layout.imp();
                imp.idle.take();
                if let Some(height) = imp.pending.take() {
                    imp.height.set(height);
                    imp.provider.get().unwrap().load_from_string(&format!(
                        ".{} {{ --dz-option-body-height: {height:.6}px; }}",
                        imp.selector.borrow()
                    ));
                }
            }));
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
            if o == gtk::Orientation::Vertical {
                let desired = body_height(w, for_size);
                self.request(desired);
                // Keep requests conservative until the next style pass. CSS
                // must never be invalidated inside an ancestor's measurement.
                let current = if self.height.get().is_finite() {
                    self.height.get()
                } else {
                    0.
                };
                let extra = (desired - current).max(0.).ceil() as i32;
                return (extra, extra, -1, -1);
            }
            let (mut min, mut nat) = (0., 0.);
            for child in children(w) {
                let (a, b, _, _) = child.measure(o, -1);
                let delta = super::super::check_layout::correction(&child);
                min += f64::from(a) + delta;
                nat += f64::from(b) + delta;
            }
            (
                min.round().max(0.) as i32,
                nat.round().max(min.round()).max(0.) as i32,
                -1,
                -1,
            )
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, allocated_height: i32, _: i32) {
            self.request(body_height(w, width));
            let column = w.downcast_ref::<gtk::CheckButton>().unwrap().child();
            for child in children(w) {
                if Some(&child) == column.as_ref() {
                    let width = width.max(child.measure(gtk::Orientation::Horizontal, -1).0);
                    let height = child.measure(gtk::Orientation::Vertical, width).1;
                    child.allocate(
                        width,
                        height,
                        -1,
                        Some(
                            gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                                0.,
                                (allocated_height - height) as f32 / 2.,
                            )),
                        ),
                    );
                } else {
                    child.allocate(0, 0, -1, None);
                }
            }
        }
    }
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager;}
pub(super) fn install(card: &gtk::CheckButton) {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let selector = format!(
        "dz-option-body-{}",
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    );
    let provider = gtk::CssProvider::new();
    let display = card.display();
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
    );
    let layout: Layout = glib::Object::new();
    layout.imp().provider.set(provider.clone()).unwrap();
    *layout.imp().selector.borrow_mut() = selector.clone();
    card.add_css_class(&selector);
    card.add_css_class("dz-option-fractional-body");
    card.connect_destroy(move |_| {
        gtk::style_context_remove_provider_for_display(&display, &provider)
    });
    card.set_layout_manager(Some(layout));
}
