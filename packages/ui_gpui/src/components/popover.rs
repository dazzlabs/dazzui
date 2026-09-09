use gpui::{
    AnyElement, App, Div, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, deferred, div, relative,
};

use crate::prelude::*;
use crate::{hairline, latched};

/// Which side of the trigger the panel opens on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// Which edge of the trigger the panel lines up with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
}

/// What builds the thing that opens a popover. It is handed the open state, so
/// a trigger can show that its panel is up.
pub type PopoverTrigger = Box<dyn FnOnce(bool, &mut App) -> AnyElement + 'static>;

/// The floating sheet a popover and a preview card both wear.
///
/// It is the menu panel's materials — the overlay surface, the strong
/// hairline, the deep shadow — padded for prose rather than for rows: a sheet
/// built for 26px rows pads by a single step, and a paragraph in that would
/// sit on its own edge.
///
/// The width is deliberately not set. A menu has a minimum because a menu
/// narrower than its trigger reads as broken; a popover is as wide as what is
/// in it, and a caller who knows better sets one.
pub(crate) fn popover_panel(vars: &'static crate::ThemeVariables) -> Div {
    div()
        .flex()
        .flex_col()
        .p(vars.spacing_4 + vars.spacing_0_5)
        .rounded(vars.radius_large)
        .border(hairline(vars))
        .border_color(vars.color_border_strong)
        .bg(vars.color_surface_overlay)
        .token_shadow(vars.shadow_lg)
        .type_style(vars.body_small)
        .text_color(vars.color_content)
}

/// Where a panel hangs, given the side and the alignment it was asked for.
pub(crate) fn anchor(panel: Div, side: PopoverSide, align: PopoverAlign, gap: gpui::Pixels) -> Div {
    let panel = match side {
        PopoverSide::Bottom => panel.top_full().mt(gap),
        PopoverSide::Top => panel.bottom_full().mb(gap),
        PopoverSide::Left => panel.right_full().mr(gap),
        PopoverSide::Right => panel.left_full().ml(gap),
    };

    match (side, align) {
        (PopoverSide::Top | PopoverSide::Bottom, PopoverAlign::Start) => panel.left_0(),
        (PopoverSide::Top | PopoverSide::Bottom, PopoverAlign::End) => panel.right_0(),
        (PopoverSide::Top | PopoverSide::Bottom, PopoverAlign::Center) => panel,
        (PopoverSide::Left | PopoverSide::Right, PopoverAlign::Start) => panel.top_0(),
        (PopoverSide::Left | PopoverSide::Right, PopoverAlign::End) => panel.bottom_0(),
        (PopoverSide::Left | PopoverSide::Right, PopoverAlign::Center) => panel,
    }
}

/// A panel of anything, anchored to what opened it.
///
/// It is the sibling of [`Menu`](crate::Menu) rather than of
/// [`Dialog`](crate::Dialog): it wears the same kind of floating sheet and is
/// positioned against its trigger, but it holds content rather than rows.
/// Reach for a dialog instead when the reader has to finish before doing
/// anything else — a popover closes on an outside press, which makes it the
/// wrong home for something half-typed.
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    /// Names the panel as well as heading it.
    title: Option<SharedString>,
    /// A line under the title.
    description: Option<SharedString>,
    child: Option<AnyElement>,
    side: PopoverSide,
    align: PopoverAlign,
    /// Overrides the panel's own width, which is otherwise its content's.
    width: Option<gpui::Pixels>,
    trigger: Option<PopoverTrigger>,
}

impl Popover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Popover {
            id: id.into(),
            title: None,
            description: None,
            child: None,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
            width: None,
            trigger: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = Some(width);
        self
    }

    /// What opens the panel. It is handed the open state, so a trigger can
    /// show that its panel is up.
    pub fn trigger(mut self, trigger: impl FnOnce(bool, &mut App) -> AnyElement + 'static) -> Self {
        self.trigger = Some(Box::new(trigger));
        self
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Popover {
            id,
            title,
            description,
            child,
            side,
            align,
            width,
            trigger,
        } = self;

        latched::<bool>(id.clone(), move |open, _window, cx| {
            let is_open = open.get();
            let trigger = trigger.map(|trigger| trigger(is_open, cx));
            let heading = title.is_some() || description.is_some();

            div()
                .relative()
                .flex()
                .flex_none()
                .when_some(trigger, |el, trigger| {
                    el.child(
                        div()
                            .id("popover-trigger")
                            .on_click({
                                let open = open.clone();
                                move |_, window, _| {
                                    open.set(!open.get());
                                    window.refresh();
                                }
                            })
                            .child(trigger),
                    )
                })
                .when(is_open, |el| {
                    el.child(deferred(
                        anchor(popover_panel(vars).absolute(), side, align, vars.spacing_1)
                            .when_some(width, |el, width| el.w(width))
                            .occlude()
                            .on_mouse_down_out({
                                let open = open.clone();
                                move |_, window, _| {
                                    open.set(false);
                                    window.refresh();
                                }
                            })
                            .when_some(title, |el, title| {
                                el.child(
                                    div()
                                        .type_style(vars.label_large)
                                        .text_color(vars.color_content)
                                        .child(title),
                                )
                            })
                            .when_some(description, |el, description| {
                                el.child(
                                    div()
                                        .mt(vars.spacing_1)
                                        .type_style(vars.body_small)
                                        .line_height(relative(1.6))
                                        .text_color(vars.color_content_secondary)
                                        .child(description),
                                )
                            })
                            // The run between the head and whatever the panel is
                            // actually for. It is a margin above the body rather
                            // than a gap on the panel, so a popover that is only a
                            // form has no leading space to trim.
                            .when_some(child, |el, child| {
                                el.child(
                                    div().when(heading, |el| el.mt(vars.spacing_3)).child(child),
                                )
                            }),
                    ))
                })
                .into_any_element()
        })
    }
}
