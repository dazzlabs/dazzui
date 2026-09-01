use dazzui::ActiveTheme;
use dazzui::prelude::*;
use gpui::{
    AnyElement, App, IntoElement, ParentElement, Pixels, RenderOnce, SharedString, Styled, Window,
    div,
};

/// The playground's own presentation — a heading and a frame around a row of
/// specimens. It draws with the design's tokens like everything else, so a
/// theme switch carries the page furniture with it.
#[derive(IntoElement)]
pub struct Specimen {
    title: SharedString,
    note: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl Specimen {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Specimen {
            title: title.into(),
            note: None,
            children: Vec::new(),
        }
    }

    pub fn note(mut self, note: impl Into<SharedString>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl ParentElement for Specimen {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Specimen {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_2)
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap(vars.spacing_2)
                    .child(
                        div()
                            .type_style(vars.label_large)
                            .text_color(vars.color_content)
                            .child(self.title),
                    )
                    .when_some(self.note, |el, note| {
                        el.child(
                            div()
                                .type_style(vars.caption_small)
                                .text_color(vars.color_content_subtle)
                                .child(note),
                        )
                    }),
            )
            .children(self.children)
    }
}

/// A page heading: what this page is, and the one sentence saying why the
/// components on it belong together.
#[derive(IntoElement)]
pub struct PageTitle {
    title: SharedString,
    subtitle: SharedString,
}

impl PageTitle {
    pub fn new(title: impl Into<SharedString>, subtitle: impl Into<SharedString>) -> Self {
        PageTitle {
            title: title.into(),
            subtitle: subtitle.into(),
        }
    }
}

impl RenderOnce for PageTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_1)
            .child(
                div()
                    .type_style(vars.headline_small)
                    .text_color(vars.color_content)
                    .child(self.title),
            )
            .child(
                div()
                    .type_style(vars.body_medium)
                    .text_color(vars.color_content_muted)
                    .child(self.subtitle),
            )
    }
}

/// A row of specimens, which is what a gallery of variants wants.
#[derive(IntoElement)]
pub struct Row {
    gap: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            gap: None,
            children: Vec::new(),
        }
    }

    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = Some(gap);
        self
    }
}

impl ParentElement for Row {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Row {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(self.gap.unwrap_or(vars.spacing_2))
            .children(self.children)
    }
}

/// A column of specimens, at the page's own rhythm.
#[derive(IntoElement)]
pub struct Col {
    gap: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl Col {
    pub fn new() -> Self {
        Col {
            gap: None,
            children: Vec::new(),
        }
    }

    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = Some(gap);
        self
    }
}

impl ParentElement for Col {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Col {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(self.gap.unwrap_or(vars.spacing_2))
            .children(self.children)
    }
}
