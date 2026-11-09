//! Window-wide editing commands. Text nodes retain identity across external
//! value assignments, as browser editing roots do: history never overwrites
//! an application's new value with an old whole-string snapshot.
//!
//! Grouping follows Blink's TypingCommand. Placeholder replay follows its
//! AppendNode/InsertNodeBefore/RemoveNode commands; see LICENSE-WEBKIT.
//! https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/core/editing/commands/

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};

use gpui::{EntityId, Global, WeakEntity, WindowId};

use super::*;

#[derive(Clone)]
struct Node {
    // None is a line break. A placeholder has empty text and zero length.
    content: Rc<RefCell<Option<String>>>,
    placeholder: bool,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.content, &other.content)
    }
}
impl Eq for Node {}

impl Node {
    fn text(text: String) -> Self {
        Self {
            content: Rc::new(RefCell::new(Some(text))),
            placeholder: false,
        }
    }
    fn line_break() -> Self {
        Self {
            content: Rc::new(RefCell::new(None)),
            placeholder: false,
        }
    }
    fn placeholder() -> Self {
        Self {
            content: Rc::new(RefCell::new(Some(String::new()))),
            placeholder: true,
        }
    }
    fn len(&self) -> usize {
        self.content.borrow().as_ref().map_or(1, String::len)
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Position {
    Root(usize),
    Before(Node),
    After(Node),
    Text(Node, usize), // UTF-16 offset, independently of subsequent node edits.
}

#[derive(Clone, PartialEq, Eq)]
struct Selection {
    start: Position,
    end: Position,
    reversed: bool,
    affinity: CaretAffinity,
}

#[derive(Default)]
pub(super) struct Document {
    nodes: Vec<Node>,
    multiline: bool,
}

fn byte_offset(text: &str, utf16: usize) -> usize {
    let mut units = 0;
    for (byte, ch) in text.char_indices() {
        if units >= utf16 {
            return byte;
        }
        units += ch.len_utf16();
    }
    text.len()
}

impl Document {
    pub(super) fn configure(&mut self, multiline: bool, text: &str) {
        if self.multiline != multiline {
            self.multiline = multiline;
            self.reset(text);
        }
    }

    pub(super) fn reset(&mut self, text: &str) {
        self.nodes.clear();
        for (i, line) in text.split('\n').enumerate() {
            if i > 0 {
                self.nodes.push(Node::line_break());
            }
            if !line.is_empty() {
                self.nodes.push(Node::text(line.to_owned()));
            }
        }
        self.placeholder_break();
    }

    fn normalize(&mut self) {
        while self.nodes.last().is_some_and(|node| {
            !node.placeholder && node.content.borrow().as_ref().is_some_and(String::is_empty)
        }) {
            self.nodes.pop();
        }
        self.placeholder_break();
    }

    fn placeholder_break(&mut self) {
        if (self.nodes.is_empty() && self.multiline
            || self
                .nodes
                .last()
                .is_some_and(|node| node.content.borrow().is_none()))
            && !self.nodes.last().is_some_and(|node| node.placeholder)
        {
            self.nodes.push(Node::placeholder());
        }
    }

    fn value(&self) -> String {
        let mut text = String::new();
        for node in &self.nodes {
            match &*node.content.borrow() {
                Some(part) => text.push_str(part),
                None => text.push('\n'),
            }
        }
        text
    }

    fn index(&self, node: &Node) -> Option<usize> {
        self.nodes.iter().position(|current| current == node)
    }

    fn position(&self, offset: usize) -> Position {
        let mut base = 0;
        for (i, node) in self.nodes.iter().enumerate() {
            if let Some(text) = &*node.content.borrow() {
                if offset <= base + text.len() {
                    let local = offset.saturating_sub(base);
                    return Position::Text(node.clone(), text[..local].encode_utf16().count());
                }
            } else if offset == base {
                return Position::Root(i);
            }
            base += node.len();
        }
        Position::Root(self.nodes.len())
    }

    fn resolve(&self, position: &Position) -> Option<usize> {
        match position {
            Position::Root(i) => Some(self.nodes.iter().take(*i).map(Node::len).sum()),
            Position::Before(node) | Position::After(node) => {
                let i = self.index(node)?;
                Some(
                    self.nodes[..i].iter().map(Node::len).sum::<usize>()
                        + if matches!(position, Position::After(_)) {
                            node.len()
                        } else {
                            0
                        },
                )
            }
            Position::Text(node, units) => {
                let i = self.index(node)?;
                let text = node.content.borrow();
                Some(
                    self.nodes[..i].iter().map(Node::len).sum::<usize>()
                        + byte_offset(text.as_ref()?, *units),
                )
            }
        }
    }

    fn editing_boundary(&self, offset: usize) -> usize {
        match self.position(offset) {
            Position::Text(node, units) => {
                let i = self.index(&node).unwrap();
                let content = node.content.borrow();
                let text = content.as_ref().unwrap();
                self.nodes[..i].iter().map(Node::len).sum::<usize>()
                    + boundary::ceil(text, byte_offset(text, units))
            }
            _ => offset,
        }
    }

    fn selection(&self, range: Range<usize>, reversed: bool, affinity: CaretAffinity) -> Selection {
        let mut selection = Selection {
            start: self.position(range.start),
            end: self.position(range.end),
            reversed,
            affinity,
        };
        if !range.is_empty() {
            let mut base = 0;
            for node in &self.nodes {
                if node.content.borrow().is_none() {
                    if base == range.start {
                        selection.start = Position::Before(node.clone());
                    }
                    if base + 1 == range.end {
                        selection.end = Position::After(node.clone());
                    }
                }
                base += node.len();
            }
        }
        selection
    }

    fn perform(&mut self, command: Command, commands: &mut Vec<Command>) {
        command.apply(self, false, true);
        commands.push(command);
    }

    /// Return the structural boundary created by typing into a new paragraph.
    /// Blink's first character there creates a node; further typing starts a
    /// fresh command. The initial node in an empty editor keeps typing open.
    fn replace(&mut self, range: Range<usize>, text: &str) -> (Vec<Command>, bool) {
        let mut commands = Vec::new();
        let placeholder = self.nodes.last().filter(|node| node.placeholder).cloned();
        let mut base = 0;
        for node in self.nodes.clone() {
            let end = base + node.len();
            let start = range.start.max(base);
            let stop = range.end.min(end);
            if start < stop {
                if start == base
                    && stop == end
                    && (text.is_empty()
                        || range.start < base
                        || range.end > end
                        || node.content.borrow().is_none())
                {
                    let index = self.index(&node).unwrap();
                    let before = self.nodes.get(index + 1).cloned();
                    self.perform(Command::detach(node, before), &mut commands);
                } else {
                    let (at, removed) = {
                        let part = node.content.borrow();
                        let part = part.as_ref().unwrap();
                        (
                            part[..start - base].encode_utf16().count(),
                            part[start - base..stop - base].to_owned(),
                        )
                    };
                    self.perform(
                        Command::Text {
                            node: node.clone(),
                            at,
                            removed,
                            inserted: String::new(),
                        },
                        &mut commands,
                    );
                }
            }
            base = end;
        }
        if text.is_empty() {
            self.normalize();
            return (commands, false);
        }
        let mut boundary = false;
        let mut offset = range.start;
        for (i, piece) in text.split('\n').enumerate() {
            if i > 0 {
                self.insert_break(offset, &mut commands);
                offset += 1;
            }
            if piece.is_empty() {
                continue;
            }
            let (node, at) = match self.position(offset) {
                Position::Text(placeholder, _) if placeholder.placeholder => {
                    boundary |= self.nodes.iter().any(|node| !node.placeholder);
                    let node = Node::text(String::new());
                    self.perform(
                        Command::Attach {
                            node: node.clone(),
                            before: Some(placeholder),
                        },
                        &mut commands,
                    );
                    (node, 0)
                }
                Position::Text(node, at) => (node, at),
                Position::Before(_) | Position::After(_) => unreachable!(),
                Position::Root(index) => {
                    // A text node immediately after this break can accept the
                    // insertion without allocating another paragraph node.
                    if let Some(node) = self
                        .nodes
                        .get(index)
                        .filter(|node| node.content.borrow().is_some())
                    {
                        (node.clone(), 0)
                    } else {
                        boundary |= !self.nodes.is_empty();
                        let node = Node::text(String::new());
                        let before = self.nodes.get(index).cloned();
                        self.perform(
                            Command::Attach {
                                node: node.clone(),
                                before,
                            },
                            &mut commands,
                        );
                        (node, 0)
                    }
                }
            };
            self.perform(
                Command::Text {
                    node,
                    at,
                    removed: String::new(),
                    inserted: piece.to_owned(),
                },
                &mut commands,
            );
            offset += piece.len();
        }
        if let Some(placeholder) = placeholder {
            if self.index(&placeholder).is_some() && !self.value().ends_with('\n') {
                self.perform(Command::detach(placeholder, None), &mut commands);
            }
        }
        if self
            .nodes
            .last()
            .is_some_and(|node| node.content.borrow().is_none())
        {
            self.perform(
                Command::Attach {
                    node: Node::placeholder(),
                    before: None,
                },
                &mut commands,
            );
        }
        (commands, boundary)
    }

    fn insert_break(&mut self, offset: usize, commands: &mut Vec<Command>) {
        let index = match self.position(offset) {
            Position::Before(_) | Position::After(_) => unreachable!(),
            Position::Root(i) => i,
            Position::Text(node, at) => {
                let index = self.index(&node).unwrap();
                let text = node.content.borrow().as_ref().unwrap().clone();
                let byte = byte_offset(&text, at);
                if node.placeholder {
                    index
                } else if byte == text.len() {
                    index + 1
                } else if byte == 0 {
                    index
                } else {
                    // Preserve the suffix node, as splitting a DOM text node
                    // does; the prefix and separator are inserted before it.
                    let prefix = text[..byte].to_owned();
                    self.perform(
                        Command::Text {
                            node: node.clone(),
                            at: 0,
                            removed: prefix.clone(),
                            inserted: String::new(),
                        },
                        commands,
                    );
                    self.perform(
                        Command::Attach {
                            node: Node::text(prefix),
                            before: Some(node),
                        },
                        commands,
                    );
                    index + 1
                }
            }
        };
        let before = self.nodes.get(index).cloned();
        self.perform(
            Command::Attach {
                node: Node::line_break(),
                before,
            },
            commands,
        );
    }
}

#[derive(Clone)]
enum Command {
    Attach {
        node: Node,
        before: Option<Node>,
    },
    Detach {
        node: Node,
        before: Option<Node>,
        restorable: Rc<Cell<bool>>,
    },
    Text {
        node: Node,
        at: usize,
        removed: String,
        inserted: String,
    },
}

impl Command {
    fn detach(node: Node, before: Option<Node>) -> Self {
        Self::Detach {
            node,
            before: before.filter(|node| !node.placeholder),
            restorable: Rc::new(Cell::new(false)),
        }
    }

    fn apply(&self, document: &mut Document, undo: bool, editable: bool) {
        match self {
            Self::Attach { node, before } | Self::Detach { node, before, .. } => {
                // A removed node releases its remembered parent on undo even
                // if the current editor has become readonly. Replaying later
                // cannot reconnect an already detached reference sibling.
                let parent = match self {
                    Self::Detach { restorable, .. } if undo => restorable.replace(false),
                    _ => true,
                };
                if !editable || !parent {
                    return;
                }
                if matches!(self, Self::Detach { .. })
                    && undo
                    && before.is_none()
                    && document.nodes.last().is_some_and(|node| node.placeholder)
                {
                    document.nodes.pop();
                }
                let index = match before {
                    Some(before) => match document.index(before) {
                        Some(index) => index,
                        None => return,
                    },
                    None => document.nodes.len(),
                };
                let attach = matches!(self, Self::Attach { .. }) != undo;
                if attach {
                    if document.index(node).is_none() {
                        document.nodes.insert(index, node.clone());
                    }
                } else if let Some(index) = document.index(node) {
                    document.nodes.remove(index);
                    if let Self::Detach { restorable, .. } = self {
                        restorable.set(true);
                    }
                }
            }
            Self::Text {
                node,
                at,
                removed,
                inserted,
            } => {
                if !editable || document.index(node).is_none() {
                    return;
                }
                let (remove, insert) = if undo {
                    (inserted, removed)
                } else {
                    (removed, inserted)
                };
                let mut part = node.content.borrow_mut();
                let Some(text) = part.as_mut() else { return };
                if *at > text.encode_utf16().count() {
                    return;
                }
                let start = byte_offset(text, *at);
                let end = byte_offset(text, at + remove.encode_utf16().count());
                text.replace_range(start..end, insert);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Edit {
    Typing,
    Insert,
    Backward,
    Forward,
    WordBackward,
    WordForward,
    Cut,
    Paste,
    Composition,
    Commit,
    Newline,
}

impl Edit {
    fn open(self) -> bool {
        !matches!(self, Self::Insert | Self::Cut | Self::Paste | Self::Commit)
    }
    fn last(self) -> Self {
        match self {
            Self::WordBackward => Self::Backward,
            Self::WordForward => Self::Forward,
            Self::Composition | Self::Commit => Self::Typing,
            _ => self,
        }
    }
}

struct Step {
    target: WeakEntity<InputState>,
    commands: Vec<Command>,
    before: Selection,
    after: Selection,
    open: bool,
    last: Edit,
    backward: bool,
}

#[derive(Default)]
pub(super) struct History {
    undo: VecDeque<Step>,
    redo: Vec<Step>,
}

#[derive(Default)]
struct Histories(HashMap<WindowId, Weak<RefCell<History>>>);
impl Global for Histories {}

pub(super) fn shared(window: &Window, cx: &mut App) -> Rc<RefCell<History>> {
    let histories = cx.default_global::<Histories>();
    histories.0.retain(|_, history| history.strong_count() > 0);
    let id = window.window_handle().window_id();
    if let Some(history) = histories.0.get(&id).and_then(Weak::upgrade) {
        return history;
    }
    let history = Rc::new(RefCell::new(History::default()));
    histories.0.insert(id, Rc::downgrade(&history));
    history
}

impl History {
    pub(super) fn close(&mut self) {
        if let Some(step) = self.undo.back_mut() {
            step.open = false;
        }
    }
    pub(super) fn remove(&mut self, owner: EntityId) {
        self.undo.retain(|step| step.target.entity_id() != owner);
        self.redo.retain(|step| step.target.entity_id() != owner);
    }
}

impl InputState {
    fn history_selection(&self) -> Selection {
        self.document.selection(
            self.selected_range.clone(),
            self.selection_reversed,
            self.caret_affinity,
        )
    }

    pub(crate) fn close_history(&self) {
        self.history.borrow_mut().close();
    }

    /// Preserve both ends of the action's selection before deletion expands a
    /// caret. Mac undo selects removed text; other platforms restore that caret.
    pub(super) fn edit_document(
        &mut self,
        range: Range<usize>,
        text: &str,
        edit: Edit,
        before_range: Range<usize>,
        before_reversed: bool,
        cx: &mut Context<Self>,
    ) {
        // Composition replaces its marked range while the visible selection
        // may be only a caret inside it. Blink updates the open command with
        // that replacement selection before every preedit/commit insertion.
        let before =
            if matches!(edit, Edit::Composition | Edit::Commit) && self.marked_range.is_some() {
                self.document
                    .selection(range.clone(), false, self.caret_affinity)
            } else {
                self.document
                    .selection(before_range.clone(), before_reversed, self.caret_affinity)
            };
        let mut undo_selection = self.document.selection(
            range.clone(),
            before_reversed && !before_range.is_empty(),
            self.caret_affinity,
        );
        let mut history = self.history.borrow_mut();
        let append = history.undo.back().is_some_and(|step| {
            step.target.entity_id() == cx.entity_id()
                && step.open
                && match edit {
                    Edit::Typing
                    | Edit::Forward
                    | Edit::Composition
                    | Edit::Commit
                    | Edit::Newline => true,
                    Edit::Backward => step.last == Edit::Backward,
                    _ => false,
                }
        });
        if append {
            let step = history.undo.back_mut().unwrap();
            if step.after != before {
                step.before = before.clone();
            }
            if cfg!(target_os = "macos") {
                if edit == Edit::Backward
                    && step.backward
                    && undo_selection.end == step.before.start
                {
                    undo_selection.end = step.before.end.clone();
                } else if edit == Edit::Forward
                    && range.start > 0
                    && undo_selection.start == step.before.start
                {
                    // Forward deletion grows the restored span in its original
                    // node. At root start Blink uses a distinct root anchor.
                    if let (Position::Text(node, at), Position::Text(old, end)) =
                        (&undo_selection.end, &step.before.end)
                    {
                        if node == old {
                            undo_selection.end = Position::Text(
                                node.clone(),
                                at + end
                                    - match &undo_selection.start {
                                        Position::Text(_, start) => *start,
                                        _ => 0,
                                    },
                            );
                        }
                    }
                }
                if edit == Edit::Forward || edit == Edit::Backward && step.backward {
                    step.before = undo_selection.clone();
                }
            }
        }
        let (commands, structural_boundary) = self.document.replace(range.clone(), text);
        self.content = self.document.value();
        let mut cursor = range.start + text.len();
        if matches!(
            edit,
            Edit::Backward | Edit::Forward | Edit::WordBackward | Edit::WordForward
        ) {
            cursor = self.document.editing_boundary(cursor);
        }
        self.selected_range = cursor..cursor;
        self.selection_reversed = false;
        self.caret_affinity = CaretAffinity::default();
        let after =
            self.document
                .selection(self.selected_range.clone(), false, self.caret_affinity);
        history.redo.clear();
        if append {
            let step = history.undo.back_mut().unwrap();
            step.commands.extend(commands);
            step.after = after;
            step.last = edit.last();
            step.open = edit.open() && !structural_boundary;
        } else {
            history.close();
            let backward = matches!(edit, Edit::Backward | Edit::WordBackward);
            let select_deleted = cfg!(target_os = "macos")
                && (backward || matches!(edit, Edit::Forward | Edit::WordForward));
            history.undo.push_back(Step {
                target: cx.entity().downgrade(),
                commands,
                before: if select_deleted {
                    undo_selection
                } else {
                    before
                },
                after,
                open: edit.open() && !structural_boundary,
                last: edit.last(),
                backward,
            });
            if history.undo.len() > 1000 {
                history.undo.pop_front();
            }
        }
    }

    pub(super) fn update_history_selection(&mut self) {
        let selection = self.history_selection();
        if let Some(step) = self.history.borrow_mut().undo.back_mut() {
            step.after = selection;
        }
    }

    fn replay(&mut self, step: &Step, undo: bool, window: &mut Window, cx: &mut Context<Self>) {
        let old = self.content.clone();
        let editable = self.editable();
        if undo {
            for command in step.commands.iter().rev() {
                command.apply(&mut self.document, true, editable);
            }
        } else {
            for command in &step.commands {
                command.apply(&mut self.document, false, editable);
            }
        }
        self.document.normalize();
        self.content = self.document.value();
        let selection = if undo { &step.before } else { &step.after };
        if let (Some(start), Some(end)) = (
            self.document.resolve(&selection.start),
            self.document.resolve(&selection.end),
        ) {
            self.selected_range = start.min(end)..start.max(end);
            self.selection_reversed = selection.reversed;
            self.caret_affinity = selection.affinity;
        } else {
            self.selected_range = self
                .content
                .ceil_char_boundary(self.selected_range.start.min(self.content.len()))
                ..self
                    .content
                    .ceil_char_boundary(self.selected_range.end.min(self.content.len()));
        }
        if self.selected_range.is_empty() {
            self.selection_reversed = false;
        }
        self.marked_range = None;
        self.is_selecting = false;
        self.selection_drag = None;
        self.preferred_x = None;
        self.reveal_cursor = true;
        window.focus(&self.focus_handle);
        cx.notify();
        if self.content != old {
            self.emit_change(window, cx);
        }
    }

    fn history_action(&mut self, undo: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        let step = {
            let mut history = self.history.borrow_mut();
            history.close();
            if undo {
                history.undo.pop_back()
            } else {
                history.redo.pop()
            }
        };
        let Some(mut step) = step else { return };
        step.open = false;
        if step.target.entity_id() == cx.entity_id() {
            self.replay(&step, undo, window, cx);
        } else if let Some(target) = step.target.upgrade() {
            target.update(cx, |target, cx| target.replay(&step, undo, window, cx));
        } else {
            return;
        }
        let mut history = self.history.borrow_mut();
        if undo {
            history.redo.push(step);
        } else {
            history.undo.push_back(step);
        }
    }

    pub(super) fn undo(&mut self, _: &Undo, window: &mut Window, cx: &mut Context<Self>) {
        self.history_action(true, window, cx);
    }
    pub(super) fn redo(&mut self, _: &Redo, window: &mut Window, cx: &mut Context<Self>) {
        self.history_action(false, window, cx);
    }
}
