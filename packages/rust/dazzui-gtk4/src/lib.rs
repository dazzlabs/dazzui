//! DazzUI drawing over native GTK4 behaviour.
//!
//! Call [`Theme::apply`] on a window or container before adding controls.
//! Constructors return GTK widgets, so state, signals, models and accessibility
//! remain available through the ordinary gtk4-rs API.
//! ```no_run
//! use dazzui_gtk4::{gtk, prelude::*, *};
//! gtk::init().unwrap();
//! let window = gtk::Window::new();
//! Theme::new("studio-light").unwrap().apply(&window);
//! let save = button("Save", Variant::Filled, Tint::Primary, Size::Medium);
//! save.connect_clicked(|_| println!("saved"));
//! window.set_child(Some(&save));
//! ```
pub use gtk;
pub mod prelude {
    pub use crate::DazzWidgetExt;
    pub use gtk::prelude::*;
}
mod components;
mod theme;
pub use components::*;
pub use theme::{COMPONENTS_CSS, THEMES, TOKENS_CSS, Theme, ThemeError, init};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tint {
    #[default]
    Primary,
    Neutral,
    Info,
    Success,
    Warning,
    Danger,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    Filled,
    Tinted,
    Outlined,
    #[default]
    Normal,
    Recessed,
    Plain,
}
impl Size {
    pub fn name(self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
impl Tint {
    pub fn name(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Neutral => "neutral",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}
impl Variant {
    pub fn name(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Tinted => "tinted",
            Self::Outlined => "outlined",
            Self::Normal => "normal",
            Self::Recessed => "recessed",
            Self::Plain => "plain",
        }
    }
}

use gtk::prelude::*;
fn axis(w: &impl IsA<gtk::Widget>, prefix: &str, names: &[&str], chosen: &str) {
    for name in names {
        let class = format!("dz-{prefix}-{name}");
        if *name == chosen {
            w.add_css_class(&class);
        } else {
            w.remove_css_class(&class);
        }
    }
}
/// Styling axes update the drawing without replacing the widget or its state.
pub trait DazzWidgetExt: IsA<gtk::Widget> {
    fn set_dazz_size(&self, size: Size) {
        axis(
            self,
            "size",
            &["tiny", "small", "medium", "large"],
            size.name(),
        );
    }
    fn set_dazz_tint(&self, tint: Tint) {
        axis(
            self,
            "tint",
            &["primary", "neutral", "info", "success", "warning", "danger"],
            tint.name(),
        );
    }
    fn set_dazz_variant(&self, variant: Variant) {
        axis(
            self,
            "variant",
            &[
                "filled", "tinted", "outlined", "normal", "recessed", "plain",
            ],
            variant.name(),
        );
    }
    fn set_dazz_invalid(&self, invalid: bool) {
        if invalid {
            self.add_css_class("dz-invalid");
        } else {
            self.remove_css_class("dz-invalid");
        }
        self.as_ref()
            .update_state(&[gtk::accessible::State::Invalid(if invalid {
                gtk::AccessibleInvalidState::True
            } else {
                gtk::AccessibleInvalidState::False
            })]);
    }
    fn set_dazz_label(&self, label: &str) {
        self.as_ref()
            .update_property(&[gtk::accessible::Property::Label(label)]);
    }
}
impl<T: IsA<gtk::Widget>> DazzWidgetExt for T {}
fn styled<T: IsA<gtk::Widget>>(widget: T, class: &str, tint: Tint, size: Size) -> T {
    widget.add_css_class(class);
    widget.set_dazz_tint(tint);
    widget.set_dazz_size(size);
    widget
}
