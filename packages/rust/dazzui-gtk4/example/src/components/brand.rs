use dazzui_gtk4::{gtk, prelude::*, *};
pub fn brand() -> gtk::Button {
    let brand = button("DazzUI", Variant::Plain, Tint::Neutral, Size::Large);
    brand.add_css_class("gk-brand");
    let row = box_(gtk::Orientation::Horizontal, "2");
    row.set_halign(gtk::Align::Start);
    let mark = box_(gtk::Orientation::Vertical, "0");
    mark.add_css_class("gk-brand-mark");
    mark.set_valign(gtk::Align::Center);
    row.append(&mark);
    row.append(&label("DazzUI", "title-small"));
    brand.set_child(Some(&row));
    brand.set_dazz_label("DazzUI — open Overview");
    brand
}
