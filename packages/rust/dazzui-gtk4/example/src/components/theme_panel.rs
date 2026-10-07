use super::theme_preview::theme_preview;
use crate::support::themes::FAMILY_GROUPS;
use dazzui_gtk4::{gtk, prelude::*, *};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct Choice {
    family: &'static str,
    button: gtk::ToggleButton,
    preview: gtk::Box,
}
struct Controller {
    window: gtk::glib::WeakRef<gtk::ApplicationWindow>,
    current: RefCell<Theme>,
    updating: Cell<bool>,
    mode: gtk::ToggleButton,
    choices: Vec<Choice>,
}
impl Controller {
    fn select(&self, theme: Theme) {
        self.updating.set(true);
        *self.current.borrow_mut() = theme;
        if let Some(window) = self.window.upgrade() {
            theme.apply(&window);
        }
        self.mode.set_active(theme.is_dark());
        self.mode.set_dazz_label(if theme.is_dark() {
            "Switch to light"
        } else {
            "Switch to dark"
        });
        for choice in &self.choices {
            let id = format!(
                "{}-{}",
                choice.family,
                if theme.is_dark() { "dark" } else { "light" }
            );
            Theme::new(&id)
                .expect("Generated theme family")
                .apply(&choice.preview);
            choice
                .button
                .set_active(theme.name().rsplit_once('-').unwrap().0 == choice.family);
        }
        self.updating.set(false);
    }
}
pub struct ThemePanel {
    pub widget: gtk::Box,
    controller: Rc<Controller>,
}
impl ThemePanel {
    pub fn new(window: &gtk::ApplicationWindow, theme: Theme) -> Self {
        let panel = box_(gtk::Orientation::Vertical, "0");
        panel.add_css_class("gk-themes");
        panel.set_size_request(280, -1);
        let head = box_(gtk::Orientation::Horizontal, "2");
        head.add_css_class("gk-toolbar");
        let title = label("Theme", "label-strong");
        title.set_hexpand(true);
        head.append(&title);
        let mode = toggle("Dark", Tint::Neutral, Size::Small);
        mode.set_valign(gtk::Align::Center);
        head.append(&mode);
        panel.append(&head);
        let groups = box_(gtk::Orientation::Vertical, "2");
        let mut choices = vec![];
        let mut first: Option<gtk::ToggleButton> = None;
        for group in FAMILY_GROUPS {
            let section = box_(gtk::Orientation::Vertical, "2");
            section.add_css_class("gk-theme-group");
            section.append(&section_label(group.title));
            let grid = gtk::Grid::new();
            grid.set_column_homogeneous(true);
            for (i, family) in group.families.iter().enumerate() {
                let button = gtk::ToggleButton::builder()
                    .accessible_role(gtk::AccessibleRole::Radio)
                    .build();
                button.add_css_class("dz-button");
                button.set_dazz_size(Size::Small);
                button.set_dazz_tint(Tint::Neutral);
                button.set_dazz_variant(Variant::Normal);
                if let Some(first) = &first {
                    button.set_group(Some(first));
                }
                button.add_css_class("gk-theme-tile");
                button.set_dazz_label(family.title);
                if first.is_none() {
                    first = Some(button.clone());
                }
                let column = box_(gtk::Orientation::Vertical, "0");
                let preview =
                    theme_preview(Theme::new(&format!("{}-light", family.value)).unwrap());
                column.append(&preview);
                let name = label(family.title, "label-small");
                name.set_xalign(0.5);
                name.set_ellipsize(gtk::pango::EllipsizeMode::End);
                name.set_max_width_chars(12);
                name.add_css_class("gk-theme-name");
                column.append(&name);
                button.set_child(Some(&column));
                grid.attach(&button, (i % 3) as i32, (i / 3) as i32, 1, 1);
                choices.push(Choice {
                    family: family.value,
                    button,
                    preview,
                });
            }
            section.append(&grid);
            groups.append(&section);
        }
        let scroll = gtk::ScrolledWindow::builder()
            .child(&groups)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vexpand(true)
            .build();
        panel.append(&scroll);
        let controller = Rc::new(Controller {
            window: window.downgrade(),
            current: RefCell::new(theme),
            updating: Cell::new(false),
            mode: mode.clone(),
            choices,
        });
        for choice in &controller.choices {
            let weak = Rc::downgrade(&controller);
            let family = choice.family;
            choice.button.connect_toggled(move |button| {
                if let Some(controller) = weak.upgrade()
                    && !controller.updating.get()
                    && button.is_active()
                {
                    let mode = if controller.current.borrow().is_dark() {
                        "dark"
                    } else {
                        "light"
                    };
                    controller.select(Theme::new(&format!("{family}-{mode}")).unwrap());
                }
            });
        }
        let weak = Rc::downgrade(&controller);
        mode.connect_toggled(move |mode| {
            if let Some(controller) = weak.upgrade()
                && !controller.updating.get()
            {
                let family = controller
                    .current
                    .borrow()
                    .name()
                    .rsplit_once('-')
                    .unwrap()
                    .0;
                controller.select(
                    Theme::new(&format!(
                        "{family}-{}",
                        if mode.is_active() { "dark" } else { "light" }
                    ))
                    .unwrap(),
                );
            }
        });
        controller.select(theme);
        Self {
            widget: panel,
            controller,
        }
    }
    pub fn theme(&self) -> Theme {
        *self.controller.current.borrow()
    }
    pub fn select(&self, theme: Theme) {
        self.controller.select(theme);
    }
    pub fn set_dark(&self, dark: bool) {
        self.controller.mode.set_active(dark);
    }
}
