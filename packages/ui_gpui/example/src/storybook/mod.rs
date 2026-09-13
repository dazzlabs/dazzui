//! The playground is organised the way `packages/ui_react/example` is, and for the same
//! reason: a gallery of six themed pages tells you what the system looks like,
//! but not where a component's states are written down. One page per
//! component, an `Overview` that argues for it, and every other story the
//! component and nothing else.
//!
//! The three types below are what Storybook's file conventions are here: a
//! [`Story`] is one export, a [`StoryPage`] is one `*.stories.tsx` file, and a
//! [`StoryGroup`] is the `Tokens/` or `Components/` half of the sidebar.

pub mod gallery;
pub mod stage;

use gpui::{AnyElement, Context, Entity};

use crate::Playground;
use crate::stories::GROUPS;

/// What a story draws.
///
/// A plain function pointer rather than a boxed closure, so the whole index is
/// a `const`: a story reads the playground's state and hands back an element,
/// and has nothing of its own to capture.
pub type StoryFn = fn(&Playground, &Entity<Playground>, &mut Context<Playground>) -> AnyElement;

/// How the canvas presents a story.
///
/// The counterpart of Storybook's `parameters.layout`, and it carries the same
/// three cases: a bare specimen sits in the middle of the canvas, a page brings
/// its own padding, and a story whose subject is fixed to the viewport — a
/// scrim, a toast viewport — takes the canvas whole.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoryLayout {
    /// A bare specimen, centred. The default, as it is on a component file.
    Centered,
    /// A documented page. It scrolls, and pads itself.
    Page,
    /// The canvas, filled and unscrolled — what a portal target would hold.
    Fullscreen,
}

/// A story's Storybook id — `components-button--all-variants` — derived the
/// way Storybook derives it from a title and an export name: lower-cased,
/// with every run of anything but a letter or a digit collapsed to one dash.
///
/// The id is the key the React reference is written under, so it is what
/// makes a story here and the story in `packages/ui_react/example` the same story. A
/// name that differs only in case or spacing (`All variants`, `All
/// Variants`) lands on the same id; a name that differs in a word does not.
pub fn story_id(group: &str, page: &str, story: &str) -> String {
    format!(
        "{}--{}",
        sanitize(&format!("{group}/{page}")),
        sanitize(story)
    )
}

fn sanitize(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        if character.is_ascii_alphanumeric() {
            out.push(character.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Every story's id, in sidebar order — what the parity test holds against
/// the React list.
pub fn story_ids() -> Vec<String> {
    GROUPS
        .iter()
        .flat_map(|group| {
            group.pages.iter().flat_map(move |page| {
                page.stories.iter().map(move |story| {
                    story
                        .source_id
                        .map(str::to_owned)
                        .unwrap_or_else(|| story_id(group.name, page.name, story.name))
                })
            })
        })
        .collect()
}

/// One story: a name in the sidebar and the element it draws.
pub struct Story {
    pub source_id: Option<&'static str>,
    pub name: &'static str,
    pub layout: StoryLayout,
    pub render: StoryFn,
}

impl Story {
    pub const fn new(name: &'static str, render: StoryFn) -> Self {
        Story {
            source_id: None,
            name,
            layout: StoryLayout::Centered,
            render,
        }
    }

    /// The documented page. One per component, and it is always `Overview`.
    pub const fn page(name: &'static str, render: StoryFn) -> Self {
        Story {
            source_id: None,
            name,
            layout: StoryLayout::Page,
            render,
        }
    }

    pub const fn fullscreen(name: &'static str, render: StoryFn) -> Self {
        Story {
            source_id: None,
            name,
            layout: StoryLayout::Fullscreen,
            render,
        }
    }
}

/// One component's stories — the `Overview` first, then the axes, then the
/// compositions, in the order they are written.
pub struct StoryPage {
    pub name: &'static str,
    pub stories: &'static [Story],
}

impl StoryPage {
    pub const fn new(name: &'static str, stories: &'static [Story]) -> Self {
        StoryPage { name, stories }
    }
}

/// A half of the sidebar: `Tokens` or `Components`.
pub struct StoryGroup {
    pub name: &'static str,
    pub pages: &'static [StoryPage],
}

impl StoryGroup {
    pub const fn new(name: &'static str, pages: &'static [StoryPage]) -> Self {
        StoryGroup { name, pages }
    }
}

/// Where the canvas is pointed: a page and one of its stories.
///
/// A page is addressed by name rather than by index, because the sidebar
/// filters and a filtered list would renumber underneath the selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoryRef {
    pub page: &'static str,
    pub story: &'static str,
}

impl StoryRef {
    pub const fn new(page: &'static str, story: &'static str) -> Self {
        StoryRef { page, story }
    }
}

/// The story the canvas is pointed at, if the index still holds it.
pub fn story(reference: StoryRef) -> Option<&'static Story> {
    GROUPS
        .iter()
        .flat_map(|group| group.pages)
        .find(|page| page.name == reference.page)?
        .stories
        .iter()
        .find(|story| story.name == reference.story)
}

/// The component pages by name. A story that needs a long column of rows takes
/// the playground's own index rather than inventing a list — an invented one is
/// the first thing that reads as fake.
pub fn component_names() -> impl Iterator<Item = &'static str> {
    GROUPS
        .iter()
        .filter(|group| group.name == "Components")
        .flat_map(|group| group.pages)
        .map(|page| page.name)
}
