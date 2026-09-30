//! Two or more regions the reader can re-proportion.
//!
//! The handle is drawn from the space between two panels: a whole spacing
//! step to grab, with the hairline every separator is drawn in centred inside
//! it. Under the pointer the rule takes the accent and doubles.

use std::rc::Rc;

use gpui::{
    AnyElement, App, AppContext, CursorStyle, DragMoveEvent, ElementId, Empty, Entity,
    InteractiveElement, IntoElement, KeyDownEvent, MouseButton, ParentElement, Pixels, RenderOnce,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use crate::components::collapsible::{kept_focus, part_id};
use crate::foundation::{Tint, focus_visible, pointer_down};
use crate::theme::ActiveTheme;

/// Which way the panels are laid out, not which way a handle runs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitterOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SplitterOrientation {
    pub fn parse(name: &str) -> Option<SplitterOrientation> {
        match name {
            "horizontal" => Some(SplitterOrientation::Horizontal),
            "vertical" => Some(SplitterOrientation::Vertical),
            _ => None,
        }
    }
}

/// How far one arrow key moves a handle, and one with shift held.
const STEP: f32 = 1.0;
const COARSE_STEP: f32 = 10.0;

/// One region of a [`Splitter`]. The sizes are shares of the run: they are
/// normalised, so `50, 25, 25` and `2, 1, 1` lay out alike. `min_size` and
/// `max_size` are percentages of the whole run.
pub struct SplitterPanel {
    default_size: Option<f32>,
    min_size: Option<f32>,
    max_size: Option<f32>,
    children: Vec<AnyElement>,
}

impl SplitterPanel {
    pub fn new() -> Self {
        SplitterPanel {
            default_size: None,
            min_size: None,
            max_size: None,
            children: Vec::new(),
        }
    }

    pub fn default_size(mut self, size: f32) -> Self {
        self.default_size = Some(size);
        self
    }

    pub fn min_size(mut self, size: f32) -> Self {
        self.min_size = Some(size);
        self
    }

    pub fn max_size(mut self, size: f32) -> Self {
        self.max_size = Some(size);
        self
    }
}

impl Default for SplitterPanel {
    fn default() -> Self {
        SplitterPanel::new()
    }
}

impl ParentElement for SplitterPanel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

type OnSizes = Rc<dyn Fn(&[f32], &mut Window, &mut App)>;

/// A row (or column) of panels divided by draggable seams. The splitter owns
/// the shares; `on_sizes_change` hears them after every change so a host can
/// persist the layout.
#[derive(IntoElement)]
pub struct Splitter {
    id: ElementId,
    orientation: SplitterOrientation,
    panels: Vec<SplitterPanel>,
    on_sizes_change: Option<OnSizes>,
}

impl Splitter {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Splitter {
            id: id.into(),
            orientation: SplitterOrientation::Horizontal,
            panels: Vec::new(),
            on_sizes_change: None,
        }
    }

    pub fn orientation(mut self, orientation: SplitterOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn panel(mut self, panel: SplitterPanel) -> Self {
        self.panels.push(panel);
        self
    }

    pub fn panels(mut self, panels: impl IntoIterator<Item = SplitterPanel>) -> Self {
        self.panels.extend(panels);
        self
    }

    pub fn on_sizes_change(
        mut self,
        handler: impl Fn(&[f32], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sizes_change = Some(Rc::new(handler));
        self
    }
}

struct SplitterState {
    sizes: Vec<f32>,
    dragging: Option<usize>,
}

/// Nothing follows the pointer: the seam itself is what moves.
struct DragGhost;

impl gpui::Render for DragGhost {
    fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// What a seam carries while it is dragged.
struct SeamDrag {
    splitter: ElementId,
    index: usize,
}

#[derive(Clone)]
struct Resizer {
    state: Entity<SplitterState>,
    bounds: Rc<Vec<(f32, f32)>>,
    on_sizes_change: Option<OnSizes>,
}

impl Resizer {
    /// Given the leading panel's new share, the trailing one takes what the
    /// pair has left. Nothing else in the row moves.
    fn resize(&self, index: usize, leading: f32, window: &mut Window, cx: &mut App) {
        let (lead_min, lead_max) = self.bounds[index];
        let (trail_min, trail_max) = self.bounds[index + 1];
        let sizes = self.state.update(cx, |state, cx| {
            let pair = state.sizes[index] + state.sizes[index + 1];
            let leading = leading
                .clamp(lead_min, lead_max)
                .max(pair - trail_max)
                .min(pair - trail_min);
            state.sizes[index] = leading;
            state.sizes[index + 1] = pair - leading;
            cx.notify();
            state.sizes.clone()
        });
        if let Some(handler) = &self.on_sizes_change {
            handler(&sizes, window, cx);
        }
        window.refresh();
    }

    fn share(&self, index: usize, cx: &App) -> f32 {
        self.state.read(cx).sizes.get(index).copied().unwrap_or(0.0)
    }
}

impl RenderOnce for Splitter {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let declared: Vec<f32> = self
            .panels
            .iter()
            .map(|panel| panel.default_size.unwrap_or(1.0))
            .collect();
        let state = window.use_keyed_state(part_id(&self.id, "state"), cx, move |_, _| {
            let sum: f32 = declared.iter().sum::<f32>().max(f32::EPSILON);
            SplitterState {
                sizes: declared.iter().map(|size| size / sum * 100.0).collect(),
                dragging: None,
            }
        });
        let count = self.panels.len();
        if state.read(cx).sizes.len() != count {
            // The panels changed under the splitter: start them over evenly.
            state.update(cx, |state, _| {
                state.sizes = vec![100.0 / count.max(1) as f32; count]
            });
        }
        let sizes = state.read(cx).sizes.clone();
        let dragging = state.read(cx).dragging.filter(|_| cx.has_active_drag());

        let horizontal = self.orientation == SplitterOrientation::Horizontal;
        let handle = t.spacing_2;
        let hairline = t.stroke_hairline;
        let accent = t
            .control_color_filled_surface_normal
            .resolve(Tint::Primary.ramp(t));
        let rule = t.color_border.hsla();

        let resizer = Resizer {
            state: state.clone(),
            bounds: Rc::new(
                self.panels
                    .iter()
                    .map(|panel| {
                        (
                            panel.min_size.unwrap_or(0.0),
                            panel.max_size.unwrap_or(100.0),
                        )
                    })
                    .collect(),
            ),
            on_sizes_change: self.on_sizes_change.clone(),
        };

        let splitter_key = part_id(&self.id, "seam");
        let mut root = div()
            .id(self.id.clone())
            .flex()
            .size_full()
            .min_w_0()
            .min_h_0()
            .map(|this| {
                if horizontal {
                    this.flex_row()
                } else {
                    this.flex_col()
                }
            });

        {
            let resizer = resizer.clone();
            let key = splitter_key.clone();
            let seams = count.saturating_sub(1) as f32;
            root = root.on_drag_move(move |event: &DragMoveEvent<SeamDrag>, window, cx| {
                let drag = event.drag(cx);
                if drag.splitter != key {
                    return;
                }
                let index = drag.index;
                let (origin, length, pointer): (Pixels, Pixels, Pixels) = if horizontal {
                    (
                        event.bounds.origin.x,
                        event.bounds.size.width,
                        event.event.position.x,
                    )
                } else {
                    (
                        event.bounds.origin.y,
                        event.bounds.size.height,
                        event.event.position.y,
                    )
                };
                let available = f32::from(length) - seams * handle;
                if available <= 0.0 {
                    return;
                }
                let before: f32 = (0..index).map(|i| resizer.share(i, cx)).sum::<f32>() / 100.0
                    * available
                    + index as f32 * handle;
                let leading = f32::from(pointer - origin) - before - handle / 2.0;
                resizer.resize(index, leading / available * 100.0, window, cx);
            });
        }

        for (index, panel) in self.panels.into_iter().enumerate() {
            let share = sizes.get(index).copied().unwrap_or(0.0);
            let mut pane = div()
                .flex()
                .flex_col()
                .overflow_hidden()
                .min_w_0()
                .min_h_0()
                .flex_basis(px(0.))
                .children(panel.children);
            pane.style().flex_grow = Some(share);
            pane.style().flex_shrink = Some(1.0);
            root = root.child(pane);

            if index + 1 == count {
                continue;
            }
            let focus = kept_focus(part_id(&self.id, format!("handle-{index}")), window, cx);
            let lit = dragging == Some(index) || focus_visible(&focus, window, cx);
            let group = format!("dz-splitter-{index}");
            let grip = div()
                .map(|this| {
                    if horizontal {
                        this.h_full()
                            .w(px(if lit { hairline * 2.0 } else { hairline }))
                    } else {
                        this.w_full()
                            .h(px(if lit { hairline * 2.0 } else { hairline }))
                    }
                })
                .bg(if lit { accent } else { rule })
                .group_hover(group.clone(), move |style| {
                    if horizontal {
                        style.bg(accent).w(px(hairline * 2.0))
                    } else {
                        style.bg(accent).h(px(hairline * 2.0))
                    }
                });

            let key = splitter_key.clone();
            let mut seam = div()
                .id(part_id(&self.id, format!("handle-{index}")))
                .group(group)
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .map(|this| {
                    if horizontal {
                        this.w(px(handle))
                            .h_full()
                            .cursor(CursorStyle::ResizeLeftRight)
                    } else {
                        this.h(px(handle))
                            .w_full()
                            .cursor(CursorStyle::ResizeUpDown)
                    }
                })
                .track_focus(&focus.clone().tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .when(focus_visible(&focus, window, cx), |this| {
                    this.shadow(vec![gpui::BoxShadow {
                        color: crate::foundation::ring_color(t),
                        offset: gpui::point(px(0.), px(0.)),
                        blur_radius: px(0.),
                        spread_radius: px(t.focus_width + t.focus_offset - t.spacing_px),
                    }])
                })
                .on_drag(
                    SeamDrag {
                        splitter: key,
                        index,
                    },
                    {
                        let state = state.clone();
                        move |_, _, _, cx| {
                            state.update(cx, |state, _| state.dragging = Some(index));
                            cx.new(|_| DragGhost)
                        }
                    },
                )
                .child(grip);

            {
                let resizer = resizer.clone();
                seam = seam.on_click(move |event, window, cx| {
                    // An even split of this pair alone.
                    if event.click_count() == 2 {
                        let even = (resizer.share(index, cx) + resizer.share(index + 1, cx)) / 2.0;
                        resizer.resize(index, even, window, cx);
                    }
                });
            }
            {
                let resizer = resizer.clone();
                seam = seam.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let stroke = &event.keystroke;
                    let (back, forward) = if horizontal {
                        ("left", "right")
                    } else {
                        ("up", "down")
                    };
                    let step = if stroke.modifiers.shift {
                        COARSE_STEP
                    } else {
                        STEP
                    };
                    let share = resizer.share(index, cx);
                    let (min, max) = resizer.bounds[index];
                    let next = match stroke.key.as_str() {
                        key if key == back => share - step,
                        key if key == forward => share + step,
                        "home" => min,
                        "end" => max,
                        _ => return,
                    };
                    resizer.resize(index, next, window, cx);
                    cx.stop_propagation();
                });
            }
            root = root.child(seam);
        }
        root
    }
}
