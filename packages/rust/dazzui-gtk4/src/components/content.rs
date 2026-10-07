use crate::{DazzWidgetExt, Size, Tint, Variant, styled};
use gtk::prelude::*;

/// `type_name` uses the token's kebab name, such as `body-medium` or `title-small`.
pub fn label(text: &str, type_name: &str) -> gtk::Label {
    let w = gtk::Label::new(Some(text));
    w.add_css_class("dz-label");
    w.add_css_class(&format!("dz-type-{type_name}"));
    w.set_xalign(0.0);
    w
}
pub fn badge(text: &str, variant: Variant, tint: Tint, size: Size) -> gtk::Label {
    let w = styled(label(text, "label-small"), "dz-badge", tint, size);
    w.set_dazz_variant(variant);
    w
}
/// Tags are selectable GTK toggles; observe `toggled` and use `set_active`.
pub fn tag(text: &str, variant: Variant, tint: Tint, size: Size) -> gtk::ToggleButton {
    let w = crate::toggle(text, tint, size);
    w.remove_css_class("dz-button");
    w.add_css_class("dz-badge");
    w.add_css_class("dz-tag");
    w.set_dazz_variant(variant);
    w
}
pub fn code(text: &str, block: bool) -> gtk::Label {
    let w = label(text, "body-small");
    w.add_css_class("dz-code");
    w.set_selectable(true);
    if block {
        w.add_css_class("dz-code-block");
        w.set_wrap(true);
    }
    w
}
pub fn key_cap(text: &str) -> gtk::Label {
    let w = label(text, "label-small");
    w.add_css_class("dz-key-cap");
    w
}
pub fn section_label(text: &str) -> gtk::Label {
    let w = label(text, "label-small");
    w.add_css_class("dz-section-label");
    w
}
pub fn avatar(initials: &str, size: Size) -> gtk::Label {
    let w = styled(
        label(initials, "label-medium"),
        "dz-avatar",
        Tint::Neutral,
        size,
    );
    w.set_xalign(0.5);
    w
}
pub fn divider(orientation: gtk::Orientation) -> gtk::Separator {
    let w = gtk::Separator::new(orientation);
    w.add_css_class("dz-divider");
    w
}
