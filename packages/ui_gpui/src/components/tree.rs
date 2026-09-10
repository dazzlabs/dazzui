use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div,
};

use crate::assets::icons;
use crate::components::Icon;
use crate::prelude::*;
use crate::{
    ControlPart, ControlState, ControlVariant, SharedActionListener, SharedValueListener, Tint,
    colors, pressable,
};

/// Two densities, both under control height: a tree is a column of names
/// rather than a column of controls, and a row at control height would fit six
/// fewer files on the screen for nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TreeSize {
    Small,
    #[default]
    Medium,
}

impl TreeSize {
    pub const ALL: [TreeSize; 2] = [TreeSize::Small, TreeSize::Medium];
}

/// One entry in a [`Tree`].
pub struct TreeNode {
    id: SharedString,
    label: SharedString,
    icon: Option<&'static str>,
    meta: Option<SharedString>,
    /// `Some`, empty included, marks the node as a branch. An empty list is how
    /// an empty folder still draws a twisty and still opens, which is the
    /// difference between "nothing in here" and "not a folder".
    children: Option<Vec<TreeNode>>,
    disabled: bool,
}

impl TreeNode {
    /// A leaf: a node that cannot open.
    pub fn leaf(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        TreeNode {
            id: id.into(),
            label: label.into(),
            icon: None,
            meta: None,
            children: None,
            disabled: false,
        }
    }

    /// A branch: a node that can, even with nothing inside it.
    pub fn branch(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        children: Vec<TreeNode>,
    ) -> Self {
        TreeNode {
            children: Some(children),
            ..TreeNode::leaf(id, label)
        }
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    /// A trailing aside: a count, a change marker.
    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// One visible row: a node, and how deep it sits.
struct Row<'a> {
    node: &'a TreeNode,
    level: usize,
    open: bool,
}

fn flatten<'a>(
    nodes: &'a [TreeNode],
    open: &[SharedString],
    level: usize,
    rows: &mut Vec<Row<'a>>,
) {
    for node in nodes {
        let is_open = node.children.is_some() && open.contains(&node.id);
        rows.push(Row {
            node,
            level,
            open: is_open,
        });
        if let Some(children) = node.children.as_ref().filter(|_| is_open) {
            flatten(children, open, level + 1, rows);
        }
    }
}

/// A hierarchy the reader walks.
///
/// The keyboard is what makes a tree a tree rather than an indented list, and
/// GPUI has no focus ring to rove: this port draws the hierarchy and answers
/// the pointer, and the arrow-key walk React writes out belongs to a host that
/// owns focus. What is here is the part the design decides — the indent, the
/// twisty's column, the selection's wash and weight.
///
/// A branch and a leaf answer a click differently on purpose. Clicking a folder
/// opens it *and* selects it, because in a file tree those are one gesture;
/// clicking the twisty alone opens it and leaves the selection where it was,
/// which is how a reader looks inside a folder without losing the file they
/// were reading.
#[derive(IntoElement)]
pub struct Tree {
    id: ElementId,
    items: Vec<TreeNode>,
    expanded: Vec<SharedString>,
    selected: Option<SharedString>,
    size: TreeSize,
    /// Which ramp the selected row is washed with. The hover comes off the
    /// `normal` recipe either way — that one reads the theme's surface roles
    /// rather than a ramp, which is what keeps a row's hover neutral while its
    /// selection carries the accent.
    tint: Tint,
    on_select: Option<SharedValueListener<SharedString>>,
    on_toggle: Option<SharedValueListener<SharedString>>,
}

impl Tree {
    pub fn new(id: impl Into<ElementId>, items: Vec<TreeNode>) -> Self {
        Tree {
            id: id.into(),
            items,
            expanded: Vec::new(),
            selected: None,
            size: TreeSize::Medium,
            tint: Tint::Primary,
            on_select: None,
            on_toggle: None,
        }
    }

    pub fn expanded(mut self, expanded: Vec<SharedString>) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn selected(mut self, selected: impl Into<SharedString>) -> Self {
        self.selected = Some(selected.into());
        self
    }

    pub fn size(mut self, size: TreeSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn on_select(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(listener));
        self
    }

    pub fn on_toggle(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(listener));
        self
    }
}

impl RenderOnce for Tree {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Tree {
            id,
            items,
            expanded,
            selected,
            size,
            tint,
            on_select,
            on_toggle,
        } = self;

        let height = match size {
            TreeSize::Small => vars.spacing_6,
            TreeSize::Medium => vars.spacing_7,
        };
        let face = match size {
            TreeSize::Small => vars.label_quiet.size(vars.label_small.size),
            TreeSize::Medium => vars.label_quiet,
        };

        let mut rows: Vec<Row> = Vec::new();
        flatten(&items, &expanded, 1, &mut rows);

        let children: Vec<AnyElement> = rows
            .iter()
            .enumerate()
            .map(|(index, row)| {
                let node_id = row.node.id.clone();
                let branch = row.node.children.is_some();
                let is_selected = selected.as_ref() == Some(&node_id);
                let disabled = row.node.disabled;
                let row_id = ElementId::from(SharedString::from(format!("tree-row-{index}")));
                let twisty_id = ElementId::from(SharedString::from(format!("tree-twisty-{index}")));
                let indent = vars.spacing_1 + vars.spacing_4 * (row.level as f32 - 1.);

                let select = on_select.clone();
                let toggle = on_toggle.clone();
                let twisty_toggle = on_toggle.clone();
                let label = row.node.label.clone();
                let icon = row.node.icon;
                let meta = row.node.meta.clone();
                let open = row.open;
                let key = node_id.clone();
                let twisty_key = node_id.clone();

                pressable(row_id.clone(), !disabled, move |state, _, _| {
                    let mut surface = colors::TRANSPARENT;
                    let mut content = if disabled {
                        vars.color_content_faint
                    } else {
                        vars.color_content_nav
                    };
                    let mut label_face = face;

                    if is_selected {
                        // Selection shifts weight as well as colour, the way
                        // every selected thing in this system does. On a column
                        // of forty rows the weight is doing most of the work —
                        // one grade of colour is hardest to find in exactly this
                        // shape.
                        surface = vars.control_color(
                            ControlVariant::Tinted,
                            ControlPart::Surface,
                            tint,
                            ControlState::Normal,
                        );
                        content = vars.control_color(
                            ControlVariant::Tinted,
                            ControlPart::Content,
                            tint,
                            ControlState::Normal,
                        );
                        label_face = face.weight(vars.label_medium.weight);
                    } else if state.hovered {
                        // The neutral control's wash — the card step — which is
                        // what a menu item, a table row and a tree row all
                        // want. `plain` would come off the tinted ramp and sit
                        // heavier than the selection one rung below it.
                        surface = vars.control_color(
                            ControlVariant::Normal,
                            ControlPart::Surface,
                            Tint::Neutral,
                            ControlState::Hovered,
                        );
                        content = vars.color_content;
                    }

                    div()
                        .id(row_id)
                        .flex()
                        .items_center()
                        .w_full()
                        .h(height)
                        // One step per level, plus the row's own leading inset.
                        // The twisty column is inside the indent, which is what
                        // keeps the glyphs of one level in a straight line.
                        .pl(indent)
                        .pr(vars.spacing_2)
                        .rounded(vars.radius_small)
                        .bg(surface)
                        .text_color(content)
                        .type_style(label_face)
                        .map(|el| {
                            if disabled {
                                el.cursor_not_allowed()
                            } else {
                                el.cursor_pointer()
                            }
                        })
                        .child(twisty(
                            twisty_id.clone(),
                            branch,
                            open,
                            disabled,
                            vars.spacing_4,
                            vars.spacing_3,
                            vars.color_content_faint,
                            twisty_toggle.map(|listener| {
                                let key = twisty_key.clone();
                                Rc::new(move |window: &mut Window, cx: &mut App| {
                                    listener(&key, window, cx)
                                }) as SharedActionListener
                            }),
                        ))
                        .when_some(icon, |el, icon| {
                            el.child(div().pl(vars.spacing_1).child(
                                Icon::new(icon).size(vars.spacing_4).color(if is_selected {
                                    content
                                } else {
                                    vars.color_content_subtle
                                }),
                            ))
                        })
                        .child(
                            div()
                                .flex_1()
                                .pl(vars.spacing_1)
                                .overflow_hidden()
                                .child(label),
                        )
                        .when_some(meta, |el, meta| {
                            el.child(
                                div()
                                    .flex_none()
                                    .pl(vars.spacing_2)
                                    .type_style(vars.caption_small)
                                    .text_color(if is_selected {
                                        content
                                    } else {
                                        vars.color_content_faint
                                    })
                                    .child(meta),
                            )
                        })
                        .when(!disabled, |el| {
                            el.on_click(move |_, window, cx| {
                                if let Some(listener) = select.as_ref() {
                                    listener(&key, window, cx);
                                }
                                if let Some(listener) = toggle.as_ref().filter(|_| branch) {
                                    listener(&key, window, cx);
                                }
                            })
                        })
                        .into_any_element()
                })
                .into_any_element()
            })
            .collect();

        div().id(id).flex().flex_col().w_full().children(children)
    }
}

/// The twisty keeps its column on a leaf as well as a branch, empty.
///
/// Files and folders at one level start at one x, which is what makes a deep
/// tree scannable — a leaf that reclaimed the space would put every filename at
/// a different indent from its siblings.
#[allow(clippy::too_many_arguments)]
fn twisty(
    id: ElementId,
    branch: bool,
    open: bool,
    disabled: bool,
    box_size: gpui::Pixels,
    glyph: gpui::Pixels,
    ink: gpui::Hsla,
    on_toggle: Option<SharedActionListener>,
) -> AnyElement {
    let column = div()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .w(box_size)
        .h(box_size);
    if !branch {
        return column.into_any_element();
    }

    column
        .id(id)
        .child(
            Icon::new(if open {
                icons::CHEVRON_DOWN_12_REGULAR
            } else {
                icons::CHEVRON_RIGHT_12_REGULAR
            })
            .size(glyph)
            .color(ink),
        )
        .when_some(on_toggle.filter(|_| !disabled), |el, listener| {
            // The twisty is the one target that opens without selecting, so its
            // click must not reach the row behind it.
            el.on_click(move |_, window, cx| listener(window, cx))
                .occlude()
        })
        .into_any_element()
}

/// A [`TreeNode`]'s glyphs, for a host that has no icon set of its own.
pub mod tree_icons {
    pub use crate::assets::icons::{DOCUMENT_16_REGULAR as FILE, FOLDER_16_REGULAR as FOLDER};
}
