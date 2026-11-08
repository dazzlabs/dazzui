//! A choice out of a list too long to scroll, and the text field that
//! suggests as you type — React's `combobox.tsx`.
//!
//! Both are the text controls' box ([`FieldBox`]) with an editable text
//! ([`TextInput`]) in it, and the shared list sheet (popup.rs) below it. The
//! keyboard stays in the field the whole time: the arrow keys, Return and
//! Escape are taken from the text before it sees them, and every other key
//! edits the query, which narrows the list. Base UI does this on the web;
//! the rows are never focused, only highlighted.

use std::rc::Rc;

use gpui::{
    AnyElement, App, Entity, InteractiveElement, IntoElement, KeyDownEvent, MouseButton,
    ParentElement, RenderOnce, SharedString, Stateful, StatefulInteractiveElement, Styled, Window,
    div, prelude::*, px,
};

use crate::StyledExt;
use crate::components::icon::{Icon, icon_slot};
use crate::components::popup::{
    Align, Anchor, Placement, PopupRow, PopupState, Side, group_label, measured, part_id,
    positioned, sheet,
};
use crate::components::select::{SelectEntry, SelectOption};
use crate::components::text_field::{FieldBox, FieldState, placeholder_color, selection_color};
use crate::foundation::text_input::{self, InputState, TextInput, input_state};
use crate::foundation::{Size, Tint};
use crate::generated::ThemeTokens;
use crate::glyphs;
use crate::theme::ActiveTheme;

/// One choice of a combobox: a value, the label shown and matched against,
/// and whether it can be picked. The same shape a `Select` takes.
pub type ComboboxOption = SelectOption;

/// A choice, or choices under a heading — React's `ComboboxOptionGroup`.
pub type ComboboxEntry = SelectEntry;

type OnValueChange = Rc<dyn Fn(Option<SharedString>, &mut Window, &mut App)>;
type OnText = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;
type Choose = Rc<dyn Fn(usize, &mut Window, &mut App)>;
type Callback = Rc<dyn Fn(&mut Window, &mut App)>;

/// Whether an option's label matches what was typed: Base UI's default
/// filter, a case-insensitive "contains".
fn matches(label: &str, query: &str) -> bool {
    query.is_empty() || label.to_lowercase().contains(&query.to_lowercase())
}

/// The entries that survive `query`, in order; a group with nothing left in
/// it goes with its heading.
fn filtered(entries: &[SelectEntry], query: &str) -> Vec<SelectEntry> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            SelectEntry::Option(option) => {
                matches(&option.label, query).then(|| SelectEntry::Option(option.clone()))
            }
            SelectEntry::Group { label, options } => {
                let options: Vec<SelectOption> = options
                    .iter()
                    .filter(|option| matches(&option.label, query))
                    .cloned()
                    .collect();
                (!options.is_empty()).then(|| SelectEntry::group(label.clone(), options))
            }
        })
        .collect()
}

fn flatten(entries: &[SelectEntry]) -> Vec<SelectOption> {
    entries
        .iter()
        .flat_map(|entry| match entry {
            SelectEntry::Option(option) => vec![option.clone()],
            SelectEntry::Group { options, .. } => options.clone(),
        })
        .collect()
}

/// What the list and the keys it answers are handed.
struct List {
    state: Entity<PopupState>,
    /// What is visible, as it is drawn.
    entries: Vec<SelectEntry>,
    /// The chosen row, for the check a combobox draws; `None` draws no check
    /// column at all, the autocomplete's rows.
    checked: Option<Option<usize>>,
    tint: Tint,
    choose: Choose,
    close: Callback,
}

impl List {
    /// The sheet: the surviving rows, or the line that says none did.
    fn sheet(self, anchor: &Anchor, empty: AnyElement, t: &ThemeTokens, cx: &App) -> AnyElement {
        let highlighted = self.state.read(cx).highlighted;
        let mut index = 0;
        let mut children: Vec<AnyElement> = Vec::new();
        let mut previous_group = false;
        let row = |option: &SelectOption, index: &mut usize| {
            let at = *index;
            *index += 1;
            let hover = self.state.clone();
            let choose = self.choose.clone();
            let mut row = PopupRow::new(("option", at), option.label.clone())
                .tint(self.tint)
                .highlighted(highlighted == Some(at))
                .disabled(option.disabled)
                .on_hover(move |hovered, _, cx| {
                    hover.update(cx, |state, cx| {
                        if hovered {
                            state.highlighted = Some(at);
                        } else if state.highlighted == Some(at) {
                            state.highlighted = None;
                        }
                        cx.notify();
                    })
                })
                .on_press(move |window, cx| choose(at, window, cx));
            if let Some(checked) = self.checked {
                row = row.check(checked == Some(at));
            }
            row.into_any_element()
        };
        for entry in &self.entries {
            match entry {
                SelectEntry::Option(option) => {
                    children.push(row(option, &mut index));
                    previous_group = false;
                }
                SelectEntry::Group { label, options } => {
                    let gap = if previous_group { t.spacing_1 } else { 0. };
                    let rows: Vec<AnyElement> = options
                        .iter()
                        .map(|option| row(option, &mut index))
                        .collect();
                    children.push(
                        div()
                            .flex()
                            .flex_col()
                            .mt(px(gap))
                            .child(group_label(t, label.clone()))
                            .children(rows)
                            .into_any_element(),
                    );
                    previous_group = true;
                }
            }
        }
        if children.is_empty() {
            // `.dz-popup__empty`: prose on the row's inset, in the faint ink.
            children.push(
                div()
                    .py(px(t.menu_item_padding))
                    .px(px(t.spacing_2))
                    .text_color(t.color_content_faint.hsla())
                    .type_style(t.body_small)
                    .child(empty)
                    .into_any_element(),
            );
        }

        // `min-width: var(--anchor-width)`: never narrower than the field.
        let width = anchor
            .bounds()
            .map(|bounds| f32::from(bounds.size.width))
            .unwrap_or(0.);
        let keep = anchor.clone();
        let close = self.close.clone();
        let list = sheet(t)
            .id("list")
            .min_w(px(width.max(t.menu_min_width)))
            .occlude()
            .on_mouse_down_out(move |event, window, cx| {
                if !keep.contains(event.position) {
                    close(window, cx);
                }
            })
            .children(children);
        positioned(anchor, Placement::side(Side::Bottom, Align::Start, t), list).into_any_element()
    }
}

/// Takes the arrow keys, Return and Escape from the text in `field` while
/// the list is theirs to answer. `open` opens the list on an arrow key;
/// `escape_closed` is what Escape does once the list is already shut.
fn keys<E: InteractiveElement>(
    field: E,
    state: &Entity<PopupState>,
    rows: Rc<Vec<SelectOption>>,
    open: Rc<dyn Fn(&'static str, &mut Window, &mut App)>,
    choose: Choose,
    close: Callback,
    escape_closed: Option<Callback>,
) -> E {
    let enabled: Rc<Vec<bool>> = Rc::new(rows.iter().map(|option| !option.disabled).collect());
    let arrow = |key: &'static str| {
        let state = state.clone();
        let enabled = enabled.clone();
        let open = open.clone();
        move |window: &mut Window, cx: &mut App| {
            if state.read(cx).open {
                state.update(cx, |state, cx| {
                    state.step(key, &enabled);
                    cx.notify();
                });
            } else {
                open(key, window, cx);
            }
        }
    };
    let up = arrow("up");
    let down = arrow("down");
    let enter = state.clone();
    let escape = state.clone();
    field
        .capture_action(move |_: &text_input::Up, window, cx| {
            cx.stop_propagation();
            up(window, cx);
        })
        .capture_action(move |_: &text_input::Down, window, cx| {
            cx.stop_propagation();
            down(window, cx);
        })
        .capture_action(move |_: &text_input::Enter, window, cx| {
            let state = enter.read(cx);
            if state.open
                && let Some(index) = state.highlighted
            {
                cx.stop_propagation();
                choose(index, window, cx);
            }
            // Otherwise Return is the text's, and through it the form's.
        })
        .capture_key_down(move |event: &KeyDownEvent, window, cx| {
            if event.keystroke.key != "escape" {
                return;
            }
            if escape.read(cx).open {
                cx.stop_propagation();
                close(window, cx);
            } else if let Some(clear) = &escape_closed {
                cx.stop_propagation();
                clear(window, cx);
            }
        })
}

/// A press in the box's padding is a press in the text — unless a control
/// in the box took it first.
fn press_through<E: InteractiveElement>(field: E, input: &Entity<InputState>) -> E {
    let input = input.downgrade();
    field.on_mouse_down(MouseButton::Left, move |event, window, cx| {
        if window.default_prevented() {
            return;
        }
        input
            .update(cx, |input, cx| input.mouse_down(event, window, cx))
            .ok();
    })
}

/// `.dz-combobox__action`: the clear and the chevron — the field's own
/// furniture rather than buttons on it. No box at rest, the subtle ink, and
/// the neutral wash only under the pointer.
fn action(
    id: &'static str,
    glyph: &'static str,
    tint: Tint,
    disabled: bool,
    press: Callback,
    t: &ThemeTokens,
) -> Stateful<gpui::Div> {
    let ramp = tint.ramp(t);
    let hovered = t.control_color_normal_surface_hovered.resolve(ramp);
    let ink = t.color_content.hsla();
    div()
        .id(id)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(t.spacing_5))
        .rounded(px(t.radius_small))
        .text_color(t.color_content_subtle.hsla())
        .child(icon_slot(t.icon_small, Icon::new(glyph)))
        .map(|this| {
            if disabled {
                this.cursor_not_allowed()
            } else {
                this.cursor_pointer()
                    .hover(move |style| style.bg(hovered).text_color(ink))
                    // The press is the control's, not the text's: the caret
                    // stays where it was.
                    .on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                    .on_click(move |_, window, cx| press(window, cx))
            }
        })
}

/// What a combobox keeps across frames: the chosen value, while nobody holds
/// it for it, and whether the text is a query being typed rather than the
/// chosen option's label.
struct Kept {
    value: Option<SharedString>,
    querying: bool,
}

/// A choice out of a list too long to scroll: the reader types to narrow it,
/// and picks one of what is left. It ends in one of its options and nothing
/// else — typed text that matches nothing is dropped when the list closes.
///
/// Controlled with [`value`](Self::value) and
/// [`on_value_change`](Self::on_value_change), or left to keep its own
/// choice from [`default_value`](Self::default_value).
#[derive(IntoElement)]
pub struct Combobox {
    id: gpui::ElementId,
    options: Vec<SelectEntry>,
    value: Option<Option<SharedString>>,
    default_value: Option<SharedString>,
    on_value_change: Option<OnValueChange>,
    placeholder: SharedString,
    empty: Option<AnyElement>,
    clearable: bool,
    size: Size,
    state: FieldState,
    tint: Tint,
    disabled: bool,
    read_only: bool,
}

impl Combobox {
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Combobox {
            id: id.into(),
            options: Vec::new(),
            value: None,
            default_value: None,
            on_value_change: None,
            placeholder: "Search…".into(),
            empty: None,
            clearable: true,
            size: Size::Medium,
            state: FieldState::Default,
            tint: Tint::Primary,
            disabled: false,
            read_only: false,
        }
    }

    /// Flat, or grouped under a heading the way `<optgroup>` groups.
    pub fn options(mut self, options: impl IntoIterator<Item = impl Into<SelectEntry>>) -> Self {
        self.options.extend(options.into_iter().map(Into::into));
        self
    }

    /// Holds the choice for the caller — `None` for nothing chosen.
    pub fn value(mut self, value: Option<SharedString>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Hears the choice change: an option's value, or `None` once cleared.
    pub fn on_value_change(
        mut self,
        handler: impl Fn(Option<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    /// What the field prints while nothing is chosen.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// What the list says when the query matches nothing; "No matches" by
    /// default.
    pub fn empty(mut self, empty: impl IntoElement) -> Self {
        self.empty = Some(empty.into_any_element());
        self
    }

    /// Offers a control that clears the chosen option. On by default.
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: FieldState) -> Self {
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

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Accepted for React's shape; the constraint is the form's.
    pub fn required(self, _required: bool) -> Self {
        self
    }
}

impl RenderOnce for Combobox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let id = self.id.clone();
        let default_value = self.default_value.clone();
        let kept = window.use_keyed_state(part_id(&id, "value"), cx, |_, _| Kept {
            value: default_value,
            querying: false,
        });
        let input = input_state(part_id(&id, "input"), window, cx);
        let state = PopupState::keyed(&id, false, window, cx);
        let interactive = !self.disabled && !self.read_only;
        let open = state.read(cx).open && interactive;
        let anchor = state.read(cx).anchor.clone();

        let controlled = self.value.is_some();
        let current = match &self.value {
            Some(value) => value.clone(),
            None => kept.read(cx).value.clone(),
        };
        let querying = kept.read(cx).querying;
        let all = flatten(&self.options);
        let chosen = current
            .as_ref()
            .and_then(|value| all.iter().find(|option| &option.value == value));
        let label: SharedString = chosen
            .map(|option| option.label.clone())
            .unwrap_or_default();

        // While a query is being typed the list is narrowed by it; otherwise
        // the field shows the chosen label and the list shows everything.
        let query = if querying {
            input.read(cx).value().to_string()
        } else {
            String::new()
        };
        let entries = filtered(&self.options, &query);
        let rows = Rc::new(flatten(&entries));
        let checked = current
            .as_ref()
            .and_then(|value| rows.iter().position(|option| &option.value == value));

        // Closing forgets the query: the text goes back to the chosen label.
        let close: Callback = {
            let state = state.clone();
            let kept = kept.clone();
            Rc::new(move |window, cx| {
                state.update(cx, |state, cx| state.hide(window, cx));
                kept.update(cx, |kept, cx| {
                    kept.querying = false;
                    cx.notify();
                });
            })
        };
        let set: Rc<dyn Fn(Option<SharedString>, &mut Window, &mut App)> = {
            let kept = kept.clone();
            let notify = self.on_value_change.clone();
            Rc::new(move |value, window, cx| {
                if !controlled {
                    let value = value.clone();
                    kept.update(cx, |kept, cx| {
                        kept.value = value;
                        cx.notify();
                    });
                }
                if let Some(notify) = &notify {
                    notify(value, window, cx);
                }
            })
        };
        let choose: Choose = {
            let rows = rows.clone();
            let close = close.clone();
            let set = set.clone();
            Rc::new(move |index, window, cx| {
                let Some(option) = rows.get(index).filter(|option| !option.disabled) else {
                    return;
                };
                close(window, cx);
                set(Some(option.value.clone()), window, cx);
            })
        };
        let clear: Callback = {
            let close = close.clone();
            let set = set.clone();
            let input = input.clone();
            Rc::new(move |window, cx| {
                close(window, cx);
                input.update(cx, |input, cx| input.set_value("", cx));
                set(None, window, cx);
            })
        };
        // Opening shows the whole list with the chosen row reached.
        let show: Rc<dyn Fn(Option<&'static str>, &mut Window, &mut App)> = {
            let state = state.clone();
            let kept = kept.clone();
            let enabled: Vec<bool> = flatten(&self.options)
                .iter()
                .map(|option| !option.disabled)
                .collect();
            let selected = current
                .as_ref()
                .and_then(|value| all.iter().position(|option| &option.value == value));
            Rc::new(move |key, _, cx| {
                kept.update(cx, |kept, cx| {
                    kept.querying = false;
                    cx.notify();
                });
                state.update(cx, |state, cx| {
                    state.open = true;
                    state.highlighted = selected;
                    if selected.is_none()
                        && let Some(key) = key
                    {
                        state.step(key, &enabled);
                    }
                    cx.notify();
                });
            })
        };

        let focused = input.read(cx).is_focused(window);
        let chrome = FieldBox::new(self.size)
            .tint(self.tint)
            .state(self.state)
            .focused(focused || open)
            .disabled(self.disabled);

        let mut text = TextInput::new(part_id(&id, "text"))
            .state(input.clone())
            .placeholder(self.placeholder.clone())
            .placeholder_color(placeholder_color(t))
            .selection_color(selection_color(t))
            .read_only(self.read_only)
            .disabled(self.disabled);
        if !querying {
            text = text.value(label.clone());
        }
        {
            let state = state.clone();
            let kept = kept.clone();
            text = text.on_change(move |_, _, cx| {
                kept.update(cx, |kept, cx| {
                    kept.querying = true;
                    cx.notify();
                });
                state.update(cx, |state, cx| {
                    state.open = true;
                    state.highlighted = None;
                    cx.notify();
                });
            });
        }
        {
            let close = close.clone();
            text = text.on_blur(move |window, cx| close(window, cx));
        }

        // The trailing controls: clear while there is something to clear,
        // and the chevron, which turns while the list is open.
        let has_text = querying && !input.read(cx).value().is_empty();
        let clear_button = (self.clearable && (current.is_some() || has_text)).then(|| {
            action(
                "clear",
                glyphs::DISMISS_12_REGULAR,
                self.tint,
                !interactive,
                clear.clone(),
                t,
            )
        });
        let toggle: Callback = {
            let state = state.clone();
            let close = close.clone();
            let show = show.clone();
            let input = input.clone();
            Rc::new(move |window, cx| {
                if state.read(cx).open {
                    close(window, cx);
                } else {
                    show(None, window, cx);
                }
                input.read(cx).focus(window);
            })
        };
        let chevron = action(
            "trigger",
            if open {
                glyphs::CHEVRON_UP_12_REGULAR
            } else {
                glyphs::CHEVRON_DOWN_12_REGULAR
            },
            self.tint,
            !interactive,
            toggle,
            t,
        );
        let actions = div()
            .flex()
            .flex_none()
            .items_center()
            .children(clear_button)
            .child(chevron);

        let mut field = chrome
            .animated_style(div().id("field"), id.clone(), t, window, cx)
            .gap(px(t.spacing_1))
            .pr(px(t.spacing_1))
            .child(text)
            .child(actions);
        if self.disabled {
            field = field.cursor_not_allowed();
        } else {
            field = press_through(field.cursor_text(), &input);
        }
        if interactive {
            let open_key = show.clone();
            let escape_closed = self.clearable.then(|| clear.clone());
            field = keys(
                field,
                &state,
                rows.clone(),
                Rc::new(move |key, window, cx| open_key(Some(key), window, cx)),
                choose.clone(),
                close.clone(),
                escape_closed,
            );
        }

        let popup = open.then(|| {
            List {
                state: state.clone(),
                entries,
                checked: Some(checked),
                tint: self.tint,
                choose,
                close,
            }
            .sheet(
                &anchor,
                self.empty
                    .unwrap_or_else(|| "No matches".into_any_element()),
                t,
                cx,
            )
        });

        div()
            .id(id)
            .flex()
            .flex_col()
            .w_full()
            .child(measured(&anchor, field))
            .children(popup)
    }
}

/// A text field that suggests as you type. Its value is whatever was typed;
/// the list is a shortcut to a likely answer rather than the set of allowed
/// ones — a branch name, a search, a tag that may not exist yet. So there is
/// no chevron and no clear: nothing here is opened or chosen, and the field
/// is emptied the way any text field is.
#[derive(IntoElement)]
pub struct Autocomplete {
    id: gpui::ElementId,
    items: Vec<SharedString>,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    on_value_change: Option<OnText>,
    placeholder: SharedString,
    empty: Option<AnyElement>,
    size: Size,
    state: FieldState,
    tint: Tint,
    mono: bool,
    disabled: bool,
    read_only: bool,
}

impl Autocomplete {
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Autocomplete {
            id: id.into(),
            items: Vec::new(),
            value: None,
            default_value: None,
            on_value_change: None,
            placeholder: SharedString::default(),
            empty: None,
            size: Size::Medium,
            state: FieldState::Default,
            tint: Tint::Primary,
            mono: false,
            disabled: false,
            read_only: false,
        }
    }

    /// The suggestions offered as the reader types.
    pub fn items(mut self, items: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// What the list says when the text matches nothing.
    pub fn empty(mut self, empty: impl IntoElement) -> Self {
        self.empty = Some(empty.into_any_element());
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: FieldState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Set the mono face — for a path, a branch name, a key.
    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Accepted for React's shape; the constraint is the form's.
    pub fn required(self, _required: bool) -> Self {
        self
    }
}

impl RenderOnce for Autocomplete {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let id = self.id.clone();
        let input = input_state(part_id(&id, "input"), window, cx);
        let state = PopupState::keyed(&id, false, window, cx);
        let interactive = !self.disabled && !self.read_only;
        let open = state.read(cx).open && interactive;
        let anchor = state.read(cx).anchor.clone();

        let text_now = match &self.value {
            Some(value) => value.to_string(),
            None => input.read(cx).value().to_string(),
        };
        let entries: Vec<SelectEntry> = self
            .items
            .iter()
            .map(|item| SelectEntry::Option(SelectOption::new(item.clone(), item.clone())))
            .collect();
        let entries = filtered(&entries, &text_now);
        let rows = Rc::new(flatten(&entries));

        let close: Callback = {
            let state = state.clone();
            Rc::new(move |window, cx| state.update(cx, |state, cx| state.hide(window, cx)))
        };
        let choose: Choose = {
            let rows = rows.clone();
            let close = close.clone();
            let input = input.clone();
            let notify = self.on_value_change.clone();
            let controlled = self.value.is_some();
            Rc::new(move |index, window, cx| {
                let Some(option) = rows.get(index) else {
                    return;
                };
                close(window, cx);
                if !controlled {
                    let value = option.value.clone();
                    input.update(cx, |input, cx| input.set_value(value.to_string(), cx));
                }
                if let Some(notify) = &notify {
                    notify(option.value.clone(), window, cx);
                }
            })
        };
        let show: Rc<dyn Fn(&'static str, &mut Window, &mut App)> = {
            let state = state.clone();
            let enabled: Vec<bool> = rows.iter().map(|option| !option.disabled).collect();
            Rc::new(move |key, _, cx| {
                state.update(cx, |state, cx| {
                    state.open = true;
                    state.highlighted = None;
                    state.step(key, &enabled);
                    cx.notify();
                })
            })
        };

        let focused = input.read(cx).is_focused(window);
        let chrome = FieldBox::new(self.size)
            .tint(self.tint)
            .state(self.state)
            .focused(focused)
            .disabled(self.disabled)
            .mono(self.mono);

        let mut text = TextInput::new(part_id(&id, "text"))
            .state(input.clone())
            .placeholder(self.placeholder.clone())
            .placeholder_color(placeholder_color(t))
            .selection_color(selection_color(t))
            .read_only(self.read_only)
            .disabled(self.disabled);
        if let Some(value) = &self.value {
            text = text.value(value.clone());
        }
        if let Some(value) = &self.default_value {
            text = text.default_value(value.clone());
        }
        {
            let state = state.clone();
            let notify = self.on_value_change.clone();
            text = text.on_change(move |value, window, cx| {
                state.update(cx, |state, cx| {
                    state.open = true;
                    state.highlighted = None;
                    cx.notify();
                });
                if let Some(notify) = &notify {
                    notify(SharedString::from(value.to_string()), window, cx);
                }
            });
        }
        {
            let close = close.clone();
            text = text.on_blur(move |window, cx| close(window, cx));
        }

        let mut field = chrome
            .animated_style(div().id("field"), id.clone(), t, window, cx)
            .child(text);
        if self.disabled {
            field = field.cursor_not_allowed();
        } else {
            field = press_through(field.cursor_text(), &input);
        }
        if interactive {
            field = keys(
                field,
                &state,
                rows.clone(),
                show,
                choose.clone(),
                close.clone(),
                None,
            );
        }

        let popup = open.then(|| {
            List {
                state: state.clone(),
                entries,
                checked: None,
                tint: self.tint,
                choose,
                close,
            }
            .sheet(
                &anchor,
                self.empty
                    .unwrap_or_else(|| "No matches".into_any_element()),
                t,
                cx,
            )
        });

        div()
            .id(id)
            .flex()
            .flex_col()
            .w_full()
            .child(measured(&anchor, field))
            .children(popup)
    }
}
