import SwiftUI

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
