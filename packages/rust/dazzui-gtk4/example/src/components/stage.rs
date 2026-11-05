use crate::support::{scene, story::Scene};
use dazzui_gtk4::{gtk, prelude::*, *};
use gtk::{glib, subclass::prelude::*};

mod imp {
    use super::*;
    #[derive(Default)]
    pub struct Viewport {
        pub content: glib::WeakRef<gtk::Widget>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for Viewport {
        const NAME: &'static str = "DazzGalleryStageViewport";
        type Type = super::Viewport;
        type ParentType = gtk::Widget;
    }
    impl ObjectImpl for Viewport {
        fn dispose(&self) {
            if let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Viewport {
        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }
        fn measure(&self, o: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let Some(content) = self.content.upgrade() else {
                return (0, 0, -1, -1);
            };
            let (min, nat, _, _) = content.measure(gtk::Orientation::Horizontal, -1);
            if o == gtk::Orientation::Horizontal {
                return (0, nat, -1, -1);
            }
            let width = if for_size >= 0 {
                for_size.max(min)
            } else {
                nat
            };
            content.measure(o, width)
        }
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            if let Some(viewport) = self.obj().first_child() {
                viewport.allocate(width, height, baseline, None);
            }
        }
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            if let Some(viewport) = self.obj().first_child() {
                self.obj().snapshot_child(&viewport, snapshot);
            }
        }
        fn focus(&self, direction: gtk::DirectionType) -> bool {
            self.obj()
                .first_child()
                .is_some_and(|viewport| viewport.child_focus(direction))
        }
    }
}
glib::wrapper! {pub struct Viewport(ObjectSubclass<imp::Viewport>) @extends gtk::Widget,@implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget;}

/// Shared frame for overview previews and document examples.
pub fn stage(value: &Scene, overview: bool) -> gtk::Box {
    let stage = card(CardVariant::Outlined, Tint::Neutral, Size::Large);
    stage.add_css_class("gk-stage");
    if overview {
        stage.add_css_class("gk-preview-stage");
    }
    let content: gtk::Widget = if value.is_supported() {
        match scene::render_scene(value, &mut vec![]) {
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
    viewport.add_css_class("gk-stage-viewport");
    if overview {
        stage.append(&viewport);
    } else {
        // ScrolledWindow's natural height is unconstrained by width. Keep its
        // horizontal scrolling while measuring the actual wrapped paragraph.
        let frame: Viewport = glib::Object::new();
        frame.set_accessible_role(gtk::AccessibleRole::Presentation);
        frame.set_hexpand(true);
        frame.set_vexpand(true);
        frame.imp().content.set(Some(&content));
        viewport.set_parent(&frame);
        stage.append(&frame);
    }
    stage
}
