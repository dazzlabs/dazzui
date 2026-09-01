use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    relative,
};

use crate::assets::icons;
use crate::components::{Icon, Spinner, SpinnerSize};
use crate::prelude::*;
use crate::{ControlPart, ControlState, ControlVariant, Tint};

/// Where a step stands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepStatus {
    Done,
    Active,
    #[default]
    Idle,
}

/// The wash a finished step's marker is filled with.
///
/// `step-list.css` writes it as a literal for the same reason the spinner's
/// track is one: it is a weight of a hue the graph has no role for.
const DONE_WASH: f32 = 0.14;

/// One stage of a pipeline, carrying its own status.
#[derive(IntoElement)]
pub struct Step {
    status: StepStatus,
    label: SharedString,
    /// Trailing detail — a count, a duration.
    meta: Option<SharedString>,
    tint: Tint,
}

impl Step {
    pub fn new(label: impl Into<SharedString>, status: StepStatus) -> Self {
        Step {
            status,
            label: label.into(),
            meta: None,
            tint: Tint::Primary,
        }
    }

    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl RenderOnce for Step {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let success = vars.color_success.shade(600);
        let accent_ink = vars.control_color(
            ControlVariant::Plain,
            ControlPart::Content,
            self.tint,
            ControlState::Normal,
        );

        div()
            .flex()
            .items_center()
            .gap(vars.spacing_2_5)
            .type_style(vars.label_quiet)
            .map(|el| match self.status {
                StepStatus::Done => el.text_color(vars.color_content_muted),
                StepStatus::Active => el
                    .text_color(vars.color_content)
                    .font_weight(vars.label_medium.weight),
                // Not started yet: the whole row recedes, marker and meta
                // included — the plan is visible, the past and present are
                // what read.
                StepStatus::Idle => el.text_color(vars.color_content_subtle).opacity(0.8),
            })
            .map(|el| match self.status {
                StepStatus::Active => {
                    el.child(Spinner::new().size(SpinnerSize::Medium).tint(self.tint))
                }
                status => el.child(
                    div()
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .size(vars.spacing_4)
                        .rounded(vars.radius_full)
                        .map(|marker| match status {
                            // Done is success wherever the list is tinted: a
                            // finished step is not a variation on the accent,
                            // it is the one fixed meaning in the column.
                            StepStatus::Done => marker
                                .bg(success.opacity(DONE_WASH))
                                .text_color(success)
                                .child(
                                    Icon::new(icons::CHECKMARK_12_FILLED)
                                        .size(vars.label_small.size)
                                        .color(success),
                                ),
                            _ => marker
                                .border(vars.stroke_control)
                                .border_color(vars.color_border_muted),
                        }),
                ),
            })
            .child(div().min_w_0().line_height(relative(1.0)).child(self.label))
            .when_some(self.meta, |el, meta| {
                // The trailing detail runs a step under the label and two ink
                // steps back; only the active row's meta takes the accent —
                // in its text grade, not the fill.
                el.child(
                    div()
                        .flex_none()
                        .ml_auto()
                        .type_style(vars.caption_small.size(vars.label_small.size))
                        .line_height(relative(1.0))
                        .text_color(if self.status == StepStatus::Active {
                            accent_ink
                        } else {
                            vars.color_content_faint
                        })
                        .child(meta),
                )
            })
    }
}

/// The column the steps stand in.
#[derive(IntoElement)]
pub struct StepList {
    children: Vec<AnyElement>,
}

impl StepList {
    pub fn new() -> Self {
        StepList {
            children: Vec::new(),
        }
    }
}

impl Default for StepList {
    fn default() -> Self {
        StepList::new()
    }
}

impl ParentElement for StepList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for StepList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_2)
            .children(self.children)
    }
}
