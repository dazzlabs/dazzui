//! A square button holding one glyph.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, InteractiveElement, IntoElement,
    MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, div, prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::components::icon::icon_slot;
use crate::foundation::press::ButtonPress;
use crate::foundation::{
    ControlColors, Size, State, StyledExt, Tint, Variant, control, focus_visible, pointer_down,
};
use crate::theme::ActiveTheme;

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A square at the control height. The default posture — `plain`,
/// `neutral` — is quiet chrome: a subtle-ink glyph that washes grey and
/// takes the full ink under the pointer. It does not latch; a glyph button
/// that stays down is a [`Toggle`](crate::Toggle).
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    label: SharedString,
    size: Size,
    variant: Variant,
    tint: Tint,
    disabled: bool,
    children: Vec<AnyElement>,
    on_click: Option<OnClick>,
    focus: Option<FocusHandle>,
}

impl IconButton {
    /// `label` is the accessible name, surfaced as the tooltip.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        IconButton {
            id: id.into(),
            label: label.into(),
            size: Size::Tiny,
            variant: Variant::Plain,
            tint: Tint::Neutral,
            disabled: false,
            children: Vec::new(),
            on_click: None,
            focus: None,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub(crate) fn focus_handle(mut self, focus: FocusHandle) -> Self {
        self.focus = Some(focus);
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for IconButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for IconButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let profile = self.size.profile(t);
        let focus = self
            .focus
            .unwrap_or_else(|| kit::focus_handle(&self.id, window, cx));
        let visible = !self.disabled && focus_visible(&focus, window, cx);
        let press = ButtonPress::new(&self.id, &focus, self.disabled, window, cx);

        let mut button = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(profile.size))
            .rounded(px(profile.radius))
            .border_1();
        // The quiet neutral glyph button has a posture of its own rather
        // than the plain recipe; either way the state machine is set once.
        if self.variant == Variant::Plain && self.tint == Tint::Neutral && !self.disabled {
            let subtle = t.color_surface_subtle.hsla();
            let content = t.color_content.hsla();
            button = button
                .text_color(t.color_content_subtle.hsla())
                .hover(move |style| style.bg(subtle).text_color(content))
                .active(move |style| style.bg(subtle).text_color(content));
            if press.pressed(cx) {
                button = button.bg(subtle).text_color(content);
            }
        } else {
            button = if press.pressed(cx) {
                button.control_colors(ControlColors::resolve(
                    t,
                    self.variant,
                    self.tint,
                    State::Pressed,
                ))
            } else {
                control(button, t, self.variant, self.tint, self.disabled)
            };
        }
        button = kit::hint(button, Some(self.label));
        button = crate::foundation::motion::color_transitions(
            button,
            self.id,
            t,
            (self.variant != Variant::Plain || self.tint != Tint::Neutral || self.disabled)
                .then_some((self.variant, self.tint)),
            window,
            cx,
        );

        if self.disabled {
            button = button.cursor_not_allowed();
        } else {
            button = button
                .cursor_pointer()
                .track_focus(&focus.tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx));
        }

        // Alone in the tiny square the profile's small glyph is half the
        // box; the square takes its own, `icon-button.tiny.icon-size`.
        let icon_size = if self.size == Size::Tiny {
            t.icon_button_tiny_icon_size
        } else {
            profile.icon_size
        };
        icon_slot(
            icon_size,
            press.bind(
                button
                    .children(self.children)
                    .when(visible, |this| this.child(kit::outline(t, profile.radius))),
                self.on_click,
            ),
        )
    }
}
