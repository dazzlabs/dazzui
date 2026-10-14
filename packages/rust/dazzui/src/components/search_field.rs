//! Inline search for a list or a table.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, Focusable, InteractiveElement, IntoElement, MouseButton,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::components::icon::{Icon, icon_slot};
use crate::components::text_field::{FieldBox, part_id, placeholder_color, selection_color};
use crate::foundation::text_input::{OnEvent, OnText, TextInput, input_state};
use crate::foundation::{Size, Tint, focusable, pointer_down};
use crate::glyphs;
use crate::theme::ActiveTheme;

/// A field that filters what is already on screen. Always controlled: the
/// list it filters holds the query, so the query is `value` and every edit
/// is `on_value_change`.
///
/// Escape clears it first and dismisses only once nothing is left, so a
/// stray press never closes a field mid-search.
#[derive(IntoElement)]
pub struct SearchField {
    id: ElementId,
    value: SharedString,
    on_value_change: Option<OnText>,
    placeholder: SharedString,
    hint: Option<AnyElement>,
    size: Size,
    tint: Tint,
    auto_focus: bool,
    on_dismiss: Option<OnEvent>,
}

impl SearchField {
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        SearchField {
            id: id.into(),
            value: value.into(),
            on_value_change: None,
            placeholder: "Search".into(),
            hint: None,
            size: Size::Medium,
            tint: Tint::Primary,
            auto_focus: false,
            on_dismiss: None,
        }
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Trailing hint shown while the field is empty — usually a `KeyCap`.
    pub fn hint(mut self, hint: impl IntoElement) -> Self {
        self.hint = Some(crate::foundation::text_child(hint));
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Focus on first draw, for a field opened by a shortcut.
    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    /// Called on Escape once the field is already empty.
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }

    /// Accepted for React's shape; GPUI 0.2 has no accessibility tree to
    /// name the field or its clear button in.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    /// See [`aria_label`](Self::aria_label).
    pub fn clear_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

impl RenderOnce for SearchField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let profile = self.size.profile(t);
        let input = input_state(self.id.clone(), window, cx);
        let focused = input.read(cx).is_focused(window);

        if self.auto_focus {
            let focused_once =
                window.use_keyed_state(part_id(&self.id, "autofocus"), cx, |_, _| false);
            if !*focused_once.read(cx) {
                focused_once.update(cx, |done, _| *done = true);
                let handle = input.read(cx).focus_handle(cx);

                window.defer(cx, move |window, _| window.focus(&handle));
            }
        }

        let change = self.on_value_change.clone();
        let dismiss = self.on_dismiss.clone();
        let has_value = !self.value.is_empty();
        let clear_focus = window
            .use_keyed_state(part_id(&self.id, "clear-focus"), cx, |_, cx| {
                cx.focus_handle()
            })
            .read(cx)
            .clone();

        let mut text = TextInput::new(self.id.clone())
            .state(input.clone())
            .value(self.value.clone())
            .placeholder(self.placeholder)
            .placeholder_color(placeholder_color(t))
            .selection_color(selection_color(t));
        if let Some(change) = self.on_value_change.clone() {
            text = text.on_change(move |value, window, cx| change(value, window, cx));
        }

        let trailing = if has_value {
            let clear = self.on_value_change.clone();
            let button = div()
                .id(part_id(&self.id, "clear"))
                .flex()
                .flex_none()
                .items_center()
                .text_color(t.color_content_faint.hsla())
                .hover(|style| style.text_color(t.color_content.hsla()))
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, |_, window, cx| {
                    // The press is the clear button's, not the field's.
                    pointer_down(window, cx);
                    cx.stop_propagation();
                })
                .on_click(move |_, window, cx| {
                    if let Some(clear) = &clear {
                        clear("", window, cx);
                    }
                })
                .child(Icon::new(glyphs::DISMISS_12_REGULAR).size(t.icon_small));
            Some(focusable(button, &clear_focus, false, window, cx).into_any_element())
        } else {
            self.hint
        };

        let chrome = FieldBox::new(self.size).tint(self.tint).focused(focused);
        let body = chrome
            .paint(div(), t)
            .w_full()
            .flex()
            .items_center()
            .h(px(profile.size))
            .gap(px(profile.gap))
            .px(px(t.spacing_3))
            .rounded(px(t.radius_medium))
            .font_family(profile.content.face.family())
            .text_size(px(profile.content.size))
            .font_weight(t.body_small.font_weight())
            .line_height(px(profile.content.line_height))
            .cursor_text()
            .on_mouse_down(MouseButton::Left, {
                let input = input.downgrade();
                move |event, window, cx| {
                    input
                        .update(cx, |input, cx| input.mouse_down(event, window, cx))
                        .ok();
                }
            })
            .on_key_down(move |event, window, cx| {
                if event.keystroke.key != "escape" {
                    return;
                }
                cx.stop_propagation();
                if has_value {
                    if let Some(change) = &change {
                        change("", window, cx);
                    }
                } else if let Some(dismiss) = &dismiss {
                    dismiss(window, cx);
                }
            })
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .text_color(t.color_content_faint.hsla())
                    .child(Icon::new(glyphs::SEARCH_16_REGULAR).size(profile.icon_size)),
            )
            .child(text)
            .when_some(trailing, |this, trailing| this.child(trailing));
        icon_slot(profile.icon_size, body)
    }
}
