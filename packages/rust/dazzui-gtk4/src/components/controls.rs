use crate::{DazzWidgetExt, Size, Tint, Variant, styled};
use gtk::prelude::*;

pub fn button(label: &str, variant: Variant, tint: Tint, size: Size) -> gtk::Button {
    let w = styled(gtk::Button::with_label(label), "dz-button", tint, size);
    w.set_dazz_variant(variant);
    w
}
/// A labelled button with a host icon-theme glyph at either end, or both.
///
/// As in React, a glyph beside a label is drawn `control.label-glyph.shrink`
/// under the profile's icon size, sits the profile's gap from the label, and
/// the end it sits at pads `control.label-glyph.inset` less. With no glyph
/// this is [`button`]; a glyph with no label is an [`icon_button`].
pub fn glyph_button(
    label: &str,
    leading: Option<&str>,
    trailing: Option<&str>,
    variant: Variant,
    tint: Tint,
    size: Size,
) -> gtk::Button {
    let w = button(label, variant, tint, size);
    if leading.is_none() && trailing.is_none() {
        return w;
    }
    let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    content.add_css_class("dz-button-content");
    let glyph = |icon: &str| {
        let image = gtk::Image::from_icon_name(icon);
        image.add_css_class("dz-button-glyph");
        image
    };
    if let Some(icon) = leading {
        content.append(&glyph(icon));
        w.add_css_class("dz-glyph-leading");
    }
    content.append(&gtk::Label::new(Some(label)));
    if let Some(icon) = trailing {
        content.append(&glyph(icon));
        w.add_css_class("dz-glyph-trailing");
    }
    w.set_child(Some(&content));
    w.set_dazz_label(label);
    w
}
/// Icons are supplied by the host's icon theme; give the control an accessible label.
pub fn icon_button(
    icon: &str,
    label: &str,
    variant: Variant,
    tint: Tint,
    size: Size,
) -> gtk::Button {
    let w = button("", variant, tint, size);
    w.add_css_class("dz-icon-button");
    w.set_child(Some(&gtk::Image::from_icon_name(icon)));
    w.set_dazz_label(label);
    w.set_tooltip_text(Some(label));
    w
}
pub fn toggle(label: &str, tint: Tint, size: Size) -> gtk::ToggleButton {
    let w = styled(
        gtk::ToggleButton::with_label(label),
        "dz-button",
        tint,
        size,
    );
    w.add_css_class("dz-toggle");
    w.set_dazz_variant(Variant::Normal);
    w
}
pub fn checkbox(label: &str, tint: Tint, size: Size) -> gtk::CheckButton {
    styled(
        gtk::CheckButton::with_label(label),
        "dz-checkbox",
        tint,
        size,
    )
}
pub fn radio(
    label: &str,
    group: Option<&gtk::CheckButton>,
    tint: Tint,
    size: Size,
) -> gtk::CheckButton {
    let w = checkbox(label, tint, size);
    w.remove_css_class("dz-checkbox");
    w.add_css_class("dz-radio");
    if let Some(group) = group {
        w.set_group(Some(group));
    }
    // An ungrouped first item still needs GTK's radio CSS node and semantics.
    else {
        let anchor = gtk::CheckButton::new();
        w.set_group(Some(&anchor));
        // Keep the anchor alive for the lifetime of the first member.
        w.connect_destroy(move |_| {
            let _ = &anchor;
        });
    }
    w
}
pub fn switch(tint: Tint, size: Size) -> gtk::Switch {
    let w = styled(gtk::Switch::new(), "dz-switch", tint, size);
    w.set_halign(gtk::Align::Start);
    w.set_valign(gtk::Align::Center);
    w
}
pub fn text_field(placeholder: &str, size: Size) -> gtk::Entry {
    let w = styled(gtk::Entry::new(), "dz-field", Tint::Primary, size);
    w.set_placeholder_text(Some(placeholder));
    w
}
pub fn password_field(size: Size) -> gtk::PasswordEntry {
    let w = styled(gtk::PasswordEntry::new(), "dz-field", Tint::Primary, size);
    w.set_show_peek_icon(true);
    w
}
pub fn text_area() -> gtk::TextView {
    let w = styled(
        gtk::TextView::new(),
        "dz-text-area",
        Tint::Primary,
        Size::Medium,
    );
    w.set_wrap_mode(gtk::WrapMode::WordChar);
    w
}
pub fn search_field(placeholder: &str, size: Size) -> gtk::SearchEntry {
    let w = styled(gtk::SearchEntry::new(), "dz-field", Tint::Primary, size);
    w.add_css_class("dz-search-field");
    w.set_placeholder_text(Some(placeholder));
    w
}
pub fn number_field(min: f64, max: f64, step: f64, size: Size) -> gtk::SpinButton {
    styled(
        gtk::SpinButton::with_range(min, max, step),
        "dz-field",
        Tint::Primary,
        size,
    )
}
pub fn select(labels: &[&str], size: Size) -> gtk::DropDown {
    styled(
        gtk::DropDown::from_strings(labels),
        "dz-select",
        Tint::Primary,
        size,
    )
}
#[derive(Clone, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}
#[derive(Clone, Debug)]
pub enum SelectItem {
    Option(SelectOption),
    Group {
        label: String,
        options: Vec<SelectOption>,
    },
}
/// A native selector with non-selectable group headings and disabled options.
/// Index zero represents no value; headings also occupy native model indices.
/// Use [`select_values`] to map a selected position to its domain value.
pub fn select_options(items: &[SelectItem], placeholder: &str, size: Size) -> gtk::DropDown {
    use std::{cell::Cell, rc::Rc};
    // GTK's selection auto-selects its first row. A hidden, non-activatable
    // empty row gives that position the same meaning as React's null value.
    let mut rows = vec![(placeholder.to_owned(), false, false)];
    for item in items {
        match item {
            SelectItem::Option(option) => rows.push((option.label.clone(), false, option.disabled)),
            SelectItem::Group { label, options } => {
                rows.push((label.clone(), true, true));
                rows.extend(
                    options
                        .iter()
                        .map(|option| (option.label.clone(), false, option.disabled)),
                );
            }
        }
    }
    let rows = Rc::new(rows);
    let labels: Vec<_> = rows.iter().map(|row| row.0.as_str()).collect();
    let w = select(&labels, size);
    let factory = |popup: bool| {
        let factory = gtk::SignalListItemFactory::new();
        let setup_placeholder = placeholder.to_owned();
        factory.connect_setup(move |_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let label = gtk::Label::new(Some(&setup_placeholder));
            label.set_xalign(0.);
            item.set_child(Some(&label));
        });
        let rows = rows.clone();
        let placeholder = placeholder.to_owned();
        factory.connect_bind(move |_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let label = item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let row = rows.get(item.position() as usize);
            let empty = item.position() == 0;
            label.set_label(row.map(|row| row.0.as_str()).unwrap_or(&placeholder));
            let heading = popup && row.is_some_and(|row| row.1);
            if heading {
                label.add_css_class("dz-select-group");
            } else {
                label.remove_css_class("dz-select-group");
            }
            if !popup && empty {
                label.add_css_class("dz-select-placeholder");
            } else {
                label.remove_css_class("dz-select-placeholder");
            }
            if popup && let Some(parent) = label.parent() {
                parent.set_visible(!empty);
            }
            let enabled = !row.is_some_and(|row| row.2);
            item.set_selectable(enabled && !empty);
            item.set_activatable(enabled && !empty);
            label.set_sensitive(enabled || heading);
        });
        factory
    };
    w.set_factory(Some(&factory(false)));
    w.set_list_factory(Some(&factory(true)));
    w.set_selected(0);
    w.update_property(&[gtk::accessible::Property::ValueText("")]);
    let previous = Cell::new(0);
    let correcting = Cell::new(false);
    w.connect_selected_notify(move |w| {
        if correcting.get() {
            return;
        }
        let selected = w.selected();
        if rows.get(selected as usize).is_some_and(|row| row.2) {
            let forward =
                previous.get() == gtk::INVALID_LIST_POSITION || selected >= previous.get();
            let positions: Box<dyn Iterator<Item = usize>> = if forward {
                Box::new(selected as usize..rows.len())
            } else {
                Box::new((0..=selected as usize).rev())
            };
            let next = positions
                .filter(|&index| index > 0 && !rows[index].2)
                .map(|index| index as u32)
                .next()
                .unwrap_or(previous.get());
            correcting.set(true);
            w.set_selected(next);
            correcting.set(false);
        }
        previous.set(w.selected());
        if w.selected() == 0 {
            w.update_property(&[gtk::accessible::Property::ValueText("")]);
        }
    });
    w
}
/// The domain value at each native position; empty and heading rows have none.
pub fn select_values(items: &[SelectItem]) -> Vec<Option<String>> {
    let mut values = vec![None];
    for item in items {
        match item {
            SelectItem::Option(option) => values.push(Some(option.value.clone())),
            SelectItem::Group { options, .. } => {
                values.push(None);
                values.extend(options.iter().map(|option| Some(option.value.clone())));
            }
        }
    }
    values
}
/// A searchable selector. Text stays an option from the model; free-form
/// autocomplete uses a host-owned Entry and Popover.
pub fn combobox(labels: &[&str], size: Size) -> gtk::DropDown {
    let w = select(labels, size);
    w.set_enable_search(true);
    w
}
pub fn slider(min: f64, max: f64, step: f64, tint: Tint, size: Size) -> gtk::Scale {
    let w = styled(
        gtk::Scale::with_range(gtk::Orientation::Horizontal, min, max, step),
        "dz-slider",
        tint,
        size,
    );
    w.set_draw_value(false);
    w.set_hexpand(true);
    w
}
pub fn progress(fraction: f64, tint: Tint, size: Size) -> gtk::ProgressBar {
    let w = styled(gtk::ProgressBar::new(), "dz-progress", tint, size);
    w.set_fraction(fraction.clamp(0.0, 1.0));
    w
}
pub fn meter(min: f64, max: f64, value: f64, tint: Tint, size: Size) -> gtk::LevelBar {
    let w = styled(
        gtk::LevelBar::for_interval(min, max),
        "dz-meter",
        tint,
        size,
    );
    w.set_value(value.clamp(min, max));
    w
}
pub fn spinner(size: Size) -> gtk::Spinner {
    let w = styled(gtk::Spinner::new(), "dz-spinner", Tint::Primary, size);
    w.start();
    w
}
pub fn link(label: &str, uri: &str) -> gtk::LinkButton {
    styled(
        gtk::LinkButton::with_label(uri, label),
        "dz-link",
        Tint::Primary,
        Size::Medium,
    )
}
