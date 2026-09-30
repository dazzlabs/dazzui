//! The code face, at two scales: a run inside a sentence, and a block read
//! rather than edited.

use std::time::Duration;

use gpui::{
    AnyElement, App, ClickEvent, ClipboardItem, ElementId, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::components::icon::{Icon, icon_slot};
use crate::foundation::focusable;
use crate::glyphs;
use crate::theme::ActiveTheme;
use crate::tokens::Face;

/// A run of code inside a sentence — a flag, a path, an identifier. A wash
/// and a face, not a box: the chip corner and the tiny inset keep it
/// separable from the prose without breaking the line's rhythm.
#[derive(IntoElement, Default)]
pub struct Code {
    children: Vec<AnyElement>,
}

impl Code {
    pub fn new() -> Self {
        Code::default()
    }
}

impl ParentElement for Code {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Code {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        // A step under the prose it sits in: a monospace face at the same
        // nominal size always reads larger.
        let style = t.caption_small;
        div()
            .flex()
            .flex_none()
            .py(px(t.spacing_0_5))
            .px(px(t.spacing_1))
            .bg(t.color_surface_sunken.hsla())
            .rounded(px(t.radius_tiny))
            .text_color(t.color_content.hsla())
            .font_family(Face::Mono.family())
            .text_size(px(style.size))
            .line_height(px(style.line_height))
            .whitespace_nowrap()
            .children(self.children)
    }
}

/// A block of code, read rather than edited.
///
/// The gutter is a column of its own, so a selection that crosses ten lines
/// would copy ten lines of code and not ten line numbers. A head is drawn
/// only where there is something to put in it.
#[derive(IntoElement)]
pub struct CodeBlock {
    id: ElementId,
    code: SharedString,
    title: Option<AnyElement>,
    language: Option<SharedString>,
    line_numbers: bool,
    wrap: bool,
    copyable: bool,
    actions: Vec<AnyElement>,
}

impl CodeBlock {
    /// `code` is the source, verbatim; trailing newlines are trimmed.
    pub fn new(id: impl Into<ElementId>, code: impl Into<SharedString>) -> Self {
        CodeBlock {
            id: id.into(),
            code: code.into(),
            title: None,
            language: None,
            line_numbers: false,
            wrap: false,
            copyable: false,
            actions: Vec::new(),
        }
    }

    /// What the block is — a path, a filename, a shell's name.
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }

    /// Named at the trailing edge of the head, and nothing more: the kit
    /// ships no highlighter.
    pub fn language(mut self, language: impl Into<SharedString>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn line_numbers(mut self, line_numbers: bool) -> Self {
        self.line_numbers = line_numbers;
        self
    }

    /// Wraps long lines instead of scrolling them.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Offers the whole block to the clipboard.
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
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let body: SharedString = self.code.trim_end_matches('\n').to_string().into();
        let lines: Vec<SharedString> = body
            .split('\n')
            .map(|line| SharedString::from(line.to_string()))
            .collect();
        let has_head = self.title.is_some()
            || self.language.is_some()
            || !self.actions.is_empty()
            || self.copyable;
        let code_size = t.caption_small.size;
        let leading = t.body_small.line_height;

        let head = has_head.then(|| {
            let copy = self
                .copyable
                .then(|| copy_button(self.id.clone(), body.clone(), window, cx));
            // The run between title, language and actions is a margin on
            // each rather than a `gap`: beside the actions' auto margin,
            // taffy lays the row out without its gaps.
            let has_title = self.title.is_some();
            div()
                .flex()
                .items_center()
                .min_h(px(t.spacing_8))
                .py(px(t.spacing_1))
                .px(px(t.spacing_3))
                .bg(t.color_surface_chrome.hsla())
                .border_b(px(t.stroke_hairline))
                .border_color(t.color_border.hsla())
                .when_some(self.title, |this, title| {
                    this.child(
                        div()
                            .min_w_0()
                            .whitespace_nowrap()
                            .text_color(t.color_content_secondary.hsla())
                            .font_family(Face::Mono.family())
                            .text_size(px(code_size))
                            .line_height(px(t.caption_small.line_height))
                            .child(title),
                    )
                })
                .when_some(self.language, |this, language| {
                    this.child(
                        div()
                            .flex_none()
                            .when(has_title, |this| this.ml(px(t.spacing_2)))
                            .text_color(t.color_content_faint.hsla())
                            .font_family(t.label_strong.face.family())
                            .text_size(px(t.label_small.size))
                            .font_weight(t.label_medium.font_weight())
                            .line_height(px(t.label_small.line_height))
                            .child(SharedString::from(language.to_uppercase())),
                    )
                })
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(t.spacing_0_5))
                        .pl(px(t.spacing_2))
                        .ml_auto()
                        .children(self.actions)
                        .children(copy),
                )
        });

        // One box per line, on the same leading in both columns: that is what
        // keeps a number beside its line.
        let line_box = |text: SharedString| div().h(px(leading)).flex_none().child(text);
        let gutter = self.line_numbers.then(|| {
            div()
                .flex()
                .flex_col()
                .flex_none()
                .items_end()
                .min_w(px(t.spacing_10))
                .pr(px(t.spacing_3))
                .bg(t.color_surface_muted.hsla())
                .text_color(t.color_content_faint.hsla())
                .children((1..=lines.len()).map(|n| line_box(SharedString::from(n.to_string()))))
        });
        let wrap = self.wrap;
        let pre = div()
            .flex()
            .flex_col()
            .px(px(t.spacing_3))
            .when(self.line_numbers, |this| this.pl_0())
            .text_color(t.color_content.hsla())
            .map(|this| {
                if wrap {
                    this.flex_1().min_w_0().children(
                        lines
                            .iter()
                            .map(|line| div().min_h(px(leading)).child(line.clone())),
                    )
                } else {
                    this.flex_none()
                        .whitespace_nowrap()
                        .children(lines.iter().map(|line| line_box(line.clone())))
                }
            });

        div()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(t.color_surface_muted.hsla())
            .border(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
            .rounded(px(t.control_container_radius))
            .children(head)
            .child(
                div()
                    .id(self.id)
                    .flex()
                    .items_start()
                    .when(!wrap, |this| this.overflow_x_scroll())
                    .py(px(t.spacing_2_5))
                    .font_family(Face::Mono.family())
                    .text_size(px(code_size))
                    .line_height(px(leading))
                    .children(gutter)
                    .child(pre),
            )
    }
}

/// The head's copy button: `IconButton` at its defaults — tiny, plain,
/// neutral — whose glyph turns to a tick for a moment once the clipboard
/// has the code.
fn copy_button(
    id: ElementId,
    body: SharedString,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let t = cx.theme();
    let button_id = ElementId::NamedChild(Box::new(id), "copy".into());
    let copied = window.use_keyed_state(button_id.clone(), cx, |_, _| false);
    let focus = window
        .use_keyed_state(
            ElementId::NamedChild(Box::new(button_id.clone()), "focus".into()),
            cx,
            |_, cx| cx.focus_handle(),
        )
        .read(cx)
        .clone();
    let glyph = if *copied.read(cx) {
        glyphs::CHECKMARK_16_REGULAR
    } else {
        glyphs::COPY_16_REGULAR
    };
    let hovered = t.color_surface_subtle.hsla();
    let hovered_ink = t.color_content.hsla();
    let button = div()
        .id(button_id)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(t.control_tiny_size))
        .rounded(px(t.control_tiny_radius))
        .border(px(t.spacing_px))
        .border_color(gpui::transparent_black())
        .text_color(t.color_content_subtle.hsla())
        .cursor_pointer()
        .hover(move |style| style.bg(hovered).text_color(hovered_ink))
        .on_click(move |_: &ClickEvent, _, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(body.to_string()));
            copied.update(cx, |copied, cx| {
                *copied = true;
                cx.notify();
            });
            let copied = copied.clone();
            cx.spawn(async move |cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(1600))
                    .await;
                let _ = copied.update(cx, |copied, cx| {
                    *copied = false;
                    cx.notify();
                });
            })
            .detach();
        });
    let button = focusable(button, &focus, false, window, cx);
    icon_slot(t.control_tiny_icon_size, button.child(Icon::new(glyph)))
}
