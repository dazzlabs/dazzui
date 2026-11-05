use crate::{Size, Theme, Tint, Variant, styles};
use nsh_core::{
    element::{KeyOptions, on_key_with},
    keymap::{Key, Keystroke, When},
    prelude::*,
};
#[derive(Clone, PartialEq, Props)]
pub struct ShortcutRecorderProps {
    pub value: Prop<SharedString>,
    #[prop(default)]
    pub disabled: Prop<bool>,
    #[prop(default)]
    pub error: Prop<bool>,
    pub accessibility_label: Prop<Option<SharedString>>,
}
#[derive(Clone, PartialEq, Event)]
pub enum ShortcutRecorderEvent {
    #[args(value)]
    Change(SharedString),
    Clear,
}
pub fn format_shortcut(stroke: Keystroke) -> Option<String> {
    let m = stroke.modifiers;
    if !m.ctrl && !m.alt && !m.cmd && !matches!(stroke.key, Key::F(_)) {
        return None;
    }
    let key = match stroke.key {
        Key::Char(' ') => "Space".into(),
        Key::Char(c) => c.to_ascii_uppercase().to_string(),
        Key::F(n) => format!("F{n}"),
        Key::Enter => "↩".into(),
        Key::Tab => "⇥".into(),
        Key::Backspace => "⌫".into(),
        Key::Delete => "⌦".into(),
        Key::Up => "↑".into(),
        Key::Down => "↓".into(),
        Key::Left => "←".into(),
        Key::Right => "→".into(),
        Key::Home => "↖".into(),
        Key::End => "↘".into(),
        Key::PageUp => "⇞".into(),
        Key::PageDown => "⇟".into(),
        _ => return None,
    };
    let modifiers = [(m.ctrl, "⌃"), (m.alt, "⌥"), (m.shift, "⇧"), (m.cmd, "⌘")]
        .into_iter()
        .filter_map(|(on, g)| on.then_some(g))
        .collect::<String>();
    let gap = if !modifiers.is_empty() && key.chars().count() > 1 {
        " "
    } else {
        ""
    };
    Some(format!("{modifiers}{gap}{key}"))
}
#[component]
pub fn ShortcutRecorder(
    p: ShortcutRecorderProps,
    cx: Scope<ShortcutRecorderEvent>,
) -> impl IntoElement {
    let theme = Theme::current();
    let recording = signal(false);
    let rejected = signal(false);
    let editor = element_ref();
    editor.on_focus_out(move || recording.set(false));
    effect(move || {
        if !recording.get() || p.disabled.get() {
            return;
        }
        let keys = (b'a'..=b'z')
            .chain(b'0'..=b'9')
            .map(|c| (c as char).to_string())
            .chain(
                [
                    "space",
                    "enter",
                    "tab",
                    "backspace",
                    "delete",
                    "up",
                    "down",
                    "left",
                    "right",
                    "home",
                    "end",
                    "pageup",
                    "pagedown",
                    "escape",
                    "-",
                    "=",
                    "[",
                    "]",
                    "\\",
                    ";",
                    "'",
                    ",",
                    ".",
                    "/",
                    "`",
                ]
                .into_iter()
                .map(str::to_owned),
            )
            .chain((1..=20).map(|n| format!("f{n}")))
            .collect::<Vec<_>>();
        for mask in 0..16 {
            for key in &keys {
                let chord = [
                    (mask & 1 != 0, "ctrl+"),
                    (mask & 2 != 0, "alt+"),
                    (mask & 4 != 0, "shift+"),
                    (mask & 8 != 0, "meta+"),
                ]
                .into_iter()
                .filter_map(|(on, s)| on.then_some(s))
                .collect::<String>()
                    + key;
                let Ok(stroke) = Keystroke::parse_with(&chord, nsh_core::element::apple_keys())
                else {
                    continue;
                };
                if stroke.reserved(nsh_core::element::os()).is_some() {
                    continue;
                }
                on_key_with(
                    &chord,
                    KeyOptions {
                        when: When::Always,
                        repeat: false,
                    },
                    move |_| {
                        if mask == 0 && stroke.key == Key::Escape {
                            recording.set(false);
                            return;
                        }
                        if mask == 0 && matches!(stroke.key, Key::Backspace | Key::Delete) {
                            recording.set(false);
                            cx.emit(ShortcutRecorderEvent::Clear);
                            return;
                        }
                        if let Some(next) = format_shortcut(stroke) {
                            recording.set(false);
                            cx.emit(ShortcutRecorderEvent::Change(next.into()));
                        } else {
                            rejected.set(true);
                        }
                    },
                );
            }
        }
    });
    view()
        .relative()
        .flex_row()
        .items_center()
        .child(
            nsh_base::button()
                .disabled(p.disabled)
                .accessibility_label(p.accessibility_label)
                .element_ref(&editor)
                .style_with(move |s| {
                    styles::field(
                        s,
                        theme.tokens(),
                        Size::Medium,
                        if p.error.get() {
                            Tint::Danger
                        } else {
                            Tint::Primary
                        },
                        p.disabled.get(),
                        p.error.get(),
                    )
                    .min_w(px(theme.tokens().shortcut_recorder_width))
                    .px(px(if !p.value.get().is_empty() && !p.disabled.get() {
                        theme.tokens().spacing_8
                    } else {
                        theme.tokens().spacing_3
                    }))
                    .font_weight(if p.value.get().is_empty() {
                        theme.tokens().label_quiet.font_weight()
                    } else {
                        theme.tokens().label_strong.font_weight()
                    })
                })
                .on_press(move || {
                    editor.focus();
                    rejected.set(false);
                    recording.set(true);
                })
                .child(text(move || {
                    if recording.get() {
                        if rejected.get() {
                            "Use Ctrl, Alt or Command…".into()
                        } else {
                            "Press keys…".into()
                        }
                    } else if p.value.get().is_empty() {
                        "Record shortcut".into()
                    } else {
                        p.value.get()
                    }
                })),
        )
        .child(dynamic(move || {
            (!recording.get() && !p.value.get().is_empty() && !p.disabled.get()).then(|| {
                crate::button()
                    .variant(Variant::Plain)
                    .size(Size::Tiny)
                    .icon_only(true)
                    .absolute()
                    .right(move || px(theme.tokens().spacing_2))
                    .top(move || {
                        px((theme.tokens().control_medium_size - theme.tokens().icon_small) / 2.0)
                    })
                    .w(move || px(theme.tokens().icon_small))
                    .h(move || px(theme.tokens().icon_small))
                    .accessibility_label("Clear shortcut")
                    .on_press(move || cx.emit(ShortcutRecorderEvent::Clear))
                    .child(crate::icon("DismissCircle12Filled"))
            })
        }))
}
