//! Compositions of native controls. GTK retains focus, activation and state;
//! these groups add only coordination and the shared ds drawing.
use crate::{Size, Tint, avatar, box_, icon, styled};
use gtk::prelude::*;

pub fn avatar_group(size: Size) -> gtk::Box {
    styled(
        box_(gtk::Orientation::Horizontal, "0"),
        "dz-avatar-group",
        Tint::Neutral,
        size,
    )
}

/// A count at the end of an AvatarGroup. Hidden avatars need not be created.
pub fn avatar_overflow(count: usize, size: Size) -> gtk::Label {
    let w = avatar(&format!("+{count}"), size);
    w.add_css_class("dz-avatar-rest");
    w.set_accessible_role(gtk::AccessibleRole::Presentation);
    w
}

pub fn accordion(multiple: bool, divided: bool) -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "0");
    w.add_css_class("dz-accordion");
    group_keyboard(&w, true, false);
    w.set_accessible_role(gtk::AccessibleRole::Group);
    if multiple {
        w.add_css_class("dz-accordion-multiple");
    }
    if divided {
        w.add_css_class("dz-accordion-divided");
    }
    w
}

pub fn accordion_append(group: &gtk::Box, item: &gtk::Expander) {
    group.append(item);
    let weak = group.downgrade();
    item.connect_expanded_notify(move |item| {
        if !item.is_expanded() {
            return;
        }
        if let Some(group) = weak.upgrade()
            && !group.has_css_class("dz-accordion-multiple")
        {
            let mut sibling = group.first_child();
            while let Some(w) = sibling {
                sibling = w.next_sibling();
                if let Some(other) = w.downcast_ref::<gtk::Expander>()
                    && other != item
                {
                    other.set_expanded(false);
                }
            }
        }
    });
}

pub fn toggle_group(multiple: bool) -> gtk::Box {
    let w = box_(gtk::Orientation::Horizontal, "0");
    w.add_css_class("dz-toggle-group");
    group_keyboard(&w, false, true);
    w.set_accessible_role(gtk::AccessibleRole::Group);
    if multiple {
        w.add_css_class("dz-toggle-group-multiple");
    }
    w
}

/// Appending does not put toggles in GTK's radio group: a single-selection
/// ToggleGroup may also be empty, unlike a radio group.
pub fn toggle_group_append(group: &gtk::Box, item: &gtk::ToggleButton) {
    group.append(item);
    let weak = group.downgrade();
    item.connect_toggled(move |item| {
        if !item.is_active() {
            return;
        }
        if let Some(group) = weak.upgrade()
            && !group.has_css_class("dz-toggle-group-multiple")
        {
            let mut sibling = group.first_child();
            while let Some(w) = sibling {
                sibling = w.next_sibling();
                if let Some(other) = w.downcast_ref::<gtk::ToggleButton>()
                    && other != item
                {
                    other.set_active(false);
                }
            }
        }
    });
    let weak = group.downgrade();
    item.connect_toggled(move |_| {
        if let Some(group) = weak.upgrade() {
            roving(&group);
        }
    });
    let focus = gtk::GestureClick::new();
    focus.set_propagation_phase(gtk::PropagationPhase::Capture);
    let weak = group.downgrade();
    let item_ref = item.downgrade();
    focus.connect_pressed(move |_, _, _, _| {
        if let (Some(group), Some(item)) = (weak.upgrade(), item_ref.upgrade()) {
            for child in group_children(&group) {
                child.set_focusable(child == item);
            }
            item.grab_focus();
        }
    });
    item.add_controller(focus);
    let weak = group.downgrade();
    item.connect_sensitive_notify(move |_| {
        if let Some(group) = weak.upgrade() {
            roving(&group);
        }
    });
    let weak = group.downgrade();
    item.connect_map(move |_| {
        if let Some(group) = weak.upgrade() {
            roving(&group);
        }
    });
    let weak = group.downgrade();
    item.connect_unmap(move |_| {
        let weak = weak.clone();
        gtk::glib::idle_add_local_once(move || {
            if let Some(group) = weak.upgrade() {
                roving(&group);
            }
        });
    });
    roving(group);
}

pub fn step_list() -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "2");
    w.add_css_class("dz-step-list");
    w.set_accessible_role(gtk::AccessibleRole::List);
    w
}
#[derive(Clone, Copy, Debug)]
pub enum StepStatus {
    Done,
    Active,
    Idle,
}
pub fn step(text: &str, meta: Option<&str>, status: StepStatus, tint: Tint) -> gtk::Box {
    let title = gtk::Label::new(Some(text));
    title.set_xalign(0.);
    let meta = meta.map(|text| gtk::Label::new(Some(text)));
    step_with_slots(
        title.upcast_ref(),
        meta.as_ref().map(|w| w.upcast_ref()),
        status,
        tint,
    )
}
pub fn step_with_slots(
    title: &gtk::Widget,
    meta: Option<&gtk::Widget>,
    status: StepStatus,
    tint: Tint,
) -> gtk::Box {
    let w = styled(
        box_(gtk::Orientation::Horizontal, "2-5"),
        "dz-step",
        tint,
        Size::Medium,
    );
    w.set_accessible_role(gtk::AccessibleRole::ListItem);
    w.add_css_class(match status {
        StepStatus::Done => "dz-step-done",
        StepStatus::Active => "dz-step-active",
        StepStatus::Idle => "dz-step-idle",
    });
    if matches!(status, StepStatus::Active) {
        let spinner = crate::spinner(Size::Medium);
        spinner.set_valign(gtk::Align::Center);
        crate::DazzWidgetExt::set_dazz_tint(&spinner, tint);
        w.append(&spinner);
    } else {
        let marker = gtk::CenterBox::new();
        marker.add_css_class("dz-step-marker");
        marker.set_valign(gtk::Align::Center);
        if matches!(status, StepStatus::Done) {
            marker.set_center_widget(Some(&icon("Checkmark12Filled")));
        }
        w.append(&marker);
    }
    title.set_hexpand(true);
    title.set_valign(gtk::Align::Center);
    w.append(title);
    if let Some(meta) = meta {
        let slot = box_(gtk::Orientation::Horizontal, "0");
        slot.add_css_class("dz-step-meta");
        slot.set_valign(gtk::Align::Center);
        slot.append(meta);
        w.append(&slot);
    }
    super::check_layout::install_row(&w, "2-5");
    w
}

fn group_children(group: &impl IsA<gtk::Widget>) -> Vec<gtk::Widget> {
    let group = group.as_ref();
    let mut children = vec![];
    let mut next = group.first_child();
    while let Some(child) = next {
        next = child.next_sibling();
        if child.is_visible() && child.is_sensitive() {
            children.push(child);
        }
    }
    children
}
fn focused(widget: &gtk::Widget) -> bool {
    widget.has_focus()
        || widget
            .root()
            .and_downcast::<gtk::Window>()
            .and_then(|window| gtk::prelude::GtkWindowExt::focus(&window))
            .as_ref()
            == Some(widget)
}
fn roving(group: &impl IsA<gtk::Widget>) {
    let children = group_children(group);
    let active = children
        .iter()
        .position(focused)
        .or_else(|| {
            children.iter().position(|w| {
                w.downcast_ref::<gtk::ToggleButton>()
                    .is_some_and(|w| w.is_active())
            })
        })
        .unwrap_or(0);
    for (index, child) in children.iter().enumerate() {
        child.set_focusable(index == active);
    }
}
pub(super) fn group_keyboard(group: &impl IsA<gtk::Widget>, vertical: bool, rove: bool) {
    let group = group.as_ref();
    let keys = gtk::EventControllerKey::new();
    keys.set_propagation_phase(gtk::PropagationPhase::Capture);
    let weak = group.downgrade();
    keys.connect_key_pressed(move |_, key, _, modifiers| {
        use gtk::{gdk::Key, glib::Propagation};
        if modifiers.intersects(
            gtk::gdk::ModifierType::CONTROL_MASK
                | gtk::gdk::ModifierType::ALT_MASK
                | gtk::gdk::ModifierType::SUPER_MASK
                | gtk::gdk::ModifierType::META_MASK,
        ) {
            return Propagation::Proceed;
        }
        let Some(group) = weak.upgrade() else {
            return Propagation::Proceed;
        };
        let items = group_children(&group);
        let Some(current) = items.iter().position(focused) else {
            return Propagation::Proceed;
        };
        let forward = if vertical {
            Key::Down
        } else if group.direction() == gtk::TextDirection::Rtl {
            Key::Left
        } else {
            Key::Right
        };
        let backward = if vertical {
            Key::Up
        } else if group.direction() == gtk::TextDirection::Rtl {
            Key::Right
        } else {
            Key::Left
        };
        let next = if key == forward {
            (current + 1) % items.len()
        } else if key == backward {
            (current + items.len() - 1) % items.len()
        } else if key == Key::Home {
            0
        } else if key == Key::End {
            items.len() - 1
        } else {
            return Propagation::Proceed;
        };
        if rove {
            for (i, item) in items.iter().enumerate() {
                item.set_focusable(i == next);
            }
        }
        items[next].grab_focus();
        Propagation::Stop
    });
    group.add_controller(keys);
    if rove {
        group.connect_map(roving);
        group.connect_sensitive_notify(roving);
    }
}
