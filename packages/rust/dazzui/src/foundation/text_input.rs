//! The editable text every field is built on.
//!
//! The web hands a field the browser's own editing — a caret, a selection, an
//! input method, the clipboard — and GPUI hands it nothing: editing is a
//! view's job there. So this is the one piece of the kit with no counterpart
//! in `@dazzlabs/dazzui`. It owns the string, the selection and the focus; it
//! draws no box. [`TextField`](crate::TextField) draws the box around it, and
//! every other text control — a search field, a number field, the cells of a
//! code — wears the same editing rather than a second implementation of it.
//!
//! The text takes its face, size, leading and colour from the text style it
//! is laid out in, the way an `<input>` inherits `font` from the box around
//! it, so a wrapper styles the box and the text follows.
//!
//! Two pieces: [`InputState`], an entity holding the value and the selection
//! across frames, and [`TextInput`], the element that edits one. Keys arrive
//! as actions bound in [`init`]; typed characters arrive through the
//! platform's input handler, which is what keeps input methods working.

use std::ops::Range;
use std::rc::Rc;

use gpui::{
    Action, App, AvailableSpace, Bounds, ClipboardItem, ContentMask, Context, CursorStyle,
    DispatchPhase, Element, ElementId, ElementInputHandler, Entity, EntityInputHandler,
    FocusHandle, Focusable, GlobalElementId, Hsla, InspectorElementId, InteractiveElement,
    IntoElement, KeyBinding, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ParentElement, Pixels, Point, RenderOnce, SharedString, Style, Styled, Subscription, TextAlign,
    TextRun, TextStyle, UTF16Selection, UnderlineStyle, Window, WrappedLine, actions, div, fill,
    point, prelude::FluentBuilder, px, relative, size,
};

use crate::foundation::scope::scoped;

fn editor_line_height(window: &Window) -> Pixels {
    let style = window.text_style();
    // Window::line_height rounds to whole logical pixels. Keep the declared
    // leading for wrapping, selection and input-method rectangles alike.
    style
        .line_height
        .to_pixels(style.font_size, window.rem_size())
}

actions!(
    dazzui_text_input,
    [
        Backspace,
        Delete,
        DeleteWordLeft,
        DeleteWordRight,
        DeleteToLineStart,
        Left,
        Right,
        Up,
        Down,
        SelectLeft,
        SelectRight,
        SelectUp,
        SelectDown,
        WordLeft,
        WordRight,
        SelectWordLeft,
        SelectWordRight,
        LineStart,
        LineEnd,
        SelectToLineStart,
        SelectToLineEnd,
        DocumentStart,
        DocumentEnd,
        SelectToDocumentStart,
        SelectToDocumentEnd,
        SelectAll,
        Copy,
        Cut,
        Paste,
        Enter,
        ShowCharacterPalette,
    ]
);

/// The key context every [`TextInput`] declares; its bindings apply only
/// inside it, so a key a field does not use still reaches the page.
pub const CONTEXT: &str = "DazzTextInput";

/// Binds the editing keys. [`crate::init`] calls it.
pub fn init(cx: &mut App) {
    // The platform's own modifiers: ⌘ for the line and the clipboard and ⌥
    // for the word on a Mac, Ctrl for both elsewhere.
    let (primary, word) = if cfg!(target_os = "macos") {
        ("cmd", "alt")
    } else {
        ("ctrl", "ctrl")
    };
    fn key<A: Action>(keys: impl AsRef<str>, action: A) -> KeyBinding {
        KeyBinding::new(keys.as_ref(), action, Some(CONTEXT))
    }
    let mut bindings = vec![
        key("backspace", Backspace),
        key("shift-backspace", Backspace),
        key("delete", Delete),
        key(format!("{word}-backspace"), DeleteWordLeft),
        key(format!("{word}-delete"), DeleteWordRight),
        key("left", Left),
        key("right", Right),
        key("up", Up),
        key("down", Down),
        key("shift-left", SelectLeft),
        key("shift-right", SelectRight),
        key("shift-up", SelectUp),
        key("shift-down", SelectDown),
        key(format!("{word}-left"), WordLeft),
        key(format!("{word}-right"), WordRight),
        key(format!("{word}-shift-left"), SelectWordLeft),
        key(format!("{word}-shift-right"), SelectWordRight),
        key("home", LineStart),
        key("end", LineEnd),
        key("shift-home", SelectToLineStart),
        key("shift-end", SelectToLineEnd),
        key(format!("{primary}-home"), DocumentStart),
        key(format!("{primary}-end"), DocumentEnd),
        key(format!("{primary}-a"), SelectAll),
        key(format!("{primary}-c"), Copy),
        key(format!("{primary}-x"), Cut),
        key(format!("{primary}-v"), Paste),
        key("enter", Enter),
        key("shift-enter", Enter),
    ];
    if cfg!(target_os = "macos") {
        bindings.extend([
            key("cmd-backspace", DeleteToLineStart),
            key("cmd-left", LineStart),
            key("cmd-right", LineEnd),
            key("cmd-shift-left", SelectToLineStart),
            key("cmd-shift-right", SelectToLineEnd),
            key("cmd-up", DocumentStart),
            key("cmd-down", DocumentEnd),
            key("cmd-shift-up", SelectToDocumentStart),
            key("cmd-shift-down", SelectToDocumentEnd),
            key("ctrl-a", LineStart),
            key("ctrl-e", LineEnd),
            key("ctrl-cmd-space", ShowCharacterPalette),
        ]);
    }
    cx.bind_keys(bindings);
}

/// A value callback: the text as it now stands.
pub type OnText = Rc<dyn Fn(&str, &mut Window, &mut App)>;
/// A callback with nothing to pass.
pub type OnEvent = Rc<dyn Fn(&mut Window, &mut App)>;

/// What a form hands the fields inside it: Return in any one of them
/// submits it, the way Return in an `<input>` submits its `<form>`.
#[derive(Clone)]
pub struct SubmitScope(pub OnEvent);

/// The bullet a password is drawn in, one per character.
const MASK: char = '•';

/// The value, the selection and the focus of one editable text, kept across
/// frames. A component gets one with [`input_state`] and hands it to its
/// [`TextInput`]; a host that wants to drive a field — focus it, read it,
/// set it — keeps the entity.
pub struct InputState {
    focus_handle: FocusHandle,
    content: String,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    initialized: bool,
    is_selecting: bool,

    // Set by the element every frame.
    placeholder: SharedString,
    multiline: bool,
    masked: bool,
    read_only: bool,
    disabled: bool,
    max_length: Option<usize>,
    select_on_focus: bool,
    on_change: Option<OnText>,
    on_submit: Option<OnText>,
    on_blur: Option<OnEvent>,

    // What the element last laid out, in display offsets.
    lines: Vec<WrappedLine>,
    line_height: Pixels,
    text_origin: Point<Pixels>,
    bounds: Option<Bounds<Pixels>>,
    scroll_x: Pixels,

    _subscriptions: Vec<Subscription>,
}

/// The state keyed by `id` in the element being rendered: created the first
/// time, the same entity every frame after.
pub fn input_state(
    id: impl Into<ElementId>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputState> {
    window.use_keyed_state(id, cx, InputState::new)
}

impl InputState {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let subscriptions = vec![
            cx.on_focus(&focus_handle, window, |this, _, cx| {
                if this.select_on_focus {
                    this.selected_range = 0..this.content.len();
                    this.selection_reversed = false;
                    cx.notify();
                }
            }),
            cx.on_blur(&focus_handle, window, |this, window, cx| {
                this.is_selecting = false;
                if let Some(on_blur) = this.on_blur.clone() {
                    on_blur(window, cx);
                }
                cx.notify();
            }),
        ];
        InputState {
            focus_handle,
            content: String::new(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            initialized: false,
            is_selecting: false,
            placeholder: SharedString::default(),
            multiline: false,
            masked: false,
            read_only: false,
            disabled: false,
            max_length: None,
            select_on_focus: false,
            on_change: None,
            on_submit: None,
            on_blur: None,
            lines: Vec::new(),
            line_height: px(0.),
            text_origin: Point::default(),
            bounds: None,
            scroll_x: px(0.),
            _subscriptions: subscriptions,
        }
    }

    /// The text as it stands.
    pub fn value(&self) -> &str {
        &self.content
    }

    /// Replaces the text and puts the caret at its end. Not an edit: no
    /// change is reported.
    pub fn set_value(&mut self, value: impl Into<String>, cx: &mut Context<Self>) {
        self.replace_value(value.into());
        cx.notify();
    }

    fn replace_value(&mut self, value: String) {
        if value != self.content {
            self.content = value;
            let end = self.content.len();
            self.selected_range = end..end;
            self.selection_reversed = false;
            self.marked_range = None;
        }
    }

    /// The selection, as byte offsets into [`value`](Self::value).
    pub fn selected_range(&self) -> Range<usize> {
        self.selected_range.clone()
    }

    pub fn focus(&self, window: &mut Window) {
        window.focus(&self.focus_handle);
    }

    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    /// Selects everything.
    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        self.selected_range = 0..self.content.len();
        self.selection_reversed = false;
        cx.notify();
    }

    fn editable(&self) -> bool {
        !self.read_only && !self.disabled
    }

    fn emit_change(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(on_change) = self.on_change.clone() {
            on_change(&self.content, window, cx);
        }
    }

    // Movement ------------------------------------------------------------

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        cx.notify();
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify();
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content[..offset]
            .char_indices()
            .next_back()
            .map(|(index, _)| index)
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content[offset..]
            .chars()
            .next()
            .map(|ch| offset + ch.len_utf8())
            .unwrap_or(self.content.len())
    }

    /// The start of the word before `offset`: skip the gap, then the word. A
    /// password has no words, only its two ends.
    fn previous_word(&self, offset: usize) -> usize {
        if self.masked {
            return 0;
        }
        let mut chars = self.content[..offset].char_indices().rev().peekable();
        while chars.next_if(|(_, ch)| !is_word(*ch)).is_some() {}
        let mut start = chars.peek().map(|(i, ch)| i + ch.len_utf8()).unwrap_or(0);
        while let Some((index, _)) = chars.next_if(|(_, ch)| is_word(*ch)) {
            start = index;
        }
        start
    }

    fn next_word(&self, offset: usize) -> usize {
        if self.masked {
            return self.content.len();
        }
        let mut chars = self.content[offset..].char_indices().peekable();
        while chars.next_if(|(_, ch)| !is_word(*ch)).is_some() {}
        while chars.next_if(|(_, ch)| is_word(*ch)).is_some() {}
        chars
            .peek()
            .map(|(i, _)| offset + i)
            .unwrap_or(self.content.len())
    }

    fn line_start(&self, offset: usize) -> usize {
        self.content[..offset]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0)
    }

    fn line_end(&self, offset: usize) -> usize {
        self.content[offset..]
            .find('\n')
            .map(|i| offset + i)
            .unwrap_or(self.content.len())
    }

    /// The offset a row up or down from `offset`, at the same x. A single
    /// line has no row above or below, so it goes to an end, as AppKit does.
    fn vertical(&self, offset: usize, rows: f32) -> usize {
        if !self.multiline {
            return if rows < 0. { 0 } else { self.content.len() };
        }
        let Some(position) = self.local_position(self.display_offset(offset)) else {
            return offset;
        };
        let target = point(position.x, position.y + self.line_height * (rows + 0.5));
        if target.y < px(0.) {
            return 0;
        }
        self.content_offset(self.index_at_local(target))
    }

    fn word_range(&self, offset: usize) -> Range<usize> {
        if self.masked {
            return 0..self.content.len();
        }
        let is_word_at = |i: usize| self.content[i..].chars().next().is_some_and(is_word);
        let mut start = offset;
        let mut end = offset;
        if is_word_at(offset) || (offset > 0 && is_word_at(self.previous_boundary(offset))) {
            while start > 0 && is_word_at(self.previous_boundary(start)) {
                start = self.previous_boundary(start);
            }
            while end < self.content.len() && is_word_at(end) {
                end = self.next_boundary(end);
            }
        } else {
            end = self.next_boundary(offset);
        }
        start..end
    }

    // Display offsets -----------------------------------------------------
    //
    // A password is laid out as bullets, which are wider in bytes than what
    // they stand for, so the layout is addressed in display offsets and the
    // value in content offsets, and these two convert.

    fn display_text(&self) -> String {
        if self.masked {
            self.content.chars().map(|_| MASK).collect()
        } else {
            self.content.clone()
        }
    }

    fn display_offset(&self, offset: usize) -> usize {
        if self.masked {
            self.content[..offset.min(self.content.len())]
                .chars()
                .count()
                * MASK.len_utf8()
        } else {
            offset
        }
    }

    fn content_offset(&self, offset: usize) -> usize {
        if self.masked {
            self.content
                .char_indices()
                .nth(offset / MASK.len_utf8())
                .map(|(i, _)| i)
                .unwrap_or(self.content.len())
        } else {
            offset.min(self.content.len())
        }
    }

    // Layout --------------------------------------------------------------

    /// Where a display offset sits, relative to the text's origin.
    fn local_position(&self, offset: usize) -> Option<Point<Pixels>> {
        if self.lines.is_empty() {
            return Some(Point::default());
        }
        let mut y = px(0.);
        let mut start = 0;
        for line in &self.lines {
            let end = start + line.len();
            if offset <= end {
                let position = line.position_for_index(offset - start, self.line_height)?;
                return Some(point(position.x, y + position.y));
            }
            y += line.size(self.line_height).height;
            start = end + 1;
        }
        None
    }

    /// The display offset nearest a point relative to the text's origin.
    fn index_at_local(&self, local: Point<Pixels>) -> usize {
        let mut y = px(0.);
        let mut start = 0;
        let count = self.lines.len();
        for (index, line) in self.lines.iter().enumerate() {
            let height = line.size(self.line_height).height;
            if local.y < y + height || index + 1 == count {
                let within = point(local.x, (local.y - y).max(px(0.)).min(height - px(0.01)));
                let found = line
                    .closest_index_for_position(within, self.line_height)
                    .unwrap_or_else(|index| index);
                return start + found.min(line.len());
            }
            y += height;
            start += line.len() + 1;
        }
        0
    }

    fn index_for_mouse(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        self.content_offset(self.index_at_local(position - self.text_origin))
    }

    // Pointer -------------------------------------------------------------

    /// A press in the field — on the text or anywhere in the box around it.
    pub fn mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        crate::foundation::pointer_down(window, cx);
        window.focus(&self.focus_handle);
        let offset = self.index_for_mouse(event.position);
        match event.click_count {
            2 => {
                self.selected_range = self.word_range(offset);
                self.selection_reversed = false;
                cx.notify();
            }
            n if n >= 3 => {
                self.selected_range = if self.multiline {
                    self.line_start(offset)..self.line_end(offset)
                } else {
                    0..self.content.len()
                };
                self.selection_reversed = false;
                cx.notify();
            }
            _ => {
                self.is_selecting = true;
                if event.modifiers.shift {
                    self.select_to(offset, cx);
                } else {
                    self.move_to(offset, cx);
                }
            }
        }
    }

    fn mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if self.is_selecting {
            let offset = self.index_for_mouse(event.position);
            self.select_to(offset, cx);
        }
    }

    fn mouse_up(&mut self, _: &MouseUpEvent, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    // Actions -------------------------------------------------------------

    fn delete_to(&mut self, offset: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(offset, cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        self.delete_to(self.previous_boundary(self.cursor_offset()), window, cx);
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        self.delete_to(self.next_boundary(self.cursor_offset()), window, cx);
    }

    fn delete_word_left(
        &mut self,
        _: &DeleteWordLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.delete_to(self.previous_word(self.cursor_offset()), window, cx);
    }

    fn delete_word_right(
        &mut self,
        _: &DeleteWordRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.delete_to(self.next_word(self.cursor_offset()), window, cx);
    }

    fn delete_to_line_start(
        &mut self,
        _: &DeleteToLineStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.delete_to(self.line_start(self.cursor_offset()), window, cx);
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.vertical(self.cursor_offset(), -1.), cx);
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.vertical(self.cursor_offset(), 1.), cx);
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.vertical(self.cursor_offset(), -1.), cx);
    }

    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.vertical(self.cursor_offset(), 1.), cx);
    }

    fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.previous_word(self.cursor_offset()), cx);
    }

    fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.next_word(self.cursor_offset()), cx);
    }

    fn select_word_left(&mut self, _: &SelectWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_word(self.cursor_offset()), cx);
    }

    fn select_word_right(&mut self, _: &SelectWordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_word(self.cursor_offset()), cx);
    }

    fn line_start_action(&mut self, _: &LineStart, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.line_start(self.cursor_offset()), cx);
    }

    fn line_end_action(&mut self, _: &LineEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.line_end(self.cursor_offset()), cx);
    }

    fn select_to_line_start(
        &mut self,
        _: &SelectToLineStart,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(self.line_start(self.cursor_offset()), cx);
    }

    fn select_to_line_end(&mut self, _: &SelectToLineEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.line_end(self.cursor_offset()), cx);
    }

    fn document_start(&mut self, _: &DocumentStart, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn document_end(&mut self, _: &DocumentEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    fn select_to_document_start(
        &mut self,
        _: &SelectToDocumentStart,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(0, cx);
    }

    fn select_to_document_end(
        &mut self,
        _: &SelectToDocumentEnd,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(self.content.len(), cx);
    }

    fn select_all_action(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.select_all(cx);
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        // A password is never handed to the clipboard, as a browser never
        // hands one over.
        if !self.selected_range.is_empty() && !self.masked {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() && !self.masked && self.editable() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx);
        }
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    fn enter(&mut self, _: &Enter, window: &mut Window, cx: &mut Context<Self>) {
        if self.multiline {
            self.replace_text_in_range(None, "\n", window, cx);
        } else if let Some(on_submit) = self.on_submit.clone() {
            on_submit(&self.content, window, cx);
        } else {
            // Nothing here answers Return, so the page may.
            cx.propagate();
        }
    }

    fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }

    // Offsets for the platform, which counts in UTF-16 --------------------

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8 = 0;
        let mut utf16 = 0;
        for ch in self.content.chars() {
            if utf16 >= offset {
                break;
            }
            utf16 += ch.len_utf16();
            utf8 += ch.len_utf8();
        }
        utf8
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16 = 0;
        let mut utf8 = 0;
        for ch in self.content.chars() {
            if utf8 >= offset {
                break;
            }
            utf8 += ch.len_utf8();
            utf16 += ch.len_utf16();
        }
        utf16
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    /// `new_text` as this field accepts it: one line unless it is multiline,
    /// and no longer than what `max_length` leaves room for.
    fn accept(&self, range: &Range<usize>, new_text: &str) -> String {
        let mut text = if self.multiline {
            new_text.replace("\r\n", "\n")
        } else {
            new_text.replace("\r\n", " ").replace('\n', " ")
        };
        if let Some(max) = self.max_length {
            let kept = self.content.chars().count() - self.content[range.clone()].chars().count();
            let room = max.saturating_sub(kept);
            text = text.chars().take(room).collect();
        }
        text
    }
}

fn is_word(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

impl Focusable for InputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for InputState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable() {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range| self.range_from_utf16(range))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let text = self.accept(&range, new_text);
        if text.is_empty() && range.is_empty() && self.marked_range.is_none() {
            return;
        }
        self.content.replace_range(range.clone(), &text);
        let cursor = range.start + text.len();
        self.selected_range = cursor..cursor;
        self.selection_reversed = false;
        self.marked_range = None;
        cx.notify();
        self.emit_change(window, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable() {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range| self.range_from_utf16(range))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let text = self.accept(&range, new_text);
        self.content.replace_range(range.clone(), &text);
        self.marked_range = (!text.is_empty()).then(|| range.start..range.start + text.len());
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|selected| {
                // The selection is relative to the marked text, in UTF-16.
                let start = text
                    .char_indices()
                    .scan(0, |utf16, (i, ch)| {
                        let at = *utf16;
                        *utf16 += ch.len_utf16();
                        Some((at, i))
                    })
                    .find(|(at, _)| *at >= selected.start)
                    .map(|(_, i)| i)
                    .unwrap_or(text.len());
                let end = text
                    .char_indices()
                    .scan(0, |utf16, (i, ch)| {
                        let at = *utf16;
                        *utf16 += ch.len_utf16();
                        Some((at, i))
                    })
                    .find(|(at, _)| *at >= selected.end)
                    .map(|(_, i)| i)
                    .unwrap_or(text.len());
                range.start + start..range.start + end
            })
            .unwrap_or_else(|| {
                let cursor = range.start + text.len();
                cursor..cursor
            });
        self.selection_reversed = false;
        cx.notify();
        self.emit_change(window, cx);
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _: Bounds<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let range = self.range_from_utf16(&range_utf16);
        let start = self.local_position(self.display_offset(range.start))?;
        let end = self.local_position(self.display_offset(range.end))?;
        let end = if end.y > start.y {
            point(start.x, start.y)
        } else {
            end
        };
        Some(Bounds::from_corners(
            self.text_origin + start,
            self.text_origin + point(end.x, end.y + self.line_height),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        let offset = self.content_offset(self.index_at_local(point - self.text_origin));
        Some(self.offset_to_utf16(offset))
    }
}

/// An editable text, drawn in the text style it is laid out in. It draws
/// no box: that is the field's, which is what lets one editing serve every
/// text control.
///
/// Controlled with [`value`](Self::value) and [`on_change`](Self::on_change),
/// or left to keep its own text from [`default_value`](Self::default_value)
/// — React's split.
#[derive(IntoElement)]
pub struct TextInput {
    id: ElementId,
    state: Option<Entity<InputState>>,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    placeholder: SharedString,
    multiline: bool,
    rows: usize,
    masked: bool,
    read_only: bool,
    disabled: bool,
    max_length: Option<usize>,
    select_on_focus: bool,
    centered: bool,
    placeholder_color: Option<Hsla>,
    selection_color: Option<Hsla>,
    caret_color: Option<Hsla>,
    on_change: Option<OnText>,
    on_submit: Option<OnText>,
    on_blur: Option<OnEvent>,
}

impl TextInput {
    pub fn new(id: impl Into<ElementId>) -> Self {
        TextInput {
            id: id.into(),
            state: None,
            value: None,
            default_value: None,
            placeholder: SharedString::default(),
            multiline: false,
            rows: 1,
            masked: false,
            read_only: false,
            disabled: false,
            max_length: None,
            select_on_focus: false,
            centered: false,
            placeholder_color: None,
            selection_color: None,
            caret_color: None,
            on_change: None,
            on_submit: None,
            on_blur: None,
        }
    }

    /// Edit this state rather than one keyed by the id — for a component that
    /// needs the entity itself, to focus it or to read it.
    pub fn state(mut self, state: Entity<InputState>) -> Self {
        self.state = Some(state);
        self
    }

    /// The text, controlled: what this says is what is shown.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The text it starts with, uncontrolled.
    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Return inserts a line, and the text wraps.
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// The rows a multiline text is tall at least.
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    /// Draws each character as a bullet and keeps it off the clipboard —
    /// `type="password"`.
    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    /// Focusable and selectable, never edited.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Neither focusable nor edited.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The most characters it holds.
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Selects everything when focus arrives, so typing replaces it.
    pub fn select_on_focus(mut self, select_on_focus: bool) -> Self {
        self.select_on_focus = select_on_focus;
        self
    }

    /// Centres a single line in the room it has.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// The placeholder's ink; the text's own at half strength otherwise.
    pub fn placeholder_color(mut self, color: impl Into<Hsla>) -> Self {
        self.placeholder_color = Some(color.into());
        self
    }

    pub fn selection_color(mut self, color: impl Into<Hsla>) -> Self {
        self.selection_color = Some(color.into());
        self
    }

    /// The caret's ink; the text's own otherwise, as `caret-color: auto`.
    pub fn caret_color(mut self, color: impl Into<Hsla>) -> Self {
        self.caret_color = Some(color.into());
        self
    }

    /// Called after every edit with the text as it now stands.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Called on Return in a single line. Without it, Return submits the
    /// enclosing form, if there is one.
    pub fn on_submit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Rc::new(handler));
        self
    }

    /// Called when focus leaves.
    pub fn on_blur(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_blur = Some(Rc::new(handler));
        self
    }
}

fn act<A: Action>(
    state: &Entity<InputState>,
    f: fn(&mut InputState, &A, &mut Window, &mut Context<InputState>),
) -> impl Fn(&A, &mut Window, &mut App) + 'static {
    let state = state.downgrade();
    move |action, window, cx| {
        state
            .update(cx, |state, cx| f(state, action, window, cx))
            .ok();
    }
}

impl RenderOnce for TextInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = match self.state {
            Some(state) => state,
            None => input_state(self.id.clone(), window, cx),
        };
        let on_submit = self.on_submit.or_else(|| {
            scoped::<SubmitScope>(cx).map(|scope| {
                let submit = scope.0;
                Rc::new(move |_: &str, window: &mut Window, cx: &mut App| submit(window, cx))
                    as OnText
            })
        });
        let disabled = self.disabled;
        let multiline = self.multiline;
        state.update(cx, |state, _| {
            if let Some(value) = &self.value {
                state.replace_value(value.to_string());
            } else if !state.initialized
                && let Some(value) = &self.default_value
            {
                state.replace_value(value.to_string());
            }
            state.initialized = true;
            state.placeholder = self.placeholder;
            state.multiline = multiline;
            state.masked = self.masked;
            state.read_only = self.read_only;
            state.disabled = disabled;
            state.max_length = self.max_length;
            state.select_on_focus = self.select_on_focus;
            state.on_change = self.on_change;
            state.on_submit = on_submit;
            state.on_blur = self.on_blur;
            if disabled {
                state.is_selecting = false;
            }
        });

        let body = TextBody {
            input: state.clone(),
            rows: if multiline { self.rows } else { 1 },
            centered: self.centered && !multiline,
            placeholder_color: self.placeholder_color,
            selection_color: self.selection_color,
            caret_color: self.caret_color,
        };
        let focus = state.read(cx).focus_handle.clone();

        div()
            .id(self.id)
            .flex_1()
            .min_w_0()
            .when(!disabled, |this| {
                this.key_context(CONTEXT)
                    .track_focus(&focus.tab_stop(true))
                    .cursor(CursorStyle::IBeam)
                    .on_mouse_down(MouseButton::Left, {
                        let state = state.downgrade();
                        move |event, window, cx| {
                            cx.stop_propagation();
                            state
                                .update(cx, |state, cx| state.mouse_down(event, window, cx))
                                .ok();
                        }
                    })
                    .on_action(act(&state, InputState::backspace))
                    .on_action(act(&state, InputState::delete))
                    .on_action(act(&state, InputState::delete_word_left))
                    .on_action(act(&state, InputState::delete_word_right))
                    .on_action(act(&state, InputState::delete_to_line_start))
                    .on_action(act(&state, InputState::left))
                    .on_action(act(&state, InputState::right))
                    .on_action(act(&state, InputState::up))
                    .on_action(act(&state, InputState::down))
                    .on_action(act(&state, InputState::select_left))
                    .on_action(act(&state, InputState::select_right))
                    .on_action(act(&state, InputState::select_up))
                    .on_action(act(&state, InputState::select_down))
                    .on_action(act(&state, InputState::word_left))
                    .on_action(act(&state, InputState::word_right))
                    .on_action(act(&state, InputState::select_word_left))
                    .on_action(act(&state, InputState::select_word_right))
                    .on_action(act(&state, InputState::line_start_action))
                    .on_action(act(&state, InputState::line_end_action))
                    .on_action(act(&state, InputState::select_to_line_start))
                    .on_action(act(&state, InputState::select_to_line_end))
                    .on_action(act(&state, InputState::document_start))
                    .on_action(act(&state, InputState::document_end))
                    .on_action(act(&state, InputState::select_to_document_start))
                    .on_action(act(&state, InputState::select_to_document_end))
                    .on_action(act(&state, InputState::select_all_action))
                    .on_action(act(&state, InputState::copy))
                    .on_action(act(&state, InputState::cut))
                    .on_action(act(&state, InputState::paste))
                    .on_action(act(&state, InputState::enter))
                    .on_action(act(&state, InputState::show_character_palette))
            })
            .child(body)
    }
}

/// The text, the caret and the selection: the part of a field GPUI has no
/// element for.
struct TextBody {
    input: Entity<InputState>,
    rows: usize,
    centered: bool,
    placeholder_color: Option<Hsla>,
    selection_color: Option<Hsla>,
    caret_color: Option<Hsla>,
}

struct BodyLayout {
    lines: Vec<WrappedLine>,
    line_height: Pixels,
    origin: Point<Pixels>,
    caret: Option<Bounds<Pixels>>,
    selection: Vec<Bounds<Pixels>>,
    caret_color: Hsla,
    selection_color: Hsla,
}

/// The text shaped into lines — the placeholder's when there is none —
/// and whether it was the placeholder.
fn shape(
    input: &InputState,
    style: &TextStyle,
    placeholder_color: Hsla,
    wrap_width: Option<Pixels>,
    window: &Window,
) -> (Vec<WrappedLine>, bool) {
    let empty = input.content.is_empty();
    let text: SharedString = if empty {
        input.placeholder.clone()
    } else {
        input.display_text().into()
    };
    let color = if empty {
        placeholder_color
    } else {
        style.color
    };
    let run = TextRun {
        len: text.len(),
        font: style.font(),
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    // Text an input method is still composing is underlined until it is
    // committed.
    let marked = input
        .marked_range
        .as_ref()
        .filter(|_| !empty)
        .map(|range| input.display_offset(range.start)..input.display_offset(range.end));
    let runs = match marked {
        Some(marked) => [
            TextRun {
                len: marked.start,
                ..run.clone()
            },
            TextRun {
                len: marked.end - marked.start,
                underline: Some(UnderlineStyle {
                    color: Some(color),
                    thickness: px(1.),
                    wavy: false,
                }),
                ..run.clone()
            },
            TextRun {
                len: text.len() - marked.end,
                ..run
            },
        ]
        .into_iter()
        .filter(|run| run.len > 0)
        .collect(),
        None => vec![run],
    };
    let font_size = style.font_size.to_pixels(window.rem_size());
    let lines = window
        .text_system()
        .shape_text(text, font_size, &runs, wrap_width, None)
        .map(|lines| lines.into_iter().collect())
        .unwrap_or_default();
    (lines, empty)
}

impl IntoElement for TextBody {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for TextBody {
    type RequestLayoutState = TextStyle;
    type PrepaintState = BodyLayout;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, TextStyle) {
        let text_style = window.text_style();
        let line_height = editor_line_height(window);
        let mut style = Style::default();
        style.size.width = relative(1.).into();

        if self.rows <= 1 && !self.input.read(cx).multiline {
            style.size.height = line_height.into();
            return (window.request_layout(style, [], cx), text_style);
        }

        // A text that wraps has to be measured: how many lines it takes
        // depends on how wide it is given.
        let input = self.input.clone();
        let rows = self.rows as f32;
        let placeholder = self.placeholder_color.unwrap_or(text_style.color);
        let measure_style = text_style.clone();
        let layout = window.request_measured_layout(style, move |known, available, window, cx| {
            let width = known.width.or(match available.width {
                AvailableSpace::Definite(width) => Some(width),
                _ => None,
            });
            let (lines, _) = shape(input.read(cx), &measure_style, placeholder, width, window);
            let height = lines
                .iter()
                .map(|line| line.size(line_height).height)
                .fold(px(0.), |total, height| total + height);
            size(width.unwrap_or(px(0.)), height.max(line_height * rows))
        });
        (layout, text_style)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        text_style: &mut TextStyle,
        window: &mut Window,
        cx: &mut App,
    ) -> BodyLayout {
        let line_height = editor_line_height(window);
        let placeholder_color = self
            .placeholder_color
            .unwrap_or_else(|| crate::foundation::fade(text_style.color, 0.5));
        let input = self.input.read(cx);
        let wraps = input.multiline;
        let (lines, empty) = shape(
            input,
            text_style,
            placeholder_color,
            wraps.then_some(bounds.size.width),
            window,
        );

        let display_cursor = if empty {
            0
        } else {
            input.display_offset(input.cursor_offset())
        };
        let selected = if empty {
            0..0
        } else {
            input.display_offset(input.selected_range.start)
                ..input.display_offset(input.selected_range.end)
        };

        // Walk the laid-out lines for the caret and the selection, relative
        // to the text's origin.
        let mut caret = None;
        let mut selection = Vec::new();
        let mut y = px(0.);
        let mut start = 0;
        let row_width = bounds.size.width;
        for line in &lines {
            let end = start + line.len();
            let height = line.size(line_height).height;
            if selected.is_empty() {
                if (start..=end).contains(&display_cursor)
                    && caret.is_none()
                    && let Some(at) = line.position_for_index(display_cursor - start, line_height)
                {
                    caret = Some(point(at.x, y + at.y));
                }
            } else if selected.start <= end && selected.end >= start {
                let from = selected.start.max(start) - start;
                let to = selected.end.min(end) - start;
                if let (Some(a), Some(b)) = (
                    line.position_for_index(from, line_height),
                    line.position_for_index(to, line_height),
                ) {
                    // A selection that runs to the end of a line takes in
                    // the newline, as a text view shows it.
                    let tail = if selected.end > end && wraps {
                        row_width
                    } else {
                        b.x
                    };
                    if a.y == b.y {
                        selection.push(Bounds::from_corners(
                            point(a.x, y + a.y),
                            point(tail.max(a.x), y + a.y + line_height),
                        ));
                    } else {
                        selection.push(Bounds::from_corners(
                            point(a.x, y + a.y),
                            point(row_width, y + a.y + line_height),
                        ));
                        if b.y > a.y + line_height {
                            selection.push(Bounds::from_corners(
                                point(px(0.), y + a.y + line_height),
                                point(row_width, y + b.y),
                            ));
                        }
                        selection.push(Bounds::from_corners(
                            point(px(0.), y + b.y),
                            point(tail, y + b.y + line_height),
                        ));
                    }
                }
            }
            y += height;
            start = end + 1;
        }
        let caret = if selected.is_empty() {
            Some(caret.unwrap_or_default())
        } else {
            None
        };

        // A single line longer than its room scrolls to keep the caret in
        // view; one shorter than its room may be centred in it.
        let mut origin = bounds.origin;
        let mut scroll = input.scroll_x;
        if !wraps {
            let width = lines.first().map(|line| line.width()).unwrap_or_default();
            if width <= row_width {
                scroll = px(0.);
                if self.centered {
                    origin.x += (row_width - width) / 2.;
                }
            } else {
                let focus_x = caret.map(|caret| caret.x).unwrap_or_else(|| {
                    lines
                        .first()
                        .and_then(|line| {
                            line.position_for_index(
                                input.display_offset(input.cursor_offset()),
                                line_height,
                            )
                        })
                        .map(|at| at.x)
                        .unwrap_or_default()
                });
                if focus_x - scroll > row_width - px(1.) {
                    scroll = focus_x - row_width + px(1.);
                }
                if focus_x < scroll {
                    scroll = focus_x;
                }
                scroll = scroll.min(width - row_width + px(1.)).max(px(0.));
                origin.x -= scroll;
            }
        }
        let caret_color = self.caret_color.unwrap_or(text_style.color);
        let selection_color = self
            .selection_color
            .unwrap_or_else(|| crate::foundation::fade(text_style.color, 0.2));

        self.input.update(cx, |input, _| input.scroll_x = scroll);

        BodyLayout {
            lines,
            line_height,
            origin,
            caret: caret.map(|at| Bounds::new(origin + at, size(px(1.), line_height))),
            selection: selection
                .into_iter()
                .map(|rect| Bounds::new(origin + rect.origin, rect.size))
                .collect(),
            caret_color,
            selection_color,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut TextStyle,
        layout: &mut BodyLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (focus_handle, disabled) = {
            let input = self.input.read(cx);
            (input.focus_handle.clone(), input.disabled)
        };
        let focused = !disabled && focus_handle.is_focused(window);
        if !disabled {
            window.handle_input(
                &focus_handle,
                ElementInputHandler::new(bounds, self.input.clone()),
                cx,
            );
        }

        // A selection is drawn while the field holds the focus, as the
        // browser draws one only in the focused field.
        let clip = Bounds::new(
            bounds.origin - point(px(0.), px(0.)),
            size(bounds.size.width + px(1.), bounds.size.height),
        );
        window.with_content_mask(Some(ContentMask { bounds: clip }), |window| {
            if focused {
                for rect in layout.selection.drain(..) {
                    window.paint_quad(fill(rect, layout.selection_color));
                }
            }
            let mut origin = layout.origin;
            for line in &layout.lines {
                line.paint(
                    origin,
                    layout.line_height,
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                )
                .ok();
                origin.y += line.size(layout.line_height).height;
            }
            if focused && let Some(caret) = layout.caret {
                window.paint_quad(fill(caret, layout.caret_color));
            }
        });

        // A drag that leaves the field keeps selecting until the button
        // comes up, wherever it comes up.
        if !disabled {
            let input = self.input.downgrade();
            window.on_mouse_event(move |event: &MouseMoveEvent, phase, _, cx| {
                if phase == DispatchPhase::Bubble {
                    input
                        .update(cx, |input, cx| input.mouse_move(event, cx))
                        .ok();
                }
            });
            let input = self.input.downgrade();
            window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                if phase == DispatchPhase::Bubble {
                    input.update(cx, |input, cx| input.mouse_up(event, cx)).ok();
                }
            });
        }

        let lines = std::mem::take(&mut layout.lines);
        let line_height = layout.line_height;
        let origin = layout.origin;
        self.input.update(cx, |input, _| {
            input.lines = lines;
            input.line_height = line_height;
            input.text_origin = origin;
            input.bounds = Some(bounds);
        });
    }
}
