//! One or more native scales share a track. GTK keeps pointer capture, focus
//! and accessibility; the coordinator applies the shared collision rules.
use crate::{Size, Theme, Tint, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::{
    cell::{Cell, OnceCell, RefCell},
    rc::Rc,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderCollision {
    #[default]
    Push,
    None,
}
#[derive(Clone, Debug, glib::Boxed)]
#[boxed_type(name = "DazzSliderChange")]
pub struct SliderChange {
    pub values: Vec<f64>,
    pub active_thumb: usize,
    pub reason: String,
    canceled: Rc<Cell<bool>>,
}
impl SliderChange {
    pub fn cancel(&self) {
        self.canceled.set(true);
    }
    pub fn is_canceled(&self) -> bool {
        self.canceled.get()
    }
}
pub struct SliderOptions {
    pub value: Option<Vec<f64>>,
    pub range: bool,
    pub default_values: Vec<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub large_step: f64,
    pub min_steps_between_values: f64,
    pub collision: SliderCollision,
    pub size: Size,
    pub tint: Tint,
    pub show_value: bool,
    pub disabled: bool,
}
impl Default for SliderOptions {
    fn default() -> Self {
        Self {
            value: None,
            range: false,
            default_values: vec![0.],
            min: 0.,
            max: 100.,
            step: 1.,
            large_step: 10.,
            min_steps_between_values: 0.,
            collision: SliderCollision::Push,
            size: Size::Medium,
            tint: Tint::Primary,
            show_value: false,
            disabled: false,
        }
    }
}
pub(super) struct Pointer {
    index: usize,
    offset: f64,
    start: f64,
    start_shift: f64,
    values: Vec<f64>,
    changed: bool,
}
fn fixed(value: f64) -> f64 {
    (value * 1e12).round() / 1e12
}
fn precision(value: f64) -> usize {
    value
        .to_string()
        .split('.')
        .nth(1)
        .map(str::len)
        .unwrap_or_default()
}
fn rounded(value: f64, digits: usize) -> f64 {
    let factor = 10f64.powi(digits.min(308) as i32);
    if (value * factor).is_finite() {
        (value * factor).round() / factor
    } else {
        value
    }
}
fn snap(value: f64, min: f64, step: f64) -> f64 {
    let value = ((value - min) / step).round() * step + min;
    rounded(value, precision(min).max(precision(step)))
}
fn pushed(values: &[f64], index: usize, next: f64, min: f64, max: f64, gap: f64) -> Vec<f64> {
    let mut out = values.to_vec();
    let last = out.len() - 1;
    out[index] = next.clamp(min + index as f64 * gap, max - (last - index) as f64 * gap);
    for i in index + 1..=last {
        let lo = out[i - 1] + gap;
        let hi = max - (last - i) as f64 * gap;
        out[i] = out[i].max(lo).clamp(lo, hi);
    }
    for i in (0..index).rev() {
        let hi = out[i + 1] - gap;
        let lo = min + i as f64 * gap;
        out[i] = out[i].min(hi).clamp(lo, hi);
    }
    out.into_iter().map(fixed).collect()
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct SliderField {
        pub control: OnceCell<super::Control>,
        pub reading: OnceCell<gtk::Label>,
        pub reading_slot: OnceCell<gtk::Box>,
        pub active: Cell<Option<usize>>,
        pub last: Cell<Option<usize>>,
        pub track: OnceCell<gtk::Box>,
        pub scales: RefCell<Vec<super::ThumbScale>>,
        pub values: RefCell<Vec<f64>>,
        pub min: Cell<f64>,
        pub max: Cell<f64>,
        pub step: Cell<f64>,
        pub large: Cell<f64>,
        pub gap: Cell<f64>,
        pub collision: Cell<SliderCollision>,
        pub controlled: Cell<bool>,
        pub range: Cell<bool>,
        pub applying: Cell<bool>,
        pub(super) pointer: RefCell<Option<Pointer>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for SliderField {
        const NAME: &'static str = "DazzSliderField";
        type Type = super::SliderField;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for SliderField {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: std::sync::OnceLock<Vec<glib::subclass::Signal>> =
                std::sync::OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("value-change")
                        .param_types([SliderChange::static_type()])
                        .build(),
                    glib::subclass::Signal::builder("value-committed")
                        .param_types([glib::Variant::static_type(), String::static_type()])
                        .build(),
                ]
            })
        }
    }
    impl WidgetImpl for SliderField {
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            if ![
                gtk::DirectionType::TabForward,
                gtk::DirectionType::TabBackward,
            ]
            .contains(&direction)
            {
                return self.parent_focus(direction);
            }
            let root = self.obj();
            let scales = root.scales();
            let focus = root
                .root()
                .and_downcast::<gtk::Window>()
                .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
            let here = scales.iter().position(|scale| {
                focus
                    .as_ref()
                    .is_some_and(|f| f == scale.upcast_ref::<gtk::Widget>() || f.is_ancestor(scale))
            });
            let target = if direction == gtk::DirectionType::TabForward {
                here.map_or(Some(0), |i| i.checked_add(1))
            } else {
                here.map_or_else(|| scales.len().checked_sub(1), |i| i.checked_sub(1))
            };
            target
                .and_then(|i| scales.get(i))
                .is_some_and(|scale| scale.grab_focus())
        }
        fn direction_changed(&self, previous: gtk::TextDirection) {
            self.parent_direction_changed(previous);
            let root = self.obj();
            if let Some(control) = self.control.get() {
                control.set_direction(root.direction());
            }
            for scale in self.scales.borrow().iter() {
                scale.set_direction(root.direction());
            }
            if let Some(reading) = self.reading.get() {
                reading.set_direction(root.direction());
                reading.set_xalign(if root.direction() == gtk::TextDirection::Rtl {
                    0.
                } else {
                    1.
                });
            }
            if let Some(track) = self.track.get() {
                track.set_direction(root.direction());
                track.queue_allocate();
            }
        }
    }
    impl BoxImpl for SliderField {}
    #[derive(Default)]
    pub struct ThumbScale {
        pub owner: glib::WeakRef<super::SliderField>,
        pub index: Cell<usize>,
        pub shift: Cell<f64>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for ThumbScale {
        const NAME: &'static str = "DazzSliderThumbScale";
        type Type = super::ThumbScale;
        type ParentType = gtk::Scale;
    }
    impl ObjectImpl for ThumbScale {}
    impl WidgetImpl for ThumbScale {
        fn contains(&self, x: f64, y: f64) -> bool {
            self.owner.upgrade().map_or_else(
                || self.parent_contains(x, y),
                |root| root.target_at(x + self.shift.get(), y) == Some(self.index.get()),
            )
        }
    }
    impl RangeImpl for ThumbScale {}
    impl ScaleImpl for ThumbScale {}
    #[derive(Default)]
    pub struct Control {
        pub owner: glib::WeakRef<super::SliderField>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Control {
        const NAME: &'static str = "DazzSliderControl";
        type Type = super::Control;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<super::ControlLayout>();
        }
    }
    impl ObjectImpl for Control {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Control {
        fn contains(&self, x: f64, y: f64) -> bool {
            self.parent_contains(x, y)
                || self
                    .owner
                    .upgrade()
                    .is_some_and(|root| (0..root.values().len()).any(|i| root.hit_thumb(i, x, y)))
        }
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            if let Some(root) = self.owner.upgrade() {
                let widget = self.obj();
                if let Some(track) = root.imp().track.get() {
                    widget.snapshot_child(track, snapshot);
                }
                let scales = root.scales();
                for index in root.paint_order() {
                    widget.snapshot_child(&scales[index], snapshot);
                }
            }
        }
    }
    #[derive(Default)]
    pub struct ControlLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for ControlLayout {
        const NAME: &'static str = "DazzSliderControlLayout";
        type Type = super::ControlLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for ControlLayout {}
    impl LayoutManagerImpl for ControlLayout {
        fn measure(&self, _: &gtk::Widget, _: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            (0, 0, -1, -1)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, _: i32) {
            let control = widget.downcast_ref::<super::Control>().unwrap();
            if let Some(root) = control.imp().owner.upgrade() {
                let track = root.imp().track.get().unwrap();
                let thick = track.measure(gtk::Orientation::Vertical, -1).1;
                track.allocate(
                    width,
                    thick,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            0.,
                            ((height - thick) / 2) as f32,
                        )),
                    ),
                );
                let scales = root.imp().scales.borrow().clone();
                for (index, scale) in scales.iter().enumerate() {
                    scale.allocate(width, height, -1, None);
                    let (start, end) = scale.slider_range();
                    let shift = root.center(index) - f64::from(start + end) / 2.;
                    scale.imp().shift.set(shift);
                    scale.allocate(
                        width,
                        height,
                        -1,
                        Some(
                            gtk::gsk::Transform::new()
                                .translate(&gtk::graphene::Point::new(shift as f32, 0.)),
                        ),
                    );
                }
            }
        }
    }
    #[derive(Default)]
    pub struct ReadingLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for ReadingLayout {
        const NAME: &'static str = "DazzSliderReadingLayout";
        type Type = super::ReadingLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for ReadingLayout {}
    impl LayoutManagerImpl for ReadingLayout {
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            _: i32,
        ) -> (i32, i32, i32, i32) {
            let label = widget.first_child().and_downcast::<gtk::Label>().unwrap();
            if orientation == gtk::Orientation::Horizontal {
                let w = (f64::from(label.layout().size().0) / f64::from(gtk::pango::SCALE)).round()
                    as i32;
                (w, w, -1, -1)
            } else {
                label.measure(orientation, -1)
            }
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, _: i32) {
            let label = widget.first_child().and_downcast::<gtk::Label>().unwrap();
            let natural = label.measure(gtk::Orientation::Horizontal, -1).1;
            let x = if label.direction() == gtk::TextDirection::Rtl {
                0
            } else {
                width - natural
            };
            label.size_allocate(&gtk::Allocation::new(x, 0, natural, height), -1);
        }
    }
    #[derive(Default)]
    pub struct TrackLayout {
        pub owner: glib::WeakRef<super::SliderField>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for TrackLayout {
        const NAME: &'static str = "DazzSliderTrackLayout";
        type Type = super::TrackLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for TrackLayout {}
    impl LayoutManagerImpl for TrackLayout {
        fn measure(&self, _: &gtk::Widget, _: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            (0, 0, -1, -1)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, _: i32) {
            if let Some(root) = self.owner.upgrade()
                && let Some(fill) = widget.first_child()
            {
                let values = root.values();
                let min = root.imp().min.get();
                let span = root.imp().max.get() - min;
                let from = if root.imp().range.get() {
                    (values[0] - min) / span
                } else {
                    0.
                };
                let to = (values[values.len() - 1] - min) / span;
                let x = if root.direction() == gtk::TextDirection::Rtl {
                    1. - to
                } else {
                    from
                };
                fill.allocate(
                    ((to - from) * f64::from(width)).round() as i32,
                    height,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            (x * f64::from(width)) as f32,
                            0.,
                        )),
                    ),
                );
            }
        }
    }
}
glib::wrapper! {pub struct Control(ObjectSubclass<imp::Control>) @extends gtk::Widget,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget;}
glib::wrapper! {pub struct ControlLayout(ObjectSubclass<imp::ControlLayout>) @extends gtk::LayoutManager;}
glib::wrapper! {pub struct ReadingLayout(ObjectSubclass<imp::ReadingLayout>) @extends gtk::LayoutManager;}
glib::wrapper! {pub struct SliderField(ObjectSubclass<imp::SliderField>) @extends gtk::Widget,gtk::Box,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;}
glib::wrapper! {pub struct ThumbScale(ObjectSubclass<imp::ThumbScale>) @extends gtk::Widget,gtk::Range,gtk::Scale,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;}
glib::wrapper! {pub struct TrackLayout(ObjectSubclass<imp::TrackLayout>) @extends gtk::LayoutManager;}
impl SliderField {
    pub fn new(options: SliderOptions) -> Result<Self, &'static str> {
        if ![
            options.min,
            options.max,
            options.step,
            options.large_step,
            options.min_steps_between_values,
        ]
        .iter()
        .all(|n| n.is_finite())
            || options.min >= options.max
            || options.step <= 0.
            || options.large_step <= 0.
            || options.min_steps_between_values < 0.
        {
            return Err("Invalid slider bounds or steps");
        }
        let root: Self = glib::Object::new();
        root.add_css_class("dz-slider-field");
        root.set_hexpand(true);
        root.set_accessible_role(gtk::AccessibleRole::Group);
        root.set_dazz_size(options.size);
        root.set_dazz_tint(options.tint);
        root.imp().min.set(options.min);
        root.imp().max.set(options.max);
        root.imp().step.set(options.step);
        root.imp().large.set(options.large_step);
        root.imp()
            .gap
            .set(options.step * options.min_steps_between_values);
        root.imp().collision.set(options.collision);
        root.imp().controlled.set(options.value.is_some());
        root.imp().range.set(options.range);
        let control: Control = glib::Object::new();
        control.imp().owner.set(Some(&root));
        control.add_css_class("dz-slider-control");
        control.set_hexpand(true);
        let track = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        track.add_css_class("dz-slider-track");
        track.set_valign(gtk::Align::Fill);
        track.set_can_target(false);
        let layout: TrackLayout = glib::Object::new();
        layout.imp().owner.set(Some(&root));
        track.set_layout_manager(Some(layout));
        let fill = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        fill.add_css_class("dz-slider-interval");
        track.append(&fill);
        track.set_parent(&control);
        root.append(&control);
        let reading = gtk::Label::new(None);

        reading.set_xalign(1.);
        reading.set_valign(gtk::Align::Center);
        reading.set_can_target(false);
        reading.pango_context().set_round_glyph_positions(false);
        let slot = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        slot.add_css_class("dz-slider-reading");
        slot.set_valign(gtk::Align::Center);
        slot.set_layout_manager(Some(glib::Object::new::<ReadingLayout>()));
        slot.append(&reading);
        root.append(&slot);
        root.imp().reading_slot.set(slot).unwrap();
        root.imp().control.set(control).unwrap();
        root.imp().track.set(track).unwrap();
        root.imp().reading.set(reading).unwrap();
        root.set_show_value(options.show_value);
        root.set_values(options.value.as_deref().unwrap_or(&options.default_values))?;
        root.set_sensitive(!options.disabled);
        root.connect_unmap(|root| root.cancel_pointer());
        root.connect_sensitive_notify(|root| {
            if !root.is_sensitive() {
                root.cancel_pointer();
            }
        });
        Ok(root)
    }
    pub fn values(&self) -> Vec<f64> {
        self.imp().values.borrow().clone()
    }
    pub fn scales(&self) -> Vec<gtk::Scale> {
        self.imp()
            .scales
            .borrow()
            .iter()
            .cloned()
            .map(Cast::upcast)
            .collect()
    }
    pub fn control(&self) -> gtk::Widget {
        self.imp().control.get().unwrap().clone().upcast()
    }
    pub fn reading(&self) -> gtk::Label {
        self.imp().reading.get().unwrap().clone()
    }
    pub fn set_range(&self, range: bool) {
        self.imp().range.set(range || self.values().len() > 1);
        self.imp().track.get().unwrap().queue_allocate();
        self.control().queue_allocate();
    }
    pub fn set_show_value(&self, show: bool) {
        self.imp().reading_slot.get().unwrap().set_visible(show);
    }
    pub fn set_values(&self, values: &[f64]) -> Result<(), &'static str> {
        if values.is_empty()
            || !values.iter().all(|v| v.is_finite())
            || (values.len() - 1) as f64 * self.imp().gap.get()
                > self.imp().max.get() - self.imp().min.get()
        {
            return Err("Invalid slider values or minimum distance");
        }
        let mut values: Vec<_> = values
            .iter()
            .map(|v| v.clamp(self.imp().min.get(), self.imp().max.get()))
            .collect();
        values.sort_by(f64::total_cmp);
        let count = values.len();
        self.imp().values.replace(values);
        if count > 1 {
            self.imp().range.set(true);
        }
        if count != self.imp().scales.borrow().len() {
            self.cancel_pointer();
        }
        while self.imp().scales.borrow().len() > count {
            let scale = self.imp().scales.borrow_mut().pop().unwrap();
            scale.unparent();
        }
        while self.imp().scales.borrow().len() < count {
            let index = self.imp().scales.borrow().len();
            let scale = self.create_scale(index);
            scale.set_parent(self.imp().control.get().unwrap());
            self.imp().scales.borrow_mut().push(scale);
        }
        self.refresh();
        Ok(())
    }
    pub fn connect_value_changed(
        &self,
        callback: impl Fn(&Self, &SliderChange) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("value-change", false, move |v| {
            callback(
                &v[0].get::<Self>().unwrap(),
                &v[1].get::<SliderChange>().unwrap(),
            );
            None
        })
    }
    pub fn connect_value_committed(
        &self,
        callback: impl Fn(&Self, &[f64], &str) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("value-committed", false, move |v| {
            callback(
                &v[0].get::<Self>().unwrap(),
                &v[1]
                    .get::<glib::Variant>()
                    .unwrap()
                    .get::<Vec<f64>>()
                    .unwrap(),
                &v[2].get::<String>().unwrap(),
            );
            None
        })
    }
    fn refresh(&self) {
        let before = self.imp().applying.replace(true);
        let values = self.values();
        for (scale, value) in self.scales().iter().zip(&values) {
            scale.set_value(*value);
        }
        self.imp().applying.set(before);
        self.reading().set_label(
            &values
                .iter()
                .map(|value| format_number(*value))
                .collect::<Vec<_>>()
                .join(" – "),
        );
        self.imp().track.get().unwrap().queue_allocate();
        self.control().queue_allocate();
    }
    fn request(&self, index: usize, values: Vec<f64>, reason: &str) -> bool {
        if values == self.values() {
            self.refresh();
            return false;
        }
        let edit = SliderChange {
            values: values.clone(),
            active_thumb: index,
            reason: reason.into(),
            canceled: Rc::default(),
        };
        self.emit_by_name::<()>("value-change", &[&edit]);
        if edit.is_canceled() {
            self.refresh();
            return false;
        }
        if !self.imp().controlled.get() {
            let _ = self.set_values(&values);
        } else {
            self.refresh();
        }
        true
    }
    /// A keyboard or accessibility change respects neighbouring values.
    pub fn move_thumb(&self, index: usize, value: f64, reason: &str) -> bool {
        if !self.is_sensitive() || !value.is_finite() || index >= self.values().len() {
            return false;
        }
        let mut values = self.values();
        let min = if index > 0 {
            values[index - 1]
        } else {
            self.imp().min.get()
        };
        let max = values
            .get(index + 1)
            .copied()
            .unwrap_or(self.imp().max.get());
        values[index] = value.clamp(min, max);
        if values
            .windows(2)
            .any(|v| v[1] - v[0] + 1e-12 < self.imp().gap.get())
        {
            self.refresh();
            return false;
        }
        let accepted = self.request(index, values.clone(), reason);
        if accepted {
            self.emit_by_name::<()>("value-committed", &[&values.to_variant(), &reason]);
        }
        accepted
    }
    fn diameter(&self) -> f64 {
        let size = if self.has_css_class("dz-size-small") {
            "small"
        } else if self.has_css_class("dz-size-large") {
            "large"
        } else {
            "medium"
        };
        Theme::of(self)
            .unwrap_or_default()
            .dimension(&format!("slider-{size}-thumb"))
            .unwrap()
    }
    fn center(&self, index: usize) -> f64 {
        let percent = (self.values()[index] - self.imp().min.get())
            / (self.imp().max.get() - self.imp().min.get());
        if self.direction() == gtk::TextDirection::Rtl {
            (1. - percent) * f64::from(self.control().width())
        } else {
            percent * f64::from(self.control().width())
        }
    }
    fn hit_thumb(&self, index: usize, x: f64, y: f64) -> bool {
        let hot = self.imp().scales.borrow().get(index).is_some_and(|scale| {
            scale.has_css_class("dz-range-hover") || scale.has_css_class("dz-range-dragging")
        });
        let radius = self.diameter() / 2.
            * if hot {
                Theme::of(self)
                    .unwrap_or_default()
                    .number("slider-thumb-hot-scale")
                    .unwrap()
            } else {
                1.
            };
        (x - self.center(index)).powi(2) + (y - f64::from(self.control().height()) / 2.).powi(2)
            <= radius * radius
    }
    fn paint_order(&self) -> Vec<usize> {
        let mut order: Vec<_> = (0..self.values().len()).collect();
        let active = self.imp().active.get();
        let last = self.imp().last.get();
        order.sort_by_key(|index| {
            if Some(*index) == active {
                2
            } else if Some(*index) == last {
                1
            } else {
                0
            }
        });
        order
    }
    fn target_at(&self, x: f64, y: f64) -> Option<usize> {
        let values = self.values();
        if values.is_empty() {
            return None;
        }
        let hit = self
            .paint_order()
            .into_iter()
            .rev()
            .find(|i| self.hit_thumb(*i, x, y));
        let mut index = hit.or_else(|| {
            if x < 0.
                || x > f64::from(self.control().width())
                || y < 0.
                || y > f64::from(self.control().height())
            {
                return None;
            }
            (0..values.len()).min_by(|a, b| {
                (x - self.center(*a))
                    .abs()
                    .total_cmp(&(x - self.center(*b)).abs())
                    .then_with(|| b.cmp(a))
            })
        })?;
        if values[index] == self.imp().max.get() {
            while index > 0 && values[index - 1] == self.imp().max.get() {
                index -= 1;
            }
        }
        Some(index)
    }
    fn begin_pointer(&self, index: usize, x: f64, y: f64) {
        if !self.is_sensitive() {
            return;
        }
        let offset = if self.hit_thumb(index, x, y) {
            x - self.center(index)
        } else {
            0.
        };
        self.imp().pointer.replace(Some(Pointer {
            index,
            offset,
            start: x,
            start_shift: self.imp().scales.borrow()[index].imp().shift.get(),
            values: self.values(),
            changed: false,
        }));
        self.scales()[index].grab_focus();
        self.scales()[index].add_css_class("dz-range-dragging");
    }
    fn pointer_to(&self, x: f64) {
        let Some((index, offset, values)) = self
            .imp()
            .pointer
            .borrow()
            .as_ref()
            .map(|p| (p.index, p.offset, p.values.clone()))
        else {
            return;
        };
        let width = f64::from(self.control().width());
        if width <= 0. {
            return;
        }
        let percent = ((x - offset) / width).clamp(0., 1.);
        let percent = if self.direction() == gtk::TextDirection::Rtl {
            1. - percent
        } else {
            percent
        };
        let min = self.imp().min.get();
        let max = self.imp().max.get();
        let next = snap(min + (max - min) * percent, min, self.imp().step.get()).clamp(min, max);
        let gap = self.imp().gap.get();
        let next = if self.imp().collision.get() == SliderCollision::Push {
            pushed(&values, index, next, min, max, gap)
        } else {
            let mut out = values.clone();
            let lower = if index > 0 {
                values[index - 1] + gap
            } else {
                min
            };
            let upper = values.get(index + 1).map(|v| v - gap).unwrap_or(max);
            if lower > upper {
                return;
            }
            out[index] = next.clamp(lower, upper);
            out
        };
        if self.request(index, next.clone(), "drag")
            && let Some(pointer) = self.imp().pointer.borrow_mut().as_mut()
        {
            pointer.values = next;
            pointer.changed = true;
        }
    }
    fn end_pointer(&self) {
        let pointer = self.imp().pointer.borrow_mut().take();
        for scale in self.scales() {
            scale.remove_css_class("dz-range-dragging");
        }
        if let Some(pointer) = pointer
            && pointer.changed
        {
            self.emit_by_name::<()>("value-committed", &[&pointer.values.to_variant(), &"drag"]);
        }
    }
    fn cancel_pointer(&self) {
        self.imp().pointer.borrow_mut().take();
        for scale in self.scales() {
            scale.remove_css_class("dz-range-dragging");
        }
    }
    fn create_scale(&self, index: usize) -> ThumbScale {
        let scale: ThumbScale = glib::Object::new();
        scale.imp().owner.set(Some(self));
        scale.imp().index.set(index);
        scale.set_adjustment(&gtk::Adjustment::new(
            self.imp().min.get(),
            self.imp().min.get(),
            self.imp().max.get(),
            self.imp().step.get(),
            self.imp().large.get(),
            0.,
        ));
        scale.set_draw_value(false);
        scale.set_flippable(true);
        scale.add_css_class("dz-slider");
        scale.add_css_class("dz-slider-layer");
        scale.set_cursor_from_name(Some("pointer"));
        fn untarget(widget: &gtk::Widget) {
            let mut child = widget.first_child();
            while let Some(w) = child {
                child = w.next_sibling();
                w.set_can_target(false);
                untarget(&w);
            }
        }
        untarget(scale.upcast_ref());
        let weak = self.downgrade();
        scale.connect_change_value(move |_, _, value| {
            if let Some(root) = weak.upgrade()
                && !root.imp().applying.get()
                && root.imp().pointer.borrow().is_none()
            {
                root.move_thumb(index, value, "input-change");
            }
            glib::Propagation::Stop
        });
        let weak = self.downgrade();
        scale.connect_value_changed(move |scale| {
            if let Some(root) = weak.upgrade()
                && !root.imp().applying.get()
            {
                root.move_thumb(index, scale.value(), "input-change");
            }
        });
        let controllers = scale.observe_controllers();
        let drag = (0..controllers.n_items())
            .find_map(|i| controllers.item(i).and_downcast::<gtk::GestureDrag>())
            .expect("Native GTK range drag");
        let capture = gtk::GestureClick::new();
        capture.set_button(1);
        capture.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = self.downgrade();
        capture.connect_pressed(move |_, _, x, y| {
            if let Some(root) = weak.upgrade() {
                let shift = root.imp().scales.borrow()[index].imp().shift.get();
                root.begin_pointer(index, x + shift, y);
            }
        });
        scale.add_controller(capture.clone());
        capture.group_with(&drag);
        let weak = self.downgrade();
        drag.connect_drag_begin(move |_, x, _| {
            if let Some(root) = weak.upgrade() {
                let shift = root.imp().scales.borrow()[index].imp().shift.get();
                root.pointer_to(x + shift);
            }
        });
        let weak = self.downgrade();
        drag.connect_drag_update(move |_, dx, _| {
            if let Some(root) = weak.upgrade() {
                let start = root
                    .imp()
                    .pointer
                    .borrow()
                    .as_ref()
                    .map(|p| (p.start, p.index, p.start_shift));
                if let Some((x, index, start_shift)) = start {
                    let shift = root.imp().scales.borrow()[index].imp().shift.get();
                    root.pointer_to(x + dx + shift - start_shift);
                }
            }
        });
        let weak = self.downgrade();
        drag.connect_drag_end(move |_, _, _| {
            if let Some(root) = weak.upgrade() {
                root.end_pointer();
            }
        });
        let weak = self.downgrade();
        drag.connect_cancel(move |_, _| {
            if let Some(root) = weak.upgrade() {
                root.cancel_pointer();
            }
        });
        let motion = gtk::EventControllerMotion::new();
        let weak = self.downgrade();
        motion.connect_motion(move |motion, x, y| {
            if let Some(root) = weak.upgrade()
                && let Some(widget) = motion.widget()
            {
                let shift = root.imp().scales.borrow()[index].imp().shift.get();
                if root.hit_thumb(index, x + shift, y) {
                    widget.add_css_class("dz-range-hover");
                } else {
                    widget.remove_css_class("dz-range-hover");
                }
            }
        });
        motion.connect_leave(|motion| {
            if let Some(widget) = motion.widget() {
                widget.remove_css_class("dz-range-hover");
            }
        });
        scale.add_controller(motion);
        let focus = gtk::EventControllerFocus::new();
        let weak = self.downgrade();
        focus.connect_enter(move |_| {
            if let Some(root) = weak.upgrade() {
                root.imp().active.set(Some(index));
                root.imp().last.set(Some(index));
                root.control().queue_draw();
            }
        });
        let weak = self.downgrade();
        focus.connect_leave(move |_| {
            if let Some(root) = weak.upgrade() {
                if root.imp().active.get() == Some(index) {
                    root.imp().active.set(None);
                }
                root.control().queue_draw();
            }
        });
        scale.add_controller(focus);
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = self.downgrade();
        keys.connect_key_pressed(move |_, key, _, mods| {
            use gtk::{
                gdk::{Key, ModifierType as M},
                glib::Propagation as P,
            };
            let Some(root) = weak.upgrade().filter(|r| r.is_sensitive()) else {
                return P::Proceed;
            };
            let values = root.values();
            if index >= values.len() {
                return P::Proceed;
            }
            let min = root.imp().min.get();
            let max = root.imp().max.get();
            let gap = root.imp().gap.get();
            let step = if mods.contains(M::SHIFT_MASK) {
                root.imp().large.get()
            } else {
                root.imp().step.get()
            };
            let rtl = root.direction() == gtk::TextDirection::Rtl;
            let delta = if key == Key::Up {
                Some(step)
            } else if key == Key::Down {
                Some(-step)
            } else if key == Key::Right {
                Some(if rtl { -step } else { step })
            } else if key == Key::Left {
                Some(if rtl { step } else { -step })
            } else if key == Key::Page_Up {
                Some(root.imp().large.get())
            } else if key == Key::Page_Down {
                Some(-root.imp().large.get())
            } else {
                None
            };
            let next = if let Some(delta) = delta {
                Some(
                    rounded(
                        snap(values[index], min, root.imp().step.get()) + delta,
                        precision(values[index])
                            .max(precision(delta))
                            .max(precision(min)),
                    )
                    .clamp(min, max),
                )
            } else if key == Key::Home {
                Some(if index > 0 {
                    values[index - 1] + gap
                } else {
                    min
                })
            } else if key == Key::End {
                Some(values.get(index + 1).map(|v| v - gap).unwrap_or(max))
            } else {
                None
            };
            if let Some(value) = next {
                root.move_thumb(index, value, "keyboard");
                P::Stop
            } else {
                P::Proceed
            }
        });
        scale.add_controller(keys);
        scale
    }
}
fn format_number(value: f64) -> String {
    let rounded = (value * 1000.).round() / 1000.;
    let text = rounded.to_string();
    let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
    let negative = whole.starts_with('-');
    let whole = whole.trim_start_matches('-');
    let mut formatted = String::new();
    for (i, c) in whole.chars().enumerate() {
        if i > 0 && (whole.len() - i) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(c);
    }
    if negative {
        formatted.insert(0, '-');
    }
    if !fraction.is_empty() {
        formatted.push('.');
        formatted.push_str(fraction);
    }
    formatted
}
pub fn slider_field(options: SliderOptions) -> Result<SliderField, &'static str> {
    SliderField::new(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_range_math_and_readings() {
        assert_eq!(pushed(&[20., 80.], 0, 90., 0., 100., 0.), [90., 90.]);
        assert_eq!(pushed(&[90., 90.], 0, 30., 0., 100., 0.), [30., 90.]);
        assert_eq!(pushed(&[20., 80.], 0, 100., 0., 100., 10.), [90., 100.]);
        assert_eq!(pushed(&[20., 80.], 1, 0., 0., 100., 10.), [0., 10.]);
        assert_eq!(snap(0.26, 0., 0.1), 0.3);
        assert_eq!(rounded(snap(0.2, 0., 0.1) + 0.1, 1), 0.3);
        assert_eq!(format_number(1234.1256), "1,234.126");
    }
}
