use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div,
};

use crate::components::Icon;
use crate::prelude::*;
use crate::{
    ActionListener, ControlPart, ControlSize, ControlState, ControlVariant, SharedValueListener,
    Tint, ValueListener, colors, pressable,
};

/// The box a [`Toggle`] and a [`ToggleGroup`]'s members are both drawn as.
///
/// One drawing shared rather than two that resemble each other, the way a
/// menu's panel is shared by the three menus. The group needs its own corners
/// and no edge of its own, which is all it passes in.
///
/// `glyph_only` squares the box at the control height, the way an
/// `IconButton` is — the padding goes, because a glyph has no side bearings to
/// pay for. That is what lets a latching glyph sit in a toolbar beside the
/// buttons that do not latch without being the one cell of a different
/// width.
#[allow(clippy::too_many_arguments)]
fn toggle_body(
    id: ElementId,
    label: AnyElement,
    glyph_only: bool,
    size: ControlSize,
    variant: ControlVariant,
    tint: Tint,
    pressed: bool,
    state: crate::PressableState,
    own_edge: bool,
    cx: &App,
) -> gpui::Stateful<gpui::Div> {
    let vars = cx.theme().vars;
    let metrics = vars.metrics(size);
    let phase = state.control_state();

    // Down: the system's own selection rule, applied. The tinted wash for the
    // colour, and the label a weight heavier — the weight is the half that
    // survives on a monochrome toolbar and for a reader who cannot tell the two
    // grades apart, which is why it is not decoration.
    //
    // The recipe stays tinted when the toggle is disabled and only the weight
    // drops. Falling back to the variant would leave a disabled `plain` toggle
    // with no surface at all, where the whole system flattens a dead control to
    // the groove.
    let drawn = if pressed {
        ControlVariant::Tinted
    } else {
        variant
    };
    let colors = vars.control_colors(drawn, tint, phase);
    let face = if pressed && state.enabled {
        metrics.content.weight(vars.label_strong.weight)
    } else {
        metrics.content
    };

    div()
        .id(id)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .h(metrics.size)
        .gap(metrics.gap)
        .map(|el| {
            if glyph_only {
                el.w(metrics.size)
            } else {
                el.px(metrics.padding_inline)
            }
        })
        .py(metrics.padding_block)
        .when(own_edge, |el| el.rounded(metrics.radius))
        .border(vars.spacing_px)
        .bg(colors.surface)
        .text_color(colors.content)
        .border_color(if own_edge {
            colors.border
        } else {
            colors::TRANSPARENT
        })
        .type_style(face)
        .map(|el| {
            if state.enabled {
                el.cursor_pointer()
            } else {
                el.cursor_not_allowed()
            }
        })
        .child(label)
}

/// A button that stays down.
///
/// The distinction it draws against its neighbours is worth stating, because
/// three components here are one press away from each other. A
/// [`Switch`](crate::Switch) commits immediately and reads as a setting. A
/// [`SegmentedControl`](crate::SegmentedControl) is a choice *between* its
/// segments and always has one. This is neither: a single property that is on
/// or off — bold, wrap, show whitespace — in a row where none of them may be
/// on.
#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    label: SharedString,
    /// A leading glyph. On its own — with an empty label — it is the whole
    /// label, and the toggle is drawn square at the control height.
    icon: Option<Icon>,
    size: ControlSize,
    variant: ControlVariant,
    tint: Tint,
    pressed: bool,
    disabled: bool,
    on_toggle: Option<ActionListener>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Toggle {
            id: id.into(),
            label: label.into(),
            icon: None,
            size: ControlSize::Tiny,
            variant: ControlVariant::Plain,
            // Neutral for the reason `IconButton` is: a toolbar of these is
            // chrome, and a row of accented glyphs reads as a row of actions.
            tint: Tint::Neutral,
            pressed: false,
            disabled: false,
            on_toggle: None,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ControlVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_toggle(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let Toggle {
            id,
            label,
            icon,
            size,
            variant,
            tint,
            pressed,
            disabled,
            on_toggle,
        } = self;
        let on_toggle: Option<crate::SharedActionListener> = on_toggle.map(std::rc::Rc::from);

        pressable(id.clone(), !disabled, move |state, _, cx| {
            let listener = on_toggle.clone();
            let glyph_only = label.is_empty() && icon.is_some();
            let body: AnyElement = match (icon, label.is_empty()) {
                (Some(icon), true) => icon.into_any_element(),
                (Some(icon), false) => div()
                    .flex()
                    .items_center()
                    .child(icon)
                    .child(label)
                    .into_any_element(),
                (None, _) => label.into_any_element(),
            };
            toggle_body(
                id, body, glyph_only, size, variant, tint, pressed, state, true, cx,
            )
            .when_some(listener, |el, listener| {
                el.on_click(move |_, window, cx| listener(window, cx))
            })
            .into_any_element()
        })
    }
}

/// One member of a [`ToggleGroup`].
pub struct ToggleItem {
    pub value: SharedString,
    pub label: SharedString,
    pub disabled: bool,
    /// Draws the member as a lone glyph; the label stays its accessible name.
    pub icon: Option<Icon>,
}

impl ToggleItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        ToggleItem {
            value: value.into(),
            label: label.into(),
            disabled: false,
            icon: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Several toggles that belong together, joined into one control.
///
/// Joined rather than merely adjacent: the corners are rounded at the ends of
/// the run only and the shared edges collapse to one, which is what says the
/// group is one thing with several states rather than three buttons that
/// happen to touch. The ring belongs to the group and the members draw none of
/// their own, so a pair's shared rule is a single hairline rather than two
/// stacked.
///
/// It parts from [`SegmentedControl`](crate::SegmentedControl) on exactly one
/// point: several on at once, or none. A group that must always have exactly
/// one is that component, and drawing it with this one would let the reader
/// clear a choice that cannot be cleared.
#[derive(IntoElement)]
pub struct ToggleGroup {
    id: ElementId,
    items: Vec<ToggleItem>,
    value: Vec<SharedString>,
    size: ControlSize,
    variant: ControlVariant,
    tint: Tint,
    disabled: bool,
    on_toggle: Option<ValueListener<SharedString>>,
}

impl ToggleGroup {
    pub fn new(id: impl Into<ElementId>, items: Vec<ToggleItem>) -> Self {
        ToggleGroup {
            id: id.into(),
            items,
            value: Vec::new(),
            size: ControlSize::Small,
            variant: ControlVariant::Recessed,
            tint: Tint::Primary,
            disabled: false,
            on_toggle: None,
        }
    }

    pub fn value(mut self, value: Vec<SharedString>) -> Self {
        self.value = value;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ControlVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_toggle(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let metrics = vars.metrics(self.size);
        let ToggleGroup {
            id,
            items,
            value,
            size,
            variant,
            tint,
            disabled,
            on_toggle,
        } = self;
        let on_toggle: Option<SharedValueListener<SharedString>> = on_toggle.map(std::rc::Rc::from);

        // The run's own edge, read at rest: one ring for the whole group rather
        // than one per member, so the corners land only at the two ends.
        let edge = vars.control_color(
            variant,
            ControlPart::Border,
            tint,
            if disabled {
                ControlState::Disabled
            } else {
                ControlState::Normal
            },
        );

        let last = items.len().saturating_sub(1);
        div()
            .id(id)
            .flex()
            .flex_none()
            .items_center()
            .overflow_hidden()
            .rounded(metrics.radius)
            .border(vars.spacing_px)
            .border_color(edge)
            .children(
                items
                    .into_iter()
                    .enumerate()
                    .flat_map(move |(index, item)| {
                        let down = value.contains(&item.value);
                        let live = !disabled && !item.disabled;
                        let member_id =
                            ElementId::from(SharedString::from(format!("toggle-group-{index}")));
                        let listener = on_toggle.clone();
                        let key = item.value.clone();

                        let member = pressable(member_id.clone(), live, move |state, _, cx| {
                            let glyph_only = item.icon.is_some();
                            let body = match item.icon {
                                Some(icon) => icon.size(metrics.content.size).into_any_element(),
                                None => item.label.into_any_element(),
                            };
                            toggle_body(
                                member_id, body, glyph_only, size, variant, tint, down, state,
                                false, cx,
                            )
                            .when_some(listener, |el, listener| {
                                el.on_click(move |_, window, cx| listener(&key, window, cx))
                            })
                            .into_any_element()
                        })
                        .into_any_element();

                        // The rule between a pair, drawn once by the group.
                        let rule = (index < last).then(|| {
                            div()
                                .flex_none()
                                .w(vars.spacing_px)
                                .h_full()
                                .bg(edge)
                                .into_any_element()
                        });
                        std::iter::once(member).chain(rule)
                    }),
            )
    }
}
