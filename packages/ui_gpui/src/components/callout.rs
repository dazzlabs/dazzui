use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    relative,
};

use crate::prelude::*;
use crate::{ControlPart, ControlState, ControlVariant, Tint, hairline};

/// One shape, three densities: the type never changes, only the box around
/// it. A callout that grew its type would be competing with the content it
/// interrupts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalloutSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// A tinted notice.
///
/// It always wears a tinted treatment — a callout that needed a fill louder
/// than this would be a dialog, not a callout — so `tint` is the only colour
/// axis. Fill and edge are washes of that tint rather than opaque steps, so
/// the callout sits as correctly on a card as on the paper.
#[derive(IntoElement)]
pub struct Callout {
    size: CalloutSize,
    tint: Tint,
    /// Omit for a one-line notice; the description then carries it alone.
    title: Option<SharedString>,
    description: Option<SharedString>,
    /// Leading node — an icon or a spinner.
    icon: Option<AnyElement>,
    /// Trailing affordance, pushed to the far edge.
    action: Option<AnyElement>,
}

impl Callout {
    pub fn new() -> Self {
        Callout {
            size: CalloutSize::Medium,
            tint: Tint::Neutral,
            title: None,
            description: None,
            icon: None,
            action: None,
        }
    }

    pub fn size(mut self, size: CalloutSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

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

impl RenderOnce for Callout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let has_description = self.title.is_some() && self.description.is_some();
        let (padding_block, padding_inline, radius) = match self.size {
            // The small size is the one whose box takes the control step; it is
            // still a box that grows, so it takes the shared corner for one.
            CalloutSize::Small => (
                vars.spacing_2,
                vars.spacing_3,
                vars.control_container_radius,
            ),
            CalloutSize::Medium => (vars.spacing_3, vars.spacing_3_5, vars.radius_large),
            CalloutSize::Large => (vars.spacing_4, vars.spacing_4, vars.radius_large),
        };
        // The text grades of the tint, not its fill: a notice is read, not
        // pressed.
        let title_ink = vars.control_color(
            ControlVariant::Tinted,
            ControlPart::Content,
            self.tint,
            ControlState::Hovered,
        );
        let body_ink = vars.control_color(
            ControlVariant::Tinted,
            ControlPart::Content,
            self.tint,
            ControlState::Normal,
        );

        div()
            .flex()
            // Once a description wraps, the icon belongs beside the first line
            // rather than halfway down the block.
            .map(|el| {
                if has_description {
                    el.items_start()
                } else {
                    el.items_center()
                }
            })
            .gap(vars.spacing_2_5)
            .py(padding_block)
            .px(padding_inline)
            .rounded(radius)
            // The alphas are the shared surface wash rather than the `tinted`
            // recipe's: that recipe fills a chip at 12%, which is right for
            // something the size of a word and twice too heavy spread across
            // a full width.
            .bg(vars.wash(self.tint, vars.wash_surface))
            .border(hairline(vars))
            .border_color(vars.wash(self.tint, vars.wash_edge))
            .when_some(self.icon, |el, icon| {
                el.child(
                    div()
                        .flex()
                        .flex_none()
                        .text_color(vars.ramp(self.tint).shade(600))
                        .child(icon),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .min_w_0()
                    // The title/description pair is one block of text, so it
                    // is set tighter than the gap separating it from the icon
                    // and the action.
                    .gap(vars.spacing_1)
                    .when_some(self.title, |el, title| {
                        el.child(
                            div()
                                .type_style(vars.label_medium)
                                .line_height(relative(1.4))
                                .text_color(title_ink)
                                .child(title),
                        )
                    })
                    .when_some(self.description, |el, description| {
                        el.child(
                            div()
                                .type_style(vars.label_quiet.weight(vars.caption_small.weight))
                                .line_height(relative(1.5))
                                .text_color(body_ink)
                                .child(description),
                        )
                    }),
            )
            .when_some(self.action, |el, action| {
                el.child(div().flex_none().ml_auto().child(action))
            })
    }
}
