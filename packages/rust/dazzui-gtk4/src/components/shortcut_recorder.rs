//! A shortcut field whose native buttons own activation and focus. Recording
//! consumes local key events; it never installs an application accelerator.
use super::shortcut_key::{format_shortcut, key_name, modifier_key, modifiers, position};
use crate::{Theme, Tint, icon, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::{
    cell::{Cell, OnceCell, RefCell},
    collections::HashSet,
};

pub struct ShortcutRecorderOptions {
    pub value: String,
    pub placeholder: String,
    pub recording_label: String,
    pub clear_label: String,
    pub label: Option<String>,
    pub invalid: bool,
    pub disabled: bool,
    pub tint: Tint,
}
impl Default for ShortcutRecorderOptions {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: "Record shortcut".into(),
            recording_label: "Press keys…".into(),
            clear_label: "Clear shortcut".into(),
            label: None,
            invalid: false,
            disabled: false,
            tint: Tint::Primary,
        }
    }
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct ShortcutRecorder {
        pub button: OnceCell<gtk::Button>,
        pub clear: OnceCell<gtk::Button>,
        pub value: RefCell<String>,
        pub placeholder: RefCell<String>,
        pub recording_label: RefCell<String>,
        pub recording: Cell<bool>,
        pub held: RefCell<String>,
        pub modifier_down: RefCell<HashSet<gtk::gdk::Key>>,
        pub pending: RefCell<Option<glib::SourceId>>,
        pub keyboard: RefCell<Option<(gtk::gdk::Device, glib::SignalHandlerId)>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for ShortcutRecorder {
        const NAME: &'static str = "DazzShortcutRecorder";
        type Type = super::ShortcutRecorder;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for ShortcutRecorder {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: std::sync::OnceLock<Vec<glib::subclass::Signal>> =
                std::sync::OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("value-change")
                        .param_types([String::static_type()])
                        .build(),
                    glib::subclass::Signal::builder("clear").build(),
                ]
            })
        }
        fn dispose(&self) {
            self.obj().cancel_rejection();
            self.obj().disconnect_keyboard();
        }
    }
    impl WidgetImpl for ShortcutRecorder {
        fn direction_changed(&self, previous: gtk::TextDirection) {
            self.parent_direction_changed(previous);
            let mut child = self.obj().first_child();
            while let Some(widget) = child {
                child = widget.next_sibling();
                widget.set_direction(self.obj().direction());
            }
            if let Some(button) = self.button.get() {
                button.set_direction(self.obj().direction());
            }
            if let Some(clear) = self.clear.get() {
                clear.set_direction(self.obj().direction());
            }
        }
    }
    impl BoxImpl for ShortcutRecorder {}
}
glib::wrapper! {
    pub struct ShortcutRecorder(ObjectSubclass<imp::ShortcutRecorder>) @extends gtk::Widget,gtk::Box,
        @implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;
}
impl ShortcutRecorder {
    pub fn new(options: ShortcutRecorderOptions) -> Self {
        let root: Self = glib::Object::new();
        root.add_css_class("dz-shortcut-recorder");
        root.set_accessible_role(gtk::AccessibleRole::Generic);
        root.set_dazz_tint(options.tint);
        let button = gtk::Button::new();
        button.add_css_class("dz-shortcut-recorder-button");
        let clear = gtk::Button::new();
        clear.add_css_class("dz-shortcut-recorder-clear");
        clear.set_child(Some(&icon("DismissCircle12Filled")));
        clear.set_halign(gtk::Align::End);
        clear.set_valign(gtk::Align::Center);
        let overlay = gtk::Overlay::new();
        overlay.set_child(Some(&button));
        overlay.add_overlay(&clear);
        root.append(&overlay);
        root.imp().button.set(button.clone()).unwrap();
        root.imp().clear.set(clear.clone()).unwrap();
        root.set_prompts(
            &options.placeholder,
            &options.recording_label,
            &options.clear_label,
        );
        root.set_value(&options.value);
        root.set_invalid(options.invalid);
        root.set_sensitive(!options.disabled);
        if let Some(label) = options.label {
            button.set_dazz_label(&label);
        }
        let weak = root.downgrade();
        button.connect_clicked(move |_| {
            if let Some(root) = weak.upgrade() {
                root.start_recording();
            }
        });
        let weak = root.downgrade();
        clear.connect_clicked(move |_| {
            if let Some(root) = weak.upgrade() {
                root.request_clear();
            }
        });
        let focus = gtk::EventControllerFocus::new();
        let weak = root.downgrade();
        focus.connect_leave(move |_| {
            if let Some(root) = weak.upgrade() {
                root.stop_recording();
            }
        });
        button.add_controller(focus);
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = root.downgrade();
        keys.connect_key_pressed(move |controller, key, keycode, state| {
            let Some(root) = weak
                .upgrade()
                .filter(|root| root.is_recording() && root.is_sensitive())
            else {
                return glib::Propagation::Proceed;
            };
            if key == gtk::gdk::Key::Escape {
                root.stop_recording();
            } else if let Some(mask) = modifier_key(key) {
                root.imp().modifier_down.borrow_mut().insert(key);
                root.imp().held.replace(modifiers(state | mask));
                root.refresh();
            } else if !state.intersects(
                gtk::gdk::ModifierType::CONTROL_MASK
                    | gtk::gdk::ModifierType::ALT_MASK
                    | gtk::gdk::ModifierType::SHIFT_MASK
                    | gtk::gdk::ModifierType::META_MASK
                    | gtk::gdk::ModifierType::SUPER_MASK,
            ) && [
                gtk::gdk::Key::BackSpace,
                gtk::gdk::Key::Delete,
                gtk::gdk::Key::KP_Delete,
            ]
            .contains(&key)
            {
                root.request_clear();
            } else {
                let code = if keycode != 0 || controller.current_event().is_some() {
                    position(root.display().type_().name(), keycode)
                        .unwrap_or_else(|| "Unidentified".into())
                } else {
                    String::new()
                };
                if let Some(next) = format_shortcut(&code, &key_name(key), state) {
                    root.stop_recording();
                    root.emit_by_name::<()>("value-change", &[&next]);
                } else {
                    root.reject();
                }
            }
            glib::Propagation::Stop
        });
        let weak = root.downgrade();
        keys.connect_key_released(move |controller, key, _, state| {
            if let Some(root) = weak.upgrade().filter(|root| root.is_recording())
                && let Some(mask) = modifier_key(key)
            {
                root.imp().modifier_down.borrow_mut().remove(&key);
                let others = root
                    .imp()
                    .modifier_down
                    .borrow()
                    .iter()
                    .filter_map(|key| modifier_key(*key))
                    .fold(gtk::gdk::ModifierType::empty(), |all, mask| all | mask);
                let fallback = (state & !mask) | others;
                // GDK's event state precedes release. The keyboard's live
                // state also covers another modifier held before recording.
                let held = if controller.current_event().is_some() {
                    root.display()
                        .default_seat()
                        .and_then(|seat| seat.keyboard())
                        .map(|device| device.modifier_state())
                        .unwrap_or(fallback)
                } else {
                    fallback
                };
                root.imp().held.replace(modifiers(held));
                root.refresh();
            }
        });
        button.add_controller(keys);
        let weak = root.downgrade();
        root.connect_sensitive_notify(move |_| {
            if let Some(root) = weak.upgrade() {
                if !root.is_sensitive() {
                    root.stop_recording();
                }
                root.refresh();
            }
        });
        root.connect_unmap(|root| {
            root.stop_recording();
            root.cancel_rejection();
        });
        root.refresh();
        root
    }
    pub fn button(&self) -> gtk::Button {
        self.imp().button.get().unwrap().clone()
    }
    pub fn clear_button(&self) -> gtk::Button {
        self.imp().clear.get().unwrap().clone()
    }
    pub fn value(&self) -> String {
        self.imp().value.borrow().clone()
    }
    /// Host-controlled value: recording emits a request without changing this.
    pub fn set_value(&self, value: &str) {
        self.imp().value.replace(value.into());
        self.refresh();
    }
    pub fn set_prompts(&self, placeholder: &str, recording: &str, clear: &str) {
        self.imp().placeholder.replace(placeholder.into());
        self.imp().recording_label.replace(recording.into());
        self.clear_button().set_dazz_label(clear);
        self.refresh();
    }
    pub fn set_invalid(&self, invalid: bool) {
        self.button().set_dazz_invalid(invalid);
    }
    pub fn is_recording(&self) -> bool {
        self.imp().recording.get()
    }
    pub fn start_recording(&self) {
        if !self.button().is_sensitive() || self.imp().recording.replace(true) {
            return;
        }
        self.imp().held.borrow_mut().clear();
        self.imp().modifier_down.borrow_mut().clear();
        if let Some(device) = self
            .display()
            .default_seat()
            .and_then(|seat| seat.keyboard())
        {
            let weak = self.downgrade();
            let handler = device.connect_modifier_state_notify(move |device| {
                if let Some(root) = weak.upgrade().filter(|root| root.is_recording()) {
                    root.imp().held.replace(modifiers(device.modifier_state()));
                    root.refresh();
                }
            });
            self.imp().keyboard.replace(Some((device, handler)));
        }
        self.button().grab_focus();
        self.refresh();
    }
    pub fn stop_recording(&self) {
        self.disconnect_keyboard();
        self.imp().recording.set(false);
        self.imp().held.borrow_mut().clear();
        self.imp().modifier_down.borrow_mut().clear();
        self.refresh();
    }
    pub fn connect_value_changed(
        &self,
        callback: impl Fn(&Self, &str) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("value-change", false, move |v| {
            callback(&v[0].get::<Self>().unwrap(), &v[1].get::<String>().unwrap());
            None
        })
    }
    /// A connected clear callback replaces the default value-change("") request.
    pub fn connect_clear(&self, callback: impl Fn(&Self) + 'static) -> glib::SignalHandlerId {
        self.connect_local("clear", false, move |v| {
            callback(&v[0].get::<Self>().unwrap());
            None
        })
    }
    fn disconnect_keyboard(&self) {
        if let Some((device, handler)) = self.imp().keyboard.borrow_mut().take() {
            device.disconnect(handler);
        }
    }
    fn request_clear(&self) {
        if !self.is_sensitive() {
            return;
        }
        self.stop_recording();
        let signal = glib::subclass::SignalId::lookup("clear", Self::static_type()).unwrap();
        if glib::signal::signal_has_handler_pending(self, signal, None, true) {
            self.emit_by_name::<()>("clear", &[]);
        } else {
            self.emit_by_name::<()>("value-change", &[&""]);
        }
    }
    fn refresh(&self) {
        let Some(button) = self.imp().button.get() else {
            return;
        };
        let bound = !self.imp().value.borrow().is_empty();
        let recording = self.is_recording();
        let held = self.imp().held.borrow().clone();
        let text = if recording {
            if held.is_empty() {
                self.imp().recording_label.borrow().clone()
            } else {
                held.clone()
            }
        } else if bound {
            self.value()
        } else {
            self.imp().placeholder.borrow().clone()
        };
        button.set_label(&text);
        let glyphs = if recording { !held.is_empty() } else { bound };
        for (class, on) in [
            ("dz-glyphs", glyphs),
            ("dz-recording", recording),
            ("dz-clearable", bound && !recording && self.is_sensitive()),
        ] {
            if on {
                button.add_css_class(class);
            } else {
                button.remove_css_class(class);
            }
        }
        button.update_state(&[gtk::accessible::State::Pressed(if recording {
            gtk::AccessibleTristate::True
        } else {
            gtk::AccessibleTristate::False
        })]);
        self.clear_button()
            .set_visible(bound && !recording && self.is_sensitive());
    }
    fn reject(&self) {
        if self.imp().pending.borrow().is_some() {
            return;
        }
        self.button().add_css_class("dz-rejected");
        let weak = self.downgrade();
        let duration = Theme::of(self)
            .unwrap_or_default()
            .duration("shortcut-recorder-reject-duration")
            .unwrap();
        self.imp()
            .pending
            .replace(Some(glib::timeout_add_local_once(duration, move || {
                if let Some(root) = weak.upgrade() {
                    root.imp().pending.borrow_mut().take();
                    root.button().remove_css_class("dz-rejected");
                }
            })));
    }
    fn cancel_rejection(&self) {
        if let Some(timer) = self.imp().pending.borrow_mut().take() {
            timer.remove();
        }
        if let Some(button) = self.imp().button.get() {
            button.remove_css_class("dz-rejected");
        }
    }
}
pub fn shortcut_recorder(options: ShortcutRecorderOptions) -> ShortcutRecorder {
    ShortcutRecorder::new(options)
}
