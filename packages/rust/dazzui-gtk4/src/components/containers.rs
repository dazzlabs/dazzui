use crate::{Size, Tint, button, checkbox, label, styled};
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
    Raised,
    #[default]
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
    let w = card(CardVariant::Tinted, tint, Size::Medium);
    w.add_css_class("dz-callout");
    w.append(&label(title, "label-strong"));
    let description = label(body, "body-small");
    description.set_wrap(true);
    w.append(&description);
    w
}
pub fn empty_state(
    title: &str,
    description: &str,
    action: Option<&impl IsA<gtk::Widget>>,
) -> gtk::Box {
    let w = box_(gtk::Orientation::Vertical, "3");
    w.add_css_class("dz-empty-state");
    w.append(&label(title, "title-medium"));
    let body = label(description, "body-medium");
    body.set_wrap(true);
    w.append(&body);
    if let Some(action) = action {
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
    title.set_mnemonic_widget(Some(control));
    control
        .as_ref()
        .update_relation(&[gtk::accessible::Relation::LabelledBy(&[title.upcast_ref()])]);
    w.append(&title);
    w.append(control);
    if let Some(description) = description {
        let hint = label(description, "body-small");
        hint.add_css_class("dz-muted");
        hint.set_wrap(true);
        control
            .as_ref()
            .update_relation(&[gtk::accessible::Relation::DescribedBy(&[hint.upcast_ref()])]);
        w.append(&hint);
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
    w.append(&label(title, "title-small"));
    w
}
pub fn preference_row(
    text: &str,
    description: Option<&str>,
    control: &impl IsA<gtk::Widget>,
) -> gtk::Box {
    let w = box_(gtk::Orientation::Horizontal, "3");
    w.add_css_class("dz-preference-row");
    let column = box_(gtk::Orientation::Vertical, "1");
    column.set_hexpand(true);
    column.append(&label(text, "label-medium"));
    if let Some(description) = description {
        let hint = label(description, "body-small");
        hint.add_css_class("dz-muted");
        hint.set_wrap(true);
        column.append(&hint);
    }
    w.append(&column);
    w.append(control);
    crate::DazzWidgetExt::set_dazz_label(control, text);
    w
}
pub fn collapsible(text: &str, child: &impl IsA<gtk::Widget>) -> gtk::Expander {
    let w = gtk::Expander::new(Some(text));
    w.add_css_class("dz-collapsible");
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
/// Grouped toggles preserve GTK's navigation and exclusive selection.
pub fn segmented_control(labels: &[&str], selected: usize) -> gtk::Box {
    let w = box_(gtk::Orientation::Horizontal, "0");
    w.add_css_class("dz-segmented-control");
    let mut first: Option<gtk::ToggleButton> = None;
    for (index, text) in labels.iter().enumerate() {
        let item = crate::toggle(text, Tint::Neutral, Size::Small);
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
    w.add_css_class("dz-option-card");
    if let Some(group) = group {
        w.set_group(Some(group));
    }
    let column = box_(gtk::Orientation::Vertical, "1");
    column.append(&label(title, "label-strong"));
    column.append(&label(description, "body-small"));
    w.set_child(Some(&column));
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
    w.set_menu_model(Some(model));
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
