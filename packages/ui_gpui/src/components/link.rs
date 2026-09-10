use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div,
};

use crate::assets::icons;
use crate::components::Icon;
use crate::prelude::*;
use crate::{ClickListener, ControlPart, ControlVariant, Tint, pressable};

/// When the rule under the label is drawn.
///
/// `Hover` is the default, because a page of prose with a rule under every link
/// reads as struck through. `Always` is for a link that has to be findable
/// without a pointer — a list of references, a footer. `None` is for one whose
/// surroundings already say it is a link: a breadcrumb, a row that is entirely
/// a target.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkUnderline {
    #[default]
    Hover,
    Always,
    None,
}

impl LinkUnderline {
    pub const ALL: [LinkUnderline; 3] = [
        LinkUnderline::Hover,
        LinkUnderline::Always,
        LinkUnderline::None,
    ];
}

/// Text that goes somewhere.
///
/// It is the one interactive thing in this package that is genuinely inline,
/// which is why it exists beside `Button::variant(Plain)` rather than inside
/// it: a button keeps its box at every variant, and a box is exactly what a
/// link in a sentence must not have. So this takes the shared state machine for
/// its colour — `plain`'s content grade, a step darker on hover — and none of
/// its geometry.
#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    label: SharedString,
    tint: Tint,
    underline: LinkUnderline,
    /// Goes somewhere outside this window, and draws the arrow that says so
    /// before the reader commits.
    external: bool,
    disabled: bool,
    on_click: Option<ClickListener>,
}

impl Link {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Link {
            id: id.into(),
            label: label.into(),
            tint: Tint::Primary,
            underline: LinkUnderline::Hover,
            external: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn underline(mut self, underline: LinkUnderline) -> Self {
        self.underline = underline;
        self
    }

    pub fn external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Link {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Link {
            id,
            label,
            tint,
            underline,
            external,
            disabled,
            on_click,
        } = self;

        pressable(id.clone(), !disabled, move |state, _, _| {
            // The `plain` recipe's content grade, and only its content: the
            // wash that recipe also carries would ragged the line it sits on,
            // and the grade change is what a link has always used to answer the
            // pointer.
            let content = vars.control_color(
                ControlVariant::Plain,
                ControlPart::Content,
                tint,
                state.control_state(),
            );
            let ruled = match underline {
                LinkUnderline::Always => true,
                LinkUnderline::Hover => state.hovered,
                LinkUnderline::None => false,
            };

            div()
                .id(id)
                .flex()
                .flex_none()
                .items_center()
                .gap(vars.spacing_0_5)
                .text_color(content)
                .when(ruled, |el| el.underline())
                .map(|el| {
                    if disabled {
                        el.cursor_not_allowed()
                    } else {
                        el.cursor_pointer()
                    }
                })
                .child(label)
                .when(external, |el| {
                    el.child(
                        Icon::new(icons::ARROW_UP_RIGHT_12_REGULAR)
                            .size(vars.spacing_3)
                            .color(content),
                    )
                })
                .when_some(on_click, |el, listener| el.on_click(listener))
                .into_any_element()
        })
    }
}
