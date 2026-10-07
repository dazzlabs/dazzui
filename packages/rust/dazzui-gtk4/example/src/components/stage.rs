use crate::support::{scene, story::Scene};
use dazzui_gtk4::{gtk, prelude::*, *};

/// Shared frame for overview previews and document examples.
pub fn stage(value: &Scene, overview: bool) -> gtk::Box {
    let stage = card(CardVariant::Outlined, Tint::Neutral, Size::Large);
    stage.add_css_class("gk-stage");
    if overview {
        stage.add_css_class("gk-preview-stage");
    }
    let content: gtk::Widget = if value.is_supported() {
        match scene::render(&value.content, &mut vec![]) {
            Ok(widget) => widget,
            Err(error) => callout("Example unavailable", &error, Tint::Danger).upcast(),
        }
    } else {
        let pending = label("This example is not available in GTK4 yet.", "body-small");
        pending.add_css_class("gk-quiet");
        pending.set_wrap(true);
        pending.set_tooltip_text(Some(&value.gaps.join("\n")));
        pending.upcast()
    };
    content.set_halign(gtk::Align::Center);
    content.set_valign(gtk::Align::Center);
    content.set_vexpand(true);
    let viewport = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(if overview {
            gtk::PolicyType::External
        } else {
            gtk::PolicyType::Automatic
        })
        .vscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_width(false)
        .propagate_natural_height(true)
        .hexpand(true)
        .vexpand(true)
        .child(&content)
        .build();
    stage.append(&viewport);
    stage
}
