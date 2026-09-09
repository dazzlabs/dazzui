use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, ElementId, InteractiveElement, IntoElement, MouseButton,
    MouseDownEvent, ParentElement, Pixels, Point, RenderOnce, Styled, Window, canvas, deferred,
    div, px,
};

use crate::components::menu::{MenuItem, menu_item, menu_panel};
use crate::prelude::*;
use crate::{Tint, latched};

/// Where the panel was asked for, in the region's own coordinates.
///
/// A context menu has no trigger to hang from — it opens at the pointer — so
/// the position is the one thing it has to remember between frames, and the
/// region's bounds are what turn a window position into a local one.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct At {
    x: f32,
    y: f32,
    open: bool,
}

/// The menu a right-click opens, over the thing that was clicked.
///
/// It is the same menu a button opens — the panel and the rows are
/// [`menu_panel`] and [`menu_item`], so this shares the drawing rather than
/// resembling it. What is its own is where it opens: at the pointer rather
/// than against a trigger's edge.
///
/// Every affordance in a context menu must also be reachable somewhere a
/// pointer can see: a right-click is a shortcut, never the only door.
#[derive(IntoElement)]
pub struct ContextMenu {
    id: ElementId,
    items: Vec<MenuItem>,
    tint: Tint,
    /// The region that opens the menu. It is rendered as given — the listener
    /// is on the box around it — so a row, a card or a canvas keeps its own
    /// layout.
    child: Option<AnyElement>,
}

impl ContextMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        ContextMenu {
            id: id.into(),
            items: Vec::new(),
            tint: Tint::Neutral,
            child: None,
        }
    }

    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let ContextMenu {
            id,
            items,
            tint,
            child,
        } = self;

        latched::<At>(id.clone(), move |at, _window, _cx| {
            let state = at.get();
            // The region's own origin, captured by a canvas that paints
            // nothing: a mouse event carries a window position, and the panel
            // is placed inside this box.
            let origin: Rc<Cell<Point<Pixels>>> = Rc::new(Cell::new(Point::default()));

            div()
                .id("context-menu-region")
                .relative()
                .flex()
                .flex_none()
                .child(
                    canvas(
                        {
                            let origin = origin.clone();
                            move |bounds: Bounds<Pixels>, _, _| origin.set(bounds.origin)
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .size_full(),
                )
                .on_mouse_down(MouseButton::Right, {
                    let at = at.clone();
                    let origin = origin.clone();
                    move |event: &MouseDownEvent, window, _| {
                        let local = event.position - origin.get();
                        at.set(At {
                            x: f32::from(local.x),
                            y: f32::from(local.y),
                            open: true,
                        });
                        window.refresh();
                    }
                })
                .when_some(child, |el, child| el.child(child))
                .when(state.open, |el| {
                    el.child(deferred(
                        menu_panel(vars)
                            .absolute()
                            .left(px(state.x))
                            .top(px(state.y))
                            .occlude()
                            .on_mouse_down_out({
                                let at = at.clone();
                                move |_, window, _| {
                                    at.set(At {
                                        open: false,
                                        ..at.get()
                                    });
                                    window.refresh();
                                }
                            })
                            .children(items.into_iter().enumerate().map(|(index, item)| {
                                let at = at.clone();
                                menu_item(
                                    index,
                                    item,
                                    Rc::new(move |window: &mut Window| {
                                        at.set(At {
                                            open: false,
                                            ..at.get()
                                        });
                                        window.refresh();
                                    }),
                                    tint,
                                    vars,
                                )
                            })),
                    ))
                })
                .into_any_element()
        })
    }
}
