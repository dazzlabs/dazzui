//! The thing a person presses to make something happen.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::components::icon::icon_slot;
use crate::foundation::{Size, StyledExt, Tint, Variant, control, focusable};
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A button. The recipe is `variant` — `recessed`, the workaday grey chip,
/// by default; `filled` for the one action a view points at — resolved
/// against `tint`.
///
/// A glyph beside the label goes in with [`Button::glyph`] rather than
/// [`ParentElement::child`]: the button then draws it
/// `control.label-glyph.shrink` under the profile's icon size, and pads the
/// end it sits at `control.label-glyph.inset` less — React reads both off
/// its children, which an [`AnyElement`] cannot say. A glyph alone is an
/// [`crate::IconButton`].
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
    /// Which of `children` are glyphs, index for index.
    glyphs: Vec<bool>,
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
            glyphs: Vec::new(),
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

    /// A glyph child — an [`crate::Icon`], or whatever a caller draws as
    /// one. It sits where it is added, among the other children.
    pub fn glyph(mut self, glyph: impl IntoElement) -> Self {
        self.children.push(glyph.into_any_element());
        self.glyphs.push(true);
        self
    }

    /// Whether a glyph sits at the leading and at the trailing end of the
    /// label. Only a button with a label has ends: one child is no label.
    fn glyph_ends(&self) -> (bool, bool) {
        if self.glyphs.len() < 2 {
            return (false, false);
        }
        (self.glyphs[0], self.glyphs[self.glyphs.len() - 1])
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
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
        self.glyphs.resize(self.children.len(), false);
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
        let (leading, trailing) = self.glyph_ends();
        let trimmed = px(profile.padding_inline - t.control_label_glyph_inset);

        let mut button = div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .whitespace_nowrap()
            .border_1()
            .profile(profile)
            .when(leading, |this| this.pl(trimmed))
            .when(trailing, |this| this.pr(trimmed))
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
            profile.icon_size - t.control_label_glyph_shrink,
            button.children(self.children).children(shortcut),
        )
    }
}
