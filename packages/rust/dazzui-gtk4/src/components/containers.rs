use crate::{DazzWidgetExt, Size, Tint, button, checkbox, label, styled};
use gtk::prelude::*;

/// A semantic spacing key, e.g. `3` or `2.5`. GTK's CSS border-spacing
/// supplies the box gap and changes with a nested theme.
pub fn box_(orientation: gtk::Orientation, gap: &str) -> gtk::Box {
    let w = gtk::Box::new(orientation, 0);
    w.add_css_class(&format!("dz-gap-{}", gap.replace('.', "-")));
    w
}
#[derive(Clone, Copy, Debug, Default)]
pub enum CardVariant {
    #[default]
    Raised,
    Sunken,
    Outlined,
    Tinted,
}
pub fn card(variant: CardVariant, tint: Tint, size: Size) -> gtk::Box {
    let w = styled(box_(gtk::Orientation::Vertical, "3"), "dz-card", tint, size);
    w.add_css_class(match variant {
        CardVariant::Raised => "dz-card-raised",
        CardVariant::Sunken => "dz-card-sunken",
        CardVariant::Outlined => "dz-card-outlined",
        CardVariant::Tinted => "dz-card-tinted",
    });
    w
}
pub fn callout(title: &str, body: &str, tint: Tint) -> gtk::Box {
    let title = label(title, "label-medium");
    let description = label(body, "body-small");
    description.set_wrap(true);
    callout_with_slots(
        Some(title.upcast_ref()),
        Some(description.upcast_ref()),
        None,
        None,
        tint,
        Size::Medium,
    )
}
/// The same notice with React's leading, title, description and action slots.
pub fn callout_with_slots(
    title: Option<&gtk::Widget>,
    description: Option<&gtk::Widget>,
    leading: Option<&gtk::Widget>,
    action: Option<&gtk::Widget>,
    tint: Tint,
    size: Size,
) -> gtk::Box {
    let w = styled(
        box_(gtk::Orientation::Horizontal, "2-5"),
        "dz-callout",
        tint,
        size,
    );
    let align = if title.is_some() && description.is_some() {
        gtk::Align::Start
    } else {
        gtk::Align::Center
    };
    if let Some(icon) = leading {
        icon.add_css_class("dz-callout-icon");
        icon.set_valign(align);
        w.append(icon);
    }
    let body = box_(gtk::Orientation::Vertical, "1");
    body.set_valign(align);
    body.set_hexpand(action.is_some());
    if let Some(title) = title {
        title.add_css_class("dz-callout-title");
        body.append(title);
    }
    if let Some(description) = description {
        description.add_css_class("dz-callout-description");
        body.append(description);
    }
    w.append(&body);
    if let Some(action) = action {
        action.set_valign(align);
        w.append(action);
    }
    w
}
pub fn empty_state(
    title: &str,
    description: &str,
    action: Option<&impl IsA<gtk::Widget>>,
) -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "3");
    w.add_css_class("dz-empty-state");
    w.set_hexpand(true);
    let title = label(title, "body-medium");
    title.add_css_class("dz-empty-state-title");
    title.set_xalign(0.5);
    w.append(&title);
    if !description.is_empty() {
        let body = label(description, "body-medium");
        body.set_wrap(true);
        body.set_xalign(0.5);
        w.append(&body);
    }
    if let Some(action) = action {
        action.set_halign(gtk::Align::Center);
        w.append(action);
    }
    w
}
pub fn form_field(
    text: &str,
    control: &impl IsA<gtk::Widget>,
    description: Option<&str>,
) -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "2");
    w.add_css_class("dz-form-field");
    let title = label(text, "label-medium");
    title.add_css_class("dz-form-field-label");
    title.set_mnemonic_widget(Some(control));
    control
        .as_ref()
        .update_relation(&[gtk::accessible::Relation::LabelledBy(&[title.upcast_ref()])]);
    w.append(&title);
    w.append(control);
    if let Some(description) = description {
        let hint = label(description, "body-small");
        hint.add_css_class("dz-form-field-hint");
        hint.set_wrap(true);
        control
            .as_ref()
            .update_relation(&[gtk::accessible::Relation::DescribedBy(&[hint.upcast_ref()])]);
        w.append(&super::hint_layout::wrap(&hint));
    }
    w
}
pub fn preferences() -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "10");
    w.add_css_class("dz-preferences");
    w
}
pub fn preference_group(title: &str) -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "4");
    w.add_css_class("dz-preference-group");
    let titles = box_(gtk::Orientation::Vertical, "0-5");
    titles.add_css_class("dz-preference-group-titles");
    titles.append(&label(title, "title-medium"));
    w.append(&titles);
    super::column_layout::install(&w, "preferences-title-gap");
    w
}
pub fn preference_section(title: &str) -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "0");
    w.add_css_class("dz-preference-section");
    if !title.is_empty() {
        let heading = crate::section_label(title);
        heading.add_css_class("dz-preference-section-heading");
        w.append(&heading);
    }
    w
}
pub fn preference_row(
    text: &str,
    description: Option<&str>,
    control: &impl IsA<gtk::Widget>,
) -> gtk::Box {
    let w = box_(gtk::Orientation::Horizontal, "2-5");
    w.add_css_class("dz-preference-row");
    let column = box_(gtk::Orientation::Vertical, "1");
    column.add_css_class("dz-preference-text");
    column.set_valign(gtk::Align::Center);
    column.set_hexpand(true);
    column.append(&label(text, "label-quiet"));
    if let Some(description) = description {
        let hint = label(description, "caption-small");
        hint.add_css_class("dz-preference-subtitle");
        column.append(&hint);
    }
    w.append(&column);
    control.set_valign(gtk::Align::Center);
    w.append(control);
    crate::DazzWidgetExt::set_dazz_label(control, text);
    w
}
pub fn collapsible(text: &str, child: &impl IsA<gtk::Widget>) -> gtk::Expander {
    collapsible_with_slots(&gtk::Label::new(Some(text)).upcast(), None, None, child)
}
/// A row that opens another view. GTK owns activation and keyboard focus;
/// the same text column keeps inert and interactive rows aligned.
pub fn preference_row_button(
    text: &str,
    description: Option<&str>,
    control: &impl IsA<gtk::Widget>,
) -> gtk::Button {
    let content = preference_row(text, description, control);
    content.remove_css_class("dz-preference-row");
    let button = gtk::Button::new();
    button.add_css_class("dz-preference-row");
    button.set_dazz_tint(Tint::Neutral);
    button.set_child(Some(&content));
    button.set_dazz_label(text);
    button
}
pub fn collapsible_with_slots(
    title: &gtk::Widget,
    leading: Option<&gtk::Widget>,
    meta: Option<&gtk::Widget>,
    child: &impl IsA<gtk::Widget>,
) -> gtk::Expander {
    let w = gtk::Expander::new(None);
    w.add_css_class("dz-collapsible");
    let heading = box_(gtk::Orientation::Horizontal, "2");
    heading.set_hexpand(true);
    if let Some(icon) = leading {
        heading.append(icon);
    }
    title.set_hexpand(true);
    if let Some(label) = title.downcast_ref::<gtk::Label>() {
        label.set_wrap(false);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        label.set_xalign(0.);
    }
    heading.append(title);
    if let Some(meta) = meta {
        let slot = box_(gtk::Orientation::Horizontal, "2");
        slot.add_css_class("dz-disclosure-meta");
        slot.append(meta);
        heading.append(&slot);
    }
    w.set_label_widget(Some(&heading));
    w.update_relation(&[gtk::accessible::Relation::LabelledBy(&[title.upcast_ref()])]);
    child.add_css_class("dz-collapsible-content");
    w.set_child(Some(child));
    w
}
pub fn splitter(
    orientation: gtk::Orientation,
    first: &impl IsA<gtk::Widget>,
    second: &impl IsA<gtk::Widget>,
) -> gtk::Paned {
    let w = gtk::Paned::new(orientation);
    w.add_css_class("dz-splitter");
    w.set_start_child(Some(first));
    w.set_end_child(Some(second));
    w
}
pub fn tabs(stack: &gtk::Stack) -> gtk::StackSwitcher {
    let w = gtk::StackSwitcher::new();
    w.add_css_class("dz-tabs");
    w.set_stack(Some(stack));
    w
}
/// A free-standing filter tab, with an optional count in the label's ink.
pub struct PillTab<'a> {
    pub value: &'a str,
    pub label: &'a str,
    pub count: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub disabled: bool,
}
pub fn pill_tabs(items: &[PillTab<'_>], selected: Option<&str>, tint: Tint) -> crate::ButtonGroup {
    let root = styled(
        crate::ButtonGroup::new("2"),
        "dz-pill-tabs",
        tint,
        Size::Tiny,
    );
    root.set_accessible_role(gtk::AccessibleRole::TabList);
    root.set_valign(gtk::Align::Center);
    let mut first: Option<gtk::ToggleButton> = None;
    for item in items {
        let tab = gtk::ToggleButton::new();
        super::button_layout::install(&tab);
        tab.set_accessible_role(gtk::AccessibleRole::Tab);
        tab.add_css_class("dz-pill-tab");
        tab.set_valign(gtk::Align::Center);
        tab.set_sensitive(!item.disabled);
        if let Some(first) = &first {
            tab.set_group(Some(first));
        } else {
            first = Some(tab.clone());
        }
        let face = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        face.add_css_class("dz-pill-tab-content");
        if let Some(icon) = item.icon {
            face.append(&gtk::Image::from_icon_name(icon));
        }
        face.append(&gtk::Label::new(Some(item.label)));
        if let Some(count) = item.count {
            let count = gtk::Label::new(Some(count));
            count.add_css_class("dz-pill-tab-count");
            face.append(&count);
        }
        tab.set_child(Some(&face));
        tab.set_active(selected == Some(item.value));
        tab.connect_toggled(|tab| {
            tab.update_state(&[gtk::accessible::State::Selected(Some(tab.is_active()))])
        });
        tab.update_state(&[gtk::accessible::State::Selected(Some(tab.is_active()))]);
        root.append(&tab);
    }
    root
}
/// Grouped toggles preserve GTK's navigation and exclusive selection.
pub fn segmented_control(labels: &[&str], selected: usize) -> crate::ButtonGroup {
    let items: Vec<_> = labels.iter().map(|label| (*label, None)).collect();
    glyph_segmented_control(&items, selected, false)
}
/// A segmented control whose segments may carry a host icon-theme glyph.
///
/// Each item is `(label, icon)`. As in React, a glyph beside a label is drawn
/// `control.label-glyph.shrink` under the profile's icon, the profile's gap
/// before the label, and the segment's leading end pads
/// `control.label-glyph.inset` less. With `icon_only`, a segment that has a
/// glyph draws it alone at the full icon size and keeps the label as its
/// accessible name and tooltip.
pub fn glyph_segmented_control(
    items: &[(&str, Option<&str>)],
    selected: usize,
    icon_only: bool,
) -> crate::ButtonGroup {
    let w = styled(
        crate::ButtonGroup::new("0"),
        "dz-segmented-control",
        Tint::Primary,
        Size::Medium,
    );
    w.set_valign(gtk::Align::Center);
    let mut first: Option<gtk::ToggleButton> = None;
    for (index, (text, icon)) in items.iter().enumerate() {
        // Segments inherit the track's profile. An independent button profile
        // would reset the nested radius and stop size changes reaching them.
        let item = gtk::ToggleButton::with_label(text);
        super::button_layout::install(&item);
        item.add_css_class("dz-segment");
        if let Some(icon) = icon {
            let glyph = gtk::Image::from_icon_name(icon);
            if icon_only {
                glyph.add_css_class("dz-segment-glyph-only");
                item.set_child(Some(&glyph));
                item.set_tooltip_text(Some(text));
            } else {
                glyph.add_css_class("dz-segment-glyph");
                let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                content.add_css_class("dz-segment-content");
                content.append(&glyph);
                content.append(&gtk::Label::new(Some(text)));
                item.set_child(Some(&content));
                item.add_css_class("dz-segment-glyph-leading");
            }
            item.set_dazz_label(text);
        }
        if let Some(first) = &first {
            item.set_group(Some(first));
        } else {
            first = Some(item.clone());
        }
        item.set_active(index == selected);
        w.append(&item);
    }
    w
}
pub fn option_card(
    title: &str,
    description: &str,
    group: Option<&gtk::CheckButton>,
) -> gtk::CheckButton {
    let w = checkbox("", Tint::Primary, Size::Medium);
    w.remove_css_class("dz-checkbox");
    w.add_css_class("dz-option-card");
    w.set_accessible_role(gtk::AccessibleRole::Radio);
    w.set_hexpand(true);
    if let Some(group) = group {
        w.set_group(Some(group));
    }
    let column = box_(gtk::Orientation::Vertical, "1-5");
    let title = label(title, "label-strong");
    title.add_css_class("dz-option-card-title");
    column.append(&title);
    if !description.is_empty() {
        let description = label(description, "caption-small");
        description.add_css_class("dz-option-card-description");
        description.set_wrap(true);
        column.append(&description);
    }
    w.set_child(Some(&column));
    super::option_card_layout::install(&w);
    w
}
pub fn popover(child: &impl IsA<gtk::Widget>) -> gtk::Popover {
    let w = gtk::Popover::new();
    w.add_css_class("dz-popover");
    w.set_child(Some(child));
    w
}
pub fn menu_button(
    text: &str,
    model: &impl IsA<gtk::gio::MenuModel>,
    size: Size,
) -> gtk::MenuButton {
    let w = styled(
        gtk::MenuButton::new(),
        "dz-menu-button",
        Tint::Neutral,
        size,
    );
    w.set_label(text);
    w.set_popover(Some(&crate::menu_popup(model)));
    w
}
/// A transient modal window; GTK supplies focus and platform window behaviour.
/// A window is a separate CSS root, so it explicitly carries its parent's theme.
pub fn dialog(
    parent: &impl IsA<gtk::Window>,
    title: &str,
    body: &impl IsA<gtk::Widget>,
) -> gtk::Window {
    let w = gtk::Window::builder()
        .title(title)
        .transient_for(parent)
        .modal(true)
        .build();
    w.add_css_class("dz-dialog");
    crate::Theme::of(parent.as_ref())
        .unwrap_or_default()
        .apply(&w);
    let content = box_(gtk::Orientation::Vertical, "4");
    content.add_css_class("dz-dialog-body");
    content.append(&label(title, "title-large"));
    content.append(body);
    let close = button("Close", crate::Variant::Normal, Tint::Neutral, Size::Medium);
    let weak = w.downgrade();
    close.connect_clicked(move |_| {
        if let Some(w) = weak.upgrade() {
            w.close();
        }
    });
    content.append(&close);
    let shortcuts = gtk::ShortcutController::new();
    shortcuts.add_shortcut(gtk::Shortcut::new(
        gtk::ShortcutTrigger::parse_string("Escape"),
        Some(gtk::CallbackAction::new({
            let weak = w.downgrade();
            move |_, _| {
                if let Some(w) = weak.upgrade() {
                    w.close();
                }
                gtk::glib::Propagation::Stop
            }
        })),
    ));
    w.add_controller(shortcuts);
    w.set_child(Some(&content));
    w
}
/// Column factories and sorting remain the host's standard GTK model API.
pub fn table(model: impl IsA<gtk::SelectionModel>) -> gtk::ColumnView {
    let w = gtk::ColumnView::new(Some(model));
    w.add_css_class("dz-table");
    w.set_show_row_separators(true);
    w
}
/// Use a TreeListModel and TreeExpander in the factory for hierarchical rows.
pub fn tree(
    model: impl IsA<gtk::SelectionModel>,
    factory: impl IsA<gtk::ListItemFactory>,
) -> gtk::ListView {
    let w = gtk::ListView::new(Some(model), Some(factory));
    w.add_css_class("dz-tree");
    w
}
pub fn action_bar() -> gtk::Box {
    let w = box_(gtk::Orientation::Horizontal, "2");
    w.add_css_class("dz-action-bar");
    w
}

/// A run of fields; submission and server validation are owned by the host.
pub fn form() -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "4");
    w.add_css_class("dz-form");
    w
}
pub fn fieldset(legend: Option<&str>) -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "3");
    w.set_accessible_role(gtk::AccessibleRole::Group);
    w.add_css_class("dz-fieldset");
    if let Some(legend) = legend {
        w.set_dazz_label(legend);
        let title = label(legend, "caption-small");
        title.add_css_class("dz-fieldset-legend");
        w.append(&title);
    }
    w
}
