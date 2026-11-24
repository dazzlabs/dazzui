import SwiftUI

/// The appearance mode an app lets a user choose: following the OS, light, or dark.
public enum AppearanceMode: String, CaseIterable, Identifiable, Codable, Hashable, Sendable {
  case system
  case light
  case dark

  public var id: String { rawValue }

  /// The `ColorScheme` override passed to SwiftUI, or `nil` to follow the OS.
  public var colorScheme: ColorScheme? {
    switch self {
    case .system: return nil
    case .light: return .light
    case .dark: return .dark
    }
  }
}

private struct MobilePaletteKey: EnvironmentKey {
  static let defaultValue = Palette.brightLight
}

public extension EnvironmentValues {
  /// The ds palette installed by `mobileTheme`. Controls remain system views.
  var mobilePalette: Palette {
    get { self[MobilePaletteKey.self] }
    set { self[MobilePaletteKey.self] = newValue }
  }
}

private struct MobileThemeModifier: ViewModifier {
  @Environment(\.colorScheme) private var systemColorScheme
  let family: ThemeFamily
  let colorScheme: ColorScheme?

  func body(content: Content) -> some View {
    let palette = family.palette(for: colorScheme ?? systemColorScheme)
    content
      .environment(\.mobilePalette, palette)
      .tint(palette.accent)
      .foregroundStyle(
        palette.colorContent,
        palette.colorContentSecondary,
        palette.colorContentSubtle
      )
      .background(palette.colorSurface.ignoresSafeArea())
      .scrollContentBackground(.hidden)
      .preferredColorScheme(colorScheme)
  }
}

private struct MobileBackgroundModifier: ViewModifier {
  @Environment(\.mobilePalette) private var palette

  func body(content: Content) -> some View {
    content
      .background(palette.colorSurface.ignoresSafeArea())
      .scrollContentBackground(.hidden)
  }
}

public extension View {
  /// Apply ds colours to native controls. By default appearance follows the OS.
  /// Install at a scene's root so sheets and navigation inherit the theme.
  func mobileTheme(
    _ family: ThemeFamily = .bright,
    colorScheme: ColorScheme? = nil
  ) -> some View {
    modifier(MobileThemeModifier(family: family, colorScheme: colorScheme))
  }

  /// Apply ds colours to native controls using an ``AppearanceMode``.
  /// Install at a scene's root so sheets and navigation inherit the theme.
  func mobileTheme(
    _ family: ThemeFamily = .bright,
    mode: AppearanceMode
  ) -> some View {
    mobileTheme(family, colorScheme: mode.colorScheme)
  }

  /// Paint a screen or scrollable container with the active theme's
  /// `colorSurface`, hiding the system scroll background. Useful inside a
  /// `NavigationStack` or sheet where the container draws its own backdrop.
  func mobileBackground() -> some View {
    modifier(MobileBackgroundModifier())
  }
}
