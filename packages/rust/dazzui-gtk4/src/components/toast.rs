//! Window-scoped notification receipts over native GTK input and focus.
use crate::{DazzWidgetExt, Size, Theme, Tint, Variant, button, icon, prelude::*};
use gtk::{gdk, glib};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPlacement {
    Top,
    #[default]
    Bottom,
}

pub struct ToastAction {
    pub label: String,
    pub on_click: Rc<dyn Fn()>,
}

pub struct ToastOptions {
    pub id: Option<String>,
    pub title: String,
    pub tint: Tint,
    pub icon: Option<gtk::Widget>,
    pub action: Option<ToastAction>,
    pub dismissable: bool,
    pub dismiss_label: String,
    /// Absent uses the shared notice/action duration; danger defaults to zero.
    pub timeout: Option<Duration>,
    pub high_priority: Option<bool>,
    pub on_close: Option<Rc<dyn Fn()>>,
}
impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            id: None,
            title: String::new(),
            tint: Tint::Neutral,
            icon: None,
            action: None,
            dismissable: true,
            dismiss_label: "Dismiss".into(),
            timeout: None,
            high_priority: None,
            on_close: None,
        }
    }
}

struct Receipt {
    id: String,
    widget: gtk::Widget,
    remaining: Duration,
    timed: bool,
    on_close: Option<Rc<dyn Fn()>>,
    action: ActionCallback,
}
type ActionCallback = Rc<RefCell<Option<Rc<dyn Fn()>>>>;
type ScopeHandler = (glib::WeakRef<gtk::Widget>, glib::SignalHandlerId);
struct Hooks {
    window: glib::WeakRef<gtk::Window>,
    handlers: Vec<glib::SignalHandlerId>,
    keys: gtk::EventControllerKey,
    scopes: Vec<ScopeHandler>,
}
struct State {
    owner: glib::WeakRef<gtk::Widget>,
    stack: Stack,
    receipts: RefCell<Vec<Receipt>>,
    placement: ToastPlacement,
    limit: usize,
    serial: Cell<u64>,
    hover: Cell<bool>,
    pressed: Cell<bool>,
    paused: Cell<bool>,
    last: Cell<Instant>,
    timer: RefCell<Option<glib::SourceId>>,
    clock_busy: Cell<bool>,
    previous_focus: glib::WeakRef<gtk::Widget>,
    hooks: RefCell<Option<Hooks>>,
    theme: Cell<Option<Theme>>,
}
impl Drop for State {
    fn drop(&mut self) {
        self.cancel_timer();
        self.unwatch();
        self.detach();
    }
}
/// Install the application content layer before presenting a window. Toast
/// queues share it; it changes neither the content's measurement nor its state.
pub fn toast_layer(content: &impl IsA<gtk::Widget>) -> gtk::Overlay {
    let layer = gtk::Overlay::new();
    layer.add_css_class("dz-toast-layer");
    layer.set_child(Some(content));
    layer
}
fn find_layer(w: &gtk::Widget) -> Option<gtk::Overlay> {
    if w.has_css_class("dz-toast-layer") {
        return w.clone().downcast().ok();
    }
    children(w).iter().find_map(find_layer)
}
fn window_layer(window: &gtk::Window) -> Option<gtk::Overlay> {
    let content = window.child()?;
    if let Some(layer) = find_layer(&content) {
        return Some(layer);
    }
    let focus = gtk::prelude::GtkWindowExt::focus(window);
    window.set_child(None::<&gtk::Widget>);
    let layer = toast_layer(&content);
    window.set_child(Some(&layer));
    gtk::prelude::GtkWindowExt::set_focus(window, focus.as_ref());
    Some(layer)
}
/// Window-scoped receipts in a native overlay. Empty space never participates
/// in pointer picking; the empty notification region remains in the layout.
#[derive(Clone)]
pub struct ToastQueue(Rc<State>);
#[derive(Clone)]
pub struct WeakToastQueue(std::rc::Weak<State>);
impl WeakToastQueue {
    pub fn upgrade(&self) -> Option<ToastQueue> {
        self.0.upgrade().map(ToastQueue)
    }
}
impl ToastQueue {
    pub fn downgrade(&self) -> WeakToastQueue {
        WeakToastQueue(Rc::downgrade(&self.0))
    }
    pub fn new(owner: &impl IsA<gtk::Widget>, placement: ToastPlacement, limit: usize) -> Self {
        let stack: Stack = glib::Object::new();
        stack.add_css_class("dz-toast-popup");
        stack.add_css_class("dz-toast-viewport");
        stack.set_halign(gtk::Align::Fill);
        stack.set_valign(if placement == ToastPlacement::Top {
            gtk::Align::Start
        } else {
            gtk::Align::End
        });
        stack.set_focusable(true);
        stack.set_accessible_role(gtk::AccessibleRole::Region);
        stack.set_dazz_label("Notifications");
        let state = Rc::new(State {
            owner: owner.as_ref().downgrade(),
            stack,
            receipts: RefCell::new(vec![]),
            placement,
            limit,
            serial: Cell::new(0),
            hover: Cell::new(false),
            pressed: Cell::new(false),
            paused: Cell::new(true),
            last: Cell::new(Instant::now()),
            timer: RefCell::new(None),
            clock_busy: Cell::new(false),
            previous_focus: glib::WeakRef::new(),
            hooks: RefCell::new(None),
            theme: Cell::new(None),
        });
        let motion = gtk::EventControllerMotion::new();
        let weak = Rc::downgrade(&state);
        motion.connect_enter(move |_, _, _| {
            if let Some(s) = weak.upgrade() {
                s.hover.set(true);
                s.retime();
            }
        });
        let weak = Rc::downgrade(&state);
        motion.connect_leave(move |_| {
            if let Some(s) = weak.upgrade() {
                s.hover.set(false);
                s.retime();
            }
        });
        state.stack.add_controller(motion);
        let touch = gtk::GestureClick::new();
        touch.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(&state);
        touch.connect_pressed(move |_, _, _, _| {
            if let Some(s) = weak.upgrade() {
                s.pressed.set(true);
                s.retime();
            }
        });
        let weak = Rc::downgrade(&state);
        touch.connect_released(move |_, _, _, _| {
            if let Some(s) = weak.upgrade() {
                s.pressed.set(false);
                s.retime();
            }
        });
        let weak = Rc::downgrade(&state);
        touch.connect_cancel(move |_, _| {
            if let Some(s) = weak.upgrade() {
                s.pressed.set(false);
                s.retime();
            }
        });
        state.stack.add_controller(touch);
        let weak = Rc::downgrade(&state);
        owner.connect_map(move |_| {
            let weak = weak.clone();
            glib::idle_add_local_once(move || {
                if let Some(s) = weak.upgrade() {
                    s.mount();
                }
            });
        });
        let weak = Rc::downgrade(&state);
        owner.connect_unmap(move |_| {
            let weak = weak.clone();
            glib::idle_add_local_once(move || {
                if let Some(s) = weak.upgrade()
                    && s.window().is_none()
                {
                    s.clear(false);
                    s.unwatch();
                    s.detach();
                }
            });
        });
        let weak = Rc::downgrade(&state);
        owner.connect_destroy(move |_| {
            if let Some(s) = weak.upgrade() {
                s.clear(false);
                s.unwatch();
                s.detach();
            }
        });
        if owner.is_mapped() {
            state.mount();
        }
        Self(state)
    }
    pub fn add(&self, options: ToastOptions) -> String {
        self.0.add(options)
    }
    pub fn close(&self, id: &str) {
        self.0.close(id);
    }
    pub fn close_all(&self) {
        self.0.clear(true);
    }
    pub fn popup(&self) -> &gtk::Widget {
        self.0.stack.upcast_ref()
    }
    pub fn viewport(&self) -> &gtk::Widget {
        self.0.stack.upcast_ref()
    }
    pub fn stack(&self) -> &gtk::Widget {
        self.0.stack.upcast_ref()
    }
    pub fn ids(&self) -> Vec<String> {
        self.0
            .receipts
            .borrow()
            .iter()
            .map(|r| r.id.clone())
            .collect()
    }
}
impl State {
    fn window(&self) -> Option<gtk::Window> {
        self.owner.upgrade()?.root().and_downcast()
    }
    fn inside(&self, w: &gtk::Widget) -> bool {
        w == self.stack.upcast_ref::<gtk::Widget>() || w.is_ancestor(&self.stack)
    }
    fn detach(&self) {
        if let Some(parent) = self.stack.parent() {
            if let Ok(layer) = parent.downcast::<gtk::Overlay>() {
                layer.remove_overlay(&self.stack);
            } else {
                self.stack.unparent();
            }
        }
    }
    fn unwatch(&self) {
        if let Some(h) = self.hooks.borrow_mut().take() {
            if let Some(window) = h.window.upgrade() {
                for id in h.handlers {
                    window.disconnect(id);
                }
                window.remove_controller(&h.keys);
            }
            for (w, id) in h.scopes {
                if let Some(w) = w.upgrade() {
                    w.disconnect(id);
                }
            }
        }
    }
    fn cancel_timer(&self) {
        if let Some(timer) = self.timer.borrow_mut().take() {
            timer.remove();
        }
    }
    fn sync_theme(&self) {
        if let Some(owner) = self.owner.upgrade() {
            let theme = Theme::of(&owner).unwrap_or_default();
            if self.theme.replace(Some(theme)) != Some(theme) {
                theme.apply(&self.stack);
            }
            self.stack.set_direction(owner.direction());
            for r in self.receipts.borrow().iter() {
                r.widget.set_direction(owner.direction());
            }
        }
    }
    fn mount(self: &Rc<Self>) {
        let Some(window) = self.window() else {
            return;
        };
        let Some(layer) = window_layer(&window) else {
            return;
        };
        if self.stack.parent().as_ref() != Some(layer.upcast_ref()) {
            self.detach();
            layer.add_overlay(&self.stack);
        }
        self.sync_theme();
        self.watch(&window);
        self.retime();
    }
    fn watch(self: &Rc<Self>, window: &gtk::Window) {
        let mut scopes = vec![];
        let mut ancestor = self.owner.upgrade();
        while let Some(w) = ancestor {
            ancestor = w.parent();
            scopes.push(w);
        }
        if self.hooks.borrow().as_ref().is_some_and(|h| {
            h.window.upgrade().as_ref() == Some(window)
                && h.scopes.len() == scopes.len()
                && h.scopes
                    .iter()
                    .zip(&scopes)
                    .all(|((weak, _), w)| weak.upgrade().as_ref() == Some(w))
        }) {
            return;
        }
        self.unwatch();
        let weak = Rc::downgrade(self);
        let active = window.connect_is_active_notify(move |_| {
            if let Some(s) = weak.upgrade() {
                s.retime();
            }
        });
        let weak = Rc::downgrade(self);
        let focus = window.connect_focus_widget_notify(move |_| {
            if let Some(s) = weak.upgrade() {
                s.retime();
            }
        });
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        keys.set_propagation_limit(gtk::PropagationLimit::None);
        let weak = Rc::downgrade(self);
        keys.connect_key_pressed(move |_, key, _, mods| {
            let Some(s) = weak.upgrade() else {
                return glib::Propagation::Proceed;
            };
            let Some(window) = s.window() else {
                return glib::Propagation::Proceed;
            };
            let focus = gtk::prelude::GtkWindowExt::focus(&window);
            if key == gdk::Key::F6 && focus.as_ref() != Some(s.stack.upcast_ref()) {
                if focus.as_ref().is_some_and(|w| !s.inside(w)) {
                    s.previous_focus.set(focus.as_ref());
                }
                s.stack.grab_focus();
                return glib::Propagation::Stop;
            }
            if matches!(key, gdk::Key::Tab | gdk::Key::ISO_Left_Tab)
                && focus.as_ref().is_some_and(|w| s.inside(w))
            {
                let mut stops = vec![s.stack.clone().upcast::<gtk::Widget>()];
                for r in s.receipts.borrow().iter().take(s.limit) {
                    stops.push(r.widget.clone());
                    stops.extend(
                        children(&r.widget)
                            .into_iter()
                            .filter(|w| w.is_focusable() && w.is_sensitive()),
                    );
                }
                let index = stops.iter().position(|w| Some(w) == focus.as_ref());
                let backwards =
                    key == gdk::Key::ISO_Left_Tab || mods.contains(gdk::ModifierType::SHIFT_MASK);
                let next = index
                    .and_then(|i| {
                        if backwards {
                            i.checked_sub(1)
                        } else {
                            Some(i + 1)
                        }
                    })
                    .and_then(|i| stops.get(i));
                if let Some(next) = next {
                    next.grab_focus();
                } else {
                    s.restore_focus();
                }
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        window.add_controller(keys.clone());
        let scopes = scopes
            .into_iter()
            .map(|w| {
                let weak = Rc::downgrade(self);
                let id = w.connect_css_classes_notify(move |_| {
                    if let Some(s) = weak.upgrade() {
                        s.sync_theme();
                    }
                });
                (w.downgrade(), id)
            })
            .collect();
        *self.hooks.borrow_mut() = Some(Hooks {
            window: window.downgrade(),
            handlers: vec![active, focus],
            keys,
            scopes,
        });
    }
    fn add(self: &Rc<Self>, options: ToastOptions) -> String {
        self.mount();
        self.retime();
        self.serial.set(self.serial.get() + 1);
        let id = options
            .id
            .clone()
            .unwrap_or_else(|| format!("dazzui-toast-{}", self.serial.get()));
        let Some(window) = self.window() else {
            return id;
        };
        let theme = Theme::of(&self.stack).unwrap_or_default();
        let timeout = options.timeout.unwrap_or_else(|| {
            if options.tint == Tint::Danger {
                Duration::ZERO
            } else {
                theme
                    .duration(if options.action.is_some() {
                        "toast-action-duration"
                    } else {
                        "toast-notice-duration"
                    })
                    .unwrap()
            }
        });
        let old_index = self.receipts.borrow().iter().position(|r| r.id == id);
        let index = old_index.unwrap_or(0);
        let old = old_index.map(|i| self.receipts.borrow_mut().remove(i));
        let saved_focus = gtk::prelude::GtkWindowExt::focus(&window).filter(|f| {
            old.as_ref()
                .is_some_and(|r| f == &r.widget || f.is_ancestor(&r.widget))
        });
        let action_callback = old.as_ref().map(|r| r.action.clone()).unwrap_or_default();
        *action_callback.borrow_mut() = options.action.as_ref().map(|a| a.on_click.clone());
        let callbacks = action_callback.clone();
        let weak = Rc::downgrade(self);
        let key = id.clone();
        let widget = card(
            &options,
            old.as_ref().map(|r| &r.widget),
            move |button, is_action| {
                let callbacks = callbacks.clone();
                let weak = weak.clone();
                let key = key.clone();
                button.connect_clicked(move |_| {
                    if is_action {
                        let action = callbacks.borrow().clone();
                        if let Some(action) = action {
                            action();
                        }
                    }
                    if let Some(s) = weak.upgrade() {
                        s.close(&key);
                    }
                });
            },
        );
        widget.set_halign(gtk::Align::Center);
        let replaced = old.as_ref().is_none_or(|r| r.widget != widget);
        if let Some(old) = &old
            && replaced
        {
            old.widget.unparent();
        }
        if replaced {
            let keys = gtk::EventControllerKey::new();
            keys.set_propagation_phase(gtk::PropagationPhase::Capture);
            let weak = Rc::downgrade(self);
            let key = id.clone();
            keys.connect_key_pressed(move |_, pressed, _, _| {
                if pressed == gdk::Key::Escape
                    && let Some(s) = weak.upgrade()
                {
                    s.close(&key);
                    return glib::Propagation::Stop;
                }
                glib::Propagation::Proceed
            });
            widget.add_controller(keys);
        }
        self.receipts.borrow_mut().insert(
            index,
            Receipt {
                id: id.clone(),
                widget: widget.clone(),
                remaining: timeout,
                timed: !timeout.is_zero(),
                on_close: options.on_close,
                action: action_callback,
            },
        );
        self.reorder();
        if let Some(focus) = saved_focus {
            if old.as_ref().is_some_and(|r| r.widget == focus) {
                widget.grab_focus();
            } else {
                focus.grab_focus();
            }
        }
        self.retime();
        // GTK's macOS AT context currently has a null announce vfunc. Native
        // roles and labels remain available there; direct announcements crash.
        #[cfg(not(target_os = "macos"))]
        self.stack.announce(
            &options.title,
            if options
                .high_priority
                .unwrap_or(options.tint == Tint::Danger)
            {
                gtk::AccessibleAnnouncementPriority::High
            } else {
                gtk::AccessibleAnnouncementPriority::Low
            },
        );
        id
    }

    fn reorder(&self) {
        let receipts = self.receipts.borrow();
        let mut order: Vec<_> = receipts.iter().enumerate().collect();
        if self.placement == ToastPlacement::Top {
            order.reverse();
        }
        let mut previous = None;
        for (i, r) in order {
            r.widget.insert_after(&self.stack, previous);
            r.widget.set_can_focus(i < self.limit);
            r.widget.set_can_target(i < self.limit);
            previous = Some(&r.widget);
        }
    }
    fn close(self: &Rc<Self>, id: &str) {
        let index = self.receipts.borrow().iter().position(|r| r.id == id);
        let Some(index) = index else {
            return;
        };
        let r = self.receipts.borrow_mut().remove(index);
        let focused = self
            .window()
            .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w))
            .is_some_and(|w| w == r.widget || w.is_ancestor(&r.widget));
        r.widget.unparent();
        self.reorder();
        if focused {
            let next = {
                let receipts = self.receipts.borrow();
                receipts
                    .get(index)
                    .or_else(|| receipts.last())
                    .map(|r| r.widget.clone())
            };
            if let Some(next) = next {
                next.grab_focus();
            } else {
                self.restore_focus();
            }
        }
        if let Some(close) = r.on_close {
            close();
        }
        self.retime();
    }
    fn clear(self: &Rc<Self>, notify: bool) {
        let focused = notify
            && self
                .window()
                .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w))
                .is_some_and(|w| self.inside(&w));
        let receipts = std::mem::take(&mut *self.receipts.borrow_mut());
        for r in &receipts {
            r.widget.unparent();
        }
        self.hover.set(false);
        self.pressed.set(false);
        if focused {
            self.restore_focus();
        }
        for r in receipts {
            if notify && let Some(close) = r.on_close {
                close();
            }
        }
        self.retime();
    }
    fn restore_focus(&self) {
        if let Some(w) = self.previous_focus.upgrade() {
            w.grab_focus();
        } else if let Some(window) = self.window() {
            gtk::prelude::GtkWindowExt::set_focus(&window, None::<&gtk::Widget>);
        }
    }
    fn is_paused(&self) -> bool {
        self.hover.get()
            || self.pressed.get()
            || self.window().is_none_or(|w| {
                !w.is_active()
                    || gtk::prelude::GtkWindowExt::focus(&w).is_some_and(|f| self.inside(&f))
            })
    }
    fn retime(self: &Rc<Self>) {
        if self.clock_busy.replace(true) {
            return;
        }
        let now = Instant::now();
        let elapsed = now.duration_since(self.last.replace(now));
        let mut expired = vec![];
        if !self.paused.get() {
            for r in self.receipts.borrow_mut().iter_mut().filter(|r| r.timed) {
                r.remaining = r.remaining.saturating_sub(elapsed);
                if r.remaining.is_zero() {
                    expired.push(r.id.clone());
                }
            }
        }
        self.cancel_timer();
        for id in expired {
            self.close(&id);
        }
        self.paused.set(self.is_paused());
        let next = if self.paused.get() {
            None
        } else {
            self.receipts
                .borrow()
                .iter()
                .filter(|r| r.timed)
                .map(|r| r.remaining)
                .min()
        };
        if let Some(delay) = next {
            let weak = Rc::downgrade(self);
            let timer =
                glib::timeout_add_local_once(delay.max(Duration::from_millis(1)), move || {
                    if let Some(s) = weak.upgrade() {
                        s.timer.borrow_mut().take();
                        s.retime();
                    }
                });
            *self.timer.borrow_mut() = Some(timer);
        }
        self.clock_busy.set(false);
    }
}
mod imp {
    use super::*;
    use gtk::subclass::prelude::*;
    #[derive(Default)]
    pub struct Card;
    #[glib::object_subclass]
    impl ObjectSubclass for Card {
        const NAME: &'static str = "DazzToastCard";
        type Type = super::Card;
        type ParentType = gtk::Widget;
        fn class_init(class: &mut Self::Class) {
            class.set_layout_manager_type::<super::Layout>();
        }
    }
    impl ObjectImpl for Card {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Card {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let mut child = self.obj().first_child();
            while let Some(node) = child {
                child = node.next_sibling();
                self.obj().snapshot_child(&node, snapshot);
            }
        }
    }
    #[derive(Default)]
    pub struct Stack;
    #[glib::object_subclass]
    impl ObjectSubclass for Stack {
        const NAME: &'static str = "DazzToastStack";
        type Type = super::Stack;
        type ParentType = gtk::Widget;
        fn class_init(class: &mut Self::Class) {
            class.set_layout_manager_type::<super::StackLayout>();
        }
    }
    impl ObjectImpl for Stack {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Stack {
        fn contains(&self, x: f64, y: f64) -> bool {
            children(self.obj().upcast_ref()).iter().any(|w| {
                w.can_target()
                    && w.compute_bounds(self.obj().upcast_ref::<gtk::Widget>())
                        .is_some_and(|r| {
                            r.contains_point(&gtk::graphene::Point::new(x as f32, y as f32))
                        })
            })
        }
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            for child in children(self.obj().upcast_ref()) {
                self.obj().snapshot_child(&child, snapshot);
            }
        }
    }
    #[derive(Default)]
    pub struct StackLayout;
    #[glib::object_subclass]
    impl ObjectSubclass for StackLayout {
        const NAME: &'static str = "DazzToastStackLayout";
        type Type = super::StackLayout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for StackLayout {}
    impl LayoutManagerImpl for StackLayout {
        fn measure(&self, w: &gtk::Widget, o: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            let kids = children(w);
            let gap = Theme::of(w)
                .unwrap_or_default()
                .dimension("spacing-2")
                .unwrap() as i32;
            let n = if o == gtk::Orientation::Horizontal {
                kids.iter().map(|c| c.measure(o, -1).1).max().unwrap_or(0)
            } else {
                kids.iter().map(|c| c.measure(o, -1).1).sum::<i32>()
                    + gap * kids.len().saturating_sub(1) as i32
            };
            (
                if o == gtk::Orientation::Horizontal {
                    0
                } else {
                    n
                },
                n,
                -1,
                -1,
            )
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, _: i32, _: i32) {
            let theme = Theme::of(w).unwrap_or_default();
            let gap = theme.dimension("spacing-2").unwrap();
            let mut y = 0.;
            for child in children(w) {
                let ideal = card_width(&child).min(f64::from(width));
                let cw = ideal.round().max(0.) as i32;
                let ch = child.measure(gtk::Orientation::Vertical, cw).1;
                child.allocate(
                    cw,
                    ch,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            ((f64::from(width) - ideal) / 2.) as f32,
                            y as f32,
                        )),
                    ),
                );
                y += f64::from(ch) + gap;
            }
        }
    }
    #[derive(Default)]
    pub struct Layout;
    #[glib::object_subclass]
    impl ObjectSubclass for Layout {
        const NAME: &'static str = "DazzToastLayout";
        type Type = super::Layout;
        type ParentType = gtk::LayoutManager;
    }
    impl ObjectImpl for Layout {}
    impl LayoutManagerImpl for Layout {
        fn measure(&self, w: &gtk::Widget, o: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
            let kids = children(w);
            let theme = Theme::of(w).unwrap_or_default();
            let gap = theme.dimension("spacing-2-5").unwrap();
            let n = if o == gtk::Orientation::Horizontal {
                let sum = kids.iter().map(advance).sum::<f64>()
                    + gap * kids.len().saturating_sub(1) as f64;
                let inset = theme.dimension("spacing-3").unwrap()
                    + theme.dimension("spacing-1-5").unwrap()
                    + 2. * theme.dimension("stroke-hairline").unwrap();
                sum.min(theme.dimension("toast-max-width").unwrap() - inset)
                    .round() as i32
            } else {
                kids.iter().map(|c| c.measure(o, -1).1).max().unwrap_or(0)
            };
            (
                if o == gtk::Orientation::Horizontal {
                    0
                } else {
                    n
                },
                n,
                -1,
                -1,
            )
        }
        fn allocate(&self, w: &gtk::Widget, width: i32, height: i32, _: i32) {
            let kids = children(w);
            let gap = Theme::of(w)
                .unwrap_or_default()
                .dimension("spacing-2-5")
                .unwrap();
            let total =
                kids.iter().map(advance).sum::<f64>() + gap * kids.len().saturating_sub(1) as f64;
            let overflow = (total - f64::from(width)).max(0.);
            let mut x = 0.;
            for child in kids {
                let natural = advance(&child);
                let available = if child.has_css_class("dz-toast-message") && overflow > 0.5 {
                    (natural - overflow).max(0.)
                } else {
                    natural
                };
                let cw = available.ceil() as i32;
                let ch = child.measure(gtk::Orientation::Vertical, cw).1;
                let px = if w.direction() == gtk::TextDirection::Rtl {
                    f64::from(width) - x - available
                } else {
                    x
                };
                child.allocate(
                    cw,
                    ch,
                    -1,
                    Some(
                        gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(
                            px as f32,
                            (height - ch) as f32 / 2.,
                        )),
                    ),
                );
                x += available + gap;
            }
        }
    }
}
glib::wrapper! {pub struct Card(ObjectSubclass<imp::Card>) @extends gtk::Widget,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget;}
glib::wrapper! {pub struct Layout(ObjectSubclass<imp::Layout>) @extends gtk::LayoutManager;}
glib::wrapper! {pub struct Stack(ObjectSubclass<imp::Stack>) @extends gtk::Widget,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget;}
glib::wrapper! {pub struct StackLayout(ObjectSubclass<imp::StackLayout>) @extends gtk::LayoutManager;}
fn card_width(w: &gtk::Widget) -> f64 {
    let theme = Theme::of(w).unwrap_or_default();
    let kids = children(w);
    let inset = theme.dimension("spacing-3").unwrap()
        + theme.dimension("spacing-1-5").unwrap()
        + 2. * theme.dimension("stroke-hairline").unwrap();
    (kids.iter().map(advance).sum::<f64>()
        + theme.dimension("spacing-2-5").unwrap() * kids.len().saturating_sub(1) as f64
        + inset)
        .min(theme.dimension("toast-max-width").unwrap())
}
fn children(w: &gtk::Widget) -> Vec<gtk::Widget> {
    let mut result = vec![];
    let mut child = w.first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        result.push(node);
    }
    result
}
fn advance(w: &gtk::Widget) -> f64 {
    if let Some(label) = w.downcast_ref::<gtk::Label>() {
        let layout = label.layout().copy();
        layout.set_width(-1);
        f64::from(layout.size().0) / f64::from(gtk::pango::SCALE)
    } else {
        f64::from(w.measure(gtk::Orientation::Horizontal, -1).1)
            + w.downcast_ref::<gtk::Button>()
                .map_or(0., super::button_layout::remainder)
    }
}
fn card(
    options: &ToastOptions,
    existing: Option<&gtk::Widget>,
    wire: impl Fn(&gtk::Button, bool),
) -> gtk::Widget {
    let role = if options
        .high_priority
        .unwrap_or(options.tint == Tint::Danger)
    {
        gtk::AccessibleRole::AlertDialog
    } else {
        gtk::AccessibleRole::Dialog
    };
    let old = existing.map(children).unwrap_or_default();
    // GTK fixes the accessible role when a widget is realized. Keep controls
    // when the role changes, and only replace the immutable outer container.
    let card: Card = existing
        .filter(|w| w.accessible_role() == role)
        .map(|w| w.clone().downcast().unwrap())
        .unwrap_or_else(glib::Object::new);
    let part = |class: &str| old.iter().find(|w| w.has_css_class(class)).cloned();
    let mut ordered = vec![];
    card.add_css_class("dz-toast");
    card.set_dazz_tint(options.tint);
    card.set_focusable(true);
    if !card.is_realized() {
        card.set_accessible_role(role);
    }
    card.update_property(&[gtk::accessible::Property::Modal(false)]);
    if let Some(icon) = &options.icon {
        icon.add_css_class("dz-toast-icon");
        ordered.push(icon.clone());
    }
    let message = part("dz-toast-message")
        .and_then(|w| w.downcast::<gtk::Label>().ok())
        .unwrap_or_default();
    message.set_text(&options.title);
    message.add_css_class("dz-toast-message");
    message.set_ellipsize(gtk::pango::EllipsizeMode::End);
    message.set_xalign(0.);
    ordered.push(message.clone().upcast());
    card.update_relation(&[gtk::accessible::Relation::LabelledBy(&[
        message.upcast_ref()
    ])]);
    if let Some(action) = &options.action {
        let control = part("dz-toast-action")
            .and_then(|w| w.downcast::<gtk::Button>().ok())
            .unwrap_or_else(|| {
                let control = button("", Variant::Plain, options.tint, Size::Small);
                wire(&control, true);
                control
            });
        control.set_label(&action.label);
        control.set_dazz_tint(options.tint);
        control.add_css_class("dz-toast-action");
        ordered.push(control.upcast());
    }
    if options.dismissable {
        let close = part("dz-toast-dismiss")
            .and_then(|w| w.downcast::<gtk::Button>().ok())
            .unwrap_or_else(|| {
                let control = gtk::Button::new();
                wire(&control, false);
                control
            });
        close.add_css_class("dz-toast-dismiss");
        close.set_child(Some(&icon("Dismiss12Regular")));
        close.set_dazz_label(&options.dismiss_label);
        ordered.push(close.upcast());
    }
    for child in old {
        if !ordered.contains(&child) {
            child.unparent();
        }
    }
    let mut previous = None;
    for child in &ordered {
        if child.parent().is_some_and(|p| p != card) {
            child.unparent();
        }
        child.insert_after(&card, previous);
        previous = Some(child);
    }
    card.queue_resize();
    card.upcast()
}
