use std::ops::Range;

use gpui::{
    App, Bounds, ClipboardItem, Context, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, Focusable, GlobalElementId, Hsla, InspectorElementId, InteractiveElement,
    IntoElement, KeyDownEvent, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ParentElement, Pixels, Point, RenderOnce, SharedString, Style, Styled, TextRun, UTF16Selection,
    UnderlineStyle, Window, WrappedLine, div, fill, point, px, relative, size,
};

use crate::assets::icons;
use crate::components::{Icon, Menu, MenuItem};
use crate::prelude::*;
use crate::{
    ControlPart, ControlSize, ControlState, ControlVariant, SharedValueListener, Tint, TypeStyle,
    ValueListener,
};

/// The three text-control sizes. A text control is one of the sized controls,
/// so the geometry comes off the same profile a button reads — but only three
/// of the four steps, which is the set React exposes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextFieldSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TextFieldSize {
    pub(crate) fn control(self) -> ControlSize {
        match self {
            TextFieldSize::Small => ControlSize::Small,
            TextFieldSize::Medium => ControlSize::Medium,
            TextFieldSize::Large => ControlSize::Large,
        }
    }
}

/// Whether the control is reporting a problem.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextFieldState {
    #[default]
    Default,
    Error,
}

impl TextFieldState {
    /// An erroring control is re-tinted rather than given a second palette,
    /// so its border, wash and focus ring all move together.
    pub fn tint(self, tint: Tint) -> Tint {
        match self {
            TextFieldState::Error => Tint::Danger,
            TextFieldState::Default => tint,
        }
    }
}

/// The editable buffer behind a text control.
///
/// The web hands a component the DOM's own editing — a caret, a selection, an
/// input method — and GPUI hands it nothing: text editing is a view's job
/// there, so this is the piece with no counterpart in `packages/ui_react`. It
/// owns the string, the selection and the focus; [`TextField`] draws the box
/// around it — single-line or multiline — and a [`SearchField`] wears the
/// same buffer.
///
/// [`SearchField`]: crate::SearchField
pub struct TextBuffer {
    focus_handle: gpui::FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    multi_line: bool,
    lines: Vec<WrappedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    last_line_height: Pixels,
    is_selecting: bool,
}

impl TextBuffer {
    pub fn new(cx: &mut Context<Self>, placeholder: impl Into<SharedString>) -> Self {
        TextBuffer {
            focus_handle: cx.focus_handle(),
            content: SharedString::default(),
            placeholder: placeholder.into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            multi_line: false,
            lines: Vec::new(),
            last_bounds: None,
            last_line_height: px(0.),
            is_selecting: false,
        }
    }

    /// A field that starts with something in it.
    pub fn with_value(mut self, value: impl Into<SharedString>) -> Self {
        self.content = value.into();
        self.selected_range = self.content.len()..self.content.len();
        self
    }

    /// A field whose Return inserts a line rather than doing nothing.
    pub fn multi_line(mut self, multi_line: bool) -> Self {
        self.multi_line = multi_line;
        self
    }

    pub fn value(&self) -> &SharedString {
        &self.content
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn placeholder(&self) -> &SharedString {
        &self.placeholder
    }

    pub fn set_value(&mut self, value: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.content = value.into();
        let end = self.content.len();
        self.selected_range = end..end;
        self.marked_range = None;
        cx.notify();
    }

    /// Give this field the keyboard.
    pub fn focus(&self, window: &mut Window) {
        window.focus(&self.focus_handle);
    }

    /// Does this field have the keyboard?
    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    /// The keys a text control answers to.
    ///
    /// Typed characters never come through here — the platform sends those to
    /// the input handler the element installs, which is what keeps input
    /// methods working. This is only the editing keys, matched on the
    /// keystroke rather than on actions, so a host does not have to register
    /// a keymap to get a working field.
    fn on_key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let keystroke = &event.keystroke;
        let command = keystroke.modifiers.platform;
        let shift = keystroke.modifiers.shift;

        match keystroke.key.as_str() {
            "backspace" => {
                if self.selected_range.is_empty() {
                    self.select_to(self.previous_boundary(self.cursor_offset()), cx);
                }
                self.replace_text_in_range(None, "", window, cx);
            }
            "delete" => {
                if self.selected_range.is_empty() {
                    self.select_to(self.next_boundary(self.cursor_offset()), cx);
                }
                self.replace_text_in_range(None, "", window, cx);
            }
            "left" if shift => self.select_to(self.previous_boundary(self.cursor_offset()), cx),
            "left" => {
                let to = if self.selected_range.is_empty() {
                    self.previous_boundary(self.cursor_offset())
                } else {
                    self.selected_range.start
                };
                self.move_to(to, cx);
            }
            "right" if shift => self.select_to(self.next_boundary(self.cursor_offset()), cx),
            "right" => {
                let to = if self.selected_range.is_empty() {
                    self.next_boundary(self.cursor_offset())
                } else {
                    self.selected_range.end
                };
                self.move_to(to, cx);
            }
            "home" => self.move_to(0, cx),
            "end" => self.move_to(self.content.len(), cx),
            "enter" if self.multi_line => self.replace_text_in_range(None, "\n", window, cx),
            "a" if command => {
                self.move_to(0, cx);
                self.select_to(self.content.len(), cx);
            }
            "c" if command => self.copy(cx),
            "x" if command => {
                self.copy(cx);
                self.replace_text_in_range(None, "", window, cx);
            }
            "v" if command => {
                if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                    let text = if self.multi_line {
                        text
                    } else {
                        text.replace('\n', " ")
                    };
                    self.replace_text_in_range(None, &text, window, cx);
                }
            }
            _ => {}
        }
    }

    fn copy(&self, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle);
        self.is_selecting = true;
        let offset = self.index_for_position(event.position);
        if event.modifiers.shift {
            self.select_to(offset, cx);
        } else {
            self.move_to(offset, cx);
        }
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if self.is_selecting {
            let offset = self.index_for_position(event.position);
            self.select_to(offset, cx);
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        cx.notify();
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
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

    /// The byte offset under a point, walking the wrapped lines the element
    /// last laid out.
    fn index_for_position(&self, position: Point<Pixels>) -> usize {
        let Some(bounds) = self.last_bounds else {
            return 0;
        };
        let line_height = self.last_line_height;
        let mut origin = bounds.origin;
        let mut offset = 0;

        for line in &self.lines {
            let height = line.size(line_height).height;
            let local = point(position.x - origin.x, position.y - origin.y);
            if position.y < origin.y + height || std::ptr::eq(line, self.lines.last().unwrap()) {
                return offset
                    + line
                        .closest_index_for_position(local, line_height)
                        .unwrap_or(0);
            }
            origin.y += height;
            // The newline the shaper split on is not part of either line.
            offset += line.len() + 1;
        }
        self.content.len()
    }

    /// The next character boundary. Byte offsets rather than graphemes: a
    /// cluster stays whole when it is one code point, which is what every
    /// string in this system is.
    fn previous_boundary(&self, offset: usize) -> usize {
        self.content[..offset]
            .char_indices()
            .next_back()
            .map(|(index, _)| index)
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content[offset..]
            .char_indices()
            .nth(1)
            .map(|(index, _)| offset + index)
            .unwrap_or(self.content.len())
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }
}

impl Focusable for TextBuffer {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for TextBuffer {
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
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range| self.range_from_utf16(range))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        let cursor = range.start + new_text.len();
        self.selected_range = cursor..cursor;
        self.marked_range.take();
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range| self.range_from_utf16(range))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        self.marked_range =
            (!new_text.is_empty()).then(|| range.start..range.start + new_text.len());
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| {
                let cursor = range.start + new_text.len();
                cursor..cursor
            });
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let line = self.lines.first()?;
        let range = self.range_from_utf16(&range_utf16);
        let line_height = self.last_line_height;
        let start = line.position_for_index(range.start, line_height)?;
        let end = line.position_for_index(range.end, line_height)?;
        Some(Bounds::from_corners(
            bounds.origin + start,
            point(
                bounds.origin.x + end.x,
                bounds.origin.y + end.y + line_height,
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.offset_to_utf16(self.index_for_position(point)))
    }
}

/// The text, the caret and the selection — the part of a text control GPUI
/// has no element for.
struct TextBody {
    field: Entity<TextBuffer>,
    face: TypeStyle,
    line_height: Pixels,
    text: Hsla,
    placeholder: Hsla,
    caret: Hsla,
    selection: Hsla,
    wrap: bool,
}

struct TextBodyLayout {
    lines: Vec<WrappedLine>,
    caret: Option<Bounds<Pixels>>,
    selection: Vec<Bounds<Pixels>>,
}

impl IntoElement for TextBody {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl TextBody {
    /// Shape the content — or the placeholder, when there is none — into the
    /// lines this element paints.
    fn shape(
        &self,
        wrap_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> (Vec<WrappedLine>, bool) {
        let field = self.field.read(cx);
        let empty = field.content.is_empty();
        let text = if empty {
            field.placeholder.clone()
        } else {
            field.content.clone()
        };
        let color = if empty { self.placeholder } else { self.text };

        let run = TextRun {
            len: text.len(),
            font: gpui::Font {
                family: self.face.font_family().into(),
                features: Default::default(),
                fallbacks: None,
                weight: self.face.weight,
                style: Default::default(),
            },
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        // A marked range is text an input method is still composing; it is
        // underlined until it is committed.
        let runs = match field.marked_range.as_ref().filter(|_| !empty) {
            Some(marked) => vec![
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

        let lines = window
            .text_system()
            .shape_text(text, self.face.size, &runs, wrap_width, None)
            .map(|lines| lines.to_vec())
            .unwrap_or_default();
        (lines, empty)
    }
}

impl Element for TextBody {
    type RequestLayoutState = ();
    type PrepaintState = TextBodyLayout;

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
    ) -> (LayoutId, ()) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();

        if !self.wrap {
            style.size.height = self.line_height.into();
            return (window.request_layout(style, [], cx), ());
        }

        // A box that grows with what is typed into it has to be measured,
        // because how many lines the text takes depends on how wide it is.
        let line_height = self.line_height;
        let body = TextBody {
            field: self.field.clone(),
            ..*self
        };
        let layout_id =
            window.request_measured_layout(style, move |known, available, window, cx| {
                let width = known.width.or(match available.width {
                    gpui::AvailableSpace::Definite(width) => Some(width),
                    _ => None,
                });
                let (lines, _) = body.shape(width, window, cx);
                let height = lines
                    .iter()
                    .map(|line| line.size(line_height).height)
                    .fold(px(0.), |total, height| total + height);
                size(width.unwrap_or(px(0.)), height.max(line_height))
            });
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> TextBodyLayout {
        let (lines, empty) = self.shape(self.wrap.then_some(bounds.size.width), window, cx);
        let field = self.field.read(cx);
        let selected = field.selected_range.clone();
        let cursor = field.cursor_offset();
        let line_height = self.line_height;

        let mut caret = None;
        let mut selection = Vec::new();

        if !empty {
            let mut origin = bounds.origin;
            let mut offset = 0;

            for line in &lines {
                let height = line.size(line_height).height;
                let end = offset + line.len();

                if selected.is_empty() {
                    if (offset..=end).contains(&cursor)
                        && let Some(position) =
                            line.position_for_index(cursor - offset, line_height)
                    {
                        caret = Some(Bounds::new(origin + position, size(px(2.), line_height)));
                    }
                } else if selected.start <= end && selected.end >= offset {
                    let from = selected.start.max(offset) - offset;
                    let to = selected.end.min(end) - offset;
                    if let (Some(start), Some(finish)) = (
                        line.position_for_index(from, line_height),
                        line.position_for_index(to, line_height),
                    ) {
                        selection.push(Bounds::from_corners(
                            origin + start,
                            origin + finish + point(px(0.), line_height),
                        ));
                    }
                }

                origin.y += height;
                offset = end + 1;
            }
        } else if let Some(position) = lines
            .first()
            .and_then(|line| line.position_for_index(0, line_height))
        {
            caret = Some(Bounds::new(
                bounds.origin + position,
                size(px(2.), line_height),
            ));
        }

        TextBodyLayout {
            lines,
            caret,
            selection,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        layout: &mut TextBodyLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.field.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.field.clone()),
            cx,
        );

        for quad in layout.selection.drain(..) {
            window.paint_quad(fill(quad, self.selection));
        }

        let mut origin = bounds.origin;
        for line in &layout.lines {
            line.paint(
                origin,
                self.line_height,
                gpui::TextAlign::Left,
                None,
                window,
                cx,
            )
            .ok();
            origin.y += line.size(self.line_height).height;
        }

        if focus_handle.is_focused(window)
            && let Some(caret) = layout.caret
        {
            window.paint_quad(fill(caret, self.caret));
        }

        let lines = std::mem::take(&mut layout.lines);
        let line_height = self.line_height;
        self.field.update(cx, |field, _| {
            field.lines = lines;
            field.last_bounds = Some(bounds);
            field.last_line_height = line_height;
        });
    }
}

/// A text control.
///
/// The box is a recessed card — one step below the paper — that lifts to the
/// paper on focus behind an accent border and a soft accent wash. That
/// card→paper transition is the focus signal; a field that stays white can
/// only signal with its ring.
///
/// The error state is not a second palette bolted on: the control is
/// re-tinted to `danger`, so the border, the wash, the value's ink and the
/// focus wash all move together.
///
/// `multiline` is the axis rather than a control of its own, because the box,
/// the states and the tinting are one design either way. What it changes: the
/// box takes the container corner rather than the control corner — a theme
/// that makes controls pills would otherwise turn a growing box into a
/// lozenge — the leading opens up, because this is the one text control whose
/// text wraps, and the height becomes a floor rather than the control height.
#[derive(IntoElement)]
pub struct TextField {
    field: Entity<TextBuffer>,
    size: TextFieldSize,
    state: TextFieldState,
    tint: Tint,
    /// Set the mono face — for a key, an ID, or a URL.
    mono: bool,
    disabled: bool,
    /// Draw as a box that grows rather than a single line.
    multiline: bool,
}

impl TextField {
    pub fn new(field: &Entity<TextBuffer>) -> Self {
        TextField {
            field: field.clone(),
            size: TextFieldSize::Medium,
            state: TextFieldState::Default,
            tint: Tint::Primary,
            mono: false,
            disabled: false,
            multiline: false,
        }
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: TextFieldState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for TextField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        text_control(
            self.field,
            self.size,
            self.state,
            self.tint,
            self.mono,
            self.disabled,
            self.multiline,
            window,
            cx,
        )
    }
}

/// The editable region: the text, the caret and the keys that move them.
///
/// Both the box below and [`SearchField`](crate::SearchField) wrap this, which
/// is what keeps a search field's editing the same as an input's rather than a
/// second implementation of it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn editable(
    field: &Entity<TextBuffer>,
    face: TypeStyle,
    line_height: Pixels,
    text: Hsla,
    placeholder: Hsla,
    caret: Hsla,
    selection: Hsla,
    wrap: bool,
    cx: &App,
) -> impl IntoElement {
    div()
        .flex_1()
        .min_w_0()
        // The keyboard is routed to whichever element holds the focus, and
        // only to that element and its ancestors — so the keys have to be
        // listened for here rather than on the box drawn around this.
        .track_focus(&field.read(cx).focus_handle)
        .child(TextBody {
            field: field.clone(),
            face,
            line_height,
            text,
            placeholder,
            caret,
            selection,
            wrap,
        })
        .on_mouse_down(MouseButton::Left, {
            let field = field.clone();
            move |event, window, cx| {
                field.update(cx, |field, cx| field.on_mouse_down(event, window, cx));
            }
        })
        .on_mouse_move({
            let field = field.clone();
            move |event, _, cx| {
                field.update(cx, |field, cx| field.on_mouse_move(event, cx));
            }
        })
        .on_mouse_up(MouseButton::Left, {
            let field = field.clone();
            move |event, _, cx| {
                field.update(cx, |field, cx| field.on_mouse_up(event, cx));
            }
        })
        .on_key_down({
            let field = field.clone();
            move |event, window, cx| {
                field.update(cx, |field, cx| field.on_key_down(event, window, cx));
            }
        })
}

/// The box the three text controls share, and share deliberately: a `Select`
/// is a `TextField` with a chevron and a `FieldValue` is one that was never
/// editable, so the three drifting apart would be the bug rather than the
/// feature.
#[allow(clippy::too_many_arguments)]
fn text_control(
    field: Entity<TextBuffer>,
    size: TextFieldSize,
    state: TextFieldState,
    tint: Tint,
    mono: bool,
    disabled: bool,
    wrap: bool,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let vars = cx.theme().vars;
    let metrics = vars.metrics(size.control());
    let tint = state.tint(tint);
    let erroring = state == TextFieldState::Error;
    let focused = field.read(cx).focus_handle.is_focused(window);
    let accent = vars.ramp(tint).shade(600);

    let disabled_surface = vars.control_color(
        ControlVariant::Normal,
        ControlPart::Surface,
        tint,
        ControlState::Disabled,
    );
    let disabled_content = vars.control_color(
        ControlVariant::Normal,
        ControlPart::Content,
        tint,
        ControlState::Disabled,
    );

    // Entered text is body weight, not label weight — a search query typed in
    // semibold reads as shouting.
    let face = TypeStyle {
        family: if mono {
            vars.body_small.family
        } else {
            metrics.content.family
        },
        size: metrics.content.size,
        weight: vars.body_small.weight,
        line_height: metrics.content.line_height,
    };
    let face = if mono {
        // The mono face is the one place a component reaches a `base.*`
        // primitive, which `text-field.css` does too: `base.font.code` has no
        // semantic token in front of it.
        TypeStyle {
            family: &["ui-monospace"],
            ..face
        }
    } else {
        face
    };

    let text = if disabled {
        disabled_content
    } else if erroring {
        vars.ramp(tint).shade(800)
    } else {
        vars.color_content
    };
    let surface = if disabled {
        disabled_surface
    } else if erroring {
        vars.ramp(tint).shade(50)
    } else if focused {
        vars.color_surface
    } else {
        vars.color_surface_muted
    };
    let border = if focused || erroring {
        accent
    } else {
        vars.color_border_strong
    };

    div()
        .w_full()
        .flex()
        .items_center()
        // Clicking the box's padding is clicking the field.
        .on_mouse_down(MouseButton::Left, {
            let field = field.clone();
            move |_, window, cx| field.read(cx).focus(window)
        })
        .map(|el| {
            if wrap {
                el.min_h(vars.spacing_20)
                    .py(vars.spacing_2_5)
                    .rounded(vars.radius_large)
            } else {
                el.h(metrics.size)
                    .py(metrics.padding_block)
                    .rounded(vars.radius_medium)
            }
        })
        .px(vars.spacing_3)
        .border(crate::hairline(vars))
        .bg(surface)
        .border_color(border)
        .text_color(text)
        // Focus lifts the card to the paper behind a soft accent wash.
        .when(focused && !disabled, |el| {
            el.shadow(vec![gpui::BoxShadow {
                color: accent.opacity(vars.focus_glow_alpha),
                offset: point(px(0.), px(0.)),
                blur_radius: px(0.),
                spread_radius: vars.focus_width,
            }])
        })
        .map(|el| {
            if disabled {
                el.cursor_not_allowed()
            } else {
                el.cursor_text()
            }
        })
        .child(editable(
            &field,
            face,
            if wrap {
                // Real leading: a paragraph's lines need air between them,
                // where a control's label is set solid.
                vars.body_small.line_height
            } else {
                metrics.content.line_height
            },
            text,
            vars.color_content_faint,
            accent,
            accent.opacity(vars.focus_glow_alpha),
            wrap,
            cx,
        ))
}

/// A value in the shape of an input, for a row that reports rather than
/// edits.
///
/// Not a disabled input: nothing here was ever editable, so nothing is greyed
/// out to say it stopped being.
#[derive(IntoElement)]
pub struct FieldValue {
    value: SharedString,
    size: TextFieldSize,
    mono: bool,
}

impl FieldValue {
    pub fn new(value: impl Into<SharedString>) -> Self {
        FieldValue {
            value: value.into(),
            size: TextFieldSize::Medium,
            mono: false,
        }
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }
}

impl RenderOnce for FieldValue {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let metrics = vars.metrics(self.size.control());

        div()
            .w_full()
            .flex()
            .items_center()
            .h(metrics.size)
            .px(vars.spacing_3)
            .py(metrics.padding_block)
            .rounded(vars.radius_medium)
            .border(crate::hairline(vars))
            .bg(vars.color_surface_muted)
            .border_color(vars.color_border_strong)
            .text_color(vars.color_content)
            .type_style(TypeStyle {
                family: if self.mono {
                    &["ui-monospace"]
                } else {
                    metrics.content.family
                },
                weight: vars.body_small.weight,
                ..metrics.content
            })
            .child(self.value)
    }
}

/// A value picked from a list.
///
/// The web has a native select; GPUI has none, so this one wears the text
/// control's box and opens the system's own [`Menu`] under it — which is what
/// the native control does too.
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    value: SharedString,
    options: Vec<(SharedString, SharedString)>,
    size: TextFieldSize,
    state: TextFieldState,
    tint: Tint,
    disabled: bool,
    on_select: Option<ValueListener<SharedString>>,
}

impl Select {
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        Select {
            id: id.into(),
            value: value.into(),
            options: Vec::new(),
            size: TextFieldSize::Medium,
            state: TextFieldState::Default,
            tint: Tint::Primary,
            disabled: false,
            on_select: None,
        }
    }

    pub fn option(
        mut self,
        value: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        self.options.push((value.into(), label.into()));
        self
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: TextFieldState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Select {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let metrics = vars.metrics(self.size.control());
        let tint = self.state.tint(self.tint);
        let erroring = self.state == TextFieldState::Error;
        let on_select: Option<SharedValueListener<SharedString>> =
            self.on_select.map(std::rc::Rc::from);
        let label = self
            .options
            .iter()
            .find(|(value, _)| value == &self.value)
            .map(|(_, label)| label.clone())
            .unwrap_or_else(|| self.value.clone());

        let mut menu = Menu::new(self.id).tint(Tint::Neutral);
        for (value, option) in self.options {
            let chosen = value == self.value;
            let on_select = on_select.clone();
            menu = menu.item(
                MenuItem::new(option)
                    .checked(chosen)
                    .on_select(move |window, cx| {
                        if let Some(listener) = on_select.as_ref() {
                            listener(&value, window, cx)
                        }
                    }),
            );
        }

        menu.disabled(self.disabled).trigger(move |_, _| {
            div()
                .w_full()
                .flex()
                .items_center()
                .justify_between()
                .h(metrics.size)
                .px(vars.spacing_3)
                .py(metrics.padding_block)
                .rounded(vars.radius_medium)
                .border(vars.stroke_hairline)
                .bg(if erroring {
                    vars.ramp(tint).shade(50)
                } else {
                    vars.color_surface_muted
                })
                .border_color(if erroring {
                    vars.ramp(tint).shade(600)
                } else {
                    vars.color_border_strong
                })
                .text_color(if erroring {
                    vars.ramp(tint).shade(800)
                } else {
                    vars.color_content
                })
                .type_style(TypeStyle {
                    weight: vars.body_small.weight,
                    ..metrics.content
                })
                .child(label.clone())
                // The native arrow ignores padding, so the control draws its
                // own.
                .child(
                    Icon::new(icons::CHEVRON_DOWN_12_REGULAR)
                        .size(vars.spacing_3)
                        .color(vars.color_content_subtle),
                )
                .into_any_element()
        })
    }
}
