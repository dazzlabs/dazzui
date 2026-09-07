//! The registry, the way `styles/components.css` is for the React package and
//! `components/mod.rs` is for this one's library half: a story page ships once
//! it is named here.
//!
//! Tokens before Components, because the components are drawn out of them; and
//! within Components, alphabetical — there is one page per component, so the
//! group is an index you look a name up in.

pub mod tokens;

pub mod action_bar;
pub mod aside;
pub mod badge;
pub mod button;
pub mod callout;
pub mod card;
pub mod checkbox;
pub mod dialog;
pub mod divider;
pub mod empty_state;
pub mod form_field;
pub mod icon;
pub mod icon_button;
pub mod key_cap;
pub mod menu;
pub mod nav_item;
pub mod option_card;
pub mod pill_tabs;
pub mod preferences;
pub mod progress;
pub mod radio;
pub mod rail;
pub mod search_field;
pub mod section_label;
pub mod segmented_control;
pub mod select;
pub mod shortcut_recorder;
pub mod sidebar;
pub mod spinner;
pub mod step_list;
pub mod switch;
pub mod table;
pub mod text_field;
pub mod toast;

use crate::storybook::{StoryGroup, StoryRef};

pub const GROUPS: &[StoryGroup] = &[
    StoryGroup::new(
        "Tokens",
        &[
            tokens::color::PAGE,
            tokens::typography::PAGE,
            tokens::spacing::PAGE,
            tokens::radius::PAGE,
            tokens::effect::PAGE,
            tokens::control::PAGE,
            tokens::theme::PAGE,
        ],
    ),
    StoryGroup::new(
        "Components",
        &[
            action_bar::PAGE,
            aside::PAGE,
            badge::PAGE,
            button::PAGE,
            callout::PAGE,
            card::PAGE,
            checkbox::PAGE,
            dialog::PAGE,
            divider::PAGE,
            empty_state::PAGE,
            form_field::PAGE,
            icon::PAGE,
            icon_button::PAGE,
            key_cap::PAGE,
            menu::PAGE,
            nav_item::PAGE,
            option_card::PAGE,
            pill_tabs::PAGE,
            preferences::PAGE,
            progress::PAGE,
            radio::PAGE,
            rail::PAGE,
            search_field::PAGE,
            section_label::PAGE,
            segmented_control::PAGE,
            select::PAGE,
            shortcut_recorder::PAGE,
            sidebar::PAGE,
            spinner::PAGE,
            step_list::PAGE,
            switch::PAGE,
            table::PAGE,
            text_field::PAGE,
            toast::PAGE,
        ],
    ),
];

/// Where the canvas points when the window opens.
pub const FIRST: StoryRef = StoryRef::new("Button", "Overview");
