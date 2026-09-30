//! One key hint or cap. A chord is several of these, composed by the caller.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::foundation::Size;
use crate::theme::ActiveTheme;

/// `hint` is bare faint text — the default, because most keys shown in an
/// interface are asides. `strong` darkens it a step. `key` alone draws the
/// recessed cap.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyCapVariant {
    #[default]
    Hint,
    Strong,
    Key,
}

impl KeyCapVariant {
    pub fn parse(name: &str) -> Option<KeyCapVariant> {
        Some(match name {
            "hint" => KeyCapVariant::Hint,
            "strong" => KeyCapVariant::Strong,
            "key" => KeyCapVariant::Key,
            _ => return None,
        })
    }
}

#[derive(IntoElement)]
pub struct KeyCap {
    size: Size,
    variant: KeyCapVariant,
    children: Vec<AnyElement>,
}

impl KeyCap {
    pub fn new() -> Self {
        KeyCap {
            size: Size::Medium,
            variant: KeyCapVariant::Hint,
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: KeyCapVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Default for KeyCap {
    fn default() -> Self {
        KeyCap::new()
    }
}

impl ParentElement for KeyCap {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for KeyCap {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let font_size = match self.size {
            Size::Large => t.label_medium.size,
            _ => t.label_small.size,
        };
        let key = self.variant == KeyCapVariant::Key;
        let color = match self.variant {
            KeyCapVariant::Hint => t.color_content_faint,
            KeyCapVariant::Strong => t.color_content_subtle,
            KeyCapVariant::Key => t.color_content,
        };
        let weight = if key {
            t.label_strong.font_weight()
        } else {
            t.label_medium.font_weight()
        };
        div()
            .flex()
            .flex_none()
            .items_center()
            .whitespace_nowrap()
            .font_family(t.label_strong.face.family())
            .font_weight(weight)
            .text_size(px(font_size))
            .line_height(px(font_size))
            .text_color(color.hsla())
            .when(key, |this| {
                this.justify_center()
                    .h(px(t.spacing_5 + t.spacing_0_5))
                    .px(px(t.spacing_2_5))
                    .rounded(px(t.radius_tiny))
                    .bg(t.color_surface_inset.hsla())
            })
            .children(self.children)
    }
}
