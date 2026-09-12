import SwiftUI

/// The colour token set for one theme.
///
/// DO NOT EDIT - This file is auto-generated
public struct Palette: Sendable {
  public init(
    colorPrimary: ColorSwatch = Colors.brand,
    colorNeutral: ColorSwatch = Colors.neutral,
    colorInfo: ColorSwatch = Colors.sky,
    colorSuccess: ColorSwatch = Colors.green,
    colorWarning: ColorSwatch = Colors.amber,
    colorDanger: ColorSwatch = Colors.red,
    colorCanvas: Color = Color(argb: 0xFFEE_ECF6),
    colorSurface: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color = Color(argb: 0xFFF7_F7FA),
    colorSurfaceSunken: Color = Color(argb: 0xFFE3_E3EC),
    colorSurfaceSubtle: Color = Color(argb: 0x0F14_162A),
    colorSurfaceInset: Color = Color(argb: 0xFFF0_F0F5),
    colorSurfaceRaised: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color = Color(argb: 0xFFF7_F7FA),
    colorSurfaceColumn: Color = Color(argb: 0xFFFA_FAFC),
    colorContent: Color = Color(argb: 0xFF12_142A),
    colorContentSecondary: Color = Color(argb: 0xFF3C_405C),
    colorContentNav: Color = Color(argb: 0xFF4A_4F6B),
    colorContentMuted: Color = Color(argb: 0xFF56_5B78),
    colorContentSubtle: Color = Color(argb: 0xFF8C_92AA),
    colorContentFaint: Color = Color(argb: 0xFFA2_A7BD),
    colorBorder: Color = Color(argb: 0x1214_162A),
    colorBorderStrong: Color = Color(argb: 0x1714_162A),
    colorBorderMuted: Color = Color(argb: 0xFF5F_6478),
    colorOnAccent: Color = Color(argb: 0xFFFF_FFFF),
    nativeTintShade: Int = 600
  ) {
    self.colorPrimary = colorPrimary
    self.colorNeutral = colorNeutral
    self.colorInfo = colorInfo
    self.colorSuccess = colorSuccess
    self.colorWarning = colorWarning
    self.colorDanger = colorDanger
    self.colorCanvas = colorCanvas
    self.colorSurface = colorSurface
    self.colorSurfaceMuted = colorSurfaceMuted
    self.colorSurfaceSunken = colorSurfaceSunken
    self.colorSurfaceSubtle = colorSurfaceSubtle
    self.colorSurfaceInset = colorSurfaceInset
    self.colorSurfaceRaised = colorSurfaceRaised
    self.colorSurfaceOverlay = colorSurfaceOverlay
    self.colorSurfaceChrome = colorSurfaceChrome
    self.colorSurfaceColumn = colorSurfaceColumn
    self.colorContent = colorContent
    self.colorContentSecondary = colorContentSecondary
    self.colorContentNav = colorContentNav
    self.colorContentMuted = colorContentMuted
    self.colorContentSubtle = colorContentSubtle
    self.colorContentFaint = colorContentFaint
    self.colorBorder = colorBorder
    self.colorBorderStrong = colorBorderStrong
    self.colorBorderMuted = colorBorderMuted
    self.colorOnAccent = colorOnAccent
    self.nativeTintShade = nativeTintShade
  }

  public let colorPrimary: ColorSwatch
  public let colorNeutral: ColorSwatch
  public let colorInfo: ColorSwatch
  public let colorSuccess: ColorSwatch
  public let colorWarning: ColorSwatch
  public let colorDanger: ColorSwatch
  public let colorCanvas: Color
  public let colorSurface: Color
  public let colorSurfaceMuted: Color
  public let colorSurfaceSunken: Color
  public let colorSurfaceSubtle: Color
  public let colorSurfaceInset: Color
  public let colorSurfaceRaised: Color
  public let colorSurfaceOverlay: Color
  public let colorSurfaceChrome: Color
  public let colorSurfaceColumn: Color
  public let colorContent: Color
  public let colorContentSecondary: Color
  public let colorContentNav: Color
  public let colorContentMuted: Color
  public let colorContentSubtle: Color
  public let colorContentFaint: Color
  public let colorBorder: Color
  public let colorBorderStrong: Color
  public let colorBorderMuted: Color
  public let colorOnAccent: Color
  public let nativeTintShade: Int


}

public extension Palette {
  /// Studio Light — the baseline every other theme is written against.
  static let studioLight = Palette()

  /// The `bright-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let brightLight = Palette(
    colorPrimary: Colors.ink,
    colorCanvas: Color(argb: 0xFFE6_EADE),
    colorSurface: Color(argb: 0xFFFB_FAF7),
    colorSurfaceMuted: Color(argb: 0xFFFF_FFFF),
    colorSurfaceSunken: Color(argb: 0xFFE5_E3DB),
    colorSurfaceSubtle: Color(argb: 0x0F11_1C2E),
    colorSurfaceInset: Color(argb: 0xFFF0_EFE9),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFB_FAF7),
    colorSurfaceChrome: Color(argb: 0xFFFF_FFFF),
    colorSurfaceColumn: Color(argb: 0xFFF4_F3EE),
    colorContent: Color(argb: 0xFF11_1C2E),
    colorContentSecondary: Color(argb: 0xC711_1C2E),
    colorContentNav: Color(argb: 0xB311_1C2E),
    colorContentMuted: Color(argb: 0xA811_1C2E),
    colorContentSubtle: Color(argb: 0x7311_1C2E),
    colorContentFaint: Color(argb: 0x6111_1C2E),
    colorBorder: Color(argb: 0x1411_1C2E),
    colorBorderStrong: Color(argb: 0x1711_1C2E),
    colorBorderMuted: Color(argb: 0x4011_1C2E),
    colorOnAccent: Color(argb: 0xFFD6_FF3F)
  )

  /// The `bright-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let brightDark = Palette(
    colorPrimary: Colors.acid,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF06_0B12),
    colorSurface: Color(argb: 0xFF0C_141E),
    colorSurfaceMuted: Color(argb: 0xFF14_1D29),
    colorSurfaceSunken: Color(argb: 0xFF1C_2734),
    colorSurfaceSubtle: Color(argb: 0x12F2_F4EF),
    colorSurfaceInset: Color(argb: 0xFF16_202C),
    colorSurfaceRaised: Color(argb: 0xFF2A_3644),
    colorSurfaceOverlay: Color(argb: 0xFF11_1A26),
    colorSurfaceChrome: Color(argb: 0xFF11_1A26),
    colorSurfaceColumn: Color(argb: 0xFF0A_111A),
    colorContent: Color(argb: 0xFFF2_F4EF),
    colorContentSecondary: Color(argb: 0xC7F2_F4EF),
    colorContentNav: Color(argb: 0xB3F2_F4EF),
    colorContentMuted: Color(argb: 0xA8F2_F4EF),
    colorContentSubtle: Color(argb: 0x73F2_F4EF),
    colorContentFaint: Color(argb: 0x61F2_F4EF),
    colorBorder: Color(argb: 0x14F2_F4EF),
    colorBorderStrong: Color(argb: 0x17F2_F4EF),
    colorBorderMuted: Color(argb: 0xFF35_414F),
    colorOnAccent: Color(argb: 0xFF11_1C2E),
    nativeTintShade: 500
  )

  /// The `frost-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let frostLight = Palette(
    colorPrimary: Colors.frost,
    colorCanvas: Color(argb: 0xFFE5_EDF0),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF2_F6F8),
    colorSurfaceSunken: Color(argb: 0xFFDB_E6EA),
    colorSurfaceSubtle: Color(argb: 0x0F0E_1F26),
    colorSurfaceInset: Color(argb: 0xFFEC_F2F4),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF2_F6F8),
    colorSurfaceColumn: Color(argb: 0xFFF2_F6F8),
    colorContent: Color(argb: 0xFF0E_1F26),
    colorContentSecondary: Color(argb: 0xFF2F_4C58),
    colorContentNav: Color(argb: 0xFF3A_5B68),
    colorContentMuted: Color(argb: 0xFF45_6573),
    colorContentSubtle: Color(argb: 0xFF77_94A0),
    colorContentFaint: Color(argb: 0xFF9A_B1BA),
    colorBorder: Color(argb: 0x210E_1F26),
    colorBorderStrong: Color(argb: 0x2B0E_1F26),
    colorBorderMuted: Color(argb: 0xFF45_6573),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `frost-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let frostDark = Palette(
    colorPrimary: Colors.frost,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF07_0D11),
    colorSurface: Color(argb: 0xFF0D_151A),
    colorSurfaceMuted: Color(argb: 0xFF11_1A20),
    colorSurfaceSunken: Color(argb: 0xFF23_333D),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF18_262E),
    colorSurfaceRaised: Color(argb: 0xFF37_4B57),
    colorSurfaceOverlay: Color(argb: 0xFF11_1A20),
    colorSurfaceChrome: Color(argb: 0xFF11_1A20),
    colorSurfaceColumn: Color(argb: 0xFF11_1A20),
    colorContent: Color(argb: 0xFFEA_F3F7),
    colorContentSecondary: Color(argb: 0xFFBC_D0D9),
    colorContentNav: Color(argb: 0xFF93_AAB5),
    colorContentMuted: Color(argb: 0xFF93_AAB5),
    colorContentSubtle: Color(argb: 0xFF7F_99A4),
    colorContentFaint: Color(argb: 0xFF62_808D),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF32_454F),
    colorOnAccent: Color(argb: 0xFF04_171D),
    nativeTintShade: 500
  )

  /// The `graphite-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let graphiteLight = Palette(
    colorPrimary: Colors.graphite,
    colorCanvas: Color(argb: 0xFFE8_E8EC),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF6_F6F8),
    colorSurfaceSunken: Color(argb: 0xFFE2_E2E7),
    colorSurfaceSubtle: Color(argb: 0x0F17_171B),
    colorSurfaceInset: Color(argb: 0xFFEF_EFF2),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF6_F6F8),
    colorSurfaceColumn: Color(argb: 0xFFF6_F6F8),
    colorContent: Color(argb: 0xFF17_171B),
    colorContentSecondary: Color(argb: 0xFF3D_3D45),
    colorContentNav: Color(argb: 0xFF4C_4C56),
    colorContentMuted: Color(argb: 0xFF57_575F),
    colorContentSubtle: Color(argb: 0xFF8A_8A93),
    colorContentFaint: Color(argb: 0xFFA3_A3AB),
    colorBorder: Color(argb: 0x2117_171B),
    colorBorderStrong: Color(argb: 0x2B17_171B),
    colorBorderMuted: Color(argb: 0xFF57_575F),
    colorOnAccent: Color(argb: 0xFFFA_FAFA)
  )

  /// The `graphite-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let graphiteDark = Palette(
    colorPrimary: Colors.graphiteDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF08_080A),
    colorSurface: Color(argb: 0xFF0F_0F12),
    colorSurfaceMuted: Color(argb: 0xFF14_1417),
    colorSurfaceSunken: Color(argb: 0xFF27_272C),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF1C_1C21),
    colorSurfaceRaised: Color(argb: 0xFF3B_3B43),
    colorSurfaceOverlay: Color(argb: 0xFF14_1417),
    colorSurfaceChrome: Color(argb: 0xFF14_1417),
    colorSurfaceColumn: Color(argb: 0xFF14_1417),
    colorContent: Color(argb: 0xFFF4_F4F6),
    colorContentSecondary: Color(argb: 0xFFC9_C9D0),
    colorContentNav: Color(argb: 0xFF9D_9DA6),
    colorContentMuted: Color(argb: 0xFF9D_9DA6),
    colorContentSubtle: Color(argb: 0xFF8A_8A93),
    colorContentFaint: Color(argb: 0xFF6A_6A73),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF38_383F),
    colorOnAccent: Color(argb: 0xFF13_1316),
    nativeTintShade: 500
  )

  /// The `ember-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let emberLight = Palette(
    colorPrimary: Colors.ember,
    colorCanvas: Color(argb: 0xFFEA_E2D4),
    colorSurface: Color(argb: 0xFFFF_FDF9),
    colorSurfaceMuted: Color(argb: 0xFFF6_F1E8),
    colorSurfaceSunken: Color(argb: 0xFFE6_DDCD),
    colorSurfaceSubtle: Color(argb: 0x0F24_1A11),
    colorSurfaceInset: Color(argb: 0xFFF2_EBDF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FDF9),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FDF9),
    colorSurfaceChrome: Color(argb: 0xFFF6_F1E8),
    colorSurfaceColumn: Color(argb: 0xFFF6_F1E8),
    colorContent: Color(argb: 0xFF24_1A11),
    colorContentSecondary: Color(argb: 0xFF4B_3A2B),
    colorContentNav: Color(argb: 0xFF5B_4839),
    colorContentMuted: Color(argb: 0xFF6A_5747),
    colorContentSubtle: Color(argb: 0xFF96_867A),
    colorContentFaint: Color(argb: 0xFFB0_A294),
    colorBorder: Color(argb: 0x2124_1A11),
    colorBorderStrong: Color(argb: 0x2B24_1A11),
    colorBorderMuted: Color(argb: 0xFF6A_5747),
    colorOnAccent: Color(argb: 0xFFFF_FAF3)
  )

  /// The `ember-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let emberDark = Palette(
    colorPrimary: Colors.emberDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0C_0906),
    colorSurface: Color(argb: 0xFF14_100B),
    colorSurfaceMuted: Color(argb: 0xFF19_1410),
    colorSurfaceSunken: Color(argb: 0xFF2F_271E),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF23_1D16),
    colorSurfaceRaised: Color(argb: 0xFF45_3A2D),
    colorSurfaceOverlay: Color(argb: 0xFF19_1410),
    colorSurfaceChrome: Color(argb: 0xFF19_1410),
    colorSurfaceColumn: Color(argb: 0xFF19_1410),
    colorContent: Color(argb: 0xFFF6_EFE4),
    colorContentSecondary: Color(argb: 0xFFD5_C6B4),
    colorContentNav: Color(argb: 0xFFAB_9886),
    colorContentMuted: Color(argb: 0xFFAB_9886),
    colorContentSubtle: Color(argb: 0xFF94_806D),
    colorContentFaint: Color(argb: 0xFF71_604F),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF44_3A2D),
    colorOnAccent: Color(argb: 0xFF24_1206),
    nativeTintShade: 500
  )

  /// The `nocturne-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let nocturneLight = Palette(
    colorPrimary: Colors.nocturne,
    colorCanvas: Color(argb: 0xFFE7_E7EF),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF6_F6FA),
    colorSurfaceSunken: Color(argb: 0xFFE2_E2EC),
    colorSurfaceSubtle: Color(argb: 0x0F23_242E),
    colorSurfaceInset: Color(argb: 0xFFEE_EEF5),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF6_F6FA),
    colorSurfaceColumn: Color(argb: 0xFFF6_F6FA),
    colorContent: Color(argb: 0xFF23_242E),
    colorContentSecondary: Color(argb: 0xFF38_3A49),
    colorContentNav: Color(argb: 0xFF4C_4E61),
    colorContentMuted: Color(argb: 0xFF63_667D),
    colorContentSubtle: Color(argb: 0xFF85_889E),
    colorContentFaint: Color(argb: 0xFFAD_AFC1),
    colorBorder: Color(argb: 0x2123_242E),
    colorBorderStrong: Color(argb: 0x2B23_242E),
    colorBorderMuted: Color(argb: 0xFFD8_D9E4),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `nocturne-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let nocturneDark = Palette(
    colorPrimary: Colors.nocturne,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF10_1120),
    colorSurface: Color(argb: 0xFF16_1826),
    colorSurfaceMuted: Color(argb: 0xFF23_2532),
    colorSurfaceSunken: Color(argb: 0xFF2F_313C),
    colorSurfaceSubtle: Color(argb: 0x12E9_E9ED),
    colorSurfaceInset: Color(argb: 0xFF29_2B31),
    colorSurfaceRaised: Color(argb: 0xFF3F_424D),
    colorSurfaceOverlay: Color(argb: 0xFF23_2532),
    colorSurfaceChrome: Color(argb: 0xFF1C_1E2C),
    colorSurfaceColumn: Color(argb: 0xFF23_2532),
    colorContent: Color(argb: 0xFFE9_E9ED),
    colorContentSecondary: Color(argb: 0xFFCF_D3E5),
    colorContentNav: Color(argb: 0xFFB2_B6CA),
    colorContentMuted: Color(argb: 0xFF93_97AB),
    colorContentSubtle: Color(argb: 0xFF75_798C),
    colorContentFaint: Color(argb: 0xFF59_5D6C),
    colorBorder: Color(argb: 0x29E9_E9ED),
    colorBorderStrong: Color(argb: 0x33E9_E9ED),
    colorBorderMuted: Color(argb: 0xFF3F_424D),
    colorOnAccent: Color(argb: 0xFF16_1826),
    nativeTintShade: 500
  )

  /// The `studio-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let studioDark = Palette(
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0A_0B10),
    colorSurface: Color(argb: 0xFF0D_0F14),
    colorSurfaceMuted: Color(argb: 0xFF12_141A),
    colorSurfaceSunken: Color(argb: 0xFF26_2A34),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF1B_1E26),
    colorSurfaceRaised: Color(argb: 0xFF3A_3F4D),
    colorSurfaceOverlay: Color(argb: 0xFF12_141A),
    colorSurfaceChrome: Color(argb: 0xFF12_141A),
    colorSurfaceColumn: Color(argb: 0xFF0A_0C11),
    colorContent: Color(argb: 0xFFF2_F3FA),
    colorContentSecondary: Color(argb: 0xFFC3_C8DC),
    colorContentNav: Color(argb: 0xFF9A_A1BB),
    colorContentMuted: Color(argb: 0xFF9A_A1BB),
    colorContentSubtle: Color(argb: 0xFF8B_93B0),
    colorContentFaint: Color(argb: 0xFF6A_7090),
    colorBorder: Color(argb: 0x0FFF_FFFF),
    colorBorderStrong: Color(argb: 0x17FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF36_3C4D),
    nativeTintShade: 500
  )
}

public extension Palette {
  /// The ds filled action colour, applied through SwiftUI tint.
  var accent: Color { colorPrimary[nativeTintShade] }
}
