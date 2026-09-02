use dazzui::assets::icons;
use dazzui::{
    Badge, BadgeVariant, Button, ControlSize, ControlVariant, Icon, IconButton, KeyCap,
    KeyCapVariant, PillTabs, SegmentedControl, Tint,
};
use gpui::{AnyElement, Context, Entity, IntoElement, ParentElement, SharedString};

use crate::gallery::{PageTitle, Row, Specimen};
use crate::{Playground, act, act_with};

/// The control vocabulary: every recipe, every size, and the annotations that
/// sit beside them.
pub fn controls(
    state: &Playground,
    this: &Entity<Playground>,
    _cx: &mut Context<Playground>,
) -> Vec<AnyElement> {
    let mut variants = Row::new();
    for variant in ControlVariant::ALL {
        variants = variants
            .child(Button::new(variant.name(), capitalize(variant.name())).variant(variant));
    }

    let mut sizes = Row::new();
    for size in ControlSize::ALL {
        sizes = sizes.child(
            Button::new(size.name(), capitalize(size.name()))
                .size(size)
                .variant(ControlVariant::Filled),
        );
    }

    let mut tints = Row::new();
    for tint in Tint::ALL {
        tints = tints.child(
            Button::new(tint.name(), capitalize(tint.name()))
                .variant(ControlVariant::Tinted)
                .tint(tint),
        );
    }

    let mut disabled = Row::new();
    for variant in ControlVariant::ALL {
        disabled = disabled.child(
            Button::new(
                SharedString::from(format!("disabled-{}", variant.name())),
                capitalize(variant.name()),
            )
            .variant(variant)
            .disabled(true),
        );
    }

    vec![
        PageTitle::new(
            "Controls",
            "A variant picks the recipe, a tint picks the ramp it resolves against.",
        )
        .into_any_element(),
        Specimen::new("Variants")
            .note("recessed · normal · filled · tinted · outlined · plain")
            .child(variants)
            .into_any_element(),
        Specimen::new("Sizes")
            .note("geometry and type both come off the size profile")
            .child(sizes)
            .into_any_element(),
        Specimen::new("Tints")
            .note("one recipe table, six ramps")
            .child(tints)
            .into_any_element(),
        Specimen::new("Disabled")
            .note("every variant flattens to the same groove")
            .child(disabled)
            .into_any_element(),
        Specimen::new("With a shortcut")
            .child(
                Row::new()
                    .child(
                        Button::new("save", "Save")
                            .variant(ControlVariant::Filled)
                            .shortcut("⌘S"),
                    )
                    .child(Button::new("cancel", "Cancel").shortcut("esc")),
            )
            .into_any_element(),
        Specimen::new("Icon buttons")
            .note("quiet chrome until touched; the pin holds an on-state")
            .child(
                Row::new()
                    .child(IconButton::new(
                        "back",
                        "Back",
                        Icon::new(icons::CHEVRON_LEFT_16_REGULAR),
                    ))
                    .child(IconButton::new(
                        "forward",
                        "Forward",
                        Icon::new(icons::CHEVRON_RIGHT_16_REGULAR),
                    ))
                    .child(
                        IconButton::new("pin", "Pin", Icon::new(icons::PIN_16_REGULAR))
                            .active(state.pinned)
                            .on_click({
                                let toggle = act(this, |state, _, _| state.pinned = !state.pinned);
                                move |_, window, cx| toggle(window, cx)
                            }),
                    )
                    .child(
                        IconButton::new("delete", "Delete", Icon::new(icons::DELETE_16_REGULAR))
                            .variant(ControlVariant::Tinted)
                            .tint(Tint::Danger),
                    )
                    .child(
                        IconButton::new("star", "Disabled", Icon::new(icons::STAR_16_REGULAR))
                            .disabled(true),
                    ),
            )
            .into_any_element(),
        Specimen::new("Badges")
            .note("annotations, so they never react to the pointer")
            .child(
                Row::new()
                    .child(Badge::new("Filled").variant(BadgeVariant::Filled))
                    .child(Badge::new("Tinted").variant(BadgeVariant::Tinted))
                    .child(Badge::new("Outlined").variant(BadgeVariant::Outlined))
                    .child(Badge::new("Raised").variant(BadgeVariant::Raised))
                    .child(Badge::new("Beta").tint(Tint::Warning))
                    .child(Badge::new("Failed").tint(Tint::Danger)),
            )
            .into_any_element(),
        Specimen::new("Keys")
            .child(
                Row::new()
                    .child(KeyCap::new("⌘K"))
                    .child(KeyCap::new("⇧⌘P").variant(KeyCapVariant::Strong))
                    .child(KeyCap::new("Esc").variant(KeyCapVariant::Key)),
            )
            .into_any_element(),
        Specimen::new("Pill tabs")
            .child(
                PillTabs::new("tabs", state.tab.clone())
                    .counted_item("all", "All", 24)
                    .counted_item("open", "Open", 3)
                    .counted_item("done", "Done", 21)
                    .item("archived", "Archived")
                    .on_select(act_with(this, |state, value: &SharedString, _, _| {
                        state.tab = value.clone();
                    })),
            )
            .into_any_element(),
        Specimen::new("Segmented control")
            .child(
                SegmentedControl::new("mode", state.mode.clone())
                    .item("design", "Design")
                    .item("code", "Code")
                    .item("preview", "Preview")
                    .on_select(act_with(this, |state, value: &SharedString, _, _| {
                        state.mode = value.clone();
                    })),
            )
            .into_any_element(),
    ]
}

/// The playground prints an axis's own name, which the system spells in
/// lower case.
pub fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
