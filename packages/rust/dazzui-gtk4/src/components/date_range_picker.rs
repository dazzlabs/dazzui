//! Native popover composition with an isolated draft. Only Done commits it.
use super::{
    calendar_names::CalendarNames,
    date_range_value::{self as dates, Draft},
};
use crate::{
    Calendar, CalendarDate, CalendarMode, CalendarOptions, CalendarSize, CalendarValue, DateRange,
    DateRangeValue, Size, Theme, Tint, Variant, prelude::*,
};
use gtk::{gio, glib, subclass::prelude::*};
use std::{
    cell::{Cell, OnceCell, RefCell},
    sync::OnceLock,
};
#[derive(Clone, Debug)]
pub struct DateRangePreset {
    pub label: String,
    pub days: (i32, i32),
}
#[derive(Clone, Debug)]
pub struct DateRangeDuration {
    pub label: String,
    pub minutes: i32,
}
#[derive(Clone, Debug)]
pub struct DateRangePickerLabels {
    pub placeholder: String,
    pub presets: String,
    pub start: String,
    pub end: String,
    pub duration: String,
    pub all_day: String,
    pub cancel: String,
    pub done: String,
}
impl Default for DateRangePickerLabels {
    fn default() -> Self {
        Self {
            placeholder: "Choose dates".into(),
            presets: "Quick pick".into(),
            start: "Start".into(),
            end: "End".into(),
            duration: "Duration".into(),
            all_day: "All day".into(),
            cancel: "Cancel".into(),
            done: "Done".into(),
        }
    }
}
pub struct DateRangePickerOptions {
    /// `None` is uncontrolled; `Some(None)` is a controlled empty value.
    pub value: Option<Option<DateRangeValue>>,
    pub default_value: Option<DateRangeValue>,
    pub presets: Option<Vec<DateRangePreset>>,
    pub durations: Vec<DateRangeDuration>,
    pub time_step: u16,
    pub months: usize,
    pub week_starts_on: u8,
    pub locale: Option<String>,
    pub today: Option<CalendarDate>,
    pub labels: DateRangePickerLabels,
    pub size: Size,
    pub tint: Tint,
    pub invalid: bool,
    pub disabled: bool,
}
impl Default for DateRangePickerOptions {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            presets: None,
            durations: vec![
                DateRangeDuration {
                    label: "30 min".into(),
                    minutes: 30,
                },
                DateRangeDuration {
                    label: "1 hour".into(),
                    minutes: 60,
                },
                DateRangeDuration {
                    label: "2 hours".into(),
                    minutes: 120,
                },
            ],
            time_step: 15,
            months: 1,
            week_starts_on: 1,
            locale: None,
            today: None,
            labels: Default::default(),
            size: Size::Medium,
            tint: Tint::Primary,
            invalid: false,
            disabled: false,
        }
    }
}
#[derive(Clone)]
struct Config {
    presets: Option<Vec<DateRangePreset>>,
    durations: Vec<DateRangeDuration>,
    time_step: u16,
    months: usize,
    week_start: u8,
    locale: Option<String>,
    today: CalendarDate,
    labels: DateRangePickerLabels,
    tint: Tint,
    invalid: bool,
}
struct Hooks {
    window: glib::WeakRef<gtk::Window>,
    controllers: Vec<gtk::EventController>,
    signals: Vec<glib::SignalHandlerId>,
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Picker {
        pub(super) config: RefCell<Option<Config>>,
        pub(super) names: RefCell<Option<CalendarNames>>,
        pub(super) own: RefCell<Option<DateRangeValue>>,
        pub(super) controlled: RefCell<Option<Option<DateRangeValue>>>,
        pub(super) draft: RefCell<Option<Draft>>,
        pub(super) popup: OnceCell<gtk::Popover>,
        pub(super) months: OnceCell<gtk::Box>,
        pub(super) calendar: RefCell<Option<Calendar>>,
        pub(super) summary: OnceCell<gtk::Box>,
        pub(super) footer_summary: OnceCell<gtk::Box>,
        pub(super) start_date: OnceCell<gtk::Label>,
        pub(super) end_date: OnceCell<gtk::Label>,
        pub(super) start_time: OnceCell<gtk::DropDown>,
        pub(super) end_time: OnceCell<gtk::DropDown>,
        pub(super) durations: RefCell<Vec<gtk::ToggleButton>>,
        pub(super) all_day: OnceCell<gtk::ToggleButton>,
        pub(super) presets: RefCell<Vec<gio::SimpleAction>>,
        pub(super) quick: OnceCell<gtk::Button>,
        pub(super) cancel: OnceCell<gtk::Button>,
        pub(super) done: OnceCell<gtk::Button>,
        pub(super) syncing: Cell<bool>,
        pub(super) hooks: RefCell<Option<Hooks>>,
        pub(super) focus_idle: Cell<bool>,
        pub(super) open: Cell<bool>,
        pub(super) presentation: RefCell<Option<glib::SourceId>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Picker {
        const NAME: &'static str = "DazzDateRangePicker";
        type Type = super::DateRangePicker;
        type ParentType = gtk::Button;
        fn class_init(c: &mut Self::Class) {
            c.set_layout_manager_type::<super::FieldLayout>();
        }
    }
    impl ObjectImpl for Picker {
        fn signals() -> &'static [glib::subclass::Signal] {
            static S: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            S.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("value-change")
                        .param_types([DateRangeValue::static_type()])
                        .build(),
                ]
            })
        }
        fn dispose(&self) {
            self.obj().cancel_presentation();
            self.obj().unwatch();
            if let Some(p) = self.popup.get() {
                p.unparent();
            }
        }
    }
    impl WidgetImpl for Picker {
        fn root(&self) {
            self.parent_root();
            self.obj().watch();
        }
        fn unroot(&self) {
            self.obj().close(false);
            self.obj().unwatch();
            self.parent_unroot();
        }
        fn unmap(&self) {
            self.obj().close(false);
            self.parent_unmap();
        }
    }
    impl ButtonImpl for Picker {}
    #[derive(Default)]
    pub struct FieldLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for FieldLayout {
        const NAME: &'static str = "DazzRangeFieldLayout";
        type Type = super::FieldLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for FieldLayout {}
    impl LayoutManagerImpl for FieldLayout {
        fn measure(&self, w: &gtk::Widget, o: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            if o == gtk::Orientation::Vertical {
                return (0, 0, -1, -1);
            }
            w.downcast_ref::<gtk::Button>()
                .unwrap()
                .child()
                .map_or((0, 0, -1, -1), |c| c.measure(o, -1))
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            if let Some(c) = w.downcast_ref::<gtk::Button>().unwrap().child() {
                let h = c.measure(gtk::Orientation::Vertical, width).1;
                c.allocate(
                    width,
                    h,
                    -1,
                    Some(
                        gtk::gsk::Transform::new()
                            .translate(&gtk::graphene::Point::new(0., (height - h) as f32 / 2.)),
                    ),
                );
            }
        }
    }
    #[derive(Default)]
    pub struct RunLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for RunLayout {
        const NAME: &'static str = "DazzRangeSummaryLayout";
        type Type = super::RunLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for RunLayout {}
    impl LayoutManagerImpl for RunLayout {
        fn measure(&self, w: &gtk::Widget, o: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            let children = children(w);
            if o == gtk::Orientation::Horizontal {
                (
                    0,
                    children.iter().map(run_width).sum::<f64>().round() as i32,
                    -1,
                    -1,
                )
            } else {
                let h = children
                    .iter()
                    .map(|c| c.measure(o, -1).1)
                    .max()
                    .unwrap_or(0);
                (h, h, -1, -1)
            }
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            let mut x = 0.;
            for c in children(w) {
                let n = run_width(&c);
                let h = c.measure(gtk::Orientation::Vertical, -1).1;
                let px = if w.direction() == gtk::TextDirection::Rtl {
                    f64::from(width) - x - n
                } else {
                    x
                };
                c.allocate(
                    n.ceil() as i32,
                    h,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            px as f32,
                            (height - h) as f32 / 2.,
                        )),
                    ),
                );
                x += n;
            }
        }
    }
    #[derive(Default)]
    pub struct SideLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for SideLayout {
        const NAME: &'static str = "DazzRangeSideLayout";
        type Type = super::SideLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for SideLayout {}
    impl LayoutManagerImpl for SideLayout {
        fn request_mode(&self, _: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            w: &gtk::Widget,
            o: gtk::Orientation,
            width: i32,
        ) -> (i32, i32, i32, i32) {
            if o == gtk::Orientation::Horizontal {
                return (0, 0, -1, -1);
            }
            let gap = metric(w, "spacing-3");
            let children = children(w);
            let width = if width < 0 {
                metric(w, "date-range-picker-side-width") - 2 * gap - metric(w, "stroke-hairline")
            } else {
                width
            };
            let h = children
                .iter()
                .map(|c| {
                    c.measure(o, width.max(c.measure(gtk::Orientation::Horizontal, -1).0))
                        .1
                })
                .sum::<i32>()
                + gap * children.len().saturating_sub(1) as i32;
            (h, h, -1, -1)
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, _: i32, _: i32) {
            let mut y = 0;
            for c in children(w) {
                let width = width.max(c.measure(gtk::Orientation::Horizontal, -1).0);
                let h = c.measure(gtk::Orientation::Vertical, width).1;
                c.size_allocate(&gtk::Allocation::new(0, y, width, h), -1);
                y += h + metric(w, "spacing-3");
            }
        }
    }
    #[derive(Default)]
    pub struct QuickLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for QuickLayout {
        const NAME: &'static str = "DazzRangeQuickLayout";
        type Type = super::QuickLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for QuickLayout {}
    impl LayoutManagerImpl for QuickLayout {
        fn measure(&self, w: &gtk::Widget, o: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            let c = w.first_child().unwrap();
            if o == gtk::Orientation::Horizontal {
                let n = c.measure(o, -1).1 - metric(w, "control-tiny-padding-inline");
                (0, n.max(0), -1, -1)
            } else {
                c.measure(o, -1)
            }
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            let c = w.first_child().unwrap();
            let n = c.measure(gtk::Orientation::Horizontal, -1).1;
            let pad = metric(w, "control-tiny-padding-inline");
            let x = if w.direction() == gtk::TextDirection::Rtl {
                width - n + pad
            } else {
                -pad
            };
            c.size_allocate(&gtk::Allocation::new(x, 0, n, height), -1);
        }
    }
    #[derive(Default)]
    pub struct ChipsLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for ChipsLayout {
        const NAME: &'static str = "DazzRangeChipsLayout";
        type Type = super::ChipsLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for ChipsLayout {}
    impl LayoutManagerImpl for ChipsLayout {
        fn request_mode(&self, _: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            w: &gtk::Widget,
            o: gtk::Orientation,
            width: i32,
        ) -> (i32, i32, i32, i32) {
            let children = children(w);
            if o == gtk::Orientation::Horizontal {
                let min = children
                    .iter()
                    .map(|c| c.measure(o, -1).0)
                    .max()
                    .unwrap_or(0);
                let nat = children.iter().map(|c| c.measure(o, -1).1).sum::<i32>()
                    + metric(w, "spacing-1") * children.len().saturating_sub(1) as i32;
                (min, nat, -1, -1)
            } else {
                let rows = chip_rows(w, if width < 0 { i32::MAX } else { width });
                let h = rows
                    .iter()
                    .map(|r| r.iter().map(|(_, _, h)| *h).max().unwrap_or(0))
                    .sum::<i32>()
                    + metric(w, "spacing-1") * rows.len().saturating_sub(1) as i32;
                (h, h, -1, -1)
            }
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, _: i32, _: i32) {
            let gap = metric(w, "spacing-1");
            let mut y = 0;
            for row in chip_rows(w, width) {
                let h = row.iter().map(|(_, _, h)| *h).max().unwrap_or(0);
                let mut x = 0;
                for (c, n, ch) in row {
                    let px = if w.direction() == gtk::TextDirection::Rtl {
                        width - x - n
                    } else {
                        x
                    };
                    c.size_allocate(&gtk::Allocation::new(px, y + (h - ch) / 2, n, ch), -1);
                    x += n + gap;
                }
                y += h + gap;
            }
        }
    }
}
glib::wrapper! {pub struct DateRangePicker(ObjectSubclass<imp::Picker>) @extends gtk::Widget,gtk::Button,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Actionable;}
glib::wrapper! {pub struct FieldLayout(ObjectSubclass<imp::FieldLayout>) @extends gtk::LayoutManager;}
glib::wrapper! {pub struct RunLayout(ObjectSubclass<imp::RunLayout>) @extends gtk::LayoutManager;}
glib::wrapper! {pub struct SideLayout(ObjectSubclass<imp::SideLayout>) @extends gtk::LayoutManager;}
glib::wrapper! {pub struct QuickLayout(ObjectSubclass<imp::QuickLayout>) @extends gtk::LayoutManager;}
glib::wrapper! {pub struct ChipsLayout(ObjectSubclass<imp::ChipsLayout>) @extends gtk::LayoutManager;}
fn run_width(w: &gtk::Widget) -> f64 {
    let natural = w.measure(gtk::Orientation::Horizontal, -1).1;
    if let Some(label) = w.downcast_ref::<gtk::Label>() {
        let layout = label.layout().copy();
        layout.set_width(-1);
        let advance = layout.size().0 as f64 / f64::from(gtk::pango::SCALE);
        f64::from(natural) + advance - advance.ceil()
    } else {
        f64::from(natural)
    }
}
fn metric(w: &impl IsA<gtk::Widget>, name: &str) -> i32 {
    Theme::of(w)
        .unwrap_or_default()
        .dimension(name)
        .unwrap_or_else(|| panic!("Missing range layout token {name}"))
        .round() as i32
}
fn children(w: &gtk::Widget) -> Vec<gtk::Widget> {
    let mut out = vec![];
    let mut c = w.first_child();
    while let Some(n) = c {
        c = n.next_sibling();
        if n.should_layout() {
            out.push(n);
        }
    }
    out
}
type ChipRow = Vec<(gtk::Widget, i32, i32)>;
fn chip_rows(w: &gtk::Widget, width: i32) -> Vec<ChipRow> {
    let mut rows = vec![];
    let mut row = vec![];
    let mut used = 0;
    let gap = metric(w, "spacing-1");
    for c in children(w) {
        let n = c.measure(gtk::Orientation::Horizontal, -1).1;
        let h = c.measure(gtk::Orientation::Vertical, n).1;
        if !row.is_empty() && used + gap + n > width {
            rows.push(std::mem::take(&mut row));
            used = 0;
        }
        if !row.is_empty() {
            used += gap;
        }
        used += n;
        row.push((c, n, h));
    }
    if !row.is_empty() {
        rows.push(row);
    }
    rows
}
fn label(text: &str, class: &str) -> gtk::Label {
    let w = gtk::Label::new(Some(text));
    w.add_css_class(class);
    w.set_xalign(0.);
    w
}
impl DateRangePicker {
    pub fn new(options: DateRangePickerOptions) -> Result<Self, String> {
        if !(1..=2).contains(&options.months)
            || options.week_starts_on > 6
            || options.time_step == 0
            || options.time_step > 1440
            || options.size == Size::Tiny
        {
            return Err("Invalid date-range picker geometry or time step".into());
        }
        let names = CalendarNames::new(options.locale.as_deref())?;
        let today = options.today.unwrap_or_else(CalendarDate::today);
        let w: Self = glib::Object::new();
        w.add_css_class("dz-date-range-trigger");
        w.set_dazz_size(options.size);
        w.set_sensitive(!options.disabled);
        w.set_valign(gtk::Align::Center);
        w.imp().own.replace(options.default_value);
        w.imp().controlled.replace(options.value);
        w.imp().draft.replace(Some(Draft::new(dates::blank(today))));
        w.imp().names.replace(Some(names));
        w.imp().config.replace(Some(Config {
            presets: options.presets,
            durations: options.durations,
            time_step: options.time_step,
            months: options.months,
            week_start: options.week_starts_on,
            locale: options.locale,
            today,
            labels: options.labels,
            tint: options.tint,
            invalid: options.invalid,
        }));
        let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        content.add_css_class("dz-range-trigger-content");
        let glyph = crate::icon("CalendarLtr16Regular");
        glyph.add_css_class("dz-range-leading");
        content.append(&glyph);
        let summary = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        summary.add_css_class("dz-range-summary");
        summary.set_layout_manager(Some(glib::Object::new::<RunLayout>()));
        summary.set_hexpand(true);
        summary.set_overflow(gtk::Overflow::Hidden);
        content.append(&summary);
        w.imp().summary.set(summary).unwrap();
        let chevron = crate::icon("ChevronDown12Regular");
        chevron.add_css_class("dz-range-chevron");
        content.append(&chevron);
        w.set_child(Some(&content));
        w.build_popup();
        let weak = w.downgrade();
        w.connect_clicked(move |_| {
            if let Some(w) = weak.upgrade() {
                if w.is_open() {
                    w.close(true);
                } else {
                    let _ = w.open();
                }
            }
        });
        w.paint_trigger();
        Ok(w)
    }
    fn config(&self) -> Config {
        self.imp().config.borrow().as_ref().unwrap().clone()
    }
    pub fn value(&self) -> Option<DateRangeValue> {
        self.imp()
            .controlled
            .borrow()
            .clone()
            .unwrap_or_else(|| self.imp().own.borrow().clone())
    }
    pub fn set_controlled_value(&self, value: Option<Option<DateRangeValue>>) {
        self.imp().controlled.replace(value);
        self.paint_trigger();
    }
    pub fn draft(&self) -> DateRangeValue {
        self.imp().draft.borrow().as_ref().unwrap().value.clone()
    }
    pub fn popup(&self) -> gtk::Popover {
        self.imp().popup.get().unwrap().clone()
    }
    pub fn calendar(&self) -> Option<Calendar> {
        self.imp().calendar.borrow().clone()
    }
    pub fn start_time(&self) -> gtk::DropDown {
        self.imp().start_time.get().unwrap().clone()
    }
    pub fn end_time(&self) -> gtk::DropDown {
        self.imp().end_time.get().unwrap().clone()
    }
    pub fn duration_buttons(&self) -> Vec<gtk::ToggleButton> {
        self.imp().durations.borrow().clone()
    }
    pub fn all_day_button(&self) -> gtk::ToggleButton {
        self.imp().all_day.get().unwrap().clone()
    }
    pub fn done_button(&self) -> gtk::Button {
        self.imp().done.get().unwrap().clone()
    }
    pub fn cancel_button(&self) -> gtk::Button {
        self.imp().cancel.get().unwrap().clone()
    }
    pub fn preset_button(&self) -> gtk::Button {
        self.imp().quick.get().unwrap().clone()
    }
    pub fn is_open(&self) -> bool {
        self.imp().open.get()
    }
    pub fn connect_value_change(
        &self,
        f: impl Fn(&Self, &DateRangeValue) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("value-change", false, move |v| {
            f(
                &v[0].get::<Self>().unwrap(),
                &v[1].get::<DateRangeValue>().unwrap(),
            );
            None
        })
    }
    pub fn set_appearance(&self, size: Size, tint: Tint, invalid: bool) {
        self.set_dazz_size(size);
        {
            let mut c = self.imp().config.borrow_mut();
            let c = c.as_mut().unwrap();
            c.tint = tint;
            c.invalid = invalid;
        }
        self.paint_trigger();
        if self.is_open() {
            self.paint_draft();
        }
    }
    pub fn open(&self) -> Result<(), String> {
        if !self.is_sensitive() || !self.is_mapped() || self.is_open() {
            return Ok(());
        }
        let c = self.config();
        let initial = self.value().unwrap_or_else(|| dates::blank(c.today));
        self.imp().draft.replace(Some(Draft::new(initial)));
        if let Some(old) = self.imp().calendar.borrow_mut().take() {
            self.imp().months.get().unwrap().remove(&old);
        }
        let (days, month) = {
            let draft = self.imp().draft.borrow();
            let draft = draft.as_ref().unwrap();
            (draft.days, draft.month)
        };
        let calendar = Calendar::new(CalendarOptions {
            mode: CalendarMode::Range,
            size: CalendarSize::Small,
            months: c.months,
            value: Some(CalendarValue::Range(days)),
            month: Some(month),
            today: Some(c.today),
            week_starts_on: c.week_start,
            locale: c.locale.clone(),
            tint: c.tint,
            ..Default::default()
        })?;
        let weak = self.downgrade();
        calendar.connect_value_change(move |_, value| {
            if let Some(w) = weak.upgrade()
                && let CalendarValue::Range(days) = value
            {
                let _ = w.set_days(days);
            }
        });
        let weak = self.downgrade();
        calendar.connect_month_change(move |_, month| {
            if let Some(w) = weak.upgrade() {
                w.imp().draft.borrow_mut().as_mut().unwrap().month = month;
                w.paint_draft();
            }
        });
        self.imp().months.get().unwrap().append(&calendar);
        self.imp().calendar.replace(Some(calendar));
        self.paint_draft();
        self.expanded(true);
        // Present after the current native signal, so closing or unmapping in
        // that signal can cancel the pending surface presentation.
        let weak = self.downgrade();
        let presentation = glib::idle_add_local_once(move || {
            if let Some(w) = weak.upgrade() {
                w.imp().presentation.borrow_mut().take();
                if w.is_open() && w.is_mapped() {
                    w.popup().popup();
                }
            }
        });
        self.imp().presentation.replace(Some(presentation));
        Ok(())
    }
    pub fn cancel(&self) {
        self.close(true);
    }
    pub fn commit(&self) {
        let next = self.draft();
        if self.imp().controlled.borrow().is_none() {
            self.imp().own.replace(Some(next.clone()));
        }
        self.paint_trigger();
        self.emit_by_name::<()>("value-change", &[&next]);
        self.close(true);
    }
    fn close(&self, restore: bool) {
        self.cancel_presentation();
        if let Some(p) = self.imp().popup.get() {
            p.popdown();
        }
        self.expanded(false);
        if restore {
            self.grab_focus();
        }
    }
    fn expanded(&self, open: bool) {
        self.imp().open.set(open);
        if open {
            self.add_css_class("dz-popup-open");
        } else {
            self.remove_css_class("dz-popup-open");
        }
        self.update_state(&[gtk::accessible::State::Expanded(Some(open))]);
    }
    fn cancel_presentation(&self) {
        if let Some(source) = self.imp().presentation.borrow_mut().take() {
            source.remove();
        }
    }
    pub fn set_days(&self, days: DateRange) -> Result<(), String> {
        self.imp()
            .draft
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_days(days)?;
        self.paint_draft();
        Ok(())
    }
    pub fn set_start_time(&self, time: &str) -> Result<(), String> {
        self.imp()
            .draft
            .borrow_mut()
            .as_mut()
            .unwrap()
            .start_time(time)?;
        self.paint_draft();
        Ok(())
    }
    pub fn set_end_time(&self, time: &str) -> Result<(), String> {
        self.imp()
            .draft
            .borrow_mut()
            .as_mut()
            .unwrap()
            .end_time(time)?;
        self.paint_draft();
        Ok(())
    }
    pub fn choose_duration(&self, index: usize) -> Result<(), String> {
        let d = self
            .config()
            .durations
            .get(index)
            .cloned()
            .ok_or("Unknown duration")?;
        if !self.draft().one_day() {
            return Ok(());
        }
        self.imp()
            .draft
            .borrow_mut()
            .as_mut()
            .unwrap()
            .duration(d.minutes)?;
        self.paint_draft();
        Ok(())
    }
    pub fn set_all_day(&self, all_day: bool) {
        self.imp()
            .draft
            .borrow_mut()
            .as_mut()
            .unwrap()
            .value
            .all_day = all_day;
        self.paint_draft();
    }
    fn presets(&self) -> Vec<DateRangePreset> {
        let c = self.config();
        c.presets.unwrap_or_else(|| {
            let today = c.today;
            let weekend = ((6 - i32::from(today.weekday()) + 7) % 7).max(0);
            let weekend = if weekend == 0 { 7 } else { weekend };
            let monday = (8 - i32::from(today.weekday())) % 7;
            let monday = if monday == 0 { 7 } else { monday };
            let first = 1 - i32::from(today.day);
            let end =
                today.first_of_month().add_months(1).add_days(-1).day as i32 - today.day as i32;
            [
                ("Today", (0, 0)),
                ("Tomorrow", (1, 1)),
                ("This weekend", (weekend, weekend + 1)),
                ("Next Monday", (monday, monday)),
                ("Next week", (monday, monday + 6)),
                ("This month", (first, end)),
            ]
            .into_iter()
            .map(|(label, days)| DateRangePreset {
                label: label.into(),
                days,
            })
            .collect()
        })
    }
    pub fn choose_preset(&self, index: usize) -> Result<(), String> {
        let c = self.config();
        let p = self.presets().get(index).cloned().ok_or("Unknown preset")?;
        let start = c.today.add_days(p.days.0);
        let end = c.today.add_days(p.days.1);
        {
            let mut draft = self.imp().draft.borrow_mut();
            let d = draft.as_mut().unwrap();
            d.month = start.first_of_month();
            d.set_days(DateRange {
                start: Some(start),
                end: Some(end),
            })?;
        }
        self.paint_draft();
        Ok(())
    }
    fn summary_parts(&self, value: &DateRangeValue) -> Vec<(String, bool)> {
        let names = self.imp().names.borrow();
        let names = names.as_ref().unwrap();
        let start = names.short_day(dates::day(&value.start));
        let end = names.short_day(dates::day(&value.end));
        if value.all_day {
            if value.one_day() {
                vec![
                    (start, false),
                    ("·".into(), true),
                    ("All day".into(), false),
                ]
            } else {
                vec![(start, false), ("→".into(), true), (end, false)]
            }
        } else if value.one_day() {
            vec![
                (start, false),
                ("·".into(), true),
                (dates::clock(&value.start), false),
                ("→".into(), true),
                (dates::clock(&value.end), false),
            ]
        } else {
            vec![
                (format!("{start} {}", dates::clock(&value.start)), false),
                ("→".into(), true),
                (format!("{end} {}", dates::clock(&value.end)), false),
            ]
        }
    }
    fn render_summary(&self, root: &gtk::Box, value: Option<&DateRangeValue>) {
        while let Some(c) = root.first_child() {
            root.remove(&c);
        }
        if let Some(value) = value {
            for (text, sep) in self.summary_parts(value) {
                let l = label(&text, if sep { "dz-range-sep" } else { "dz-range-text" });
                root.append(&l);
            }
        } else {
            root.append(&label(
                &self.config().labels.placeholder,
                "dz-range-placeholder",
            ));
        }
    }
    fn paint_trigger(&self) {
        let c = self.config();
        self.set_dazz_tint(if c.invalid { Tint::Danger } else { c.tint });
        if c.invalid {
            self.add_css_class("dz-invalid");
        } else {
            self.remove_css_class("dz-invalid");
        }
        self.update_state(&[gtk::accessible::State::Invalid(if c.invalid {
            gtk::AccessibleInvalidState::True
        } else {
            gtk::AccessibleInvalidState::False
        })]);
        let value = self.value();
        if let Some(root) = self.imp().summary.get() {
            self.render_summary(root, value.as_ref());
        }
        self.update_property(&[gtk::accessible::Property::ValueText(
            &value
                .as_ref()
                .map(|v| {
                    self.summary_parts(v)
                        .into_iter()
                        .map(|(s, _)| s)
                        .collect::<String>()
                })
                .unwrap_or_default(),
        )]);
    }
    fn paint_draft(&self) {
        if self.imp().syncing.replace(true) {
            return;
        }
        let c = self.config();
        let value = self.draft();
        let (month, days) = {
            let d = self.imp().draft.borrow();
            let d = d.as_ref().unwrap();
            (d.month, d.days)
        };
        self.popup().set_dazz_tint(c.tint);
        if let Some(calendar) = self.calendar() {
            let _ = calendar.set_controlled_value(Some(CalendarValue::Range(days)));
            calendar.set_controlled_month(Some(month));
            calendar.set_dazz_tint(c.tint);
        }
        let names = self.imp().names.borrow();
        let names = names.as_ref().unwrap();
        self.imp()
            .start_date
            .get()
            .unwrap()
            .set_label(&names.short_day(dates::day(&value.start)));
        self.imp()
            .end_date
            .get()
            .unwrap()
            .set_label(&names.short_day(dates::day(&value.end)));
        for (control, date) in [
            (self.start_time(), &value.start),
            (self.end_time(), &value.end),
        ] {
            update_times(&control, date, c.time_step);
            control.set_sensitive(!value.all_day);
        }
        for (index, b) in self.imp().durations.borrow().iter().enumerate() {
            b.set_active(
                !value.all_day && value.duration_minutes() == f64::from(c.durations[index].minutes),
            );
            b.set_sensitive(value.one_day());
            b.set_dazz_tint(c.tint);
        }
        self.all_day_button().set_active(value.all_day);
        self.all_day_button().set_dazz_tint(c.tint);
        self.done_button().set_dazz_tint(c.tint);
        for (action, p) in self.imp().presets.borrow().iter().zip(self.presets()) {
            let active = c.today.add_days(p.days.0) == dates::day(&value.start)
                && c.today.add_days(p.days.1) == dates::day(&value.end);
            action.set_state(&active.to_variant());
        }
        self.render_summary(self.imp().footer_summary.get().unwrap(), Some(&value));
        self.imp().syncing.set(false);
    }
    fn build_popup(&self) {
        let c = self.config();
        let body = gtk::Box::new(gtk::Orientation::Vertical, 0);
        body.add_css_class("dz-range-panel");
        let main = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let months = gtk::Box::new(gtk::Orientation::Vertical, 0);
        months.add_css_class("dz-range-months");
        main.append(&months);
        self.imp().months.set(months).unwrap();
        let side = gtk::Box::new(gtk::Orientation::Vertical, 0);
        side.add_css_class("dz-range-side");
        side.set_layout_manager(Some(glib::Object::new::<SideLayout>()));
        let quickrow = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        quickrow.set_layout_manager(Some(glib::Object::new::<QuickLayout>()));
        let quick = crate::glyph_button(
            &c.labels.presets,
            None,
            Some("dazzui-ChevronDown12Regular-symbolic"),
            Variant::Plain,
            Tint::Neutral,
            Size::Tiny,
        );
        quickrow.append(&quick);
        side.append(&quickrow);
        let model = gio::Menu::new();
        let group = gio::SimpleActionGroup::new();
        for (index, p) in self.presets().iter().enumerate() {
            let name = format!("preset{index}");
            let action = gio::SimpleAction::new_stateful(&name, None, &false.to_variant());
            let weak = self.downgrade();
            action.connect_activate(move |_, _| {
                if let Some(w) = weak.upgrade() {
                    let _ = w.choose_preset(index);
                }
            });
            group.add_action(&action);
            model.append(Some(&p.label), Some(&format!("range.{name}")));
            self.imp().presets.borrow_mut().push(action);
        }
        quick.insert_action_group("range", Some(&group));
        let menu = crate::menu_popup(&model);
        menu.set_dazz_tint(c.tint);
        let weak = self.downgrade();
        menu.connect_map(move |menu| {
            if let Some(w) = weak.upgrade() {
                menu.set_dazz_tint(w.config().tint);
            }
        });
        crate::attach_popover(&quick, &menu, gtk::PositionType::Bottom, gtk::Align::Start);
        let weak = menu.downgrade();
        quick.connect_clicked(move |_| {
            if let Some(p) = weak.upgrade() {
                p.popup();
            }
        });
        self.imp().quick.set(quick).unwrap();
        for (start, key) in [
            (true, c.labels.start.as_str()),
            (false, c.labels.end.as_str()),
        ] {
            let section = gtk::Box::new(gtk::Orientation::Vertical, 0);
            section.add_css_class("dz-range-end");
            section.append(&key_label(key));
            let row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            row.add_css_class("dz-range-end-row");
            let date = label("", "dz-range-date");
            date.set_hexpand(true);
            date.set_wrap(true);
            date.set_wrap_mode(gtk::pango::WrapMode::Word);
            row.append(&date);
            let time = crate::select(&["00:00"], Size::Small);
            time.add_css_class("dz-range-time");
            time.set_hexpand(false);
            time.set_dazz_label(&format!("{key} time"));
            row.append(&time);
            section.append(&row);
            side.append(&section);
            let weak = self.downgrade();
            time.connect_selected_notify(move |control| {
                if let Some(w) = weak.upgrade()
                    && !w.imp().syncing.get()
                    && let Some(item) = control.selected_item().and_downcast::<gtk::StringObject>()
                {
                    if start {
                        let _ = w.set_start_time(&item.string());
                    } else {
                        let _ = w.set_end_time(&item.string());
                    }
                }
            });
            if start {
                self.imp().start_date.set(date).unwrap();
                self.imp().start_time.set(time).unwrap();
            } else {
                self.imp().end_date.set(date).unwrap();
                self.imp().end_time.set(time).unwrap();
            }
        }
        let durations = gtk::Box::new(gtk::Orientation::Vertical, 0);
        durations.add_css_class("dz-range-durations");
        durations.append(&key_label(&c.labels.duration));
        let chips = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        chips.set_layout_manager(Some(glib::Object::new::<ChipsLayout>()));
        for (index, d) in c.durations.iter().enumerate() {
            let b = crate::toggle(&d.label, c.tint, Size::Tiny);
            b.set_layout_manager(Some(glib::Object::new::<super::button_layout::Layout>()));
            b.set_dazz_variant(Variant::Outlined);
            let weak = self.downgrade();
            b.connect_toggled(move |_| {
                if let Some(w) = weak.upgrade()
                    && !w.imp().syncing.get()
                {
                    let _ = w.choose_duration(index);
                }
            });
            chips.append(&b);
            self.imp().durations.borrow_mut().push(b);
        }
        let all = crate::toggle(&c.labels.all_day, c.tint, Size::Tiny);
        all.set_layout_manager(Some(glib::Object::new::<super::button_layout::Layout>()));
        all.set_dazz_variant(Variant::Outlined);
        let weak = self.downgrade();
        all.connect_toggled(move |b| {
            if let Some(w) = weak.upgrade()
                && !w.imp().syncing.get()
            {
                w.set_all_day(b.is_active());
            }
        });
        chips.append(&all);
        self.imp().all_day.set(all).unwrap();
        durations.append(&chips);
        side.append(&durations);
        main.append(&side);
        body.append(&main);
        let footer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        footer.add_css_class("dz-range-footer");
        let summary = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        summary.add_css_class("dz-range-footer-summary");
        summary.set_layout_manager(Some(glib::Object::new::<RunLayout>()));
        summary.set_hexpand(true);
        summary.set_overflow(gtk::Overflow::Hidden);
        footer.append(&summary);
        self.imp().footer_summary.set(summary).unwrap();
        let actions = crate::box_(gtk::Orientation::Horizontal, "2");
        let cancel = crate::button(
            &c.labels.cancel,
            Variant::Normal,
            Tint::Primary,
            Size::Small,
        );
        let weak = self.downgrade();
        cancel.connect_clicked(move |_| {
            if let Some(w) = weak.upgrade() {
                w.cancel();
            }
        });
        actions.append(&cancel);
        self.imp().cancel.set(cancel).unwrap();
        let done = crate::button(&c.labels.done, Variant::Filled, c.tint, Size::Small);
        let weak = self.downgrade();
        done.connect_clicked(move |_| {
            if let Some(w) = weak.upgrade() {
                w.commit();
            }
        });
        actions.append(&done);
        self.imp().done.set(done).unwrap();
        footer.append(&actions);
        body.append(&footer);
        let popup = crate::rich_popover(self, &body, gtk::PositionType::Bottom, gtk::Align::Start);
        popup.remove_css_class("dz-rich-popover");
        popup.add_css_class("dz-range-popup");
        popup.set_autohide(false);
        popup.set_can_focus(true);
        popup.set_accessible_role(gtk::AccessibleRole::Dialog);
        popup.set_dazz_label(&c.labels.placeholder);
        let weak = self.downgrade();
        popup.connect_map(move |_| {
            let weak = weak.clone();
            glib::idle_add_local_once(move || {
                if let Some(w) = weak.upgrade()
                    && w.is_open()
                    && let Some(b) = w.calendar().and_then(|c| c.previous_button())
                {
                    b.grab_focus();
                }
            });
        });
        let weak = self.downgrade();
        popup.connect_closed(move |_| {
            if let Some(w) = weak.upgrade() {
                w.expanded(false);
            }
        });
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        keys.set_propagation_limit(gtk::PropagationLimit::SameNative);
        let weak = self.downgrade();
        keys.connect_key_pressed(move |_, key, _, mods| {
            let Some(w) = weak.upgrade() else {
                return glib::Propagation::Proceed;
            };
            if key == gtk::gdk::Key::Escape {
                w.cancel();
                glib::Propagation::Stop
            } else if key == gtk::gdk::Key::Tab || key == gtk::gdk::Key::ISO_Left_Tab {
                w.tab(
                    mods.contains(gtk::gdk::ModifierType::SHIFT_MASK)
                        || key == gtk::gdk::Key::ISO_Left_Tab,
                );
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        popup.add_controller(keys);
        self.imp().popup.set(popup).unwrap();
    }
    fn tab(&self, back: bool) {
        let Some(window) = self.root().and_downcast::<gtk::Window>() else {
            return;
        };
        let focus = gtk::prelude::GtkWindowExt::focus(&window);
        let Some(calendar) = self.calendar() else {
            return;
        };
        let dir = if back {
            gtk::DirectionType::TabBackward
        } else {
            gtk::DirectionType::TabForward
        };
        if focus
            .as_ref()
            .is_some_and(|f| f == calendar.upcast_ref::<gtk::Widget>() || f.is_ancestor(&calendar))
        {
            if calendar.child_focus(dir) {
                return;
            }
            if back {
                self.close(true);
                return;
            }
            self.preset_button().grab_focus();
            return;
        }
        let mut controls: Vec<gtk::Widget> = vec![
            self.preset_button().upcast(),
            self.start_time().upcast(),
            self.end_time().upcast(),
        ];
        controls.extend(self.duration_buttons().into_iter().map(|b| b.upcast()));
        controls.push(self.all_day_button().upcast());
        controls.push(self.cancel_button().upcast());
        controls.push(self.done_button().upcast());
        let current = controls
            .iter()
            .position(|w| focus.as_ref().is_some_and(|f| f == w || f.is_ancestor(w)));
        let eligible = |i: usize| controls[i].is_sensitive();
        let next = if back {
            (0..current.unwrap_or(controls.len()))
                .rev()
                .find(|i| eligible(*i))
        } else {
            (current.map_or(0, |i| i + 1)..controls.len()).find(|i| eligible(*i))
        };
        if let Some(i) = next {
            controls[i].grab_focus();
        } else if back {
            calendar.child_focus(dir);
        } else {
            self.close(true);
            window.child_focus(dir);
        }
    }
    fn watch(&self) {
        self.unwatch();
        let Some(window) = self.root().and_downcast::<gtk::Window>() else {
            return;
        };
        let click = gtk::GestureClick::new();
        click.set_button(0);
        click.set_propagation_phase(gtk::PropagationPhase::Capture);
        click.set_propagation_limit(gtk::PropagationLimit::SameNative);
        let weak = self.downgrade();
        click.connect_pressed(move |c, _, x, y| {
            let Some(w) = weak.upgrade().filter(|w| w.is_open()) else {
                return;
            };
            let Some(window) = c.widget().and_downcast::<gtk::Window>() else {
                return;
            };
            let inside = window
                .pick(x, y, gtk::PickFlags::DEFAULT)
                .is_some_and(|p| p == w.clone().upcast::<gtk::Widget>() || p.is_ancestor(&w));
            if !inside {
                w.close(false);
            }
        });
        window.add_controller(click.clone());
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        keys.set_propagation_limit(gtk::PropagationLimit::SameNative);
        let weak = self.downgrade();
        keys.connect_key_pressed(move |_, key, _, _| {
            if key == gtk::gdk::Key::Escape
                && let Some(w) = weak.upgrade().filter(|w| w.is_open())
            {
                w.cancel();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        window.add_controller(keys.clone());
        let weak = self.downgrade();
        let signal = window.connect_notify_local(Some("focus-widget"), move |_, _| {
            let Some(w) = weak.upgrade().filter(|w| w.is_open()) else {
                return;
            };
            if w.imp().focus_idle.replace(true) {
                return;
            }
            let weak = w.downgrade();
            glib::idle_add_local_once(move || {
                if let Some(w) = weak.upgrade() {
                    w.imp().focus_idle.set(false);
                    if !w.is_open() {
                        return;
                    }
                    let focus = w
                        .root()
                        .and_downcast::<gtk::Window>()
                        .and_then(|r| gtk::prelude::GtkWindowExt::focus(&r));
                    if focus.is_some_and(|f| {
                        f != w.clone().upcast::<gtk::Widget>() && !f.is_ancestor(&w)
                    }) {
                        w.close(false);
                    }
                }
            });
        });
        self.imp().hooks.replace(Some(Hooks {
            window: window.downgrade(),
            controllers: vec![click.upcast(), keys.upcast()],
            signals: vec![signal],
        }));
    }
    fn unwatch(&self) {
        if let Some(h) = self.imp().hooks.borrow_mut().take()
            && let Some(w) = h.window.upgrade()
        {
            for c in h.controllers {
                w.remove_controller(&c);
            }
            for s in h.signals {
                w.disconnect(s);
            }
        }
    }
}
fn key_label(text: &str) -> gtk::Label {
    let l = label(&text.to_uppercase(), "dz-range-key");
    l.connect_map(|l| {
        let theme = Theme::of(l).unwrap_or_default();
        let spacing = theme.number("date-range-picker-key-tracking").unwrap()
            * theme.dimension("caption-small-font-size").unwrap();
        let attrs = gtk::pango::AttrList::new();
        attrs.insert(gtk::pango::AttrInt::new_letter_spacing(
            (spacing * f64::from(gtk::pango::SCALE)).round() as i32,
        ));
        l.set_attributes(Some(&attrs));
    });
    l
}
fn update_times(control: &gtk::DropDown, date: &glib::DateTime, step: u16) {
    let current = dates::clock(date);
    let mut labels = vec![];
    if (date.hour() * 60 + date.minute()) % i32::from(step) != 0 {
        labels.push(current.clone());
    }
    for m in (0..1440).step_by(step as usize) {
        labels.push(format!("{:02}:{:02}", m / 60, m % 60));
    }
    let same = control
        .model()
        .and_downcast::<gtk::StringList>()
        .is_some_and(|m| {
            m.n_items() == labels.len() as u32
                && labels
                    .iter()
                    .enumerate()
                    .all(|(i, s)| m.string(i as u32).as_deref() == Some(s))
        });
    if !same {
        let refs: Vec<_> = labels.iter().map(String::as_str).collect();
        control.set_model(Some(&gtk::StringList::new(&refs)));
    }
    if let Some(index) = labels.iter().position(|s| *s == current) {
        control.set_selected(index as u32);
    }
}
