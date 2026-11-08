//! A number, stepped or typed, with the unit it is counted in.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, prelude::FluentBuilder, px,
};

use crate::components::icon::Icon;
use crate::components::text_field::{
    FieldBox, FieldState, part_id, placeholder_color, selection_color,
};
use crate::foundation::text_input::{TextEditKind, TextInput, input_state};
use crate::foundation::{ControlColors, Size, State, StyledExt, Tint, Variant, pointer_down};
use crate::glyphs;
use crate::theme::ActiveTheme;

mod editing;
mod stepper;
use editing::parsed;

type OnNumber = Rc<dyn Fn(Option<f64>, &mut NumberFieldChangeDetails, &mut Window, &mut App)>;
type OnNumberCommit = Rc<dyn Fn(Option<f64>, NumberFieldCommitReason, &mut Window, &mut App)>;

/// The interaction proposing a numeric value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum NumberFieldChangeReason {
    /// A parseable text edit, including input methods.
    InputChange,
    /// The editor was cleared.
    InputClear,
    /// A nonempty value was normalized on focus loss.
    InputBlur,
    /// Clipboard text proposed a number.
    InputPaste,
    /// Arrow keys or an explicit Home/End boundary.
    Keyboard,
    /// Increment-button normalization or stepping.
    IncrementPress,
    /// Decrement-button normalization or stepping.
    DecrementPress,
    /// A focused wheel event.
    Wheel,
}

impl NumberFieldChangeReason {
    /// The source React component's event reason.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InputChange => "input-change",
            Self::InputClear => "input-clear",
            Self::InputBlur => "input-blur",
            Self::InputPaste => "input-paste",
            Self::Keyboard => "keyboard",
            Self::IncrementPress => "increment-press",
            Self::DecrementPress => "decrement-press",
            Self::Wheel => "wheel",
        }
    }
}

/// A proposed change. Canceling keeps the authoritative number unchanged;
/// text entry may still retain its editing draft, as the source input does.
#[derive(Debug)]
pub struct NumberFieldChangeDetails {
    /// What proposed the value.
    pub reason: NumberFieldChangeReason,
    /// `-1` or `1` for a relative step. Empty seeds, text normalization and
    /// explicit Home/End boundaries have no direction.
    pub direction: Option<i8>,
    canceled: bool,
}

impl NumberFieldChangeDetails {
    /// Reject the proposal before numeric state or commit history is changed.
    pub fn cancel(&mut self) {
        self.canceled = true;
    }
    /// Whether this proposal has been rejected.
    pub fn is_canceled(&self) -> bool {
        self.canceled
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChangeResult {
    Unchanged,
    Changed,
    Canceled,
}

impl ChangeResult {
    fn changed(self) -> bool {
        self == Self::Changed
    }
    fn canceled(self) -> bool {
        self == Self::Canceled
    }
}

type SetNumber = Rc<
    dyn Fn(
        Option<f64>,
        bool,
        NumberFieldChangeReason,
        Option<i8>,
        &mut Window,
        &mut App,
    ) -> ChangeResult,
>;

struct PressSeed {
    base: Option<f64>,
    previous: Option<f64>,
}

/// The interaction that finished a numeric edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum NumberFieldCommitReason {
    /// A nonempty text value was committed on focus loss.
    InputBlur,
    /// An empty text value was committed on focus loss.
    InputClear,
    /// An arrow key or explicit Home/End boundary changed the number.
    Keyboard,
    /// The increment stepper was released, including outside the field.
    IncrementPress,
    /// The decrement stepper was released, including outside the field.
    DecrementPress,
    /// A focused wheel event changed the number.
    Wheel,
}

impl NumberFieldCommitReason {
    /// The source React component's event reason.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InputBlur => "input-blur",
            Self::InputClear => "input-clear",
            Self::Keyboard => "keyboard",
            Self::IncrementPress => "increment-press",
            Self::DecrementPress => "decrement-press",
            Self::Wheel => "wheel",
        }
    }
}

/// What the field holds between frames: the number, and the text being
/// typed while it is not yet one.
#[derive(Default)]
struct Held {
    value: Option<f64>,
    draft: Option<String>,
    // A canceled step or blur can leave visible text without letting that
    // text supply the next step's numeric base.
    manual_input: bool,
    initialized: bool,
    pending_commit: bool,
    last_changed: Option<Option<f64>>,
    mount: Weak<KeyboardMount>,
    press: Option<PressSeed>,
}

type KeyboardHandler = Rc<dyn Fn(&gpui::KeystrokeEvent, &mut Window, &mut App)>;
struct Keyboard {
    handler: Rc<RefCell<Option<KeyboardHandler>>>,
    mount: Weak<KeyboardMount>,
}
struct KeyboardMount {
    _subscription: gpui::Subscription,
}

/// A number, with the two things a plain text field never gets right: a
/// stepper, and a value that stays a number. Parseable edits report numbers
/// immediately; partial edits remain a draft until stepping or committing.
/// Arrow keys use the regular, Shift or Alt step without losing the numeric
/// precision hidden by the formatted display.
/// Pointer steppers act on press and repeat while held; leaving pauses the
/// repeat and reentering starts it again until release.
///
/// The box is the [`TextField`](crate::TextField)'s, so a form's rows do not
/// step; what is the number field's own is the stepper at its trailing edge.
#[derive(IntoElement)]
pub struct NumberField {
    id: ElementId,
    value: Option<Option<f64>>,
    default_value: Option<f64>,
    on_value_change: Option<OnNumber>,
    on_value_committed: Option<OnNumberCommit>,
    min: Option<f64>,
    max: Option<f64>,
    step: f64,
    small_step: f64,
    large_step: f64,
    snap_on_step: bool,
    allow_wheel_scrub: bool,
    allow_out_of_range: bool,
    placeholder: SharedString,
    unit: Option<AnyElement>,
    size: Size,
    state: FieldState,
    tint: Tint,
    disabled: bool,
    read_only: bool,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>) -> Self {
        NumberField {
            id: id.into(),
            value: None,
            default_value: None,
            on_value_change: None,
            on_value_committed: None,
            min: None,
            max: None,
            step: 1.0,
            small_step: 0.1,
            large_step: 10.,
            snap_on_step: false,
            allow_wheel_scrub: false,
            allow_out_of_range: false,
            placeholder: SharedString::default(),
            unit: None,
            size: Size::Medium,
            state: FieldState::Default,
            tint: Tint::Primary,
            disabled: false,
            read_only: false,
        }
    }

    /// The number, controlled. `None` is an empty field.
    pub fn value(mut self, value: Option<f64>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: f64) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(Option<f64>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(move |value, _, window, cx| {
            handler(value, window, cx)
        }));
        self
    }

    /// Observe the proposal's reason and direction and optionally reject it.
    /// Like `on_value_change`, this installs the field's change handler.
    ///
    /// ```
    /// use dazzui::NumberField;
    /// let field = NumberField::new("count").on_value_change_with_details(
    ///     |value, details, _, _| {
    ///         if value.is_some_and(|value| value > 100.) {
    ///             details.cancel();
    ///         }
    ///     },
    /// );
    /// ```
    pub fn on_value_change_with_details(
        mut self,
        handler: impl Fn(Option<f64>, &mut NumberFieldChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Called on blur after manual editing, on release after a pointer hold,
    /// and immediately after a changed keyboard or wheel step.
    pub fn on_value_committed(
        mut self,
        handler: impl Fn(Option<f64>, NumberFieldCommitReason, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_committed = Some(Rc::new(handler));
        self
    }

    /// Keep out-of-range typed, pasted and blurred values. Interactive
    /// stepping still clamps to the bounds. Off by default.
    pub fn allow_out_of_range(mut self, allow: bool) -> Self {
        self.allow_out_of_range = allow;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// Alt uses this step instead of the regular or Shift step.
    pub fn small_step(mut self, step: f64) -> Self {
        self.small_step = step;
        self
    }
    pub fn large_step(mut self, step: f64) -> Self {
        self.large_step = step;
        self
    }
    /// Opt into alignment to multiples of the step, based at `min` or zero.
    pub fn snap_on_step(mut self, snap: bool) -> Self {
        self.snap_on_step = snap;
        self
    }

    /// What the field prints while it is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Allow focused wheel input to step the number. Off by default so a
    /// form can scroll past fields; Ctrl-wheel remains available for zoom.
    pub fn allow_wheel_scrub(mut self, allow: bool) -> Self {
        self.allow_wheel_scrub = allow;
        self
    }

    /// A trailing unit inside the box — `px`, `%`, `ms`.
    pub fn unit(mut self, unit: impl IntoElement) -> Self {
        self.unit = Some(unit.into_any_element());
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

    /// Accepted for React's shape; GPUI 0.2 has no accessibility tree.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

/// The default decimal display: grouping and up to three fraction digits.
/// Rounding the display must never replace the authoritative numeric value.
fn format(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let raw = value.abs().to_string();
    let (integer, fraction) = raw.split_once('.').unwrap_or((&raw, ""));
    let mut digits = format!("{integer}{:0<3}", &fraction[..fraction.len().min(3)]).into_bytes();
    if fraction
        .as_bytes()
        .get(3)
        .is_some_and(|digit| *digit >= b'5')
    {
        let mut carry = true;
        for digit in digits.iter_mut().rev() {
            if *digit == b'9' {
                *digit = b'0';
            } else {
                *digit += 1;
                carry = false;
                break;
            }
        }
        if carry {
            digits.insert(0, b'1');
        }
    }
    let split = digits.len() - 3;
    let integer = std::str::from_utf8(&digits[..split]).unwrap();
    let fraction = std::str::from_utf8(&digits[split..])
        .unwrap()
        .trim_end_matches('0');
    let mut text = String::new();
    if value.is_sign_negative() {
        text.push('-');
    }
    for (index, digit) in integer.chars().enumerate() {
        if index > 0 && (integer.len() - index) % 3 == 0 {
            text.push(',');
        }
        text.push(digit);
    }
    if !fraction.is_empty() {
        text.push('.');
        text.push_str(fraction);
    }
    text
}

#[derive(Clone)]
struct Rules {
    min: Option<f64>,
    max: Option<f64>,
    step: f64,
    small_step: f64,
    large_step: f64,
    snap: bool,
    allow_out_of_range: bool,
}
impl Rules {
    fn entered(&self, value: f64) -> f64 {
        if self.allow_out_of_range {
            value
        } else {
            self.clamp(value)
        }
    }
    fn clamp(&self, value: f64) -> f64 {
        value
            .max(self.min.unwrap_or(-9_007_199_254_740_991.))
            .min(self.max.unwrap_or(9_007_199_254_740_991.))
    }
    fn amount(&self, modifiers: gpui::Modifiers) -> f64 {
        if modifiers.alt {
            self.small_step
        } else if modifiers.shift {
            self.large_step
        } else {
            self.step
        }
    }
    fn stepped(&self, value: Option<f64>, direction: f64, modifiers: gpui::Modifiers) -> f64 {
        let Some(value) = value else {
            return self.clamp(0.);
        };
        let step = self.amount(modifiers) * direction;
        let mut next = value + step;
        if self.snap && step != 0. {
            let base = self.min.unwrap_or(0.);
            let size = step.abs();
            let raw = next - base + size * 1e-10 * step.signum();
            next = base
                + if modifiers.alt {
                    (raw / step + 0.5).floor() * step
                } else if step > 0. {
                    (raw / size).floor() * size
                } else {
                    (raw / size).ceil() * size
                };
        }
        next = self.clamp(next);
        // Base UI removes arithmetic noise only when doing so cannot erase
        // meaningful fractional digits at large magnitudes.
        let rounded = format!("{next:.14e}").parse::<f64>().unwrap_or(next);
        if (rounded - next).abs() <= (f64::EPSILON * next.abs().max(1.)).min(1e-10) {
            next = rounded;
        }
        self.clamp(next)
    }
}

impl RenderOnce for NumberField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let input = input_state(self.id.clone(), window, cx);
        let focused = input.read(cx).is_focused(window);
        let held = window
            .use_keyed_state(part_id(&self.id, "held"), cx, |_, _| {
                Rc::new(RefCell::new(Held::default()))
            })
            .read(cx)
            .clone();
        {
            let mut held = held.borrow_mut();
            if !held.initialized {
                held.value = self.default_value;
                held.initialized = true;
            }
            if let Some(value) = self.value {
                if !focused && held.value != value {
                    held.draft = None;
                }
                held.value = value;
            }
        }
        let rules = Rules {
            min: self.min,
            max: self.max,
            step: self.step,
            small_step: self.small_step,
            large_step: self.large_step,
            snap: self.snap_on_step,
            allow_out_of_range: self.allow_out_of_range,
        };
        let current = held.borrow().value;
        let text = held
            .borrow()
            .draft
            .clone()
            .unwrap_or_else(|| current.map(format).unwrap_or_default());
        let controlled = self.value.is_some();
        let on_change = self.on_value_change.clone();
        let editable = !self.disabled && !self.read_only;

        // Report a proposal before touching numeric state or commit history.
        // Accepted text edits restore their draft after this call.
        let set: SetNumber = {
            let held = held.clone();
            let on_change = on_change.clone();
            Rc::new(move |value, force, reason, direction, window, cx| {
                let previous = held.borrow().value;
                let changed = previous != value || force;
                if changed && let Some(on_change) = &on_change {
                    let mut details = NumberFieldChangeDetails {
                        reason,
                        direction,
                        canceled: false,
                    };
                    on_change(value, &mut details, window, cx);
                    if details.is_canceled() {
                        return ChangeResult::Canceled;
                    }
                }
                {
                    let mut held = held.borrow_mut();
                    held.draft = None;
                    if !controlled {
                        held.value = value;
                    }
                    held.last_changed = Some(value);
                    held.pending_commit |= changed;
                }
                window.refresh();
                if changed {
                    ChangeResult::Changed
                } else {
                    ChangeResult::Unchanged
                }
            })
        };
        let emit_commit: OnNumberCommit = {
            let held = held.clone();
            let on_commit = self.on_value_committed.clone();
            Rc::new(move |value, reason, window, cx| {
                held.borrow_mut().pending_commit = false;
                if let Some(on_commit) = &on_commit {
                    on_commit(value, reason, window, cx);
                }
            })
        };
        let commit = {
            let held = held.clone();
            let set = set.clone();
            let rules = rules.clone();
            let emit = emit_commit.clone();
            move |window: &mut Window, cx: &mut App| {
                let (draft, current, pending, manual) = {
                    let mut held = held.borrow_mut();
                    let manual = std::mem::take(&mut held.manual_input);
                    (held.draft.clone(), held.value, held.pending_commit, manual)
                };
                let text = draft.unwrap_or_else(|| current.map(format).unwrap_or_default());
                if text.trim().is_empty() {
                    if set(
                        None,
                        false,
                        NumberFieldChangeReason::InputClear,
                        None,
                        window,
                        cx,
                    )
                    .canceled()
                    {
                        return;
                    }
                    if manual || pending || current.is_some() {
                        emit(None, NumberFieldCommitReason::InputClear, window, cx);
                    }
                } else {
                    let Some(parsed) = parsed(&text) else { return };
                    // A rounded display is not a fresh manual edit. Preserve
                    // authoritative precision through an untouched focus/blur.
                    let value = if manual { Some(parsed) } else { current };
                    let changed = value != current;
                    let committed = if changed {
                        let value = value.map(|value| rules.entered(value));
                        if set(
                            value,
                            true,
                            NumberFieldChangeReason::InputBlur,
                            None,
                            window,
                            cx,
                        )
                        .canceled()
                        {
                            return;
                        }
                        value
                    } else {
                        current
                    };
                    held.borrow_mut().draft = None;
                    if manual || changed || pending {
                        emit(committed, NumberFieldCommitReason::InputBlur, window, cx);
                    }
                }
            }
        };
        let step_by = {
            let held = held.clone();
            let set = set.clone();
            let rules = rules.clone();
            move |direction: f64,
                  modifiers: gpui::Modifiers,
                  reason: NumberFieldChangeReason,
                  window: &mut Window,
                  cx: &mut App| {
                // A dirty draft is the stepping base even when a controlled
                // parent refused its numeric proposal. Publish only the step.
                let mut held = held.borrow_mut();
                let manual = std::mem::take(&mut held.manual_input);
                let press = held.press.take();
                let value = if let Some(press) = &press {
                    press.base
                } else if manual {
                    held.draft
                        .as_ref()
                        .and_then(|text| parsed(text))
                        .or(held.value)
                } else {
                    held.value
                };
                if !manual && press.is_none() {
                    held.last_changed = Some(held.value);
                }
                drop(held);
                let next = rules.stepped(value, direction, modifiers);
                set(
                    Some(next),
                    press.is_some_and(|press| press.previous != Some(next)),
                    reason,
                    value.map(|_| if direction > 0. { 1 } else { -1 }),
                    window,
                    cx,
                )
            }
        };

        let can_increment = !self.disabled
            && match (current, self.max) {
                (Some(value), Some(max)) => value < max,
                _ => true,
            };
        let can_decrement = !self.disabled
            && match (current, self.min) {
                (Some(value), Some(min)) => value > min,
                _ => true,
            };

        let edit_kind = Rc::new(Cell::new(TextEditKind::Input));
        let validate_kind = edit_kind.clone();
        let mut text_input = TextInput::new(self.id.clone())
            .state(input.clone())
            .value(text)
            .placeholder(self.placeholder)
            .placeholder_color(placeholder_color(t))
            .selection_color(selection_color(t))
            .disabled(self.disabled)
            .read_only(self.read_only)
            .validate_edit(move |text, kind| {
                validate_kind.set(kind);
                match kind {
                    TextEditKind::Input => editing::valid_text(text),
                    TextEditKind::Paste => parsed(text).is_some(),
                }
            })
            .on_change({
                let held = held.clone();
                let set = set.clone();
                let rules = rules.clone();
                move |text, window, cx| {
                    held.borrow_mut().manual_input = true;
                    if text.trim().is_empty() {
                        set(
                            None,
                            true,
                            NumberFieldChangeReason::InputClear,
                            None,
                            window,
                            cx,
                        );
                    } else if let Some(value) = parsed(text) {
                        let reason = if edit_kind.get() == TextEditKind::Paste {
                            NumberFieldChangeReason::InputPaste
                        } else {
                            NumberFieldChangeReason::InputChange
                        };
                        set(Some(rules.entered(value)), true, reason, None, window, cx);
                    }
                    held.borrow_mut().draft = Some(text.to_string());
                }
            })
            .on_blur({
                let commit = commit.clone();
                let live = held.clone();
                move |window, cx| {
                    if !editable || live.borrow().mount.upgrade().is_none() {
                        return;
                    }
                    commit(window, cx);
                    // Focus observers run after paint. Refresh outside that
                    // pass so a normalized draft reaches the visible editor.
                    window.defer(cx, |window, _| window.refresh());
                }
            });

        if self.allow_wheel_scrub && editable {
            let wheel_input = input.clone();
            let wheel_held = held.clone();
            let wheel_set = set.clone();
            let wheel_rules = rules.clone();
            let emit = emit_commit.clone();
            text_input = text_input.on_scroll_wheel(move |event, window, cx| {
                if event.modifiers.control || !wheel_input.read(cx).is_focused(window) {
                    return;
                }
                window.prevent_default();
                // GPUI's content-motion delta has the opposite sign from
                // DOM WheelEvent.deltaY. Each event is one step regardless
                // of pixel/line magnitude, including a horizontal event.
                let direction = if event.delta.pixel_delta(px(1.)).y < px(0.) {
                    -1.
                } else {
                    1.
                };
                // Wheel steps use the authoritative number, unlike keyboard
                // steps from a dirty, controlled input draft.
                let value = wheel_held.borrow().value;
                wheel_held.borrow_mut().manual_input = false;
                let next = Some(wheel_rules.stepped(value, direction, event.modifiers));
                if wheel_set(
                    next,
                    false,
                    NumberFieldChangeReason::Wheel,
                    value.map(|_| if direction > 0. { 1 } else { -1 }),
                    window,
                    cx,
                )
                .changed()
                {
                    emit(next, NumberFieldCommitReason::Wheel, window, cx);
                }
            });
        }

        let tint = self.state.tint(self.tint);
        let mut step_button =
            |id: &'static str, glyph: &'static str, enabled: bool, direction: f64| {
                let step_by = step_by.clone();
                let step_input = input.clone();
                let press_held = held.clone();
                let press_set = set.clone();
                let press_rules = rules.clone();
                let stop_held = held.clone();
                let emit = emit_commit.clone();
                let read_only = self.read_only;
                let disabled = ControlColors::resolve(t, Variant::Normal, tint, State::Disabled);
                let hovered = ControlColors::resolve(t, Variant::Normal, tint, State::Hovered);
                // Two stacked actions share the field's height, so the glyph
                // must fit its half even in the most compact theme.
                let glyph_size = t.icon_small.min(t.spacing_3);
                let button = div()
                    .id(part_id(&self.id, id))
                    .debug_selector(move || format!("number-{id}"))
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(t.spacing_4))
                    .h(px(t.spacing_3))
                    .rounded(px(t.radius_tiny))
                    .map(|this| {
                        if enabled {
                            this.text_color(t.color_content_subtle.hsla())
                                .cursor_pointer()
                                .hover(move |style| {
                                    style.bg(hovered.surface).text_color(t.color_content.hsla())
                                })
                        } else {
                            this.text_color(disabled.content).cursor_not_allowed()
                        }
                    })
                    .child(Icon::new(glyph).size(glyph_size));
                stepper::bind(
                    button,
                    part_id(&part_id(&self.id, id), "hold"),
                    (enabled && !read_only).then(|| {
                        Rc::new(move |modifiers, window: &mut Window, cx: &mut App| {
                            step_by(
                                direction,
                                modifiers,
                                if direction > 0. {
                                    NumberFieldChangeReason::IncrementPress
                                } else {
                                    NumberFieldChangeReason::DecrementPress
                                },
                                window,
                                cx,
                            )
                            .changed()
                        })
                            as Rc<dyn Fn(gpui::Modifiers, &mut Window, &mut App) -> bool>
                    }),
                    move |window, cx| {
                        pointer_down(window, cx);
                        step_input.read(cx).focus(window);
                        // The source batches pointer-down normalization and
                        // its first tick against the same render's value.
                        let previous = press_held.borrow().value;
                        let mut base = previous;
                        // Base UI commits dirty input before the first step.
                        // Only an accepted normalization supplies that base,
                        // even when a controlled parent keeps its own value.
                        let draft = {
                            let mut held = press_held.borrow_mut();
                            let manual = std::mem::take(&mut held.manual_input);
                            if manual { held.draft.clone() } else { None }
                        };
                        if let Some(draft) = draft
                            && let Some(value) = parsed(&draft)
                        {
                            let result = press_set(
                                Some(press_rules.clamp(value)),
                                false,
                                if direction > 0. {
                                    NumberFieldChangeReason::IncrementPress
                                } else {
                                    NumberFieldChangeReason::DecrementPress
                                },
                                None,
                                window,
                                cx,
                            );
                            if !result.canceled() {
                                base = Some(value);
                            }
                        }
                        press_held.borrow_mut().press = Some(PressSeed { base, previous });
                        press_held.borrow_mut().last_changed = None;
                    },
                    move |window, cx| {
                        let value = {
                            let held = stop_held.borrow();
                            held.last_changed.flatten().or(held.value)
                        };
                        emit(
                            value,
                            if direction > 0. {
                                NumberFieldCommitReason::IncrementPress
                            } else {
                                NumberFieldCommitReason::DecrementPress
                            },
                            window,
                            cx,
                        );
                    },
                    window,
                    cx,
                )
            };

        // The stepper: two half-height buttons in one column, hairlined off
        // the value. Stacked rather than a minus and a plus on either side —
        // a value with a control on both sides reads as three things.
        let stepper = div()
            .flex()
            .flex_col()
            .flex_none()
            .map(|mut this| {
                this.style().align_self = Some(gpui::AlignSelf::Stretch);
                this
            })
            .justify_center()
            .my(px(t.spacing_1))
            .border_l(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
            .pl(px(t.spacing_1))
            .child(step_button(
                "increment",
                glyphs::CHEVRON_UP_12_REGULAR,
                can_increment,
                1.0,
            ))
            .child(step_button(
                "decrement",
                glyphs::CHEVRON_DOWN_12_REGULAR,
                can_decrement,
                -1.0,
            ));

        let label = t.label_quiet;
        let unit = self.unit.map(|unit| {
            div()
                .flex_none()
                .text_color(t.color_content_faint.hsla())
                .font_family(label.face.family())
                .text_size(px(t.label_small.size))
                .line_height(px(t.label_small.size))
                .child(unit)
        });

        let chrome = FieldBox::new(self.size)
            .tint(self.tint)
            .state(self.state)
            .focused(focused)
            .disabled(self.disabled);
        // Editing actions run before ordinary key handlers in GPUI. Keep
        // numeric keys ahead of those actions, scoped to this focused input.
        let keyboard = window.use_keyed_state(part_id(&self.id, "keyboard"), cx, |_, _| Keyboard {
            handler: Rc::default(),
            mount: Weak::new(),
        });
        let mount = keyboard.update(cx, |keyboard, cx| {
            if let Some(mount) = keyboard.mount.upgrade() {
                return mount;
            }
            let callback = keyboard.handler.clone();
            let subscription = cx.intercept_keystrokes(move |event, window, cx| {
                let handler = callback.borrow().clone();
                if let Some(handler) = handler {
                    handler(event, window, cx);
                }
            });
            let mount = Rc::new(KeyboardMount {
                _subscription: subscription,
            });
            keyboard.mount = Rc::downgrade(&mount);
            mount
        });
        held.borrow_mut().mount = Rc::downgrade(&mount);
        let window_id = window.window_handle().window_id();
        let key_input = input.clone();
        let key_held = held.clone();
        let key_emit = emit_commit.clone();
        *keyboard.read(cx).handler.borrow_mut() = if editable {
            Some(Rc::new(
                move |event: &gpui::KeystrokeEvent, window: &mut Window, cx: &mut App| {
                    let key = &event.keystroke;
                    if window.window_handle().window_id() != window_id
                        || !key_input.read(cx).is_focused(window)
                        || window.default_prevented()
                        || key.modifiers.control
                        || key.modifiers.platform
                    {
                        return;
                    }
                    if key.modifiers.alt && !matches!(key.key.as_str(), "up" | "down") {
                        return;
                    }
                    let input = key_input.read(cx);
                    if !editing::allowed_key(
                        &key.key,
                        input.value(),
                        input.selected_range(),
                        rules.min,
                        rules.allow_out_of_range,
                    ) {
                        window.prevent_default();
                        cx.stop_propagation();
                        return;
                    }
                    let action = match key.key.as_str() {
                        "up" => Some((None, 1.)),
                        "down" => Some((None, -1.)),
                        "home" if !key.modifiers.alt => rules.min.map(|value| (Some(value), 0.)),
                        "end" if !key.modifiers.alt => rules.max.map(|value| (Some(value), 0.)),
                        _ => None,
                    };
                    if let Some((boundary, direction)) = action {
                        window.prevent_default();
                        cx.stop_propagation();
                        let changed = if let Some(value) = boundary {
                            key_held.borrow_mut().manual_input = false;
                            set(
                                Some(value),
                                false,
                                NumberFieldChangeReason::Keyboard,
                                None,
                                window,
                                cx,
                            )
                        } else {
                            step_by(
                                direction,
                                key.modifiers,
                                NumberFieldChangeReason::Keyboard,
                                window,
                                cx,
                            )
                        };
                        if changed.changed() {
                            let value = key_held.borrow().last_changed.flatten();
                            key_emit(value, NumberFieldCommitReason::Keyboard, window, cx);
                        }
                    }
                },
            ) as KeyboardHandler)
        } else {
            None
        };
        let field = chrome
            .animated_style(div().id(part_id(&self.id, "field")), self.id, t, window, cx)
            .tabular_nums()
            .gap(px(t.spacing_1))
            // The trailing inset is the stepper's own: it is a control rather
            // than text, so it sits closer to the edge than a value would.
            .pr(px(t.spacing_1))
            .map(|this| {
                if self.disabled {
                    this.cursor_not_allowed()
                } else {
                    this.cursor_text()
                }
            })
            .when(!self.disabled, |this| {
                let input = input.downgrade();
                this.on_mouse_down(MouseButton::Left, move |event, window, cx| {
                    input
                        .update(cx, |input, cx| input.mouse_down(event, window, cx))
                        .ok();
                })
            })
            .child(text_input)
            .children(unit)
            .child(stepper);
        // Dispatch listeners live with the rendered tree, including between
        // paints. Retain the subscription there, releasing it on unmount.
        field.on_key_down(move |_, _, _| {
            let _ = &mount;
        })
    }
}
