//! The stages of a pipeline, each carrying its own status.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::components::icon::{Icon, icon_slot};
use crate::components::spinner::Spinner;
use crate::foundation::{Size, Tint};
use crate::glyphs;
use crate::theme::ActiveTheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum StepStatus {
    Done,
    Active,
    #[default]
    Idle,
}

impl StepStatus {
    pub fn parse(name: &str) -> Option<StepStatus> {
        Some(match name {
            "done" => StepStatus::Done,
            "active" => StepStatus::Active,
            "idle" => StepStatus::Idle,
            _ => return None,
        })
    }
}

/// A column of [`Step`]s. The markers carry the status, so there is no
/// numbering.
#[derive(IntoElement, Default)]
pub struct StepList {
    children: Vec<AnyElement>,
}

impl StepList {
    pub fn new() -> Self {
        StepList::default()
    }
}

impl ParentElement for StepList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for StepList {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .flex()
            .flex_col()
            .gap(px(t.spacing_2))
            .children(self.children)
    }
}

/// One stage. Only the active row takes the label weight; a done marker is
/// success wherever the list is tinted, and an idle row recedes whole.
#[derive(IntoElement)]
pub struct Step {
    status: StepStatus,
    label: AnyElement,
    meta: Option<AnyElement>,
    tint: Tint,
}

impl Step {
    pub fn new(status: StepStatus, label: impl IntoElement) -> Self {
        Step {
            status,
            label: crate::foundation::text_child(label),
            meta: None,
            tint: Tint::Primary,
        }
    }

    /// Trailing detail — a count, a duration.
    pub fn meta(mut self, meta: impl IntoElement) -> Self {
        self.meta = Some(crate::foundation::text_child(meta));
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl RenderOnce for Step {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let style = t.label_quiet;
        let (color, weight) = match self.status {
            StepStatus::Done => (t.color_content_muted.hsla(), style.font_weight()),
            StepStatus::Active => (t.color_content.hsla(), t.label_medium.font_weight()),
            StepStatus::Idle => (t.color_content_subtle.hsla(), style.font_weight()),
        };

        let marker = match self.status {
            StepStatus::Active => Spinner::new()
                .size(Size::Medium)
                .tint(self.tint)
                .into_any_element(),
            status => {
                let success = t.color_success.shade(600);
                icon_slot(
                    t.icon_small,
                    div()
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .size(px(t.icon_medium))
                        .rounded(px(t.radius_full))
                        .map(|this| match status {
                            StepStatus::Done => this
                                .bg(success.alpha(0.14))
                                .text_color(success.hsla())
                                .child(Icon::new(glyphs::CHECKMARK_12_FILLED)),
                            _ => this
                                .border(px(crate::foundation::border_width(t.stroke_control)))
                                .border_color(t.color_border_muted.hsla()),
                        }),
                )
                .into_any_element()
            }
        };

        let meta = self.meta.map(|meta| {
            let meta_color = if self.status == StepStatus::Active {
                t.control_color_plain_content_normal
                    .resolve(self.tint.ramp(t))
            } else {
                t.color_content_faint.hsla()
            };
            div()
                .flex_none()
                .whitespace_nowrap()
                .pl(px(t.spacing_2_5))
                .ml_auto()
                .text_color(meta_color)
                .font_family(t.caption_small.face.family())
                .text_size(px(t.label_small.size))
                .font_weight(t.caption_small.font_weight())
                .line_height(px(t.label_small.size))
                .child(meta)
        });

        // Margins rather than `gap`: beside the meta's auto margin, taffy
        // lays the row out without its gaps.
        div()
            .flex()
            .items_center()
            .font_family(style.face.family())
            .text_size(px(style.size))
            .line_height(px(style.line_height))
            .font_weight(weight)
            .text_color(color)
            .when(self.status == StepStatus::Idle, |this| this.opacity(0.8))
            .child(marker)
            .child(
                div()
                    .min_w_0()
                    .ml(px(t.spacing_2_5))
                    .child(crate::foundation::text_child(self.label)),
            )
            .children(meta)
    }
}
