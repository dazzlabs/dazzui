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
