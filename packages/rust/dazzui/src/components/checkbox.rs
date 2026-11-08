//! A choice that stands on its own, or one of several that may all hold.

mod group;
pub use group::CheckboxGroup;

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::components::icon::Icon;
use crate::foundation::{ControlColors, Size, State, StyledExt, Tint, Variant, pointer_down};
use crate::glyphs;
use crate::theme::ActiveTheme;

type OnCheckedChange = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// A box and a label on one row. The box defines the row; the label rests
/// quiet and takes the full ink and label weight only when its box is
/// checked.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    size: Size,
    tint: Tint,
    checked: Option<bool>,
    default_checked: bool,
    indeterminate: bool,
    disabled: bool,
    read_only: bool,
    value: Option<SharedString>,
    parent: bool,
    indented: bool,
    note: Option<AnyElement>,
    children: Vec<AnyElement>,
    on_checked_change: Option<OnCheckedChange>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Checkbox {
            id: id.into(),
            size: Size::Medium,
            tint: Tint::Primary,
            checked: None,
            default_checked: false,
            indeterminate: false,
            disabled: false,
            read_only: false,
            value: None,
            parent: false,
            indented: false,
            note: None,
            children: Vec::new(),
            on_checked_change: None,
        }
    }

    /// `small`, `medium` or `large`; `tiny` draws as `small`.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Controlled: the box shows this, and a press only reports.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Uncontrolled: where the box starts; it keeps its own state after.
    pub fn default_checked(mut self, checked: bool) -> Self {
        self.default_checked = checked;
        self
    }

    /// The mixed state: filled like checked, with a dash for the tick.
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The value this checkbox contributes to its nearest group.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Counts and toggles a group's children when that group has `all_values`.
    pub fn parent(mut self, parent: bool) -> Self {
        self.parent = parent;
        self
    }

    /// Keeps focus and drawing, while refusing selection changes.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// A de-emphasised aside on the label — a count, a size.
    pub fn note(mut self, note: impl IntoElement) -> Self {
        self.note = Some(note.into_any_element());
        self
    }

    pub fn on_checked_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Checkbox {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for Checkbox {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let group = group::GroupScope::current(cx);
        self.disabled |= group.as_ref().is_some_and(|group| group.disabled);
        if let (Some(group), Some(value)) = (&group, &self.value) {
            group
                .disabled_states
                .borrow_mut()
                .insert(value.clone(), self.disabled);
        }
        let state = window.use_keyed_state(kit::child_id(&self.id, "checked"), cx, {
            let default = self.default_checked;
            move |_, _| default
        });
        let focus = kit::focus_handle(&self.id, window, cx);
        let mut checked = self.checked.unwrap_or(*state.read(cx));
        let mut indeterminate = self.indeterminate;
        let grouped_parent = self.parent
            && group
                .as_ref()
                .is_some_and(|group| group.all_values.is_some());
        let grouped_child = !self.parent && self.value.is_some() && group.is_some();
        if let Some(group) = &group {
            if grouped_parent {
                let count = group.all_values.as_ref().unwrap().len();
                checked = group.value.len() == count;
                indeterminate |= !group.value.is_empty() && group.value.len() != count;
            } else if grouped_child {
                checked = group.value.contains(self.value.as_ref().unwrap());
            } else if group.all_values.is_some() {
                checked = self.checked.unwrap_or(false);
            }
        }
        let marked = checked || indeterminate;
        let visible = !self.disabled && crate::foundation::focus_visible(&focus, window, cx);

        let edge = match self.size {
            Size::Tiny | Size::Small => t.checkbox_small_box,
            Size::Medium => t.checkbox_medium_box,
            Size::Large => t.checkbox_large_box,
        };
        let filled = ControlColors::resolve(t, Variant::Filled, self.tint, State::Normal);
        let glyph = if indeterminate {
            glyphs::SUBTRACT_12_FILLED
        } else {
            glyphs::CHECKMARK_12_FILLED
        };
        let glyph_edge = t.icon_small.min(edge - 2. * t.checkbox_thickness);

        let mark = div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(edge))
            .border(px(t.checkbox_thickness))
            .rounded(px(t.checkbox_radius))
            .border_color(t.color_border_muted.hsla())
            .when(marked, |this| {
                this.bg(filled.surface)
                    .border_color(filled.surface)
                    .text_color(filled.content)
                    .child(Icon::new(glyph).size(glyph_edge))
            });

        let label = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .min_w_0()
            .children(self.children)
            .when_some(
                self.note
                    .and_then(|note| crate::foundation::text_with_prefix(" ", note)),
                |this, note| {
                    this.child(
                        div()
                            .flex()
                            .text_color(t.color_content_faint.hsla())
                            .child(note),
                    )
                },
            );

        let mut row = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .items_center()
            .gap(px(t.spacing_2_5))
            .when(self.indented, |this| {
                this.pl(px(t.checkbox_medium_box + t.spacing_2_5))
            })
            .type_style(t.label_quiet)
            .text_color(if checked {
                t.color_content.hsla()
            } else {
                t.color_content_muted.hsla()
            })
            .when(checked, |this| {
                this.font_weight(t.label_medium.font_weight())
            })
            .child(mark)
            .child(crate::foundation::text_child(label))
            .when(visible, |this| this.child(kit::outline(t, t.radius_tiny)));

        if self.disabled {
            row = row.opacity(0.6).cursor_not_allowed();
        } else {
            let controlled = self.checked.is_some();
            let handler = self.on_checked_change;
            let read_only = self.read_only;
            let value = self.value;
            row = row
                .cursor_pointer()
                .track_focus(&focus.tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .on_click(move |event, window, cx| {
                    if kit::is_enter(event) || read_only {
                        return;
                    }
                    let next = !checked;
                    if let Some(handler) = &handler {
                        handler(next, window, cx);
                    }
                    if grouped_parent {
                        group.as_ref().unwrap().parent_change(window, cx);
                    } else if grouped_child {
                        group.as_ref().unwrap().child_change(
                            value.as_ref().unwrap(),
                            next,
                            window,
                            cx,
                        );
                    } else if !controlled {
                        state.update(cx, |value, cx| {
                            *value = next;
                            cx.notify();
                        });
                    }
                });
        }
        row
    }
}

/// What the choice controls share: part ids, the keyboard focus outline, the
/// roving focus a group moves with the arrow keys, and the hint a glyph-only
/// control carries.
pub(crate) mod kit {
    use gpui::{
        App, AppContext, ClickEvent, Div, ElementId, FocusHandle, IntoElement, KeyboardButton,
        ParentElement, Render, SharedString, Stateful, StatefulInteractiveElement, Styled, Window,
        div, px,
    };

    use crate::foundation::StyledExt;
    use crate::theme::ActiveTheme;
    use crate::tokens::shadows;

    /// An id for one part of a component.
    pub fn child_id(id: &ElementId, part: &str) -> ElementId {
        ElementId::NamedChild(Box::new(id.clone()), SharedString::from(part.to_string()))
    }

    /// The focus handle a component keeps under its id.
    pub fn focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
        window
            .use_keyed_state(child_id(id, "focus"), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone()
    }

    /// One focus handle per member of a group, kept across frames.
    pub fn focus_handles(
        id: &ElementId,
        count: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<FocusHandle> {
        let handles = window.use_keyed_state(child_id(id, "focus-roving"), cx, |_, _| {
            Vec::<FocusHandle>::new()
        });
        if handles.read(cx).len() < count {
            handles.update(cx, |handles, cx| {
                while handles.len() < count {
                    handles.push(cx.focus_handle());
                }
            });
        }
        handles.read(cx)[..count].to_vec()
    }

    pub use crate::foundation::outline;

    /// Where an arrow key moves the focus in a group.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Move {
        Prev,
        Next,
        First,
        Last,
    }

    /// The move a key asks for. A horizontal group reads left and right; a
    /// vertical one up and down as well.
    pub fn arrow(key: &str, vertical: bool) -> Option<Move> {
        Some(match key {
            "left" => Move::Prev,
            "right" => Move::Next,
            "up" if vertical => Move::Prev,
            "down" if vertical => Move::Next,
            "home" => Move::First,
            "end" => Move::Last,
            _ => return None,
        })
    }

    /// The member a move lands on, skipping disabled ones and looping at the
    /// ends.
    pub fn target(current: usize, enabled: &[bool], step: Move) -> Option<usize> {
        let count = enabled.len();
        if count == 0 {
            return None;
        }
        let order: Vec<usize> = match step {
            Move::First => (0..count).collect(),
            Move::Last => (0..count).rev().collect(),
            Move::Next => (1..=count).map(|n| (current + n) % count).collect(),
            Move::Prev => (1..=count).map(|n| (current + count - n) % count).collect(),
        };
        order.into_iter().find(|index| enabled[*index])
    }

    /// Which member holds focus.
    pub fn focused(handles: &[FocusHandle], window: &Window) -> Option<usize> {
        handles.iter().position(|handle| handle.is_focused(window))
    }

    /// A keyboard click from Enter, which a checkbox and a radio
    /// ignore — only Space presses them.
    pub fn is_enter(event: &ClickEvent) -> bool {
        matches!(
            event,
            ClickEvent::Keyboard(click) if click.button == KeyboardButton::Enter
        )
    }

    /// The label a glyph-only control surfaces on hover, drawn as
    /// `tooltip.css` draws its panel. GPUI places it by the pointer rather
    /// than above the control.
    pub fn hint(element: Stateful<Div>, label: Option<SharedString>) -> Stateful<Div> {
        match label {
            Some(label) => element.tooltip(move |_, cx| {
                let label = label.clone();
                cx.new(|_| Hint(label)).into()
            }),
            None => element,
        }
    }

    struct Hint(SharedString);

    impl Render for Hint {
        fn render(&mut self, _: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
            let t = cx.theme();
            div()
                .py(px(t.spacing_1 + t.spacing_0_5))
                .px(px(t.spacing_2))
                .bg(t.color_surface_raised.hsla())
                .border(px(t.stroke_hairline))
                .border_color(t.color_border.hsla())
                .rounded(px(t.radius_small))
                .shadow(shadows(t.shadow_sm))
                .text_color(t.color_content.hsla())
                .type_style(t.label_small)
                .child(self.0.clone())
        }
    }
}
