use crate::storybook::StoryPage;
mod accordion;
mod action_bar;
mod aside;
mod avatar;
mod badge;
mod button;
mod callout;
mod card;
mod checkbox;
mod checkbox_group;
mod code;
mod collapsible;
mod combobox;
mod context_menu;
mod dialog;
mod divider;
mod drawer;
mod empty_state;
mod form;
mod form_field;
mod icon_button;
mod key_cap;
mod link;
mod menu;
mod menubar;
mod meter;
mod nav_item;
mod number_field;
mod option_card;
mod otp_field;
mod pill_tabs;
mod popover;
mod preferences;
mod preview_card;
mod progress;
mod radio;
mod rail;
mod search_field;
mod section_label;
mod segmented_control;
mod sidebar;
mod skeleton;
mod slider;
mod spinner;
mod splitter;
mod step_list;
mod switch;
mod table;
mod tag;
mod text_field;
mod toast;
mod toggle;
mod tooltip;
mod tree;
pub const PAGES: &[StoryPage] = &[
    StoryPage {
        name: "Button",
        description: "Six variants share four sizes and six semantic tints.",
        render: button::overview,
    },
    StoryPage {
        name: "IconButton",
        description: "A square action at the control height.",
        render: icon_button::overview,
    },
    StoryPage {
        name: "Badge",
        description: "An annotation that reads the resting color recipe.",
        render: badge::overview,
    },
    StoryPage {
        name: "Card",
        description: "A surface for content, with raised, sunken, outlined and tinted treatments.",
        render: card::overview,
    },
    StoryPage {
        name: "Checkbox",
        description: "A controlled check with mixed-state support.",
        render: checkbox::overview,
    },
    StoryPage {
        name: "Radio",
        description: "One selection from a group of choices.",
        render: radio::overview,
    },
    StoryPage {
        name: "Switch",
        description: "A setting that takes effect immediately.",
        render: switch::overview,
    },
    StoryPage {
        name: "Toggle",
        description: "A button that stays down, with optional controlled state.",
        render: toggle::overview,
    },
    StoryPage {
        name: "TextField",
        description: "Text editing, selection and IME with DazzUI field decoration.",
        render: text_field::overview,
    },
    StoryPage {
        name: "SearchField",
        description: "A search field with a clear action.",
        render: search_field::overview,
    },
    StoryPage {
        name: "NumberField",
        description: "Type a number or step between bounded values.",
        render: number_field::overview,
    },
    StoryPage {
        name: "Slider",
        description: "Pointer and keyboard input for a bounded number.",
        render: slider::overview,
    },
    StoryPage {
        name: "OtpField",
        description: "One editable input underneath a row of code cells.",
        render: otp_field::overview,
    },
    StoryPage {
        name: "Combobox",
        description: "Filter a list and select a value.",
        render: combobox::overview,
    },
    StoryPage {
        name: "SegmentedControl",
        description: "A mutually exclusive choice within one inset track.",
        render: segmented_control::overview,
    },
    StoryPage {
        name: "PillTabs",
        description: "Compact selected navigation with optional counts.",
        render: pill_tabs::overview,
    },
    StoryPage {
        name: "Divider",
        description: "The separator occupies only its own stroke.",
        render: divider::overview,
    },
    StoryPage {
        name: "SectionLabel",
        description: "A quiet heading for a group of related content.",
        render: section_label::overview,
    },
    StoryPage {
        name: "KeyCap",
        description: "A shortcut hint or a boxed keyboard key.",
        render: key_cap::overview,
    },
    StoryPage {
        name: "Avatar",
        description: "Initials or an image on a tinted surface.",
        render: avatar::overview,
    },
    StoryPage {
        name: "Callout",
        description: "A title and message washed in a semantic tint.",
        render: callout::overview,
    },
    StoryPage {
        name: "EmptyState",
        description: "Explain an empty region and provide a next action.",
        render: empty_state::overview,
    },
    StoryPage {
        name: "Code",
        description: "Inline code and scrollable code blocks.",
        render: code::overview,
    },
    StoryPage {
        name: "Progress",
        description: "A determinate fill or an animated indeterminate sweep.",
        render: progress::overview,
    },
    StoryPage {
        name: "Meter",
        description: "A measurement normalized between a minimum and maximum.",
        render: meter::overview,
    },
    StoryPage {
        name: "Spinner",
        description: "An animated ring for pending work.",
        render: spinner::overview,
    },
    StoryPage {
        name: "Skeleton",
        description: "An animated placeholder for content that is loading.",
        render: skeleton::overview,
    },
    StoryPage {
        name: "Collapsible",
        description: "A disclosure header with locally held or controlled open state.",
        render: collapsible::overview,
    },
    StoryPage {
        name: "Accordion",
        description: "A coordinated set of disclosure sections.",
        render: accordion::overview,
    },
    StoryPage {
        name: "NavItem",
        description: "A selected or resting navigation row.",
        render: nav_item::overview,
    },
    StoryPage {
        name: "Sidebar",
        description: "A scrollable navigation column.",
        render: sidebar::overview,
    },
    StoryPage {
        name: "Rail",
        description: "A narrow column of navigation actions.",
        render: rail::overview,
    },
    StoryPage {
        name: "Aside",
        description: "A contextual secondary column.",
        render: aside::overview,
    },
    StoryPage {
        name: "Tree",
        description: "Nested disclosure branches with selectable leaves.",
        render: tree::overview,
    },
    StoryPage {
        name: "Popover",
        description: "A floating content panel anchored to its trigger.",
        render: popover::overview,
    },
    StoryPage {
        name: "Dialog",
        description: "A modal route with a themed scrim and focus scope.",
        render: dialog::overview,
    },
    StoryPage {
        name: "Drawer",
        description: "An edge sheet presented through the navigator.",
        render: drawer::overview,
    },
    StoryPage {
        name: "Menu",
        description: "Actions in an anchored menu, dismissed after selection.",
        render: menu::overview,
    },
    StoryPage {
        name: "Menubar",
        description: "A row of menus.",
        render: menubar::overview,
    },
    StoryPage {
        name: "ContextMenu",
        description: "Secondary-click actions attached to a region.",
        render: context_menu::overview,
    },
    StoryPage {
        name: "Tooltip",
        description: "A short explanation on hover or keyboard focus.",
        render: tooltip::overview,
    },
    StoryPage {
        name: "PreviewCard",
        description: "A hover preview with contextual content.",
        render: preview_card::overview,
    },
    StoryPage {
        name: "Toast",
        description: "Timed messages with individual dismissal.",
        render: toast::overview,
    },
    StoryPage {
        name: "Form",
        description: "Server errors are distributed to named fields.",
        render: form::overview,
    },
    StoryPage {
        name: "FormField",
        description: "A label, control and hint with one shared rhythm.",
        render: form_field::overview,
    },
    StoryPage {
        name: "Preferences",
        description: "Grouped settings with descriptive rows.",
        render: preferences::overview,
    },
    StoryPage {
        name: "ActionBar",
        description: "A horizontal or vertical group of actions.",
        render: action_bar::overview,
    },
    StoryPage {
        name: "StepList",
        description: "Idle, active and completed steps.",
        render: step_list::overview,
    },
    StoryPage {
        name: "OptionCard",
        description: "A large selectable option with supporting copy.",
        render: option_card::overview,
    },
    StoryPage {
        name: "Table",
        description: "Aligned headers and data rows.",
        render: table::overview,
    },
    StoryPage {
        name: "CheckboxGroup",
        description: "Multiple choices with a mixed-state parent.",
        render: checkbox_group::overview,
    },
    StoryPage {
        name: "Link",
        description: "Inline navigation without a button box.",
        render: link::overview,
    },
    StoryPage {
        name: "Tag",
        description: "A selectable or dismissible annotation.",
        render: tag::overview,
    },
    StoryPage {
        name: "Splitter",
        description: "Resizable panels with pointer and keyboard control.",
        render: splitter::overview,
    },
];
