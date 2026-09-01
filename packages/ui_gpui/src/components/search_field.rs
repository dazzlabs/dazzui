use gpui::{
    AnyElement, App, ElementId, Entity, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, point, px,
};

use crate::assets::icons;
use crate::components::{Icon, InputSize, TextField, editable};
use crate::prelude::*;
use crate::{Tint, TypeStyle, hairline, pressable};

/// Inline search for a list or a table.
///
/// The box belongs to the wrapper rather than to the field, because the glyph
/// and the trailing slot sit inside it — so the focus lift follows the
/// field's focus, which is what `:focus-within` does on the web.
#[derive(IntoElement)]
pub struct SearchField {
    id: ElementId,
    field: Entity<TextField>,
    size: InputSize,
    tint: Tint,
    /// Trailing hint shown while the field is empty — usually a `Kbd`.
    hint: Option<AnyElement>,
    /// Accessible name of the clear button.
    clear_label: SharedString,
}

impl SearchField {
    pub fn new(id: impl Into<ElementId>, field: &Entity<TextField>) -> Self {
        SearchField {
            id: id.into(),
            field: field.clone(),
            size: InputSize::Medium,
            tint: Tint::Primary,
            hint: None,
            clear_label: "Clear search".into(),
        }
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn hint(mut self, hint: impl IntoElement) -> Self {
        self.hint = Some(hint.into_any_element());
        self
    }

    pub fn clear_label(mut self, label: impl Into<SharedString>) -> Self {
        self.clear_label = label.into();
        self
    }
}

impl RenderOnce for SearchField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let metrics = vars.metrics(self.size.control());
        let accent = vars.ramp(self.tint).shade(600);
        let focused = self.field.read(cx).is_focused(window);
        let empty = self.field.read(cx).is_empty();
        let SearchField {
            id, field, hint, ..
        } = self;

        div()
            .flex()
            .items_center()
            .h(metrics.size)
            .gap(metrics.gap)
            .px(vars.spacing_3)
            .rounded(vars.radius_medium)
            .border(hairline(vars, window))
            .bg(if focused {
                vars.color_surface
            } else {
                vars.color_surface_muted
            })
            .border_color(if focused {
                accent
            } else {
                vars.color_border_strong
            })
            .text_color(vars.color_content)
            // Focus lifts the card to the paper behind a soft accent wash —
            // the same move every text control makes.
            .when(focused, |el| {
                el.shadow(vec![gpui::BoxShadow {
                    color: accent.opacity(vars.focus_glow_alpha),
                    offset: point(px(0.), px(0.)),
                    blur_radius: px(0.),
                    spread_radius: vars.focus_width,
                }])
            })
            .child(
                Icon::new(icons::SEARCH_16_REGULAR)
                    .size(vars.spacing_4)
                    .color(vars.color_content_faint),
            )
            .child(editable(
                &field,
                TypeStyle {
                    // Typed queries are body weight — a search that types in
                    // semibold shouts.
                    weight: vars.body_small.weight,
                    ..metrics.content
                },
                metrics.content.line_height,
                vars.color_content,
                vars.color_content_faint,
                accent,
                accent.opacity(vars.focus_glow_alpha),
                false,
                cx,
            ))
            .map(|el| {
                if empty {
                    el.when_some(hint, |el, hint| el.child(div().flex_none().child(hint)))
                } else {
                    el.child(pressable(id.clone(), true, move |state, _, _| {
                        div()
                            .id(id.clone())
                            .flex()
                            .flex_none()
                            .items_center()
                            .cursor_pointer()
                            .text_color(if state.hovered {
                                vars.color_content
                            } else {
                                vars.color_content_faint
                            })
                            .child(Icon::new(icons::DISMISS_12_REGULAR).size(vars.spacing_3))
                            .on_click({
                                let field = field.clone();
                                move |_, window, cx| {
                                    field.update(cx, |field, cx| {
                                        field.set_value("", cx);
                                        field.focus(window);
                                    });
                                }
                            })
                            .into_any_element()
                    }))
                }
            })
    }
}
