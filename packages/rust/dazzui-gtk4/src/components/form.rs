//! Form coordination over native text controls. GTK owns editing, activation,
//! focus and selection; the field keeps validation boundaries and the form
//! collects fields before publishing a submission.
use crate::{DazzWidgetExt, Tint, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::{
    cell::{Cell, OnceCell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ValidationMode {
    #[default]
    OnSubmit,
    OnBlur,
    OnChange,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextConstraint {
    #[default]
    Text,
    Email,
    Url,
}
#[derive(Clone, Debug, Default)]
pub struct FieldRules {
    pub name: Option<String>,
    pub required: bool,
    pub constraint: TextConstraint,
    pub validation_mode: Option<ValidationMode>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FieldValidity {
    pub valid: bool,
    pub value_missing: bool,
    pub type_mismatch: bool,
}
#[derive(Clone)]
pub(super) enum Input {
    Entry(gtk::Entry),
    TextView(gtk::TextView),
}
impl Input {
    fn widget(&self) -> gtk::Widget {
        match self {
            Self::Entry(e) => e.clone().upcast(),
            Self::TextView(e) => e.clone().upcast(),
        }
    }
    fn value(&self) -> String {
        match self {
            Self::Entry(e) => e.text().into(),
            Self::TextView(e) => {
                let b = e.buffer();
                b.text(&b.start_iter(), &b.end_iter(), true).into()
            }
        }
    }
    fn editable(&self) -> bool {
        match self {
            Self::Entry(e) => e.is_editable(),
            Self::TextView(e) => e.is_editable(),
        }
    }
    fn select(&self) {
        if let Self::Entry(e) = self {
            e.select_region(0, -1);
        }
    }
}
fn input_in(widget: &gtk::Widget) -> Option<Input> {
    if let Some(entry) = widget.downcast_ref::<gtk::Entry>() {
        return Some(Input::Entry(entry.clone()));
    }
    if let Some(view) = widget.downcast_ref::<gtk::TextView>() {
        return Some(Input::TextView(view.clone()));
    }
    if widget.is::<crate::Combobox>() || widget.is::<crate::OtpField>() {
        return None;
    }
    let mut child = widget.first_child();
    let mut found = None;
    while let Some(node) = child {
        child = node.next_sibling();
        if let Some(input) = input_in(&node) {
            if found.is_some() {
                return None;
            }
            found = Some(input);
        }
    }
    found
}
fn focus_in(widget: &gtk::Widget) -> gtk::Widget {
    if widget.is_focusable() {
        return widget.clone();
    }
    let mut child = widget.first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        let result = focus_in(&node);
        if result.is_focusable() {
            return result;
        }
    }
    widget.clone()
}
fn invalid(widget: &gtk::Widget, value: bool) {
    if value {
        widget.add_css_class("dz-field-invalid");
    } else {
        widget.remove_css_class("dz-field-invalid");
    }
    widget.update_state(&[gtk::accessible::State::Invalid(
        if value || widget.has_css_class("dz-invalid") {
            gtk::AccessibleInvalidState::True
        } else {
            gtk::AccessibleInvalidState::False
        },
    )]);
}
fn paint_controls(widget: &gtk::Widget, value: bool) {
    if [
        "dz-field",
        "dz-select",
        "dz-text-area",
        "dz-text-area-container",
    ]
    .iter()
    .any(|class| widget.has_css_class(class))
    {
        invalid(widget, value && widget.is_sensitive());
    }
    let mut child = widget.first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        paint_controls(&node, value);
    }
}
pub(super) type Resetter = (glib::WeakRef<gtk::Widget>, Rc<dyn Fn()>);
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct FormField {
        pub control: OnceCell<gtk::Widget>,
        pub title: OnceCell<gtk::Label>,
        pub hint: OnceCell<gtk::Label>,
        pub hint_slot: OnceCell<gtk::Box>,
        pub(super) input: OnceCell<Input>,
        pub rules: RefCell<FieldRules>,
        pub initial: RefCell<String>,
        pub validity: Cell<Option<FieldValidity>>,
        pub forced: Cell<bool>,
        pub dirty: Cell<bool>,
        pub marked_dirty: Cell<bool>,
        pub touched: Cell<bool>,
        pub filled: Cell<bool>,
        pub form: glib::WeakRef<super::Form>,
        pub pending: RefCell<Option<glib::SourceId>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for FormField {
        const NAME: &'static str = "DazzFormField";
        type Type = super::FormField;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for FormField {
        fn dispose(&self) {
            if let Some(source) = self.pending.borrow_mut().take() {
                source.remove();
            }
        }
    }
    impl WidgetImpl for FormField {}
    impl BoxImpl for FormField {}
    #[derive(Default)]
    pub struct Form {
        pub fields: RefCell<Vec<glib::WeakRef<super::FormField>>>,
        pub mode: Cell<ValidationMode>,
        pub attempted: Cell<bool>,
        pub submitted: Cell<bool>,
        pub resetting: Cell<bool>,
        pub errors: RefCell<BTreeMap<String, Vec<String>>>,
        pub(super) resetters: RefCell<Vec<Resetter>>,
        pub submitters: RefCell<Vec<glib::WeakRef<gtk::Button>>>,
        pub entries: Cell<usize>,
        pub activations: RefCell<Vec<(glib::WeakRef<gtk::Entry>, glib::SignalHandlerId)>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Form {
        const NAME: &'static str = "DazzForm";
        type Type = super::Form;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for Form {
        fn dispose(&self) {
            for (entry, handler) in self.activations.borrow_mut().drain(..) {
                if let Some(entry) = entry.upgrade() {
                    entry.disconnect(handler);
                }
            }
        }
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: std::sync::OnceLock<Vec<glib::subclass::Signal>> =
                std::sync::OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("submit")
                        .param_types([glib::Variant::static_type()])
                        .build(),
                    glib::subclass::Signal::builder("reset").build(),
                ]
            })
        }
    }
    impl WidgetImpl for Form {}
    impl BoxImpl for Form {}
}
glib::wrapper! {pub struct FormField(ObjectSubclass<imp::FormField>) @extends gtk::Widget,gtk::Box,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;}
glib::wrapper! {pub struct Form(ObjectSubclass<imp::Form>) @extends gtk::Widget,gtk::Box,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;}
impl FormField {
    pub fn new(
        label: &str,
        control: &impl IsA<gtk::Widget>,
        hint: Option<&str>,
        rules: FieldRules,
    ) -> Self {
        let field: Self = glib::Object::new();
        field.set_orientation(gtk::Orientation::Vertical);
        field.add_css_class("dz-form-field");
        field.set_dazz_tint(Tint::Primary);
        let control = control.as_ref();
        let input = input_in(control);
        let target = input
            .as_ref()
            .map(Input::widget)
            .unwrap_or_else(|| focus_in(control));
        let title = crate::label(label, "label-small");
        title.add_css_class("dz-form-field-label");
        title.set_halign(gtk::Align::Start);
        title.set_mnemonic_widget(Some(&target));
        target.update_relation(&[gtk::accessible::Relation::LabelledBy(&[title.upcast_ref()])]);
        let click = gtk::GestureClick::new();
        click.set_button(1);
        let weak = target.downgrade();
        click.connect_released(move |_, _, _, _| {
            if let Some(target) = weak.upgrade() {
                target.grab_focus();
            }
        });
        title.add_controller(click);
        field.append(&title);
        field.append(control);
        let description = crate::label(hint.unwrap_or(""), "label-quiet");
        description.add_css_class("dz-form-field-hint");
        description.set_wrap(true);
        let hint_slot = super::hint_layout::wrap(&description);
        hint_slot.set_visible(hint.is_some_and(|hint| !hint.is_empty()));
        field.append(&hint_slot);
        target.update_relation(&[gtk::accessible::Relation::DescribedBy(&[
            description.upcast_ref()
        ])]);
        field.imp().control.set(control.clone()).unwrap();
        field.imp().title.set(title).unwrap();
        field.imp().hint.set(description).unwrap();
        field.imp().hint_slot.set(hint_slot).unwrap();
        if let Some(input) = input {
            field
                .imp()
                .initial
                .replace(sanitized(input.value(), rules.constraint));
            field.imp().filled.set(!input.value().is_empty());
            let weak = field.downgrade();
            match &input {
                Input::Entry(entry) => {
                    entry.connect_changed(move |_| {
                        if let Some(field) = weak.upgrade() {
                            field.schedule_change();
                        }
                    });
                    let weak = field.downgrade();
                    entry.connect_activate(move |_| {
                        if let Some(field) = weak.upgrade() {
                            field.flush();
                            field.imp().touched.set(true);
                            field.commit(false);
                        }
                    });
                }
                Input::TextView(view) => {
                    view.buffer().connect_changed(move |_| {
                        if let Some(field) = weak.upgrade() {
                            field.schedule_change();
                        }
                    });
                }
            }
            let focus = gtk::EventControllerFocus::new();
            let weak = field.downgrade();
            focus.connect_leave(move |_| {
                if let Some(field) = weak.upgrade() {
                    field.flush();
                    field.imp().touched.set(true);
                    if field.mode() == ValidationMode::OnBlur {
                        field.commit(false);
                    }
                    field.apply();
                }
            });
            input.widget().add_controller(focus);
            let weak = field.downgrade();
            input.widget().connect_state_flags_changed(move |_, _| {
                if let Some(field) = weak.upgrade() {
                    field.apply();
                }
            });
            field.imp().input.set(input).ok().unwrap();
        }
        field.set_rules(rules);
        field.connect_state_flags_changed(|field, _| field.apply());
        field
    }
    pub fn control(&self) -> gtk::Widget {
        self.imp().control.get().unwrap().clone()
    }
    pub fn set_label(&self, label: &str) {
        self.imp().title.get().unwrap().set_label(label);
    }
    pub fn set_hint(&self, hint: Option<&str>) {
        let label = self.imp().hint.get().unwrap();
        label.set_label(hint.unwrap_or(""));
        self.imp()
            .hint_slot
            .get()
            .unwrap()
            .set_visible(hint.is_some_and(|hint| !hint.is_empty()));
    }
    pub fn set_rules(&self, rules: FieldRules) {
        if let Some(input) = self.imp().input.get() {
            input
                .widget()
                .update_property(&[gtk::accessible::Property::Required(rules.required)]);
        }
        self.imp().rules.replace(rules);
        self.apply();
    }
    pub fn set_invalid(&self, value: bool) {
        self.imp().forced.set(value);
        self.apply();
    }
    pub fn value(&self) -> String {
        sanitized(
            self.imp().input.get().map(Input::value).unwrap_or_default(),
            self.imp().rules.borrow().constraint,
        )
    }
    pub fn is_dirty(&self) -> bool {
        self.imp().dirty.get()
    }
    pub fn is_touched(&self) -> bool {
        self.imp().touched.get()
    }
    pub fn validity(&self) -> Option<FieldValidity> {
        self.imp().validity.get()
    }
    pub fn is_invalid(&self) -> bool {
        self.imp().forced.get()
            || !self.server_errors().is_empty()
            || self.is_sensitive() && self.imp().validity.get().is_some_and(|v| !v.valid)
    }
    pub fn server_errors(&self) -> Vec<String> {
        self.imp()
            .form
            .upgrade()
            .and_then(|form| {
                self.imp()
                    .rules
                    .borrow()
                    .name
                    .as_ref()
                    .and_then(|name| form.imp().errors.borrow().get(name).cloned())
            })
            .unwrap_or_default()
    }
    pub fn validate(&self) -> bool {
        self.flush();
        self.commit(true);
        !self.is_invalid()
    }
    fn eligible(&self) -> bool {
        self.imp()
            .input
            .get()
            .is_some_and(|input| input.widget().is_sensitive())
    }
    fn mode(&self) -> ValidationMode {
        self.imp()
            .rules
            .borrow()
            .validation_mode
            .unwrap_or_else(|| {
                self.imp()
                    .form
                    .upgrade()
                    .map_or(ValidationMode::OnSubmit, |form| form.imp().mode.get())
            })
    }
    fn native_validity(&self) -> FieldValidity {
        let Some(input) = self.imp().input.get() else {
            return FieldValidity {
                valid: true,
                ..Default::default()
            };
        };
        let rules = self.imp().rules.borrow();
        let value = self.value();
        let missing = rules.required && input.editable() && value.is_empty();
        let mismatch = if value.is_empty() {
            false
        } else {
            match rules.constraint {
                TextConstraint::Text => false,
                TextConstraint::Email => !valid_email(&value),
                TextConstraint::Url => url::Url::parse(&value).is_err(),
            }
        };
        FieldValidity {
            valid: !missing && !mismatch,
            value_missing: missing,
            type_mismatch: mismatch,
        }
    }
    fn commit(&self, mark_dirty: bool) {
        if mark_dirty {
            self.imp().marked_dirty.set(true);
        }
        let mut validity = if self.eligible() {
            self.native_validity()
        } else {
            FieldValidity {
                valid: true,
                ..Default::default()
            }
        };
        if validity.value_missing && !validity.type_mismatch && !self.imp().marked_dirty.get() {
            validity.valid = true;
            validity.value_missing = false;
        }
        self.imp().validity.set(Some(validity));
        self.apply();
    }
    fn apply(&self) {
        let value = self.is_invalid();
        invalid(self.upcast_ref(), value);
        paint_controls(&self.control(), value);
        for (class, active) in [
            ("dz-dirty", self.is_dirty()),
            ("dz-touched", self.is_touched()),
            ("dz-filled", self.imp().filled.get()),
        ] {
            if active {
                self.add_css_class(class);
            } else {
                self.remove_css_class(class);
            }
        }
    }
    fn schedule_change(&self) {
        if self
            .imp()
            .form
            .upgrade()
            .is_some_and(|form| form.imp().resetting.get())
            || self.imp().pending.borrow().is_some()
        {
            return;
        }
        let weak = self.downgrade();
        let source = glib::idle_add_local_once(move || {
            if let Some(field) = weak.upgrade() {
                field.imp().pending.borrow_mut().take();
                field.changed();
            }
        });
        self.imp().pending.replace(Some(source));
    }
    fn flush(&self) {
        let pending = self.imp().pending.borrow_mut().take();
        if let Some(source) = pending {
            source.remove();
            self.changed();
        }
    }
    fn changed(&self) {
        let value = self.value();
        let dirty = value != *self.imp().initial.borrow();
        self.imp().dirty.set(dirty);
        if dirty {
            self.imp().marked_dirty.set(true);
        }
        self.imp().filled.set(!value.is_empty());
        if let Some(form) = self.imp().form.upgrade()
            && let Some(name) = self.imp().rules.borrow().name.as_ref()
        {
            form.imp().errors.borrow_mut().remove(name);
        }
        let full = self.mode() == ValidationMode::OnChange
            || self.mode() == ValidationMode::OnSubmit
                && self
                    .imp()
                    .form
                    .upgrade()
                    .is_some_and(|form| form.imp().attempted.get());
        if full {
            self.commit(false);
        } else if self.imp().validity.get().is_some_and(|v| !v.valid) && self.eligible() {
            let native = self.native_validity();
            self.imp().validity.set(Some(if !native.value_missing {
                FieldValidity {
                    valid: true,
                    ..Default::default()
                }
            } else {
                native
            }));
        }
        self.apply();
    }
    fn focus_invalid(&self) {
        let target = self
            .imp()
            .input
            .get()
            .map(Input::widget)
            .unwrap_or_else(|| focus_in(&self.control()));
        target.grab_focus();
        if let Some(input) = self.imp().input.get() {
            input.select();
        }
    }
}
impl Default for Form {
    fn default() -> Self {
        Self::new()
    }
}
impl Form {
    pub fn new() -> Self {
        let form: Self = glib::Object::new();
        form.set_orientation(gtk::Orientation::Vertical);
        form.add_css_class("dz-form");
        form.set_accessible_role(gtk::AccessibleRole::Form);
        form
    }
    pub fn set_validation_mode(&self, mode: ValidationMode) {
        self.imp().mode.set(mode);
    }
    pub fn register_field(&self, field: &FormField) -> Result<(), &'static str> {
        if field.imp().input.get().is_none() {
            return Err("Form validation currently requires a native text input");
        }
        if !self.fields().contains(field) {
            self.imp().fields.borrow_mut().push(field.downgrade());
        }
        field.imp().form.set(Some(self));
        field.apply();
        Ok(())
    }
    pub fn unregister_field(&self, field: &FormField) {
        self.imp()
            .fields
            .borrow_mut()
            .retain(|f| f.upgrade().as_ref() != Some(field));
        field.imp().form.set(None::<&Form>);
        field.apply();
    }
    pub fn set_errors(&self, errors: BTreeMap<String, Vec<String>>) {
        self.imp().errors.replace(errors);
        for field in self.fields() {
            field.apply();
        }
        if self.imp().submitted.replace(false) {
            self.focus_first_invalid();
        }
    }
    pub fn values(&self) -> BTreeMap<String, String> {
        self.fields()
            .into_iter()
            .filter(|field| field.eligible())
            .filter_map(|field| {
                field
                    .imp()
                    .rules
                    .borrow()
                    .name
                    .clone()
                    .map(|name| (name, field.value()))
            })
            .collect()
    }
    pub fn validate(&self) -> bool {
        let mut valid = true;
        for field in self.fields() {
            if field.eligible() {
                valid &= field.validate();
            }
        }
        valid
    }
    pub fn submit(&self) -> bool {
        self.imp().attempted.set(true);
        if !self.validate() {
            self.focus_first_invalid();
            return false;
        }
        self.imp().submitted.set(true);
        self.emit_by_name::<()>("submit", &[&self.values().to_variant()]);
        true
    }
    pub fn reset(&self) {
        for field in self.fields() {
            field.flush();
        }
        self.imp().resetting.set(true);
        for (_, reset) in self.imp().resetters.borrow().clone() {
            reset();
        }
        self.imp().resetting.set(false);
        self.emit_by_name::<()>("reset", &[]);
    }
    pub fn connect_submit(
        &self,
        callback: impl Fn(&Self, &BTreeMap<String, String>) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("submit", false, move |v| {
            callback(
                &v[0].get::<Self>().unwrap(),
                &v[1]
                    .get::<glib::Variant>()
                    .unwrap()
                    .get::<BTreeMap<String, String>>()
                    .unwrap(),
            );
            None
        })
    }
    pub fn connect_reset(&self, callback: impl Fn(&Self) + 'static) -> glib::SignalHandlerId {
        self.connect_local("reset", false, move |v| {
            callback(&v[0].get::<Self>().unwrap());
            None
        })
    }
    pub fn submit_button(&self, button: &gtk::Button) {
        self.imp().submitters.borrow_mut().push(button.downgrade());
        let weak = self.downgrade();
        button.connect_clicked(move |button| {
            if button.is_sensitive()
                && let Some(form) = weak.upgrade()
            {
                form.submit();
            }
        });
    }
    pub fn reset_button(&self, button: &gtk::Button) {
        let weak = self.downgrade();
        button.connect_clicked(move |button| {
            if button.is_sensitive()
                && let Some(form) = weak.upgrade()
            {
                form.reset();
            }
        });
    }
    /// Capture the default text of the form's controls after its children have
    /// been constructed. Reset changes the native values without fabricating an
    /// input event or clearing validation state, just as a browser reset does.
    pub fn capture_defaults(&self) {
        for (entry, handler) in self.imp().activations.borrow_mut().drain(..) {
            if let Some(entry) = entry.upgrade() {
                entry.disconnect(handler);
            }
        }
        let mut widgets = vec![];
        walk(self.upcast_ref(), &mut widgets);
        self.imp().resetters.borrow_mut().clear();
        self.imp().entries.set(0);
        for widget in widgets {
            if let Some(entry) = widget.downcast_ref::<gtk::Entry>() {
                let initial = entry.text().to_string();
                let weak = entry.downgrade();
                self.imp().resetters.borrow_mut().push((
                    widget.downgrade(),
                    Rc::new(move || {
                        if let Some(entry) = weak.upgrade() {
                            entry.set_text(&initial);
                        }
                    }),
                ));
                self.imp().entries.set(self.imp().entries.get() + 1);
                let weak = self.downgrade();
                entry.set_activates_default(false);
                let handler = entry.connect_activate(move |_| {
                    if let Some(form) = weak.upgrade() {
                        form.implicit_submit();
                    }
                });
                self.imp()
                    .activations
                    .borrow_mut()
                    .push((entry.downgrade(), handler));
            } else if let Some(view) = widget.downcast_ref::<gtk::TextView>() {
                let buffer = view.buffer();
                let initial = buffer
                    .text(&buffer.start_iter(), &buffer.end_iter(), true)
                    .to_string();
                let weak = view.downgrade();
                self.imp().resetters.borrow_mut().push((
                    widget.downgrade(),
                    Rc::new(move || {
                        if let Some(view) = weak.upgrade() {
                            view.buffer().set_text(&initial);
                        }
                    }),
                ));
            }
        }
    }
    fn implicit_submit(&self) {
        let first = self
            .imp()
            .submitters
            .borrow()
            .iter()
            .find_map(glib::WeakRef::upgrade);
        if let Some(button) = first {
            if button.is_sensitive() {
                button.emit_clicked();
            }
        } else if self.imp().entries.get() <= 1 {
            self.submit();
        }
    }
    fn fields(&self) -> Vec<FormField> {
        let mut order = vec![];
        walk(self.upcast_ref(), &mut order);
        let mut fields: Vec<_> = self
            .imp()
            .fields
            .borrow()
            .iter()
            .filter_map(glib::WeakRef::upgrade)
            .collect();
        fields.sort_by_key(|field| {
            order
                .iter()
                .position(|widget| widget == field.upcast_ref::<gtk::Widget>())
                .unwrap_or(usize::MAX)
        });
        fields
    }
    fn focus_first_invalid(&self) {
        if let Some(field) = self
            .fields()
            .into_iter()
            .find(|field| field.eligible() && field.is_invalid())
        {
            field.focus_invalid();
        }
    }
}
fn walk(widget: &gtk::Widget, out: &mut Vec<gtk::Widget>) {
    out.push(widget.clone());
    let mut child = widget.first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        if !node.is::<Form>() {
            walk(&node, out);
        }
    }
}
fn sanitized(raw: String, constraint: TextConstraint) -> String {
    if constraint == TextConstraint::Text {
        return raw;
    }
    raw.replace(['\n', '\r'], "")
        .trim_matches(|c| matches!(c, '\t' | '\x0c' | ' '))
        .to_owned()
}
fn valid_email(raw: &str) -> bool {
    let value = raw.trim_matches(|c| matches!(c, '\t' | '\n' | '\x0c' | '\r' | ' '));
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && local
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b".!#$%&'*+/=?^_`{|}~-".contains(&c))
        && domain.split('.').all(|part| {
            !part.is_empty()
                && part.len() <= 63
                && part.as_bytes()[0].is_ascii_alphanumeric()
                && part.as_bytes()[part.len() - 1].is_ascii_alphanumeric()
                && part.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-')
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn email_uses_the_html_constraint_not_a_mail_delivery_rule() {
        for value in ["a@b", "a+b@b.c", ".a..b@localhost", "a@b.123", " a@b "] {
            assert!(valid_email(value), "{value}");
        }
        for value in [
            "",
            "a@",
            "a b@c",
            "a@-b",
            "a@b-",
            "a@b..c",
            "a@b@c",
            "啊@例子.com",
        ] {
            assert!(!valid_email(value), "{value}");
        }
    }
}
