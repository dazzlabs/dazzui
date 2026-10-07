use dazzui_gtk4::{gtk, prelude::*, *};

/// The same miniature window as React: column, paper, ink and accent.
pub fn theme_preview(theme: Theme) -> gtk::Box {
    let preview = box_(gtk::Orientation::Horizontal, "0");
    preview.add_css_class("g-preview");
    Theme::apply(theme, &preview);
    let column = box_(gtk::Orientation::Vertical, "0");
    column.add_css_class("g-preview-column");
    preview.append(&column);
    let paper = box_(gtk::Orientation::Vertical, "1");
    paper.add_css_class("g-preview-paper");
    paper.set_hexpand(true);
    for _ in 0..2 {
        let ink = box_(gtk::Orientation::Vertical, "0");
        ink.add_css_class("g-preview-ink");
        ink.set_halign(gtk::Align::Start);
        paper.append(&ink);
    }
    let accent = box_(gtk::Orientation::Vertical, "0");
    accent.add_css_class("g-preview-accent");
    accent.set_halign(gtk::Align::Start);
    paper.append(&accent);
    preview.append(&paper);
    preview
}
