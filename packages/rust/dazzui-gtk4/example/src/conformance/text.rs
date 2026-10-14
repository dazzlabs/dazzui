use dazzui_gtk4::{gtk, prelude::*};

/// The first visible text owned by this specimen, excluding nested specimens.
/// Native controls put their label, editable text and placeholder below their
/// behaviour root; reading the root's style loses those nodes' own ink/type.
pub fn first_text(root: &gtk::Widget, specimens: &[gtk::Widget]) -> Option<gtk::Widget> {
    fn visit(
        widget: &gtk::Widget,
        root: &gtk::Widget,
        specimens: &[gtk::Widget],
    ) -> Option<gtk::Widget> {
        if !widget.is_mapped() || (widget != root && specimens.contains(widget)) {
            return None;
        }
        let has_text = if let Some(label) = widget.downcast_ref::<gtk::Label>() {
            !label.text().trim().is_empty()
        } else if let Some(text) = widget.downcast_ref::<gtk::Text>() {
            !text.text().is_empty()
        } else if let Some(view) = widget.downcast_ref::<gtk::TextView>() {
            view.buffer().char_count() > 0
        } else {
            false
        };
        if has_text {
            return Some(widget.clone());
        }
        let mut child = widget.first_child();
        while let Some(node) = child {
            if let Some(text) = visit(&node, root, specimens) {
                return Some(text);
            }
            child = node.next_sibling();
        }
        None
    }
    visit(root, root, specimens)
}

pub fn first_glyph(root: &gtk::Widget, specimens: &[gtk::Widget]) -> Option<gtk::Widget> {
    if !root.is_mapped() {
        return None;
    }
    if root.is::<gtk::Image>() {
        return Some(root.clone());
    }
    let mut child = root.first_child();
    while let Some(node) = child {
        if !specimens.contains(&node)
            && let Some(glyph) = first_glyph(&node, specimens)
        {
            return Some(glyph);
        }
        child = node.next_sibling();
    }
    None
}
