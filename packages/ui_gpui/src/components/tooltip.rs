use gpui::{
    AnyView, App, Context, IntoElement, ParentElement, Render, SharedString, Styled, Window, div,
};

use crate::plus;
use crate::prelude::*;

/// The label a control surfaces on hover.
///
/// React gets this from a native `title` attribute, which GPUI has no
/// counterpart for — a tooltip there is a view. It is not a component of the
/// design system and so is not exported: it holds no values of its own and
/// wears the popover's, the way `Menu`'s panel does.
pub(crate) struct Tooltip {
    text: SharedString,
}

impl Tooltip {
    pub(crate) fn view(text: SharedString, cx: &mut App) -> AnyView {
        cx.new(|_| Tooltip { text }).into()
    }
}

impl Render for Tooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .px(vars.spacing_2)
            .py(plus(vars.spacing_1, vars.spacing_0_5))
            .rounded(vars.radius_small)
            .bg(vars.color_surface_raised)
            .border(vars.spacing_px)
            .border_color(vars.color_border)
            .token_shadow(vars.shadow_sm)
            .type_style(vars.label_small)
            .text_color(vars.color_content)
            .child(self.text.clone())
    }
}
