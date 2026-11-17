import Foundation
import SwiftUI

/// An Omarchy theme, as the three colours DazzUI reads it off: the background,
/// the foreground and the accent of its `colors.toml`, in the one appearance
/// it ships in.
///
/// The generated `omarchy-*` themes are the built-in ones, derived at build
/// time; this is the same derivation at run time, so a machine wearing a theme
/// DazzUI has never seen — one the user installed or wrote — is drawn in it
/// too. Read the machine's current one with ``readOmarchyPalette(directory:config:)``,
/// or follow it with ``watchOmarchyPalette(directory:config:)`` or ``OmarchyTheme``.
public struct OmarchyPalette: Sendable, Hashable, CustomStringConvertible {
  public let background: Color
  public let foreground: Color
  public let accent: Color

  /// The appearance the theme ships in.
  public let colorScheme: ColorScheme

  /// The theme's name as Omarchy stores it (`tokyo-night`), when known.
  public let name: String?

  /// The sizes Omarchy's shell sets its text and controls at on this machine.
  public let shellStyle: OmarchyShellStyle

  public init(
    background: Color,
    foreground: Color,
    accent: Color,
    colorScheme: ColorScheme,
    name: String? = nil,
    shellStyle: OmarchyShellStyle = OmarchyShellStyle()
  ) {
    self.background = background
    self.foreground = foreground
    self.accent = accent
    self.colorScheme = colorScheme
    self.name = name
    self.shellStyle = shellStyle
  }

  public init?(
    colorsToml: String,
    name: String? = nil,
    colorScheme: ColorScheme? = nil,
    shellStyle: OmarchyShellStyle = OmarchyShellStyle()
  ) {
    guard
      let parsed = Self.parse(
        colorsToml,
        name: name,
        colorScheme: colorScheme,
        shellStyle: shellStyle
      )
    else { return nil }
    self = parsed
  }

  /// A palette out of a `colors.toml`.
  ///
  /// Only `background`, `foreground` and `accent` are read, with `mode` where
  /// the file states one (`"light"` or `"dark"`). A file without it —
  /// Omarchy's older ones marked a light theme with a `light.mode` file beside
  /// it instead — takes `colorScheme` when given, else the background's own
  /// luminance. Returns `nil` when one of the three colours is missing or is
  /// not a hex colour.
  public static func parse(
    _ colorsToml: String,
    name: String? = nil,
    colorScheme: ColorScheme? = nil,
    shellStyle: OmarchyShellStyle = OmarchyShellStyle()
  ) -> OmarchyPalette? {
    let values = readOmarchyToml(colorsToml)
    guard
      let background = parseHexColor(values["background"]),
      let foreground = parseHexColor(values["foreground"]),
      let accent = parseHexColor(values["accent"])
    else {
      return nil
    }
    let mode: ColorScheme?
    switch values["mode"]?.lowercased() {
    case "light": mode = .light
    case "dark": mode = .dark
    default: mode = nil
    }
    let resolvedScheme =
      mode
      ?? colorScheme
      ?? (omarchyLuminance(background) > 0.5 ? .light : .dark)
    return OmarchyPalette(
      background: background,
      foreground: foreground,
      accent: accent,
      colorScheme: resolvedScheme,
      name: name,
      shellStyle: shellStyle
    )
  }

  /// The same theme in the other appearance, derived from these three colours
  /// the way the generated themes' other halves are. DazzUI's reading of the
  /// theme, not Omarchy's.
  public var opposite: OmarchyPalette {
    let other = oppositeOmarchyColors(
      background: background,
      foreground: foreground,
      accent: accent,
      dark: colorScheme == .dark
    )
    return OmarchyPalette(
      background: other.background,
      foreground: other.foreground,
      accent: other.accent,
      colorScheme: colorScheme == .dark ? .light : .dark,
      name: name,
      shellStyle: shellStyle
    )
  }

  /// Every token, with this palette's colours derived into it, Omarchy's
  /// square geometry, and the shell's sizes.
  public var themeVariables: ThemeVariables {
    shellStyle.apply(
      to: deriveOmarchyThemeVariables(
        background: background,
        foreground: foreground,
        accent: accent,
        dark: colorScheme == .dark
      )
    )
  }

  /// The resolved ``Theme`` value for this palette.
  public var theme: Theme {
    Theme(omarchy: self)
  }

  /// The name the way `omarchy theme current` prints it: `Tokyo Night`.
  public var title: String? {
    name?
      .split(separator: "-")
      .filter { !$0.isEmpty }
      .map { $0.prefix(1).uppercased() + $0.dropFirst() }
      .joined(separator: " ")
  }

  public var description: String {
    let scheme = colorScheme == .dark ? "dark" : "light"
    return "OmarchyPalette(\(name ?? "unnamed"), \(scheme), "
      + "background: \(background), foreground: \(foreground), accent: \(accent), "
      + "\(shellStyle))"
  }
}

private func parseHexColor(_ value: String?) -> Color? {
  guard var raw = value?.trimmingCharacters(in: .whitespacesAndNewlines), !raw.isEmpty else {
    return nil
  }
  if raw.hasPrefix("#") { raw.removeFirst() }
  guard raw.count == 6 || raw.count == 8,
    raw.allSatisfy({ $0.isASCII && $0.isHexDigit })
  else {
    return nil
  }
  guard let rgb = UInt32(raw.prefix(6), radix: 16) else { return nil }
  let alpha: UInt32
  if raw.count == 8 {
    guard let parsedAlpha = UInt32(raw.suffix(2), radix: 16) else { return nil }
    alpha = parsedAlpha
  } else {
    alpha = 0xFF
  }
  return Color(argb: (alpha << 24) | rgb)
}
