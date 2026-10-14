//! A `Badge` you can act on: select it, dismiss it, or both.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, Hsla, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::components::icon::{Icon, icon_slot};
use crate::foundation::{ControlColors, Size, State, StyledExt, Tint, Variant, fade, pointer_down};
use crate::generated::ThemeTokens;
use crate::glyphs;
use crate::theme::ActiveTheme;

type OnSelectedChange = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;
type OnRemove = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// The recipes a tag can wear: a badge's, which are the `control.color`
/// rows that read as a label, plus `raised` — paper on a grey card.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TagVariant {
    Filled,
    #[default]
    Tinted,
    Outlined,
    Raised,
}

impl TagVariant {
    pub fn parse(name: &str) -> Option<TagVariant> {
        Some(match name {
            "filled" => TagVariant::Filled,
            "tinted" => TagVariant::Tinted,
            "outlined" => TagVariant::Outlined,
            "raised" => TagVariant::Raised,
            _ => return None,
        })
    }

    /// The recipe row the state machine reads. `raised` has none; a
    /// pressable raised tag takes `normal`, the paper-faced control.
    fn recipe(self) -> Variant {
        match self {
            TagVariant::Filled => Variant::Filled,
            TagVariant::Tinted => Variant::Tinted,
            TagVariant::Outlined => Variant::Outlined,
            TagVariant::Raised => Variant::Normal,
        }
    }
}

/// A chip. Passing `selected` at all makes the label pressable, and a
/// selected tag is drawn `tinted` with the tint's edge whatever `variant`
/// says. `on_remove` draws the dismiss button, a control of its own.
#[derive(IntoElement)]
pub struct Tag {
    id: ElementId,
    size: Size,
    variant: TagVariant,
    tint: Tint,
    icon: Option<AnyElement>,
    selected: Option<bool>,
    disabled: bool,
    children: Vec<AnyElement>,
    on_selected_change: Option<OnSelectedChange>,
    on_remove: Option<OnRemove>,
}

impl Tag {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Tag {
            id: id.into(),
            size: Size::Medium,
            variant: TagVariant::Tinted,
            tint: Tint::Primary,
            icon: None,
            selected: None,
            disabled: false,
            children: Vec::new(),
            on_selected_change: None,
            on_remove: None,
        }
    }

    /// `small`, `medium` or `large`; `tiny` draws as `small`.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: TagVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// A glyph before the label.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    /// Latched. Setting it is what makes the label pressable.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_selected_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selected_change = Some(Rc::new(handler));
        self
    }

    /// Draws the dismiss button, and is what it does.
    pub fn on_remove(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_remove = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Tag {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

/// A static tag's colours: the variant's resting recipe, as a badge reads it.
fn resting(t: &ThemeTokens, variant: TagVariant, tint: Tint) -> ControlColors {
    let transparent = Hsla::transparent_black();
    let recipe = |v| ControlColors::resolve(t, v, tint, State::Normal);
    match variant {
        TagVariant::Filled => ControlColors {
            border: transparent,
            ..recipe(Variant::Filled)
        },
        TagVariant::Tinted => ControlColors {
            border: transparent,
            ..recipe(Variant::Tinted)
        },
        TagVariant::Outlined => ControlColors {
            surface: transparent,
            ..recipe(Variant::Outlined)
        },
        TagVariant::Raised => ControlColors {
            surface: t.color_surface.hsla(),
            content: t.color_content_muted.hsla(),
            border: transparent,
        },
    }
}

impl RenderOnce for Tag {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let pressable = self.selected.is_some();
        let selected = self.selected.unwrap_or(false);
        let (padding_block, padding_inline, font_size) = match self.size {
            Size::Tiny | Size::Small => (t.spacing_0_5, t.spacing_1_5, t.label_small.size),
            Size::Medium => (t.spacing_0_5, t.spacing_2, t.label_small.size),
            Size::Large => (t.spacing_1, t.spacing_2, t.label_medium.size),
        };

        let mut tag = div()
            .id(self.id.clone())
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .whitespace_nowrap()
            .border_1()
            .py(px(padding_block))
            .px(px(padding_inline))
            .rounded(px(t.radius_tiny))
            .font_family(t.label_strong.face.family())
            .font_weight(t.label_medium.font_weight())
            .text_size(px(font_size))
            .line_height(px(font_size * 1.4));

        // The ink at rest, which the dismiss button's hover wash is mixed from.
        let ink;
        if pressable {
            let variant = if selected {
                Variant::Tinted
            } else {
                self.variant.recipe()
            };
            let edge =
                ControlColors::resolve(t, Variant::Outlined, self.tint, State::Normal).border;
            let colors = |state| {
                let colors = ControlColors::resolve(t, variant, self.tint, state);
                if selected {
                    ControlColors {
                        border: edge,
                        ..colors
                    }
                } else {
                    colors
                }
            };
            if self.disabled {
                let disabled = colors(State::Disabled);
                ink = disabled.content;
                tag = tag.control_colors(disabled).cursor_not_allowed();
            } else {
                let (normal, hovered, pressed) = (
                    colors(State::Normal),
                    colors(State::Hovered),
                    colors(State::Pressed),
                );
                ink = normal.content;
                tag = tag
                    .control_colors(normal)
                    .hover(move |style| style.control_colors(hovered))
                    .active(move |style| style.control_colors(pressed));
            }
        } else {
            let colors = resting(t, self.variant, self.tint);
            ink = colors.content;
            tag = tag
                .control_colors(colors)
                .when(self.disabled, |this| this.cursor_not_allowed());
        }

        let icon = self.icon.map(|icon| {
            icon_slot(
                t.icon_small,
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .mr(px(t.spacing_1))
                    .child(icon),
            )
        });

        let label = if pressable {
            let focus = kit::focus_handle(&self.id, window, cx);
            let visible = !self.disabled && crate::foundation::focus_visible(&focus, window, cx);
            let mut label = div()
                .id(kit::child_id(&self.id, "label"))
                .relative()
                .flex()
                .children(self.children)
                .when(visible, |this| this.child(kit::outline(t, t.radius_tiny)));
            if self.disabled {
                label = label.cursor_not_allowed();
            } else {
                let handler = self.on_selected_change;
                label = label
                    .cursor_pointer()
                    .track_focus(&focus.tab_stop(true))
                    .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                    .on_click(move |_, window, cx| {
                        if let Some(handler) = &handler {
                            handler(!selected, window, cx);
                        }
                    });
            }
            crate::foundation::text_child(label)
        } else {
            div().flex().children(self.children).into_any_element()
        };

        let remove = self.on_remove.map(|on_remove| {
            let remove_id = kit::child_id(&self.id, "remove");
            let focus = window
                .use_keyed_state(kit::child_id(&self.id, "remove-focus"), cx, |_, cx| {
                    cx.focus_handle()
                })
                .read(cx)
                .clone();
            let visible = !self.disabled && crate::foundation::focus_visible(&focus, window, cx);
            let mut button = div()
                .id(remove_id)
                .relative()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .ml(px(t.spacing_1))
                .rounded(px(t.radius_full))
                .child(Icon::new(glyphs::DISMISS_12_REGULAR).size(t.icon_small))
                .when(visible, |this| this.child(kit::outline(t, t.radius_tiny)));
            if self.disabled {
                button = button.opacity(0.4).cursor_not_allowed();
            } else {
                let wash = fade(ink, 0.16);
                button = button
                    .opacity(0.6)
                    .cursor_pointer()
                    .hover(move |style| style.bg(wash).opacity(1.))
                    .track_focus(&focus.tab_stop(true))
                    .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                    .on_click(move |_, window, cx| on_remove(window, cx));
            }
            button
        });

        tag.children(icon)
            .child(crate::foundation::text_child(label))
            .children(remove)
    }
}
