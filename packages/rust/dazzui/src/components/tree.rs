//! A hierarchy the reader walks.
//!
//! The one component with no primitive under it, in React as here: a tree's
//! keyboard is not a list's. The whole tree is one tab stop and focus roves
//! inside it; `ArrowRight` opens a shut branch and descends into an open one;
//! `ArrowLeft` shuts an open branch and climbs out of a leaf; Home and End
//! reach the ends; Enter and Space select and toggle; a printable key jumps by
//! type-ahead over the visible rows.

use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, App, BoxShadow, ElementId, Entity, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    Styled, Window, div, point, prelude::FluentBuilder, px,
};

use crate::components::collapsible::{kept_focus, part_id};
use crate::components::icon::{Icon, icon_slot};
use crate::foundation::{Tint, focus_visible, pointer_down, ring_color};
use crate::glyphs;
use crate::theme::ActiveTheme;

/// How long a run of keystrokes counts as one word for type-ahead.
const TYPEAHEAD_WINDOW: Duration = Duration::from_millis(600);

/// The row height: `small` in a narrow column, `medium` by default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TreeSize {
    Small,
    #[default]
    Medium,
}

impl TreeSize {
    pub fn parse(name: &str) -> Option<TreeSize> {
        match name {
            "small" => Some(TreeSize::Small),
            "medium" => Some(TreeSize::Medium),
            _ => None,
        }
    }
}

/// One node. `children` present at all — empty included — marks a branch.
pub struct TreeNode {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<AnyElement>,
    pub meta: Option<AnyElement>,
    pub disabled: bool,
    pub children: Option<Vec<TreeNode>>,
}

impl TreeNode {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        TreeNode {
            id: id.into(),
            label: label.into(),
            icon: None,
            meta: None,
            disabled: false,
            children: None,
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn meta(mut self, meta: impl IntoElement) -> Self {
        self.meta = Some(meta.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Makes this a branch; an empty list is an empty folder.
    pub fn children(mut self, children: impl IntoIterator<Item = TreeNode>) -> Self {
        self.children = Some(children.into_iter().collect());
        self
    }
}

type OnSelect = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;
type OnExpanded = Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Tree {
    id: ElementId,
    items: Vec<TreeNode>,
    expanded: Option<Vec<SharedString>>,
    default_expanded: Vec<SharedString>,
    on_expanded_change: Option<OnExpanded>,
    selected: Option<SharedString>,
    on_selected_change: Option<OnSelect>,
    size: TreeSize,
    tint: Tint,
}

impl Tree {
    pub fn new(id: impl Into<ElementId>, items: impl IntoIterator<Item = TreeNode>) -> Self {
        Tree {
            id: id.into(),
            items: items.into_iter().collect(),
            expanded: None,
            default_expanded: Vec::new(),
            on_expanded_change: None,
            selected: None,
            on_selected_change: None,
            size: TreeSize::Medium,
            tint: Tint::Primary,
        }
    }

    /// Controlled open branches. Pair with `on_expanded_change`.
    pub fn expanded(mut self, expanded: Vec<SharedString>) -> Self {
        self.expanded = Some(expanded);
        self
    }

    pub fn default_expanded(mut self, expanded: Vec<SharedString>) -> Self {
        self.default_expanded = expanded;
        self
    }

    pub fn on_expanded_change(
        mut self,
        handler: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_expanded_change = Some(Rc::new(handler));
        self
    }

    pub fn selected(mut self, selected: Option<SharedString>) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_selected_change(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selected_change = Some(Rc::new(handler));
        self
    }

    pub fn size(mut self, size: TreeSize) -> Self {
        self.size = size;
        self
    }

    /// Which ramp the selected row is washed with.
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// The tree's accessible name. GPUI has no accessibility tree yet, so it
    /// is accepted for the API's sake and has nowhere to go.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

struct TreeState {
    expanded: Vec<SharedString>,
    active: Option<SharedString>,
    typed: String,
    typed_at: Option<Instant>,
}

/// A visible row, flattened: the model the keyboard is written against.
struct Row {
    node: TreeNode,
    level: usize,
    parent: Option<SharedString>,
    branch: bool,
    open: bool,
}

fn flatten(
    items: Vec<TreeNode>,
    open: &[SharedString],
    level: usize,
    parent: Option<SharedString>,
    rows: &mut Vec<Row>,
) {
    for mut node in items {
        let branch = node.children.is_some();
        let is_open = branch && open.contains(&node.id);
        let children = if is_open { node.children.take() } else { None };
        let id = node.id.clone();
        rows.push(Row {
            node,
            level,
            parent: parent.clone(),
            branch,
            open: is_open,
        });
        if let Some(children) = children {
            flatten(children, open, level + 1, Some(id), rows);
        }
    }
}

/// What a row knows about the rows around it, kept for the key handler.
#[derive(Clone)]
struct RowInfo {
    id: SharedString,
    label: SharedString,
    parent: Option<SharedString>,
    branch: bool,
    open: bool,
    disabled: bool,
    focus: FocusHandle,
}

#[derive(Clone)]
struct Wiring {
    state: Entity<TreeState>,
    rows: Rc<Vec<RowInfo>>,
    controlled: bool,
    on_expanded_change: Option<OnExpanded>,
    on_selected_change: Option<OnSelect>,
    expanded: Vec<SharedString>,
}

impl Wiring {
    fn set_open(&self, id: &SharedString, next: bool, window: &mut Window, cx: &mut App) {
        let mut list: Vec<SharedString> =
            self.expanded.iter().filter(|v| *v != id).cloned().collect();
        if next {
            list.push(id.clone());
        }
        if !self.controlled {
            let list = list.clone();
            self.state.update(cx, |state, cx| {
                state.expanded = list;
                cx.notify();
            });
        }
        if let Some(handler) = &self.on_expanded_change {
            handler(list, window, cx);
        }
        window.refresh();
    }

    fn focus_row(&self, id: &SharedString, window: &mut Window, cx: &mut App) {
        if let Some(row) = self.rows.iter().find(|row| &row.id == id) {
            window.focus(&row.focus);
        }
        let id = id.clone();
        self.state.update(cx, |state, cx| {
            state.active = Some(id);
            cx.notify();
        });
    }

    fn activate(&self, index: usize, window: &mut Window, cx: &mut App) {
        let row = &self.rows[index];
        if row.disabled {
            return;
        }
        if let Some(handler) = &self.on_selected_change {
            handler(&row.id, window, cx);
        }
        if row.branch {
            self.set_open(&row.id, !row.open, window, cx);
        }
    }

    fn key(&self, index: usize, event: &KeyDownEvent, window: &mut Window, cx: &mut App) -> bool {
        let rows = &self.rows;
        let row = &rows[index];
        let movable =
            |to: Option<&RowInfo>| to.filter(|row| !row.disabled).map(|row| row.id.clone());
        let modifiers = event.keystroke.modifiers;
        match event.keystroke.key.as_str() {
            "down" => {
                if let Some(id) = movable(rows.get(index + 1)) {
                    self.focus_row(&id, window, cx);
                }
            }
            "up" => {
                if let Some(id) = movable(index.checked_sub(1).and_then(|i| rows.get(i))) {
                    self.focus_row(&id, window, cx);
                }
            }
            "right" => {
                // Open, then descend — two presses, so opening a folder does
                // not also lose the reader's place.
                if row.branch && !row.open {
                    self.set_open(&row.id, true, window, cx);
                } else if row.open
                    && let Some(id) = movable(rows.get(index + 1))
                {
                    self.focus_row(&id, window, cx);
                }
            }
            "left" => {
                if row.branch && row.open {
                    self.set_open(&row.id, false, window, cx);
                } else if let Some(parent) = &row.parent {
                    self.focus_row(parent, window, cx);
                }
            }
            // On the press rather than the release, as the web does — the
            // row's click ignores the keyboard's for the same reason.
            "enter" | "space" => self.activate(index, window, cx),
            "home" => {
                if let Some(first) = rows.first() {
                    self.focus_row(&first.id.clone(), window, cx);
                }
            }
            "end" => {
                if let Some(last) = rows.last() {
                    self.focus_row(&last.id.clone(), window, cx);
                }
            }
            key if key.chars().count() == 1 && !modifiers.platform && !modifiers.control => {
                // Type-ahead over the visible rows only.
                let now = Instant::now();
                let key = key.to_lowercase();
                let prefix = self.state.update(cx, |state, _| {
                    let recent = state
                        .typed_at
                        .is_some_and(|at| now.duration_since(at) < TYPEAHEAD_WINDOW);
                    state.typed = if recent {
                        format!("{}{key}", state.typed)
                    } else {
                        key
                    };
                    state.typed_at = Some(now);
                    state.typed.clone()
                });
                let hit = rows[index + 1..]
                    .iter()
                    .chain(rows[..=index].iter())
                    .find(|row| {
                        !row.disabled && row.label.trim().to_lowercase().starts_with(&prefix)
                    })
                    .map(|row| row.id.clone());
                if let Some(id) = hit {
                    self.focus_row(&id, window, cx);
                }
            }
            _ => return false,
        }
        true
    }
}

impl RenderOnce for Tree {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let default_expanded = self.default_expanded.clone();
        let first = self.items.first().map(|node| node.id.clone());
        let initial_active = self.selected.clone().or(first);
        let state = window.use_keyed_state(part_id(&self.id, "state"), cx, |_, _| TreeState {
            expanded: default_expanded,
            active: initial_active,
            typed: String::new(),
            typed_at: None,
        });
        let expanded = self
            .expanded
            .clone()
            .unwrap_or_else(|| state.read(cx).expanded.clone());

        let mut rows = Vec::new();
        flatten(self.items, &expanded, 1, None, &mut rows);

        // The roving tab stop falls back to the first row when shutting a
        // folder takes the active row out of sight.
        let active = state.read(cx).active.clone();
        let stop = active
            .filter(|id| rows.iter().any(|row| &row.node.id == id))
            .or_else(|| rows.first().map(|row| row.node.id.clone()));

        let infos: Vec<RowInfo> = rows
            .iter()
            .map(|row| RowInfo {
                id: row.node.id.clone(),
                label: row.node.label.clone(),
                parent: row.parent.clone(),
                branch: row.branch,
                open: row.open,
                disabled: row.node.disabled,
                focus: kept_focus(
                    part_id(&self.id, format!("row-{}", row.node.id)),
                    window,
                    cx,
                ),
            })
            .collect();
        let wiring = Wiring {
            state: state.clone(),
            rows: Rc::new(infos),
            controlled: self.expanded.is_some(),
            on_expanded_change: self.on_expanded_change.clone(),
            on_selected_change: self.on_selected_change.clone(),
            expanded: expanded.clone(),
        };

        let ramp = self.tint.ramp(t);
        let hover_surface = t
            .control_color_normal_surface_hovered
            .resolve(Tint::Neutral.ramp(t));
        let selected_surface = t.control_color_tinted_surface_normal.resolve(ramp);
        let selected_content = t.control_color_tinted_content_normal.resolve(ramp);
        let (height, font_size) = match self.size {
            TreeSize::Small => (t.spacing_6, t.label_small.size),
            TreeSize::Medium => (t.spacing_7, t.label_quiet.size),
        };

        let mut tree = div().flex().flex_col().min_w_0();
        for (index, row) in rows.into_iter().enumerate() {
            let info = wiring.rows[index].clone();
            let id = info.id.clone();
            let selected = self.selected.as_ref() == Some(&id);
            let disabled = row.node.disabled;
            let visible = focus_visible(&info.focus, window, cx);
            let is_stop = stop.as_ref() == Some(&id);

            let twisty = div()
                .id(part_id(&self.id, format!("twisty-{id}")))
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .size(px(t.spacing_4))
                .rounded(px(t.radius_tiny))
                .text_color(t.color_content_faint.hsla())
                .when(row.branch, |this| {
                    this.child(
                        Icon::new(if row.open {
                            glyphs::CHEVRON_DOWN_12_REGULAR
                        } else {
                            glyphs::CHEVRON_RIGHT_12_REGULAR
                        })
                        .size(t.icon_small),
                    )
                })
                .when(row.branch && !disabled, |this| {
                    let wiring = wiring.clone();
                    let id = id.clone();
                    let open = row.open;
                    // The one target that opens without selecting.
                    this.on_click(move |_, window, cx| {
                        cx.stop_propagation();
                        wiring.set_open(&id, !open, window, cx);
                    })
                });

            let mut item = div()
                .id(part_id(&self.id, format!("item-{id}")))
                .debug_selector(|| format!("dz-tree-row-{id}"))
                .flex()
                .items_center()
                .gap(px(t.spacing_1))
                .h(px(height))
                .pl(px(t.spacing_1 + (row.level - 1) as f32 * t.spacing_4))
                .pr(px(t.spacing_2))
                .rounded(px(t.radius_small))
                .font_family(t.label_quiet.face.family())
                .font_weight(t.label_quiet.font_weight())
                .text_size(px(font_size))
                .track_focus(&info.focus.clone().tab_stop(is_stop))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .when(visible, |this| {
                    // Inset by a pixel, because rows touch.
                    this.shadow(vec![BoxShadow {
                        color: ring_color(t),
                        offset: point(px(0.), px(0.)),
                        blur_radius: px(0.),
                        spread_radius: px(t.focus_width + t.focus_offset - t.spacing_px),
                    }])
                })
                .map(|this| {
                    if disabled {
                        this.text_color(t.color_content_faint.hsla())
                            .cursor_not_allowed()
                    } else if selected {
                        this.bg(selected_surface)
                            .text_color(selected_content)
                            .font_family(t.label_medium.face.family())
                            .font_weight(t.label_medium.font_weight())
                            .cursor_pointer()
                    } else {
                        this.text_color(t.color_content_nav.hsla())
                            .cursor_pointer()
                            .hover(move |style| {
                                style.bg(hover_surface).text_color(t.color_content.hsla())
                            })
                    }
                })
                .child(twisty)
                .when_some(row.node.icon, |this, icon| {
                    this.child(icon_slot(
                        t.icon_medium,
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .when(!selected, |this| {
                                this.text_color(t.color_content_subtle.hsla())
                            })
                            .child(icon),
                    ))
                })
                .child(
                    div()
                        .min_w_0()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .child(row.node.label.clone()),
                )
                .when_some(row.node.meta, |this, meta| {
                    this.child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .ml_auto()
                            .pl(px(t.spacing_2))
                            .when(!selected, |this| {
                                this.text_color(t.color_content_faint.hsla())
                            })
                            .font_family(t.caption_small.face.family())
                            .text_size(px(t.caption_small.size))
                            .font_weight(t.caption_small.font_weight())
                            .child(meta),
                    )
                });

            {
                let wiring = wiring.clone();
                let id = id.clone();
                item = item.on_click(move |event, window, cx| {
                    if event.is_keyboard() {
                        return;
                    }
                    wiring.focus_row(&id, window, cx);
                    wiring.activate(index, window, cx);
                });
            }
            {
                let wiring = wiring.clone();
                item = item.on_key_down(move |event, window, cx| {
                    if wiring.key(index, event, window, cx) {
                        cx.stop_propagation();
                    }
                });
            }
            {
                let state = state.clone();
                let id = id.clone();
                let focus = info.focus.clone();
                item = item.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    window.focus(&focus);
                    let id = id.clone();
                    state.update(cx, |state, _| state.active = Some(id));
                });
            }
            tree = tree.child(item);
        }
        tree
    }
}
