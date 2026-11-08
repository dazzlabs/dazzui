//! Pointer travel follows Base UI's safePolygon, including its short intent
//! timeout. Geometry is normalized to a popup above its trigger so all four
//! sides share the same edge and quadrilateral checks.

use super::group::DelayGroup;
use crate::components::popup::{Anchor, PopupState, Side};
use gpui::{App, Bounds, Entity, Pixels, Point, Task, Window};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Clone, Default)]
pub(super) struct HoverTravel(Rc<RefCell<State>>);

#[derive(Default)]
struct State {
    trip: Option<Trip>,
    timer: Option<Task<()>>,
}

struct Trip {
    origin: P,
    landed: bool,
    closing: bool,
    previous: Option<(P, Instant)>,
}

#[derive(Clone)]
pub(super) struct TravelContext {
    pub state: Entity<PopupState>,
    pub surface: Anchor,
    pub side: Side,
    pub gap: f32,
    pub group: Option<DelayGroup>,
    pub close_delay: Option<Duration>,
    pub default_close: Duration,
}

#[derive(Clone, Copy)]
struct P {
    x: f32,
    y: f32,
}
#[derive(Clone, Copy)]
struct Rect {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}
impl Rect {
    fn contains(self, point: P) -> bool {
        point.x >= self.left
            && point.x <= self.right
            && point.y >= self.top
            && point.y <= self.bottom
    }
}

impl TravelContext {
    fn geometry(&self, cx: &App) -> Option<(Side, Rect, Rect)> {
        let trigger = self.state.read(cx).anchor.bounds()?;
        let surface = self.surface.bounds()?;
        // Positioning can flip the requested side at a viewport edge.
        let side = match self.side {
            Side::Top | Side::Bottom if surface.center().y < trigger.center().y => Side::Top,
            Side::Top | Side::Bottom => Side::Bottom,
            _ if surface.center().x < trigger.center().x => Side::Left,
            _ => Side::Right,
        };
        let trigger = normalize_rect(trigger, side);
        let mut popup = normalize_rect(surface, side);
        // React's floating element includes the positioner's gap padding.
        popup.bottom += self.gap;
        Some((side, trigger, popup))
    }
    fn close(&self, window: &mut Window, cx: &mut App) {
        let close = self.group.as_ref().map_or_else(
            || self.close_delay.unwrap_or(self.default_close),
            |group| group.close_delay(self.close_delay, self.default_close),
        );
        self.state.update(cx, |state, cx| {
            state.hover(0, false, Duration::ZERO, close, window, cx)
        });
    }
    fn keep(&self, window: &mut Window, cx: &mut App) {
        self.state.update(cx, |state, cx| {
            state.hover(0, true, Duration::ZERO, Duration::ZERO, window, cx)
        });
    }
}

impl HoverTravel {
    pub fn reset(&self) {
        let mut state = self.0.borrow_mut();
        state.trip = None;
        state.timer = None;
    }

    pub fn leave(
        &self,
        point: Point<Pixels>,
        context: &TravelContext,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some((side, _, _)) = context.geometry(cx) else {
            context.close(window, cx);
            return;
        };
        self.0.borrow_mut().trip = Some(Trip {
            origin: normalize(point, side),
            landed: false,
            closing: false,
            previous: None,
        });
        self.motion(point, true, context, window, cx);
    }

    pub fn motion(
        &self,
        point: Point<Pixels>,
        leave: bool,
        context: &TravelContext,
        window: &mut Window,
        cx: &mut App,
    ) {
        if !context.state.read(cx).open {
            self.reset();
            return;
        }
        let Some((side, trigger, popup)) = context.geometry(cx) else {
            return;
        };
        let point = normalize(point, side);
        let now = cx.background_executor().now();
        let action = {
            let mut state = self.0.borrow_mut();
            state.timer = None;
            let Some(trip) = &mut state.trip else { return };
            if popup.contains(point) || trigger.contains(point) {
                trip.landed = true;
                trip.closing = false;
                Action::Keep
            } else if trip.closing {
                return;
            } else {
                let x = trip.origin.x;
                let y = trip.origin.y;
                let wider = popup.right - popup.left > trigger.right - trigger.left;
                let left = if wider { trigger.left } else { popup.left };
                let right = if wider { trigger.right } else { popup.right };
                // Opposite-edge exits must close even if a buffered polygon
                // happens to overlap the pointer.
                if y >= trigger.bottom - 1. {
                    Action::Close
                } else if point.x >= left
                    && point.x <= right
                    && point.y >= (trigger.top + 1.).min(popup.bottom - 1.)
                    && point.y <= (trigger.top + 1.).max(popup.bottom - 1.)
                {
                    Action::Keep
                } else if trip.landed {
                    Action::Close
                } else {
                    let slow = !leave
                        && trip.previous.is_some_and(|(previous, time)| {
                            let elapsed = now.duration_since(time).as_secs_f32() * 1000.;
                            (point.x - previous.x).powi(2) + (point.y - previous.y).powi(2)
                                < elapsed.powi(2) * 0.01
                        });
                    if !leave {
                        trip.previous = Some((point, now));
                    }
                    let from_right = x > (popup.left + popup.right) / 2.;
                    let offset = if wider { 0.25 } else { 2. };
                    let first_x = x + if wider || from_right { offset } else { -offset };
                    let second_x = x + if !wider && from_right {
                        offset
                    } else {
                        -offset
                    };
                    let cursor_y = y + if matches!(side, Side::Top | Side::Left) {
                        1.5
                    } else {
                        0.5
                    };
                    let near = popup.bottom - 0.5;
                    let left_y = if from_right || wider { near } else { popup.top };
                    let right_y = if !from_right || wider {
                        near
                    } else {
                        popup.top
                    };
                    if slow
                        || !in_quad(
                            point,
                            [
                                P {
                                    x: first_x,
                                    y: cursor_y,
                                },
                                P {
                                    x: second_x,
                                    y: cursor_y,
                                },
                                P {
                                    x: popup.left,
                                    y: left_y,
                                },
                                P {
                                    x: popup.right,
                                    y: right_y,
                                },
                            ],
                        )
                    {
                        Action::Close
                    } else {
                        Action::Wait
                    }
                }
            }
        };
        match action {
            Action::Keep => context.keep(window, cx),
            Action::Close => {
                if let Some(trip) = &mut self.0.borrow_mut().trip {
                    trip.closing = true;
                }
                context.close(window, cx);
            }
            Action::Wait => {
                let weak = Rc::downgrade(&self.0);
                let context = context.clone();
                let task = window.spawn(cx, async move |cx| {
                    cx.background_executor()
                        .timer(Duration::from_millis(40))
                        .await;
                    let Some(state) = weak.upgrade() else { return };
                    if let Some(trip) = &mut state.borrow_mut().trip {
                        trip.closing = true;
                    }
                    let _ = cx.update(|window, cx| context.close(window, cx));
                });
                self.0.borrow_mut().timer = Some(task);
            }
        }
    }
}

enum Action {
    Keep,
    Close,
    Wait,
}

fn normalize(point: Point<Pixels>, side: Side) -> P {
    let x = f32::from(point.x);
    let y = f32::from(point.y);
    match side {
        Side::Top => P { x, y },
        Side::Bottom => P { x, y: -y },
        Side::Left => P { x: y, y: x },
        Side::Right => P { x: y, y: -x },
    }
}
fn normalize_rect(bounds: Bounds<Pixels>, side: Side) -> Rect {
    let a = normalize(bounds.origin, side);
    let b = normalize(bounds.bottom_right(), side);
    Rect {
        left: a.x.min(b.x),
        top: a.y.min(b.y),
        right: a.x.max(b.x),
        bottom: a.y.max(b.y),
    }
}
fn in_quad(point: P, vertices: [P; 4]) -> bool {
    let mut inside = false;
    for index in 0..4 {
        let a = vertices[index];
        let b = vertices[(index + 1) % 4];
        if (a.y >= point.y) != (b.y >= point.y)
            && point.x <= (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x
        {
            inside = !inside;
        }
    }
    inside
}
