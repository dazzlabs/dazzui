//! The control a settings row rebinds a shortcut with.

use std::rc::Rc;
use std::time::Duration;

use gpui::{
    App, BoxShadow, ElementId, InteractiveElement, IntoElement, KeyDownEvent, Keystroke, Modifiers,
    MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, div, point, prelude::FluentBuilder, px,
};

use crate::components::icon::Icon;
use crate::components::text_field::{FieldState, glow, part_id};
use crate::foundation::text_input::{OnEvent, OnText};
use crate::foundation::{Tint, focus_visible, pointer_down, ring};
use crate::glyphs;
use crate::theme::ActiveTheme;

/// Modifiers in the order macOS prints them: ⌃ ⌥ ⇧ ⌘.
fn modifier_glyphs(modifiers: &Modifiers) -> String {
    [
        (modifiers.control, '⌃'),
        (modifiers.alt, '⌥'),
        (modifiers.shift, '⇧'),
        (modifiers.platform, '⌘'),
    ]
    .iter()
    .filter(|(held, _)| *held)
    .map(|(_, glyph)| *glyph)
    .collect()
}

/// A key by where it is rather than by what it types: with ⌥ held the T key
/// types `†`, and a recorded shortcut would print as gibberish. GPUI's
/// `key` is already the unmodified key, which is what this reads.
fn key_glyph(key: &str) -> Option<String> {
    let named = match key {
        "space" => "Space",
        "enter" => "↩",
        "tab" => "⇥",
        "backspace" => "⌫",
        "delete" => "⌦",
        "up" => "↑",
        "down" => "↓",
        "left" => "←",
        "right" => "→",
        "home" => "↖",
        "end" => "↘",
        "pageup" => "⇞",
        "pagedown" => "⇟",
        _ => "",
    };
    if !named.is_empty() {
        return Some(named.into());
    }
    if let Some(number) = key.strip_prefix('f')
        && let Ok(number) = number.parse::<u8>()
        && (1..=20).contains(&number)
    {
        return Some(format!("F{number}"));
    }
    let mut chars = key.chars();
    match (chars.next(), chars.next()) {
        (Some(ch), None) if ch.is_ascii_graphic() => Some(ch.to_ascii_uppercase().to_string()),
        _ => None,
    }
}

/// The glyph string for a keystroke — `⌥⇧2`, `⌥ Space` — or `None` when it
/// is not something a shortcut can be made of.
///
/// A key on its own is typing, not a shortcut, so it has to carry ⌃, ⌥ or ⌘
/// — ⇧ alone is still typing — unless it is a function key, which is a
/// shortcut by itself. Word keys get a space after the modifiers, so
/// `⌥ Space` reads as two things and `⌥⇧2` as one.
pub fn format_shortcut(keystroke: &Keystroke) -> Option<String> {
    let key = key_glyph(&keystroke.key)?;
    let function = key.starts_with('F') && key.len() > 1;
    let m = &keystroke.modifiers;
    if !function && !m.control && !m.alt && !m.platform {
        return None;
    }
    let mods = modifier_glyphs(m);
    Some(if key.chars().count() > 1 {
        format!("{mods}{}{key}", if mods.is_empty() { "" } else { " " })
    } else {
        format!("{mods}{key}")
    })
}

#[derive(Default)]
struct Recording {
    recording: bool,
    /// Modifiers held right now, printed while recording.
    held: String,
    /// Set for a beat after a press the recorder could not take.
    rejected: bool,
}

/// Click it and it listens; press a combination and it keeps it. Escape
/// backs out, ⌫ clears, and leaving it cancels. Held modifiers show live
/// while it listens, so pressing ⌥⇧ and then wondering which key to add is
/// not done blind.
///
/// Drawn as a field, not a key cap: a `KeyCap` says what the key is, a field
/// says you can change it.
#[derive(IntoElement)]
pub struct ShortcutRecorder {
    id: ElementId,
    value: SharedString,
    on_value_change: Option<OnText>,
    on_clear: Option<OnEvent>,
    placeholder: SharedString,
    recording_label: SharedString,
    state: FieldState,
    disabled: bool,
    tint: Tint,
}

impl ShortcutRecorder {
    /// `value` is the bound keys as printed — `⌥⇧2`, `⌥ Space`; empty is
    /// unbound.
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        ShortcutRecorder {
            id: id.into(),
            value: value.into(),
            on_value_change: None,
            on_clear: None,
            placeholder: "Record shortcut".into(),
            recording_label: "Press keys…".into(),
            state: FieldState::Default,
            disabled: false,
            tint: Tint::Primary,
        }
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    /// Called on clear or ⌫ while recording; `on_value_change("")` otherwise.
    pub fn on_clear(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_clear = Some(Rc::new(handler));
        self
    }

    /// Shown while idle and unbound.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Shown while the control waits for a key.
    pub fn recording_label(mut self, label: impl Into<SharedString>) -> Self {
        self.recording_label = label.into();
        self
    }

    /// `Error` for a binding the page cannot accept — one already taken.
    pub fn state(mut self, state: FieldState) -> Self {
        self.state = state;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Accepted for React's shape; GPUI 0.2 has no accessibility tree.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    /// See [`aria_label`](Self::aria_label).
    pub fn clear_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

impl RenderOnce for ShortcutRecorder {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let focus = window
            .use_keyed_state(part_id(&self.id, "focus"), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let state = window.use_keyed_state(part_id(&self.id, "recording"), cx, |_, _| {
            Recording::default()
        });
        // Leaving the control cancels a recording.
        let focused = focus.is_focused(window);
        if !focused && state.read(cx).recording {
            state.update(cx, |state, _| {
                state.recording = false;
                state.held.clear();
            });
        }
        let (recording, held, rejected) = {
            let state = state.read(cx);
            (state.recording, state.held.clone(), state.rejected)
        };

        let bound = !self.value.is_empty();
        let label: SharedString = if recording {
            if held.is_empty() {
                self.recording_label.clone()
            } else {
                held.clone().into()
            }
        } else if bound {
            self.value.clone()
        } else {
            self.placeholder.clone()
        };
        let shows_glyphs = if recording { !held.is_empty() } else { bound };
        let clearable = bound && !recording && !self.disabled;

        let clear: OnEvent = {
            let on_clear = self.on_clear.clone();
            let on_change = self.on_value_change.clone();
            let state = state.downgrade();
            Rc::new(move |window, cx| {
                state
                    .update(cx, |state, cx| {
                        state.recording = false;
                        state.held.clear();
                        cx.notify();
                    })
                    .ok();
                if let Some(on_clear) = &on_clear {
                    on_clear(window, cx);
                } else if let Some(on_change) = &on_change {
                    on_change("", window, cx);
                }
            })
        };

        // The stylesheet's rules, in its order: the prompt faint, recording
        // lifted behind the accent, a rejected press flashing danger, a
        // stored conflict repainting the chip, disabled dimming it.
        let tint_ramp = self.tint.ramp(t);
        let danger = &t.color_danger;
        let mut border = t.color_border_strong.hsla();
        let mut surface = t.color_surface_muted.hsla();
        let mut ink = t.color_content.hsla();
        let mut shadow: Vec<BoxShadow> = Vec::new();
        let type_style = if shows_glyphs {
            t.label_strong
        } else {
            ink = t.color_content_faint.hsla();
            crate::tokens::TypeStyle {
                face: t.label_quiet.face,
                weight: t.label_quiet.weight,
                ..t.label_strong
            }
        };
        if recording {
            border = tint_ramp.shade(600).hsla();
            surface = t.color_surface.hsla();
            shadow = vec![glow(t, self.tint)];
        }
        if rejected {
            border = danger.shade(600).hsla();
            shadow = vec![BoxShadow {
                color: danger.shade(50).hsla(),
                offset: point(px(0.), px(0.)),
                blur_radius: px(0.),
                spread_radius: px(t.focus_width),
            }];
        }
        if self.state == FieldState::Error {
            border = danger.shade(600).hsla();
            surface = danger.shade(50).hsla();
            ink = danger.shade(800).hsla();
        }
        if !recording && focus_visible(&focus, window, cx) {
            shadow.push(ring(t));
        }
        if self.disabled {
            ink = t.color_content_faint.hsla();
        }

        let on_key_down = {
            let state = state.downgrade();
            let on_change = self.on_value_change.clone();
            let clear = clear.clone();
            move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                let Some(entity) = state.upgrade() else {
                    return;
                };
                let keystroke = &event.keystroke;
                if !entity.read(cx).recording {
                    // A button starts on Space or Return.
                    if matches!(keystroke.key.as_str(), "space" | "enter")
                        && !keystroke.modifiers.modified()
                    {
                        cx.stop_propagation();
                        entity.update(cx, |state, cx| {
                            state.recording = true;
                            cx.notify();
                        });
                    }
                    return;
                }
                // Nothing typed here may reach the page: no dialog closes on
                // this Escape, no ⌘F opens a search.
                cx.stop_propagation();
                let stop = |cx: &mut App| {
                    entity.update(cx, |state, cx| {
                        state.recording = false;
                        state.held.clear();
                        cx.notify();
                    });
                };
                let m = &keystroke.modifiers;
                if keystroke.key == "escape" {
                    stop(cx);
                    return;
                }
                let bare = !m.control && !m.alt && !m.platform && !m.shift;
                if bare && matches!(keystroke.key.as_str(), "backspace" | "delete") {
                    clear(window, cx);
                    return;
                }
                match format_shortcut(keystroke) {
                    Some(next) => {
                        stop(cx);
                        if let Some(on_change) = &on_change {
                            on_change(&next, window, cx);
                        }
                    }
                    None => {
                        entity.update(cx, |state, cx| {
                            state.rejected = true;
                            cx.notify();
                        });
                        let weak = entity.downgrade();
                        cx.spawn(async move |cx| {
                            cx.background_executor()
                                .timer(Duration::from_secs_f32(
                                    t.shortcut_recorder_reject_duration / 1000.,
                                ))
                                .await;
                            weak.update(cx, |state, cx| {
                                state.rejected = false;
                                cx.notify();
                            })
                            .ok();
                        })
                        .detach();
                    }
                }
            }
        };

        let disabled = self.disabled;
        let mut button = div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .justify_center()
            .h(px(t.control_medium_size))
            .min_w(px(t.shortcut_recorder_width))
            .px(px(if clearable { t.spacing_8 } else { t.spacing_3 }))
            .border(px(t.stroke_hairline))
            .border_color(border)
            .rounded(px(t.control_field_radius))
            .bg(surface)
            .text_color(ink)
            .whitespace_nowrap()
            .font_family(type_style.face.family())
            .text_size(px(type_style.size))
            .font_weight(type_style.font_weight())
            .line_height(px(type_style.line_height))
            .shadow(shadow)
            .child(crate::foundation::text_child(label));
        if disabled {
            button = button.cursor_not_allowed().opacity(0.6);
        } else {
            let state = state.downgrade();
            let modifiers_state = state.clone();
            let start_focus = focus.clone();
            button = button
                .track_focus(&focus.clone().tab_stop(true))
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .on_click(move |_, window, cx| {
                    window.focus(&start_focus);
                    state
                        .update(cx, |state, cx| {
                            if !state.recording {
                                state.recording = true;
                                cx.notify();
                            }
                        })
                        .ok();
                })
                .on_key_down(on_key_down)
                .on_modifiers_changed({
                    let state = modifiers_state;
                    move |event, _, cx| {
                        state
                            .update(cx, |state, cx| {
                                if state.recording {
                                    state.held = modifier_glyphs(&event.modifiers);
                                    cx.notify();
                                }
                            })
                            .ok();
                    }
                });
        }

        let clear_button = clearable.then(|| {
            div()
                .id(part_id(&self.id, "clear"))
                .absolute()
                .top_0()
                .bottom_0()
                .right(px(t.spacing_2))
                .flex()
                .items_center()
                .text_color(t.color_content_faint.hsla())
                .hover(|style| style.text_color(t.color_content_subtle.hsla()))
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, |_, window, cx| {
                    pointer_down(window, cx);
                    cx.stop_propagation();
                })
                .on_click(move |_, window, cx| clear(window, cx))
                .child(Icon::new(glyphs::DISMISS_CIRCLE_12_FILLED).size(t.icon_small))
        });

        div()
            .relative()
            .flex()
            .flex_none()
            .child(button)
            .when_some(clear_button, |this, clear| this.child(clear))
    }
}
