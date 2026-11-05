//! Native tree models, expander gestures and list rows, with the shared
//! tree keyboard and host-controlled selection rather than a disclosure list.
use crate::{Theme, Tint, box_, prelude::*};
use gtk::{glib, subclass::prelude::*};
use std::{
    cell::{Cell, OnceCell, RefCell},
    collections::{HashMap, HashSet},
    time::Instant,
};

fn unique(ids: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    ids.into_iter()
        .filter(|id| seen.insert(id.clone()))
        .collect()
}
/// Rich content for one native tree row. IDs must be unique in the whole tree.
pub struct TreeNode {
    pub id: String,
    pub label: gtk::Widget,
    pub icon: Option<gtk::Widget>,
    pub meta: Option<gtk::Widget>,
    pub text: String,
    pub disabled: bool,
    /// Some(empty) is a branch, None a leaf.
    pub children: Option<Vec<TreeNode>>,
}
impl TreeNode {
    pub fn new(
        id: impl Into<String>,
        label: &impl IsA<gtk::Widget>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.as_ref().clone(),
            icon: None,
            meta: None,
            text: text.into(),
            disabled: false,
            children: None,
        }
    }
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TreeSize {
    Small,
    #[default]
    Medium,
}
#[derive(Default)]
pub struct TreeOptions {
    pub expanded: Option<Vec<String>>,
    pub default_expanded: Vec<String>,
    pub selected: Option<String>,
    pub size: TreeSize,
    pub tint: Tint,
}
pub struct NodeData {
    id: String,
    parent: Option<String>,
    index: usize,
    siblings: usize,
    text: RefCell<String>,
    disabled: Cell<bool>,
    children: Option<gtk::gio::ListStore>,
    row: gtk::ListBoxRow,
    expander: gtk::TreeExpander,
    bound: RefCell<Option<gtk::TreeListRow>>,
    notify: RefCell<Option<glib::SignalHandlerId>>,
}
#[derive(Default)]
pub struct Typed {
    prefix: String,
    at: Option<Instant>,
}
mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Node {
        pub data: OnceCell<NodeData>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Node {
        const NAME: &'static str = "DazzTreeNode";
        type Type = super::NodeObject;
    }
    impl ObjectImpl for Node {}
    #[derive(Default)]
    pub struct Tree {
        pub nodes: RefCell<HashMap<String, NodeObject>>,
        pub model: OnceCell<gtk::TreeListModel>,
        pub rows: OnceCell<gtk::ListBox>,
        pub open: RefCell<Vec<String>>,
        pub selected: RefCell<Option<String>>,
        pub active: RefCell<Option<String>>,
        pub controlled: Cell<bool>,
        pub applying: Cell<bool>,
        pub pending: Cell<bool>,
        pub disposed: Cell<bool>,
        pub typed: RefCell<Typed>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Tree {
        const NAME: &'static str = "DazzTree";
        type Type = super::Tree;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for Tree {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: std::sync::OnceLock<Vec<glib::subclass::Signal>> =
                std::sync::OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("selected-change")
                        .param_types([String::static_type()])
                        .build(),
                    glib::subclass::Signal::builder("expanded-change")
                        .param_types([glib::Variant::static_type()])
                        .build(),
                ]
            })
        }
        fn dispose(&self) {
            self.disposed.set(true);
            self.applying.set(true);
            if let Some(rows) = self.rows.get() {
                rows.bind_model(None::<&gtk::gio::ListModel>, |_| {
                    gtk::Box::new(gtk::Orientation::Horizontal, 0).upcast()
                });
            }
            for node in self.nodes.borrow().values() {
                node.clear_binding();
            }
            self.nodes.borrow_mut().clear();
        }
    }
    impl WidgetImpl for Tree {
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            let tree = self.obj();
            let focus = tree
                .root()
                .and_downcast::<gtk::Window>()
                .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
            if focus
                .as_ref()
                .is_some_and(|focus| focus.is_ancestor(tree.upcast_ref::<gtk::Widget>()))
            {
                let before = focus;
                let moved = tree.rows().child_focus(direction);
                let after = tree
                    .root()
                    .and_downcast::<gtk::Window>()
                    .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
                return moved && after != before;
            }
            let ids = tree.visible_ids();
            let active = tree
                .imp()
                .active
                .borrow()
                .clone()
                .filter(|id| ids.contains(id))
                .or_else(|| ids.first().cloned());
            active.is_some_and(|id| tree.focus_id(&id))
        }
        fn direction_changed(&self, previous: gtk::TextDirection) {
            self.parent_direction_changed(previous);
            if let Some(rows) = self.rows.get() {
                rows.set_direction(self.obj().direction());
            }
            for node in self.nodes.borrow().values() {
                node.data().expander.set_direction(self.obj().direction());
            }
        }
    }
    impl BoxImpl for Tree {}
}
glib::wrapper! {pub struct NodeObject(ObjectSubclass<imp::Node>);}
impl NodeObject {
    fn data(&self) -> &NodeData {
        self.imp().data.get().unwrap()
    }
    fn clear_binding(&self) {
        let Some(data) = self.imp().data.get() else {
            return;
        };
        if let Some(row) = data.bound.borrow_mut().take()
            && let Some(handler) = data.notify.borrow_mut().take()
        {
            row.disconnect(handler);
        }
        data.expander.set_list_row(None::<&gtk::TreeListRow>);
    }
}
glib::wrapper! {
    pub struct Tree(ObjectSubclass<imp::Tree>) @extends gtk::Widget,gtk::Box,
        @implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;
}
impl Tree {
    pub fn new(items: Vec<TreeNode>, options: TreeOptions) -> Result<Self, &'static str> {
        let tree: Self = glib::Object::new();
        tree.set_orientation(gtk::Orientation::Vertical);
        tree.add_css_class("dz-tree-content");
        tree.set_accessible_role(gtk::AccessibleRole::Tree);
        tree.set_dazz_tint(options.tint);
        tree.add_css_class(if options.size == TreeSize::Small {
            "dz-size-small"
        } else {
            "dz-size-medium"
        });
        let first = items.first().map(|n| n.id.clone());
        tree.imp()
            .active
            .replace(options.selected.clone().or(first));
        tree.imp().selected.replace(options.selected);
        tree.imp().controlled.set(options.expanded.is_some());
        tree.imp()
            .open
            .replace(unique(options.expanded.unwrap_or(options.default_expanded)));
        let root = tree.build_nodes(items, None)?;
        let model = gtk::TreeListModel::new(root, false, false, |item| {
            item.downcast_ref::<NodeObject>()
                .unwrap()
                .data()
                .children
                .as_ref()
                .map(|model| model.clone().upcast())
        });
        let rows = gtk::ListBox::new();
        rows.add_css_class("dz-tree-list");
        rows.set_selection_mode(gtk::SelectionMode::None);
        rows.set_activate_on_single_click(true);
        rows.set_accessible_role(gtk::AccessibleRole::Group);
        tree.imp().model.set(model.clone()).unwrap();
        tree.imp().rows.set(rows.clone()).unwrap();
        tree.append(&rows);
        let weak = tree.downgrade();
        rows.bind_model(Some(&model), move |item| {
            let row = item.downcast_ref::<gtk::TreeListRow>().unwrap();
            let node = row.item().and_downcast::<NodeObject>().unwrap();
            if let Some(tree) = weak.upgrade() {
                tree.bind_node(&node, row);
            }
            node.data().row.clone().upcast()
        });
        let weak = tree.downgrade();
        model.connect_items_changed(move |_, _, _, _| {
            if let Some(tree) = weak.upgrade() {
                tree.schedule_refresh();
            }
        });
        let weak = tree.downgrade();
        rows.connect_row_activated(move |_, row| {
            if let Some(tree) = weak.upgrade() {
                let id = tree
                    .imp()
                    .nodes
                    .borrow()
                    .values()
                    .find(|node| node.data().row == *row)
                    .map(|node| node.data().id.clone());
                if let Some(id) = id
                    && !tree.is_disabled(&id)
                {
                    tree.activate_id(&id);
                }
            }
        });
        tree.install_keyboard();
        tree.apply_expanded();
        Ok(tree)
    }
    fn build_nodes(
        &self,
        items: Vec<TreeNode>,
        parent: Option<String>,
    ) -> Result<gtk::gio::ListStore, &'static str> {
        let model = gtk::gio::ListStore::new::<NodeObject>();
        let siblings = items.len();
        for (index, node) in items.into_iter().enumerate() {
            if self.imp().nodes.borrow().contains_key(&node.id) {
                return Err("Tree ids must be unique");
            }
            let object: NodeObject = glib::Object::new();
            let row = gtk::ListBoxRow::new();
            row.add_css_class("dz-tree-node");
            row.set_selectable(false);
            row.set_accessible_role(gtk::AccessibleRole::TreeItem);
            let expander = gtk::TreeExpander::new();
            expander.add_css_class("dz-tree-expander");
            expander.set_focusable(false);
            expander.set_accessible_role(gtk::AccessibleRole::Group);
            expander.set_indent_for_icon(true);
            expander.set_indent_for_depth(true);
            if node.children.is_none() {
                expander.add_css_class("dz-leaf");
            }
            let content = box_(gtk::Orientation::Horizontal, "1");
            content.set_hexpand(true);
            if let Some(icon) = node.icon {
                let slot = box_(gtk::Orientation::Horizontal, "0");
                slot.add_css_class("dz-tree-icon");
                slot.set_valign(gtk::Align::Center);
                slot.append(&icon);
                content.append(&slot);
            }
            if let Some(label) = node.label.downcast_ref::<gtk::Label>() {
                label.set_wrap(false);
                label.set_ellipsize(gtk::pango::EllipsizeMode::End);
                label.set_width_chars(0);
            }
            let label = box_(gtk::Orientation::Horizontal, "0");
            label.add_css_class("dz-tree-label");
            label.set_hexpand(true);
            node.label.set_hexpand(true);
            label.append(&node.label);
            content.append(&label);
            if let Some(meta) = node.meta {
                let slot = box_(gtk::Orientation::Horizontal, "0");
                slot.add_css_class("dz-tree-meta");
                slot.set_valign(gtk::Align::Center);
                slot.append(&meta);
                content.append(&slot);
            }
            expander.set_child(Some(&content));
            row.set_child(Some(&expander));
            let id = node.id;
            let weak = self.downgrade();
            let focus_id = id.clone();
            let focus = gtk::EventControllerFocus::new();
            focus.connect_enter(move |_| {
                if let Some(tree) = weak.upgrade() {
                    tree.imp().active.replace(Some(focus_id.clone()));
                    tree.update_roving();
                }
            });
            row.add_controller(focus);
            self.imp()
                .nodes
                .borrow_mut()
                .insert(id.clone(), object.clone());
            let children = node
                .children
                .map(|children| self.build_nodes(children, Some(id.clone())))
                .transpose()?;
            object
                .imp()
                .data
                .set(NodeData {
                    id,
                    parent: parent.clone(),
                    index,
                    siblings,
                    text: RefCell::new(node.text),
                    disabled: Cell::new(node.disabled),
                    children,
                    row,
                    expander,
                    bound: RefCell::default(),
                    notify: RefCell::default(),
                })
                .ok()
                .unwrap();
            model.append(&object);
        }
        Ok(model)
    }
    fn bind_node(&self, node: &NodeObject, row: &gtk::TreeListRow) {
        node.clear_binding();
        let data = node.data();
        data.expander.set_list_row(Some(row));
        data.row
            .update_property(&[gtk::accessible::Property::Level(row.depth() as i32 + 1)]);
        data.row.update_relation(&[
            gtk::accessible::Relation::PosInSet(data.index as i32 + 1),
            gtk::accessible::Relation::SetSize(data.siblings as i32),
        ]);
        let weak = self.downgrade();
        let id = data.id.clone();
        let handler = row.connect_expanded_notify(move |row| {
            if let Some(tree) = weak.upgrade()
                && !tree.imp().applying.get()
                // GTK also notifies rows invalidated by an ancestor collapse.
                // Those are removals, not a user expansion request.
                && row.is_expandable()
            {
                if tree.is_disabled(&id) {
                    tree.apply_expanded();
                } else {
                    tree.request_expanded(&id, row.is_expanded());
                }
            }
        });
        data.bound.replace(Some(row.clone()));
        data.notify.replace(Some(handler));
        self.style_node(node);
    }
    pub fn rows(&self) -> gtk::ListBox {
        self.imp().rows.get().unwrap().clone()
    }
    pub fn model(&self) -> gtk::TreeListModel {
        self.imp().model.get().unwrap().clone()
    }
    pub fn row(&self, id: &str) -> Option<gtk::ListBoxRow> {
        self.imp()
            .nodes
            .borrow()
            .get(id)
            .map(|node| node.data().row.clone())
    }
    pub fn expander(&self, id: &str) -> Option<gtk::TreeExpander> {
        self.imp()
            .nodes
            .borrow()
            .get(id)
            .map(|node| node.data().expander.clone())
    }
    pub fn visible_ids(&self) -> Vec<String> {
        (0..self.model().n_items())
            .filter_map(|i| {
                self.model()
                    .row(i)?
                    .item()
                    .and_downcast::<NodeObject>()
                    .map(|node| node.data().id.clone())
            })
            .collect()
    }
    pub fn expanded(&self) -> Vec<String> {
        self.imp().open.borrow().clone()
    }
    pub fn selected(&self) -> Option<String> {
        self.imp().selected.borrow().clone()
    }
    pub fn set_selected(&self, selected: Option<&str>) {
        self.imp().selected.replace(selected.map(str::to_owned));
        self.refresh();
    }
    pub fn set_expanded(&self, expanded: &[String]) {
        self.imp().open.replace(unique(expanded.to_vec()));
        self.apply_expanded();
    }
    pub fn set_item_disabled(&self, id: &str, disabled: bool) {
        if let Some(node) = self.imp().nodes.borrow().get(id) {
            node.data().disabled.set(disabled);
            self.style_node(node);
        }
    }
    pub fn set_item_text(&self, id: &str, text: &str) {
        if let Some(node) = self.imp().nodes.borrow().get(id) {
            node.data().text.replace(text.to_owned());
        }
    }
    pub fn connect_selected_changed(
        &self,
        callback: impl Fn(&Self, &str) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("selected-change", false, move |v| {
            callback(&v[0].get::<Self>().unwrap(), &v[1].get::<String>().unwrap());
            None
        })
    }
    pub fn connect_expanded_changed(
        &self,
        callback: impl Fn(&Self, &[String]) + 'static,
    ) -> glib::SignalHandlerId {
        self.connect_local("expanded-change", false, move |v| {
            callback(
                &v[0].get::<Self>().unwrap(),
                &v[1]
                    .get::<glib::Variant>()
                    .unwrap()
                    .get::<Vec<String>>()
                    .unwrap(),
            );
            None
        })
    }
    fn schedule_refresh(&self) {
        if self.imp().pending.replace(true) {
            return;
        }
        let weak = self.downgrade();
        glib::idle_add_local_once(move || {
            if let Some(tree) = weak.upgrade()
                && !tree.imp().disposed.get()
            {
                tree.apply_expanded();
                tree.imp().pending.set(false);
            }
        });
    }
    fn apply_expanded(&self) {
        if self.imp().disposed.get() {
            return;
        }
        let before = self.imp().applying.replace(true);
        let wanted = self.expanded();
        let mut i = 0;
        while i < self.model().n_items() {
            let row = self.model().row(i).unwrap();
            let node = row.item().and_downcast::<NodeObject>().unwrap();
            let expanded = node.data().children.is_some() && wanted.contains(&node.data().id);
            if row.is_expanded() != expanded {
                row.set_expanded(expanded);
            }
            i += 1;
        }
        self.imp().applying.set(before);
        self.refresh();
    }
    fn refresh(&self) {
        if self.imp().disposed.get() {
            return;
        }
        let visible = self.visible_ids();
        for node in self.imp().nodes.borrow().values() {
            if !visible.contains(&node.data().id) {
                node.clear_binding();
            }
            self.style_node(node);
        }
        self.update_roving();
    }
    fn style_node(&self, node: &NodeObject) {
        let data = node.data();
        let selected = self.imp().selected.borrow().as_deref() == Some(&data.id);
        if selected {
            data.row.add_css_class("dz-selected");
        } else {
            data.row.remove_css_class("dz-selected");
        }
        if data.disabled.get() {
            data.row.add_css_class("dz-disabled");
        } else {
            data.row.remove_css_class("dz-disabled");
        }
        data.row.set_activatable(!data.disabled.get());
        data.row.set_cursor_from_name(Some(if data.disabled.get() {
            "not-allowed"
        } else {
            "pointer"
        }));
        data.row.update_state(&[
            gtk::accessible::State::Selected(Some(selected)),
            gtk::accessible::State::Disabled(data.disabled.get()),
            gtk::accessible::State::Expanded(
                data.children
                    .as_ref()
                    .map(|_| self.imp().open.borrow().contains(&data.id)),
            ),
        ]);
    }
    fn update_roving(&self) {
        let ids = self.visible_ids();
        let active = self
            .imp()
            .active
            .borrow()
            .clone()
            .filter(|id| ids.contains(id))
            .or_else(|| ids.first().cloned());
        for node in self.imp().nodes.borrow().values() {
            node.data()
                .row
                .set_focusable(active.as_deref() == Some(&node.data().id));
        }
    }
    pub fn focus_id(&self, id: &str) -> bool {
        if !self.visible_ids().iter().any(|candidate| candidate == id) {
            return false;
        }
        self.imp().active.replace(Some(id.to_owned()));
        let Some(row) = self.row(id) else {
            return false;
        };
        row.set_focusable(true);
        let result = row.grab_focus();
        self.update_roving();
        result
    }
    fn is_disabled(&self, id: &str) -> bool {
        self.imp()
            .nodes
            .borrow()
            .get(id)
            .is_some_and(|node| node.data().disabled.get())
    }
    fn request_expanded(&self, id: &str, next: bool) {
        let mut expanded = self.expanded();
        if next {
            if !expanded.iter().any(|value| value == id) {
                expanded.push(id.to_owned());
            }
        } else {
            expanded.retain(|value| value != id);
        }
        if !self.imp().controlled.get() {
            self.imp().open.replace(expanded.clone());
        }
        self.apply_expanded();
        self.emit_by_name::<()>("expanded-change", &[&expanded.to_variant()]);
    }
    fn activate_id(&self, id: &str) {
        self.focus_id(id);
        if !self.is_disabled(id) {
            self.emit_by_name::<()>("selected-change", &[&id]);
        }
        let branch = self
            .imp()
            .nodes
            .borrow()
            .get(id)
            .is_some_and(|node| node.data().children.is_some());
        if branch {
            let next = !self.imp().open.borrow().iter().any(|value| value == id);
            self.request_expanded(id, next);
        }
    }
    fn install_keyboard(&self) {
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = self.downgrade();
        keys.connect_key_pressed(move |_, key, _, mods| {
            use gtk::{
                gdk::{Key, ModifierType as M},
                glib::Propagation,
            };
            let Some(tree) = weak.upgrade() else {
                return Propagation::Proceed;
            };
            let ids = tree.visible_ids();
            let Some(id) = tree
                .imp()
                .active
                .borrow()
                .clone()
                .filter(|id| ids.contains(id))
                .or_else(|| ids.first().cloned())
            else {
                return Propagation::Proceed;
            };
            let here = ids.iter().position(|item| item == &id).unwrap();
            let node = tree.imp().nodes.borrow().get(&id).cloned().unwrap();
            let branch = node.data().children.is_some();
            let open = tree.imp().open.borrow().contains(&id);
            if key == Key::Down || key == Key::Up {
                let next = if key == Key::Down {
                    here.checked_add(1)
                } else {
                    here.checked_sub(1)
                };
                if let Some(next) = next.and_then(|i| ids.get(i))
                    && !tree.is_disabled(next)
                {
                    tree.focus_id(next);
                }
            } else if key == Key::Right {
                if branch && !open {
                    tree.request_expanded(&id, true);
                } else if open
                    && let Some(next) = ids.get(here + 1)
                    && !tree.is_disabled(next)
                {
                    tree.focus_id(next);
                }
            } else if key == Key::Left {
                if branch && open {
                    tree.request_expanded(&id, false);
                } else if let Some(parent) = &node.data().parent {
                    tree.focus_id(parent);
                }
            } else if key == Key::Home {
                tree.focus_id(&ids[0]);
            } else if key == Key::End {
                tree.focus_id(ids.last().unwrap());
            } else if key == Key::Return || key == Key::KP_Enter || key == Key::space {
                tree.activate_id(&id);
            } else {
                if mods.intersects(M::CONTROL_MASK | M::META_MASK | M::SUPER_MASK) {
                    return Propagation::Proceed;
                }
                let Some(character) = key
                    .to_unicode()
                    .filter(|c| !c.is_control() && c.len_utf16() == 1)
                else {
                    return Propagation::Proceed;
                };
                let prefix = {
                    let mut typed = tree.imp().typed.borrow_mut();
                    let duration = Theme::of(&tree)
                        .unwrap_or_default()
                        .duration("tree-typeahead-window")
                        .unwrap();
                    if !typed.at.is_some_and(|at| at.elapsed() < duration) {
                        typed.prefix.clear();
                    }
                    typed.prefix.extend(character.to_lowercase());
                    typed.at = Some(Instant::now());
                    typed.prefix.clone()
                };
                let target = (1..=ids.len())
                    .map(|by| &ids[(here + by) % ids.len()])
                    .find(|id| {
                        !tree.is_disabled(id)
                            && tree.imp().nodes.borrow().get(*id).is_some_and(|node| {
                                node.data()
                                    .text
                                    .borrow()
                                    .trim()
                                    .to_lowercase()
                                    .starts_with(&prefix)
                            })
                    })
                    .cloned();
                if let Some(target) = target {
                    tree.focus_id(&target);
                }
            }
            Propagation::Stop
        });
        self.add_controller(keys);
    }
}
pub fn tree_content(items: Vec<TreeNode>, options: TreeOptions) -> Result<Tree, &'static str> {
    Tree::new(items, options)
}
