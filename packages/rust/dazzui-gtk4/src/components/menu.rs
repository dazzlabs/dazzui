//! Menu models keep GTK's focus, activation, radio state and dismissal. The
//! row slots change only their drawing and never register hint accelerators.
use crate::{Theme, Tint, prelude::*};
use gtk::glib;
use std::{cell::RefCell, rc::Rc, time::Instant};

pub fn menu_rows(root: &impl IsA<gtk::Widget>) -> Vec<gtk::Widget> {
    fn visit(w: &gtk::Widget, out: &mut Vec<gtk::Widget>) {
        if w.css_name() == "modelbutton" || w.has_css_class("model") {
            out.push(w.clone());
            return;
        }
        let mut child = w.first_child();
        while let Some(w) = child {
            child = w.next_sibling();
            visit(&w, out);
        }
    }
    let mut out = vec![];
    visit(root.as_ref(), &mut out);
    out
}

fn row_label(row: &gtk::Widget) -> Option<gtk::Label> {
    let mut child = row.first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        if w.css_name() == "label"
            && let Some(label) = w.downcast_ref::<gtk::Label>()
        {
            return Some(label.clone());
        }
    }
    None
}
fn style_row(row: &gtk::Widget) {
    if row.has_css_class("dz-menu-row") {
        refresh_row(row);
        return;
    }
    row.add_css_class("dz-menu-row");
    if let Some(label) = row_label(row) {
        label.add_css_class("dz-menu-label");
        label.set_hexpand(true);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        label.set_width_chars(0);
    }
    refresh_row(row);
    row.connect_notify_local(Some("role"), |row, _| refresh_row(row));
    row.connect_notify_local(Some("icon"), |row, _| refresh_row(row));
    // Space on a GTK radio row normally keeps the panel open. DazzUI's
    // checkable menu items close after selection, including keyboard use.
    row.connect_closure(
        "clicked",
        false,
        glib::closure_local!(|row: gtk::Widget| {
            let submenu = row.property::<Option<String>>("menu-name").is_some();
            let nested = row.property::<Option<gtk::Widget>>("popover").is_some();
            if !submenu
                && !nested
                && let Some(popup) = row
                    .ancestor(gtk::Popover::static_type())
                    .and_downcast::<gtk::Popover>()
            {
                popup.popdown();
            }
        }),
    );
}
fn refresh_row(row: &gtk::Widget) {
    // The empty native indicator box reserves a column across the model.
    // React reserves that column only on checkable rows. Its children may
    // arrive after the action group is attached, so refresh on role changes.
    let mut child = row.first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        if w.is::<gtk::Box>()
            && !w.has_css_class("dz-menu-icon")
            && !w.has_css_class("dz-menu-shortcut")
        {
            w.set_visible(w.first_child().is_some());
        }
        if let Some(image) = w.downcast_ref::<gtk::Image>() {
            image.set_visible(image.gicon().is_some() || image.icon_name().is_some());
        }
    }
}

/// Add presentation slots to a native model row. Its native label, gesture
/// controllers, action tracker and keyboard implementation remain intact.
pub fn menu_row_content(
    row: &gtk::Widget,
    leading: Option<&gtk::Widget>,
    trailing: Option<&gtk::Widget>,
) {
    style_row(row);
    if let Some(label) = row_label(row)
        && let Some(leading) = leading
    {
        leading.add_css_class("dz-menu-icon");
        leading.set_can_target(false);
        leading.set_can_focus(false);
        leading.insert_before(row, Some(&label));
        detach_on_destroy(row, leading);
    }
    if let Some(trailing) = trailing {
        trailing.add_css_class("dz-menu-shortcut");
        trailing.set_can_target(false);
        trailing.set_can_focus(false);
        trailing.set_parent(row);
        detach_on_destroy(row, trailing);
    }
}
fn detach_on_destroy(row: &gtk::Widget, child: &gtk::Widget) {
    let weak = child.downgrade();
    row.connect_destroy(move |row| {
        if let Some(child) = weak.upgrade()
            && child.parent().as_ref() == Some(row)
        {
            child.unparent();
        }
    });
}

pub fn menu_popup(model: &impl IsA<gtk::gio::MenuModel>) -> gtk::PopoverMenu {
    let popup = gtk::PopoverMenu::from_model(Some(model));
    style_menu(&popup);
    let weak = popup.downgrade();
    model.connect_items_changed(move |_, _, _, _| {
        if let Some(popup) = weak.upgrade() {
            for row in menu_rows(&popup) {
                style_row(&row);
            }
        }
    });
    popup
}

pub fn menubar(model: &impl IsA<gtk::gio::MenuModel>) -> gtk::PopoverMenuBar {
    let bar = gtk::PopoverMenuBar::from_model(Some(model));
    super::menubar_layout::install(&bar);
    bar.add_css_class("dz-menubar");
    bar.set_dazz_tint(Tint::Neutral);
    fn style(w: &gtk::Widget) {
        if let Some(popup) = w.downcast_ref::<gtk::PopoverMenu>() {
            style_menu(popup);
            return;
        }
        let mut child = w.first_child();
        while let Some(w) = child {
            child = w.next_sibling();
            style(&w);
        }
    }
    style(bar.upcast_ref());
    bar.connect_map(|bar| style(bar.upcast_ref()));
    let weak = bar.downgrade();
    model.connect_items_changed(move |_, _, _, _| {
        if let Some(bar) = weak.upgrade() {
            super::menubar_layout::install(&bar);
            style(bar.upcast_ref());
        }
    });
    bar
}
fn style_menu(popup: &gtk::PopoverMenu) {
    if popup.has_css_class("dz-menu") {
        return;
    }
    popup.add_css_class("dz-menu");
    popup.add_css_class("dz-popover");
    popup.set_has_arrow(false);
    popup.set_dazz_tint(Tint::Neutral);
    for row in menu_rows(popup) {
        style_row(&row);
    }
    popup.connect_map(|popup| {
        // GTK initially focuses a row even for a pointer open. Base UI waits
        // for pointer motion or navigation before painting a highlight.
        if popup
            .root()
            .and_downcast::<gtk::Window>()
            .is_some_and(|w| w.gets_focus_visible())
        {
            popup.remove_css_class("dz-menu-initial");
        } else {
            popup.add_css_class("dz-menu-initial");
        }
        for row in menu_rows(popup) {
            style_row(&row);
        }
    });
    let motion = gtk::EventControllerMotion::new();
    let weak = popup.downgrade();
    motion.connect_motion(move |_, _, _| {
        if let Some(popup) = weak.upgrade() {
            popup.remove_css_class("dz-menu-initial");
        }
    });
    popup.add_controller(motion);
    typeahead(popup);
}

fn typeahead(popup: &gtk::PopoverMenu) {
    #[derive(Default)]
    struct Search {
        text: String,
        at: Option<Instant>,
        origin: isize,
        matched: Option<usize>,
    }
    let state = Rc::new(RefCell::new(Search::default()));
    let clear = state.clone();
    popup.connect_unmap(move |_| *clear.borrow_mut() = Search::default());
    let keys = gtk::EventControllerKey::new();
    keys.set_propagation_phase(gtk::PropagationPhase::Capture);
    let weak = popup.downgrade();
    keys.connect_key_pressed(move |_, key, _, mods| {
        use gtk::{
            gdk::{Key, ModifierType as M},
            glib::Propagation,
        };
        if mods.intersects(M::CONTROL_MASK | M::ALT_MASK | M::SUPER_MASK | M::META_MASK) {
            return Propagation::Proceed;
        }
        let Some(popup) = weak.upgrade() else {
            return Propagation::Proceed;
        };
        let initial = popup.has_css_class("dz-menu-initial");
        if initial && [Key::Return, Key::KP_Enter, Key::space].contains(&key) {
            return Propagation::Stop;
        }
        popup.remove_css_class("dz-menu-initial");
        let rows: Vec<_> = menu_rows(&popup)
            .into_iter()
            .filter(|w| w.is_mapped() && w.is_sensitive())
            .collect();
        if rows.is_empty() {
            return Propagation::Proceed;
        }
        if key == Key::Home || key == Key::End || (initial && (key == Key::Down || key == Key::Up))
        {
            *state.borrow_mut() = Search::default();
            rows[if key == Key::Home || key == Key::Down {
                0
            } else {
                rows.len() - 1
            }]
            .grab_focus();
            return Propagation::Stop;
        }
        let Some(ch) = key.to_unicode().filter(|c| !c.is_control()) else {
            return Propagation::Proceed;
        };
        let focus = popup
            .root()
            .and_downcast::<gtk::Window>()
            .and_then(|w| gtk::prelude::GtkWindowExt::focus(&w));
        let current = (!initial)
            .then(|| rows.iter().position(|w| focus.as_ref() == Some(w)))
            .flatten();
        let labels: Vec<_> = rows
            .iter()
            .map(|w| {
                row_label(w)
                    .map(|l| l.text().to_string())
                    .unwrap_or_default()
            })
            .collect();
        let matched = {
            let mut search = state.borrow_mut();
            let delay = Theme::of(&popup)
                .unwrap_or_default()
                .duration("menu-typeahead-reset-duration")
                .unwrap();
            if search.at.is_some_and(|at| at.elapsed() > delay) {
                search.text.clear();
            }
            if search.text.is_empty() {
                if ch == ' ' {
                    return Propagation::Proceed;
                }
                search.origin = current.map(|i| i as isize).unwrap_or(-1);
            }
            let letter = ch.to_lowercase().to_string();
            let rapid = !labels
                .iter()
                .any(|s| s.to_lowercase().starts_with(&letter.repeat(2)));
            if rapid && search.text.to_lowercase() == letter {
                search.text.clear();
                search.origin = search.matched.map(|i| i as isize).unwrap_or(-1);
            }
            search.text.push(ch);
            search.at = Some(Instant::now());
            let prefix = search.text.to_lowercase();
            let start = (search.origin + 1).max(0) as usize;
            let found = (0..rows.len())
                .map(|i| (start + i) % rows.len())
                .find(|&i| labels[i].to_lowercase().starts_with(&prefix));
            if let Some(found) = found {
                search.matched = Some(found);
            } else {
                search.text.clear();
            }
            found
        };
        if let Some(i) = matched {
            rows[i].grab_focus();
        }
        Propagation::Stop
    });
    popup.add_controller(keys);
}
