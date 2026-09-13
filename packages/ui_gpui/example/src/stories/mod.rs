//! The registry, the way `styles/components.css` is for the React package and
//! `components/mod.rs` is for this one's library half: a story page ships once
//! it is named here.
//!
//! Tokens before Components, because the components are drawn out of them; and
//! within Components, alphabetical — there is one page per component, so the
//! group is an index you look a name up in.

pub mod tokens;
pub mod tree;

pub mod accordion;
pub mod action_bar;
pub mod aside;
pub mod autocomplete;
pub mod avatar;
pub mod checkbox;
pub mod code;
pub mod code_block;
pub mod collapsible;
pub mod combobox;
pub mod context_menu;
pub mod dialog;
pub mod drawer;
pub mod empty_state;
pub mod form;
pub mod form_field;
pub mod icon;
pub mod icon_button;
pub mod key_cap;
pub mod link;
pub mod menu;
pub mod menubar;
pub mod meter;
pub mod nav_item;
pub mod number_field;
pub mod option_card;
pub mod otp_field;
pub mod pill_tabs;
pub mod popover;
pub mod preferences;
pub mod preview_card;
pub mod progress;
pub mod radio;
pub mod rail;
pub mod search_field;
pub mod section_label;
pub mod segmented_control;
pub mod select;
pub mod shortcut_recorder;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod splitter;
pub mod switch;
pub mod table;
pub mod tag;
pub mod text_field;
pub mod toast;
pub mod toggle;
pub mod toggle_group;

use crate::storybook::StoryRef;

pub use crate::generated_stories::GROUPS;

/// Where the canvas points when the window opens.
pub const FIRST: StoryRef = StoryRef::new("Button", "Overview");
