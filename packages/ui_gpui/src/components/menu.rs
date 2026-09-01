use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, deferred, div, relative,
};

use crate::assets::icons;
use crate::components::Icon;
use crate::prelude::*;
use crate::{
    ActionListener, ControlPart, ControlState, ControlVariant, SharedActionListener, Tint, colors,
    hairline, latched, pressable,
};

/// What builds the thing that opens a menu. It is handed the open state, so
/// a trigger can show that it is held down.
pub type MenuTrigger = Box<dyn FnOnce(bool, &mut App) -> AnyElement + 'static>;

/// Which edge of the trigger the panel hangs from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuAlign {
    Start,
    #[default]
    End,
}

/// One row of a menu.
pub struct MenuItem {
    label: SharedString,
    icon: Option<Icon>,
    /// Trailing hint — the keys that also run this item.
    shortcut: Option<SharedString>,
    /// Radio-style check state. Set it on every item of a group, including
    /// the unchecked ones, so the leading column stays aligned.
    checked: Option<bool>,
    disabled: bool,
    on_select: Option<ActionListener>,
}

impl MenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        MenuItem {
            label: label.into(),
            icon: None,
            shortcut: None,
            checked: None,
            disabled: false,
            on_select: None,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(listener));
        self
    }
}

/// A popover menu.
///
/// It closes on an outside pointer press and when an item is chosen. The
/// panel is deferred, so it paints over whatever it hangs across rather than
/// in the order it was built.
#[derive(IntoElement)]
pub struct Menu {
    id: ElementId,
    items: Vec<MenuItem>,
    align: MenuAlign,
    tint: Tint,
    disabled: bool,
    trigger: Option<MenuTrigger>,
}

impl Menu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Menu {
            id: id.into(),
            items: Vec::new(),
            align: MenuAlign::End,
            tint: Tint::Neutral,
            disabled: false,
            trigger: None,
        }
    }

    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn align(mut self, align: MenuAlign) -> Self {
        self.align = align;
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

    /// What opens the menu. It is handed the open state, so a trigger can
    /// show that it is held down.
    pub fn trigger(mut self, trigger: impl FnOnce(bool, &mut App) -> AnyElement + 'static) -> Self {
        self.trigger = Some(Box::new(trigger));
        self
    }
}

impl RenderOnce for Menu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Menu {
            id,
            items,
            align,
            tint,
            disabled,
            trigger,
        } = self;

        latched(id.clone(), move |open, window, cx| {
            let rule = hairline(vars, window);
            let is_open = open.get() && !disabled;
            let trigger = trigger.map(|trigger| trigger(is_open, cx));

            div()
                .relative()
                .flex()
                .flex_none()
                .when_some(trigger, |el, trigger| {
                    el.child(
                        div()
                            .id("menu-trigger")
                            .when(!disabled, |el| {
                                el.on_click({
                                    let open = open.clone();
                                    move |_, window, _| {
                                        open.set(!open.get());
                                        window.refresh();
                                    }
                                })
                            })
                            .child(trigger),
                    )
                })
                .when(is_open, |el| {
                    el.child(deferred(
                        div()
                            .absolute()
                            .top_full()
                            .map(|el| match align {
                                MenuAlign::Start => el.left_0(),
                                MenuAlign::End => el.right_0(),
                            })
                            .mt(vars.spacing_1)
                            .min_w(vars.menu_min_width)
                            .p(vars.spacing_1)
                            .rounded(vars.radius_large)
                            .border(rule)
                            .bg(vars.color_surface)
                            .border_color(vars.color_border_strong)
                            .token_shadow(vars.shadow_lg)
                            .occlude()
                            .on_mouse_down_out({
                                let open = open.clone();
                                move |_, window, _| {
                                    open.set(false);
                                    window.refresh();
                                }
                            })
                            .children(items.into_iter().enumerate().map(|(index, item)| {
                                menu_item(index, item, open.clone(), tint, vars)
                            })),
                    ))
                })
                .into_any_element()
        })
    }
}

/// One row.
///
/// The wash is the `normal` recipe's, not `plain`'s. Both have no resting
/// fill, but `plain` is the quiet *accent* control — its wash comes off the
/// tinted ramp, which on a row would land heavier than a tinted selection
/// sitting one step lower. A row's wash is neutral, and it holds while
/// pressed: a menu item has no pressed look of its own.
fn menu_item(
    index: usize,
    item: MenuItem,
    open: std::rc::Rc<std::cell::Cell<bool>>,
    tint: Tint,
    vars: &'static crate::ThemeVariables,
) -> impl IntoElement {
    let MenuItem {
        label,
        icon,
        shortcut,
        checked,
        disabled,
        on_select,
    } = item;
    let on_select: Option<SharedActionListener> = on_select.map(std::rc::Rc::from);

    pressable(
        SharedString::from(format!("menu-item-{index}")),
        !disabled,
        move |state, _, _| {
            let wash = vars.control_color(
                ControlVariant::Normal,
                ControlPart::Surface,
                tint,
                ControlState::Hovered,
            );

            div()
                .id(SharedString::from(format!("menu-item-body-{index}")))
                .flex()
                .items_center()
                .gap(vars.spacing_2)
                .w_full()
                .py(vars.menu_item_padding)
                .px(vars.spacing_2)
                // The item's corner is the panel's minus its padding, so the
                // hover wash stays concentric with the panel edge.
                .rounded(vars.radius_large - vars.spacing_1)
                .type_style(vars.label_quiet)
                .line_height(relative(1.0))
                .map(|el| {
                    if disabled {
                        el.cursor_not_allowed().text_color(vars.control_color(
                            ControlVariant::Normal,
                            ControlPart::Content,
                            tint,
                            ControlState::Disabled,
                        ))
                    } else {
                        el.cursor_pointer().text_color(vars.color_content).bg(
                            if state.hovered || state.pressed {
                                wash
                            } else {
                                colors::TRANSPARENT
                            },
                        )
                    }
                })
                // The checkmark column holds its width whether or not the
                // item is checked, so the labels of a radio group stay on one
                // left edge.
                .when_some(checked, |el, checked| {
                    el.child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .w(vars.spacing_3)
                            .when(checked, |el| {
                                el.child(
                                    Icon::new(icons::CHECKMARK_12_FILLED)
                                        .size(vars.label_small.size),
                                )
                            }),
                    )
                })
                .when_some(icon, |el, icon| {
                    el.child(icon.size(vars.spacing_4).color(vars.color_content_muted))
                })
                .child(div().flex_1().child(label.clone()))
                // The shortcut is a faint hint in the display cut — never a
                // boxed key cap; a cap in a 26px row would be furniture.
                .when_some(shortcut.clone(), |el, shortcut| {
                    el.child(
                        div()
                            .flex_none()
                            .type_style(vars.label_small)
                            .line_height(relative(1.0))
                            .font_family(vars.label_strong.font_family())
                            .text_color(vars.color_content_faint)
                            .child(shortcut),
                    )
                })
                .when(!disabled, |el| {
                    let open = open.clone();
                    let on_select = on_select.clone();
                    el.on_click(move |_, window, cx| {
                        open.set(false);
                        window.refresh();
                        if let Some(listener) = on_select.as_ref() {
                            listener(window, cx);
                        }
                    })
                })
                .into_any_element()
        },
    )
}
