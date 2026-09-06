//! Documentation presentation — the counterpart of the Storybook's
//! `components/gallery.tsx`, and used by the same one story per component:
//! `Overview`. Every other story is the component alone and reaches for
//! [`stage`](super::stage) instead.
//!
//! It is drawn with the design system's own tokens, so a theme switch carries
//! the page furniture with it too.

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, ColorSwatch, Table, TableCell, TableHead, TableRow, ThemeVariables, TypeStyle,
    hairline, plus,
};
use gpui::{
    AnyElement, App, Hsla, IntoElement, ParentElement, Pixels, RenderOnce, SharedString, Styled,
    Window, div, px,
};

/// The measure the Storybook sets its prose to — 48rem. A line length is a
/// typographic decision rather than a step off the spacing scale, so it is
/// stated once here instead of being composed out of one.
pub const PROSE_MEASURE: Pixels = px(768.);

/// The name column a specimen and a bar both hang their label in: the
/// Storybook's 6rem, composed out of the two steps that make it rather than
/// written down.
pub fn name_column(vars: &ThemeVariables) -> Pixels {
    plus(vars.spacing_20, vars.spacing_4)
}

/// A face for the token names, keys and values a documentation page prints.
///
/// The same reach `text-field.css` makes for its mono field: `base.font.code`
/// has no semantic token in front of it, so the primitive is named here.
pub fn code(face: TypeStyle) -> TypeStyle {
    TypeStyle {
        family: &["ui-monospace"],
        ..face
    }
}

/// A length, printed the way the token graph states it.
pub fn px_label(value: Pixels) -> String {
    let value = f32::from(value);
    if value.fract() == 0. {
        format!("{}px", value as i32)
    } else {
        format!("{value}px")
    }
}

/// An enum case, lowercased the way a story labels it.
///
/// The axes carry a `name()` of their own; a component's own enums do not, so
/// this reads the one thing every one of them derives.
pub fn case(value: impl std::fmt::Debug) -> String {
    format!("{value:?}").to_lowercase()
}

/// An axis case, printed the way a story labels it.
pub fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// A documented page: what the component is, then the sections that argue for
/// each of its axes.
#[derive(IntoElement)]
pub struct DocPage {
    title: SharedString,
    description: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl DocPage {
    pub fn new(title: impl Into<SharedString>) -> Self {
        DocPage {
            title: title.into(),
            description: None,
            children: Vec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl ParentElement for DocPage {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DocPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_10)
            .p(vars.spacing_8)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(vars.spacing_2)
                    .child(
                        div()
                            .type_style(vars.headline_medium)
                            .text_color(vars.color_content)
                            .child(self.title),
                    )
                    .when_some(self.description, |el, description| {
                        el.child(Prose::new(description))
                    }),
            )
            .children(self.children)
    }
}

/// One axis of a component, named and argued for.
#[derive(IntoElement)]
pub struct Section {
    title: SharedString,
    description: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl Section {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Section {
            title: title.into(),
            description: None,
            children: Vec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl ParentElement for Section {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Section {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_4)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(vars.spacing_2)
                    .pb(vars.spacing_3)
                    .border_b(hairline(vars, window))
                    .border_color(vars.color_border)
                    .child(
                        div()
                            .type_style(vars.title_medium)
                            .text_color(vars.color_content)
                            .child(self.title),
                    )
                    .when_some(self.description, |el, description| {
                        el.child(Prose::new(description))
                    }),
            )
            .children(self.children)
    }
}

/// A paragraph, held to the measure.
#[derive(IntoElement)]
pub struct Prose {
    text: SharedString,
}

impl Prose {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Prose { text: text.into() }
    }
}

impl RenderOnce for Prose {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .max_w(PROSE_MEASURE)
            .type_style(vars.body_small)
            .text_color(vars.color_content_muted)
            .child(self.text)
    }
}

/// A run of specimens.
#[derive(IntoElement)]
pub struct Specimens {
    children: Vec<AnyElement>,
}

impl Specimens {
    pub fn new() -> Self {
        Specimens {
            children: Vec::new(),
        }
    }
}

impl Default for Specimens {
    fn default() -> Self {
        Specimens::new()
    }
}

impl ParentElement for Specimens {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Specimens {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_3)
            .children(self.children)
    }
}

/// One demo row: what the variation is called, and the live component.
#[derive(IntoElement)]
pub struct Specimen {
    label: SharedString,
    children: Vec<AnyElement>,
}

impl Specimen {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Specimen {
            label: label.into(),
            children: Vec::new(),
        }
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
            .items_center()
            .gap(vars.spacing_4)
            .child(
                div()
                    .flex_none()
                    // The Storybook's specimen grid: a 6rem name column, then
                    // the stage.
                    .w(name_column(vars))
                    .type_style(code(vars.caption_medium))
                    .text_color(vars.color_content_muted)
                    .child(self.label),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_w_0()
                    .flex_wrap()
                    .items_center()
                    .gap(vars.spacing_3)
                    .children(self.children),
            )
    }
}

/// A grid of tiles, for the token pages where a shelf would run off the edge.
#[derive(IntoElement)]
pub struct Tiles {
    min: Pixels,
    children: Vec<AnyElement>,
}

impl Tiles {
    pub fn new(min: Pixels) -> Self {
        Tiles {
            min,
            children: Vec::new(),
        }
    }
}

impl ParentElement for Tiles {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tiles {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let min = self.min;

        div().flex().flex_wrap().gap(vars.spacing_4).children(
            self.children
                .into_iter()
                .map(move |child| div().w(min).child(child)),
        )
    }
}

/// The tokens a component owns, and what they resolve to in this theme.
///
/// It appears only where the component genuinely owns tokens — the files in
/// `packages/ds/tokens/component/`. Most components own none: they read the
/// shared vocabulary directly, and a table that would come back empty is not
/// worth a section.
#[derive(IntoElement)]
pub struct TokenTable {
    rows: Vec<(SharedString, SharedString)>,
}

impl TokenTable {
    pub fn new() -> Self {
        TokenTable { rows: Vec::new() }
    }

    pub fn row(mut self, name: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        self.rows.push((name.into(), value.into()));
        self
    }

    /// A length, printed the way the token graph states it.
    pub fn length(self, name: impl Into<SharedString>, value: Pixels) -> Self {
        self.row(name, px_label(value))
    }
}

impl Default for TokenTable {
    fn default() -> Self {
        TokenTable::new()
    }
}

impl RenderOnce for TokenTable {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div().max_w(PROSE_MEASURE).child(
            Table::new()
                .child(
                    TableHead::new()
                        .child(TableCell::new().head(true).child("Token"))
                        .child(
                            TableCell::new()
                                .head(true)
                                .align(dazzui::TableAlign::End)
                                .child("Resolved"),
                        ),
                )
                .children(self.rows.into_iter().map(|(name, value)| {
                    TableRow::new(name.clone())
                        .child(
                            TableCell::new().child(
                                div()
                                    .type_style(code(vars.caption_medium))
                                    .child(name.clone()),
                            ),
                        )
                        .child(
                            TableCell::new().align(dazzui::TableAlign::End).child(
                                div()
                                    .type_style(code(vars.caption_small))
                                    .text_color(vars.color_content_muted)
                                    .child(value),
                            ),
                        )
                })),
        )
    }
}

/// A colour role: the swatch, its name, and what it is for.
#[derive(IntoElement)]
pub struct Swatch {
    name: SharedString,
    color: Hsla,
    note: Option<SharedString>,
}

impl Swatch {
    pub fn new(name: impl Into<SharedString>, color: Hsla) -> Self {
        Swatch {
            name: name.into(),
            color,
            note: None,
        }
    }

    pub fn note(mut self, note: impl Into<SharedString>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl RenderOnce for Swatch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_2)
            .child(
                div()
                    .h(vars.spacing_14)
                    .rounded(vars.radius_medium)
                    .bg(self.color)
                    .border(vars.spacing_px)
                    .border_color(vars.color_border),
            )
            .child(
                div()
                    .type_style(code(vars.caption_medium))
                    .text_color(vars.color_content)
                    .child(self.name),
            )
            .when_some(self.note, |el, note| {
                el.child(
                    div()
                        .type_style(vars.caption_small)
                        .text_color(vars.color_content_muted)
                        .child(note),
                )
            })
    }
}

/// The eleven steps of one ramp, in the theme it is read against.
#[derive(IntoElement)]
pub struct Ramp {
    name: SharedString,
    swatch: ColorSwatch,
}

impl Ramp {
    pub fn new(name: impl Into<SharedString>, swatch: ColorSwatch) -> Self {
        Ramp {
            name: name.into(),
            swatch,
        }
    }
}

impl RenderOnce for Ramp {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_2)
            .child(
                div()
                    .type_style(vars.label_medium)
                    .text_color(vars.color_content)
                    .child(self.name),
            )
            .child(div().flex().flex_wrap().gap(vars.spacing_2).children(
                self.swatch.steps().iter().map(|(step, color)| {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(vars.spacing_1)
                        .child(
                            div()
                                .w(vars.spacing_12)
                                .h(vars.spacing_10)
                                .rounded(vars.radius_small)
                                .bg(*color)
                                .border(vars.spacing_px)
                                .border_color(vars.color_border),
                        )
                        .child(
                            div()
                                .type_style(code(vars.caption_small))
                                .text_color(vars.color_content_muted)
                                .child(step.to_string()),
                        )
                }),
            ))
    }
}

/// A framed preview with its name under it — a radius, a shadow, a face.
#[derive(IntoElement)]
pub struct Tile {
    name: SharedString,
    note: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl Tile {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Tile {
            name: name.into(),
            note: None,
            children: Vec::new(),
        }
    }

    pub fn note(mut self, note: impl Into<SharedString>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl ParentElement for Tile {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tile {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_3)
            .p(vars.spacing_4)
            .rounded(vars.radius_medium)
            .bg(vars.color_surface)
            .border(vars.spacing_px)
            .border_color(vars.color_border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .min_h(vars.spacing_20)
                    .children(self.children),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(vars.spacing_0_5)
                    .child(
                        div()
                            .type_style(code(vars.caption_medium))
                            .text_color(vars.color_content)
                            .child(self.name),
                    )
                    .when_some(self.note, |el, note| {
                        el.child(
                            div()
                                .type_style(vars.caption_small)
                                .text_color(vars.color_content_muted)
                                .child(note),
                        )
                    }),
            )
    }
}

/// A length, drawn to scale beside its name — the spacing scale's page.
#[derive(IntoElement)]
pub struct Bar {
    name: SharedString,
    length: Pixels,
}

impl Bar {
    pub fn new(name: impl Into<SharedString>, length: Pixels) -> Self {
        Bar {
            name: name.into(),
            length,
        }
    }
}

impl RenderOnce for Bar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .items_center()
            .gap(vars.spacing_4)
            .child(
                div()
                    .flex_none()
                    .w(name_column(vars))
                    .type_style(code(vars.caption_medium))
                    .text_color(vars.color_content)
                    .child(self.name),
            )
            .child(
                div()
                    .w(self.length.max(vars.spacing_px))
                    .h(vars.spacing_3)
                    .rounded(vars.radius_tiny)
                    .bg(vars.ramp(dazzui::Tint::Primary).shade(500)),
            )
            .child(
                div()
                    .type_style(code(vars.caption_small))
                    .text_color(vars.color_content_muted)
                    .child(px_label(self.length)),
            )
    }
}
