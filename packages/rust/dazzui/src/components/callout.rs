//! A tinted notice.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::components::icon::icon_slot;
use crate::foundation::{Size, Tint};
use crate::theme::ActiveTheme;

/// A notice washed in its tint. Fill and edge are washes rather than opaque
/// steps, so it sits as correctly on a card as on the paper; the title and
/// the description take the tint's text grades.
#[derive(IntoElement)]
pub struct Callout {
    size: Size,
    tint: Tint,
    icon: Option<AnyElement>,
    title: Option<AnyElement>,
    action: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl Callout {
    pub fn new() -> Self {
        Callout {
            size: Size::Medium,
            tint: Tint::Neutral,
            icon: None,
            title: None,
            action: None,
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Leading node — an icon or a spinner.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    /// Omit for a one-line notice; the description then carries it alone.
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(crate::foundation::text_child(title));
        self
    }

    /// Trailing affordance, pushed to the far edge.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl Default for Callout {
    fn default() -> Self {
        Callout::new()
    }
}

impl ParentElement for Callout {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for Callout {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let ramp = self.tint.ramp(t);
        let tint = ramp.shade(600);
        let (padding_block, padding_inline, radius) = match self.size {
            Size::Tiny | Size::Small => (t.spacing_2, t.spacing_3, t.control_container_radius),
            Size::Medium => (t.spacing_3, t.spacing_3_5, t.radius_large),
            Size::Large => (t.spacing_4, t.spacing_4, t.radius_large),
        };
        let has_description = self.title.is_some() && !self.children.is_empty();

        let title = self.title.map(|title| {
            let style = t.label_medium;
            div()
                .text_color(t.control_color_tinted_content_hovered.resolve(ramp))
                .font_family(style.face.family())
                .text_size(px(style.size))
                .font_weight(style.font_weight())
                .line_height(px(style.size * 1.4))
                .child(crate::foundation::text_child(title))
        });
        let description = (!self.children.is_empty()).then(|| {
            let size = t.label_quiet.size;
            div()
                .text_color(t.control_color_tinted_content_normal.resolve(ramp))
                .font_family(t.label_quiet.face.family())
                .text_size(px(size))
                .font_weight(t.caption_small.font_weight())
                .line_height(px(size * 1.5))
                .children(self.children)
        });

        // Margins rather than `gap`: beside the action's auto margin, taffy
        // lays the row out without its gaps.
        let has_icon = self.icon.is_some();
        div()
            .flex()
            .flex_row()
            .map(|this| {
                if has_description {
                    this.items_start()
                } else {
                    this.items_center()
                }
            })
            .border(px(t.stroke_hairline))
            .border_color(tint.alpha(t.wash_edge))
            .bg(tint.alpha(t.wash_surface))
            .py(px(padding_block))
            .px(px(padding_inline))
            .rounded(px(radius))
            .when_some(self.icon, |this, icon| {
                this.child(icon_slot(
                    t.icon_medium,
                    div().flex().flex_none().text_color(tint.hsla()).child(icon),
                ))
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(t.spacing_1))
                    .when(has_icon, |this| this.ml(px(t.spacing_2_5)))
                    .min_w_0()
                    .flex_shrink()
                    .children(title)
                    .children(description),
            )
            .when_some(self.action, |this, action| {
                this.child(
                    div()
                        .flex()
                        .flex_none()
                        .pl(px(t.spacing_2_5))
                        .ml_auto()
                        .child(action),
                )
            })
    }
}
