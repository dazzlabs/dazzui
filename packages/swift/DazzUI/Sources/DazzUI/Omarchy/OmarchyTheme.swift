import Foundation
import SwiftUI

private struct OmarchyPaletteKey: EnvironmentKey {
  static let defaultValue: OmarchyPalette? = nil
}

public extension EnvironmentValues {
  /// The palette the nearest ``OmarchyTheme`` is drawing, as Omarchy ships it,
  /// or `nil` when it is drawing its fallback.
  var omarchyPalette: OmarchyPalette? {
    get { self[OmarchyPaletteKey.self] }
    set { self[OmarchyPaletteKey.self] = newValue }
  }
}

/// Draws `content` in the theme the Omarchy machine it runs on is wearing, and
/// follows it when `omarchy theme set` or `shell.toml` changes it.
///
/// Off Omarchy — another desktop, macOS, the web — `content` is drawn in
/// `fallback`.
public struct OmarchyTheme<Content: View>: View {
  public let fallback: Theme
  public let colorScheme: ColorScheme?
  public let directory: String?
  public let config: String?
  private let content: Content

  public init(
    fallback: Theme = .default,
    colorScheme: ColorScheme? = nil,
    directory: String? = nil,
    config: String? = nil,
    @ViewBuilder content: () -> Content
  ) {
    self.fallback = fallback
    self.colorScheme = colorScheme
    self.directory = directory
    self.config = config
    self.content = content()
  }

  public init(
    fallback: ThemeName,
    colorScheme: ColorScheme? = nil,
    directory: String? = nil,
    config: String? = nil,
    @ViewBuilder content: () -> Content
  ) {
    self.init(
      fallback: Theme(fallback),
      colorScheme: colorScheme,
      directory: directory,
      config: config,
      content: content
    )
  }

  public var body: some View {
    content.modifier(
      OmarchyThemeModifier(
        fallback: fallback,
        colorScheme: colorScheme,
        directory: directory,
        config: config
      )
    )
  }
}

private struct OmarchyWatchSource: Equatable {
  let directory: String?
  let config: String?
}

private struct OmarchyThemeModifier: ViewModifier {
  let fallback: Theme
  let colorScheme: ColorScheme?
  let directory: String?
  let config: String?

  @State private var palette: OmarchyPalette?

  private var source: OmarchyWatchSource {
    OmarchyWatchSource(directory: directory, config: config)
  }

  private var resolved: Theme {
    guard let palette else { return fallback }
    let wanted = colorScheme ?? palette.colorScheme
    let half = wanted == palette.colorScheme ? palette : palette.opposite
    return half.theme
  }

  func body(content: Content) -> some View {
    let active = resolved
    content
      .environment(\.omarchyPalette, palette)
      .environment(\.theme, active)
      .font(active.vars.bodyMedium.font)
      .environment(\.colorScheme, active.colorScheme)
      .onAppear {
        palette = readOmarchyPalette(directory: directory, config: config)
      }
      .onChange(of: source) { next in
        palette = readOmarchyPalette(directory: next.directory, config: next.config)
      }
      .task(id: source) {
        for await next in watchOmarchyPalette(directory: directory, config: config) {
          if Task.isCancelled { break }
          palette = next
        }
      }
  }
}

public extension View {
  /// Scopes the machine's Omarchy theme to this subtree, following theme
  /// switches and `shell.toml` edits, and falling back to `fallback` off
  /// Omarchy.
  func omarchyTheme(
    fallback: Theme = .default,
    colorScheme: ColorScheme? = nil,
    directory: String? = nil,
    config: String? = nil
  ) -> some View {
    modifier(
      OmarchyThemeModifier(
        fallback: fallback,
        colorScheme: colorScheme,
        directory: directory,
        config: config
      )
    )
  }

  /// Scopes the machine's Omarchy theme to this subtree, falling back to the
  /// named theme off Omarchy.
  func omarchyTheme(
    fallback: ThemeName,
    colorScheme: ColorScheme? = nil,
    directory: String? = nil,
    config: String? = nil
  ) -> some View {
    omarchyTheme(
      fallback: Theme(fallback),
      colorScheme: colorScheme,
      directory: directory,
      config: config
    )
  }
}
