use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::components::Icon;
use crate::prelude::*;
use crate::{ActionListener, ControlPart, ControlState, ControlVariant, Tint, colors, pressable};

/// `medium` in a sidebar, `small` in the narrower rail.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavItemSize {
    Small,
    #[default]
    Medium,
}

/// One row of a navigation column.
///
/// The idle row rests in the secondary ink at the quiet weight — a column of
/// full-ink rows would leave the selected one nothing to win by — and its
/// hover is the accent at 8%, a preview of the filled selection sitting far
/// below it. A menu item or a table row hovers neutral instead; the
/// difference is what the selected state is.
#[derive(IntoElement)]
pub struct NavItem {
    id: ElementId,
    label: SharedString,
    /// Marks this row as the page or pane currently shown.
    active: bool,
    /// Leading glyph. It follows the row's colour, so it tints with selection.
    icon: Option<Icon>,
    size: NavItemSize,
    tint: Tint,
    disabled: bool,
    /// An action adds rather than selects, so it is printed in the ramp's
    /// text grade instead of filled with it, and never takes the
    /// current-page state.
    action: bool,
    on_click: Option<ActionListener>,
}

impl NavItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        NavItem {
            id: id.into(),
            label: label.into(),
            active: false,
            icon: None,
            size: NavItemSize::Medium,
            tint: Tint::Primary,
            disabled: false,
            action: false,
            on_click: None,
        }
    }

    /// The trailing action pinned to the foot of a navigation column — "New",
    /// "Add a provider". Shaped like a row so the column reads as one list.
    pub fn action(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        NavItem {
            action: true,
            ..NavItem::new(id, label)
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn size(mut self, size: NavItemSize) -> Self {
        self.size = size;
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

    pub fn on_click(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for NavItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let NavItem {
            id,
            label,
            active,
            icon,
            size,
            tint,
            disabled,
            action,
            on_click,
        } = self;

        let filled_surface = vars.control_color(
            ControlVariant::Filled,
            ControlPart::Surface,
            tint,
            ControlState::Normal,
        );
        let filled_content = vars.control_color(
            ControlVariant::Filled,
            ControlPart::Content,
            tint,
            ControlState::Normal,
        );
        let wash = vars.control_color(
            ControlVariant::Plain,
            ControlPart::Surface,
            tint,
            ControlState::Hovered,
        );
        let action_ink = vars.control_color(
            ControlVariant::Plain,
            ControlPart::Content,
            tint,
            ControlState::Normal,
        );
        let disabled_ink = vars.control_color(
            ControlVariant::Normal,
            ControlPart::Content,
            tint,
            ControlState::Disabled,
        );
        // The sidebar row: 12px on 8px pads — 28px, the regular control
        // height. The rail row is a step tighter on both axes.
        let padding_block = match size {
            NavItemSize::Medium => vars.spacing_2,
            NavItemSize::Small => vars.menu_item_padding,
        };

        pressable(id.clone(), !disabled, move |state, _, _| {
            div()
                .id(id)
                .flex()
                .items_center()
                .w_full()
                .gap(vars.spacing_1_5)
                .py(padding_block)
                .px(vars.spacing_2_5)
                .rounded(vars.radius_small)
                .type_style(vars.label_quiet)
                .line_height(relative(1.0))
                .map(|el| {
                    if !state.enabled {
                        el.cursor_not_allowed().text_color(disabled_ink)
                    } else if active {
                        el.cursor_pointer()
                            .bg(filled_surface)
                            .text_color(filled_content)
                    } else if state.hovered || state.pressed {
                        el.cursor_pointer().bg(wash).text_color(if action {
                            action_ink
                        } else {
                            vars.color_content
                        })
                    } else {
                        el.cursor_pointer()
                            .bg(colors::TRANSPARENT)
                            .text_color(if action {
                                action_ink
                            } else {
                                vars.color_content_nav
                            })
                    }
                })
                .when(action, |el| el.font_weight(vars.label_medium.weight))
                .when_some(icon, |el, icon| {
                    // The glyph is taller than the label, so it is boxed to
                    // the row's line height and allowed to overflow rather
                    // than stretching the row.
                    el.child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .w(vars.spacing_4)
                            .child(icon.size(vars.spacing_4)),
                    )
                })
                .child(label)
                .when_some(on_click.filter(|_| state.enabled), |el, listener| {
                    el.on_click(move |_, window, cx| listener(window, cx))
                })
                .into_any_element()
        })
    }
}
