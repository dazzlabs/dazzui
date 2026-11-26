import SwiftUI

/// Pure colour roles for drawing a theme-family swatch using native SwiftUI
/// shapes, keeping geometry and control drawing owned by the app and OS.
public struct ThemeSwatchColors: Sendable, Equatable {
  public let canvas: Color
  public let surface: Color
  public let inset: Color
  public let border: Color
  public let content: Color
  public let accent: Color
  public let onAccent: Color

  public init(
    canvas: Color,
    surface: Color,
    inset: Color,
    border: Color,
    content: Color,
    accent: Color,
    onAccent: Color
  ) {
    self.canvas = canvas
    self.surface = surface
    self.inset = inset
    self.border = border
    self.content = content
    self.accent = accent
    self.onAccent = onAccent
  }
}

public extension Palette {
  /// Whether this palette belongs to a dark colour scheme.
  var isDark: Bool { nativeTintShade == 500 }

  /// Ramp step used for tinted/plain text on the palette's ground:
  /// shade 700 on light surfaces, shade 300 on dark surfaces.
  private var textShade: Int { isDark ? 300 : 700 }

  /// Foreground colour legible on top of ``accent``.
  var onAccent: Color { colorOnAccent }

  /// Accent ink legible on ``colorSurface`` (grade 700 light, 300 dark).
  var accentText: Color { colorPrimary[textShade] }

  /// Soft translucent wash of ``accent`` (12% opacity).
  var accentSurface: Color { accent.opacity(0.12) }

  /// Hairline stroke tinted with ``accent`` (25% opacity).
  var accentHairline: Color { accent.opacity(0.25) }

  /// Semantic danger tint at the palette's active fill shade.
  var danger: Color { colorDanger[nativeTintShade] }

  /// Danger ink legible on ``colorSurface``.
  var dangerText: Color { colorDanger[textShade] }

  /// Soft translucent danger wash (12% opacity).
  var dangerSurface: Color { danger.opacity(0.12) }

  /// Danger banner or outline hairline (25% opacity).
  var dangerHairline: Color { danger.opacity(0.25) }

  /// Semantic warning tint at the palette's active fill shade.
  var warn: Color { colorWarning[nativeTintShade] }

  /// Warning ink legible on ``colorSurface``.
  var warnText: Color { colorWarning[textShade] }

  /// Soft translucent warning wash (12% opacity).
  var warnSurface: Color { warn.opacity(0.12) }

  /// Warning banner or outline hairline (25% opacity).
  var warnHairline: Color { warn.opacity(0.25) }

  /// Semantic success tint at the palette's active fill shade.
  var success: Color { colorSuccess[nativeTintShade] }

  /// Success ink legible on ``colorSurface``.
  var successText: Color { colorSuccess[textShade] }

  /// Soft translucent success wash (12% opacity).
  var successSurface: Color { success.opacity(0.12) }

  /// Semantic info tint at the palette's active fill shade.
  var info: Color { colorInfo[nativeTintShade] }

  /// Info ink legible on ``colorSurface``.
  var infoText: Color { colorInfo[textShade] }

  /// Soft translucent info wash (12% opacity).
  var infoSurface: Color { info.opacity(0.12) }

  /// Standard hairline divider colour (`colorBorder`).
  var separator: Color { colorBorder }

  /// Stronger hairline for floating edges and card strokes (`colorBorderStrong`).
  var hairlineStrong: Color { colorBorderStrong }

  /// Grouped screen background (`colorSurface`).
  var grouped: Color { colorSurface }

  /// Grouped list row background (`colorSurfaceMuted`).
  var cell: Color { colorSurfaceMuted }

  /// Swatch colour roles derived from this palette.
  var swatchColors: ThemeSwatchColors {
    ThemeSwatchColors(
      canvas: colorCanvas,
      surface: colorSurface,
      inset: colorSurfaceInset,
      border: colorBorderStrong,
      content: colorContent,
      accent: accent,
      onAccent: colorOnAccent
    )
  }
}

public extension ThemeFamily {
  /// The six drawn DazzUI theme families.
  static let drawn: [ThemeFamily] = [
    .bright,
    .studio,
    .frost,
    .graphite,
    .ember,
    .nocturne,
  ]

  /// Whether this family is one of the six drawn DazzUI families.
  var isDrawn: Bool {
    switch self {
    case .bright, .studio, .frost, .graphite, .ember, .nocturne:
      return true
    default:
      return false
    }
  }

  /// Human-readable display name for pickers and theme lists.
  var displayName: String {
    switch self {
    case .bright: return "Bright"
    case .studio: return "Studio"
    case .frost: return "Frost"
    case .graphite: return "Graphite"
    case .ember: return "Ember"
    case .nocturne: return "Nocturne"
    case .omarchyTokyoNight: return "Tokyo Night"
    case .omarchyCatppuccin: return "Catppuccin"
    case .omarchyCatppuccinLatte: return "Catppuccin Latte"
    case .omarchyEthereal: return "Ethereal"
    case .omarchyEverforest: return "Everforest"
    case .omarchyFlexokiLight: return "Flexoki"
    case .omarchyGruvbox: return "Gruvbox"
    case .omarchyHackerman: return "Hackerman"
    case .omarchyKanagawa: return "Kanagawa"
    case .omarchyLastHorizon: return "Last Horizon"
    case .omarchyLumon: return "Lumon"
    case .omarchyLupine: return "Lupine"
    case .omarchyMatteBlack: return "Matte Black"
    case .omarchyMiasma: return "Miasma"
    case .omarchyNord: return "Nord"
    case .omarchyOsakaJade: return "Osaka Jade"
    case .omarchyRetro82: return "Retro 82"
    case .omarchyRistretto: return "Ristretto"
    case .omarchyRosePine: return "Rosé Pine"
    case .omarchySolitude: return "Solitude"
    case .omarchyVantablack: return "Vantablack"
    case .omarchyWhite: return "White"
    case .macos27: return "macOS 27"
    case .macos15: return "macOS 15"
    case .windows11: return "Windows 11"
    case .ubuntu: return "Ubuntu"
    case .debian: return "Debian"
    case .fedora: return "Fedora"
    case .kde: return "KDE"
    case .spotify: return "Spotify"
    }
  }

  /// Resolve the palette for an ``AppearanceMode``, falling back to
  /// `systemColorScheme` when `mode` is `.system`.
  func palette(
    for mode: AppearanceMode,
    systemColorScheme: ColorScheme
  ) -> Palette {
    palette(for: mode.colorScheme ?? systemColorScheme)
  }

  /// Swatch colour roles for this family under the given colour scheme.
  func swatchColors(for colorScheme: ColorScheme) -> ThemeSwatchColors {
    palette(for: colorScheme).swatchColors
  }

  /// Swatch colour roles for this family under an ``AppearanceMode``.
  func swatchColors(
    for mode: AppearanceMode,
    systemColorScheme: ColorScheme
  ) -> ThemeSwatchColors {
    palette(for: mode, systemColorScheme: systemColorScheme).swatchColors
  }
}
