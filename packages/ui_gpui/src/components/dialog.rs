use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    relative,
};

use crate::prelude::*;
use crate::{Edge, hairline, plus, rule};

/// How loud the sheet is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogTone {
    #[default]
    Default,
    Danger,
}

/// How much of the danger ramp a dangerous dialog draws its edge in.
///
/// `dialog.css`'s own literal: an edge that reads as a warning without
/// becoming a border.
const DANGER_EDGE: f32 = 0.34;

/// The sheet a modal form or a confirmation is built in.
///
/// It never grows past its scrim, and when the content still does not fit,
/// the body scrolls while the header and footer stay put.
#[derive(IntoElement)]
pub struct Dialog {
    /// Overrides the token width for a sheet that needs more or less room.
    width: Option<gpui::Pixels>,
    tone: DialogTone,
    children: Vec<AnyElement>,
}

impl Dialog {
    pub fn new() -> Self {
        Dialog {
            width: None,
            tone: DialogTone::Default,
            children: Vec::new(),
        }
    }

    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn tone(mut self, tone: DialogTone) -> Self {
        self.tone = tone;
        self
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Dialog::new()
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Dialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .overflow_hidden()
            .w(self.width.unwrap_or(vars.dialog_width))
            .max_w_full()
            .max_h_full()
            .bg(vars.color_surface_overlay)
            .border(hairline(vars))
            .border_color(match self.tone {
                DialogTone::Default => vars.color_border_strong,
                DialogTone::Danger => vars.color_danger.shade(600).opacity(DANGER_EDGE),
            })
            // The window corner and the popover shadow: a dialog is a window
            // that floats, not a panel that hovers at the window's own
            // elevation.
            .rounded(vars.frame_window_radius)
            .token_shadow(vars.shadow_lg)
            .children(self.children)
    }
}

/// The title strip.
#[derive(IntoElement)]
pub struct DialogHeader {
    title: SharedString,
    subtitle: Option<SharedString>,
}

impl DialogHeader {
    pub fn new(title: impl Into<SharedString>) -> Self {
        DialogHeader {
            title: title.into(),
            subtitle: None,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
}

impl RenderOnce for DialogHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .relative()
            .flex()
            .flex_col()
            .justify_center()
            .flex_none()
            .gap(vars.spacing_1)
            // Floored at 44px so a subtitle-less header still balances the
            // footer.
            .min_h(vars.spacing_11)
            .py(vars.spacing_3)
            .px(vars.spacing_5)
            .child(
                div()
                    .type_style(vars.label_strong.size(vars.title_small.size))
                    .line_height(relative(1.0))
                    .text_color(vars.color_content)
                    .child(self.title),
            )
            .when_some(self.subtitle, |el, subtitle| {
                el.child(
                    div()
                        .type_style(vars.caption_small)
                        .text_color(vars.color_content_subtle)
                        .child(subtitle),
                )
            })
            .child(rule(vars.color_border, Edge::Bottom, hairline(vars)))
    }
}

/// The only part that scrolls, so the only part that may shrink.
#[derive(IntoElement)]
pub struct DialogBody {
    children: Vec<AnyElement>,
}

impl DialogBody {
    pub fn new() -> Self {
        DialogBody {
            children: Vec::new(),
        }
    }
}

impl Default for DialogBody {
    fn default() -> Self {
        DialogBody::new()
    }
}

impl ParentElement for DialogBody {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogBody {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_4)
            .min_h_0()
            // A step and a half, the way the preference column reaches the
            // same value: the body pads deeper than the chrome above and
            // below it, so the content sits clear of both rules rather than
            // level with their text.
            .py(plus(vars.spacing_4, vars.spacing_0_5))
            .px(vars.spacing_5)
            .children(self.children)
    }
}

/// The action strip.
#[derive(IntoElement)]
pub struct DialogFooter {
    children: Vec<AnyElement>,
}

impl DialogFooter {
    pub fn new() -> Self {
        DialogFooter {
            children: Vec::new(),
        }
    }
}

impl Default for DialogFooter {
    fn default() -> Self {
        DialogFooter::new()
    }
}

impl ParentElement for DialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .relative()
            .flex()
            .items_center()
            .flex_none()
            .gap(vars.spacing_2)
            .py(vars.spacing_3)
            .px(vars.spacing_5)
            .bg(vars.color_surface_chrome)
            .children(self.children)
            .child(rule(vars.color_border, Edge::Top, hairline(vars)))
    }
}

/// The dimmed backdrop a [`Dialog`] is centred in.
///
/// Absolute rather than fixed: GPUI has no viewport of its own, so the scrim
/// fills whatever it is placed in — put it in the window's root and it covers
/// the window, which is what `position: fixed` means on the web.
#[derive(IntoElement)]
pub struct DialogScrim {
    children: Vec<AnyElement>,
}

impl DialogScrim {
    pub fn new() -> Self {
        DialogScrim {
            children: Vec::new(),
        }
    }
}

impl Default for DialogScrim {
    fn default() -> Self {
        DialogScrim::new()
    }
}

impl ParentElement for DialogScrim {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogScrim {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .p(vars.spacing_6)
            .bg(vars.dialog_scrim_color.opacity(vars.dialog_scrim_alpha))
            .children(self.children)
    }
}
