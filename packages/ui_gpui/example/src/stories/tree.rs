//! `Components/Tree` — a hierarchy the reader walks.

use dazzui::{Tint, Tree, TreeNode, TreeSize, assets::icons};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act_with;
use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::{Story, StoryPage};

fn file(id: &'static str, label: &'static str) -> TreeNode {
    TreeNode::leaf(id, label).icon(icons::DOCUMENT_16_REGULAR)
}

fn folder(id: &'static str, label: &'static str, children: Vec<TreeNode>) -> TreeNode {
    TreeNode::branch(id, label, children).icon(icons::FOLDER_16_REGULAR)
}

fn files() -> Vec<TreeNode> {
    vec![
        folder(
            "src",
            "src",
            vec![
                folder(
                    "components",
                    "components",
                    vec![
                        file("badge.tsx", "badge.tsx"),
                        file("button.tsx", "button.tsx"),
                        file("tree.tsx", "tree.tsx"),
                    ],
                ),
                folder(
                    "styles",
                    "styles",
                    vec![
                        file("base.css", "base.css"),
                        file("components.css", "components.css"),
                    ],
                ),
                file("index.ts", "index.ts"),
            ],
        ),
        folder("empty", "generated", vec![]),
        file("package.json", "package.json"),
        file("README.md", "README.md"),
    ]
}

fn changed() -> Vec<TreeNode> {
    vec![
        folder(
            "src2",
            "src",
            vec![
                file("a", "tree.tsx").meta("M"),
                file("b", "tree.css").meta("A"),
                file("c", "menu.tsx").meta("+12 −3"),
            ],
        ),
        file("d", "AGENTS.md").meta("locked").disabled(true),
    ]
}

fn opened() -> Vec<SharedString> {
    vec![SharedString::from("src"), SharedString::from("components")]
}

pub const PAGE: StoryPage = StoryPage::new(
    "Tree",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("Tree")
                .description(
                    "A hierarchy the reader walks. React's is the one component in the package \
                     with no Base UI part under it, because a tree's keyboard is not a list's. \
                     GPUI has no focus ring to rove, so what is here is the part the design \
                     decides — the indent, the twisty's column, the selection's wash and \
                     weight — and the arrow-key walk belongs to a host that owns focus.",
                )
                .child(
                    Section::new("Branches and leaves")
                        .description(
                            "children present at all — an empty list included — is what makes a \
                             node a branch. That is how an empty folder still draws a twisty \
                             and still opens, which is the difference between “nothing in here” \
                             and “not a folder”. The twisty keeps its column on a leaf too, \
                             empty: files and folders at one level start at one x.",
                        )
                        .child(
                            div()
                                .w(px(288.))
                                .child(Tree::new("doc-files", files()).expanded(opened())),
                        ),
                )
                .child(
                    Section::new("Two targets, two meanings")
                        .description(
                            "Clicking a folder opens it and selects it, because in a file tree \
                             those are one gesture. Clicking the twisty alone opens it and \
                             leaves the selection where it was — which is how a reader looks \
                             inside a folder without losing the file they were reading.",
                        )
                        .child(
                            div().w(px(288.)).child(
                                Tree::new("doc-two", files())
                                    .expanded(opened())
                                    .selected("tree.tsx"),
                            ),
                        ),
                )
                .child(
                    Section::new("Selection")
                        .description(
                            "Drawn the way every selected thing in this system is: the tinted \
                             wash, and the label at the label weight rather than the quiet one. \
                             On a column of forty rows the weight is doing most of the work — \
                             one grade of colour is hardest to find in exactly this shape.",
                        )
                        .child(
                            div().w(px(288.)).child(
                                Tree::new("doc-changed", changed())
                                    .expanded(vec![SharedString::from("src2")])
                                    .selected("a"),
                            ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("File tree", |state, this, _| {
            let open = state.tree_open.clone();
            let picked = state.tree_pick.clone();
            div()
                .w(px(288.))
                .child(
                    Tree::new("files", files())
                        .expanded(open.clone())
                        .selected(picked)
                        .on_select(act_with(this, move |state, id: &SharedString, _, _| {
                            state.tree_pick = id.clone();
                        }))
                        .on_toggle(act_with(this, move |state, id: &SharedString, _, _| {
                            if state.tree_open.contains(id) {
                                state.tree_open.retain(|one| one != id);
                            } else {
                                state.tree_open.push(id.clone());
                            }
                        })),
                )
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            div()
                .flex()
                .flex_col()
                .gap(px(16.))
                .w(px(288.))
                .child(
                    Tree::new("small", files())
                        .expanded(opened())
                        .size(TreeSize::Small)
                        .selected("tree.tsx"),
                )
                .child(
                    Tree::new("medium", files())
                        .expanded(opened())
                        .selected("tree.tsx"),
                )
                .into_any_element()
        }),
        Story::new("With meta", |_, _, _| {
            div()
                .w(px(288.))
                .child(
                    Tree::new("meta", changed())
                        .expanded(vec![SharedString::from("src2")])
                        .selected("a"),
                )
                .into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            div()
                .flex()
                .flex_col()
                .gap(px(16.))
                .w(px(288.))
                .children(Tint::ALL.iter().enumerate().map(|(index, tint)| {
                    Tree::new(SharedString::from(format!("tint-{index}")), files())
                        .expanded(vec![SharedString::from("src")])
                        .selected("index.ts")
                        .tint(*tint)
                }))
                .into_any_element()
        }),
        Story::new("Collapsed", |_, _, _| {
            div()
                .w(px(288.))
                .child(Tree::new("collapsed", files()))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            div()
                .w(px(288.))
                .child(Tree::new("disabled", changed()).expanded(vec![SharedString::from("src2")]))
                .into_any_element()
        }),
    ],
);
