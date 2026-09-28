import Foundation
import FluentIcons
import SwiftUI

#if os(macOS)
import AppKit
#endif

// A shortcut is kept as the glyph string a settings page prints — `⌥⇧2`,
// `⌥ Space` — rather than as a key-code tuple, because that is what every
// other place showing a shortcut already carries. A host that stores
// `Option+Shift+2` is one mapping away.

/// Modifiers in the order macOS prints them: ⌃ ⌥ ⇧ ⌘.
private let modifierKeys: Set<String> = ["Control", "Alt", "Shift", "Meta"]

/// Keys named by their physical position (the web `code`), not by the
/// character they produce: with ⌥ held, the character for the T key is `†` on
/// a US layout, and the recorded shortcut would print as gibberish.
private let keyGlyphs: [String: String] = [
  "Space": "Space",
  "Enter": "↩",
  "NumpadEnter": "⌤",
  "Tab": "⇥",
  "Backspace": "⌫",
  "Delete": "⌦",
  "ArrowUp": "↑",
  "ArrowDown": "↓",
  "ArrowLeft": "←",
  "ArrowRight": "→",
  "Home": "↖",
  "End": "↘",
  "PageUp": "⇞",
  "PageDown": "⇟",
  "Minus": "-",
  "Equal": "=",
  "BracketLeft": "[",
  "BracketRight": "]",
  "Backslash": "\\",
  "Semicolon": ";",
  "Quote": "'",
  "Comma": ",",
  "Period": ".",
  "Slash": "/",
  "Backquote": "`",
]

/// A key press, in the shape the web event carries it: a physical position, a
/// produced character, and the four modifier flags.
public struct ShortcutEvent: Sendable, Equatable {
  /// The physical key, named as the web names it — `KeyT`, `Digit2`, `Space`.
  public var code: String
  /// The character or key name the press produced.
  public var key: String
  public var ctrlKey: Bool
  public var altKey: Bool
  public var shiftKey: Bool
  public var metaKey: Bool

  public init(
    code: String = "",
    key: String = "",
    ctrlKey: Bool = false,
    altKey: Bool = false,
    shiftKey: Bool = false,
    metaKey: Bool = false
  ) {
    self.code = code
    self.key = key
    self.ctrlKey = ctrlKey
    self.altKey = altKey
    self.shiftKey = shiftKey
    self.metaKey = metaKey
  }
}

private func matches(_ value: String, _ pattern: String) -> Bool {
  value.range(of: pattern, options: .regularExpression) != nil
}

private func keyGlyph(_ code: String, _ key: String) -> String? {
  if let glyph = keyGlyphs[code] { return glyph }
  if matches(code, "^Key[A-Z]$") { return String(code.dropFirst(3)) }
  if matches(code, "^(Digit|Numpad)\\d$") { return String(code.suffix(1)) }
  if matches(code, "^F([1-9]|1\\d|20)$") { return code }

  // No position — a synthetic event, an on-screen keyboard — so fall back to
  // the character, which is only trustworthy while it is plain ASCII.
  if code.isEmpty {
    if key == " " { return "Space" }
    if let glyph = keyGlyphs[key] { return glyph }
    if matches(key, "^F([1-9]|1\\d|20)$") { return key }
    if matches(key, "^[\\x21-\\x7e]$") { return key.uppercased() }
  }
  return nil
}

private func modifierGlyphs(_ event: ShortcutEvent) -> String {
  var glyphs = ""
  if event.ctrlKey { glyphs += "⌃" }
  if event.altKey { glyphs += "⌥" }
  if event.shiftKey { glyphs += "⇧" }
  if event.metaKey { glyphs += "⌘" }
  return glyphs
}

/// The glyph string for a key event, or `nil` when the event is not something
/// a shortcut can be made of.
///
/// A key on its own is typing, not a shortcut, so it has to carry ⌃, ⌥ or ⌘ —
/// ⇧ alone is still typing — unless it is a function key, which is a shortcut
/// by itself. Word keys (`Space`, `F5`) get a space after the modifiers so
/// `⌥ Space` reads as two things and `⌥⇧2` as one.
public func formatShortcut(_ event: ShortcutEvent) -> String? {
  guard let key = keyGlyph(event.code, event.key) else { return nil }

  let isFunctionKey = matches(key, "^F\\d+$")
  if !isFunctionKey && !event.ctrlKey && !event.altKey && !event.metaKey {
    return nil
  }

  let mods = modifierGlyphs(event)
  if key.count > 1 {
    return "\(mods)\(mods.isEmpty ? "" : " ")\(key)"
  }
  return "\(mods)\(key)"
}

/// `error` for a binding the page cannot accept — one already taken.
public enum ShortcutRecorderState: String, CaseIterable, Sendable {
  case `default`
  case error
}

/// The control a settings row uses to rebind a shortcut. Click it and it
/// listens; press a combination and it keeps it. Escape backs out, ⌫ clears,
/// and clicking away cancels.
///
/// Held modifiers show live while it listens, so pressing ⌥⇧ and then
/// wondering which key to add is not done blind.
///
/// Drawn as a field, not a key cap: a ``KeyCap`` says "this is what the key is",
/// a field says "you can change this". Like every field it is a recessed card
/// that lifts to the paper while recording, behind the accent wash.
public struct ShortcutRecorder: View {
  /// The bound keys as printed — `⌥⇧2`, `⌥ Space`. Empty means unbound.
  public var value: String
  public var onValueChange: (String) -> Void
  /// Called on clear or ⌫ while recording; defaults to `onValueChange("")`.
  public var onClear: (() -> Void)?
  /// Shown while idle and unbound.
  public var placeholder: String
  /// Shown while the control waits for a key.
  public var recordingLabel: String
  /// Accessible name of the clear button.
  public var clearLabel: String
  public var state: ShortcutRecorderState
  public var disabled: Bool
  public var tint: Tint
  /// Accessible name of the recorder itself.
  public var accessibilityLabel: String?

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @FocusState private var focused: Bool

  @State private var recording = false
  /// Modifiers held right now, printed while recording.
  @State private var held = ""
  /// Set for a beat after a press the recorder could not take.
  @State private var rejected = false
  #if os(macOS)
  @State private var monitor = ShortcutMonitor()
  #endif

  public init(
    value: String,
    onValueChange: @escaping (String) -> Void,
    onClear: (() -> Void)? = nil,
    placeholder: String = "Record shortcut",
    recordingLabel: String = "Press keys…",
    clearLabel: String = "Clear shortcut",
    state: ShortcutRecorderState = .default,
    disabled: Bool = false,
    tint: Tint = .primary,
    accessibilityLabel: String? = nil
  ) {
    self.value = value
    self.onValueChange = onValueChange
    self.onClear = onClear
    self.placeholder = placeholder
    self.recordingLabel = recordingLabel
    self.clearLabel = clearLabel
    self.state = state
    self.disabled = disabled
    self.tint = tint
    self.accessibilityLabel = accessibilityLabel
  }

  public var body: some View {
    let vars = theme.vars

    Pressable(
      action: beginRecording,
      enabled: !disabled,
      cornerRadius: vars.controlFieldRadius
    ) { _ in
      Text(label)
        .font(.system(size: vars.labelStrong.size, weight: labelWeight))
        .lineLimit(1)
        .foregroundStyle(content)
        // Room for the clear button, mirrored on the left so the keys stay
        // centred whether or not it is there.
        .padding(.horizontal, clearable ? vars.spacing8 : vars.spacing3)
        .frame(minWidth: vars.shortcutRecorderWidth)
        .frame(height: vars.controlMediumSize)
        .background(surface)
        .clipShape(RoundedRectangle(cornerRadius: vars.controlFieldRadius, style: .circular))
        .border(border, width: hairline, cornerRadius: vars.controlFieldRadius)
        // The glow behind the field: the accent while listening, danger for a
        // press it could not take.
        .focusRing(
          visible: glow != nil,
          color: glow ?? .clear,
          cornerRadius: vars.controlFieldRadius,
          width: vars.focusWidth
        )
        .opacity(disabled ? Self.disabledOpacity : 1)
        .animation(vars.motion, value: recording)
        .animation(vars.motion, value: rejected)
    }
    .overlay(alignment: .trailing) {
      if clearable {
        ClearButton(label: clearLabel, action: clear)
          .padding(.trailing, vars.spacing2)
      }
    }
    .accessibilityLabel(accessibilityLabel ?? "")
    .focused($focused)
    .focusRing(
      visible: focused && !disabled,
      color: vars.focusRing,
      cornerRadius: vars.controlFieldRadius,
      width: vars.focusWidth
    )
    .onChange(of: focused) { isFocused in
      // Clicking away cancels, the way the blur handler does.
      if !isFocused { stop() }
    }
    .task(id: rejected) {
      guard rejected else { return }
      try? await Task.sleep(nanoseconds: UInt64(Self.rejectFlash * 1_000_000_000))
      rejected = false
    }
  }

  /// How long a refused press flashes for. A beat, not a token — nothing else
  /// in the system times a flash.
  private static var rejectFlash: Double { 0.36 }

  /// The stylesheet dims a disabled recorder rather than flattening it: the
  /// stored binding still has to be readable.
  private static var disabledOpacity: CGFloat { 0.6 }

  private var bound: Bool { value != "" }

  private var label: String {
    if recording { return held.isEmpty ? recordingLabel : held }
    return bound ? value : placeholder
  }

  /// Unbound, or waiting with nothing held: the label is a prompt, not a value.
  private var showsGlyphs: Bool {
    recording ? !held.isEmpty : bound
  }

  private var clearable: Bool { bound && !recording && !disabled }

  private var labelWeight: Font.Weight {
    showsGlyphs ? theme.vars.labelStrong.weight : theme.vars.labelQuiet.weight
  }

  private var content: Color {
    let vars = theme.vars
    if disabled { return vars.colorContentFaint }
    if state == .error { return vars.colorDanger[800] }
    if !showsGlyphs { return vars.colorContentFaint }
    return vars.colorContent
  }

  private var surface: Color {
    let vars = theme.vars
    if state == .error { return vars.colorDanger[50] }
    return recording ? vars.colorSurface : vars.colorSurfaceMuted
  }

  /// A press the recorder could not take flashes a danger ring rather than
  /// beeping — the resting fill stays, because the flash is about the press,
  /// not the stored value. A *stored* conflict repaints the whole chip.
  private var border: Color {
    let vars = theme.vars
    if state == .error || rejected { return vars.colorDanger[600] }
    return recording ? vars.ramp(tint)[600] : vars.colorBorderStrong
  }

  private var glow: Color? {
    let vars = theme.vars
    if rejected { return vars.colorDanger[50] }
    if recording { return vars.ramp(tint)[600].opacity(vars.focusGlowAlpha) }
    return nil
  }

  private func beginRecording() {
    guard !recording else { return }
    #if os(macOS)
    recording = true
    // The keys have to be caught before the window acts on them: no dialog
    // closes on this Escape, no ⌘F opens a search.
    monitor.start(handle)
    #endif
  }

  private func stop() {
    recording = false
    held = ""
    #if os(macOS)
    monitor.stop()
    #endif
  }

  private func clear() {
    stop()
    if let onClear {
      onClear()
    } else {
      onValueChange("")
    }
  }

  #if os(macOS)
  /// Returns true when the event was consumed and must not reach the window.
  private func handle(_ event: NSEvent) -> Bool {
    guard recording else { return false }

    let shortcut = shortcutEvent(event)
    if event.type == .flagsChanged {
      held = modifierGlyphs(shortcut)
      return true
    }

    if shortcut.key == "Escape" {
      stop()
      return true
    }
    if modifierKeys.contains(shortcut.key) {
      held = modifierGlyphs(shortcut)
      return true
    }

    let bare = !shortcut.ctrlKey && !shortcut.altKey && !shortcut.metaKey
      && !shortcut.shiftKey
    if bare && (shortcut.key == "Backspace" || shortcut.key == "Delete") {
      clear()
      return true
    }

    guard let next = formatShortcut(shortcut) else {
      rejected = true
      return true
    }
    stop()
    onValueChange(next)
    return true
  }
  #endif
}

/// The ✕ that unbinds a shortcut.
private struct ClearButton: View {
  let label: String
  let action: () -> Void

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  var body: some View {
    let vars = theme.vars

    Pressable(action: action, cornerRadius: vars.radiusNone, help: label) { state in
      Icon(.dismissCircle12Filled, size: vars.labelMedium.size)
        .foregroundStyle(
          state.hovered || state.pressed ? vars.colorContentSubtle : vars.colorContentFaint
        )
        .animation(vars.motion, value: state.controlState)
    }
    .accessibilityLabel(label)
    .focused($focused)
    .focusRing(
      visible: focused,
      color: vars.focusRing,
      cornerRadius: vars.radiusNone,
      width: vars.focusWidth
    )
  }
}

#if os(macOS)
/// Holds the local event monitor for as long as the recorder is listening.
///
/// A local monitor is the AppKit half of the React handler's
/// `preventDefault` + `stopPropagation`: returning nil swallows the event
/// before the window's key equivalents see it.
private final class ShortcutMonitor {
  private var token: Any?

  func start(_ handle: @escaping (NSEvent) -> Bool) {
    stop()
    token = NSEvent.addLocalMonitorForEvents(matching: [.keyDown, .flagsChanged]) { event in
      handle(event) ? nil : event
    }
  }

  func stop() {
    if let token { NSEvent.removeMonitor(token) }
    token = nil
  }

  deinit { stop() }
}

/// The physical key each AppKit virtual key code names, spelled the way the
/// web spells it — because that is the vocabulary `keyGlyphs` is written in.
private let virtualKeyCodes: [UInt16: String] = [
  0: "KeyA", 1: "KeyS", 2: "KeyD", 3: "KeyF", 4: "KeyH", 5: "KeyG",
  6: "KeyZ", 7: "KeyX", 8: "KeyC", 9: "KeyV", 11: "KeyB", 12: "KeyQ",
  13: "KeyW", 14: "KeyE", 15: "KeyR", 16: "KeyY", 17: "KeyT", 31: "KeyO",
  32: "KeyU", 34: "KeyI", 35: "KeyP", 37: "KeyL", 38: "KeyJ", 40: "KeyK",
  45: "KeyN", 46: "KeyM",
  18: "Digit1", 19: "Digit2", 20: "Digit3", 21: "Digit4", 22: "Digit6",
  23: "Digit5", 25: "Digit9", 26: "Digit7", 28: "Digit8", 29: "Digit0",
  24: "Equal", 27: "Minus", 30: "BracketRight", 33: "BracketLeft",
  39: "Quote", 41: "Semicolon", 42: "Backslash", 43: "Comma", 44: "Slash",
  47: "Period", 50: "Backquote",
  36: "Enter", 48: "Tab", 49: "Space", 51: "Backspace", 53: "Escape",
  76: "NumpadEnter",
  82: "Numpad0", 83: "Numpad1", 84: "Numpad2", 85: "Numpad3", 86: "Numpad4",
  87: "Numpad5", 88: "Numpad6", 89: "Numpad7", 91: "Numpad8", 92: "Numpad9",
  115: "Home", 116: "PageUp", 117: "Delete", 119: "End", 121: "PageDown",
  123: "ArrowLeft", 124: "ArrowRight", 125: "ArrowDown", 126: "ArrowUp",
  122: "F1", 120: "F2", 99: "F3", 118: "F4", 96: "F5", 97: "F6", 98: "F7",
  100: "F8", 101: "F9", 109: "F10", 103: "F11", 111: "F12", 105: "F13",
  107: "F14", 113: "F15",
]

/// The key *name* the web reports for the positions the recorder branches on.
/// Everything else falls through to the character the key produces, which is
/// only ever read when there is no position at all.
private let domKeyNames: [String: String] = [
  "Escape": "Escape",
  "Backspace": "Backspace",
  "Delete": "Delete",
  "Enter": "Enter",
  "NumpadEnter": "Enter",
  "Tab": "Tab",
  "Space": " ",
  "ArrowUp": "ArrowUp",
  "ArrowDown": "ArrowDown",
  "ArrowLeft": "ArrowLeft",
  "ArrowRight": "ArrowRight",
  "Home": "Home",
  "End": "End",
  "PageUp": "PageUp",
  "PageDown": "PageDown",
]

private func shortcutEvent(_ event: NSEvent) -> ShortcutEvent {
  let flags = event.modifierFlags
  let code = virtualKeyCodes[event.keyCode] ?? ""
  let key = domKeyNames[code]
    ?? (event.type == .keyDown ? event.charactersIgnoringModifiers ?? "" : "")

  return ShortcutEvent(
    code: code,
    key: key,
    ctrlKey: flags.contains(.control),
    altKey: flags.contains(.option),
    shiftKey: flags.contains(.shift),
    metaKey: flags.contains(.command)
  )
}
#endif
