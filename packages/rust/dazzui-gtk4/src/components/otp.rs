//! Native editable slots, coordinated as one contiguous code and one Tab stop.
use crate::{Size, Tint, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::{
    cell::{Cell, OnceCell, RefCell},
    rc::Rc,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OtpValidation {
    #[default]
    Numeric,
    Alpha,
    Alphanumeric,
    None,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpReason {
    InputChange,
    InputClear,
    InputPaste,
    Keyboard,
}
impl OtpReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InputChange => "input-change",
            Self::InputClear => "input-clear",
            Self::InputPaste => "input-paste",
            Self::Keyboard => "keyboard",
        }
    }
}
/// A request before the value changes. Cancellation also suppresses focus and completion.
#[derive(Clone, Debug, glib::Boxed)]
#[boxed_type(name = "DazzOtpChange")]
pub struct OtpChange {
    pub value: String,
    pub reason: OtpReason,
    canceled: Rc<Cell<bool>>,
}
impl OtpChange {
    pub fn cancel(&self) {
        self.canceled.set(true);
    }
    pub fn is_canceled(&self) -> bool {
        self.canceled.get()
    }
}
pub struct OtpOptions {
    pub length: usize,
    pub value: Option<String>,
    pub default_value: String,
    pub group_after: Option<usize>,
    pub size: Size,
    pub tint: Tint,
    pub validation: OtpValidation,
    pub mask: bool,
    pub read_only: bool,
    pub disabled: bool,
    pub required: bool,
}
impl Default for OtpOptions {
    fn default() -> Self {
        Self {
            length: 0,
            value: None,
            default_value: String::new(),
            group_after: None,
            size: Size::Medium,
            tint: Tint::Primary,
            validation: OtpValidation::Numeric,
            mask: false,
            read_only: false,
            disabled: false,
            required: false,
        }
    }
}
fn whitespace(c: char) -> bool {
    matches!(c,'\t'..='\r'|' '|'\u{a0}'|'\u{1680}'|'\u{2000}'..='\u{200a}'|'\u{2028}'|'\u{2029}'|'\u{202f}'|'\u{205f}'|'\u{3000}'|'\u{feff}')
}
fn normalize(raw: &str, length: usize, mode: OtpValidation) -> (String, bool) {
    let stripped: Vec<_> = raw.chars().filter(|c| !whitespace(*c)).collect();
    let filtered: Vec<_> = stripped
        .iter()
        .copied()
        .filter(|c| match mode {
            OtpValidation::Numeric => c.is_ascii_digit(),
            OtpValidation::Alpha => c.is_ascii_alphabetic(),
            OtpValidation::Alphanumeric => c.is_ascii_alphanumeric(),
            OtpValidation::None => true,
        })
        .collect();
    (
        filtered.iter().take(length).collect(),
        filtered.len() != stripped.len() || filtered.len() > length,
    )
}
fn removed(value: &str, index: usize) -> String {
    value
        .chars()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, c)| c)
        .collect()
}
fn replaced(value: &str, index: usize, insert: &str, length: usize) -> String {
    value
        .chars()
        .take(index)
        .chain(insert.chars())
        .chain(value.chars().skip(index + insert.chars().count()))
        .take(length)
        .collect()
}
pub(super) struct Commit {
    value: String,
    focus: usize,
    reason: OtpReason,
    complete: bool,
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct OtpField {
        pub entries: OnceCell<Vec<gtk::Entry>>,
        pub value: RefCell<String>,
        pub controlled: Cell<bool>,
        pub applying: Cell<bool>,
        pub active: Cell<usize>,
        pub validation: Cell<OtpValidation>,
        pub read_only: Cell<bool>,
        pub pending: RefCell<Option<glib::SourceId>>,
        pub(super) waiting: RefCell<Option<Commit>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for OtpField {
        const NAME: &'static str = "DazzOtpField";
        type Type = super::OtpField;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for OtpField {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: std::sync::OnceLock<Vec<glib::subclass::Signal>> =
                std::sync::OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("value-change")
                        .param_types([OtpChange::static_type()])
                        .build(),
                    glib::subclass::Signal::builder("value-complete")
                        .param_types([String::static_type(), String::static_type()])
                        .build(),
                    glib::subclass::Signal::builder("value-invalid")
                        .param_types([String::static_type(), String::static_type()])
                        .build(),
                ]
            })
        }
        fn dispose(&self) {
            self.obj().cancel_pending();
        }
    }
    impl WidgetImpl for OtpField {
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            let root = self.obj();
            let focus = root
                .root()
                .and_downcast::<gtk::Window>()
                .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
            if focus.is_some_and(|focus| focus.is_ancestor(root.upcast_ref::<gtk::Widget>())) {
                return false;
            }
            if [
                gtk::DirectionType::TabForward,
                gtk::DirectionType::TabBackward,
            ]
            .contains(&direction)
            {
                root.focus_cell(root.end_index())
            } else {
                self.parent_focus(direction)
            }
        }
        fn direction_changed(&self, previous: gtk::TextDirection) {
            self.parent_direction_changed(previous);
            if let Some(entries) = self.entries.get() {
                for entry in entries {
                    entry.set_direction(self.obj().direction());
                }
            }
        }
    }
    impl BoxImpl for OtpField {}
}
glib::wrapper! {pub struct OtpField(ObjectSubclass<imp::OtpField>) @extends gtk::Widget,gtk::Box,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;}
impl OtpField {
    pub fn new(options: OtpOptions) -> Result<Self, &'static str> {
        if options.length == 0 || options.length > i32::MAX as usize {
            return Err("OTP length must be a positive GTK-sized integer");
        }
        let root: Self = glib::Object::new();
        root.add_css_class("dz-otp-field");
        root.set_accessible_role(gtk::AccessibleRole::Group);
        root.set_orientation(gtk::Orientation::Horizontal);
        root.set_dazz_size(options.size);
        root.set_dazz_tint(options.tint);
        root.imp().controlled.set(options.value.is_some());
        root.imp().validation.set(options.validation);
        root.imp().read_only.set(options.read_only);
        let entries: Vec<_> = (0..options.length)
            .map(|index| {
                let entry = gtk::Entry::new();
                entry.add_css_class("dz-otp-cell");
                entry.set_width_chars(1);
                entry.set_max_width_chars(1);
                gtk::prelude::EditableExt::set_alignment(&entry, 0.5);
                entry.set_visibility(!options.mask);
                entry.set_editable(!options.read_only);
                entry.set_input_purpose(if options.validation == OtpValidation::Numeric {
                    gtk::InputPurpose::Digits
                } else {
                    gtk::InputPurpose::FreeForm
                });
                entry.set_input_hints(
                    gtk::InputHints::NO_SPELLCHECK
                        | if options.validation == OtpValidation::None {
                            gtk::InputHints::empty()
                        } else {
                            gtk::InputHints::NO_EMOJI
                        },
                );
                if options.group_after.is_some_and(|by| {
                    by > 0 && index > 0 && index % by == 0 && index != options.length - 1
                }) {
                    entry.add_css_class("dz-group-end");
                }
                entry.update_property(&[gtk::accessible::Property::Required(options.required)]);
                root.append(&entry);
                entry
            })
            .collect();
        root.imp().entries.set(entries.clone()).unwrap();
        root.set_value(options.value.as_deref().unwrap_or(&options.default_value));
        root.set_sensitive(!options.disabled);
        for (index, entry) in entries.into_iter().enumerate() {
            root.bind_entry(index, &entry);
        }
        let focus = gtk::EventControllerFocus::new();
        let weak = root.downgrade();
        focus.connect_leave(move |_| {
            if let Some(root) = weak.upgrade() {
                root.imp().active.set(root.end_index());
                root.roving();
            }
        });
        root.add_controller(focus);
        root.connect_unmap(|root| root.cancel_pending());
        Ok(root)
    }
    pub fn cells(&self) -> Vec<gtk::Entry> {
        self.imp().entries.get().unwrap().clone()
    }
    pub fn value(&self) -> String {
        self.imp().value.borrow().clone()
    }
    pub fn set_value(&self, value: &str) {
        let value = normalize(value, self.cells().len(), self.imp().validation.get()).0;
        let changed = self.value() != value;
        self.imp().value.replace(value.clone());
        self.refresh();
        if changed {
            let waiting = self.imp().waiting.borrow_mut().take();
            if let Some(commit) = waiting
                && commit.value == value
            {
                self.finish(commit);
            }
        }
    }
    pub fn set_read_only(&self, read_only: bool) {
        self.imp().read_only.set(read_only);
        for entry in self.cells() {
            entry.set_editable(!read_only);
        }
    }
    pub fn set_mask(&self, mask: bool) {
        for entry in self.cells() {
            entry.set_visibility(!mask);
        }
    }
    pub fn set_invalid(&self, invalid: bool) {
        for entry in self.cells() {
            entry.set_dazz_invalid(invalid);
        }
    }
    pub fn connect_value_changed(
        &self,
        callback: impl Fn(&Self, &OtpChange) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("value-change", false, move |v| {
            callback(
                &v[0].get::<Self>().unwrap(),
                &v[1].get::<OtpChange>().unwrap(),
            );
            None
        })
    }
    pub fn connect_value_complete(
        &self,
        callback: impl Fn(&Self, &str, &str) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_text_signal("value-complete", callback)
    }
    pub fn connect_value_invalid(
        &self,
        callback: impl Fn(&Self, &str, &str) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_text_signal("value-invalid", callback)
    }
    fn connect_text_signal(
        &self,
        name: &str,
        callback: impl Fn(&Self, &str, &str) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local(name, false, move |v| {
            callback(
                &v[0].get::<Self>().unwrap(),
                &v[1].get::<String>().unwrap(),
                &v[2].get::<String>().unwrap(),
            );
            None
        })
    }
    fn end_index(&self) -> usize {
        self.value().chars().count().min(self.cells().len() - 1)
    }
    pub fn focus_cell(&self, index: usize) -> bool {
        if !self.is_sensitive() {
            return false;
        }
        let index = index.min(self.end_index());
        let entry = self.cells()[index].clone();
        self.imp().active.set(index);
        let text = entry
            .delegate()
            .unwrap()
            .dynamic_cast::<gtk::Widget>()
            .unwrap();
        text.set_focusable(true);
        let focused = entry.grab_focus();
        entry.select_region(0, -1);
        self.roving();
        focused
    }
    fn roving(&self) {
        for (index, entry) in self.cells().iter().enumerate() {
            entry
                .delegate()
                .unwrap()
                .dynamic_cast::<gtk::Widget>()
                .unwrap()
                .set_focusable(index == self.imp().active.get());
        }
    }
    fn refresh(&self) {
        let before = self.imp().applying.replace(true);
        let chars: Vec<_> = self.value().chars().collect();
        for (index, entry) in self.cells().iter().enumerate() {
            let value = chars.get(index).map(char::to_string).unwrap_or_default();
            if entry.text() != value {
                entry.set_text(&value);
            }
            if value.is_empty() {
                entry.remove_css_class("dz-filled");
            } else {
                entry.add_css_class("dz-filled");
            }
        }
        if !self.has_focus_within() {
            self.imp().active.set(self.end_index());
        }
        self.roving();
        self.imp().applying.set(before);
    }
    fn has_focus_within(&self) -> bool {
        self.root()
            .and_downcast::<gtk::Window>()
            .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w))
            .is_some_and(|focus| focus.is_ancestor(self))
    }
    fn change(&self, next: String, reason: OtpReason, focus: usize) {
        let old = self.value();
        let can_complete = matches!(reason, OtpReason::InputChange | OtpReason::InputPaste);
        let complete = can_complete
            && next.chars().count() == self.cells().len()
            && (old.chars().count() != self.cells().len() || reason == OtpReason::InputPaste);
        if old == next {
            if complete {
                self.emit_by_name::<()>("value-complete", &[&next, &reason.as_str()]);
            }
            return;
        }
        let edit = OtpChange {
            value: next.clone(),
            reason,
            canceled: Rc::default(),
        };
        self.emit_by_name::<()>("value-change", &[&edit]);
        if edit.is_canceled() {
            self.refresh();
            return;
        }
        if !self.imp().controlled.get() {
            self.set_value(&next);
        }
        let commit = Commit {
            value: next,
            focus,
            reason,
            complete,
        };
        if self.value() == commit.value {
            self.finish(commit);
        } else {
            self.imp().waiting.replace(Some(commit));
            self.refresh();
        }
    }
    fn finish(&self, commit: Commit) {
        self.cancel_pending();
        let weak = self.downgrade();
        self.imp()
            .pending
            .replace(Some(glib::idle_add_local_once(move || {
                if let Some(root) = weak.upgrade() {
                    root.imp().pending.borrow_mut().take();
                    if root.value() == commit.value {
                        if commit.reason != OtpReason::InputClear {
                            root.focus_cell(commit.focus);
                        }
                        if commit.complete {
                            root.emit_by_name::<()>(
                                "value-complete",
                                &[&commit.value, &commit.reason.as_str()],
                            );
                        }
                    }
                }
            })));
    }
    /// Insert or paste text at a cell. Native clipboard and input methods feed this same path.
    pub fn insert(&self, index: usize, raw: &str, reason: OtpReason) {
        if !self.is_sensitive() || self.imp().read_only.get() || index >= self.cells().len() {
            return;
        }
        self.cancel_pending();
        let (digits, rejected) = normalize(raw, self.cells().len(), self.imp().validation.get());
        if rejected {
            self.emit_by_name::<()>("value-invalid", &[&raw, &reason.as_str()]);
        }
        if digits.is_empty() {
            self.refresh();
            if self.value().chars().nth(index).is_some() {
                self.finish(Commit {
                    value: self.value(),
                    focus: index,
                    reason,
                    complete: false,
                });
            }
            return;
        }
        let index = index.min(self.value().chars().count());
        let next = replaced(&self.value(), index, &digits, self.cells().len());
        self.change(
            next,
            reason,
            (index + digits.chars().count()).min(self.cells().len() - 1),
        );
    }
    fn cancel_pending(&self) {
        if let Some(pending) = self.imp().pending.borrow_mut().take() {
            pending.remove();
        }
    }
    fn queue_delete(&self, index: usize) {
        self.cancel_pending();
        let weak = self.downgrade();
        self.imp()
            .pending
            .replace(Some(glib::idle_add_local_once(move || {
                if let Some(root) = weak.upgrade() {
                    root.imp().pending.borrow_mut().take();
                    if root.is_sensitive() && !root.imp().read_only.get() {
                        root.change(removed(&root.value(), index), OtpReason::InputClear, index);
                    }
                }
            })));
    }
    fn bind_entry(&self, index: usize, entry: &gtk::Entry) {
        let weak = self.downgrade();
        entry.connect_changed(move |_| {
            if let Some(root) = weak.upgrade()
                && !root.imp().applying.get()
            {
                root.cancel_pending();
                let weak = root.downgrade();
                root.imp()
                    .pending
                    .replace(Some(glib::idle_add_local_once(move || {
                        if let Some(root) = weak.upgrade() {
                            root.imp().pending.borrow_mut().take();
                            let raw = root.cells()[index].text();
                            root.refresh();
                            if raw.is_empty() {
                                root.change(
                                    removed(&root.value(), index),
                                    OtpReason::InputClear,
                                    index,
                                );
                            } else {
                                root.insert(index, &raw, OtpReason::InputChange);
                            }
                        }
                    })));
            }
        });

        let delegate = entry.delegate().unwrap();
        let text = delegate.clone().dynamic_cast::<gtk::Text>().unwrap();
        let weak = self.downgrade();
        delegate.connect_insert_text(move |editable, raw, _| {
            if let Some(root) = weak.upgrade()
                && !root.imp().applying.get()
            {
                editable.stop_signal_emission_by_name("insert-text");
                root.insert(index, raw, OtpReason::InputChange);
            }
        });
        let weak = self.downgrade();
        delegate.connect_delete_text(move |editable, _, _| {
            if let Some(root) = weak.upgrade()
                && !root.imp().applying.get()
            {
                editable.stop_signal_emission_by_name("delete-text");
                if root.is_sensitive() && !root.imp().read_only.get() {
                    root.queue_delete(index);
                }
            }
        });
        let weak = self.downgrade();
        text.connect_paste_clipboard(move |text| {
            text.stop_signal_emission_by_name("paste-clipboard");
            let Some(root) = weak.upgrade().filter(|root| {
                root.is_sensitive() && !root.imp().read_only.get() && root.is_mapped()
            }) else {
                return;
            };
            let weak = root.downgrade();
            text.clipboard()
                .read_text_async(None::<&gtk::gio::Cancellable>, move |result| {
                    if let Some(root) = weak.upgrade()
                        && root.is_mapped()
                        && let Ok(Some(raw)) = result
                    {
                        root.insert(index, &raw, OtpReason::InputPaste);
                    }
                });
        });
        let focus = gtk::EventControllerFocus::new();
        let weak = self.downgrade();
        focus.connect_enter(move |_| {
            if let Some(root) = weak.upgrade() {
                root.focus_cell(index);
            }
        });
        entry.add_controller(focus);
        let click = gtk::GestureClick::new();
        click.set_button(1);
        click.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = self.downgrade();
        click.connect_pressed(move |click, _, _, _| {
            if let Some(root) = weak.upgrade()
                && root.is_sensitive()
            {
                root.focus_cell(index);
                click.set_state(gtk::EventSequenceState::Claimed);
            }
        });
        entry.add_controller(click);
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
            let rtl = root.direction() == gtk::TextDirection::Rtl;
            let previous = if rtl { Key::Right } else { Key::Left };
            let next = if rtl { Key::Left } else { Key::Right };
            let boundary = mods.intersects(M::CONTROL_MASK | M::META_MASK | M::SUPER_MASK)
                && !mods.contains(M::ALT_MASK);
            let target = if key == previous {
                Some(if boundary { 0 } else { index.saturating_sub(1) })
            } else if key == next {
                Some(if boundary {
                    root.end_index()
                } else {
                    index + 1
                })
            } else if [Key::Home, Key::Up].contains(&key) {
                Some(0)
            } else if [Key::End, Key::Down].contains(&key) {
                Some(root.end_index())
            } else {
                None
            };
            if let Some(target) = target {
                root.focus_cell(target);
                return P::Stop;
            }
            if root.imp().read_only.get() {
                return P::Proceed;
            }
            if key == Key::BackSpace || key == Key::Delete || key == Key::KP_Delete {
                root.cancel_pending();
                let back = key == Key::BackSpace;
                let target = if back { index.saturating_sub(1) } else { index };
                let delete = if root.value().chars().nth(index).is_none() {
                    target
                } else {
                    index
                };
                if back && boundary {
                    root.change(String::new(), OtpReason::Keyboard, 0);
                } else {
                    root.change(removed(&root.value(), delete), OtpReason::Keyboard, target);
                }
                return P::Stop;
            }
            if let Some(c) = key.to_unicode()
                && root.value().chars().nth(index) == Some(c)
                && root.cells()[index].selection_bounds() == Some((0, 1))
            {
                root.focus_cell(index + 1);
                return P::Stop;
            }
            P::Proceed
        });
        entry.add_controller(keys);
    }
}
pub fn otp_field(options: OtpOptions) -> Result<OtpField, &'static str> {
    OtpField::new(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalization_filters_without_counting_whitespace_as_rejected() {
        assert_eq!(
            normalize(" \u{feff}12 34\t", 4, OtpValidation::Numeric),
            ("1234".into(), false)
        );
        assert_eq!(
            normalize("1a2٣345", 4, OtpValidation::Numeric),
            ("1234".into(), true)
        );
        assert_eq!(
            normalize("A_b9cD", 4, OtpValidation::Alpha),
            ("AbcD".into(), true)
        );
        assert_eq!(
            normalize("A-b 12", 4, OtpValidation::Alphanumeric),
            ("Ab12".into(), true)
        );
        assert_eq!(
            normalize("😀α B", 2, OtpValidation::None),
            ("😀α".into(), true)
        );
        assert_eq!(replaced("123456", 1, "98", 6), "198456");
        assert_eq!(removed("123456", 2), "12456");
    }
}
