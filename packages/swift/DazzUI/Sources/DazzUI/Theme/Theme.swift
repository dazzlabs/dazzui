import SwiftUI

/// The active theme: which one it is, and the variables it resolved to.
public struct Theme: Sendable {
  public let name: ThemeName
  public let vars: ThemeVariables
  public let sizing: ThemeSizing

  public init(_ name: ThemeName) {
    self.name = name
    self.vars = name.variables
    self.sizing = .design
  }

  @MainActor public init(_ name: ThemeName, sizing: ThemeSizing) {
    self.init(name, sizing: sizing, metrics: SystemSizingMetrics.read())
  }

  internal init(_ name: ThemeName, sizing: ThemeSizing, metrics: SystemSizingMetrics) {
    self.name = name
    self.sizing = sizing
    self.vars = sizing == .system ? name.variables.resolvingSystemSizing(metrics) : name.variables
  }

  /// The default is Bright Light, the same theme `:root` holds.
  public static let `default` = Theme(.brightLight)
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

  /// Follows macOS typography and control metrics independently from color.
  @MainActor func theme(_ name: ThemeName, sizing: ThemeSizing) -> some View {
    modifier(SystemThemeModifier(name: name, sizing: sizing))
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
