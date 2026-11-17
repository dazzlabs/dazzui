import Foundation
import SwiftUI

/// The active theme: which one it is, and the variables it resolved to.
///
/// Equal themes share one resolved token table: a design theme is resolved
/// once per name, a system-sized one once per name and metrics snapshot. So a
/// `Theme(name)` rebuilt on every redraw hands the environment the same value
/// as before, and the subtree reading it is not invalidated.
public struct Theme: Sendable, Equatable {
  public let name: ThemeName?
  public let colorScheme: ColorScheme
  public let sizing: ThemeSizing
  private let resolved: ResolvedTheme

  // Each themed view captures the environment. Share the token table across
  // those captures, including the resolved system-sizing snapshot.
  public var vars: ThemeVariables { resolved.variables }

  public init(_ name: ThemeName) {
    self.name = name
    self.colorScheme = name.colorScheme
    self.sizing = .design
    self.resolved = ResolvedTheme.cache.design(name)
  }

  @MainActor public init(_ name: ThemeName, sizing: ThemeSizing) {
    self.init(name, sizing: sizing, metrics: SystemSizingMetrics.current())
  }

  internal init(_ name: ThemeName, sizing: ThemeSizing, metrics: SystemSizingMetrics) {
    self.name = name
    self.colorScheme = name.colorScheme
    self.sizing = sizing
    self.resolved =
      sizing == .system
      ? ResolvedTheme.cache.system(name, metrics) : ResolvedTheme.cache.design(name)
  }

  public init(vars: ThemeVariables, colorScheme: ColorScheme) {
    self.name = nil
    self.colorScheme = colorScheme
    self.sizing = .design
    self.resolved = ResolvedTheme(vars, metrics: nil, omarchy: nil)
  }

  internal init(omarchy palette: OmarchyPalette) {
    self.name = nil
    self.colorScheme = palette.colorScheme
    self.sizing = .design
    self.resolved = ResolvedTheme.cache.omarchy(palette)
  }

  public static func == (lhs: Theme, rhs: Theme) -> Bool {
    if lhs.resolved === rhs.resolved { return lhs.colorScheme == rhs.colorScheme }
    if let omarchy = lhs.resolved.omarchy {
      return omarchy == rhs.resolved.omarchy && lhs.colorScheme == rhs.colorScheme
    }
    guard let name = lhs.name, name == rhs.name else { return false }
    return lhs.sizing == rhs.sizing && lhs.resolved.metrics == rhs.resolved.metrics
  }

  /// The default is Bright Light, the same theme `:root` holds.
  public static let `default` = Theme(.brightLight)
}

private final class ResolvedTheme: Sendable {
  let variables: ThemeVariables
  /// The snapshot a system-sized table was resolved against; nil for design.
  let metrics: SystemSizingMetrics?
  /// The runtime Omarchy palette this table was derived from, if any.
  let omarchy: OmarchyPalette?

  init(_ variables: ThemeVariables, metrics: SystemSizingMetrics?, omarchy: OmarchyPalette? = nil) {
    self.variables = variables
    self.metrics = metrics
    self.omarchy = omarchy
  }

  static let cache = ResolvedThemeCache()
}

/// Resolved tables, shared by every `Theme` that resolves to the same one.
private final class ResolvedThemeCache: @unchecked Sendable {
  private struct SystemKey: Hashable {
    let name: ThemeName
    let metrics: SystemSizingMetrics
  }

  /// Metrics change when the reader changes their text size, so only a
  /// handful of snapshots are ever live; the bound only stops a test sweeping
  /// scales from growing it without end.
  private static let systemLimit = 64
  private static let omarchyLimit = 64

  private let lock = NSLock()
  private var designTables: [ThemeName: ResolvedTheme] = [:]
  private var systemTables: [SystemKey: ResolvedTheme] = [:]
  private var omarchyTables: [OmarchyPalette: ResolvedTheme] = [:]

  func design(_ name: ThemeName) -> ResolvedTheme {
    lock.lock()
    if let hit = designTables[name] {
      lock.unlock()
      return hit
    }
    lock.unlock()
    let made = ResolvedTheme(name.variables, metrics: nil)
    lock.lock()
    defer { lock.unlock() }
    if let raced = designTables[name] { return raced }
    designTables[name] = made
    return made
  }

  func system(_ name: ThemeName, _ metrics: SystemSizingMetrics) -> ResolvedTheme {
    let key = SystemKey(name: name, metrics: metrics)
    lock.lock()
    if let hit = systemTables[key] {
      lock.unlock()
      return hit
    }
    lock.unlock()
    let made = ResolvedTheme(name.variables.resolvingSystemSizing(metrics), metrics: metrics)
    lock.lock()
    defer { lock.unlock() }
    if let raced = systemTables[key] { return raced }
    if systemTables.count >= Self.systemLimit { systemTables.removeAll() }
    systemTables[key] = made
    return made
  }

  func omarchy(_ palette: OmarchyPalette) -> ResolvedTheme {
    lock.lock()
    if let hit = omarchyTables[palette] {
      lock.unlock()
      return hit
    }
    lock.unlock()
    let made = ResolvedTheme(palette.themeVariables, metrics: nil, omarchy: palette)
    lock.lock()
    defer { lock.unlock() }
    if let raced = omarchyTables[palette] { return raced }
    if omarchyTables.count >= Self.omarchyLimit { omarchyTables.removeAll() }
    omarchyTables[palette] = made
    return made
  }
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

  /// Scopes an already-resolved theme to a subtree.
  func theme(_ theme: Theme) -> some View {
    environment(\.theme, theme)
      .environment(\.colorScheme, theme.colorScheme)
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
