//! The shared six-week calendar, composed of native GTK buttons and grids.
//! GTK owns activation and focus; this coordinator supplies the calendar's
//! explicit day/week/month navigation and controlled selection rules.
use super::{calendar_date::CalendarDate, calendar_names::CalendarNames};
use crate::{Size, Theme, Tint, Variant, prelude::*};
use gtk::{gdk, glib, subclass::prelude::*};
use std::{
    cell::{Cell, OnceCell, RefCell},
    rc::Rc,
    sync::OnceLock,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarMode {
    #[default]
    Single,
    Range,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarSize {
    Small,
    #[default]
    Medium,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateRange {
    pub start: Option<CalendarDate>,
    pub end: Option<CalendarDate>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "DazzCalendarValue")]
pub enum CalendarValue {
    Single(Option<CalendarDate>),
    Range(DateRange),
}
impl Default for CalendarValue {
    fn default() -> Self {
        Self::Single(None)
    }
}
impl CalendarValue {
    pub fn range(self) -> DateRange {
        match self {
            Self::Single(day) => DateRange {
                start: day,
                end: day,
            },
            Self::Range(range) => range,
        }
    }
}
pub struct CalendarOptions {
    pub mode: CalendarMode,
    pub size: CalendarSize,
    pub months: usize,
    pub value: Option<CalendarValue>,
    pub default_value: Option<CalendarValue>,
    pub month: Option<CalendarDate>,
    pub default_month: Option<CalendarDate>,
    pub min: Option<CalendarDate>,
    pub max: Option<CalendarDate>,
    pub week_starts_on: u8,
    pub locale: Option<String>,
    pub today: Option<CalendarDate>,
    pub navigation: bool,
    pub tint: Tint,
}
impl Default for CalendarOptions {
    fn default() -> Self {
        Self {
            mode: CalendarMode::Single,
            size: CalendarSize::Medium,
            months: 1,
            value: None,
            default_value: None,
            month: None,
            default_month: None,
            min: None,
            max: None,
            week_starts_on: 1,
            locale: None,
            today: None,
            navigation: true,
            tint: Tint::Primary,
        }
    }
}
struct DayCell {
    date: Rc<Cell<CalendarDate>>,
    month_index: usize,
    day_index: usize,
    outside: Cell<bool>,
    named: Cell<bool>,
    cell: gtk::Box,
    button: gtk::Button,
    label: gtk::Label,
    dot: gtk::Box,
}
struct MonthView {
    title: gtk::Label,
    month: Cell<Option<CalendarDate>>,
    widget: gtk::Box,
}
struct FocusEntry {
    button: gtk::Button,
    day: Option<usize>,
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Calendar {
        pub(super) mode: Cell<CalendarMode>,
        pub(super) size: Cell<CalendarSize>,
        pub(super) count: Cell<usize>,
        pub(super) week_start: Cell<u8>,
        pub(super) today: Cell<Option<CalendarDate>>,
        pub(super) min: Cell<Option<CalendarDate>>,
        pub(super) max: Cell<Option<CalendarDate>>,
        pub(super) own_value: Cell<CalendarValue>,
        pub(super) controlled_value: Cell<Option<CalendarValue>>,
        pub(super) own_month: Cell<Option<CalendarDate>>,
        pub(super) controlled_month: Cell<Option<CalendarDate>>,
        pub(super) focused: Cell<Option<CalendarDate>>,
        pub(super) hovered: Cell<Option<CalendarDate>>,
        pub(super) names: RefCell<Option<CalendarNames>>,
        pub(super) days: RefCell<Vec<DayCell>>,
        pub(super) views: RefCell<Vec<MonthView>>,
        pub(super) focus_entries: RefCell<Vec<FocusEntry>>,
        pub(super) previous: OnceCell<gtk::Button>,
        pub(super) next: OnceCell<gtk::Button>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Calendar {
        const NAME: &'static str = "DazzCalendar";
        type Type = super::Calendar;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for Calendar {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("value-change")
                        .param_types([CalendarValue::static_type()])
                        .build(),
                    glib::subclass::Signal::builder("month-change")
                        .param_types([CalendarDate::static_type()])
                        .build(),
                ]
            })
        }
    }
    impl WidgetImpl for Calendar {
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            if matches!(
                direction,
                gtk::DirectionType::TabForward | gtk::DirectionType::TabBackward
            ) {
                self.obj().focus_tab(direction)
            } else {
                false
            }
        }
    }
    impl BoxImpl for Calendar {}
    #[derive(Default)]
    pub struct FocusBox;
    #[glib::object_subclass]
    impl ObjectSubclass for FocusBox {
        const NAME: &'static str = "DazzCalendarFocusBox";
        type Type = super::FocusBox;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for FocusBox {}
    impl WidgetImpl for FocusBox {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            // A focused day has z-index 1 in React. Snapshot its native cell
            // and row last so a neighbouring range cannot cover its outline.
            let obj = self.obj();
            let focused = obj.focus_child();
            for child in children(obj.upcast_ref()) {
                if Some(&child) != focused.as_ref() {
                    obj.snapshot_child(&child, snapshot);
                }
            }
            if let Some(child) = focused.filter(|c| c.should_layout()) {
                obj.snapshot_child(&child, snapshot);
            }
        }
    }
    impl BoxImpl for FocusBox {}
    #[derive(Default)]
    pub struct MonthLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for MonthLayout {
        const NAME: &'static str = "DazzCalendarMonthLayout";
        type Type = super::MonthLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for MonthLayout {}
    impl LayoutManagerImpl for MonthLayout {
        fn request_mode(&self, _: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            _: i32,
        ) -> (i32, i32, i32, i32) {
            let width = month_width(widget);
            if orientation == gtk::Orientation::Horizontal {
                return (width, width, -1, -1);
            }
            let gap = Theme::of(widget)
                .unwrap_or_default()
                .dimension("spacing-1")
                .unwrap()
                .round() as i32;
            let children = children(widget);
            let height = children
                .iter()
                .map(|w| {
                    w.measure(
                        orientation,
                        width.max(w.measure(gtk::Orientation::Horizontal, -1).0),
                    )
                    .1
                })
                .sum::<i32>()
                + gap * children.len().saturating_sub(1) as i32;
            (height, height, -1, -1)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, _: i32, baseline: i32) {
            let gap = Theme::of(widget)
                .unwrap_or_default()
                .dimension("spacing-1")
                .unwrap()
                .round() as i32;
            let mut y = 0;
            for child in children(widget) {
                let actual = width.max(child.measure(gtk::Orientation::Horizontal, -1).0);
                let height = child.measure(gtk::Orientation::Vertical, actual).1;
                child.size_allocate(&gtk::Allocation::new(0, y, actual, height), baseline);
                y += height + gap;
            }
        }
    }
}
glib::wrapper! { pub struct Calendar(ObjectSubclass<imp::Calendar>) @extends gtk::Widget,gtk::Box, @implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable; }
glib::wrapper! { pub struct FocusBox(ObjectSubclass<imp::FocusBox>) @extends gtk::Widget,gtk::Box, @implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable; }
impl FocusBox {
    fn new(orientation: gtk::Orientation) -> Self {
        let w: Self = glib::Object::new();
        w.set_orientation(orientation);
        w
    }
}
glib::wrapper! { pub struct MonthLayout(ObjectSubclass<imp::MonthLayout>) @extends gtk::LayoutManager; }
fn children(widget: &gtk::Widget) -> Vec<gtk::Widget> {
    let mut out = vec![];
    let mut child = widget.first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        if w.should_layout() {
            out.push(w);
        }
    }
    out
}
fn month_width(widget: &gtk::Widget) -> i32 {
    let mut root = widget.parent();
    let mut size = CalendarSize::Medium;
    while let Some(w) = root {
        root = w.parent();
        if let Some(calendar) = w.downcast_ref::<Calendar>() {
            size = calendar.imp().size.get();
            break;
        }
    }
    let theme = Theme::of(widget).unwrap_or_default();
    let cell = theme
        .dimension(if size == CalendarSize::Small {
            "calendar-small-cell-size"
        } else {
            "calendar-medium-cell-size"
        })
        .unwrap();
    (7. * cell + 6. * theme.dimension("calendar-cell-gap").unwrap()).round() as i32
}
fn class(w: &impl IsA<gtk::Widget>, name: &str, on: bool) {
    if on {
        w.add_css_class(name);
    } else {
        w.remove_css_class(name);
    }
}
impl Calendar {
    pub fn new(options: CalendarOptions) -> Result<Self, String> {
        if !(1..=2).contains(&options.months) {
            return Err("Calendar shows one or two months".into());
        }
        if options.week_starts_on > 6 {
            return Err("Calendar week starts on a day from 0 to 6".into());
        }
        for value in [options.value, options.default_value].into_iter().flatten() {
            if !matches!(
                (options.mode, value),
                (CalendarMode::Single, CalendarValue::Single(_))
                    | (CalendarMode::Range, CalendarValue::Range(_))
            ) {
                return Err("Calendar value must match its selection mode".into());
            }
        }
        let names = CalendarNames::new(options.locale.as_deref())?;
        let today = options.today.unwrap_or_else(CalendarDate::today);
        let empty = if options.mode == CalendarMode::Range {
            CalendarValue::Range(DateRange::default())
        } else {
            CalendarValue::Single(None)
        };
        let own = options.default_value.unwrap_or(empty);
        let start = options.value.unwrap_or(own).range().start;
        let w: Self = glib::Object::new();
        w.set_orientation(gtk::Orientation::Horizontal);
        w.set_hexpand(false);
        w.set_valign(gtk::Align::Start);
        w.set_accessible_role(gtk::AccessibleRole::Group);
        w.add_css_class("dz-calendar");
        w.imp().mode.set(options.mode);
        w.imp().size.set(options.size);
        w.imp().count.set(options.months);
        w.imp().week_start.set(options.week_starts_on);
        w.imp().today.set(Some(today));
        w.imp().min.set(options.min);
        w.imp().max.set(options.max);
        w.imp().own_value.set(own);
        w.imp().controlled_value.set(options.value);
        w.imp().own_month.set(Some(
            options
                .default_month
                .or(start)
                .unwrap_or(today)
                .first_of_month(),
        ));
        w.imp()
            .controlled_month
            .set(options.month.map(CalendarDate::first_of_month));
        w.imp().focused.set(Some(start.unwrap_or(today)));
        w.imp().names.replace(Some(names));
        w.set_dazz_tint(options.tint);
        w.set_dazz_size(if options.size == CalendarSize::Small {
            Size::Small
        } else {
            Size::Medium
        });
        for i in 0..options.months {
            w.build_month(i, options.navigation);
        }
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = w.downgrade();
        keys.connect_key_pressed(move |_, key, _, modifiers| {
            weak.upgrade()
                .map_or(glib::Propagation::Proceed, |w| w.key(key, modifiers))
        });
        w.add_controller(keys);
        let motion = gtk::EventControllerMotion::new();
        let weak = w.downgrade();
        motion.connect_leave(move |_| {
            if let Some(w) = weak.upgrade() {
                w.imp().hovered.set(None);
                w.paint();
            }
        });
        w.add_controller(motion);
        w.paint();
        Ok(w)
    }
    pub fn value(&self) -> CalendarValue {
        self.imp()
            .controlled_value
            .get()
            .unwrap_or(self.imp().own_value.get())
    }
    pub fn mode(&self) -> CalendarMode {
        self.imp().mode.get()
    }
    pub fn first_month(&self) -> CalendarDate {
        self.imp()
            .controlled_month
            .get()
            .or(self.imp().own_month.get())
            .unwrap()
    }
    pub fn day_buttons(&self) -> Vec<(CalendarDate, gtk::Button)> {
        self.imp()
            .days
            .borrow()
            .iter()
            .map(|c| (c.date.get(), c.button.clone()))
            .collect()
    }
    /// The first matching cell, including the neighbouring month's overflow.
    pub fn day_button(&self, date: CalendarDate) -> Option<gtk::Button> {
        self.imp()
            .days
            .borrow()
            .iter()
            .find(|c| c.date.get() == date)
            .map(|c| c.button.clone())
    }
    pub fn previous_button(&self) -> Option<gtk::Button> {
        self.imp().previous.get().cloned()
    }
    pub fn next_button(&self) -> Option<gtk::Button> {
        self.imp().next.get().cloned()
    }
    pub fn set_controlled_value(&self, value: Option<CalendarValue>) -> Result<(), String> {
        if value.is_some_and(|v| {
            !matches!(
                (self.mode(), v),
                (CalendarMode::Single, CalendarValue::Single(_))
                    | (CalendarMode::Range, CalendarValue::Range(_))
            )
        }) {
            return Err("Calendar value must match its selection mode".into());
        }
        self.imp().controlled_value.set(value);
        self.paint();
        Ok(())
    }
    pub fn set_controlled_month(&self, month: Option<CalendarDate>) {
        self.imp()
            .controlled_month
            .set(month.map(CalendarDate::first_of_month));
        self.paint();
    }
    pub fn set_bounds(&self, min: Option<CalendarDate>, max: Option<CalendarDate>) {
        self.imp().min.set(min);
        self.imp().max.set(max);
        self.paint();
    }
    pub fn set_today(&self, today: CalendarDate) {
        self.imp().today.set(Some(today));
        self.paint();
    }
    pub fn set_size(&self, size: CalendarSize) {
        if self.imp().size.replace(size) == size {
            return;
        }
        self.set_dazz_size(if size == CalendarSize::Small {
            Size::Small
        } else {
            Size::Medium
        });
        for view in self.imp().views.borrow().iter() {
            view.widget.queue_resize();
        }
        self.queue_resize();
    }
    pub fn connect_value_change(
        &self,
        f: impl Fn(&Self, CalendarValue) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("value-change", false, move |args| {
            f(&args[0].get::<Self>().unwrap(), args[1].get().unwrap());
            None
        })
    }
    pub fn connect_month_change(
        &self,
        f: impl Fn(&Self, CalendarDate) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("month-change", false, move |args| {
            f(&args[0].get::<Self>().unwrap(), args[1].get().unwrap());
            None
        })
    }
    pub fn request_month(&self, month: CalendarDate) {
        let month = month.first_of_month();
        if self.imp().controlled_month.get().is_none() {
            self.imp().own_month.set(Some(month));
        }
        self.paint();
        self.emit_by_name::<()>("month-change", &[&month]);
    }
    fn visible(&self, date: CalendarDate) -> bool {
        (0..self.imp().count.get())
            .any(|i| self.first_month().add_months(i as i32).same_month(date))
    }
    fn tab_stop(&self) -> CalendarDate {
        let focused = self.imp().focused.get().unwrap();
        let today = self.imp().today.get().unwrap();
        if self.visible(focused) {
            focused
        } else if self.visible(today) {
            today
        } else {
            self.first_month()
        }
    }
    fn unavailable(&self, date: CalendarDate) -> bool {
        self.imp().min.get().is_some_and(|min| date < min)
            || self.imp().max.get().is_some_and(|max| date > max)
    }
    pub fn pick_day(&self, day: CalendarDate) {
        self.imp().focused.set(Some(day));
        if self.unavailable(day) {
            self.paint();
            return;
        }
        let range = self.value().range();
        let next = if self.mode() == CalendarMode::Range {
            CalendarValue::Range(match (range.start, range.end) {
                (Some(start), None) if day < start => DateRange {
                    start: Some(day),
                    end: Some(start),
                },
                (Some(start), None) => DateRange {
                    start: Some(start),
                    end: Some(day),
                },
                _ => DateRange {
                    start: Some(day),
                    end: None,
                },
            })
        } else {
            CalendarValue::Single(Some(day))
        };
        if self.imp().controlled_value.get().is_none() {
            self.imp().own_value.set(next);
        }
        self.paint();
        self.emit_by_name::<()>("value-change", &[&next]);
    }
    pub fn move_focus(&self, day: CalendarDate) {
        self.imp().focused.set(Some(day));
        if !self.visible(day) {
            let forward = day > self.first_month();
            self.request_month(if forward {
                day.add_months(1 - self.imp().count.get() as i32)
            } else {
                day
            });
        }
        self.paint();
        if let Some(button) = self.day_button(day) {
            button.grab_focus();
        }
    }
    fn key(&self, key: gdk::Key, mods: gdk::ModifierType) -> glib::Propagation {
        let focused = self
            .root()
            .and_downcast::<gtk::Window>()
            .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
        let day = self
            .imp()
            .days
            .borrow()
            .iter()
            .find(|c| {
                focused.as_ref().is_some_and(|f| {
                    *f == c.button.clone().upcast::<gtk::Widget>() || f.is_ancestor(&c.button)
                })
            })
            .map(|c| c.date.get());
        let Some(day) = day else {
            return glib::Propagation::Proceed;
        };
        let next = match key {
            gdk::Key::Left | gdk::Key::KP_Left => day.add_days(-1),
            gdk::Key::Right | gdk::Key::KP_Right => day.add_days(1),
            gdk::Key::Up | gdk::Key::KP_Up => day.add_days(-7),
            gdk::Key::Down | gdk::Key::KP_Down => day.add_days(7),
            gdk::Key::Page_Up | gdk::Key::KP_Page_Up => {
                day.page_month(if mods.contains(gdk::ModifierType::SHIFT_MASK) {
                    -12
                } else {
                    -1
                })
            }
            gdk::Key::Page_Down | gdk::Key::KP_Page_Down => {
                day.page_month(if mods.contains(gdk::ModifierType::SHIFT_MASK) {
                    12
                } else {
                    1
                })
            }
            gdk::Key::Home | gdk::Key::KP_Home => day.add_days(
                -((i32::from(day.weekday()) - i32::from(self.imp().week_start.get()) + 7) % 7),
            ),
            gdk::Key::End | gdk::Key::KP_End => day.add_days(
                6 - ((i32::from(day.weekday()) - i32::from(self.imp().week_start.get()) + 7) % 7),
            ),
            gdk::Key::Return | gdk::Key::KP_Enter | gdk::Key::space | gdk::Key::KP_Space => {
                self.pick_day(day);
                return glib::Propagation::Stop;
            }
            _ => return glib::Propagation::Proceed,
        };
        self.move_focus(next);
        glib::Propagation::Stop
    }
    fn focus_tab(&self, direction: gtk::DirectionType) -> bool {
        let entries = self.imp().focus_entries.borrow();
        let days = self.imp().days.borrow();
        let stop = self.tab_stop();
        let focus = self
            .root()
            .and_downcast::<gtk::Window>()
            .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
        let current = entries.iter().position(|e| {
            focus.as_ref().is_some_and(|f| {
                *f == e.button.clone().upcast::<gtk::Widget>() || f.is_ancestor(&e.button)
            })
        });
        let eligible = |i: usize| {
            entries[i].button.is_sensitive()
                && entries[i]
                    .day
                    .is_none_or(|day| days[day].date.get() == stop && !days[day].outside.get())
        };
        let next = if direction == gtk::DirectionType::TabForward {
            (current.map_or(0, |i| i + 1)..entries.len()).find(|i| eligible(*i))
        } else {
            (0..current.unwrap_or(entries.len()))
                .rev()
                .find(|i| eligible(*i))
        };
        next.is_some_and(|i| entries[i].button.grab_focus())
    }
    fn build_month(&self, index: usize, navigation: bool) {
        let month = gtk::Box::new(gtk::Orientation::Vertical, 0);
        month.add_css_class("dz-calendar-month");
        month.set_hexpand(false);
        month.set_layout_manager(Some(glib::Object::new::<MonthLayout>()));
        let header = gtk::CenterBox::new();
        header.add_css_class("dz-calendar-head");
        let title = gtk::Label::new(None);
        title.add_css_class("dz-calendar-title");
        title.set_accessible_role(gtk::AccessibleRole::Status);
        title.set_hexpand(true);
        header.set_center_widget(Some(&title));
        if navigation {
            for previous in [true, false] {
                let button = if (previous && index == 0)
                    || (!previous && index + 1 == self.imp().count.get())
                {
                    let label = if previous {
                        "Previous month"
                    } else {
                        "Next month"
                    };
                    let button = crate::icon_button(
                        if previous {
                            "dazzui-ChevronLeft12Regular-symbolic"
                        } else {
                            "dazzui-ChevronRight12Regular-symbolic"
                        },
                        label,
                        Variant::Plain,
                        Tint::Neutral,
                        Size::Tiny,
                    );
                    attach_hint(&button, label);
                    let weak = self.downgrade();
                    button.connect_clicked(move |_| {
                        if let Some(w) = weak.upgrade() {
                            w.request_month(w.first_month().add_months(if previous {
                                -1
                            } else {
                                1
                            }));
                        }
                    });
                    self.imp().focus_entries.borrow_mut().push(FocusEntry {
                        button: button.clone(),
                        day: None,
                    });
                    if previous {
                        self.imp().previous.set(button.clone()).unwrap();
                    } else {
                        self.imp().next.set(button.clone()).unwrap();
                    }
                    Some(button)
                } else {
                    None
                };
                let child: gtk::Widget = button.map(|b| b.upcast()).unwrap_or_else(|| {
                    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    spacer.add_css_class("dz-calendar-spacer");
                    spacer.upcast()
                });
                if previous {
                    header.set_start_widget(Some(&child));
                } else {
                    header.set_end_widget(Some(&child));
                }
            }
        }
        month.append(&header);
        let weekdays = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        weekdays.add_css_class("dz-calendar-weekdays");
        weekdays.update_state(&[gtk::accessible::State::Hidden(true)]);
        let anchor = CalendarDate::new(2024, 1, 7)
            .unwrap()
            .add_days(i32::from(self.imp().week_start.get()));
        for i in 0..7 {
            let label = gtk::Label::new(Some(
                &self
                    .imp()
                    .names
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .weekday(anchor.add_days(i)),
            ));
            label.add_css_class("dz-calendar-weekday");
            weekdays.append(&label);
        }
        month.append(&weekdays);
        let grid = FocusBox::new(gtk::Orientation::Vertical);
        grid.add_css_class("dz-calendar-grid");
        grid.set_accessible_role(gtk::AccessibleRole::Grid);
        grid.update_relation(&[
            gtk::accessible::Relation::RowCount(6),
            gtk::accessible::Relation::ColCount(7),
        ]);
        for r in 0..6 {
            let row = FocusBox::new(gtk::Orientation::Horizontal);
            row.add_css_class("dz-calendar-row");
            row.set_accessible_role(gtk::AccessibleRole::Row);
            row.update_relation(&[gtk::accessible::Relation::RowIndex(r as i32 + 1)]);
            for c in 0..7 {
                let cell = gtk::Box::new(gtk::Orientation::Vertical, 0);
                cell.set_accessible_role(gtk::AccessibleRole::GridCell);
                cell.update_relation(&[
                    gtk::accessible::Relation::RowIndex(r as i32 + 1),
                    gtk::accessible::Relation::ColIndex(c as i32 + 1),
                ]);
                cell.add_css_class("dz-calendar-cell");
                let button = gtk::Button::new();
                button.add_css_class("dz-calendar-day");
                button.set_hexpand(false);
                button.set_focusable(true);
                let content = gtk::Overlay::new();
                content.add_css_class("dz-calendar-day-content");
                content.set_accessible_role(gtk::AccessibleRole::Presentation);
                let label = gtk::Label::new(None);
                label.set_accessible_role(gtk::AccessibleRole::Presentation);
                content.set_child(Some(&label));
                let dot = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                dot.add_css_class("dz-calendar-today-dot");
                dot.set_halign(gtk::Align::Center);
                dot.set_valign(gtk::Align::End);
                dot.set_can_target(false);
                dot.set_accessible_role(gtk::AccessibleRole::Presentation);
                content.add_overlay(&dot);
                content.set_measure_overlay(&dot, false);
                button.set_child(Some(&content));
                cell.append(&button);
                row.append(&cell);
                let date = Rc::new(Cell::new(self.imp().today.get().unwrap()));
                let weak = self.downgrade();
                let value = date.clone();
                button.connect_clicked(move |_| {
                    if let Some(w) = weak.upgrade() {
                        w.pick_day(value.get());
                    }
                });
                let focus = gtk::EventControllerFocus::new();
                let weak = self.downgrade();
                let value = date.clone();
                focus.connect_enter(move |_| {
                    if let Some(w) = weak.upgrade() {
                        w.imp().focused.set(Some(value.get()));
                        w.paint();
                    }
                });
                button.add_controller(focus);
                let motion = gtk::EventControllerMotion::new();
                let weak = self.downgrade();
                let value = date.clone();
                motion.connect_enter(move |_, _, _| {
                    if let Some(w) = weak.upgrade() {
                        w.imp().hovered.set(Some(value.get()));
                        w.paint();
                    }
                });
                button.add_controller(motion);
                let day = self.imp().days.borrow().len();
                self.imp().focus_entries.borrow_mut().push(FocusEntry {
                    button: button.clone(),
                    day: Some(day),
                });
                self.imp().days.borrow_mut().push(DayCell {
                    date,
                    month_index: index,
                    day_index: r * 7 + c,
                    outside: Cell::new(false),
                    named: Cell::new(false),
                    cell,
                    button,
                    label,
                    dot,
                });
            }
            grid.append(&row);
        }
        month.append(&grid);
        self.append(&month);
        self.imp().views.borrow_mut().push(MonthView {
            title,
            month: Cell::new(None),
            widget: month,
        });
    }
    fn paint(&self) {
        let names = self.imp().names.borrow();
        let Some(names) = names.as_ref() else {
            return;
        };
        let month = self.first_month();
        let today = self.imp().today.get().unwrap();
        let range = self.value().range();
        let pending =
            self.mode() == CalendarMode::Range && range.start.is_some() && range.end.is_none();
        let preview = if pending {
            self.imp().hovered.get().or(self.imp().focused.get())
        } else {
            None
        };
        let (a, b) = if pending {
            (range.start, preview)
        } else {
            (range.start, range.end)
        };
        let (lo, hi) = match (a, b) {
            (Some(a), Some(b)) if a > b => (Some(b), Some(a)),
            (Some(a), Some(b)) => (Some(a), Some(b)),
            (a, b) => (a.or(b), None),
        };
        let grids: Vec<_> = (0..self.imp().count.get())
            .map(|i| {
                month
                    .add_months(i as i32)
                    .month_grid(self.imp().week_start.get())
            })
            .collect();
        for (index, view) in self.imp().views.borrow().iter().enumerate() {
            let current = month.add_months(index as i32);
            if view.month.replace(Some(current)) != Some(current) {
                view.title.set_label(&names.title(current));
            }
        }
        // React's date keys remove the old focused cell on a month change.
        // Keep native buttons, but release that obsolete focus before a
        // keyboard move explicitly restores its requested day below.
        if let Some(window) = self.root().and_downcast::<gtk::Window>() {
            let focus = gtk::prelude::GtkWindowExt::focus(&window);
            let removed = self.imp().days.borrow().iter().any(|cell| {
                cell.date.get() != grids[cell.month_index][cell.day_index]
                    && focus.as_ref().is_some_and(|f| {
                        *f == cell.button.clone().upcast::<gtk::Widget>()
                            || f.is_ancestor(&cell.button)
                    })
            });
            if removed {
                gtk::prelude::GtkWindowExt::set_focus(&window, None::<&gtk::Widget>);
            }
        }
        for cell in self.imp().days.borrow().iter() {
            let day = grids[cell.month_index][cell.day_index];
            let changed = cell.date.replace(day) != day;
            let named = cell.named.replace(true);
            if changed || !named {
                cell.label.set_label(&day.day.to_string());
                cell.button.set_dazz_label(&names.full(day));
            }
            let outside = !month.add_months(cell.month_index as i32).same_month(day);
            cell.outside.set(outside);
            let picked = range.start == Some(day)
                || (self.mode() == CalendarMode::Range && range.end == Some(day));
            let span = self.mode() == CalendarMode::Range
                && lo.zip(hi).is_some_and(|(lo, hi)| day >= lo && day <= hi);
            cell.button.set_dazz_variant(if picked {
                Variant::Filled
            } else if span {
                Variant::Tinted
            } else {
                Variant::Plain
            });
            for (name, on) in [
                ("dz-outside", outside),
                ("dz-today", day == today),
                ("dz-selected", picked),
                ("dz-range", span),
                ("dz-range-start", span && lo == Some(day)),
                ("dz-range-end", span && hi == Some(day)),
                (
                    "dz-preview",
                    pending && preview == Some(day) && range.start != Some(day),
                ),
                ("dz-unavailable", self.unavailable(day)),
            ] {
                class(&cell.button, name, on);
            }
            cell.dot.set_visible(day == today);
            cell.cell
                .update_state(&[gtk::accessible::State::Selected(Some(picked || span))]);
            cell.button
                .update_state(&[gtk::accessible::State::Disabled(self.unavailable(day))]);
        }
    }
}
fn attach_hint(button: &gtk::Button, text: &str) {
    button.set_tooltip_text(None);
    let label = gtk::Label::new(Some(text));
    let popup = crate::rich_popover(button, &label, gtk::PositionType::Top, gtk::Align::Center);
    popup.remove_css_class("dz-rich-popover");
    popup.add_css_class("dz-tooltip");
    popup.set_accessible_role(gtk::AccessibleRole::Tooltip);
    popup.set_dazz_label(text);
    crate::hover_popup(
        button,
        &popup,
        Theme::default().duration("tooltip-open-delay").unwrap(),
        Theme::default().duration("tooltip-close-delay").unwrap(),
        false,
    );
}
