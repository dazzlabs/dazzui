import SwiftUI

/// The four themes, as they are named in `terrazzo.config.mjs`.
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

  public var variables: ThemeVariables {
    switch self {
    case .studioLight: return .studioLight
    case .studioDark: return .studioDark
    case .brightLight: return .brightLight
    case .brightDark: return .brightDark
    }
  }

  public var colorScheme: ColorScheme {
    switch self {
    case .studioLight, .brightLight: return .light
    case .studioDark, .brightDark: return .dark
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

/// A separator is one *device* pixel.
///
/// A 1pt line on a Retina display is twice the weight of the real thing, so
/// the hairline halves — the same rule `base.css` applies with its
/// `min-resolution: 2dppx` block. Drawn control outlines use
/// `strokeControl` instead and stay put.
public extension EnvironmentValues {
  var hairline: CGFloat {
    1 / max(displayScale, 1)
  }
}
