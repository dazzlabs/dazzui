use crate::{DazzWidgetExt, Size, TextBox, Tint, Variant, styled};
use gtk::prelude::*;

/// A Fluent glyph, with the same name and outline the React export uses.
/// Symbolic rendering inherits the surrounding text colour and icon size.
pub fn icon(name: &str) -> gtk::Image {
    let w = gtk::Image::from_icon_name(&format!("dazzui-{name}-symbolic"));
    w.add_css_class("dz-icon");
    w.set_halign(gtk::Align::Center);
    w.set_valign(gtk::Align::Center);
    w.set_accessible_role(gtk::AccessibleRole::Presentation);
    w
}

/// `type_name` uses the token's kebab name, such as `body-medium` or `title-small`.
pub fn label(text: &str, type_name: &str) -> gtk::Label {
    let w = gtk::Label::new(Some(text));
    w.add_css_class("dz-label");
    w.add_css_class(&format!("dz-type-{type_name}"));
    w.set_xalign(0.0);
    w
}
pub fn badge(text: &str, variant: Variant, tint: Tint, size: Size) -> TextBox {
    let w = styled(TextBox::new(text), "dz-badge", tint, size);
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
/// Convenience form; use `inline_code` or `code_block` for their concrete
/// GTK types and the block's header, gutter and clipboard options.
pub fn code(text: &str, block: bool) -> gtk::Widget {
    if block {
        crate::code_block(text, None, None, None, crate::CodeBlockOptions::default()).upcast()
    } else {
        crate::inline_code(text).upcast()
    }
}
pub fn key_cap(text: &str) -> TextBox {
    let w = TextBox::new(text);
    w.add_css_class("dz-key-cap");
    w.set_valign(gtk::Align::Center);
    w.set_dazz_size(Size::Medium);
    w
}
pub fn section_label(text: &str) -> TextBox {
    let w = TextBox::new(text);
    w.add_css_class("dz-type-label-small");
    w.add_css_class("dz-section-label");
    w
}
pub fn avatar(initials: &str, size: Size) -> gtk::Label {
    let w = styled(
        label(initials, "label-medium"),
        "dz-avatar",
        Tint::Primary,
        size,
    );
    w.set_accessible_role(gtk::AccessibleRole::Img);
    w.set_dazz_label(initials);
    w.set_xalign(0.5);
    w
}
pub fn divider(orientation: gtk::Orientation) -> gtk::Separator {
    let w = gtk::Separator::new(orientation);
    w.add_css_class("dz-divider");
    w
}
