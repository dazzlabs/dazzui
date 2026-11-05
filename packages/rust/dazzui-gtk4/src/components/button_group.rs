//! A horizontal run rounds the combined native button widths once. Each
//! button retains GTK activation, grouping, focus and accessibility.
use crate::{Theme, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::cell::{Cell, RefCell};

fn children(w: &gtk::Widget) -> Vec<gtk::Button> {
    let mut children = vec![];
    let mut child = w.first_child();
    while let Some(c) = child {
        child = c.next_sibling();
        if c.get_visible()
            && let Ok(b) = c.downcast::<gtk::Button>()
        {
            children.push(b);
        }
    }
    children
}
fn widths(button: &gtk::Button) -> (f64, f64) {
    let (min, nat, _, _) = button.measure(gtk::Orientation::Horizontal, -1);
    let delta = super::button_layout::remainder(button);
    (f64::from(min) + delta, f64::from(nat) + delta)
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Group {
        pub(super) gap: RefCell<String>,
        pub(super) stretch: Cell<bool>,
        pub(super) last_focus: glib::WeakRef<gtk::Button>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Group {
        const NAME: &'static str = "DazzButtonGroup";
        type Type = super::ButtonGroup;
        type ParentType = gtk::Widget;
        fn class_init(class: &mut Self::Class) {
            class.set_layout_manager_type::<super::Layout>();
        }
    }
    impl ObjectImpl for Group {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Group {
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            if !matches!(
                direction,
                gtk::DirectionType::TabForward | gtk::DirectionType::TabBackward
            ) {
                return self.parent_focus(direction);
            }
            let group = self.obj();
            let buttons: Vec<_> = group
                .buttons()
                .into_iter()
                .filter(|b| b.is_sensitive())
                .collect();
            let focused = group
                .root()
                .and_downcast::<gtk::Window>()
                .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
            if let Some(button) = buttons
                .iter()
                .find(|b| focused.as_ref() == Some(b.upcast_ref()))
            {
                self.last_focus.set(Some(button));
                return false;
            }
            let last = self.last_focus.upgrade().filter(|b| buttons.contains(b));
            let active = || {
                buttons
                    .iter()
                    .find(|b| {
                        b.downcast_ref::<gtk::ToggleButton>()
                            .is_some_and(|b| b.is_active())
                    })
                    .cloned()
            };
            let target = last.or_else(active).or_else(|| buttons.first().cloned());
            target.is_some_and(|b| {
                self.last_focus.set(Some(&b));
                b.grab_focus()
            })
        }
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            for child in children(self.obj().upcast_ref()) {
                self.obj().snapshot_child(&child, snapshot);
            }
        }
    }
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzButtonGroupLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {}
    impl LayoutManagerImpl for Layout {
        fn measure(&self, w: &gtk::Widget, o: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            let group = w.downcast_ref::<super::ButtonGroup>().unwrap();
            let children = children(w);
            if o == gtk::Orientation::Vertical {
                let height = children
                    .iter()
                    .map(|b| b.measure(o, -1).1)
                    .max()
                    .unwrap_or(0);
                return (height, height, -1, -1);
            }
            let gap = group.gap() * children.len().saturating_sub(1) as f64;
            let min = gap + children.iter().map(|b| widths(b).0).sum::<f64>();
            let nat = gap + children.iter().map(|b| widths(b).1).sum::<f64>();
            (min.round() as i32, nat.round() as i32, -1, -1)
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            let group = w.downcast_ref::<super::ButtonGroup>().unwrap();
            let buttons = children(w);
            let gap = group.gap();
            let count = buttons.len();
            let equal =
                (f64::from(width) - gap * count.saturating_sub(1) as f64) / count.max(1) as f64;
            let mut x = 0.;
            for b in buttons {
                let bw = if group.imp().stretch.get() {
                    equal.max(0.)
                } else {
                    widths(&b).1
                };
                let bh = b.measure(gtk::Orientation::Vertical, bw.round() as i32).1;
                let bx = if w.direction() == gtk::TextDirection::Rtl {
                    f64::from(width) - x - bw
                } else {
                    x
                };
                b.allocate(
                    bw.round() as i32,
                    bh,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            bx as f32,
                            (height - bh) as f32 / 2.,
                        )),
                    ),
                );
                x += bw + gap;
            }
        }
    }
}
glib::wrapper! {
    pub struct ButtonGroup(ObjectSubclass<imp::Group>) @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager;}
impl ButtonGroup {
    pub(super) fn new(gap: &str) -> Self {
        let w: Self = glib::Object::new();
        w.set_accessible_role(gtk::AccessibleRole::Group);
        w.imp().gap.replace(gap.into());
        super::collections::group_keyboard(&w, false, false);
        w
    }
    fn gap(&self) -> f64 {
        Theme::of(self)
            .unwrap_or_default()
            .dimension(&format!("spacing-{}", self.imp().gap.borrow()))
            .unwrap()
    }
    pub fn append(&self, button: &impl IsA<gtk::Button>) {
        let button = button.as_ref();
        button.set_parent(self);
        let weak = self.downgrade();
        button.connect_has_focus_notify(move |b| {
            if b.has_focus()
                && let Some(group) = weak.upgrade()
            {
                group.imp().last_focus.set(Some(b));
            }
        });
    }
    pub fn buttons(&self) -> Vec<gtk::Button> {
        children(self.upcast_ref())
    }
    /// Give each button an equal share of the available content width.
    pub fn set_stretch(&self, stretch: bool) {
        if self.imp().stretch.replace(stretch) != stretch {
            self.queue_resize();
        }
    }
}
