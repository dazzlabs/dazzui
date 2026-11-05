//! An edge sheet in a native, modal popup covering its owning window.
use crate::{Theme, box_, label, prelude::*};
use gtk::{gdk, glib};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    Left,
    #[default]
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Debug, Default)]
pub struct DrawerOptions {
    pub side: DrawerSide,
    /// Logical pixels; absent uses the ds drawer size.
    pub size: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct Drawer {
    popup: gtk::Popover,
    panel: gtk::Box,
    layout: std::rc::Rc<dyn Fn(&gtk::Popover)>,
}

impl Drawer {
    pub fn new(
        trigger: &impl IsA<gtk::Widget>,
        content: &impl IsA<gtk::Widget>,
        footer: Option<&gtk::Widget>,
        options: DrawerOptions,
    ) -> Self {
        let popup = gtk::Popover::builder()
            .has_arrow(false)
            .autohide(true)
            .position(gtk::PositionType::Bottom)
            .halign(gtk::Align::Center)
            .build();
        popup.add_css_class("dz-drawer-popup");
        let viewport = gtk::Overlay::new();
        viewport.set_overflow(gtk::Overflow::Hidden);
        let scrim = box_(gtk::Orientation::Vertical, "0");
        scrim.add_css_class("dz-drawer-scrim");
        viewport.set_child(Some(&scrim));
        let panel = box_(gtk::Orientation::Vertical, "0");
        panel.add_css_class("dz-drawer");
        panel.set_accessible_role(gtk::AccessibleRole::Dialog);
        panel.set_focusable(true);
        panel.set_overflow(gtk::Overflow::Hidden);
        panel.add_css_class(match options.side {
            DrawerSide::Left => "dz-drawer-left",
            DrawerSide::Right => "dz-drawer-right",
            DrawerSide::Top => "dz-drawer-top",
            DrawerSide::Bottom => "dz-drawer-bottom",
        });
        let vertical = matches!(options.side, DrawerSide::Top | DrawerSide::Bottom);
        panel.set_halign(if vertical {
            gtk::Align::Fill
        } else if options.side == DrawerSide::Left {
            gtk::Align::Start
        } else {
            gtk::Align::End
        });
        panel.set_valign(if !vertical {
            gtk::Align::Fill
        } else if options.side == DrawerSide::Top {
            gtk::Align::Start
        } else {
            gtk::Align::End
        });
        if vertical {
            let grabber = box_(gtk::Orientation::Horizontal, "0");
            grabber.add_css_class("dz-drawer-grabber");
            grabber.set_halign(gtk::Align::Center);
            panel.append(&grabber);
        }
        if options.title.is_some() || options.description.is_some() {
            let header = box_(gtk::Orientation::Vertical, "1");
            header.add_css_class("dz-drawer-header");
            header.set_valign(gtk::Align::Start);
            if let Some(title) = &options.title {
                let title = label(title, "label-strong");
                title.add_css_class("dz-drawer-title");
                title.set_wrap(true);
                panel.update_relation(&[gtk::accessible::Relation::LabelledBy(&[
                    title.upcast_ref()
                ])]);
                header.append(&title);
            }
            if let Some(description) = &options.description {
                let description = label(description, "caption-small");
                description.add_css_class("dz-drawer-subtitle");
                description.set_wrap(true);
                panel.update_relation(&[gtk::accessible::Relation::DescribedBy(&[
                    description.upcast_ref()
                ])]);
                header.append(&description);
            }
            panel.append(&header);
        }
        let body = box_(gtk::Orientation::Vertical, "4");
        body.add_css_class("dz-drawer-body");
        body.append(content);
        let scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .child(&body)
            .build();
        panel.append(&scroll);
        if let Some(footer) = footer {
            let row = box_(gtk::Orientation::Horizontal, "2");
            row.add_css_class("dz-drawer-footer");
            row.append(footer);
            panel.append(&row);
        }
        viewport.add_overlay(&panel);
        popup.set_child(Some(&viewport));
        popup.set_parent(trigger);
        let weak = popup.downgrade();
        trigger.connect_unmap(move |_| {
            if let Some(popup) = weak.upgrade() {
                popup.popdown();
            }
        });
        let weak = popup.downgrade();
        trigger.connect_destroy(move |_| {
            if let Some(popup) = weak.upgrade() {
                popup.unparent();
            }
        });
        let click = gtk::GestureClick::new();
        let weak = popup.downgrade();
        click.connect_released(move |_, _, _, _| {
            if let Some(popup) = weak.upgrade() {
                popup.popdown();
            }
        });
        scrim.add_controller(click);
        let weak_panel = panel.downgrade();
        let weak_viewport = viewport.downgrade();
        let previous = std::cell::Cell::new(None);
        let layout = move |popup: &gtk::Popover| {
            let Some(trigger) = popup.parent() else {
                return;
            };
            let Some(window) = trigger.root().and_downcast::<gtk::Window>() else {
                return;
            };
            let Some(bounds) = trigger.compute_bounds(&window) else {
                return;
            };
            let (Some(panel), Some(viewport)) = (weak_panel.upgrade(), weak_viewport.upgrade())
            else {
                return;
            };
            let (width, height) = (window.width().max(1), window.height().max(1));
            let theme = Theme::of(&trigger).unwrap_or_default();
            let size = options
                .size
                .unwrap_or_else(|| theme.dimension("drawer-size").unwrap().round() as i32)
                .max(1);
            let x = -(bounds.x().round() as i32);
            let y = -(bounds.y().round() as i32);
            let rtl = trigger.direction() == gtk::TextDirection::Rtl;
            let current = (width, height, x, y, size, rtl);
            if previous.replace(Some(current)) == Some(current) {
                return;
            }
            popup.set_direction(trigger.direction());
            panel.set_direction(trigger.direction());
            if viewport.width_request() != width || viewport.height_request() != height {
                viewport.set_size_request(width, height);
            }
            panel.set_size_request(
                if vertical { -1 } else { size.min(width) },
                if vertical { size.min(height) } else { -1 },
            );
            popup.set_pointing_to(Some(&gdk::Rectangle::new(x, y, width, 0)));
        };
        let layout = std::rc::Rc::new(layout);
        let update = layout.clone();
        popup.connect_show(move |popup| update(popup));
        let update = layout.clone();
        popup.add_tick_callback(move |popup, _| {
            update(popup);
            glib::ControlFlow::Continue
        });
        Self {
            popup,
            panel,
            layout,
        }
    }
    pub fn popup(&self) -> &gtk::Popover {
        &self.popup
    }
    pub fn panel(&self) -> &gtk::Box {
        &self.panel
    }
    pub fn set_open(&self, open: bool) {
        if open {
            if self.popup.is_visible() {
                return;
            }
            (self.layout)(&self.popup);
            self.popup.popup();
            // The reference initially focuses the sheet for both pointer
            // and keyboard opening. GTK retains Tab navigation within it.
            self.panel.grab_focus();
        } else {
            self.popup.popdown();
        }
    }
}
