//! `Components/CodeBlock` — a block of code, read rather than edited.

use dazzui::{Button, CodeBlock, ControlSize, ControlVariant};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::{Story, StoryPage};

const SAMPLE: &str = "export function Badge({ size = 'medium', tint = 'primary' }: BadgeProps) {\n  \
                      return <span data-size={size} data-tint={tint} className=\"dz-badge\" />\n}";

const SHELL: &str = "pnpm install\npnpm tokens\npnpm parity";

const WIDE: &str = "const themes = ['studio-dark', 'bright-light', 'bright-dark', \
                    'frost-light', 'frost-dark', 'graphite-light']";

pub const PAGE: StoryPage = StoryPage::new(
    "CodeBlock",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("CodeBlock")
                .description(
                    "A block of code, read rather than edited. It takes the card's materials \
                     rather than the popup's — this sits in the page, not over it — and its \
                     head is drawn only where there is something to put in it. A bar holding \
                     one label is furniture that says less than the space it takes.",
                )
                .child(
                    Section::new("The gutter is a column")
                        .description(
                            "Line numbers live in their own column rather than in the same run \
                             as the code, which is what lets a selection that crosses ten lines \
                             take ten lines of code and not ten line numbers.",
                        )
                        .child(
                            div().w(px(480.)).child(
                                CodeBlock::new(SAMPLE)
                                    .line_numbers(true)
                                    .title("src/components/badge.tsx"),
                            ),
                        ),
                )
                .child(
                    Section::new("No highlighter")
                        .description(
                            "language names the grammar at the trailing edge of the head and \
                             does nothing else. This package ships no highlighter, because a \
                             grammar per language is a dependency a design system has no \
                             opinion about.",
                        )
                        .child(
                            div()
                                .w(px(480.))
                                .child(CodeBlock::new(SHELL).language("bash")),
                        ),
                )
                .child(
                    Section::new("What this port leaves out")
                        .description(
                            "The copy button writes the block to the clipboard and then stays a \
                             copy button: React swaps its glyph for a tick and clears it a \
                             second and a half later, and a tick here would need a timer to \
                             un-set it. There is no sideways scroller either — how wide a \
                             listing may be is the page's decision on this platform.",
                        )
                        .child(div().w(px(480.)).child(CodeBlock::new(WIDE).language("ts"))),
                )
                .into_any_element()
        }),
        Story::new("Plain", |_, _, _| {
            div()
                .w(px(480.))
                .child(CodeBlock::new(SHELL))
                .into_any_element()
        }),
        Story::new("With head", |_, _, _| {
            div()
                .w(px(480.))
                .child(
                    CodeBlock::new(SAMPLE)
                        .language("tsx")
                        .title("src/components/badge.tsx"),
                )
                .into_any_element()
        }),
        Story::new("Line numbers", |_, _, _| {
            div()
                .w(px(480.))
                .child(CodeBlock::new(SAMPLE).line_numbers(true))
                .into_any_element()
        }),
        Story::new("Copyable", |_, _, _| {
            div()
                .w(px(480.))
                .child(CodeBlock::new(SHELL).copyable(true).language("bash"))
                .into_any_element()
        }),
        Story::new("With actions", |_, _, _| {
            div()
                .w(px(480.))
                .child(
                    CodeBlock::new(SHELL)
                        .title("install.sh")
                        .copyable(true)
                        .action(
                            Button::new("code-run", "Run")
                                .size(ControlSize::Tiny)
                                .variant(ControlVariant::Plain),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Wrapped", |_, _, _| {
            div()
                .w(px(384.))
                .child(CodeBlock::new(WIDE).language("ts"))
                .into_any_element()
        }),
    ],
);
