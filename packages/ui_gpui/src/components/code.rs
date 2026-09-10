use gpui::{
    AnyElement, App, ClipboardItem, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, px,
};

use crate::assets::icons;
use crate::components::{Icon, IconButton};
use crate::foundation::TypeStyle;
use crate::prelude::*;

/// The code face.
///
/// The one place a component reaches a `base.*` primitive, exactly as
/// `text-field.css` does for a `mono` field: `base.font.code` has no semantic
/// token in front of it, because the code face is not a role — it is the face,
/// and a component that asked for it by any other name would be asking for
/// something else.
fn code_face(base: TypeStyle) -> TypeStyle {
    TypeStyle {
        family: &["ui-monospace"],
        ..base
    }
}

/// A run of code inside a sentence — a flag, a path, an identifier.
///
/// A wash and a face, not a box: the chip corner and the half-step inset keep
/// it separable from the prose around it without breaking the line's rhythm,
/// which is what a border at this size always does.
#[derive(IntoElement)]
pub struct Code {
    code: SharedString,
}

impl Code {
    pub fn new(code: impl Into<SharedString>) -> Self {
        Code { code: code.into() }
    }
}

impl RenderOnce for Code {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_none()
            .items_center()
            .px(vars.spacing_1)
            .py(vars.spacing_0_5)
            .rounded(vars.radius_tiny)
            .bg(vars.color_surface_sunken)
            .text_color(vars.color_content)
            // A step under the prose it sits in: a monospace face at the same
            // nominal size always reads larger, and the run would ride above
            // the line.
            .type_style(code_face(vars.caption_small))
            .child(self.code)
    }
}

/// A block of code, read rather than edited.
///
/// The gutter is a column of its own rather than text in the same run, which
/// is what lets a selection that crosses ten lines take ten lines of code and
/// not ten line numbers — the single thing a reader does with one of these.
///
/// A head is drawn only where there is something to put in it. A block with no
/// title and no language is the code alone, because a bar holding one label
/// says less than the space it takes.
///
/// The copy button writes the block to the system clipboard and does not
/// change afterwards. React swaps its glyph for a tick and clears it a second
/// and a half later; a tick here would need a timer to un-set it, and a tick
/// that never cleared would be worse than none.
///
/// There is no scroller either: React's block scrolls sideways through
/// `overflow`, and how tall or how wide a listing is allowed to be is the
/// page's decision on this platform rather than the listing's.
#[derive(IntoElement)]
pub struct CodeBlock {
    code: SharedString,
    title: Option<SharedString>,
    language: Option<SharedString>,
    line_numbers: bool,
    copyable: bool,
    actions: Vec<AnyElement>,
}

impl CodeBlock {
    pub fn new(code: impl Into<SharedString>) -> Self {
        CodeBlock {
            code: code.into(),
            title: None,
            language: None,
            line_numbers: false,
            copyable: false,
            actions: Vec::new(),
        }
    }

    /// What the block is — a path, a filename, a shell's name.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Named at the trailing edge of the head, and nothing more: this package
    /// ships no highlighter, because a grammar per language is a dependency a
    /// design system has no opinion about.
    pub fn language(mut self, language: impl Into<SharedString>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn line_numbers(mut self, line_numbers: bool) -> Self {
        self.line_numbers = line_numbers;
        self
    }

    /// Offers the whole block to the system clipboard.
    pub fn copyable(mut self, copyable: bool) -> Self {
        self.copyable = copyable;
        self
    }

    /// Extra affordances in the head, before the copy button.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}

impl RenderOnce for CodeBlock {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let body = self.code.trim_end_matches('\n').to_string();
        let lines: Vec<String> = body.lines().map(str::to_string).collect();
        let head = self.title.is_some()
            || self.language.is_some()
            || self.copyable
            || !self.actions.is_empty();
        let copyable = self.copyable;
        let actions = self.actions;
        let clipboard = body.clone();

        // Both columns are set in one face at one size with one leading, and
        // that is what aligns them: a number and its line are two boxes
        // agreeing on a line box, not one box holding both.
        let face = TypeStyle {
            line_height: vars.body_small.line_height,
            ..code_face(vars.caption_small)
        };

        div()
            .flex()
            .flex_col()
            .overflow_hidden()
            // The card's materials, not the popup's: this sits *in* the page
            // rather than over it, so it takes the muted surface and the
            // ordinary border rather than the overlay's edge and shadow.
            .bg(vars.color_surface_muted)
            .border(vars.stroke_hairline)
            .border_color(vars.color_border)
            .rounded(vars.radius_medium)
            .when(head, |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(vars.spacing_2)
                        .min_h(vars.spacing_8)
                        .px(vars.spacing_3)
                        .py(vars.spacing_1)
                        // Chrome, and it reads as chrome: the titlebar's step
                        // and a rule under it.
                        .bg(vars.color_surface_chrome)
                        .border_b(vars.stroke_hairline)
                        .border_color(vars.color_border)
                        .when_some(self.title.clone(), |el, title| {
                            el.child(
                                div()
                                    // Set in the code face too: the title is
                                    // nearly always a path, and a proportional
                                    // face beside a monospace body reads as a
                                    // different document rather than as its
                                    // label.
                                    .type_style(code_face(vars.caption_small))
                                    .text_color(vars.color_content_secondary)
                                    .child(title),
                            )
                        })
                        .child(div().flex_1())
                        .when_some(self.language.clone(), |el, language| {
                            el.child(
                                div()
                                    .type_style(vars.label_small.weight(vars.label_medium.weight))
                                    .text_color(vars.color_content_faint)
                                    .child(SharedString::from(language.to_uppercase())),
                            )
                        })
                        .children(actions)
                        .when(copyable, |el| {
                            el.child(
                                IconButton::new(
                                    "code-block-copy",
                                    "Copy",
                                    Icon::new(icons::COPY_16_REGULAR),
                                )
                                .on_click(move |_, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        clipboard.clone(),
                                    ));
                                }),
                            )
                        }),
                )
            })
            .child(
                div()
                    .flex()
                    .items_start()
                    .py(vars.spacing_2_5)
                    .when(self.line_numbers, |el| {
                        el.child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_none()
                                // Right-aligned so the ones column stays a
                                // column past line 9, and wide enough for four
                                // digits before it pushes the code across.
                                .w(vars.spacing_10)
                                .pr(vars.spacing_3)
                                .text_right()
                                .type_style(face)
                                .text_color(vars.color_content_faint)
                                .children(
                                    (1..=lines.len().max(1))
                                        .map(|line| SharedString::from(line.to_string())),
                                ),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .when(!self.line_numbers, |el| el.pl(vars.spacing_3))
                            .pr(vars.spacing_3)
                            .type_style(face)
                            .text_color(vars.color_content)
                            .children(lines.into_iter().map(|line| {
                                // A blank line still occupies its line box, so
                                // the gutter beside it stays in step.
                                div().h(face.line_height).child(SharedString::from(
                                    if line.is_empty() { " ".into() } else { line },
                                ))
                            })),
                    ),
            )
            .min_w(px(0.))
    }
}
