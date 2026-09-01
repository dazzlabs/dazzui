//! The components.
//!
//! One file per component, named after the React export it ports —
//! `button.rs` is `Button`, `icon_button.rs` is `IconButton`. A component is
//! listed here once it ships, the way `styles/components.css` is the registry
//! on the React side.

mod action_bar;
mod alert_banner;
mod aside;
mod badge;
mod button;
mod card;
mod checkbox;
mod dialog;
mod divider;
mod empty_state;
mod form_field;
mod icon;
mod icon_button;
mod input;
mod kbd;
mod menu;
mod nav_item;
mod option_card;
mod pill_tabs;
mod preference;
mod progress;
mod radio;
mod rail;
mod search_field;
mod section_label;
mod segmented_control;
mod shortcut_recorder;
mod sidebar;
mod spinner;
mod step_list;
mod table;
mod toast;
mod toggle;
mod tooltip;

pub use action_bar::*;
pub use alert_banner::*;
pub use aside::*;
pub use badge::*;
pub use button::*;
pub use card::*;
pub use checkbox::*;
pub use dialog::*;
pub use divider::*;
pub use empty_state::*;
pub use form_field::*;
pub use icon::*;
pub use icon_button::*;
pub use input::*;
pub use kbd::*;
pub use menu::*;
pub use nav_item::*;
pub use option_card::*;
pub use pill_tabs::*;
pub use preference::*;
pub use progress::*;
pub use radio::*;
pub use rail::*;
pub use search_field::*;
pub use section_label::*;
pub use segmented_control::*;
pub use shortcut_recorder::*;
pub use sidebar::*;
pub use spinner::*;
pub use step_list::*;
pub use table::*;
pub use toast::*;
pub use toggle::*;

pub(crate) use input::editable;
