//! The settings column: `Preferences`, its groups and sections, and the rows
//! a label on one edge and a control on the other are drawn in.
//!
//! The spacing is the design: the six `preferences.*-gap` steps, read between
//! runs of text, decide every distance here.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, MouseButton,
    ParentElement, RenderOnce, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::components::collapsible::{kept_focus, part_id};
use crate::foundation::{StyledExt, Tint, focus_visible, pointer_down, ring};
use crate::theme::ActiveTheme;

/// The column the groups run down, at the design's width and never wider
/// than its pane.
#[derive(IntoElement, Default)]
pub struct Preferences {
    children: Vec<AnyElement>,
}

impl Preferences {
    pub fn new() -> Self {
        Preferences::default()
    }
}

impl ParentElement for Preferences {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Preferences {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .flex()
            .flex_col()
            .gap(px(t.preferences_group_gap))
            .w(px(t.preferences_width))
            .max_w_full()
            .children(self.children)
    }
}

/// A heading's control, in a zero-height slot it overhangs on both sides, so
/// a heading with a button stands no taller than one without.
fn heading_action(action: AnyElement) -> impl IntoElement {
    div().flex().items_center().h(px(0.)).child(action)
}

/// A labelled run of preference rows.
#[derive(IntoElement, Default)]
pub struct PreferenceSection {
    label: Option<AnyElement>,
    action: Option<AnyElement>,
    footer: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl PreferenceSection {
    pub fn new() -> Self {
        PreferenceSection::default()
    }

    /// The heading over the rows. Omit for an unlabelled run.
    pub fn label(mut self, label: impl IntoElement) -> Self {
        self.label = Some(label.into_any_element());
        self
    }

    /// A control on the heading line, pushed to the trailing edge.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    /// A muted note under the rows.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
}

impl ParentElement for PreferenceSection {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PreferenceSection {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let heading = (self.label.is_some() || self.action.is_some()).then(|| {
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(t.spacing_4))
                .mb(px(t.preferences_heading_gap))
                // A SectionLabel: the labelling ink at `labelSmall`.
                .child(
                    div()
                        .type_style(t.label_small)
                        .text_color(t.color_content_subtle.hsla())
                        .children(self.label),
                )
                .children(self.action.map(heading_action))
        });
        div()
            .flex()
            .flex_col()
            .children(heading)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(t.preferences_row_gap))
                    .children(self.children),
            )
            .when_some(self.footer, |this, footer| {
                this.child(
                    div()
                        .mt(px(t.preferences_footer_gap))
                        .text_color(t.color_content_subtle.hsla())
                        .type_style(t.caption_small)
                        .line_height(px(t.caption_small.size * 1.7))
                        .child(footer),
                )
            })
    }
}

/// A titled run of sections, for a page whose sections come in bunches.
#[derive(IntoElement)]
pub struct PreferenceGroup {
    title: AnyElement,
    description: Option<AnyElement>,
    action: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl PreferenceGroup {
    pub fn new(title: impl IntoElement) -> Self {
        PreferenceGroup {
            title: title.into_any_element(),
            description: None,
            action: None,
            children: Vec::new(),
        }
    }

    /// The line under the title, for what the group is for.
    pub fn description(mut self, description: impl IntoElement) -> Self {
        self.description = Some(description.into_any_element());
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(t.spacing_4))
                    .mb(px(t.preferences_title_gap))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(t.spacing_0_5))
                            .child(
                                div()
                                    .text_color(t.color_content.hsla())
                                    .type_style(t.title_medium)
                                    .child(self.title),
                            )
                            .when_some(self.description, |this, description| {
                                this.child(
                                    div()
                                        .text_color(t.color_content_subtle.hsla())
                                        .type_style(t.caption_small)
                                        .line_height(px(t.caption_small.size * 1.7))
                                        .child(description),
                                )
                            }),
                    )
                    .children(self.action.map(heading_action)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(t.preferences_section_gap))
                    .children(self.children),
            )
    }
}

type OnOpen = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// One preference row: title and optional subtitle left, control right. With
/// `on_open` the row opens something — it becomes pressable, grows the hover
/// wash that says so, and trails a chevron.
#[derive(IntoElement)]
pub struct PreferenceRow {
    id: ElementId,
    title: AnyElement,
    subtitle: Option<AnyElement>,
    icon: Option<AnyElement>,
    on_open: Option<OnOpen>,
    tint: Tint,
    children: Vec<AnyElement>,
}

impl PreferenceRow {
    pub fn new(id: impl Into<ElementId>, title: impl IntoElement) -> Self {
        PreferenceRow {
            id: id.into(),
            title: title.into_any_element(),
            subtitle: None,
            icon: None,
            on_open: None,
            tint: Tint::Neutral,
            children: Vec::new(),
        }
    }

    /// The second line, for what the title cannot say in three words.
    pub fn subtitle(mut self, subtitle: impl IntoElement) -> Self {
        self.subtitle = Some(subtitle.into_any_element());
        self
    }

    /// A glyph or avatar before the title.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn on_open(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open = Some(Rc::new(handler));
        self
    }

    /// Which ramp an interactive row's hover wash comes off.
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl ParentElement for PreferenceRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PreferenceRow {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let interactive = self.on_open.is_some();
        let trailing = (!self.children.is_empty() || interactive).then(|| {
            div()
                .flex()
                .flex_none()
                .items_center()
                .gap(px(t.spacing_2_5))
                .ml_auto()
                .children(self.children)
                .when(interactive, |this| this.child("›"))
        });

        let row = div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .gap(px(t.spacing_2_5))
            .min_h(px(t.control_medium_size))
            .when_some(self.icon, |this, icon| {
                this.child(crate::components::icon::icon_slot(
                    t.icon_medium,
                    div().flex().flex_none().items_center().child(icon),
                ))
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(t.spacing_1))
                    .min_w_0()
                    .py(px(t.preferences_row_padding))
                    .child(
                        div()
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .text_ellipsis()
                            .text_color(t.color_content.hsla())
                            .type_style(t.label_quiet)
                            .child(self.title),
                    )
                    .when_some(self.subtitle, |this, subtitle| {
                        this.child(
                            div()
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .text_ellipsis()
                                .text_color(t.color_content_subtle.hsla())
                                .type_style(t.caption_small)
                                .child(subtitle),
                        )
                    }),
            )
            .children(trailing);

        match self.on_open {
            None => row.w_full(),
            Some(on_open) => {
                let focus = kept_focus(part_id(&self.id, "focus"), window, cx);
                let wash = t
                    .control_color_plain_surface_hovered
                    .resolve(self.tint.ramp(t));
                // Pulled out and pushed back in, so the wash is wider than
                // the text while the text keeps the inert rows' left edge.
                row.mx(px(-t.spacing_2))
                    .px(px(t.spacing_2))
                    .rounded(px(t.control_container_radius))
                    .cursor_pointer()
                    .hover(move |style| style.bg(wash))
                    .track_focus(&focus.clone().tab_stop(true))
                    .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                    .when(focus_visible(&focus, window, cx), |this| {
                        this.shadow(vec![ring(t)])
                    })
                    .on_click(move |event, window, cx| on_open(event, window, cx))
            }
        }
    }
}
