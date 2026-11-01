import SwiftUI

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
      .preferredColorScheme(colorScheme)
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
}
