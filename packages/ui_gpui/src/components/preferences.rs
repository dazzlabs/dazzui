use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::components::SectionLabel;
use crate::prelude::*;
use crate::{
    ActionListener, ControlPart, ControlState, ControlVariant, Tint, colors, plus, pressable,
};

/// The settings column, and the root the rest of this file sits in.
///
/// It owns the two things no group or section can: the run between groups,
/// which is the widest step of the spacing ladder this column climbs, and the
/// width. The column is capped rather than filling its pane — a row puts its
/// label on one edge and its control on the other, and past
/// `preferences_width` the two stop reading as one line.
#[derive(IntoElement)]
pub struct Preferences {
    children: Vec<AnyElement>,
}

impl Preferences {
    pub fn new() -> Self {
        Preferences {
            children: Vec::new(),
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences::new()
    }
}

impl ParentElement for Preferences {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Preferences {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        // Full width up to the cap, so it fills a narrow pane and stops in a
        // wide one: `width: 100%; max-width` in the stylesheet.
        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_8)
            .w_full()
            .max_w(vars.preferences_width)
            .children(self.children)
    }
}

/// A labelled run of preference rows.
#[derive(IntoElement)]
pub struct PreferenceSection {
    /// The heading over the rows. Omit for an unlabelled run.
    label: Option<SharedString>,
    /// A control on the heading line, pushed to the trailing edge.
    action: Option<AnyElement>,
    /// A muted note under the rows, for what the section cannot fit.
    footer: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl PreferenceSection {
    pub fn new() -> Self {
        PreferenceSection {
            label: None,
            action: None,
            footer: None,
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    pub fn footer(mut self, footer: impl Into<SharedString>) -> Self {
        self.footer = Some(footer.into());
        self
    }
}

impl Default for PreferenceSection {
    fn default() -> Self {
        PreferenceSection::new()
    }
}

impl ParentElement for PreferenceSection {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PreferenceSection {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let heading = self.label.is_some() || self.action.is_some();

        div()
            .flex()
            .flex_col()
            .when(heading, |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(vars.spacing_4)
                        .mb(vars.spacing_3)
                        .child(
                            div().when_some(self.label, |el, label| {
                                el.child(SectionLabel::new(label))
                            }),
                        )
                        .when_some(self.action, |el, action| {
                            // A heading's control sits in a zero-height slot
                            // it overhangs on both sides: letting a 32px
                            // control size the line would make a heading with
                            // a button taller than one without, and two
                            // sections on the same page would then start at
                            // different heights.
                            el.child(div().flex().items_center().h_0().child(action))
                        }),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(vars.spacing_1)
                    .children(self.children),
            )
            .when_some(self.footer, |el, footer| {
                el.child(
                    div()
                        .mt(vars.spacing_2)
                        .type_style(vars.caption_small)
                        .line_height(relative(1.7))
                        .text_color(vars.color_content_subtle)
                        .child(footer),
                )
            })
    }
}

/// A titled run of sections, for a page whose sections come in bunches.
///
/// Sections stay one size everywhere; the level above them is a title that
/// outranks them, not a section shrunk to fit underneath one. A heading then
/// means the same thing wherever it is met.
#[derive(IntoElement)]
pub struct PreferenceGroup {
    title: SharedString,
    /// The line under the title, for what the group is for.
    description: Option<SharedString>,
    action: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl PreferenceGroup {
    pub fn new(title: impl Into<SharedString>) -> Self {
        PreferenceGroup {
            title: title.into(),
            description: None,
            action: None,
            children: Vec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl ParentElement for PreferenceGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PreferenceGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(vars.spacing_4)
                    .mb(vars.spacing_3)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(vars.spacing_1_5)
                            .child(
                                // The group outranks its sections
                                // typographically: the display cut at the
                                // strong weight, where a section heading is a
                                // SectionLabel.
                                div()
                                    .type_style(vars.label_strong)
                                    .text_color(vars.color_content)
                                    .child(self.title),
                            )
                            .when_some(self.description, |el, description| {
                                el.child(
                                    div()
                                        .type_style(vars.caption_small)
                                        .line_height(relative(1.7))
                                        .text_color(vars.color_content_subtle)
                                        .child(description),
                                )
                            }),
                    )
                    .when_some(self.action, |el, action| {
                        el.child(div().flex().items_center().h_0().child(action))
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(plus(vars.spacing_4, vars.spacing_0_5))
                    .children(self.children),
            )
    }
}

/// One preference row: title and optional subtitle left, control right.
///
/// A row's height is a minimum plus a pad on its text, never a fixed height.
/// The minimum keeps the text column in one rhythm whatever control sits on
/// the right; the pad rides on the text so a row grows for content — a
/// subtitle, a wrapped title — and never for a control.
#[derive(IntoElement)]
pub struct PreferenceRow {
    id: ElementId,
    title: SharedString,
    /// The second line, for what the title cannot say in three words.
    subtitle: Option<SharedString>,
    /// A glyph or avatar before the title, for rows that name a thing.
    icon: Option<AnyElement>,
    tint: Tint,
    /// Makes the row open something. It grows the hover wash that says so;
    /// rows that only carry a control leave this off and stay inert.
    on_open: Option<ActionListener>,
    children: Vec<AnyElement>,
}

impl PreferenceRow {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        PreferenceRow {
            id: id.into(),
            title: title.into(),
            subtitle: None,
            icon: None,
            tint: Tint::Neutral,
            on_open: None,
            children: Vec::new(),
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn on_open(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_open = Some(Box::new(listener));
        self
    }
}

impl ParentElement for PreferenceRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PreferenceRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let PreferenceRow {
            id,
            title,
            subtitle,
            icon,
            tint,
            on_open,
            children,
        } = self;
        let interactive = on_open.is_some();
        // The wash previews selection — the accent at 8% — because an
        // interactive row here opens something; a neutral wash is for rows
        // that merely light.
        let wash = vars.control_color(
            ControlVariant::Plain,
            ControlPart::Surface,
            tint,
            ControlState::Hovered,
        );
        let trailing = !children.is_empty() || interactive;

        pressable(id.clone(), interactive, move |state, _, _| {
            div()
                .id(id)
                .flex()
                .items_center()
                .gap(vars.spacing_2_5)
                .min_h(vars.control_medium_size)
                .w_full()
                .when(interactive, |el| {
                    // Pulled out and pushed back in, so the wash is wider
                    // than the text while the row still starts on the same
                    // left edge as the inert rows above it.
                    el.cursor_pointer()
                        .rounded(vars.radius_small)
                        .mx(-vars.spacing_2)
                        .px(vars.spacing_2)
                        .bg(if state.hovered {
                            wash
                        } else {
                            colors::TRANSPARENT
                        })
                })
                .when_some(icon, |el, icon| {
                    // Uncoloured on purpose: a Badge or a Switch in these
                    // slots keeps its own ink rather than inheriting a wash.
                    el.child(div().flex().flex_none().items_center().child(icon))
                })
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(vars.spacing_1)
                        .min_w_0()
                        .py(vars.spacing_1)
                        .child(
                            div()
                                .type_style(vars.label_medium)
                                .text_color(vars.color_content)
                                .child(title),
                        )
                        .when_some(subtitle, |el, subtitle| {
                            el.child(
                                div()
                                    .type_style(vars.caption_small)
                                    .text_color(vars.color_content_subtle)
                                    .child(subtitle),
                            )
                        }),
                )
                .when(trailing, |el| {
                    el.child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .gap(vars.spacing_2_5)
                            .ml_auto()
                            .children(children)
                            .when(interactive, |el| {
                                el.child(div().text_color(vars.color_content_faint).child("›"))
                            }),
                    )
                })
                .when_some(on_open.filter(|_| state.enabled), |el, listener| {
                    el.on_click(move |_, window, cx| listener(window, cx))
                })
                .into_any_element()
        })
    }
}
