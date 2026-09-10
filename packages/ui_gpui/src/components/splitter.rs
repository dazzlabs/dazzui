use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    RenderOnce, SharedString, Styled, Window, canvas, div, px, relative,
};

use crate::prelude::*;
use crate::{ControlPart, ControlState, ControlVariant, Tint, latched, pressable};

/// The drag a seam starts. GPUI keys a drag by its payload type, so this marks
/// the splitter's own.
struct SplitterDrag;

/// Which way the panels are laid out, not which way a handle runs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitterOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// How much wider the rule is drawn while the pointer is on it.
const GRIP_SWELL: f32 = 2.;

/// As many panels as a window has columns. Past this a splitter is a layout
/// problem rather than a widening problem, and the shares are a fixed array so
/// the whole state stays `Copy` — which is what [`latched`] holds.
const MAX_PANELS: usize = 8;

/// One region of a [`Splitter`].
///
/// A description as much as an element: the splitter counts these to know how
/// many handles to draw and reads their constraints to know what each handle
/// may do.
pub struct SplitterPanel {
    child: AnyElement,
    default_size: f32,
    min_size: f32,
    max_size: f32,
}

impl SplitterPanel {
    pub fn new(child: impl IntoElement) -> Self {
        SplitterPanel {
            child: child.into_any_element(),
            default_size: 1.,
            min_size: 0.,
            max_size: 100.,
        }
    }

    /// Share of the run at rest. The shares are normalised, so three panels
    /// written `50`, `25`, `25` and three written `2`, `1`, `1` lay out
    /// identically.
    pub fn default_size(mut self, size: f32) -> Self {
        self.default_size = size;
        self
    }

    /// Floor and ceiling, as percentages of the whole run.
    pub fn min_size(mut self, size: f32) -> Self {
        self.min_size = size;
        self
    }

    pub fn max_size(mut self, size: f32) -> Self {
        self.max_size = size;
        self
    }
}

/// What one splitter remembers between frames.
#[derive(Clone, Copy, Default)]
struct Shares {
    of: [f32; MAX_PANELS],
    /// The declared sizes are the first frame's answer; after that the reader's
    /// are.
    seeded: bool,
}

/// Two or more regions the reader can re-proportion.
///
/// A desktop window is mostly this — a column beside a document beside an
/// inspector — and which of the three deserves the room is the reader's
/// decision rather than the designer's. Where a column has one right width and
/// the reader only opens or shuts it, that is [`Sidebar`](crate::Sidebar)
/// instead: it carries its own width token and its own rail, and drawing it
/// with this would offer a choice the design has already made.
///
/// The handle is drawn *from* the space between two panels rather than added to
/// it: the hit target is a whole step of the scale, so grabbing a seam does not
/// demand pixel accuracy, while the rule itself stays the hairline every other
/// separator in the system is.
///
/// The shares live in GPUI's element state through [`latched`], so no host
/// holds them: a drag that had to round-trip through a host's state would lag
/// the pointer, and a seam that lags the pointer feels broken.
#[derive(IntoElement)]
pub struct Splitter {
    id: ElementId,
    panels: Vec<SplitterPanel>,
    orientation: SplitterOrientation,
}

impl Splitter {
    pub fn new(id: impl Into<ElementId>, panels: Vec<SplitterPanel>) -> Self {
        Splitter {
            id: id.into(),
            panels,
            orientation: SplitterOrientation::Horizontal,
        }
    }

    pub fn orientation(mut self, orientation: SplitterOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl RenderOnce for Splitter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let Splitter {
            id,
            panels,
            orientation,
        } = self;
        let horizontal = orientation == SplitterOrientation::Horizontal;

        // Normalised to a hundred once, here, so everything downstream — the
        // clamps, the flex weights, what a seam commits — speaks one unit
        // rather than whatever units the panels were written in.
        let sum: f32 = panels.iter().map(|panel| panel.default_size).sum();
        let declared: Vec<f32> = panels
            .iter()
            .map(|panel| panel.default_size / sum * 100.)
            .collect();
        let limits: Vec<(f32, f32)> = panels
            .iter()
            .map(|panel| (panel.min_size, panel.max_size))
            .collect();
        let last = panels.len().saturating_sub(1);

        latched::<Shares>(id.clone(), move |cell, _, cx| {
            let mut state = cell.get();
            if !state.seeded {
                for (index, share) in declared.iter().take(MAX_PANELS).enumerate() {
                    state.of[index] = *share;
                }
                state.seeded = true;
                cell.set(state);
            }

            // The splitter's own box, captured by a canvas that paints nothing:
            // GPUI gives a drag the dragged element's bounds, and what a seam
            // needs is the run its share is a share *of*.
            let run = Rc::new(Cell::new(Bounds::<Pixels>::default()));

            let mut children: Vec<AnyElement> = Vec::new();
            for (index, panel) in panels.into_iter().enumerate() {
                let share = state.of[index.min(MAX_PANELS - 1)].max(0.01);
                children.push(
                    div()
                        .flex()
                        .overflow_hidden()
                        .min_w(px(0.))
                        .min_h(px(0.))
                        .map(|el| {
                            if horizontal {
                                el.w(relative(share / 100.))
                            } else {
                                el.h(relative(share / 100.))
                            }
                        })
                        .child(panel.child)
                        .into_any_element(),
                );
                if index < last && index + 1 < MAX_PANELS {
                    children.push(
                        seam(
                            ElementId::from(SharedString::from(format!("splitter-seam-{index}"))),
                            index,
                            horizontal,
                            cell.clone(),
                            run.clone(),
                            limits.clone(),
                            cx,
                        )
                        .into_any_element(),
                    );
                }
            }

            div()
                .relative()
                .flex()
                .map(|el| if horizontal { el } else { el.flex_col() })
                .size_full()
                .min_w(px(0.))
                .min_h(px(0.))
                .child(
                    canvas(move |bounds, _, _| run.set(bounds), |_, _, _, _| {})
                        .absolute()
                        .size_full(),
                )
                .children(children)
                .into_any_element()
        })
    }
}

/// The seam: a hit target with a rule inside it.
///
/// Under the pointer the rule takes the accent and swells, which is the whole
/// affordance: a hairline that never answers is indistinguishable from a
/// border, and a reader has no way to learn that this one can be dragged. It
/// swells rather than widens, so the panels beside it never move on the way to
/// a drag.
#[allow(clippy::too_many_arguments)]
fn seam(
    id: ElementId,
    index: usize,
    horizontal: bool,
    cell: Rc<Cell<Shares>>,
    run: Rc<Cell<Bounds<Pixels>>>,
    limits: Vec<(f32, f32)>,
    cx: &App,
) -> impl IntoElement {
    let vars = cx.theme().vars;
    let target = vars.spacing_2;

    pressable(id.clone(), true, move |state, _, _| {
        let lit = state.hovered || state.pressed;
        let ink = if lit {
            vars.control_color(
                ControlVariant::Filled,
                ControlPart::Surface,
                Tint::Primary,
                ControlState::Normal,
            )
        } else {
            vars.color_border
        };
        let weight = vars.stroke_hairline * if lit { GRIP_SWELL } else { 1. };

        // The pointer's position against the whole run says where the seam
        // should land, which is the same move the slider makes: an absolute
        // position rather than an accumulated delta, because a delta that is a
        // percentage of a percentage drifts, and a seam that drifts ends up
        // somewhere other than under the pointer.
        let commit = {
            let cell = cell.clone();
            let run = run.clone();
            let limits = limits.clone();
            move |pointer: Pixels| {
                let box_of = run.get();
                let (origin, length) = if horizontal {
                    (box_of.origin.x, box_of.size.width)
                } else {
                    (box_of.origin.y, box_of.size.height)
                };
                let length = f32::from(length);
                if length <= 0. {
                    return;
                }
                let wanted = (f32::from(pointer - origin) / length) * 100.;
                let mut state = cell.get();
                let before: f32 = state.of[..index].iter().sum();
                resize(&mut state, index, &limits, wanted - before);
                cell.set(state);
            }
        };

        div()
            .id(id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .map(|el| {
                if horizontal {
                    el.w(target).h_full()
                } else {
                    el.h(target).w_full()
                }
            })
            .child(
                div()
                    .flex_none()
                    .map(|el| {
                        if horizontal {
                            el.w(weight).h_full()
                        } else {
                            el.h(weight).w_full()
                        }
                    })
                    .bg(ink),
            )
            .on_drag(SplitterDrag, |_, _, _, cx| cx.new(|_| gpui::Empty))
            .on_drag_move::<SplitterDrag>({
                let commit = commit.clone();
                move |event, window, _| {
                    commit(if horizontal {
                        event.event.position.x
                    } else {
                        event.event.position.y
                    });
                    window.refresh();
                }
            })
            .on_click(move |event, window, _| {
                let at = event.position();
                commit(if horizontal { at.x } else { at.y });
                window.refresh();
            })
            .into_any_element()
    })
}

/// Given the leading panel's move, the trailing one takes what the pair has
/// left over. Nothing else in the row shifts, which is what stops a drag at one
/// seam from rippling through the whole window.
fn resize(state: &mut Shares, index: usize, limits: &[(f32, f32)], delta: f32) {
    let pair = state.of[index] + state.of[index + 1];
    let (lead_min, lead_max) = limits[index];
    let (trail_min, trail_max) = limits[index + 1];
    let leading = (state.of[index] + delta)
        .clamp(lead_min, lead_max)
        .clamp(pair - trail_max, pair - trail_min);
    state.of[index] = leading;
    state.of[index + 1] = pair - leading;
}
