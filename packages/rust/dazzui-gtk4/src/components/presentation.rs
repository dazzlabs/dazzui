//! Native popup positioning, modal ownership and focus, with ds surfaces.
use crate::{Theme, box_, prelude::*};
use gtk::glib;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

pub fn attach_popover(
    trigger: &impl IsA<gtk::Widget>,
    popup: &impl IsA<gtk::Popover>,
    side: gtk::PositionType,
    align: gtk::Align,
) {
    let popup = popup.as_ref();
    popup.set_has_arrow(false);
    popup.set_position(side);
    popup.set_halign(align);
    popup.set_parent(trigger);
    let weak = popup.downgrade();
    trigger.connect_unmap(move |_| {
        if let Some(popup) = weak.upgrade() {
            popup.popdown();
        }
    });
    let weak = popup.downgrade();
    trigger.connect_destroy(move |_| {
        if let Some(popup) = weak.upgrade() {
            popup.unparent();
        }
    });
    popup.connect_map(move |popup| {
        let gap = Theme::of(popup)
            .unwrap_or_default()
            .dimension("spacing-1")
            .unwrap() as i32;
        let (x, y) = match side {
            gtk::PositionType::Top => (0, -gap),
            gtk::PositionType::Left => (-gap, 0),
            gtk::PositionType::Right => (gap, 0),
            _ => (0, gap),
        };
        popup.set_offset(x, y);
    });
}

/// A rich popover. The trigger remains its original GTK widget.
pub fn rich_popover(
    trigger: &impl IsA<gtk::Widget>,
    content: &impl IsA<gtk::Widget>,
    side: gtk::PositionType,
    align: gtk::Align,
) -> gtk::Popover {
    let popup = crate::popover(content);
    popup.add_css_class("dz-rich-popover");
    attach_popover(trigger, &popup, side, align);
    popup
}

/// Close the nearest popup or modal, preserving the native focus handoff.
pub fn dismiss_from(widget: &impl IsA<gtk::Widget>) {
    let mut node = widget.as_ref().parent();
    while let Some(w) = node {
        if let Some(popup) = w.downcast_ref::<gtk::Popover>() {
            popup.popdown();
            return;
        }
        if let Some(window) = w.downcast_ref::<gtk::Window>()
            && window.has_css_class("dz-modal-window")
        {
            window.close();
            return;
        }
        node = w.parent();
    }
}

/// A modal sheet with native modality, transient ownership and Escape handling.
/// The content is retained when closed, so reopening preserves native edits.
pub fn modal(
    parent: &impl IsA<gtk::Window>,
    content: &impl IsA<gtk::Widget>,
    width: Option<i32>,
    danger: bool,
) -> gtk::Window {
    let parent = parent.as_ref();
    let window = gtk::Window::builder()
        .transient_for(parent)
        .modal(true)
        .decorated(false)
        .resizable(false)
        .hide_on_close(true)
        .destroy_with_parent(true)
        .build();
    let theme = Theme::of(parent).unwrap_or_default();
    theme.apply(&window);
    window.add_css_class("dz-modal-window");
    window.set_accessible_role(gtk::AccessibleRole::Dialog);
    content.add_css_class("dz-modal");
    content.set_overflow(gtk::Overflow::Hidden);
    if danger {
        content.add_css_class("dz-modal-danger");
    }
    let gap = theme.dimension("spacing-6").unwrap() as i32;
    let requested = width.unwrap_or_else(|| theme.dimension("dialog-width").unwrap() as i32);
    window.set_default_size(
        if parent.width() > 0 {
            requested.min((parent.width() - 2 * gap).max(1))
        } else {
            requested
        },
        -1,
    );
    window.set_child(Some(content));
    let keys = gtk::ShortcutController::new();
    keys.set_scope(gtk::ShortcutScope::Managed);
    let weak = window.downgrade();
    keys.add_shortcut(gtk::Shortcut::new(
        gtk::ShortcutTrigger::parse_string("Escape"),
        Some(gtk::CallbackAction::new(move |_, _| {
            if let Some(window) = weak.upgrade() {
                window.close();
            }
            glib::Propagation::Stop
        })),
    ));
    window.add_controller(keys);

    // GTK owns modality. The overlay adds only the reference's dimmed paper;
    // removing it puts the same widget tree back, without recreating controls.
    type Dimmed = Option<(glib::WeakRef<gtk::Window>, gtk::Overlay, gtk::Widget)>;
    let dimmed: Rc<RefCell<Dimmed>> = Rc::new(RefCell::new(None));
    let saved = dimmed.clone();
    window.connect_map(move |window| {
        let Some(parent) = window.transient_for() else {
            return;
        };
        let Some(original) = parent.child() else {
            return;
        };
        if saved.borrow().is_some() {
            return;
        }
        parent.set_child(None::<&gtk::Widget>);
        let overlay = gtk::Overlay::new();
        overlay.set_child(Some(&original));
        let scrim = box_(gtk::Orientation::Vertical, "0");
        scrim.add_css_class("dz-modal-scrim");
        scrim.set_can_target(false);
        scrim.set_hexpand(true);
        scrim.set_vexpand(true);
        overlay.add_overlay(&scrim);
        parent.set_child(Some(&overlay));
        *saved.borrow_mut() = Some((parent.downgrade(), overlay, original));
    });
    window.connect_unmap(move |_| {
        if let Some((parent, overlay, original)) = dimmed.borrow_mut().take()
            && let Some(parent) = parent.upgrade()
            && parent.child().as_ref() == Some(overlay.upcast_ref())
        {
            overlay.set_child(None::<&gtk::Widget>);
            parent.set_child(Some(&original));
        }
    });
    window
}

struct Hover {
    enabled: Rc<dyn Fn() -> bool>,
    popup: glib::WeakRef<gtk::Popover>,
    trigger: glib::WeakRef<gtk::Widget>,
    timer: RefCell<Option<glib::SourceId>>,
    over_trigger: Cell<bool>,
    over_popup: Cell<bool>,
    focused: Cell<bool>,
    popup_focused: Cell<bool>,
    dismissed: Cell<bool>,
    open: Duration,
    close: Duration,
    interactive: bool,
    root_handlers: RefCell<Option<(glib::WeakRef<gtk::Window>, Vec<glib::SignalHandlerId>)>>,
}
impl Hover {
    fn cancel(&self) {
        if let Some(timer) = self.timer.borrow_mut().take() {
            timer.remove();
        }
    }
    fn unwatch(&self) {
        if let Some((root, handlers)) = self.root_handlers.borrow_mut().take()
            && let Some(root) = root.upgrade()
        {
            for handler in handlers {
                root.disconnect(handler);
            }
        }
    }
    fn sync_focus(self: &Rc<Self>) {
        let Some(trigger) = self.trigger.upgrade() else {
            return;
        };
        let Some(root) = trigger.root().and_downcast::<gtk::Window>() else {
            return;
        };
        let focus = gtk::prelude::GtkWindowExt::focus(&root);
        let focused = root.gets_focus_visible() && focus.as_ref() == Some(&trigger);
        let in_popup = self.interactive
            && self
                .popup
                .upgrade()
                .is_some_and(|p| focus.as_ref().is_some_and(|f| f.is_ancestor(&p)));
        let was_focused = self.focused.get() || self.popup_focused.get();
        let entered = focused && !self.focused.get();
        self.focused.set(focused);
        self.popup_focused.set(in_popup);
        if entered {
            self.dismissed.set(false);
            self.show_now();
        } else if was_focused && !focused && !in_popup {
            self.blur();
        }
    }
    fn watch(self: &Rc<Self>) {
        self.unwatch();
        let Some(root) = self
            .trigger
            .upgrade()
            .and_then(|w| w.root())
            .and_downcast::<gtk::Window>()
        else {
            return;
        };
        let weak = Rc::downgrade(self);
        let focus = root.connect_focus_widget_notify(move |_| {
            if let Some(s) = weak.upgrade() {
                s.sync_focus();
            }
        });
        let weak = Rc::downgrade(self);
        let visible = root.connect_focus_visible_notify(move |_| {
            if let Some(s) = weak.upgrade() {
                s.sync_focus();
            }
        });
        *self.root_handlers.borrow_mut() = Some((root.downgrade(), vec![focus, visible]));
        self.sync_focus();
    }
    fn wanted(&self) -> bool {
        !self.dismissed.get()
            && (self.over_trigger.get()
                || self.over_popup.get()
                || self.focused.get()
                || self.popup_focused.get())
    }
    fn show_now(&self) {
        self.cancel();
        if !(self.enabled)() {
            return;
        }
        if let (Some(popup), Some(trigger)) = (self.popup.upgrade(), self.trigger.upgrade()) {
            if !trigger.is_mapped() || popup.is_visible() {
                return;
            }
            let focus = trigger
                .root()
                .and_downcast::<gtk::Window>()
                .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w).map(|f| (w, f)));
            popup.popup();
            // A hover preview does not move focus into its content. The
            // content remains focusable when a reader intentionally enters it.
            if self.interactive
                && let Some((window, focus)) = focus
            {
                gtk::prelude::GtkWindowExt::set_focus(&window, Some(&focus));
            }
        }
    }
    fn schedule(self: &Rc<Self>) {
        self.cancel();
        let show = self.wanted();
        if self.popup.upgrade().is_some_and(|p| p.is_visible() == show) {
            return;
        }
        let weak = Rc::downgrade(self);
        let timer =
            glib::timeout_add_local_once(if show { self.open } else { self.close }, move || {
                if let Some(state) = weak.upgrade() {
                    state.timer.borrow_mut().take();
                    if state.wanted() {
                        state.show_now();
                    } else if let Some(popup) = state.popup.upgrade() {
                        popup.popdown();
                    }
                }
            });
        *self.timer.borrow_mut() = Some(timer);
    }
    fn blur(self: &Rc<Self>) {
        if self.wanted() {
            self.schedule();
        } else {
            self.cancel();
            if let Some(popup) = self.popup.upgrade() {
                popup.popdown();
            }
        }
    }
    fn dismiss(&self) {
        self.cancel();
        self.focused.set(false);
        self.popup_focused.set(false);
        self.dismissed.set(true);
        if let Some(popup) = self.popup.upgrade() {
            popup.popdown();
        }
    }
}
/// Hover and keyboard-focus preview. A rich card stays open while the pointer
/// travels to it; a passive hint is never a pointer or keyboard destination.
pub fn hover_popup(
    trigger: &impl IsA<gtk::Widget>,
    popup: &gtk::Popover,
    open_delay: Duration,
    close_delay: Duration,
    interactive: bool,
) {
    hover_popup_when(trigger, popup, open_delay, close_delay, interactive, || {
        true
    });
}
/// The same native hover/focus lifecycle, gated by a live disclosure state.
pub fn hover_popup_when(
    trigger: &impl IsA<gtk::Widget>,
    popup: &gtk::Popover,
    open_delay: Duration,
    close_delay: Duration,
    interactive: bool,
    enabled: impl Fn() -> bool + 'static,
) {
    trigger.set_has_tooltip(false);
    popup.set_autohide(false);
    popup.set_can_focus(interactive);
    popup.set_focusable(false);
    popup.set_can_target(interactive);
    let state = Rc::new(Hover {
        enabled: Rc::new(enabled),
        popup: popup.downgrade(),
        trigger: trigger.as_ref().downgrade(),
        timer: RefCell::new(None),
        over_trigger: Cell::new(false),
        over_popup: Cell::new(false),
        focused: Cell::new(false),
        popup_focused: Cell::new(false),
        dismissed: Cell::new(false),
        open: open_delay,
        close: close_delay,
        interactive,
        root_handlers: RefCell::new(None),
    });
    let s = state.clone();
    trigger.connect_map(move |_| s.watch());
    if trigger.is_mapped() {
        state.watch();
    }
    let motion = gtk::EventControllerMotion::new();
    motion.set_propagation_limit(gtk::PropagationLimit::SameNative);
    let s = state.clone();
    motion.connect_enter(move |_, _, _| {
        s.dismissed.set(false);
        s.over_trigger.set(true);
        s.schedule();
    });
    let s = state.clone();
    motion.connect_leave(move |_| {
        s.over_trigger.set(false);
        s.schedule();
    });
    trigger.add_controller(motion);
    if interactive {
        let motion = gtk::EventControllerMotion::new();
        let s = state.clone();
        motion.connect_enter(move |_, _, _| {
            s.over_popup.set(true);
            s.cancel();
        });
        let s = state.clone();
        motion.connect_leave(move |_| {
            s.over_popup.set(false);
            s.schedule();
        });
        popup.add_controller(motion);
        let focus = gtk::EventControllerFocus::new();
        let s = state.clone();
        focus.connect_enter(move |_| {
            s.popup_focused.set(true);
            s.cancel();
        });
        let s = state.clone();
        focus.connect_leave(move |_| {
            s.popup_focused.set(false);
            s.blur();
        });
        popup.add_controller(focus);
    }
    let focus = gtk::EventControllerFocus::new();
    let s = state.clone();
    focus.connect_enter(move |controller| {
        if controller.widget().is_some_and(|w| w.has_visible_focus()) {
            s.dismissed.set(false);
            s.focused.set(true);
            s.show_now();
        }
    });
    let s = state.clone();
    focus.connect_leave(move |_| {
        s.focused.set(false);
        s.blur();
    });
    trigger.add_controller(focus);
    let s = state.clone();
    trigger.connect_state_flags_changed(move |trigger, previous| {
        if trigger.has_visible_focus()
            && !previous.contains(gtk::StateFlags::FOCUS_VISIBLE)
            && !s.focused.get()
        {
            s.dismissed.set(false);
            s.focused.set(true);
            s.show_now();
        }
    });
    let press = gtk::GestureClick::new();
    press.set_propagation_phase(gtk::PropagationPhase::Capture);
    press.set_propagation_limit(gtk::PropagationLimit::SameNative);
    let s = state.clone();
    press.connect_pressed(move |_, _, _, _| s.dismiss());
    trigger.add_controller(press);
    let keys = gtk::EventControllerKey::new();
    keys.set_propagation_phase(gtk::PropagationPhase::Capture);
    keys.set_propagation_limit(gtk::PropagationLimit::SameNative);
    let s = state.clone();
    keys.connect_key_pressed(move |_, key, _, _| {
        if [
            gtk::gdk::Key::Escape,
            gtk::gdk::Key::Return,
            gtk::gdk::Key::space,
        ]
        .contains(&key)
        {
            s.dismiss();
        }
        glib::Propagation::Proceed
    });
    trigger.add_controller(keys);
    if let Some(button) = trigger.as_ref().downcast_ref::<gtk::Button>() {
        let s = state.clone();
        button.connect_clicked(move |_| s.dismiss());
    }
    let s = state.clone();
    trigger.connect_unmap(move |_| {
        s.over_trigger.set(false);
        s.over_popup.set(false);
        s.unwatch();
        s.dismiss();
    });
    trigger.connect_destroy(move |_| {
        state.cancel();
        state.unwatch();
    });
}
