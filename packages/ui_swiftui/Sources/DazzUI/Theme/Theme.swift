import SwiftUI

/// The twelve themes, as they are named in `terrazzo.config.mjs`.
///
/// Studio Light is the default and carries no attribute in the React package
/// at all — selecting it means removing `data-theme` rather than writing
/// `studio-light`. Here it is simply the case whose variables are the
/// initialiser's own defaults.
public enum ThemeName: String, CaseIterable, Sendable {
  /// White card, light grey canvas, violet accent. **The baseline.**
  case studioLight = "studio-light"
  /// The same skeleton over a near-black canvas.
  case studioDark = "studio-dark"
  /// Warm paper, ink navy, acid-green marker, pill controls.
  case brightLight = "bright-light"
  /// Ink-navy canvas, with acid green as the accent itself.
  case brightDark = "bright-dark"
  /// Slate-grey paper with a teal accent — cool, and not violet.
  case frostLight = "frost-light"
  /// The same teal over a deep blue-slate canvas.
  case frostDark = "frost-dark"
  /// Monochrome: the accent is the ink, so only the status hues carry
  /// colour. The one family that also tightens the corners.
  case graphiteLight = "graphite-light"
  /// The same, inverted — a near-white accent on near-black.
  case graphiteDark = "graphite-dark"
  /// Warm paper with a copper accent.
  case emberLight = "ember-light"
  /// A charcoal canvas with that copper lifted to an amber.
  case emberDark = "ember-dark"
  /// Cool white paper, blue-grey ink and a blurple accent.
  case nocturneLight = "nocturne-light"
  /// A blue-grey near-black, with the same blurple lifted a step.
  case nocturneDark = "nocturne-dark"

  public var variables: ThemeVariables {
    switch self {
    case .studioLight: return .studioLight
    case .studioDark: return .studioDark
    case .brightLight: return .brightLight
    case .brightDark: return .brightDark
    case .frostLight: return .frostLight
    case .frostDark: return .frostDark
    case .graphiteLight: return .graphiteLight
    case .graphiteDark: return .graphiteDark
    case .emberLight: return .emberLight
    case .emberDark: return .emberDark
    case .nocturneLight: return .nocturneLight
    case .nocturneDark: return .nocturneDark
    }
  }

  public var colorScheme: ColorScheme {
    switch self {
    case .studioLight, .brightLight, .frostLight, .graphiteLight, .emberLight,
      .nocturneLight:
      return .light
    case .studioDark, .brightDark, .frostDark, .graphiteDark, .emberDark,
      .nocturneDark:
      return .dark
    }
  }
}

/// The active theme: which one it is, and the variables it resolved to.
public struct Theme: Sendable {
  public let name: ThemeName
  public let vars: ThemeVariables

  public init(_ name: ThemeName) {
    self.name = name
    self.vars = name.variables
  }

  /// The default is Studio Light, the same baseline `:root` holds.
  public static let `default` = Theme(.studioLight)
}

private struct ThemeKey: EnvironmentKey {
  static let defaultValue = Theme.default
}

public extension EnvironmentValues {
  /// The theme this subtree is drawn in.
  var theme: Theme {
    get { self[ThemeKey.self] }
    set { self[ThemeKey.self] = newValue }
  }
}

public extension View {
  /// Scopes a theme to a subtree, the way React's `data-theme` attribute
  /// does. Themes are per-subtree, so several can sit on screen at once.
  func theme(_ name: ThemeName) -> some View {
    environment(\.theme, Theme(name))
      .environment(\.colorScheme, name.colorScheme)
  }
}

/// A separator is one *point*, whatever the display scale — the 1px the
/// token holds, drawn as written.
///
/// It used to halve on Retina, to land on one device pixel; the Nocturne
/// mockups the system is drawn to rule at a full 1px, and against them a
/// halved rule read as too thin to be a decision. Drawn control outlines use
/// `strokeControl` instead, and always did.
public extension EnvironmentValues {
  var hairline: CGFloat {
    theme.vars.strokeHairline
  }
}
