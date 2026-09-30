//! The thing a person presses to make something happen.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, Styled, Window, div, prelude::*,
};

use crate::components::icon::icon_slot;
use crate::foundation::{Size, StyledExt, Tint, Variant, control, focusable};
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A button. The recipe is `variant` — `recessed`, the workaday grey chip,
/// by default; `filled` for the one action a view points at — resolved
/// against `tint`.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    size: Size,
    variant: Variant,
    tint: Tint,
    disabled: bool,
    full_width: bool,
    shortcut: Option<AnyElement>,
    children: Vec<AnyElement>,
    on_click: Option<OnClick>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Button {
            id: id.into(),
            size: Size::Small,
            variant: Variant::Recessed,
            tint: Tint::Primary,
            disabled: false,
            full_width: false,
            shortcut: None,
            children: Vec::new(),
            on_click: None,
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

    /// Which ramp the recipe resolves against. The neutral recipes —
    /// `recessed` and `normal` — read surface and ink roles, not a ramp.
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    /// Trailing hint — the key that also triggers this action.
    pub fn shortcut(mut self, shortcut: impl IntoElement) -> Self {
        self.shortcut = Some(shortcut.into_any_element());
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

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let profile = self.size.profile(t);
        let focus = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let filled = self.variant == Variant::Filled && !self.disabled;

        let mut button = div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .whitespace_nowrap()
            .border_1()
            .profile(profile)
            .when(self.full_width, |this| this.w_full())
            .when(filled, |this| {
                this.font_weight(t.label_strong.font_weight())
                    .shadow(shadows(t.shadow_2xs))
            });
        button = control(button, t, self.variant, self.tint, self.disabled);
        button = focusable(button, &focus, self.disabled, window, cx);

        if !self.disabled {
            button = button.cursor_pointer();
            if let Some(on_click) = self.on_click {
                button = button.on_click(on_click);
            }
        } else {
            button = button.cursor_not_allowed();
        }

        let shortcut = self.shortcut.map(|shortcut| {
            div()
                .font_family(t.label_strong.face.family())
                .opacity(0.7)
                .child(shortcut)
        });
        icon_slot(
            profile.icon_size,
            button.children(self.children).children(shortcut),
        )
    }
}
