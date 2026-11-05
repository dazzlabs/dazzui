//! Native editable text, list rows and popup positioning, coordinated as a
//! searchable choice or a free-text autocomplete. The input keeps focus while
//! the list exposes the active descendant.
use crate::{Size, Theme, Tint, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::cell::{Cell, OnceCell, RefCell};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}
#[derive(Clone, Debug)]
pub enum ComboboxItem {
    Option(ComboboxOption),
    Group {
        label: String,
        options: Vec<ComboboxOption>,
    },
}
#[derive(Clone, Debug, glib::Boxed)]
#[boxed_type(name = "DazzComboboxChange")]
pub struct ComboboxChange {
    pub value: Option<String>,
    pub reason: String,
    canceled: Rc<Cell<bool>>,
}
impl ComboboxChange {
    pub fn cancel(&self) {
        self.canceled.set(true);
    }
    pub fn is_canceled(&self) -> bool {
        self.canceled.get()
    }
}
pub struct ComboboxOptions {
    pub options: Vec<ComboboxItem>,
    /// `Some(None)` is a controlled empty choice.
    pub value: Option<Option<String>>,
    pub default_value: Option<String>,
    pub placeholder: String,
    pub empty: String,
    pub clearable: bool,
    pub size: Size,
    pub tint: Tint,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
}
impl Default for ComboboxOptions {
    fn default() -> Self {
        Self {
            options: vec![],
            value: None,
            default_value: None,
            placeholder: "Search…".into(),
            empty: "No matches".into(),
            clearable: true,
            size: Size::Medium,
            tint: Tint::Primary,
            disabled: false,
            read_only: false,
            required: false,
        }
    }
}
pub struct AutocompleteOptions {
    pub items: Vec<String>,
    pub value: Option<String>,
    pub default_value: String,
    pub placeholder: String,
    pub empty: String,
    pub size: Size,
    pub tint: Tint,
    pub mono: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
}
impl Default for AutocompleteOptions {
    fn default() -> Self {
        Self {
            items: vec![],
            value: None,
            default_value: String::new(),
            placeholder: String::new(),
            empty: "No matches".into(),
            size: Size::Medium,
            tint: Tint::Primary,
            mono: false,
            disabled: false,
            read_only: false,
            required: false,
        }
    }
}
#[derive(Clone)]
pub(super) struct Item {
    option: ComboboxOption,
    row: gtk::ListBoxRow,
    tick: gtk::Widget,
}
pub(super) type Watch = (glib::WeakRef<gtk::Window>, gtk::GestureClick);
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Combobox {
        pub entry: OnceCell<gtk::Entry>,
        pub popup: OnceCell<gtk::Popover>,
        pub list: OnceCell<gtk::ListBox>,
        pub empty: OnceCell<gtk::Label>,
        pub scroll: OnceCell<gtk::ScrolledWindow>,
        pub clear: OnceCell<gtk::Button>,
        pub trigger: OnceCell<gtk::Button>,
        pub actions: OnceCell<gtk::Box>,
        pub actions_slot: OnceCell<gtk::Overlay>,
        pub(super) items: RefCell<Vec<Item>>,
        pub groups: RefCell<Vec<(gtk::ListBoxRow, Vec<usize>)>>,
        pub value: RefCell<Option<String>>,
        pub selected_label: RefCell<String>,
        pub autocomplete: Cell<bool>,
        pub controlled: Cell<bool>,
        pub applying: Cell<bool>,
        pub pending: RefCell<Option<glib::SourceId>>,
        pub presentation: RefCell<Option<glib::SourceId>>,
        pub open: Cell<bool>,
        pub query_changed: Cell<bool>,
        pub read_only: Cell<bool>,
        pub clearable: Cell<bool>,
        pub active: Cell<Option<usize>>,
        pub(super) watch: RefCell<Option<Watch>>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Combobox {
        const NAME: &'static str = "DazzCombobox";
        type Type = super::Combobox;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for Combobox {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: std::sync::OnceLock<Vec<glib::subclass::Signal>> =
                std::sync::OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("value-change")
                        .param_types([ComboboxChange::static_type()])
                        .build(),
                ]
            })
        }
        fn dispose(&self) {
            let root = self.obj();
            root.unwatch();
            root.cancel_input();
            root.cancel_presentation();
            if let Some(popup) = self.popup.get() {
                popup.unparent();
            }
        }
    }
    impl WidgetImpl for Combobox {
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            self.parent_size_allocate(width, height, baseline);
            if self.open.get() {
                self.obj().position_popup();
            }
        }
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            if matches!(
                direction,
                gtk::DirectionType::TabForward | gtk::DirectionType::TabBackward
            ) {
                let entry = self.entry.get().unwrap();
                if entry.has_focus() || entry.has_visible_focus() || entry.is_focus() {
                    return false;
                }
                if let Some(focus) = self
                    .obj()
                    .root()
                    .and_downcast::<gtk::Window>()
                    .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w))
                    && focus.is_ancestor(entry)
                {
                    return false;
                }
                return entry.grab_focus();
            }
            self.parent_focus(direction)
        }
    }
    impl BoxImpl for Combobox {}
}
glib::wrapper! { pub struct Combobox(ObjectSubclass<imp::Combobox>) @extends gtk::Widget,gtk::Box,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable; }
impl Combobox {
    pub fn new(options: ComboboxOptions) -> Self {
        let root = Self::build(
            false,
            options.size,
            options.tint,
            &options.placeholder,
            &options.empty,
        );
        root.imp().controlled.set(options.value.is_some());
        root.imp().clearable.set(options.clearable);
        root.set_options(&options.options);
        root.set_value(options.value.unwrap_or(options.default_value).as_deref());
        root.set_read_only(options.read_only);
        root.set_required(options.required);
        root.set_sensitive(!options.disabled);
        root
    }
    fn build(autocomplete: bool, size: Size, tint: Tint, placeholder: &str, empty: &str) -> Self {
        let root: Self = glib::Object::new();
        root.set_orientation(gtk::Orientation::Horizontal);
        root.set_dazz_size(size);
        root.set_dazz_tint(tint);
        root.add_css_class("dz-field");
        root.add_css_class(if autocomplete {
            "dz-autocomplete"
        } else {
            "dz-combobox"
        });
        root.imp().autocomplete.set(autocomplete);
        let entry = gtk::Entry::new();
        entry.add_css_class("dz-combobox-input");
        entry.set_placeholder_text(Some(placeholder));
        entry.set_width_chars(0);
        entry.set_hexpand(true);
        entry.set_accessible_role(gtk::AccessibleRole::ComboBox);
        entry.update_property(&[
            gtk::accessible::Property::Autocomplete(gtk::AccessibleAutocomplete::List),
            gtk::accessible::Property::HasPopup(true),
        ]);
        root.append(&entry);
        let action = |name: &str, glyph: &str| {
            let button = gtk::Button::new();
            button.add_css_class("dz-combobox-action");
            button.set_child(Some(&crate::icon(glyph)));
            button.set_valign(gtk::Align::Center);
            button.set_focusable(false);
            button.set_focus_on_click(false);
            button.set_dazz_label(name);
            button
        };
        let clear = action("Clear", "Dismiss12Regular");
        let trigger = action("Open", "ChevronDown12Regular");
        let actions = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        actions.add_css_class("dz-combobox-actions");
        actions.append(&clear);
        actions.append(&trigger);
        let actions_slot = gtk::Overlay::new();
        actions_slot.set_accessible_role(gtk::AccessibleRole::Presentation);
        actions_slot.set_child(Some(&gtk::Box::new(gtk::Orientation::Horizontal, 0)));
        actions.set_valign(gtk::Align::Center);
        actions_slot.add_overlay(&actions);
        actions_slot.set_measure_overlay(&actions, false);
        actions_slot.set_clip_overlay(&actions, false);
        actions_slot.set_visible(!autocomplete);
        root.append(&actions_slot);
        let popup = gtk::Popover::new();
        popup.add_css_class("dz-popover");
        popup.add_css_class("dz-combobox-popup");
        popup.set_has_arrow(false);
        popup.set_autohide(false);
        popup.set_can_focus(false);
        popup.set_dazz_tint(tint);
        popup.set_position(gtk::PositionType::Bottom);
        popup.set_halign(gtk::Align::Start);
        popup.set_parent(&root);
        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let empty_label = gtk::Label::new(Some(empty));
        empty_label.add_css_class("dz-combobox-empty");
        empty_label.set_xalign(0.);
        empty_label.set_accessible_role(gtk::AccessibleRole::Status);
        content.append(&empty_label);
        let list = gtk::ListBox::new();
        list.add_css_class("dz-combobox-list");
        list.set_accessible_role(gtk::AccessibleRole::ListBox);
        list.set_selection_mode(gtk::SelectionMode::None);
        list.set_activate_on_single_click(true);
        list.set_can_focus(false);
        let scroll = gtk::ScrolledWindow::new();
        scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        scroll.set_propagate_natural_height(true);
        scroll.set_child(Some(&list));
        content.append(&scroll);
        popup.set_child(Some(&content));
        entry.update_relation(&[gtk::accessible::Relation::Controls(&[list.upcast_ref()])]);
        root.imp().actions.set(actions).unwrap();
        root.imp().actions_slot.set(actions_slot).unwrap();
        root.imp().entry.set(entry.clone()).unwrap();
        root.imp().popup.set(popup.clone()).unwrap();
        root.imp().list.set(list.clone()).unwrap();
        root.imp().scroll.set(scroll).unwrap();
        root.imp().empty.set(empty_label).unwrap();
        root.imp().clear.set(clear.clone()).unwrap();
        root.imp().trigger.set(trigger.clone()).unwrap();
        let weak = root.downgrade();
        entry.connect_changed(move |_| {
            if let Some(root) = weak.upgrade() {
                root.schedule_input();
            }
        });
        let weak = root.downgrade();
        list.connect_row_activated(move |_, row| {
            if let Some(root) = weak.upgrade() {
                let index = root
                    .imp()
                    .items
                    .borrow()
                    .iter()
                    .position(|item| item.row == *row);
                if let Some(index) = index {
                    root.choose(index);
                }
            }
        });
        let weak = root.downgrade();
        trigger.connect_clicked(move |_| {
            if let Some(root) = weak.upgrade().filter(|r| r.can_edit()) {
                root.entry().grab_focus();
                if root.is_open() {
                    root.close();
                } else {
                    root.open();
                }
            }
        });
        let weak = root.downgrade();
        clear.connect_clicked(move |_| {
            if let Some(root) = weak.upgrade().filter(|r| r.can_edit()) {
                root.request(None, "clear-press");
                root.write_query("");
                root.highlight(None);
                root.filter();
                root.entry().grab_focus();
            }
        });
        let click = gtk::GestureClick::new();
        click.set_button(1);
        let weak = root.downgrade();
        click.connect_pressed(move |_, _, _, _| {
            if let Some(root) = weak
                .upgrade()
                .filter(|r| !r.imp().autocomplete.get() && r.can_edit())
            {
                root.open();
            }
        });
        entry.add_controller(click);
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = root.downgrade();
        keys.connect_key_pressed(move |_, key, _, mods| {
            weak.upgrade()
                .map_or(glib::Propagation::Proceed, |root| root.key(key, mods))
        });
        entry.add_controller(keys);
        let focus = gtk::EventControllerFocus::new();
        let weak = root.downgrade();
        focus.connect_leave(move |_| {
            let weak = weak.clone();
            glib::idle_add_local_once(move || {
                if let Some(root) = weak.upgrade() {
                    let inside = root
                        .root()
                        .and_downcast::<gtk::Window>()
                        .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w))
                        .is_some_and(|f| {
                            f == root.clone().upcast::<gtk::Widget>() || f.is_ancestor(&root)
                        });
                    if !inside {
                        root.close();
                    }
                }
            });
        });
        root.add_controller(focus);
        let weak = root.downgrade();
        popup.connect_closed(move |_| {
            if let Some(root) = weak.upgrade() {
                root.closed();
            }
        });
        root.connect_unmap(|root| root.close());
        root.connect_sensitive_notify(|root| {
            if !root.is_sensitive() {
                root.close();
            }
        });
        root
    }
    pub fn entry(&self) -> gtk::Entry {
        self.imp().entry.get().unwrap().clone()
    }
    pub fn popup(&self) -> gtk::Popover {
        self.imp().popup.get().unwrap().clone()
    }
    pub fn list(&self) -> gtk::ListBox {
        self.imp().list.get().unwrap().clone()
    }
    pub fn clear_button(&self) -> gtk::Button {
        self.imp().clear.get().unwrap().clone()
    }
    pub fn trigger(&self) -> gtk::Button {
        self.imp().trigger.get().unwrap().clone()
    }
    pub fn value(&self) -> Option<String> {
        self.imp().value.borrow().clone()
    }
    pub fn is_open(&self) -> bool {
        self.imp().open.get()
    }
    pub fn highlighted(&self) -> Option<String> {
        self.imp().active.get().and_then(|i| {
            self.imp()
                .items
                .borrow()
                .get(i)
                .map(|x| x.option.value.clone())
        })
    }
    pub fn visible_options(&self) -> Vec<String> {
        self.imp()
            .items
            .borrow()
            .iter()
            .filter(|x| x.row.get_visible())
            .map(|x| x.option.label.clone())
            .collect()
    }
    pub fn set_empty(&self, text: &str) {
        self.imp().empty.get().unwrap().set_label(text);
    }
    pub fn set_required(&self, required: bool) {
        self.entry()
            .update_property(&[gtk::accessible::Property::Required(required)]);
    }
    pub fn set_read_only(&self, read_only: bool) {
        self.imp().read_only.set(read_only);
        self.entry().set_editable(!read_only);
        self.entry()
            .update_property(&[gtk::accessible::Property::ReadOnly(read_only)]);
        if read_only {
            self.close();
        }
    }
    pub fn set_clearable(&self, clearable: bool) {
        self.imp().clearable.set(clearable);
        self.refresh();
    }
    pub fn set_value(&self, value: Option<&str>) {
        let value = if self.imp().autocomplete.get() {
            Some(value.unwrap_or("").to_owned())
        } else {
            value
                .filter(|v| {
                    self.imp()
                        .items
                        .borrow()
                        .iter()
                        .any(|item| item.option.value == *v)
                })
                .map(str::to_owned)
        };
        let label = self
            .imp()
            .items
            .borrow()
            .iter()
            .find(|item| Some(&item.option.value) == value.as_ref())
            .map(|item| item.option.label.clone())
            .unwrap_or_default();
        let changed = self.value() != value || *self.imp().selected_label.borrow() != label;
        self.imp().selected_label.replace(label);
        self.imp().value.replace(value);
        if changed || !self.is_open() {
            let label = self.label();
            self.write_query(&label);
            self.imp().query_changed.set(false);
        }
        self.refresh();
        self.filter();
    }
    pub fn set_options(&self, options: &[ComboboxItem]) {
        self.highlight(None);
        while let Some(child) = self.list().first_child() {
            self.list().remove(&child);
        }
        self.imp().items.borrow_mut().clear();
        self.imp().groups.borrow_mut().clear();
        let grouped = matches!(options.first(), Some(ComboboxItem::Group { .. }));
        let options: Vec<_> = options
            .iter()
            .flat_map(|item| match (grouped, item) {
                (false, ComboboxItem::Group { options, .. }) => {
                    options.iter().cloned().map(ComboboxItem::Option).collect()
                }
                (true, ComboboxItem::Option(option)) => vec![ComboboxItem::Group {
                    label: option.label.clone(),
                    options: vec![option.clone()],
                }],
                _ => vec![item.clone()],
            })
            .collect();
        for item in &options {
            match item {
                ComboboxItem::Option(option) => {
                    self.append_option(option);
                }
                ComboboxItem::Group { label, options } => {
                    let row = gtk::ListBoxRow::new();
                    row.set_selectable(false);
                    row.set_activatable(false);
                    row.set_focusable(false);
                    row.add_css_class("dz-combobox-heading");
                    let label = gtk::Label::new(Some(label));
                    label.set_xalign(0.);
                    row.set_child(Some(&label));
                    self.list().append(&row);
                    let start = self.imp().items.borrow().len();
                    for option in options {
                        self.append_option(option);
                    }
                    let end = self.imp().items.borrow().len();
                    self.imp()
                        .groups
                        .borrow_mut()
                        .push((row, (start..end).collect()));
                }
            }
        }
        if self.imp().controlled.get() {
            let value = self.value();
            self.set_value(value.as_deref());
        } else {
            self.refresh();
            self.filter();
        }
    }
    fn append_option(&self, option: &ComboboxOption) {
        let index = self.imp().items.borrow().len();
        let row = gtk::ListBoxRow::new();
        row.add_css_class("dz-combobox-row");
        row.set_focusable(false);
        row.set_sensitive(!option.disabled);
        row.set_accessible_role(gtk::AccessibleRole::Option);
        let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        content.add_css_class("dz-combobox-row-content");
        let tick = crate::icon("Checkmark12Filled");
        tick.add_css_class("dz-combobox-check");
        tick.set_visible(!self.imp().autocomplete.get());
        content.append(&tick);
        let label = gtk::Label::new(Some(&option.label));
        label.set_xalign(0.);
        label.set_hexpand(true);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        label.add_css_class("dz-combobox-label");
        content.append(&label);
        row.set_child(Some(&content));
        self.list().append(&row);
        let motion = gtk::EventControllerMotion::new();
        let weak = self.downgrade();
        motion.connect_enter(move |_, _, _| {
            if let Some(root) = weak.upgrade() {
                root.highlight(Some(index));
            }
        });
        let weak = self.downgrade();
        motion.connect_leave(move |_| {
            if let Some(root) = weak.upgrade()
                && root.imp().active.get() == Some(index)
            {
                root.highlight(None);
            }
        });
        row.add_controller(motion);
        self.imp().items.borrow_mut().push(Item {
            option: option.clone(),
            row,
            tick: tick.upcast(),
        });
    }
    pub fn connect_value_changed(
        &self,
        callback: impl Fn(&Self, &ComboboxChange) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("value-change", false, move |v| {
            callback(
                &v[0].get::<Self>().unwrap(),
                &v[1].get::<ComboboxChange>().unwrap(),
            );
            None
        })
    }
    fn can_edit(&self) -> bool {
        self.is_sensitive() && !self.imp().read_only.get()
    }
    fn label(&self) -> String {
        let value = self.value();
        if self.imp().autocomplete.get() {
            return value.unwrap_or_default();
        }
        self.imp().selected_label.borrow().clone()
    }
    fn cancel_input(&self) {
        if let Some(source) = self.imp().pending.borrow_mut().take() {
            source.remove();
        }
    }
    fn schedule_input(&self) {
        if self.imp().applying.get() || self.imp().pending.borrow().is_some() {
            return;
        }
        let weak = self.downgrade();
        let source = glib::idle_add_local_once(move || {
            if let Some(root) = weak.upgrade() {
                root.imp().pending.borrow_mut().take();
                root.input_changed();
            }
        });
        self.imp().pending.replace(Some(source));
    }
    fn flush_input(&self) {
        let pending = self.imp().pending.borrow().is_some();
        self.cancel_input();
        if pending {
            self.input_changed();
        }
    }
    fn write_query(&self, text: &str) {
        self.cancel_input();
        let old = self.imp().applying.replace(true);
        if self.entry().text().as_str() != text {
            self.entry().set_text(text);
        }
        self.imp().applying.set(old);
    }
    fn refresh(&self) {
        let value = self.value();
        self.clear_button().set_visible(
            !self.imp().autocomplete.get() && self.imp().clearable.get() && value.is_some(),
        );
        self.imp().actions_slot.get().unwrap().set_width_request(
            self.imp()
                .actions
                .get()
                .unwrap()
                .measure(gtk::Orientation::Horizontal, -1)
                .1,
        );
        for item in self.imp().items.borrow().iter() {
            let selected = Some(&item.option.value) == value.as_ref();
            item.tick.set_opacity(if selected { 1. } else { 0. });
            if !self.imp().autocomplete.get() {
                item.row
                    .update_state(&[gtk::accessible::State::Selected(Some(selected))]);
            }
        }
    }
    fn filter(&self) {
        let text = self.entry().text();
        let selected = self.label();
        let query = if !self.imp().autocomplete.get()
            && !self.imp().query_changed.get()
            && text.as_str() == selected
        {
            ""
        } else {
            super::choice_filter::trim_query(&text)
        };
        let items = self.imp().items.borrow().clone();
        let mut count = 0;
        for item in &items {
            let visible = contains(&item.option.label, query);
            item.row.set_visible(visible);
            if visible {
                count += 1;
            }
        }
        let mut previous = false;
        for (row, indices) in self.imp().groups.borrow().iter() {
            let visible = indices.iter().any(|i| items[*i].row.get_visible());
            row.set_visible(visible);
            if visible && previous {
                row.add_css_class("dz-combobox-next-group");
            } else {
                row.remove_css_class("dz-combobox-next-group");
            }
            previous |= visible;
        }
        self.imp().empty.get().unwrap().set_visible(count == 0);
        self.imp().scroll.get().unwrap().set_visible(count != 0);
        if self
            .imp()
            .active
            .get()
            .is_some_and(|i| !items.get(i).is_some_and(|x| x.row.get_visible()))
        {
            self.highlight(None);
        }
    }
    fn request(&self, value: Option<String>, reason: &str) -> bool {
        let label = self
            .imp()
            .items
            .borrow()
            .iter()
            .find(|item| Some(&item.option.value) == value.as_ref())
            .map(|item| item.option.label.clone())
            .unwrap_or_default();
        let edit = ComboboxChange {
            value: value.clone(),
            reason: reason.into(),
            canceled: Rc::default(),
        };
        self.emit_by_name::<()>("value-change", &[&edit]);
        if edit.is_canceled() {
            return false;
        }
        if !self.imp().controlled.get() {
            self.imp().value.replace(value);
            self.imp().selected_label.replace(label);
        }
        self.refresh();
        true
    }
    fn input_changed(&self) {
        if self.imp().applying.get() {
            return;
        }
        if !self.can_edit() {
            let value = self.label();
            self.write_query(&value);
            return;
        }
        let query = self.entry().text().to_string();
        if self.imp().autocomplete.get() {
            if !self.request(Some(query.clone()), "input-change")
                || self.imp().controlled.get() && self.value().as_deref() != Some(query.as_str())
            {
                let accepted = self.label();
                self.write_query(&accepted);
            }
        } else if query.is_empty() {
            self.request(None, "input-clear");
        }
        self.imp().query_changed.set(true);
        self.highlight(None);
        self.filter();
        if self.imp().autocomplete.get() && query.is_empty() {
            self.close();
        } else if !super::choice_filter::trim_query(&query).is_empty() {
            self.open();
        }
    }
    pub fn open(&self) {
        if !self.can_edit() || !self.is_mapped() {
            return;
        }
        if self.imp().open.replace(true) {
            return;
        }
        self.add_css_class("dz-popup-open");
        self.entry()
            .update_state(&[gtk::accessible::State::Expanded(Some(true))]);
        self.filter();
        if !self.imp().query_changed.get() {
            let selected = self.value();
            let index = self.imp().items.borrow().iter().position(|x| {
                Some(&x.option.value) == selected.as_ref()
                    && x.row.get_visible()
                    && !x.option.disabled
            });
            self.highlight(index);
        }
        let weak = self.downgrade();
        let presentation = glib::idle_add_local_once(move || {
            if let Some(root) = weak.upgrade() {
                root.imp().presentation.borrow_mut().take();
                if root.is_open() && root.is_mapped() {
                    root.position_popup();
                    root.popup().popup();
                    root.watch();
                }
            }
        });
        self.imp().presentation.replace(Some(presentation));
    }
    fn cancel_presentation(&self) {
        if let Some(source) = self.imp().presentation.borrow_mut().take() {
            source.remove();
        }
    }
    pub fn close(&self) {
        self.flush_input();
        self.cancel_presentation();
        if self.is_open() {
            if self.popup().get_visible() {
                self.popup().popdown();
            }
            self.closed();
        }
    }
    fn closed(&self) {
        self.cancel_presentation();
        if !self.imp().open.replace(false) {
            return;
        }
        self.remove_css_class("dz-popup-open");
        self.entry()
            .update_state(&[gtk::accessible::State::Expanded(Some(false))]);
        self.unwatch();
        self.highlight(None);
        self.imp().query_changed.set(false);
        if !self.imp().autocomplete.get() {
            let label = self.label();
            self.write_query(&label);
        }
    }
    fn position_popup(&self) {
        let theme = Theme::of(self).unwrap_or_default();
        let gap = theme.dimension("spacing-1").unwrap() as i32;
        self.popup().set_offset(0, gap);
        let width = self
            .compute_bounds(self)
            .map_or(self.width(), |r| r.width().round() as i32);
        self.popup().set_size_request(width, -1);
        self.popup()
            .set_halign(if self.direction() == gtk::TextDirection::Rtl {
                gtk::Align::End
            } else {
                gtk::Align::Start
            });
        if let Some(window) = self.root().and_downcast::<gtk::Window>()
            && let Some(rect) = self.compute_bounds(&window)
        {
            let available = (rect
                .y()
                .max(window.height() as f32 - rect.y() - rect.height()))
                as i32;
            let chrome = 2 * (gap + theme.dimension("stroke-hairline").unwrap() as i32);
            self.imp()
                .scroll
                .get()
                .unwrap()
                .set_max_content_height((available - gap - chrome).max(1));
        }
    }
    fn highlight(&self, index: Option<usize>) {
        let items = self.imp().items.borrow().clone();
        let index = index.filter(|i| {
            items
                .get(*i)
                .is_some_and(|x| x.row.get_visible() && !x.option.disabled)
        });
        self.imp().active.set(index);
        for (i, item) in items.iter().enumerate() {
            if Some(i) == index {
                item.row.add_css_class("dz-highlighted");
            } else {
                item.row.remove_css_class("dz-highlighted");
            }
        }
        if let Some(index) = index {
            self.entry()
                .update_relation(&[gtk::accessible::Relation::ActiveDescendant(
                    items[index].row.upcast_ref(),
                )]);
            if self.is_open()
                && let Some(rect) = items[index].row.compute_bounds(&self.list())
            {
                let adjustment = self.imp().scroll.get().unwrap().vadjustment();
                let top = f64::from(rect.y());
                let bottom = top + f64::from(rect.height());
                if top < adjustment.value() {
                    adjustment.set_value(top);
                } else if bottom > adjustment.value() + adjustment.page_size() {
                    adjustment.set_value(bottom - adjustment.page_size());
                }
            }
        } else {
            self.entry()
                .reset_relation(gtk::AccessibleRelation::ActiveDescendant);
        }
    }
    fn choose(&self, index: usize) {
        let Some(item) = self.imp().items.borrow().get(index).cloned() else {
            return;
        };
        if !self.can_edit() || item.option.disabled || !item.row.get_visible() {
            return;
        }
        if self.request(Some(item.option.value.clone()), "item-press") {
            self.write_query(&item.option.label);
            self.close();
            self.entry().grab_focus();
            self.entry().set_position(-1);
        }
    }
    fn key(&self, key: gtk::gdk::Key, mods: gtk::gdk::ModifierType) -> glib::Propagation {
        use gtk::{
            gdk::{Key, ModifierType as M},
            glib::Propagation as P,
        };
        self.flush_input();
        if !self.can_edit()
            || mods.intersects(M::CONTROL_MASK | M::SHIFT_MASK | M::ALT_MASK | M::META_MASK)
        {
            return P::Proceed;
        }
        if key == Key::Escape {
            if self.is_open() {
                self.close();
                return P::Stop;
            }
            let value = if self.imp().autocomplete.get() {
                Some(String::new())
            } else {
                None
            };
            let had = self.value() != value;
            self.request(value, "escape-key");
            self.write_query("");
            self.filter();
            return if had { P::Stop } else { P::Proceed };
        }
        if key == Key::Tab || key == Key::ISO_Left_Tab {
            self.close();
            return P::Proceed;
        }
        if key == Key::Home {
            self.entry().set_position(0);
            return P::Stop;
        }
        if key == Key::End {
            self.entry().set_position(-1);
            return P::Stop;
        }
        if key == Key::Return || key == Key::KP_Enter {
            if self.is_open() {
                if let Some(index) = self.imp().active.get() {
                    self.choose(index);
                    return P::Stop;
                }
                self.close();
            }
            return P::Proceed;
        }
        if key == Key::Down || key == Key::Up {
            let was_open = self.is_open();
            self.open();
            if !self.is_open() {
                return P::Proceed;
            }
            let items = self.imp().items.borrow();
            let enabled: Vec<_> = items
                .iter()
                .enumerate()
                .filter(|(_, x)| x.row.get_visible() && !x.option.disabled)
                .map(|(i, _)| i)
                .collect();
            drop(items);
            let active = self
                .imp()
                .active
                .get()
                .and_then(|i| enabled.iter().position(|n| *n == i));
            let next = if !was_open && active.is_some() {
                self.imp().active.get()
            } else if key == Key::Down {
                active.map_or_else(|| enabled.first().copied(), |i| enabled.get(i + 1).copied())
            } else {
                active.map_or_else(
                    || enabled.last().copied(),
                    |i| i.checked_sub(1).map(|i| enabled[i]),
                )
            };
            self.highlight(next);
            return P::Stop;
        }
        P::Proceed
    }
    fn unwatch(&self) {
        if let Some((window, controller)) = self.imp().watch.borrow_mut().take()
            && let Some(window) = window.upgrade()
        {
            window.remove_controller(&controller);
        }
    }
    fn watch(&self) {
        self.unwatch();
        let Some(window) = self.root().and_downcast::<gtk::Window>() else {
            return;
        };
        let click = gtk::GestureClick::new();
        click.set_propagation_phase(gtk::PropagationPhase::Capture);
        click.set_button(0);
        let weak = self.downgrade();
        let weak_window = window.downgrade();
        click.connect_pressed(move |gesture, _, x, y| {
            if let Some(root) = weak.upgrade()
                && let Some(window) = weak_window.upgrade()
            {
                if gesture.current_event().and_then(|event| event.surface())
                    == root.popup().surface()
                {
                    return;
                }
                let inside = window.pick(x, y, gtk::PickFlags::DEFAULT).is_some_and(|w| {
                    w == root.clone().upcast::<gtk::Widget>() || w.is_ancestor(&root)
                });
                if !inside {
                    root.close();
                }
            }
        });
        window.add_controller(click.clone());
        self.imp().watch.replace(Some((window.downgrade(), click)));
    }
}
/// Free text suggestions share the same native input/list coordinator.
pub fn autocomplete(options: AutocompleteOptions) -> Combobox {
    let root = Combobox::build(
        true,
        options.size,
        options.tint,
        &options.placeholder,
        &options.empty,
    );
    root.imp().controlled.set(options.value.is_some());
    let items: Vec<_> = options
        .items
        .into_iter()
        .map(|value| {
            ComboboxItem::Option(ComboboxOption {
                label: value.clone(),
                value,
                disabled: false,
            })
        })
        .collect();
    root.set_options(&items);
    root.set_value(Some(
        options.value.as_deref().unwrap_or(&options.default_value),
    ));
    root.set_read_only(options.read_only);
    root.set_required(options.required);
    root.set_sensitive(!options.disabled);
    if options.mono {
        root.entry().add_css_class("dz-mono");
    }
    root
}

fn contains(item: &str, query: &str) -> bool {
    super::choice_filter::contains(item, query)
}
