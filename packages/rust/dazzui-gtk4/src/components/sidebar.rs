//! Window navigation composed from native buttons, scroll containers and labels.
//! Collapsing retains each control, its focus state and host-owned selection.
use crate::{Size, Theme, Tint, Variant, prelude::*};
use gtk::{gdk, glib, subclass::prelude::*};
use std::{
    cell::{Cell, OnceCell, RefCell},
    rc::Rc,
    sync::OnceLock,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SidebarOptions {
    pub side: SidebarSide,
    pub variant: SidebarVariant,
    pub collapsible: SidebarCollapsible,
    /// Logical pixels; `None` resolves the appropriate ds frame width.
    pub width: Option<f64>,
    /// A separate column disclosure, taking precedence over its provider.
    pub collapsed: Option<bool>,
}
type ProviderList = Rc<RefCell<Vec<glib::WeakRef<SidebarProvider>>>>;
thread_local! {
    static WINDOWS: RefCell<Vec<(glib::WeakRef<gtk::Window>, ProviderList)>> = const { RefCell::new(vec![]) };
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Provider {
        pub own_open: Cell<bool>,
        pub controlled: Cell<Option<bool>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Provider {
        const NAME: &'static str = "DazzSidebarProvider";
        type Type = super::SidebarProvider;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for Provider {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("open-change")
                        .param_types([bool::static_type()])
                        .build(),
                    glib::subclass::Signal::builder("state-changed")
                        .param_types([bool::static_type()])
                        .build(),
                ]
            })
        }
    }
    impl WidgetImpl for Provider {
        fn root(&self) {
            self.parent_root();
            self.obj().watch_shortcut();
        }
        fn map(&self) {
            self.parent_map();
            self.obj().watch_shortcut();
        }
    }
    impl BoxImpl for Provider {}
    #[derive(Default)]
    pub struct Row;
    #[glib::object_subclass]
    impl ObjectSubclass for Row {
        const NAME: &'static str = "DazzSidebarProviderLayout";
        type Type = super::ProviderLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Row {}
    impl LayoutManagerImpl for Row {
        fn request_mode(&self, _: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let children = column_children(widget);
            if orientation == gtk::Orientation::Horizontal {
                let sizes: Vec<_> = children
                    .iter()
                    .map(|w| w.measure(orientation, -1))
                    .collect();
                return (
                    sizes.iter().map(|s| s.0).sum(),
                    sizes.iter().map(|s| s.1).sum(),
                    -1,
                    -1,
                );
            }
            let widths = provider_widths(&children, for_size);
            let mut min = 0;
            let mut natural = 0;
            for (child, width) in children.iter().zip(widths) {
                let (a, b, _, _) = child.measure(orientation, width);
                min = min.max(a);
                natural = natural.max(b);
            }
            (min, natural.max(min), -1, -1)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
            let children = column_children(widget);
            let mut cursor = 0;
            for (child, actual) in children.iter().zip(provider_widths(&children, width)) {
                let x = if widget.direction() == gtk::TextDirection::Rtl {
                    width - cursor - actual
                } else {
                    cursor
                };
                child.size_allocate(&gtk::Allocation::new(x, 0, actual, height), baseline);
                cursor += actual;
            }
        }
    }
    #[derive(Default)]
    pub struct Header;
    #[glib::object_subclass]
    impl ObjectSubclass for Header {
        const NAME: &'static str = "DazzSidebarHeader";
        type Type = super::SidebarHeader;
        type ParentType = gtk::Widget;
    }
    impl ObjectImpl for Header {
        fn dispose(&self) {
            if let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Header {
        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let Some(child) = self.obj().first_child() else {
                return (0, 0, -1, -1);
            };
            let minimum = child.measure(gtk::Orientation::Horizontal, -1).0;
            if orientation == gtk::Orientation::Horizontal {
                (0, child.measure(orientation, -1).1, -1, -1)
            } else {
                child.measure(orientation, for_size.max(minimum))
            }
        }
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            if let Some(child) = self.obj().first_child() {
                let actual = width.max(child.measure(gtk::Orientation::Horizontal, -1).0);
                let natural = child.measure(gtk::Orientation::Vertical, actual).1;
                let x = if self.obj().direction() == gtk::TextDirection::Rtl {
                    width - actual
                } else {
                    0
                };
                let transform = gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                    x as f32,
                    (height - natural) as f32 / 2.,
                ));
                child.allocate(actual, natural, baseline, Some(transform));
            }
        }
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            if let Some(child) = self.obj().first_child() {
                self.obj().snapshot_child(&child, snapshot);
            }
        }
    }
    #[derive(Default)]
    pub struct Column;
    #[glib::object_subclass]
    impl ObjectSubclass for Column {
        const NAME: &'static str = "DazzSidebarColumnLayout";
        type Type = super::ColumnLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Column {}
    impl LayoutManagerImpl for Column {
        fn request_mode(&self, _: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let children = column_children(widget);
            if orientation == gtk::Orientation::Horizontal {
                return (
                    0,
                    children
                        .iter()
                        .map(|c| c.measure(orientation, -1).1)
                        .max()
                        .unwrap_or_default(),
                    -1,
                    -1,
                );
            }
            let gap = column_gap(widget) * children.len().saturating_sub(1) as i32;
            let (mut min, mut natural) = (gap, gap);
            for child in children {
                let width = for_size.max(child.measure(gtk::Orientation::Horizontal, -1).0);
                let (a, b, _, _) = child.measure(orientation, width);
                min += a;
                natural += b;
            }
            (min, natural.max(min), -1, -1)
        }
        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
            let children = column_children(widget);
            let gap = column_gap(widget);
            let mut rows: Vec<_> = children
                .iter()
                .map(|child| {
                    let actual = width.max(child.measure(gtk::Orientation::Horizontal, -1).0);
                    let (minimum, natural, _, _) =
                        child.measure(gtk::Orientation::Vertical, actual);
                    (
                        actual,
                        minimum,
                        natural,
                        child.compute_expand(gtk::Orientation::Vertical),
                    )
                })
                .collect();
            let total =
                rows.iter().map(|r| r.2).sum::<i32>() + gap * rows.len().saturating_sub(1) as i32;
            let expandable = rows.iter().filter(|r| r.3).count() as i32;
            let mut remaining = height - total;
            let mut count = expandable;
            for row in &mut rows {
                if row.3 {
                    let extra = remaining / count;
                    let allocated = (row.2 + extra).max(row.1);
                    remaining -= allocated - row.2;
                    row.2 = allocated;
                    count -= 1;
                }
            }
            let mut y = 0;
            for (child, (actual, _, allocated, _)) in children.iter().zip(rows) {
                let x = if widget.direction() == gtk::TextDirection::Rtl {
                    width - actual
                } else {
                    0
                };
                child.size_allocate(&gtk::Allocation::new(x, y, actual, allocated), baseline);
                y += allocated + gap;
            }
        }
    }
    #[derive(Default)]
    pub struct Sidebar {
        pub container: OnceCell<gtk::Box>,
        pub body: OnceCell<gtk::Box>,
        pub provider: glib::WeakRef<super::SidebarProvider>,
        pub options: Cell<SidebarOptions>,
        pub open: Cell<bool>,
        pub amount: Cell<f64>,
        pub tick: RefCell<Option<gtk::TickCallbackId>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Sidebar {
        const NAME: &'static str = "DazzSidebar";
        type Type = super::Sidebar;
        type ParentType = gtk::Widget;
    }
    impl ObjectImpl for Sidebar {
        fn dispose(&self) {
            if let Some(tick) = self.tick.borrow_mut().take() {
                tick.remove();
            }
            if let Some(child) = self.container.get() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Sidebar {
        fn map(&self) {
            self.obj().refresh(false);
            self.parent_map();
        }
        fn direction_changed(&self, previous: gtk::TextDirection) {
            self.parent_direction_changed(previous);
            self.obj().queue_allocate();
        }
        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(&self, orientation: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            let obj = self.obj();
            if orientation == gtk::Orientation::Horizontal {
                let width = obj.widths().0.round() as i32;
                (width, width, -1, -1)
            } else if self.options.get().collapsible != SidebarCollapsible::None {
                // The sliding column is absolutely placed in the reference:
                // it stretches to the provider but adds no intrinsic height.
                (0, 0, -1, -1)
            } else {
                let container = self.container.get().unwrap();
                let minimum = container.measure(gtk::Orientation::Horizontal, -1).0;
                container.measure(orientation, (obj.widths().1.round() as i32).max(minimum))
            }
        }
        fn size_allocate(&self, _: i32, height: i32, baseline: i32) {
            let obj = self.obj();
            let (root, column) = obj.widths();
            let options = self.options.get();
            let leading =
                (options.side == SidebarSide::Left) != (obj.direction() == gtk::TextDirection::Rtl);
            let x = if leading && options.collapsible == SidebarCollapsible::Offcanvas {
                root - column
            } else {
                0.
            };
            let transform =
                gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(x as f32, 0.));
            self.container.get().unwrap().allocate(
                column.round() as i32,
                height,
                baseline,
                Some(transform),
            );
        }
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            if let Some(child) = self.container.get().filter(|child| child.get_visible()) {
                self.obj().snapshot_child(child, snapshot);
            }
        }
    }
}
glib::wrapper! {
    pub struct SidebarProvider(ObjectSubclass<imp::Provider>) @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}
glib::wrapper! { pub struct ProviderLayout(ObjectSubclass<imp::Row>) @extends gtk::LayoutManager; }
glib::wrapper! { pub struct ColumnLayout(ObjectSubclass<imp::Column>) @extends gtk::LayoutManager; }
glib::wrapper! { pub struct SidebarHeader(ObjectSubclass<imp::Header>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget; }
impl SidebarHeader {
    pub fn set_child(&self, child: &impl IsA<gtk::Widget>) {
        if let Some(old) = self.first_child() {
            old.unparent();
        }
        child.set_parent(self);
        self.queue_resize();
    }
}
fn column_children(widget: &gtk::Widget) -> Vec<gtk::Widget> {
    let mut children = vec![];
    let mut child = widget.first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        if w.should_layout() {
            children.push(w);
        }
    }
    children
}
fn provider_widths(children: &[gtk::Widget], available: i32) -> Vec<i32> {
    let sizes: Vec<_> = children
        .iter()
        .map(|w| w.measure(gtk::Orientation::Horizontal, -1))
        .collect();
    if available < 0 {
        return sizes.iter().map(|s| s.1).collect();
    }
    let expands: Vec<_> = children
        .iter()
        .map(|w| !w.is::<Sidebar>() && w.compute_expand(gtk::Orientation::Horizontal))
        .collect();
    let mut widths: Vec<_> = sizes
        .iter()
        .zip(&expands)
        .map(|(s, expand)| if *expand { s.0 } else { s.1 })
        .collect();
    let mut remaining = available - widths.iter().sum::<i32>();
    let mut count = expands.iter().filter(|e| **e).count() as i32;
    for (i, expand) in expands.iter().enumerate() {
        if *expand {
            let extra = (remaining / count).max(0);
            widths[i] += extra;
            remaining -= extra;
            count -= 1;
        }
    }
    widths
}
fn column_gap(widget: &gtk::Widget) -> i32 {
    let key = if widget.has_css_class("dz-sidebar-menu") {
        "frame-nav-gap"
    } else if widget.has_css_class("dz-sidebar-footer")
        || widget.has_css_class("dz-sidebar-header-content")
    {
        "spacing-2"
    } else {
        return 0;
    };
    Theme::of(widget)
        .unwrap_or_default()
        .dimension(key)
        .unwrap()
        .round() as i32
}
glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>) @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
impl SidebarProvider {
    pub fn new(default_open: bool) -> Self {
        let w: Self = glib::Object::new();
        w.set_orientation(gtk::Orientation::Horizontal);
        w.set_layout_manager(Some(glib::Object::new::<ProviderLayout>()));
        w.set_hexpand(true);
        w.add_css_class("dz-sidebar-provider");
        w.imp().own_open.set(default_open);
        w
    }
    pub fn is_open(&self) -> bool {
        self.imp()
            .controlled
            .get()
            .unwrap_or(self.imp().own_open.get())
    }
    pub fn set_controlled_open(&self, open: Option<bool>) {
        let old = self.is_open();
        self.imp().controlled.set(open);
        if self.is_open() != old {
            self.emit_by_name::<()>("state-changed", &[&self.is_open()]);
        }
    }
    pub fn request_open(&self, open: bool) {
        let old = self.is_open();
        if self.imp().controlled.get().is_none() {
            self.imp().own_open.set(open);
        }
        if old != self.is_open() {
            self.emit_by_name::<()>("state-changed", &[&self.is_open()]);
        }
        self.emit_by_name::<()>("open-change", &[&open]);
    }
    pub fn toggle(&self) {
        self.request_open(!self.is_open());
    }
    pub fn connect_open_change(&self, f: impl Fn(&Self, bool) + 'static) -> glib::SignalHandlerId {
        self.connect_local("open-change", false, move |args| {
            f(&args[0].get::<Self>().unwrap(), args[1].get().unwrap());
            None
        })
    }
    pub fn connect_state_changed(
        &self,
        f: impl Fn(&Self, bool) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("state-changed", false, move |args| {
            f(&args[0].get::<Self>().unwrap(), args[1].get().unwrap());
            None
        })
    }
    fn watch_shortcut(&self) {
        let Some(window) = self.root().and_downcast::<gtk::Window>() else {
            return;
        };
        WINDOWS.with_borrow_mut(|windows| {
            windows.retain(|(w, _)| w.upgrade().is_some());
            let providers = if let Some((_, providers)) = windows
                .iter()
                .find(|(w, _)| w.upgrade().as_ref() == Some(&window))
            {
                providers.clone()
            } else {
                let providers: ProviderList = Rc::new(RefCell::new(vec![]));
                let list = providers.clone();
                let owner = window.downgrade();
                let keys = gtk::EventControllerKey::new();
                keys.set_propagation_phase(gtk::PropagationPhase::Capture);
                keys.connect_key_pressed(move |_, key, _, modifiers| {
                    if key != gdk::Key::b
                        || !modifiers.intersects(
                            gdk::ModifierType::CONTROL_MASK | gdk::ModifierType::META_MASK,
                        )
                    {
                        return glib::Propagation::Proceed;
                    }
                    // A recorder consumes the key at its target in React.
                    // Let its native capture run before this window shortcut.
                    let mut focus = owner
                        .upgrade()
                        .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
                    while let Some(widget) = focus {
                        if widget
                            .downcast_ref::<crate::ShortcutRecorder>()
                            .is_some_and(|r| r.is_recording())
                        {
                            return glib::Propagation::Proceed;
                        }
                        focus = widget.parent();
                    }
                    let mounted: Vec<_> = list
                        .borrow()
                        .iter()
                        .filter_map(glib::WeakRef::upgrade)
                        .filter(|provider| {
                            provider.root().and_downcast::<gtk::Window>() == owner.upgrade()
                        })
                        .collect();
                    if mounted.is_empty() {
                        return glib::Propagation::Proceed;
                    }
                    for provider in mounted {
                        provider.toggle();
                    }
                    glib::Propagation::Stop
                });
                window.add_controller(keys);
                windows.push((window.downgrade(), providers.clone()));
                providers
            };
            providers.borrow_mut().retain(|w| w.upgrade().is_some());
            if !providers
                .borrow()
                .iter()
                .any(|w| w.upgrade().as_ref() == Some(self))
            {
                providers.borrow_mut().push(self.downgrade());
            }
        });
    }
}
impl Sidebar {
    pub fn new(provider: &SidebarProvider, options: SidebarOptions) -> Self {
        let w: Self = glib::Object::new();
        w.add_css_class("dz-sidebar");
        w.set_accessible_role(gtk::AccessibleRole::Navigation);
        w.imp().provider.set(Some(provider));
        w.imp().options.set(options);
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.add_css_class("dz-sidebar-container");
        container.set_layout_manager(Some(glib::Object::new::<ColumnLayout>()));
        let body = gtk::Box::new(gtk::Orientation::Vertical, 0);
        body.add_css_class("dz-sidebar-inner");
        body.set_layout_manager(Some(glib::Object::new::<ColumnLayout>()));
        body.set_overflow(gtk::Overflow::Hidden);
        body.set_vexpand(true);
        container.append(&body);
        container.set_parent(&w);
        w.imp().body.set(body).unwrap();
        w.imp().container.set(container).unwrap();
        let weak = w.downgrade();
        provider.connect_state_changed(move |_, _| {
            if let Some(w) = weak.upgrade() {
                w.refresh(true);
            }
        });
        w.refresh(false);
        w
    }
    pub fn append(&self, child: &impl IsA<gtk::Widget>) {
        self.imp().body.get().unwrap().append(child);
    }
    pub fn is_collapsed(&self) -> bool {
        !self.imp().open.get()
    }
    pub fn set_options(&self, options: SidebarOptions) {
        if self.imp().options.replace(options) != options {
            self.refresh(true);
        }
    }
    pub fn options(&self) -> SidebarOptions {
        self.imp().options.get()
    }
    fn widths(&self) -> (f64, f64) {
        let options = self.options();
        let theme = Theme::of(self).unwrap_or_default();
        let full = options.width.unwrap_or_else(|| {
            theme
                .dimension(if options.side == SidebarSide::Right {
                    "frame-aside-width"
                } else if options.collapsible == SidebarCollapsible::None {
                    "frame-rail-width"
                } else {
                    "frame-sidebar-width"
                })
                .unwrap()
        });
        let small = if options.collapsible == SidebarCollapsible::Icon {
            theme.dimension("frame-sidebar-icon-width").unwrap()
                + if options.variant != SidebarVariant::Sidebar {
                    theme.dimension("spacing-4").unwrap()
                } else {
                    0.
                }
        } else {
            0.
        };
        let amount = self.imp().amount.get();
        let width = small + (full - small) * amount;
        let column = if options.collapsible == SidebarCollapsible::Offcanvas {
            full
        } else if options.variant != SidebarVariant::Sidebar {
            width + 2. * theme.dimension("stroke-hairline").unwrap() * (1. - amount)
        } else {
            width
        };
        (width, column)
    }
    fn refresh(&self, animate: bool) {
        let options = self.options();
        let provider_open = self.imp().provider.upgrade().is_none_or(|p| p.is_open());
        let open = options.collapsible == SidebarCollapsible::None
            || !options.collapsed.unwrap_or(!provider_open);
        self.imp().open.set(open);
        let icon = !open && options.collapsible == SidebarCollapsible::Icon;
        let away = !open && options.collapsible == SidebarCollapsible::Offcanvas;
        for (class, yes) in [
            ("dz-sidebar-icons", icon),
            ("dz-sidebar-away", away),
            (
                "dz-sidebar-none",
                options.collapsible == SidebarCollapsible::None,
            ),
            ("dz-sidebar-right", options.side == SidebarSide::Right),
            (
                "dz-sidebar-floating",
                options.variant == SidebarVariant::Floating,
            ),
            (
                "dz-sidebar-inset-variant",
                options.variant == SidebarVariant::Inset,
            ),
        ] {
            if yes {
                self.add_css_class(class);
            } else {
                self.remove_css_class(class);
            }
        }
        let container = self.imp().container.get().unwrap();
        container.set_can_target(!away);
        container.set_can_focus(!away);
        container.update_state(&[gtk::accessible::State::Hidden(away)]);
        container.set_visible(true);
        fn hide_parts(widget: &gtk::Widget, icon: bool) {
            for class in [
                "dz-sidebar-caption",
                "dz-sidebar-group-label",
                "dz-sidebar-badge",
                "dz-sidebar-menu-sub",
                "dz-sidebar-group-action",
                "dz-sidebar-menu-action",
            ] {
                if widget.has_css_class(class) {
                    widget.set_visible(!icon);
                    return;
                }
            }
            let mut child = widget.first_child();
            while let Some(w) = child {
                child = w.next_sibling();
                if !w.is::<gtk::Popover>() {
                    hide_parts(&w, icon);
                }
            }
        }
        hide_parts(container.upcast_ref(), icon);
        let target = if open { 1. } else { 0. };
        if let Some(tick) = self.imp().tick.borrow_mut().take() {
            tick.remove();
        }
        let theme = Theme::of(self).unwrap_or_default();
        let enabled = self.settings().is_gtk_enable_animations();
        if !animate
            || !enabled
            || !self.is_mapped()
            || (self.imp().amount.get() - target).abs() < f64::EPSILON
        {
            self.imp().amount.set(target);
            container.set_visible(!away);
            self.queue_resize();
            return;
        }
        let from = self.imp().amount.get();
        let start = std::time::Instant::now();
        let duration = theme.duration("motion-duration").unwrap();
        let easing = easing(theme);
        let tick = self.add_tick_callback(move |w, _| {
            let time = (start.elapsed().as_secs_f64() / duration.as_secs_f64()).min(1.);
            w.imp()
                .amount
                .set(from + (target - from) * bezier(time, easing));
            w.queue_resize();
            if time >= 1. {
                w.imp().container.get().unwrap().set_visible(!away);
                w.imp().tick.borrow_mut().take();
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
        self.imp().tick.replace(Some(tick));
    }
}
fn easing(theme: Theme) -> [f64; 4] {
    let selector = format!(".dz-theme-{} {{\n", theme.name());
    let block = crate::TOKENS_CSS
        .split_once(&selector)
        .unwrap()
        .1
        .split_once("\n}")
        .unwrap()
        .0;
    let value = block
        .lines()
        .find_map(|l| l.trim().strip_prefix("--motion-easing: cubic-bezier("))
        .unwrap();
    let values: Vec<f64> = value
        .trim_end_matches(");")
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    values.try_into().unwrap()
}
fn bezier(time: f64, [x1, y1, x2, y2]: [f64; 4]) -> f64 {
    let coordinate = |t: f64, a: f64, b: f64| {
        3. * (1. - t).powi(2) * t * a + 3. * (1. - t) * t * t * b + t.powi(3)
    };
    let (mut low, mut high) = (0., 1.);
    for _ in 0..24 {
        let mid = (low + high) / 2.;
        if coordinate(mid, x1, x2) < time {
            low = mid;
        } else {
            high = mid;
        }
    }
    coordinate((low + high) / 2., y1, y2)
}
/// Disclosure trigger with native button focus and activation.
pub fn sidebar_trigger(provider: &SidebarProvider) -> gtk::Button {
    sidebar_trigger_label(provider, "Toggle sidebar")
}
/// The trigger with an accessible name also used by its hint.
pub fn sidebar_trigger_label(provider: &SidebarProvider, label: &str) -> gtk::Button {
    let button = crate::icon_button(
        "dazzui-PanelLeft20Regular-symbolic",
        label,
        Variant::Plain,
        Tint::Neutral,
        Size::Tiny,
    );
    button.set_tooltip_text(None);
    button.add_css_class("dz-sidebar-trigger");
    let update = |button: &gtk::Button, open| {
        button.update_state(&[gtk::accessible::State::Expanded(Some(open))])
    };
    update(&button, provider.is_open());
    let weak = button.downgrade();
    provider.connect_state_changed(move |_, open| {
        if let Some(button) = weak.upgrade() {
            update(&button, open);
        }
    });
    let weak = provider.downgrade();
    button.connect_clicked(move |_| {
        if let Some(provider) = weak.upgrade() {
            provider.toggle();
        }
    });
    sidebar_tooltip(&button, None, label, gtk::PositionType::Top);
    button
}
/// A passive row count; CSS supplies its fixed height while its native label
/// remains centred when the surrounding row uses a taller line box.
pub fn sidebar_menu_badge(text: &str) -> crate::TextBox {
    let badge = crate::TextBox::fixed_height(text);
    badge.add_css_class("dz-sidebar-badge");
    badge.set_can_target(false);
    badge.label().set_xalign(0.5);
    badge
}
fn sidebar_tooltip(
    button: &gtk::Button,
    provider: Option<&SidebarProvider>,
    text: &str,
    side: gtk::PositionType,
) {
    let label = gtk::Label::new(Some(text));
    let popup = crate::rich_popover(button, &label, side, gtk::Align::Center);
    popup.add_css_class("dz-tooltip");
    popup.remove_css_class("dz-rich-popover");
    popup.set_accessible_role(gtk::AccessibleRole::Tooltip);
    popup.set_dazz_label(text);
    button.update_relation(&[gtk::accessible::Relation::DescribedBy(
        &[popup.upcast_ref()],
    )]);
    let weak = provider.map(ObjectExt::downgrade);
    let gate = weak.clone();
    crate::hover_popup_when(
        button,
        &popup,
        std::time::Duration::ZERO,
        Theme::default().duration("tooltip-close-delay").unwrap(),
        false,
        move || {
            gate.as_ref()
                .is_none_or(|p| p.upgrade().is_some_and(|p| !p.is_open()))
        },
    );
    if let Some(provider) = provider {
        let popup = popup.downgrade();
        provider.connect_state_changed(move |_, open| {
            if open && let Some(popup) = popup.upgrade() {
                popup.popdown();
            }
        });
    }
}
pub fn sidebar_menu_tooltip(button: &gtk::Button, provider: &SidebarProvider, text: &str) {
    sidebar_tooltip(button, Some(provider), text, gtk::PositionType::Right);
}
pub fn sidebar_header() -> SidebarHeader {
    let w: SidebarHeader = glib::Object::new();
    w.add_css_class("dz-sidebar-header");
    w
}
pub fn sidebar_box(part: &str) -> gtk::Box {
    let w = gtk::Box::new(gtk::Orientation::Vertical, 0);
    w.add_css_class(&format!("dz-sidebar-{part}"));
    w.set_layout_manager(Some(glib::Object::new::<ColumnLayout>()));
    w
}
pub fn sidebar_menu_button(
    label: &str,
    icon: Option<&impl IsA<gtk::Widget>>,
    variant: Variant,
    tint: Tint,
    size: Size,
) -> gtk::Button {
    let button = gtk::Button::new();
    button.add_css_class("dz-sidebar-menu-button");
    button.set_dazz_variant(variant);
    button.set_dazz_tint(tint);
    button.set_dazz_size(size);
    button.set_dazz_label(label);
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    row.add_css_class("dz-sidebar-row-content");
    if let Some(icon) = icon {
        let slot = gtk::Overlay::new();
        slot.add_css_class("dz-sidebar-menu-icon");
        slot.set_child(Some(&gtk::Box::new(gtk::Orientation::Horizontal, 0)));
        icon.set_halign(gtk::Align::Center);
        icon.set_valign(gtk::Align::Center);
        slot.add_overlay(icon);
        slot.set_measure_overlay(icon, false);
        slot.set_clip_overlay(icon, false);
        row.append(&slot);
    }
    let caption = gtk::Label::new(Some(label));
    caption.add_css_class("dz-sidebar-caption");
    caption.set_xalign(0.);
    caption.set_hexpand(true);
    caption.set_ellipsize(gtk::pango::EllipsizeMode::End);
    row.append(&caption);
    button.set_child(Some(&row));
    button
}
pub fn set_sidebar_active(button: &gtk::Button, active: bool) {
    if active {
        button.add_css_class("dz-active");
    } else {
        button.remove_css_class("dz-active");
    }
    button.update_state(&[gtk::accessible::State::Selected(Some(active))]);
    if active {
        button.update_property(&[gtk::accessible::Property::Description("Current page")]);
    } else {
        button.reset_property(gtk::AccessibleProperty::Description);
    }
}
