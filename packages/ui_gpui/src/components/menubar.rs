use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, deferred, div, relative,
};

use crate::components::menu::{MenuItem, menu_item, menu_panel};
use crate::prelude::*;
use crate::{
    ControlPart, ControlSize, ControlState, ControlVariant, Tint, colors, latched, pressable,
};

/// One name on the bar, and the rows it opens.
pub struct MenubarMenu {
    /// The name on the bar — File, Edit, View.
    label: SharedString,
    items: Vec<MenuItem>,
    disabled: bool,
}

impl MenubarMenu {
    pub fn new(label: impl Into<SharedString>) -> Self {
        MenubarMenu {
            label: label.into(),
            items: Vec::new(),
            disabled: false,
        }
    }

    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Which name on the bar is open, if any. `usize::MAX` is closed, so the whole
/// state is one `Copy` word and fits a [`latched`] cell.
const CLOSED: usize = usize::MAX;

/// The application menu bar: several menus that behave as one.
///
/// That is the whole reason it exists rather than a row of [`Menu`](crate::Menu)s.
/// Once one of them is open, moving the pointer along the bar opens the next
/// without a second click — the behaviour every desktop menu bar has and none
/// of them documents.
///
/// Each panel is a [`menu_panel`] of [`menu_item`]s, so the drawing is the
/// same everywhere; the bar adds only its triggers.
#[derive(IntoElement)]
pub struct Menubar {
    id: ElementId,
    menus: Vec<MenubarMenu>,
    tint: Tint,
}

impl Menubar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Menubar {
            id: id.into(),
            menus: Vec::new(),
            tint: Tint::Neutral,
        }
    }

    pub fn menu(mut self, menu: MenubarMenu) -> Self {
        self.menus.push(menu);
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl RenderOnce for Menubar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Menubar { id, menus, tint } = self;
        let metrics = vars.metrics(ControlSize::Small);

        latched::<usize>(id.clone(), move |open, _window, _cx| {
            // A fresh latch reads zero, which would mean "File is open". The
            // bar starts shut, so the stored value is offset by one and zero
            // is what "never touched" means.
            let stored = open.get();
            let current = if stored == 0 { CLOSED } else { stored - 1 };

            let mut bar = div().flex().items_center().gap(vars.spacing_0_5);

            for (index, menu) in menus.into_iter().enumerate() {
                // Built here rather than inside the builder: the id is moved
                // into one closure per menu, and a loop cannot move it twice.
                let trigger_id = SharedString::from(format!("{id:?}-trigger-{index}"));
                let name_id = SharedString::from(format!("{id:?}-name-{index}"));
                let MenubarMenu {
                    label,
                    items,
                    disabled,
                } = menu;
                let is_open = current == index;
                let set = {
                    let open = open.clone();
                    Rc::new(move |next: usize, window: &mut Window| {
                        open.set(if next == CLOSED { 0 } else { next + 1 });
                        window.refresh();
                    })
                };

                bar = bar.child(
                    div()
                        .relative()
                        .child(pressable(trigger_id, !disabled, {
                            let set = set.clone();
                            move |state, _, _| {
                                // Chrome rather than a control: no box at
                                // rest, and a wash only under the pointer
                                // or while its own menu is open. An open
                                // trigger keeps the wash — a bar where the
                                // open menu's name looked idle would leave
                                // the panel belonging to nothing.
                                let washed = !disabled && (is_open || state.hovered);

                                div()
                                    .id(name_id.clone())
                                    .flex()
                                    .items_center()
                                    .h(metrics.size)
                                    .px(vars.spacing_2)
                                    .rounded(vars.radius_small)
                                    .type_style(metrics.content)
                                    .line_height(relative(1.0))
                                    .text_color(if disabled {
                                        vars.control_color(
                                            ControlVariant::Normal,
                                            ControlPart::Content,
                                            tint,
                                            ControlState::Disabled,
                                        )
                                    } else {
                                        vars.color_content
                                    })
                                    .bg(if washed {
                                        vars.control_color(
                                            ControlVariant::Normal,
                                            ControlPart::Surface,
                                            tint,
                                            ControlState::Hovered,
                                        )
                                    } else {
                                        colors::TRANSPARENT
                                    })
                                    .child(label.clone())
                                    .when(!disabled, |el| {
                                        let press = set.clone();
                                        el.on_click(move |_, window, _| {
                                            press(if is_open { CLOSED } else { index }, window);
                                        })
                                        // Once one is open the bar is
                                        // armed: crossing another name
                                        // opens it, which is what makes
                                        // this a bar rather than a row of
                                        // separate menus.
                                        .when(
                                            current != CLOSED && !is_open,
                                            |el| {
                                                let set = set.clone();
                                                el.on_hover(move |hovered, window, _| {
                                                    if *hovered {
                                                        set(index, window);
                                                    }
                                                })
                                            },
                                        )
                                    })
                                    .into_any_element()
                            }
                        }))
                        .when(is_open, |el| {
                            el.child(deferred(
                                menu_panel(vars)
                                    .absolute()
                                    .top_full()
                                    .left_0()
                                    .mt(vars.spacing_1)
                                    .occlude()
                                    .on_mouse_down_out({
                                        let set = set.clone();
                                        move |_, window, _| set(CLOSED, window)
                                    })
                                    .children(items.into_iter().enumerate().map(|(row, item)| {
                                        let set = set.clone();
                                        menu_item(
                                            row,
                                            item,
                                            Rc::new(move |window: &mut Window| set(CLOSED, window)),
                                            tint,
                                            vars,
                                        )
                                    })),
                            ))
                        }),
                );
            }

            bar.into_any_element()
        })
    }
}
