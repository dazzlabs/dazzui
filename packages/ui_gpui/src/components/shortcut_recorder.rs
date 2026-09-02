use gpui::{
    App, ElementId, InteractiveElement, IntoElement, Keystroke, Modifiers, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, point, px,
};

use crate::assets::icons;
use crate::components::Icon;
use crate::prelude::*;
use crate::{
    SharedValueListener, TextFieldState, Tint, ValueListener, colors, hairline, latched, pressable,
};

/*
 * A shortcut is kept as the glyph string a settings page prints — `⌥⇧2`,
 * `⌥ Space` — rather than as a key-code tuple, because that is what every
 * other place showing a shortcut already carries. A host that stores
 * `Option+Shift+2` is one mapping away.
 */

/// Modifiers in the order macOS prints them: ⌃ ⌥ ⇧ ⌘.
fn modifier_glyphs(modifiers: &Modifiers) -> String {
    let mut glyphs = String::new();
    if modifiers.control {
        glyphs.push('⌃');
    }
    if modifiers.alt {
        glyphs.push('⌥');
    }
    if modifiers.shift {
        glyphs.push('⇧');
    }
    if modifiers.platform {
        glyphs.push('⌘');
    }
    glyphs
}

/// The glyph a named key prints.
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
        "escape" => return None,
        _ => "",
    };
    if !named.is_empty() {
        return Some(named.to_string());
    }

    // A function key is a shortcut on its own; everything else is one
    // character, printed the way a menu prints it.
    if key.len() >= 2 && key.starts_with('f') && key[1..].chars().all(|c| c.is_ascii_digit()) {
        return Some(key.to_uppercase());
    }
    if key.chars().count() == 1 {
        return Some(key.to_uppercase());
    }
    None
}

/// The glyph string for a keystroke, or `None` when it is not something a
/// shortcut can be made of.
///
/// A key on its own is typing, not a shortcut, so it has to carry ⌃, ⌥ or ⌘ —
/// ⇧ alone is still typing — unless it is a function key, which is a shortcut
/// by itself. Word keys (`Space`, `F5`) get a space after the modifiers so
/// `⌥ Space` reads as two things and `⌥⇧2` as one.
pub fn format_shortcut(keystroke: &Keystroke) -> Option<SharedString> {
    let key = key_glyph(&keystroke.key)?;
    let modifiers = &keystroke.modifiers;
    let function_key =
        key.starts_with('F') && key.len() > 1 && key[1..].chars().all(|c| c.is_ascii_digit());

    if !function_key && !modifiers.control && !modifiers.alt && !modifiers.platform {
        return None;
    }

    let mods = modifier_glyphs(modifiers);
    Some(SharedString::from(if key.chars().count() > 1 {
        format!("{mods}{}{key}", if mods.is_empty() { "" } else { " " })
    } else {
        format!("{mods}{key}")
    }))
}

/// The control a settings row uses to rebind a shortcut.
///
/// Click it and it listens; press a combination and it keeps it. Escape backs
/// out, ⌫ clears, and clicking away cancels. Held modifiers show live while
/// it listens, so pressing ⌥⇧ and then wondering which key to add is not done
/// blind.
///
/// It is drawn as a field, not a key cap: a `KeyCap` says "this is what the key
/// is", a field says "you can change this".
#[derive(IntoElement)]
pub struct ShortcutRecorder {
    id: ElementId,
    /// The bound keys as printed — `⌥⇧2`, `⌥ Space`. Empty means unbound.
    value: SharedString,
    /// Shown while idle and unbound.
    placeholder: SharedString,
    /// Shown while the control waits for a key.
    recording_label: SharedString,
    state: TextFieldState,
    tint: Tint,
    disabled: bool,
    on_change: Option<ValueListener<SharedString>>,
}

impl ShortcutRecorder {
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        ShortcutRecorder {
            id: id.into(),
            value: value.into(),
            placeholder: "Record shortcut".into(),
            recording_label: "Press keys…".into(),
            state: TextFieldState::Default,
            tint: Tint::Primary,
            disabled: false,
            on_change: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn recording_label(mut self, label: impl Into<SharedString>) -> Self {
        self.recording_label = label.into();
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

    /// Called with the new binding, and with an empty string when it is
    /// cleared.
    pub fn on_change(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(listener));
        self
    }
}

/// What the recorder remembers between frames: whether it is listening, and
/// which modifiers are held while it does.
type Recording = (bool, Modifiers);

impl RenderOnce for ShortcutRecorder {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let line = hairline(vars, window);
        let ShortcutRecorder {
            id,
            value,
            placeholder,
            recording_label,
            state,
            tint,
            disabled,
            on_change,
        } = self;
        let on_change: Option<SharedValueListener<SharedString>> = on_change.map(std::rc::Rc::from);
        let accent = vars.ramp(tint).shade(600);
        let danger = vars.color_danger;

        latched::<Recording>(id.clone(), move |recording, _, _| {
            let (listening, held) = recording.get();
            let listening = listening && !disabled;
            let bound = !value.is_empty();
            let held_glyphs = modifier_glyphs(&held);

            let label = if listening {
                if held_glyphs.is_empty() {
                    recording_label.clone()
                } else {
                    SharedString::from(held_glyphs.clone())
                }
            } else if bound {
                value.clone()
            } else {
                placeholder.clone()
            };
            // Unbound, or waiting with nothing held: the label is a prompt,
            // not a value.
            let glyphs = if listening {
                !held_glyphs.is_empty()
            } else {
                bound
            };
            let clearable = bound && !listening && !disabled;

            pressable(id.clone(), !disabled, move |_, _, _| {
                let clear = {
                    let on_change = on_change.clone();
                    let recording = recording.clone();
                    move |window: &mut Window, cx: &mut App| {
                        recording.set((false, Modifiers::default()));
                        if let Some(listener) = on_change.as_ref() {
                            listener(&SharedString::default(), window, cx);
                        }
                    }
                };

                div()
                    .relative()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(vars.control_medium_size)
                    .min_w(vars.shortcut_recorder_width)
                    .map(|el| {
                        // Room for the clear button, mirrored on the left so
                        // the keys stay centred whether or not it is there.
                        if clearable {
                            el.px(vars.spacing_8)
                        } else {
                            el.px(vars.spacing_3)
                        }
                    })
                    .rounded(vars.radius_medium)
                    .border(line)
                    .type_style(if glyphs {
                        vars.label_strong
                    } else {
                        vars.label_quiet
                    })
                    .map(|el| {
                        if state == TextFieldState::Error {
                            el.border_color(danger.shade(600))
                                .bg(danger.shade(50))
                                .text_color(danger.shade(800))
                        } else if listening {
                            el.border_color(accent)
                                .bg(vars.color_surface)
                                .text_color(if glyphs {
                                    vars.color_content
                                } else {
                                    vars.color_content_faint
                                })
                                .shadow(vec![gpui::BoxShadow {
                                    color: accent.opacity(vars.focus_glow_alpha),
                                    offset: point(px(0.), px(0.)),
                                    blur_radius: px(0.),
                                    spread_radius: vars.focus_width,
                                }])
                        } else {
                            el.border_color(vars.color_border_strong)
                                .bg(vars.color_surface_muted)
                                .text_color(if glyphs {
                                    vars.color_content
                                } else {
                                    vars.color_content_faint
                                })
                        }
                    })
                    .map(|el| {
                        if disabled {
                            el.cursor_not_allowed().opacity(0.6)
                        } else {
                            el.cursor_pointer()
                        }
                    })
                    .id(id.clone())
                    .focusable()
                    .child(label.clone())
                    .when(!disabled, |el| {
                        el.on_click({
                            let recording = recording.clone();
                            move |_, window, _| {
                                recording.set((true, Modifiers::default()));
                                window.refresh();
                            }
                        })
                        .on_mouse_down_out({
                            let recording = recording.clone();
                            move |_, window, _| {
                                if recording.get().0 {
                                    recording.set((false, Modifiers::default()));
                                    window.refresh();
                                }
                            }
                        })
                        .on_modifiers_changed({
                            let recording = recording.clone();
                            move |event, window, _| {
                                if recording.get().0 {
                                    recording.set((true, event.modifiers));
                                    window.refresh();
                                }
                            }
                        })
                        .on_key_down({
                            let recording = recording.clone();
                            let on_change = on_change.clone();
                            let clear = clear.clone();
                            move |event, window, cx| {
                                if !recording.get().0 {
                                    return;
                                }
                                // Nothing typed here may reach the app: no
                                // dialog closes on this Escape, no ⌘F opens a
                                // search.
                                cx.stop_propagation();

                                let keystroke = &event.keystroke;
                                if keystroke.key == "escape" {
                                    recording.set((false, Modifiers::default()));
                                    window.refresh();
                                    return;
                                }
                                let bare = keystroke.modifiers == Modifiers::default();
                                if bare && matches!(keystroke.key.as_str(), "backspace" | "delete")
                                {
                                    clear(window, cx);
                                    window.refresh();
                                    return;
                                }
                                if let Some(next) = format_shortcut(keystroke) {
                                    recording.set((false, Modifiers::default()));
                                    if let Some(listener) = on_change.as_ref() {
                                        listener(&next, window, cx);
                                    }
                                }
                                window.refresh();
                            }
                        })
                    })
                    .when(clearable, |el| {
                        el.child(
                            div()
                                .id("shortcut-clear")
                                .absolute()
                                .right(vars.spacing_2)
                                .flex()
                                .items_center()
                                .cursor_pointer()
                                .text_color(vars.color_content_faint)
                                .bg(colors::TRANSPARENT)
                                .child(
                                    Icon::new(icons::DISMISS_CIRCLE_12_FILLED).size(vars.spacing_3),
                                )
                                .on_click(move |_, window, cx| clear(window, cx)),
                        )
                    })
                    .into_any_element()
            })
            .into_any_element()
        })
    }
}
