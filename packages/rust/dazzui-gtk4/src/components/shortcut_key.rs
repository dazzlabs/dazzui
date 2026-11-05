//! The recorder stores the same display string as React. Positions are
//! independent of the current layout and of Option/Alt-produced characters.
use gtk::gdk::{Key, ModifierType as M};

pub(super) fn modifiers(state: M) -> String {
    [
        (state.contains(M::CONTROL_MASK), '⌃'),
        (state.contains(M::ALT_MASK), '⌥'),
        (state.contains(M::SHIFT_MASK), '⇧'),
        (state.intersects(M::META_MASK | M::SUPER_MASK), '⌘'),
    ]
    .into_iter()
    .filter(|(held, _)| *held)
    .map(|(_, glyph)| glyph)
    .collect()
}
fn function(key: &str) -> bool {
    key.strip_prefix('F').is_some_and(|n| {
        n.parse::<u8>()
            .is_ok_and(|n| (1..=20).contains(&n) && key == format!("F{n}"))
    })
}
fn named_glyph(code: &str) -> Option<&str> {
    Some(match code {
        "Space" => "Space",
        "Enter" => "↩",
        "NumpadEnter" => "⌤",
        "Tab" => "⇥",
        "Backspace" => "⌫",
        "Delete" => "⌦",
        "ArrowUp" => "↑",
        "ArrowDown" => "↓",
        "ArrowLeft" => "←",
        "ArrowRight" => "→",
        "Home" => "↖",
        "End" => "↘",
        "PageUp" => "⇞",
        "PageDown" => "⇟",
        "Minus" => "-",
        "Equal" => "=",
        "BracketLeft" => "[",
        "BracketRight" => "]",
        "Backslash" => "\\",
        "Semicolon" => ";",
        "Quote" => "'",
        "Comma" => ",",
        "Period" => ".",
        "Slash" => "/",
        "Backquote" => "`",
        _ => return None,
    })
}
fn glyph(code: &str) -> Option<&str> {
    named_glyph(code).or_else(|| {
        if code
            .strip_prefix("Key")
            .is_some_and(|k| k.len() == 1 && k.as_bytes()[0].is_ascii_uppercase())
        {
            Some(&code[3..])
        } else if code
            .strip_prefix("Digit")
            .or_else(|| code.strip_prefix("Numpad"))
            .is_some_and(|k| k.len() == 1 && k.as_bytes()[0].is_ascii_digit())
        {
            Some(&code[code.len() - 1..])
        } else if function(code) {
            Some(code)
        } else {
            None
        }
    })
}
/// A printed shortcut from a web-style physical key code and key. An empty
/// code permits the same ASCII-only fallback as React's synthetic events.
/// Bare typing, Shift-only typing and unsupported positions return None.
pub fn format_shortcut(code: &str, key: &str, state: M) -> Option<String> {
    let key = if code.is_empty() {
        if key == " " {
            "Space".to_owned()
        } else if let Some(glyph) = named_glyph(key) {
            glyph.to_owned()
        } else if function(key) {
            key.to_owned()
        } else if key.len() == 1 && key.as_bytes()[0].is_ascii_graphic() {
            key.to_ascii_uppercase()
        } else {
            return None;
        }
    } else {
        glyph(code)?.to_owned()
    };
    if !function(&key)
        && !state.intersects(M::CONTROL_MASK | M::ALT_MASK | M::META_MASK | M::SUPER_MASK)
    {
        return None;
    }
    let mods = modifiers(state);
    Some(format!(
        "{mods}{}{key}",
        if !mods.is_empty() && key.chars().count() > 1 {
            " "
        } else {
            ""
        }
    ))
}
pub(super) fn modifier_key(key: Key) -> Option<M> {
    if [Key::Control_L, Key::Control_R].contains(&key) {
        Some(M::CONTROL_MASK)
    } else if [Key::Alt_L, Key::Alt_R].contains(&key) {
        Some(M::ALT_MASK)
    } else if [Key::Shift_L, Key::Shift_R].contains(&key) {
        Some(M::SHIFT_MASK)
    } else if [Key::Meta_L, Key::Meta_R, Key::Super_L, Key::Super_R].contains(&key) {
        Some(M::META_MASK | M::SUPER_MASK)
    } else {
        None
    }
}
pub(super) fn key_name(key: Key) -> String {
    match key {
        Key::Return | Key::KP_Enter => "Enter".into(),
        Key::Tab | Key::ISO_Left_Tab => "Tab".into(),
        Key::BackSpace => "Backspace".into(),
        Key::Delete | Key::KP_Delete => "Delete".into(),
        Key::Up => "ArrowUp".into(),
        Key::Down => "ArrowDown".into(),
        Key::Left => "ArrowLeft".into(),
        Key::Right => "ArrowRight".into(),
        Key::Home => "Home".into(),
        Key::End => "End".into(),
        Key::Page_Up => "PageUp".into(),
        Key::Page_Down => "PageDown".into(),
        _ => key
            .to_unicode()
            .filter(|c| !c.is_control())
            .map(|c| c.to_string())
            .or_else(|| key.name().map(|s| s.into()))
            .unwrap_or_default(),
    }
}
/// Wayland GDK uses evdev + 8; X11's evdev map uses those same keycodes.
/// macOS GDK carries NSEvent.keyCode (Carbon virtual position), including A=0.
/// These are hardware positions, not design values or layout translations.
pub(super) fn position(backend: &str, keycode: u32) -> Option<String> {
    let code = if backend == "GdkMacosDisplay" {
        macos_position(keycode)
    } else if ["GdkWaylandDisplay", "GdkX11Display"].contains(&backend) {
        evdev_position(keycode.checked_sub(8)?)
    } else {
        None
    };
    code.map(str::to_owned)
}
fn evdev_position(code: u32) -> Option<&'static str> {
    Some(match code {
        2 => "Digit1",
        3 => "Digit2",
        4 => "Digit3",
        5 => "Digit4",
        6 => "Digit5",
        7 => "Digit6",
        8 => "Digit7",
        9 => "Digit8",
        10 => "Digit9",
        11 => "Digit0",
        12 => "Minus",
        13 => "Equal",
        14 => "Backspace",
        15 => "Tab",
        16 => "KeyQ",
        17 => "KeyW",
        18 => "KeyE",
        19 => "KeyR",
        20 => "KeyT",
        21 => "KeyY",
        22 => "KeyU",
        23 => "KeyI",
        24 => "KeyO",
        25 => "KeyP",
        26 => "BracketLeft",
        27 => "BracketRight",
        28 => "Enter",
        30 => "KeyA",
        31 => "KeyS",
        32 => "KeyD",
        33 => "KeyF",
        34 => "KeyG",
        35 => "KeyH",
        36 => "KeyJ",
        37 => "KeyK",
        38 => "KeyL",
        39 => "Semicolon",
        40 => "Quote",
        41 => "Backquote",
        43 => "Backslash",
        44 => "KeyZ",
        45 => "KeyX",
        46 => "KeyC",
        47 => "KeyV",
        48 => "KeyB",
        49 => "KeyN",
        50 => "KeyM",
        51 => "Comma",
        52 => "Period",
        53 => "Slash",
        57 => "Space",
        59 => "F1",
        60 => "F2",
        61 => "F3",
        62 => "F4",
        63 => "F5",
        64 => "F6",
        65 => "F7",
        66 => "F8",
        67 => "F9",
        68 => "F10",
        87 => "F11",
        88 => "F12",
        183 => "F13",
        184 => "F14",
        185 => "F15",
        186 => "F16",
        187 => "F17",
        188 => "F18",
        189 => "F19",
        190 => "F20",
        71 => "Numpad7",
        72 => "Numpad8",
        73 => "Numpad9",
        75 => "Numpad4",
        76 => "Numpad5",
        77 => "Numpad6",
        79 => "Numpad1",
        80 => "Numpad2",
        81 => "Numpad3",
        82 => "Numpad0",
        96 => "NumpadEnter",
        102 => "Home",
        103 => "ArrowUp",
        104 => "PageUp",
        105 => "ArrowLeft",
        106 => "ArrowRight",
        107 => "End",
        108 => "ArrowDown",
        109 => "PageDown",
        111 => "Delete",
        _ => return None,
    })
}
fn macos_position(code: u32) -> Option<&'static str> {
    Some(match code {
        0 => "KeyA",
        1 => "KeyS",
        2 => "KeyD",
        3 => "KeyF",
        4 => "KeyH",
        5 => "KeyG",
        6 => "KeyZ",
        7 => "KeyX",
        8 => "KeyC",
        9 => "KeyV",
        11 => "KeyB",
        12 => "KeyQ",
        13 => "KeyW",
        14 => "KeyE",
        15 => "KeyR",
        16 => "KeyY",
        17 => "KeyT",
        18 => "Digit1",
        19 => "Digit2",
        20 => "Digit3",
        21 => "Digit4",
        22 => "Digit6",
        23 => "Digit5",
        24 => "Equal",
        25 => "Digit9",
        26 => "Digit7",
        27 => "Minus",
        28 => "Digit8",
        29 => "Digit0",
        30 => "BracketRight",
        31 => "KeyO",
        32 => "KeyU",
        33 => "BracketLeft",
        34 => "KeyI",
        35 => "KeyP",
        36 => "Enter",
        37 => "KeyL",
        38 => "KeyJ",
        39 => "Quote",
        40 => "KeyK",
        41 => "Semicolon",
        42 => "Backslash",
        43 => "Comma",
        44 => "Slash",
        45 => "KeyN",
        46 => "KeyM",
        47 => "Period",
        48 => "Tab",
        49 => "Space",
        50 => "Backquote",
        51 => "Backspace",
        76 => "NumpadEnter",
        82 => "Numpad0",
        83 => "Numpad1",
        84 => "Numpad2",
        85 => "Numpad3",
        86 => "Numpad4",
        87 => "Numpad5",
        88 => "Numpad6",
        89 => "Numpad7",
        91 => "Numpad8",
        92 => "Numpad9",
        122 => "F1",
        120 => "F2",
        99 => "F3",
        118 => "F4",
        96 => "F5",
        97 => "F6",
        98 => "F7",
        100 => "F8",
        101 => "F9",
        109 => "F10",
        103 => "F11",
        111 => "F12",
        105 => "F13",
        107 => "F14",
        113 => "F15",
        106 => "F16",
        64 => "F17",
        79 => "F18",
        80 => "F19",
        90 => "F20",
        115 => "Home",
        116 => "PageUp",
        117 => "Delete",
        119 => "End",
        121 => "PageDown",
        123 => "ArrowLeft",
        124 => "ArrowRight",
        125 => "ArrowDown",
        126 => "ArrowUp",
        _ => return None,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_shortcut_format() {
        for (code, key, mods, expected) in [
            ("KeyT", "†", M::ALT_MASK, Some("⌥T")),
            ("Digit2", "@", M::ALT_MASK | M::SHIFT_MASK, Some("⌥⇧2")),
            ("Space", " ", M::ALT_MASK, Some("⌥ Space")),
            ("F5", "F5", M::empty(), Some("F5")),
            ("F20", "F20", M::CONTROL_MASK, Some("⌃ F20")),
            ("NumpadEnter", "Enter", M::CONTROL_MASK, Some("⌃⌤")),
            ("ArrowLeft", "ArrowLeft", M::SUPER_MASK, Some("⌘←")),
            (
                "BracketLeft",
                "{",
                M::CONTROL_MASK | M::ALT_MASK | M::SHIFT_MASK | M::META_MASK,
                Some("⌃⌥⇧⌘["),
            ),
            ("", "t", M::CONTROL_MASK, Some("⌃T")),
            ("", "Enter", M::META_MASK, Some("⌘↩")),
            ("KeyA", "a", M::empty(), None),
            ("KeyA", "A", M::SHIFT_MASK, None),
            ("F21", "F21", M::CONTROL_MASK, None),
            ("F01", "F01", M::CONTROL_MASK, None),
            ("NumpadAdd", "+", M::CONTROL_MASK, None),
            ("", "†", M::ALT_MASK, None),
            ("Unidentified", "a", M::ALT_MASK, None),
            ("", "KeyT", M::ALT_MASK, None),
            ("", "Digit2", M::ALT_MASK, None),
        ] {
            assert_eq!(
                format_shortcut(code, key, mods).as_deref(),
                expected,
                "{code} / {key}"
            );
        }
    }
    #[test]
    fn physical_positions_are_layout_independent() {
        for backend in ["GdkWaylandDisplay", "GdkX11Display"] {
            assert_eq!(position(backend, 28).as_deref(), Some("KeyT"));
            assert_eq!(position(backend, 104).as_deref(), Some("NumpadEnter"));
            assert_eq!(position(backend, 198).as_deref(), Some("F20"));
            assert_eq!(position(backend, 0), None);
        }
        assert_eq!(position("GdkMacosDisplay", 0).as_deref(), Some("KeyA"));
        assert_eq!(position("GdkMacosDisplay", 17).as_deref(), Some("KeyT"));
        assert_eq!(
            position("GdkMacosDisplay", 76).as_deref(),
            Some("NumpadEnter")
        );
        assert_eq!(position("GdkMacosDisplay", 90).as_deref(), Some("F20"));
        assert_eq!(position("UnknownDisplay", 17), None);
    }
}
