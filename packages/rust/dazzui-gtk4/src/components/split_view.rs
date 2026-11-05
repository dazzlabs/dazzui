//! Adjacent panels share one run. GTK Paned owns dragging; this group keeps
//! the pair constraints, percentages and keyboard steps of the shared API.
use crate::{Theme, Tint, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::cell::{Cell, RefCell};

#[derive(Clone)]
pub struct SplitPane {
    pub widget: gtk::Widget,
    pub default_size: f64,
    pub min_size: f64,
    pub max_size: f64,
}
impl SplitPane {
    pub fn new(widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            widget: widget.as_ref().clone(),
            default_size: 1.,
            min_size: 0.,
            max_size: 100.,
        }
    }
}
pub fn splitter_panel(content: &impl IsA<gtk::Widget>) -> gtk::ScrolledWindow {
    let panel = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .propagate_natural_width(false)
        .propagate_natural_height(false)
        .min_content_width(0)
        .min_content_height(0)
        .hexpand(true)
        .vexpand(true)
        .child(content)
        .build();
    panel.add_css_class("dz-split-panel");
    panel.set_focusable(false);
    panel
}

mod imp {
    use super::*;
    pub struct SplitView {
        pub orientation: Cell<gtk::Orientation>,
        pub panels: RefCell<Vec<SplitPane>>,
        pub shares: RefCell<Vec<f64>>,
        pub paneds: RefCell<Vec<gtk::Paned>>,
        pub handles: RefCell<Vec<gtk::Widget>>,
        pub applying: Cell<bool>,
    }
    impl Default for SplitView {
        fn default() -> Self {
            Self {
                orientation: Cell::new(gtk::Orientation::Horizontal),
                panels: RefCell::default(),
                shares: RefCell::default(),
                paneds: RefCell::default(),
                handles: RefCell::default(),
                applying: Cell::default(),
            }
        }
    }
    #[glib::object_subclass]
    impl ObjectSubclass for SplitView {
        const NAME: &'static str = "DazzSplitView";
        type Type = super::SplitView;
        type ParentType = gtk::Widget;
        type Interfaces = (gtk::Orientable,);
        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<super::SplitLayout>();
        }
    }
    impl ObjectImpl for SplitView {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: std::sync::OnceLock<Vec<glib::ParamSpec>> =
                std::sync::OnceLock::new();
            PROPERTIES.get_or_init(|| {
                vec![glib::ParamSpecOverride::for_interface::<gtk::Orientable>(
                    "orientation",
                )]
            })
        }
        fn set_property(&self, _: usize, value: &glib::Value, _: &glib::ParamSpec) {
            let orientation = value.get::<gtk::Orientation>().unwrap();
            self.orientation.set(orientation);
            let previous = self.applying.replace(true);
            for paned in self.paneds.borrow().iter() {
                paned.set_orientation(orientation);
            }
            self.obj().sync_positions();
            self.applying.set(previous);
            self.obj().queue_resize();
        }
        fn property(&self, _: usize, _: &glib::ParamSpec) -> glib::Value {
            self.orientation.get().to_value()
        }
        fn dispose(&self) {
            self.applying.set(true);
            if let Some(child) = self.obj().first_child() {
                child.unparent();
            }
            self.paneds.borrow_mut().clear();
            self.handles.borrow_mut().clear();
            self.panels.borrow_mut().clear();
        }
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: std::sync::OnceLock<Vec<glib::subclass::Signal>> =
                std::sync::OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("sizes-changed")
                        .param_types([glib::Variant::static_type()])
                        .build(),
                ]
            })
        }
    }
    impl WidgetImpl for SplitView {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            if let Some(child) = self.obj().first_child() {
                self.obj().snapshot_child(&child, snapshot);
            }
        }
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            if ![
                gtk::DirectionType::TabForward,
                gtk::DirectionType::TabBackward,
            ]
            .contains(&direction)
            {
                return self.parent_focus(direction);
            }
            self.obj().focus_sequence(direction)
        }
        fn direction_changed(&self, previous: gtk::TextDirection) {
            self.parent_direction_changed(previous);
            let direction = self.obj().direction();
            for paned in self.paneds.borrow().iter() {
                paned.set_direction(direction);
            }
        }
    }
    impl OrientableImpl for SplitView {}
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzSplitLayout";
        type Type = super::SplitLayout;
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
            widget
                .first_child()
                .map(|child| child.measure(orientation, for_size))
                .unwrap_or((0, 0, -1, -1))
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
            let view = widget.downcast_ref::<super::SplitView>().unwrap();
            view.imp().applying.set(true);
            if let Some(child) = widget.first_child() {
                child.size_allocate(&gtk::Allocation::new(0, 0, width, height), baseline);
            }
            view.sync_positions();
            view.imp().applying.set(false);
        }
    }
}
glib::wrapper! {
    pub struct SplitView(ObjectSubclass<imp::SplitView>) @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}
glib::wrapper! {pub struct SplitLayout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager;}
impl SplitView {
    pub fn new(
        orientation: gtk::Orientation,
        panels: Vec<SplitPane>,
    ) -> Result<Self, &'static str> {
        if panels.iter().any(|p| {
            !p.default_size.is_finite()
                || p.default_size < 0.
                || !p.min_size.is_finite()
                || !p.max_size.is_finite()
                || p.min_size < 0.
                || p.max_size > 100.
                || p.min_size > p.max_size
        }) {
            return Err("Invalid splitter shares or constraints");
        }
        let sum: f64 = panels.iter().map(|p| p.default_size).sum();
        if !panels.is_empty() && (sum <= 0. || !sum.is_finite()) {
            return Err("Splitter shares must have a positive finite sum");
        }
        let view: Self = glib::Object::new();
        view.add_css_class("dz-split-view");
        view.set_dazz_tint(Tint::Primary);
        view.set_hexpand(true);
        view.set_vexpand(true);
        view.set_orientation(orientation);
        view.imp().orientation.set(orientation);
        view.imp()
            .shares
            .replace(panels.iter().map(|p| p.default_size / sum * 100.).collect());
        view.imp().panels.replace(panels);
        let widgets: Vec<_> = view
            .imp()
            .panels
            .borrow()
            .iter()
            .map(|p| p.widget.clone())
            .collect();
        if let Some(last) = widgets.last() {
            let mut tail = last.clone();
            let mut paneds = vec![];
            let mut handles = vec![];
            for index in (0..widgets.len().saturating_sub(1)).rev() {
                let paned = gtk::Paned::new(orientation);
                paned.add_css_class("dz-split-pane");
                paned.set_wide_handle(true);
                paned.set_resize_start_child(false);
                paned.set_resize_end_child(true);
                paned.set_shrink_start_child(true);
                paned.set_shrink_end_child(true);
                paned.set_start_child(Some(&widgets[index]));
                paned.set_end_child(Some(&tail));
                paned.set_direction(view.direction());
                let mut child = paned.first_child();
                let handle = loop {
                    let node = child.ok_or("GTK Paned has no separator")?;
                    child = node.next_sibling();
                    if node.css_name() == "separator" {
                        break node;
                    }
                };
                handle.set_focusable(true);
                handle.set_accessible_role(gtk::AccessibleRole::Separator);
                handle.update_relation(&[gtk::accessible::Relation::Controls(&[
                    widgets[index].upcast_ref()
                ])]);
                let weak = view.downgrade();
                paned.connect_position_notify(move |paned| {
                    if let Some(view) = weak.upgrade()
                        && !view.imp().applying.get()
                    {
                        let run = view.run();
                        if run > 0. {
                            view.resize(index, f64::from(paned.position()) / run * 100.);
                        }
                    }
                });
                view.install_input(&paned, &handle, index);
                tail = paned.clone().upcast();
                paneds.push(paned);
                handles.push(handle);
            }
            paneds.reverse();
            handles.reverse();
            view.imp().paneds.replace(paneds);
            view.imp().handles.replace(handles);
            tail.set_parent(&view);
        }
        view.sync_accessibility();
        Ok(view)
    }
    pub fn sizes(&self) -> Vec<f64> {
        self.imp().shares.borrow().clone()
    }
    pub fn paned(&self, index: usize) -> Option<gtk::Paned> {
        self.imp().paneds.borrow().get(index).cloned()
    }
    pub fn handle(&self, index: usize) -> Option<gtk::Widget> {
        self.imp().handles.borrow().get(index).cloned()
    }
    pub fn connect_sizes_changed(
        &self,
        callback: impl Fn(&Self, &[f64]) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("sizes-changed", false, move |values| {
            let view = values[0].get::<Self>().unwrap();
            let shares = values[1]
                .get::<glib::Variant>()
                .unwrap()
                .get::<Vec<f64>>()
                .unwrap();
            callback(&view, &shares);
            None
        })
    }
    /// Re-proportion just the two panels on either side of this seam.
    pub fn resize(&self, index: usize, leading: f64) {
        if !leading.is_finite() || index + 1 >= self.imp().panels.borrow().len() {
            return;
        }
        let next = {
            let panels = self.imp().panels.borrow();
            let mut shares = self.sizes();
            let pair = shares[index] + shares[index + 1];
            let lead = &panels[index];
            let trail = &panels[index + 1];
            let leading = leading
                .clamp(lead.min_size, lead.max_size)
                .clamp(pair - trail.max_size, pair - trail.min_size);
            shares[index] = leading;
            shares[index + 1] = pair - leading;
            shares
        };
        let changed = *self.imp().shares.borrow() != next;
        self.imp().shares.replace(next.clone());
        let previous = self.imp().applying.replace(true);
        self.sync_positions();
        self.imp().applying.set(previous);
        self.queue_allocate();
        if changed {
            self.emit_by_name::<()>("sizes-changed", &[&next.to_variant()]);
        }
    }
    fn run(&self) -> f64 {
        let span = if self.imp().orientation.get() == gtk::Orientation::Horizontal {
            self.width()
        } else {
            self.height()
        };
        let gutter = Theme::of(self)
            .unwrap_or_default()
            .dimension("spacing-2")
            .unwrap();
        (f64::from(span) - gutter * self.imp().paneds.borrow().len() as f64).max(0.)
    }
    fn sync_positions(&self) {
        let run = self.run();
        let shares = self.sizes();
        for (index, paned) in self.imp().paneds.borrow().iter().enumerate() {
            let position = (run * shares[index] / 100.).round() as i32;
            if paned.position() != position {
                paned.set_position(position);
            }
        }
        self.sync_accessibility();
    }
    fn sync_accessibility(&self) {
        let panels = self.imp().panels.borrow();
        let shares = self.sizes();
        let horizontal = self.imp().orientation.get() == gtk::Orientation::Horizontal;
        for (index, handle) in self.imp().handles.borrow().iter().enumerate() {
            handle.update_property(&[
                gtk::accessible::Property::Orientation(if horizontal {
                    gtk::Orientation::Vertical
                } else {
                    gtk::Orientation::Horizontal
                }),
                gtk::accessible::Property::ValueMin(panels[index].min_size.round()),
                gtk::accessible::Property::ValueMax(panels[index].max_size.round()),
                gtk::accessible::Property::ValueNow(shares[index].round()),
            ]);
        }
    }
    fn install_input(&self, paned: &gtk::Paned, handle: &gtk::Widget, index: usize) {
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = self.downgrade();
        let seam = handle.downgrade();
        let native = paned.downgrade();
        keys.connect_key_pressed(move |_, key, _, mods| {
            use gtk::{
                gdk::{Key, ModifierType as M},
                glib::Propagation,
            };
            let (Some(view), Some(handle), Some(paned)) =
                (weak.upgrade(), seam.upgrade(), native.upgrade())
            else {
                return Propagation::Proceed;
            };
            let focus = view
                .root()
                .and_downcast::<gtk::Window>()
                .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
            if focus.as_ref() != Some(&handle) && focus.as_ref() != Some(paned.upcast_ref()) {
                return Propagation::Proceed;
            }
            if mods.intersects(M::CONTROL_MASK | M::ALT_MASK | M::META_MASK | M::SUPER_MASK) {
                return Propagation::Proceed;
            }
            let horizontal = view.imp().orientation.get() == gtk::Orientation::Horizontal;
            let (back, forward) = if horizontal {
                (Key::Left, Key::Right)
            } else {
                (Key::Up, Key::Down)
            };
            let theme = Theme::of(&view).unwrap_or_default();
            let step = theme
                .number(if mods.contains(M::SHIFT_MASK) {
                    "splitter-key-coarse-step"
                } else {
                    "splitter-key-step"
                })
                .unwrap();
            let share = view.sizes()[index];
            let panel = &view.imp().panels.borrow()[index];
            let next = if key == back {
                share - step
            } else if key == forward {
                share + step
            } else if key == Key::Home {
                panel.min_size
            } else if key == Key::End {
                panel.max_size
            } else {
                return Propagation::Proceed;
            };
            view.resize(index, next);
            Propagation::Stop
        });
        paned.add_controller(keys);
        let click = gtk::GestureClick::new();
        click.set_button(1);
        click.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = self.downgrade();
        let seam = handle.downgrade();
        let native = paned.downgrade();
        click.connect_pressed(move |_, presses, x, y| {
            let (Some(view), Some(handle), Some(paned)) =
                (weak.upgrade(), seam.upgrade(), native.upgrade())
            else {
                return;
            };
            if handle.compute_bounds(&paned).is_some_and(|rect| {
                rect.contains_point(&gtk::graphene::Point::new(x as f32, y as f32))
            }) {
                handle.grab_focus();
                if presses % 2 == 0 {
                    let sizes = view.sizes();
                    view.resize(index, (sizes[index] + sizes[index + 1]) / 2.);
                }
            }
        });
        paned.add_controller(click.clone());
        // A successful native drag claims the press even before it moves.
        // Share its sequence so it does not cancel the double-click counter.
        let controllers = paned.observe_controllers();
        for i in 0..controllers.n_items() {
            if let Some(drag) = controllers.item(i).and_downcast::<gtk::GestureDrag>()
                && !drag.is::<gtk::GesturePan>()
            {
                click.group_with(&drag);
                break;
            }
        }
    }
    fn focus_sequence(&self, direction: gtk::DirectionType) -> bool {
        let mut sequence = vec![];
        for (index, panel) in self.imp().panels.borrow().iter().enumerate() {
            sequence.push((panel.widget.clone(), false));
            if let Some(handle) = self.handle(index) {
                sequence.push((handle, true));
            }
        }
        if direction == gtk::DirectionType::TabBackward {
            sequence.reverse();
        }
        let focus = self
            .root()
            .and_downcast::<gtk::Window>()
            .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
        let current = sequence.iter().position(|(widget, _)| {
            focus
                .as_ref()
                .is_some_and(|focus| focus == widget || focus.is_ancestor(widget))
        });
        if let Some(index) = current
            && !sequence[index].1
        {
            let previous = focus.clone();
            if sequence[index].0.child_focus(direction) {
                let next = self
                    .root()
                    .and_downcast::<gtk::Window>()
                    .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
                if next != previous {
                    return true;
                }
            }
        }
        for (widget, handle) in sequence.into_iter().skip(current.map_or(0, |i| i + 1)) {
            if !widget.is_visible() || !widget.is_sensitive() {
                continue;
            }
            if handle {
                if widget.grab_focus() {
                    return true;
                }
            } else {
                let before = self
                    .root()
                    .and_downcast::<gtk::Window>()
                    .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
                if widget.child_focus(direction) {
                    let after = self
                        .root()
                        .and_downcast::<gtk::Window>()
                        .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
                    if before != after {
                        return true;
                    }
                }
            }
        }
        false
    }
}
pub fn split_view(
    orientation: gtk::Orientation,
    panels: Vec<SplitPane>,
) -> Result<SplitView, &'static str> {
    SplitView::new(orientation, panels)
}
