import SwiftUI

/// The colour token set for one theme.
///
/// DO NOT EDIT - This file is auto-generated
public struct Palette: Sendable {
  public init(
    colorPrimary: ColorSwatch = Colors.ink,
    colorNeutral: ColorSwatch = Colors.neutral,
    colorInfo: ColorSwatch = Colors.sky,
    colorSuccess: ColorSwatch = Colors.green,
    colorWarning: ColorSwatch = Colors.amber,
    colorDanger: ColorSwatch = Colors.red,
    colorCanvas: Color = Color(argb: 0xFFE6_EADE),
    colorSurface: Color = Color(argb: 0xFFFB_FAF7),
    colorSurfaceMuted: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceSunken: Color = Color(argb: 0xFFE5_E3DB),
    colorSurfaceSubtle: Color = Color(argb: 0x0F11_1C2E),
    colorSurfaceInset: Color = Color(argb: 0xFFF0_EFE9),
    colorSurfaceRaised: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color = Color(argb: 0xFFFB_FAF7),
    colorSurfaceChrome: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceColumn: Color = Color(argb: 0xFFF4_F3EE),
    colorContent: Color = Color(argb: 0xFF11_1C2E),
    colorContentSecondary: Color = Color(argb: 0xC711_1C2E),
    colorContentNav: Color = Color(argb: 0xB311_1C2E),
    colorContentMuted: Color = Color(argb: 0xA811_1C2E),
    colorContentSubtle: Color = Color(argb: 0x7311_1C2E),
    colorContentFaint: Color = Color(argb: 0x6111_1C2E),
    colorBorder: Color = Color(argb: 0x1411_1C2E),
    colorBorderStrong: Color = Color(argb: 0x1711_1C2E),
    colorBorderMuted: Color = Color(argb: 0x4011_1C2E),
    colorOnAccent: Color = Color(argb: 0xFFD6_FF3F),
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
  /// Bright Light — the default every other theme is written against.
  static let brightLight = Palette()

  /// The `studio-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let studioLight = Palette(
    colorPrimary: Colors.brand,
    colorCanvas: Color(argb: 0xFFEE_ECF6),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF7_F7FA),
    colorSurfaceSunken: Color(argb: 0xFFE3_E3EC),
    colorSurfaceSubtle: Color(argb: 0x0F14_162A),
    colorSurfaceInset: Color(argb: 0xFFF0_F0F5),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF7_F7FA),
    colorSurfaceColumn: Color(argb: 0xFFFA_FAFC),
    colorContent: Color(argb: 0xFF12_142A),
    colorContentSecondary: Color(argb: 0xFF3C_405C),
    colorContentNav: Color(argb: 0xFF4A_4F6B),
    colorContentMuted: Color(argb: 0xFF56_5B78),
    colorContentSubtle: Color(argb: 0xFF8C_92AA),
    colorContentFaint: Color(argb: 0xFFA2_A7BD),
    colorBorder: Color(argb: 0x1214_162A),
    colorBorderStrong: Color(argb: 0x1714_162A),
    colorBorderMuted: Color(argb: 0xFF5F_6478),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `studio-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let studioDark = Palette(
    colorPrimary: Colors.brand,
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
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    nativeTintShade: 500
  )

  /// The `frost-light` counterpart of ``brightLight``.
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

  /// The `frost-dark` counterpart of ``brightLight``.
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

  /// The `graphite-light` counterpart of ``brightLight``.
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

  /// The `graphite-dark` counterpart of ``brightLight``.
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

  /// The `ember-light` counterpart of ``brightLight``.
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

  /// The `ember-dark` counterpart of ``brightLight``.
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

  /// The `nocturne-light` counterpart of ``brightLight``.
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

  /// The `nocturne-dark` counterpart of ``brightLight``.
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

  /// The `omarchy-tokyo-night-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyTokyoNightLight = Palette(
    colorPrimary: Colors.omarchyTokyoNightLight,
    colorCanvas: Color(argb: 0xFFDB_DCE5),
    colorSurface: Color(argb: 0xFFEC_EEF6),
    colorSurfaceMuted: Color(argb: 0xFFE5_E6EF),
    colorSurfaceSunken: Color(argb: 0xFFCF_D0D9),
    colorSurfaceSubtle: Color(argb: 0x101A_1B26),
    colorSurfaceInset: Color(argb: 0xFFDD_DEE7),
    colorSurfaceRaised: Color(argb: 0xFFEC_EEF6),
    colorSurfaceOverlay: Color(argb: 0xFFEC_EEF6),
    colorSurfaceChrome: Color(argb: 0xFFE5_E6EF),
    colorSurfaceColumn: Color(argb: 0xFFE8_E9F2),
    colorContent: Color(argb: 0xFF1A_1B26),
    colorContentSecondary: Color(argb: 0xFF40_414B),
    colorContentNav: Color(argb: 0xFF55_5660),
    colorContentMuted: Color(argb: 0xFF66_6771),
    colorContentSubtle: Color(argb: 0xFF83_848E),
    colorContentFaint: Color(argb: 0xFFA0_A2AB),
    colorBorder: Color(argb: 0x211A_1B26),
    colorBorderStrong: Color(argb: 0x2B1A_1B26),
    colorBorderMuted: Color(argb: 0xFF9C_9EA7),
    colorOnAccent: Color(argb: 0xFFEC_EEF6)
  )

  /// The `omarchy-tokyo-night-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyTokyoNightDark = Palette(
    colorPrimary: Colors.omarchyTokyoNightDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF16_1721),
    colorSurface: Color(argb: 0xFF1A_1B26),
    colorSurfaceMuted: Color(argb: 0xFF20_212D),
    colorSurfaceSunken: Color(argb: 0xFF2E_303E),
    colorSurfaceSubtle: Color(argb: 0x14A9_B1D6),
    colorSurfaceInset: Color(argb: 0xFF26_2835),
    colorSurfaceRaised: Color(argb: 0xFF43_4659),
    colorSurfaceOverlay: Color(argb: 0xFF20_212D),
    colorSurfaceChrome: Color(argb: 0xFF20_212D),
    colorSurfaceColumn: Color(argb: 0xFF17_1822),
    colorContent: Color(argb: 0xFFA9_B1D6),
    colorContentSecondary: Color(argb: 0xFF8F_96B6),
    colorContentNav: Color(argb: 0xFF81_87A5),
    colorContentMuted: Color(argb: 0xFF7E_84A1),
    colorContentSubtle: Color(argb: 0xFF62_667E),
    colorContentFaint: Color(argb: 0xFF4D_5165),
    colorBorder: Color(argb: 0x29A9_B1D6),
    colorBorderStrong: Color(argb: 0x33A9_B1D6),
    colorBorderMuted: Color(argb: 0xFF50_5469),
    colorOnAccent: Color(argb: 0xFF1A_1B26),
    nativeTintShade: 500
  )

  /// The `omarchy-catppuccin-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyCatppuccinLight = Palette(
    colorPrimary: Colors.omarchyCatppuccinLight,
    colorCanvas: Color(argb: 0xFFE3_E4EC),
    colorSurface: Color(argb: 0xFFF4_F6FD),
    colorSurfaceMuted: Color(argb: 0xFFED_EEF6),
    colorSurfaceSunken: Color(argb: 0xFFD6_D8E0),
    colorSurfaceSubtle: Color(argb: 0x0F1E_1E2E),
    colorSurfaceInset: Color(argb: 0xFFE4_E6EE),
    colorSurfaceRaised: Color(argb: 0xFFF4_F6FD),
    colorSurfaceOverlay: Color(argb: 0xFFF4_F6FD),
    colorSurfaceChrome: Color(argb: 0xFFED_EEF6),
    colorSurfaceColumn: Color(argb: 0xFFF0_F1F9),
    colorContent: Color(argb: 0xFF1E_1E2E),
    colorContentSecondary: Color(argb: 0xFF45_4553),
    colorContentNav: Color(argb: 0xFF5A_5A68),
    colorContentMuted: Color(argb: 0xFF6B_6C79),
    colorContentSubtle: Color(argb: 0xFF89_8A96),
    colorContentFaint: Color(argb: 0xFFA7_A8B2),
    colorBorder: Color(argb: 0x211E_1E2E),
    colorBorderStrong: Color(argb: 0x2B1E_1E2E),
    colorBorderMuted: Color(argb: 0xFFA3_A4AE),
    colorOnAccent: Color(argb: 0xFFF4_F6FD)
  )

  /// The `omarchy-catppuccin-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyCatppuccinDark = Palette(
    colorPrimary: Colors.omarchyCatppuccinDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1A_1A29),
    colorSurface: Color(argb: 0xFF1E_1E2E),
    colorSurfaceMuted: Color(argb: 0xFF24_2535),
    colorSurfaceSunken: Color(argb: 0xFF32_3445),
    colorSurfaceSubtle: Color(argb: 0x12CD_D6F4),
    colorSurfaceInset: Color(argb: 0xFF2A_2B3B),
    colorSurfaceRaised: Color(argb: 0xFF48_4A5E),
    colorSurfaceOverlay: Color(argb: 0xFF24_2535),
    colorSurfaceChrome: Color(argb: 0xFF24_2535),
    colorSurfaceColumn: Color(argb: 0xFF1B_1B2A),
    colorContent: Color(argb: 0xFFCD_D6F4),
    colorContentSecondary: Color(argb: 0xFFAE_B5D0),
    colorContentNav: Color(argb: 0xFF9C_A2BD),
    colorContentMuted: Color(argb: 0xFF8E_94AD),
    colorContentSubtle: Color(argb: 0xFF76_7A91),
    colorContentFaint: Color(argb: 0xFF5D_6075),
    colorBorder: Color(argb: 0x29CD_D6F4),
    colorBorderStrong: Color(argb: 0x33CD_D6F4),
    colorBorderMuted: Color(argb: 0xFF61_6479),
    colorOnAccent: Color(argb: 0xFF1E_1E2E),
    nativeTintShade: 500
  )

  /// The `omarchy-catppuccin-latte-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyCatppuccinLatteLight = Palette(
    colorPrimary: Colors.omarchyCatppuccinLatteLight,
    colorCanvas: Color(argb: 0xFFDD_DFE6),
    colorSurface: Color(argb: 0xFFEF_F1F5),
    colorSurfaceMuted: Color(argb: 0xFFE7_E9EF),
    colorSurfaceSunken: Color(argb: 0xFFD0_D3DB),
    colorSurfaceSubtle: Color(argb: 0x144C_4F69),
    colorSurfaceInset: Color(argb: 0xFFDF_E1E8),
    colorSurfaceRaised: Color(argb: 0xFFEF_F1F5),
    colorSurfaceOverlay: Color(argb: 0xFFEF_F1F5),
    colorSurfaceChrome: Color(argb: 0xFFE7_E9EF),
    colorSurfaceColumn: Color(argb: 0xFFEA_ECF1),
    colorContent: Color(argb: 0xFF4C_4F69),
    colorContentSecondary: Color(argb: 0xFF68_6B81),
    colorContentNav: Color(argb: 0xFF68_6B81),
    colorContentMuted: Color(argb: 0xFF68_6B81),
    colorContentSubtle: Color(argb: 0xFF9E_A0AF),
    colorContentFaint: Color(argb: 0xFFB4_B7C3),
    colorBorder: Color(argb: 0x214C_4F69),
    colorBorderStrong: Color(argb: 0x2B4C_4F69),
    colorBorderMuted: Color(argb: 0xFFB1_B3C0),
    colorOnAccent: Color(argb: 0xFFEF_F1F5)
  )

  /// The `omarchy-catppuccin-latte-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyCatppuccinLatteDark = Palette(
    colorPrimary: Colors.omarchyCatppuccinLatteDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1E_2029),
    colorSurface: Color(argb: 0xFF22_242F),
    colorSurfaceMuted: Color(argb: 0xFF29_2B35),
    colorSurfaceSunken: Color(argb: 0xFF38_3A44),
    colorSurfaceSubtle: Color(argb: 0x12EF_F1F5),
    colorSurfaceInset: Color(argb: 0xFF2F_313B),
    colorSurfaceRaised: Color(argb: 0xFF4F_515A),
    colorSurfaceOverlay: Color(argb: 0xFF29_2B35),
    colorSurfaceChrome: Color(argb: 0xFF29_2B35),
    colorSurfaceColumn: Color(argb: 0xFF1F_212A),
    colorContent: Color(argb: 0xFFEF_F1F5),
    colorContentSecondary: Color(argb: 0xFFCA_CCD1),
    colorContentNav: Color(argb: 0xFFB6_B8BE),
    colorContentMuted: Color(argb: 0xFFA5_A7AE),
    colorContentSubtle: Color(argb: 0xFF89_8B92),
    colorContentFaint: Color(argb: 0xFF6C_6E76),
    colorBorder: Color(argb: 0x29EF_F1F5),
    colorBorderStrong: Color(argb: 0x33EF_F1F5),
    colorBorderMuted: Color(argb: 0xFF70_727A),
    colorOnAccent: Color(argb: 0xFF22_242F),
    nativeTintShade: 500
  )

  /// The `omarchy-ethereal-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyEtherealLight = Palette(
    colorPrimary: Colors.omarchyEtherealLight,
    colorCanvas: Color(argb: 0xFFEC_E3DE),
    colorSurface: Color(argb: 0xFFFF_F4ED),
    colorSurfaceMuted: Color(argb: 0xFFF7_EDE6),
    colorSurfaceSunken: Color(argb: 0xFFDF_D6D3),
    colorSurfaceSubtle: Color(argb: 0x0F06_0B1E),
    colorSurfaceInset: Color(argb: 0xFFEE_E4DF),
    colorSurfaceRaised: Color(argb: 0xFFFF_F4ED),
    colorSurfaceOverlay: Color(argb: 0xFFFF_F4ED),
    colorSurfaceChrome: Color(argb: 0xFFF7_EDE6),
    colorSurfaceColumn: Color(argb: 0xFFFA_EFE9),
    colorContent: Color(argb: 0xFF06_0B1E),
    colorContentSecondary: Color(argb: 0xFF33_3543),
    colorContentNav: Color(argb: 0xFF4C_4C58),
    colorContentMuted: Color(argb: 0xFF60_5F69),
    colorContentSubtle: Color(argb: 0xFF83_8086),
    colorContentFaint: Color(argb: 0xFFA5_A0A2),
    colorBorder: Color(argb: 0x2106_0B1E),
    colorBorderStrong: Color(argb: 0x2B06_0B1E),
    colorBorderMuted: Color(argb: 0xFFA0_9B9E),
    colorOnAccent: Color(argb: 0xFFFF_F4ED)
  )

  /// The `omarchy-ethereal-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyEtherealDark = Palette(
    colorPrimary: Colors.omarchyEtherealDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF02_050C),
    colorSurface: Color(argb: 0xFF06_0B1E),
    colorSurfaceMuted: Color(argb: 0xFF11_1324),
    colorSurfaceSunken: Color(argb: 0xFF22_212E),
    colorSurfaceSubtle: Color(argb: 0x12FF_CEAD),
    colorSurfaceInset: Color(argb: 0xFF18_1929),
    colorSurfaceRaised: Color(argb: 0xFF3C_363D),
    colorSurfaceOverlay: Color(argb: 0xFF11_1324),
    colorSurfaceChrome: Color(argb: 0xFF11_1324),
    colorSurfaceColumn: Color(argb: 0xFF04_0612),
    colorContent: Color(argb: 0xFFFF_CEAD),
    colorContentSecondary: Color(argb: 0xFFD2_AB93),
    colorContentNav: Color(argb: 0xFFB9_9785),
    colorContentMuted: Color(argb: 0xFFA5_887A),
    colorContentSubtle: Color(argb: 0xFF83_6D66),
    colorContentFaint: Color(argb: 0xFF60_5151),
    colorBorder: Color(argb: 0x29FF_CEAD),
    colorBorderStrong: Color(argb: 0x33FF_CEAD),
    colorBorderMuted: Color(argb: 0xFF65_5554),
    colorOnAccent: Color(argb: 0xFF06_0B1E),
    nativeTintShade: 500
  )

  /// The `omarchy-everforest-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyEverforestLight = Palette(
    colorPrimary: Colors.omarchyEverforestLight,
    colorCanvas: Color(argb: 0xFFE3_E1DC),
    colorSurface: Color(argb: 0xFFF5_F2EC),
    colorSurfaceMuted: Color(argb: 0xFFED_EBE5),
    colorSurfaceSunken: Color(argb: 0xFFD6_D4D0),
    colorSurfaceSubtle: Color(argb: 0x112D_353B),
    colorSurfaceInset: Color(argb: 0xFFE5_E2DD),
    colorSurfaceRaised: Color(argb: 0xFFF5_F2EC),
    colorSurfaceOverlay: Color(argb: 0xFFF5_F2EC),
    colorSurfaceChrome: Color(argb: 0xFFED_EBE5),
    colorSurfaceColumn: Color(argb: 0xFFF0_EDE8),
    colorContent: Color(argb: 0xFF2D_353B),
    colorContentSecondary: Color(argb: 0xFF51_575B),
    colorContentNav: Color(argb: 0xFF65_6A6D),
    colorContentMuted: Color(argb: 0xFF69_6E70),
    colorContentSubtle: Color(argb: 0xFF91_9394),
    colorContentFaint: Color(argb: 0xFFAD_AEAC),
    colorBorder: Color(argb: 0x212D_353B),
    colorBorderStrong: Color(argb: 0x2B2D_353B),
    colorBorderMuted: Color(argb: 0xFFA9_AAA9),
    colorOnAccent: Color(argb: 0xFFF5_F2EC)
  )

  /// The `omarchy-everforest-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyEverforestDark = Palette(
    colorPrimary: Colors.omarchyEverforestDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF29_3036),
    colorSurface: Color(argb: 0xFF2D_353B),
    colorSurfaceMuted: Color(argb: 0xFF34_3C40),
    colorSurfaceSunken: Color(argb: 0xFF47_4C4C),
    colorSurfaceSubtle: Color(argb: 0x16D3_C6AA),
    colorSurfaceInset: Color(argb: 0xFF3C_4345),
    colorSurfaceRaised: Color(argb: 0xFF62_635E),
    colorSurfaceOverlay: Color(argb: 0xFF34_3C40),
    colorSurfaceChrome: Color(argb: 0xFF34_3C40),
    colorSurfaceColumn: Color(argb: 0xFF2A_3137),
    colorContent: Color(argb: 0xFFD3_C6AA),
    colorContentSecondary: Color(argb: 0xFFB5_AC96),
    colorContentNav: Color(argb: 0xFFA5_9D8B),
    colorContentMuted: Color(argb: 0xFFA5_9D8B),
    colorContentSubtle: Color(argb: 0xFF80_7E72),
    colorContentFaint: Color(argb: 0xFF69_6963),
    colorBorder: Color(argb: 0x29D3_C6AA),
    colorBorderStrong: Color(argb: 0x33D3_C6AA),
    colorBorderMuted: Color(argb: 0xFF6C_6C65),
    colorOnAccent: Color(argb: 0xFF2D_353B),
    nativeTintShade: 500
  )

  /// The `omarchy-flexoki-light-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyFlexokiLightLight = Palette(
    colorPrimary: Colors.omarchyFlexokiLightLight,
    colorCanvas: Color(argb: 0xFFED_EADF),
    colorSurface: Color(argb: 0xFFFF_FCF0),
    colorSurfaceMuted: Color(argb: 0xFFF7_F5E9),
    colorSurfaceSunken: Color(argb: 0xFFE0_DED3),
    colorSurfaceSubtle: Color(argb: 0x0F10_0F0F),
    colorSurfaceInset: Color(argb: 0xFFEF_ECE1),
    colorSurfaceRaised: Color(argb: 0xFFFF_FCF0),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FCF0),
    colorSurfaceChrome: Color(argb: 0xFFF7_F5E9),
    colorSurfaceColumn: Color(argb: 0xFFFA_F7EC),
    colorContent: Color(argb: 0xFF10_0F0F),
    colorContentSecondary: Color(argb: 0xFF3B_3A38),
    colorContentNav: Color(argb: 0xFF53_514E),
    colorContentMuted: Color(argb: 0xFF66_6460),
    colorContentSubtle: Color(argb: 0xFF88_8680),
    colorContentFaint: Color(argb: 0xFFA9_A79F),
    colorBorder: Color(argb: 0x2110_0F0F),
    colorBorderStrong: Color(argb: 0x2B10_0F0F),
    colorBorderMuted: Color(argb: 0xFFA4_A29B),
    colorOnAccent: Color(argb: 0xFFFF_FCF0)
  )

  /// The `omarchy-flexoki-light-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyFlexokiLightDark = Palette(
    colorPrimary: Colors.omarchyFlexokiLightDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0E_0E0D),
    colorSurface: Color(argb: 0xFF07_0707),
    colorSurfaceMuted: Color(argb: 0xFF11_1110),
    colorSurfaceSunken: Color(argb: 0xFF20_201E),
    colorSurfaceSubtle: Color(argb: 0x12FF_FCF0),
    colorSurfaceInset: Color(argb: 0xFF18_1817),
    colorSurfaceRaised: Color(argb: 0xFF35_3532),
    colorSurfaceOverlay: Color(argb: 0xFF11_1110),
    colorSurfaceChrome: Color(argb: 0xFF11_1110),
    colorSurfaceColumn: Color(argb: 0xFF01_0101),
    colorContent: Color(argb: 0xFFFF_FCF0),
    colorContentSecondary: Color(argb: 0xFFD2_D0C6),
    colorContentNav: Color(argb: 0xFFBA_B7AF),
    colorContentMuted: Color(argb: 0xFFA6_A49C),
    colorContentSubtle: Color(argb: 0xFF83_827C),
    colorContentFaint: Color(argb: 0xFF60_5F5B),
    colorBorder: Color(argb: 0x29FF_FCF0),
    colorBorderStrong: Color(argb: 0x33FF_FCF0),
    colorBorderMuted: Color(argb: 0xFF65_6460),
    colorOnAccent: Color(argb: 0xFF07_0707),
    nativeTintShade: 500
  )

  /// The `omarchy-gruvbox-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyGruvboxLight = Palette(
    colorPrimary: Colors.omarchyGruvboxLight,
    colorCanvas: Color(argb: 0xFFE4_E0D8),
    colorSurface: Color(argb: 0xFFF6_F1E8),
    colorSurfaceMuted: Color(argb: 0xFFEE_EAE1),
    colorSurfaceSunken: Color(argb: 0xFFD8_D3CC),
    colorSurfaceSubtle: Color(argb: 0x1028_2828),
    colorSurfaceInset: Color(argb: 0xFFE6_E1D9),
    colorSurfaceRaised: Color(argb: 0xFFF6_F1E8),
    colorSurfaceOverlay: Color(argb: 0xFFF6_F1E8),
    colorSurfaceChrome: Color(argb: 0xFFEE_EAE1),
    colorSurfaceColumn: Color(argb: 0xFFF1_ECE4),
    colorContent: Color(argb: 0xFF28_2828),
    colorContentSecondary: Color(argb: 0xFF4D_4C4B),
    colorContentNav: Color(argb: 0xFF62_605E),
    colorContentMuted: Color(argb: 0xFF6E_6C69),
    colorContentSubtle: Color(argb: 0xFF8F_8D88),
    colorContentFaint: Color(argb: 0xFFAC_A9A3),
    colorBorder: Color(argb: 0x2128_2828),
    colorBorderStrong: Color(argb: 0x2B28_2828),
    colorBorderMuted: Color(argb: 0xFFA8_A59F),
    colorOnAccent: Color(argb: 0xFFF6_F1E8)
  )

  /// The `omarchy-gruvbox-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyGruvboxDark = Palette(
    colorPrimary: Colors.omarchyGruvboxDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF23_2323),
    colorSurface: Color(argb: 0xFF28_2828),
    colorSurfaceMuted: Color(argb: 0xFF2F_2F2D),
    colorSurfaceSunken: Color(argb: 0xFF41_3E38),
    colorSurfaceSubtle: Color(argb: 0x14D4_BE98),
    colorSurfaceInset: Color(argb: 0xFF36_3531),
    colorSurfaceRaised: Color(argb: 0xFF5B_5549),
    colorSurfaceOverlay: Color(argb: 0xFF2F_2F2D),
    colorSurfaceChrome: Color(argb: 0xFF2F_2F2D),
    colorSurfaceColumn: Color(argb: 0xFF24_2424),
    colorContent: Color(argb: 0xFFD4_BE98),
    colorContentSecondary: Color(argb: 0xFFB5_A384),
    colorContentNav: Color(argb: 0xFFA4_9479),
    colorContentMuted: Color(argb: 0xFF9D_8E74),
    colorContentSubtle: Color(argb: 0xFF7E_7360),
    colorContentFaint: Color(argb: 0xFF66_5E50),
    colorBorder: Color(argb: 0x29D4_BE98),
    colorBorderStrong: Color(argb: 0x33D4_BE98),
    colorBorderMuted: Color(argb: 0xFF69_6153),
    colorOnAccent: Color(argb: 0xFF28_2828),
    nativeTintShade: 500
  )

  /// The `omarchy-hackerman-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyHackermanLight = Palette(
    colorPrimary: Colors.omarchyHackermanLight,
    colorCanvas: Color(argb: 0xFFE7_EBEE),
    colorSurface: Color(argb: 0xFFF8_FDFF),
    colorSurfaceMuted: Color(argb: 0xFFF1_F5F8),
    colorSurfaceSunken: Color(argb: 0xFFDA_DEE2),
    colorSurfaceSubtle: Color(argb: 0x0F0B_0C16),
    colorSurfaceInset: Color(argb: 0xFFE8_EDF0),
    colorSurfaceRaised: Color(argb: 0xFFF8_FDFF),
    colorSurfaceOverlay: Color(argb: 0xFFF8_FDFF),
    colorSurfaceChrome: Color(argb: 0xFFF1_F5F8),
    colorSurfaceColumn: Color(argb: 0xFFF4_F8FB),
    colorContent: Color(argb: 0xFF0B_0C16),
    colorContentSecondary: Color(argb: 0xFF36_3740),
    colorContentNav: Color(argb: 0xFF4D_4F57),
    colorContentMuted: Color(argb: 0xFF60_636A),
    colorContentSubtle: Color(argb: 0xFF82_858B),
    colorContentFaint: Color(argb: 0xFFA3_A6AB),
    colorBorder: Color(argb: 0x210B_0C16),
    colorBorderStrong: Color(argb: 0x2B0B_0C16),
    colorBorderMuted: Color(argb: 0xFF9E_A1A6),
    colorOnAccent: Color(argb: 0xFFF8_FDFF)
  )

  /// The `omarchy-hackerman-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyHackermanDark = Palette(
    colorPrimary: Colors.omarchyHackermanDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF05_050A),
    colorSurface: Color(argb: 0xFF0B_0C16),
    colorSurfaceMuted: Color(argb: 0xFF13_141E),
    colorSurfaceSunken: Color(argb: 0xFF1F_232C),
    colorSurfaceSubtle: Color(argb: 0x12DD_F7FF),
    colorSurfaceInset: Color(argb: 0xFF18_1B24),
    colorSurfaceRaised: Color(argb: 0xFF33_3942),
    colorSurfaceOverlay: Color(argb: 0xFF13_141E),
    colorSurfaceChrome: Color(argb: 0xFF13_141E),
    colorSurfaceColumn: Color(argb: 0xFF06_070D),
    colorContent: Color(argb: 0xFFDD_F7FF),
    colorContentSecondary: Color(argb: 0xFFB7_CDD5),
    colorContentNav: Color(argb: 0xFFA2_B5BE),
    colorContentMuted: Color(argb: 0xFF91_A2AB),
    colorContentSubtle: Color(argb: 0xFF74_828B),
    colorContentFaint: Color(argb: 0xFF57_616A),
    colorBorder: Color(argb: 0x29DD_F7FF),
    colorBorderStrong: Color(argb: 0x33DD_F7FF),
    colorBorderMuted: Color(argb: 0xFF5B_656F),
    colorOnAccent: Color(argb: 0xFF0B_0C16),
    nativeTintShade: 500
  )

  /// The `omarchy-kanagawa-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyKanagawaLight = Palette(
    colorPrimary: Colors.omarchyKanagawaLight,
    colorCanvas: Color(argb: 0xFFE5_E4E0),
    colorSurface: Color(argb: 0xFFF7_F6F0),
    colorSurfaceMuted: Color(argb: 0xFFEF_EFE9),
    colorSurfaceSunken: Color(argb: 0xFFD8_D8D4),
    colorSurfaceSubtle: Color(argb: 0x0F1F_1F28),
    colorSurfaceInset: Color(argb: 0xFFE7_E6E2),
    colorSurfaceRaised: Color(argb: 0xFFF7_F6F0),
    colorSurfaceOverlay: Color(argb: 0xFFF7_F6F0),
    colorSurfaceChrome: Color(argb: 0xFFEF_EFE9),
    colorSurfaceColumn: Color(argb: 0xFFF2_F1EC),
    colorContent: Color(argb: 0xFF1F_1F28),
    colorContentSecondary: Color(argb: 0xFF46_464C),
    colorContentNav: Color(argb: 0xFF5B_5B60),
    colorContentMuted: Color(argb: 0xFF6D_6C70),
    colorContentSubtle: Color(argb: 0xFF8B_8B8C),
    colorContentFaint: Color(argb: 0xFFA9_A9A8),
    colorBorder: Color(argb: 0x211F_1F28),
    colorBorderStrong: Color(argb: 0x2B1F_1F28),
    colorBorderMuted: Color(argb: 0xFFA5_A4A4),
    colorOnAccent: Color(argb: 0xFFF7_F6F0)
  )

  /// The `omarchy-kanagawa-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyKanagawaDark = Palette(
    colorPrimary: Colors.omarchyKanagawaDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1B_1B23),
    colorSurface: Color(argb: 0xFF1F_1F28),
    colorSurfaceMuted: Color(argb: 0xFF26_262D),
    colorSurfaceSunken: Color(argb: 0xFF35_3539),
    colorSurfaceSubtle: Color(argb: 0x12DC_D7BA),
    colorSurfaceInset: Color(argb: 0xFF2C_2C32),
    colorSurfaceRaised: Color(argb: 0xFF4D_4C4B),
    colorSurfaceOverlay: Color(argb: 0xFF26_262D),
    colorSurfaceChrome: Color(argb: 0xFF26_262D),
    colorSurfaceColumn: Color(argb: 0xFF1C_1C24),
    colorContent: Color(argb: 0xFFDC_D7BA),
    colorContentSecondary: Color(argb: 0xFFBA_B6A0),
    colorContentNav: Color(argb: 0xFFA7_A391),
    colorContentMuted: Color(argb: 0xFF98_9585),
    colorContentSubtle: Color(argb: 0xFF7E_7B71),
    colorContentFaint: Color(argb: 0xFF63_615D),
    colorBorder: Color(argb: 0x29DC_D7BA),
    colorBorderStrong: Color(argb: 0x33DC_D7BA),
    colorBorderMuted: Color(argb: 0xFF67_655F),
    colorOnAccent: Color(argb: 0xFF1F_1F28),
    nativeTintShade: 500
  )

  /// The `omarchy-last-horizon-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLastHorizonLight = Palette(
    colorPrimary: Colors.omarchyLastHorizonLight,
    colorCanvas: Color(argb: 0xFFED_ECED),
    colorSurface: Color(argb: 0xFFFE_FEFE),
    colorSurfaceMuted: Color(argb: 0xFFF7_F6F7),
    colorSurfaceSunken: Color(argb: 0xFFE0_DFE0),
    colorSurfaceSubtle: Color(argb: 0x0F0C_0B0C),
    colorSurfaceInset: Color(argb: 0xFFEE_EEEE),
    colorSurfaceRaised: Color(argb: 0xFFFE_FEFE),
    colorSurfaceOverlay: Color(argb: 0xFFFE_FEFE),
    colorSurfaceChrome: Color(argb: 0xFFF7_F6F7),
    colorSurfaceColumn: Color(argb: 0xFFFA_F9FA),
    colorContent: Color(argb: 0xFF0C_0B0C),
    colorContentSecondary: Color(argb: 0xFF38_3738),
    colorContentNav: Color(argb: 0xFF50_4F50),
    colorContentMuted: Color(argb: 0xFF63_6263),
    colorContentSubtle: Color(argb: 0xFF85_8485),
    colorContentFaint: Color(argb: 0xFFA7_A7A7),
    colorBorder: Color(argb: 0x210C_0B0C),
    colorBorderStrong: Color(argb: 0x2B0C_0B0C),
    colorBorderMuted: Color(argb: 0xFFA2_A2A2),
    colorOnAccent: Color(argb: 0xFFFE_FEFE)
  )

  /// The `omarchy-last-horizon-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLastHorizonDark = Palette(
    colorPrimary: Colors.omarchyLastHorizonDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF04_0404),
    colorSurface: Color(argb: 0xFF0C_0B0C),
    colorSurfaceMuted: Color(argb: 0xFF14_1414),
    colorSurfaceSunken: Color(argb: 0xFF22_2222),
    colorSurfaceSubtle: Color(argb: 0x12FA_FCFB),
    colorSurfaceInset: Color(argb: 0xFF1A_1A1A),
    colorSurfaceRaised: Color(argb: 0xFF38_3738),
    colorSurfaceOverlay: Color(argb: 0xFF14_1414),
    colorSurfaceChrome: Color(argb: 0xFF14_1414),
    colorSurfaceColumn: Color(argb: 0xFF06_0506),
    colorContent: Color(argb: 0xFFFA_FCFB),
    colorContentSecondary: Color(argb: 0xFFCF_D1D0),
    colorContentNav: Color(argb: 0xFFB7_B9B8),
    colorContentMuted: Color(argb: 0xFFA4_A5A5),
    colorContentSubtle: Color(argb: 0xFF83_8484),
    colorContentFaint: Color(argb: 0xFF62_6262),
    colorBorder: Color(argb: 0x29FA_FCFB),
    colorBorderStrong: Color(argb: 0x33FA_FCFB),
    colorBorderMuted: Color(argb: 0xFF66_6767),
    colorOnAccent: Color(argb: 0xFF0C_0B0C),
    nativeTintShade: 500
  )

  /// The `omarchy-lumon-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLumonLight = Palette(
    colorPrimary: Colors.omarchyLumonLight,
    colorCanvas: Color(argb: 0xFFE4_E8EA),
    colorSurface: Color(argb: 0xFFF6_F9FB),
    colorSurfaceMuted: Color(argb: 0xFFEE_F2F4),
    colorSurfaceSunken: Color(argb: 0xFFD7_DBDE),
    colorSurfaceSubtle: Color(argb: 0x0F16_242D),
    colorSurfaceInset: Color(argb: 0xFFE6_E9EC),
    colorSurfaceRaised: Color(argb: 0xFFF6_F9FB),
    colorSurfaceOverlay: Color(argb: 0xFFF6_F9FB),
    colorSurfaceChrome: Color(argb: 0xFFEE_F2F4),
    colorSurfaceColumn: Color(argb: 0xFFF1_F4F7),
    colorContent: Color(argb: 0xFF16_242D),
    colorContentSecondary: Color(argb: 0xFF3E_4A52),
    colorContentNav: Color(argb: 0xFF55_6067),
    colorContentMuted: Color(argb: 0xFF67_7177),
    colorContentSubtle: Color(argb: 0xFF86_8F94),
    colorContentFaint: Color(argb: 0xFFA5_ACB1),
    colorBorder: Color(argb: 0x2116_242D),
    colorBorderStrong: Color(argb: 0x2B16_242D),
    colorBorderMuted: Color(argb: 0xFFA1_A8AD),
    colorOnAccent: Color(argb: 0xFFF6_F9FB)
  )

  /// The `omarchy-lumon-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLumonDark = Palette(
    colorPrimary: Colors.omarchyLumonDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF13_1F27),
    colorSurface: Color(argb: 0xFF16_242D),
    colorSurfaceMuted: Color(argb: 0xFF1D_2B34),
    colorSurfaceSunken: Color(argb: 0xFF2C_3A43),
    colorSurfaceSubtle: Color(argb: 0x12D6_E2EE),
    colorSurfaceInset: Color(argb: 0xFF23_313A),
    colorSurfaceRaised: Color(argb: 0xFF43_515A),
    colorSurfaceOverlay: Color(argb: 0xFF1D_2B34),
    colorSurfaceChrome: Color(argb: 0xFF1D_2B34),
    colorSurfaceColumn: Color(argb: 0xFF14_2029),
    colorContent: Color(argb: 0xFFD6_E2EE),
    colorContentSecondary: Color(argb: 0xFFB3_C0CB),
    colorContentNav: Color(argb: 0xFFA0_ADB8),
    colorContentMuted: Color(argb: 0xFF91_9EA9),
    colorContentSubtle: Color(argb: 0xFF76_838E),
    colorContentFaint: Color(argb: 0xFF5B_6872),
    colorBorder: Color(argb: 0x29D6_E2EE),
    colorBorderStrong: Color(argb: 0x33D6_E2EE),
    colorBorderMuted: Color(argb: 0xFF5F_6C76),
    colorOnAccent: Color(argb: 0xFF16_242D),
    nativeTintShade: 500
  )

  /// The `omarchy-lupine-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLupineLight = Palette(
    colorPrimary: Colors.omarchyLupineLight,
    colorCanvas: Color(argb: 0xFFE8_E8E8),
    colorSurface: Color(argb: 0xFFFA_FAFA),
    colorSurfaceMuted: Color(argb: 0xFFF2_F2F2),
    colorSurfaceSunken: Color(argb: 0xFFDC_DCDC),
    colorSurfaceSubtle: Color(argb: 0x0F21_2121),
    colorSurfaceInset: Color(argb: 0xFFEA_EAEA),
    colorSurfaceRaised: Color(argb: 0xFFFA_FAFA),
    colorSurfaceOverlay: Color(argb: 0xFFFA_FAFA),
    colorSurfaceChrome: Color(argb: 0xFFF2_F2F2),
    colorSurfaceColumn: Color(argb: 0xFFF5_F5F5),
    colorContent: Color(argb: 0xFF21_2121),
    colorContentSecondary: Color(argb: 0xFF48_4848),
    colorContentNav: Color(argb: 0xFF5E_5E5E),
    colorContentMuted: Color(argb: 0xFF6F_6F6F),
    colorContentSubtle: Color(argb: 0xFF8E_8E8E),
    colorContentFaint: Color(argb: 0xFFAC_ACAC),
    colorBorder: Color(argb: 0x2121_2121),
    colorBorderStrong: Color(argb: 0x2B21_2121),
    colorBorderMuted: Color(argb: 0xFFA8_A8A8),
    colorOnAccent: Color(argb: 0xFFFA_FAFA)
  )

  /// The `omarchy-lupine-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLupineDark = Palette(
    colorPrimary: Colors.omarchyLupineDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF08_0808),
    colorSurface: Color(argb: 0xFF0F_0F0F),
    colorSurfaceMuted: Color(argb: 0xFF17_1717),
    colorSurfaceSunken: Color(argb: 0xFF25_2525),
    colorSurfaceSubtle: Color(argb: 0x12FA_FAFA),
    colorSurfaceInset: Color(argb: 0xFF1D_1D1D),
    colorSurfaceRaised: Color(argb: 0xFF3A_3A3A),
    colorSurfaceOverlay: Color(argb: 0xFF17_1717),
    colorSurfaceChrome: Color(argb: 0xFF17_1717),
    colorSurfaceColumn: Color(argb: 0xFF0A_0A0A),
    colorContent: Color(argb: 0xFFFA_FAFA),
    colorContentSecondary: Color(argb: 0xFFD0_D0D0),
    colorContentNav: Color(argb: 0xFFB8_B8B8),
    colorContentMuted: Color(argb: 0xFFA5_A5A5),
    colorContentSubtle: Color(argb: 0xFF84_8484),
    colorContentFaint: Color(argb: 0xFF64_6464),
    colorBorder: Color(argb: 0x29FA_FAFA),
    colorBorderStrong: Color(argb: 0x33FA_FAFA),
    colorBorderMuted: Color(argb: 0xFF68_6868),
    colorOnAccent: Color(argb: 0xFF0F_0F0F),
    nativeTintShade: 500
  )

  /// The `omarchy-matte-black-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyMatteBlackLight = Palette(
    colorPrimary: Colors.omarchyMatteBlackLight,
    colorCanvas: Color(argb: 0xFFDF_DFDF),
    colorSurface: Color(argb: 0xFFF1_F1F1),
    colorSurfaceMuted: Color(argb: 0xFFE9_E9E9),
    colorSurfaceSunken: Color(argb: 0xFFD3_D3D3),
    colorSurfaceSubtle: Color(argb: 0x0F12_1212),
    colorSurfaceInset: Color(argb: 0xFFE1_E1E1),
    colorSurfaceRaised: Color(argb: 0xFFF1_F1F1),
    colorSurfaceOverlay: Color(argb: 0xFFF1_F1F1),
    colorSurfaceChrome: Color(argb: 0xFFE9_E9E9),
    colorSurfaceColumn: Color(argb: 0xFFEC_ECEC),
    colorContent: Color(argb: 0xFF12_1212),
    colorContentSecondary: Color(argb: 0xFF3A_3A3A),
    colorContentNav: Color(argb: 0xFF50_5050),
    colorContentMuted: Color(argb: 0xFF62_6262),
    colorContentSubtle: Color(argb: 0xFF82_8282),
    colorContentFaint: Color(argb: 0xFFA1_A1A1),
    colorBorder: Color(argb: 0x2112_1212),
    colorBorderStrong: Color(argb: 0x2B12_1212),
    colorBorderMuted: Color(argb: 0xFF9C_9C9C),
    colorOnAccent: Color(argb: 0xFFF1_F1F1)
  )

  /// The `omarchy-matte-black-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyMatteBlackDark = Palette(
    colorPrimary: Colors.omarchyMatteBlackDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0C_0C0C),
    colorSurface: Color(argb: 0xFF12_1212),
    colorSurfaceMuted: Color(argb: 0xFF19_1919),
    colorSurfaceSunken: Color(argb: 0xFF27_2727),
    colorSurfaceSubtle: Color(argb: 0x12BE_BEBE),
    colorSurfaceInset: Color(argb: 0xFF1F_1F1F),
    colorSurfaceRaised: Color(argb: 0xFF3D_3D3D),
    colorSurfaceOverlay: Color(argb: 0xFF19_1919),
    colorSurfaceChrome: Color(argb: 0xFF19_1919),
    colorSurfaceColumn: Color(argb: 0xFF0D_0D0D),
    colorContent: Color(argb: 0xFFBE_BEBE),
    colorContentSecondary: Color(argb: 0xFF9F_9F9F),
    colorContentNav: Color(argb: 0xFF8E_8E8E),
    colorContentMuted: Color(argb: 0xFF80_8080),
    colorContentSubtle: Color(argb: 0xFF68_6868),
    colorContentFaint: Color(argb: 0xFF50_5050),
    colorBorder: Color(argb: 0x29BE_BEBE),
    colorBorderStrong: Color(argb: 0x33BE_BEBE),
    colorBorderMuted: Color(argb: 0xFF53_5353),
    colorOnAccent: Color(argb: 0xFF12_1212),
    nativeTintShade: 500
  )

  /// The `omarchy-miasma-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyMiasmaLight = Palette(
    colorPrimary: Colors.omarchyMiasmaLight,
    colorCanvas: Color(argb: 0xFFE0_E0DD),
    colorSurface: Color(argb: 0xFFF2_F2EE),
    colorSurfaceMuted: Color(argb: 0xFFEA_EAE7),
    colorSurfaceSunken: Color(argb: 0xFFD4_D4D1),
    colorSurfaceSubtle: Color(argb: 0x1022_2222),
    colorSurfaceInset: Color(argb: 0xFFE2_E2DF),
    colorSurfaceRaised: Color(argb: 0xFFF2_F2EE),
    colorSurfaceOverlay: Color(argb: 0xFFF2_F2EE),
    colorSurfaceChrome: Color(argb: 0xFFEA_EAE7),
    colorSurfaceColumn: Color(argb: 0xFFED_EDEA),
    colorContent: Color(argb: 0xFF22_2222),
    colorContentSecondary: Color(argb: 0xFF47_4747),
    colorContentNav: Color(argb: 0xFF5C_5C5B),
    colorContentMuted: Color(argb: 0xFF6D_6D6B),
    colorContentSubtle: Color(argb: 0xFF8A_8A88),
    colorContentFaint: Color(argb: 0xFFA7_A7A5),
    colorBorder: Color(argb: 0x2122_2222),
    colorBorderStrong: Color(argb: 0x2B22_2222),
    colorBorderMuted: Color(argb: 0xFFA3_A3A0),
    colorOnAccent: Color(argb: 0xFFF2_F2EE)
  )

  /// The `omarchy-miasma-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyMiasmaDark = Palette(
    colorPrimary: Colors.omarchyMiasmaDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1D_1D1D),
    colorSurface: Color(argb: 0xFF22_2222),
    colorSurfaceMuted: Color(argb: 0xFF29_2928),
    colorSurfaceSunken: Color(argb: 0xFF38_3835),
    colorSurfaceSubtle: Color(argb: 0x13C2_C2B0),
    colorSurfaceInset: Color(argb: 0xFF2F_2F2D),
    colorSurfaceRaised: Color(argb: 0xFF4F_4F49),
    colorSurfaceOverlay: Color(argb: 0xFF29_2928),
    colorSurfaceChrome: Color(argb: 0xFF29_2928),
    colorSurfaceColumn: Color(argb: 0xFF1E_1E1E),
    colorContent: Color(argb: 0xFFC2_C2B0),
    colorContentSecondary: Color(argb: 0xFFA5_A596),
    colorContentNav: Color(argb: 0xFF95_9588),
    colorContentMuted: Color(argb: 0xFF8C_8C80),
    colorContentSubtle: Color(argb: 0xFF72_7269),
    colorContentFaint: Color(argb: 0xFF5C_5C55),
    colorBorder: Color(argb: 0x29C2_C2B0),
    colorBorderStrong: Color(argb: 0x33C2_C2B0),
    colorBorderMuted: Color(argb: 0xFF5F_5F58),
    colorOnAccent: Color(argb: 0xFF22_2222),
    nativeTintShade: 500
  )

  /// The `omarchy-nord-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyNordLight = Palette(
    colorPrimary: Colors.omarchyNordLight,
    colorCanvas: Color(argb: 0xFFE4_E7E9),
    colorSurface: Color(argb: 0xFFF6_F8FA),
    colorSurfaceMuted: Color(argb: 0xFFEE_F1F3),
    colorSurfaceSunken: Color(argb: 0xFFD7_DADE),
    colorSurfaceSubtle: Color(argb: 0x112E_3440),
    colorSurfaceInset: Color(argb: 0xFFE6_E8EB),
    colorSurfaceRaised: Color(argb: 0xFFF6_F8FA),
    colorSurfaceOverlay: Color(argb: 0xFFF6_F8FA),
    colorSurfaceChrome: Color(argb: 0xFFEE_F1F3),
    colorSurfaceColumn: Color(argb: 0xFFF1_F3F6),
    colorContent: Color(argb: 0xFF2E_3440),
    colorContentSecondary: Color(argb: 0xFF52_5761),
    colorContentNav: Color(argb: 0xFF66_6B74),
    colorContentMuted: Color(argb: 0xFF6C_717A),
    colorContentSubtle: Color(argb: 0xFF92_969D),
    colorContentFaint: Color(argb: 0xFFAE_B1B7),
    colorBorder: Color(argb: 0x212E_3440),
    colorBorderStrong: Color(argb: 0x2B2E_3440),
    colorBorderMuted: Color(argb: 0xFFAA_AEB3),
    colorOnAccent: Color(argb: 0xFFF6_F8FA)
  )

  /// The `omarchy-nord-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyNordDark = Palette(
    colorPrimary: Colors.omarchyNordDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF2A_2F3A),
    colorSurface: Color(argb: 0xFF2E_3440),
    colorSurfaceMuted: Color(argb: 0xFF35_3B46),
    colorSurfaceSunken: Color(argb: 0xFF45_4B56),
    colorSurfaceSubtle: Color(argb: 0x13D8_DEE9),
    colorSurfaceInset: Color(argb: 0xFF3C_424D),
    colorSurfaceRaised: Color(argb: 0xFF5D_636E),
    colorSurfaceOverlay: Color(argb: 0xFF35_3B46),
    colorSurfaceChrome: Color(argb: 0xFF35_3B46),
    colorSurfaceColumn: Color(argb: 0xFF2B_303C),
    colorContent: Color(argb: 0xFFD8_DEE9),
    colorContentSecondary: Color(argb: 0xFFB9_BFCB),
    colorContentNav: Color(argb: 0xFFA8_AEBA),
    colorContentMuted: Color(argb: 0xFF9B_A1AC),
    colorContentSubtle: Color(argb: 0xFF83_8994),
    colorContentFaint: Color(argb: 0xFF6B_717D),
    colorBorder: Color(argb: 0x29D8_DEE9),
    colorBorderStrong: Color(argb: 0x33D8_DEE9),
    colorBorderMuted: Color(argb: 0xFF6F_7580),
    colorOnAccent: Color(argb: 0xFF2E_3440),
    nativeTintShade: 500
  )

  /// The `omarchy-osaka-jade-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyOsakaJadeLight = Palette(
    colorPrimary: Colors.omarchyOsakaJadeLight,
    colorCanvas: Color(argb: 0xFFDF_E1D8),
    colorSurface: Color(argb: 0xFFF1_F2E8),
    colorSurfaceMuted: Color(argb: 0xFFE9_EBE1),
    colorSurfaceSunken: Color(argb: 0xFFD2_D4CB),
    colorSurfaceSubtle: Color(argb: 0x0F11_1C18),
    colorSurfaceInset: Color(argb: 0xFFE1_E2D9),
    colorSurfaceRaised: Color(argb: 0xFFF1_F2E8),
    colorSurfaceOverlay: Color(argb: 0xFFF1_F2E8),
    colorSurfaceChrome: Color(argb: 0xFFE9_EBE1),
    colorSurfaceColumn: Color(argb: 0xFFEC_EDE4),
    colorContent: Color(argb: 0xFF11_1C18),
    colorContentSecondary: Color(argb: 0xFF39_433D),
    colorContentNav: Color(argb: 0xFF50_5852),
    colorContentMuted: Color(argb: 0xFF62_6963),
    colorContentSubtle: Color(argb: 0xFF81_8780),
    colorContentFaint: Color(argb: 0xFFA0_A59D),
    colorBorder: Color(argb: 0x2111_1C18),
    colorBorderStrong: Color(argb: 0x2B11_1C18),
    colorBorderMuted: Color(argb: 0xFF9C_A199),
    colorOnAccent: Color(argb: 0xFFF1_F2E8)
  )

  /// The `omarchy-osaka-jade-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyOsakaJadeDark = Palette(
    colorPrimary: Colors.omarchyOsakaJadeDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0E_1714),
    colorSurface: Color(argb: 0xFF11_1C18),
    colorSurfaceMuted: Color(argb: 0xFF18_221D),
    colorSurfaceSunken: Color(argb: 0xFF27_3128),
    colorSurfaceSubtle: Color(argb: 0x12C1_C497),
    colorSurfaceInset: Color(argb: 0xFF1E_2921),
    colorSurfaceRaised: Color(argb: 0xFF3F_4839),
    colorSurfaceOverlay: Color(argb: 0xFF18_221D),
    colorSurfaceChrome: Color(argb: 0xFF18_221D),
    colorSurfaceColumn: Color(argb: 0xFF0F_1815),
    colorContent: Color(argb: 0xFFC1_C497),
    colorContentSecondary: Color(argb: 0xFFA1_A680),
    colorContentNav: Color(argb: 0xFF90_9573),
    colorContentMuted: Color(argb: 0xFF82_8869),
    colorContentSubtle: Color(argb: 0xFF69_7058),
    colorContentFaint: Color(argb: 0xFF50_5846),
    colorBorder: Color(argb: 0x29C1_C497),
    colorBorderStrong: Color(argb: 0x33C1_C497),
    colorBorderMuted: Color(argb: 0xFF54_5C48),
    colorOnAccent: Color(argb: 0xFF11_1C18),
    nativeTintShade: 500
  )

  /// The `omarchy-retro-82-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRetro82Light = Palette(
    colorPrimary: Colors.omarchyRetro82Light,
    colorCanvas: Color(argb: 0xFFEA_E6DF),
    colorSurface: Color(argb: 0xFFFD_F7ED),
    colorSurfaceMuted: Color(argb: 0xFFF5_EFE7),
    colorSurfaceSunken: Color(argb: 0xFFDC_D9D4),
    colorSurfaceSubtle: Color(argb: 0x0F05_182E),
    colorSurfaceInset: Color(argb: 0xFFEC_E7E0),
    colorSurfaceRaised: Color(argb: 0xFFFD_F7ED),
    colorSurfaceOverlay: Color(argb: 0xFFFD_F7ED),
    colorSurfaceChrome: Color(argb: 0xFFF5_EFE7),
    colorSurfaceColumn: Color(argb: 0xFFF8_F2E9),
    colorContent: Color(argb: 0xFF05_182E),
    colorContentSecondary: Color(argb: 0xFF32_4050),
    colorContentNav: Color(argb: 0xFF4A_5663),
    colorContentMuted: Color(argb: 0xFF5E_6873),
    colorContentSubtle: Color(argb: 0xFF81_888E),
    colorContentFaint: Color(argb: 0xFFA4_A7A8),
    colorBorder: Color(argb: 0x2105_182E),
    colorBorderStrong: Color(argb: 0x2B05_182E),
    colorBorderMuted: Color(argb: 0xFF9F_A2A4),
    colorOnAccent: Color(argb: 0xFFFD_F7ED)
  )

  /// The `omarchy-retro-82-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRetro82Dark = Palette(
    colorPrimary: Colors.omarchyRetro82Dark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF04_1425),
    colorSurface: Color(argb: 0xFF05_182E),
    colorSurfaceMuted: Color(argb: 0xFF0D_1F32),
    colorSurfaceSunken: Color(argb: 0xFF1F_2E3C),
    colorSurfaceSubtle: Color(argb: 0x12F6_DCAC),
    colorSurfaceInset: Color(argb: 0xFF14_2536),
    colorSurfaceRaised: Color(argb: 0xFF3A_444A),
    colorSurfaceOverlay: Color(argb: 0xFF0D_1F32),
    colorSurfaceChrome: Color(argb: 0xFF0D_1F32),
    colorSurfaceColumn: Color(argb: 0xFF04_1528),
    colorContent: Color(argb: 0xFFF6_DCAC),
    colorContentSecondary: Color(argb: 0xFFCB_B995),
    colorContentNav: Color(argb: 0xFFB3_A589),
    colorContentMuted: Color(argb: 0xFF9F_957F),
    colorContentSubtle: Color(argb: 0xFF7E_7A6D),
    colorContentFaint: Color(argb: 0xFF5C_5F5B),
    colorBorder: Color(argb: 0x29F6_DCAC),
    colorBorderStrong: Color(argb: 0x33F6_DCAC),
    colorBorderMuted: Color(argb: 0xFF61_625E),
    colorOnAccent: Color(argb: 0xFF05_182E),
    nativeTintShade: 500
  )

  /// The `omarchy-ristretto-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRistrettoLight = Palette(
    colorPrimary: Colors.omarchyRistrettoLight,
    colorCanvas: Color(argb: 0xFFE9_E5E5),
    colorSurface: Color(argb: 0xFFFA_F7F7),
    colorSurfaceMuted: Color(argb: 0xFFF3_EFEF),
    colorSurfaceSunken: Color(argb: 0xFFDC_D9D9),
    colorSurfaceSubtle: Color(argb: 0x102C_2525),
    colorSurfaceInset: Color(argb: 0xFFEB_E7E7),
    colorSurfaceRaised: Color(argb: 0xFFFA_F7F7),
    colorSurfaceOverlay: Color(argb: 0xFFFA_F7F7),
    colorSurfaceChrome: Color(argb: 0xFFF3_EFEF),
    colorSurfaceColumn: Color(argb: 0xFFF6_F2F2),
    colorContent: Color(argb: 0xFF2C_2525),
    colorContentSecondary: Color(argb: 0xFF51_4B4B),
    colorContentNav: Color(argb: 0xFF66_6060),
    colorContentMuted: Color(argb: 0xFF74_6F6F),
    colorContentSubtle: Color(argb: 0xFF93_8E8E),
    colorContentFaint: Color(argb: 0xFFB0_ABAB),
    colorBorder: Color(argb: 0x212C_2525),
    colorBorderStrong: Color(argb: 0x2B2C_2525),
    colorBorderMuted: Color(argb: 0xFFAC_A7A7),
    colorOnAccent: Color(argb: 0xFFFA_F7F7)
  )

  /// The `omarchy-ristretto-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRistrettoDark = Palette(
    colorPrimary: Colors.omarchyRistrettoDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF27_2121),
    colorSurface: Color(argb: 0xFF2C_2525),
    colorSurfaceMuted: Color(argb: 0xFF33_2C2C),
    colorSurfaceSunken: Color(argb: 0xFF42_3B3B),
    colorSurfaceSubtle: Color(argb: 0x12E6_D9DB),
    colorSurfaceInset: Color(argb: 0xFF39_3232),
    colorSurfaceRaised: Color(argb: 0xFF5A_5252),
    colorSurfaceOverlay: Color(argb: 0xFF33_2C2C),
    colorSurfaceChrome: Color(argb: 0xFF33_2C2C),
    colorSurfaceColumn: Color(argb: 0xFF28_2222),
    colorContent: Color(argb: 0xFFE6_D9DB),
    colorContentSecondary: Color(argb: 0xFFC5_B9BA),
    colorContentNav: Color(argb: 0xFFB2_A7A8),
    colorContentMuted: Color(argb: 0xFFA3_9899),
    colorContentSubtle: Color(argb: 0xFF89_7F80),
    colorContentFaint: Color(argb: 0xFF6F_6667),
    colorBorder: Color(argb: 0x29E6_D9DB),
    colorBorderStrong: Color(argb: 0x33E6_D9DB),
    colorBorderMuted: Color(argb: 0xFF73_696A),
    colorOnAccent: Color(argb: 0xFF2C_2525),
    nativeTintShade: 500
  )

  /// The `omarchy-rose-pine-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRosePineLight = Palette(
    colorPrimary: Colors.omarchyRosePineLight,
    colorCanvas: Color(argb: 0xFFE8_E2E0),
    colorSurface: Color(argb: 0xFFFA_F4ED),
    colorSurfaceMuted: Color(argb: 0xFFF2_ECE8),
    colorSurfaceSunken: Color(argb: 0xFFDB_D5D7),
    colorSurfaceSubtle: Color(argb: 0x1657_5279),
    colorSurfaceInset: Color(argb: 0xFFE9_E4E1),
    colorSurfaceRaised: Color(argb: 0xFFFA_F4ED),
    colorSurfaceOverlay: Color(argb: 0xFFFA_F4ED),
    colorSurfaceChrome: Color(argb: 0xFFF2_ECE8),
    colorSurfaceColumn: Color(argb: 0xFFF5_EFEA),
    colorContent: Color(argb: 0xFF57_5279),
    colorContentSecondary: Color(argb: 0xFF6F_6A8A),
    colorContentNav: Color(argb: 0xFF6F_6A8A),
    colorContentMuted: Color(argb: 0xFF6F_6A8A),
    colorContentSubtle: Color(argb: 0xFFA9_A3B3),
    colorContentFaint: Color(argb: 0xFFBF_BAC3),
    colorBorder: Color(argb: 0x2157_5279),
    colorBorderStrong: Color(argb: 0x2B57_5279),
    colorBorderMuted: Color(argb: 0xFFBC_B6C1),
    colorOnAccent: Color(argb: 0xFFFA_F4ED)
  )

  /// The `omarchy-rose-pine-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRosePineDark = Palette(
    colorPrimary: Colors.omarchyRosePineDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF23_2130),
    colorSurface: Color(argb: 0xFF27_2536),
    colorSurfaceMuted: Color(argb: 0xFF2E_2C3C),
    colorSurfaceSunken: Color(argb: 0xFF3D_3B4A),
    colorSurfaceSubtle: Color(argb: 0x12FA_F4ED),
    colorSurfaceInset: Color(argb: 0xFF34_3242),
    colorSurfaceRaised: Color(argb: 0xFF55_535E),
    colorSurfaceOverlay: Color(argb: 0xFF2E_2C3C),
    colorSurfaceChrome: Color(argb: 0xFF2E_2C3C),
    colorSurfaceColumn: Color(argb: 0xFF24_2231),
    colorContent: Color(argb: 0xFFFA_F4ED),
    colorContentSecondary: Color(argb: 0xFFD4_CFCC),
    colorContentNav: Color(argb: 0xFFBF_BABA),
    colorContentMuted: Color(argb: 0xFFAE_A9AB),
    colorContentSubtle: Color(argb: 0xFF91_8D92),
    colorContentFaint: Color(argb: 0xFF73_7078),
    colorBorder: Color(argb: 0x29FA_F4ED),
    colorBorderStrong: Color(argb: 0x33FA_F4ED),
    colorBorderMuted: Color(argb: 0xFF77_747C),
    colorOnAccent: Color(argb: 0xFF27_2536),
    nativeTintShade: 500
  )

  /// The `omarchy-solitude-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchySolitudeLight = Palette(
    colorPrimary: Colors.omarchySolitudeLight,
    colorCanvas: Color(argb: 0xFFE1_E3E3),
    colorSurface: Color(argb: 0xFFF3_F4F4),
    colorSurfaceMuted: Color(argb: 0xFFEB_EDED),
    colorSurfaceSunken: Color(argb: 0xFFD5_D6D7),
    colorSurfaceSubtle: Color(argb: 0x0F10_1315),
    colorSurfaceInset: Color(argb: 0xFFE3_E4E5),
    colorSurfaceRaised: Color(argb: 0xFFF3_F4F4),
    colorSurfaceOverlay: Color(argb: 0xFFF3_F4F4),
    colorSurfaceChrome: Color(argb: 0xFFEB_EDED),
    colorSurfaceColumn: Color(argb: 0xFFEE_EFF0),
    colorContent: Color(argb: 0xFF10_1315),
    colorContentSecondary: Color(argb: 0xFF39_3C3D),
    colorContentNav: Color(argb: 0xFF50_5253),
    colorContentMuted: Color(argb: 0xFF62_6465),
    colorContentSubtle: Color(argb: 0xFF82_8485),
    colorContentFaint: Color(argb: 0xFFA1_A3A4),
    colorBorder: Color(argb: 0x2110_1315),
    colorBorderStrong: Color(argb: 0x2B10_1315),
    colorBorderMuted: Color(argb: 0xFF9D_9F9F),
    colorOnAccent: Color(argb: 0xFFF3_F4F4)
  )

  /// The `omarchy-solitude-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchySolitudeDark = Palette(
    colorPrimary: Colors.omarchySolitudeDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0B_0D0F),
    colorSurface: Color(argb: 0xFF10_1315),
    colorSurfaceMuted: Color(argb: 0xFF17_1A1B),
    colorSurfaceSunken: Color(argb: 0xFF25_2829),
    colorSurfaceSubtle: Color(argb: 0x12CA_CCCC),
    colorSurfaceInset: Color(argb: 0xFF1D_2021),
    colorSurfaceRaised: Color(argb: 0xFF3B_3E3F),
    colorSurfaceOverlay: Color(argb: 0xFF17_1A1B),
    colorSurfaceChrome: Color(argb: 0xFF17_1A1B),
    colorSurfaceColumn: Color(argb: 0xFF0C_0F10),
    colorContent: Color(argb: 0xFFCA_CCCC),
    colorContentSecondary: Color(argb: 0xFFA9_ABAB),
    colorContentNav: Color(argb: 0xFF96_9899),
    colorContentMuted: Color(argb: 0xFF87_898A),
    colorContentSubtle: Color(argb: 0xFF6D_7071),
    colorContentFaint: Color(argb: 0xFF53_5657),
    colorBorder: Color(argb: 0x29CA_CCCC),
    colorBorderStrong: Color(argb: 0x33CA_CCCC),
    colorBorderMuted: Color(argb: 0xFF57_595B),
    colorOnAccent: Color(argb: 0xFF10_1315),
    nativeTintShade: 500
  )

  /// The `omarchy-vantablack-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyVantablackLight = Palette(
    colorPrimary: Colors.omarchyVantablackLight,
    colorCanvas: Color(argb: 0xFFED_EDED),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF7_F7F7),
    colorSurfaceSunken: Color(argb: 0xFFE0_E0E0),
    colorSurfaceSubtle: Color(argb: 0x0F00_0000),
    colorSurfaceInset: Color(argb: 0xFFEF_EFEF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF7_F7F7),
    colorSurfaceColumn: Color(argb: 0xFFFA_FAFA),
    colorContent: Color(argb: 0xFF00_0000),
    colorContentSecondary: Color(argb: 0xFF2E_2E2E),
    colorContentNav: Color(argb: 0xFF47_4747),
    colorContentMuted: Color(argb: 0xFF5C_5C5C),
    colorContentSubtle: Color(argb: 0xFF80_8080),
    colorContentFaint: Color(argb: 0xFFA3_A3A3),
    colorBorder: Color(argb: 0x2100_0000),
    colorBorderStrong: Color(argb: 0x2B00_0000),
    colorBorderMuted: Color(argb: 0xFF9E_9E9E),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `omarchy-vantablack-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyVantablackDark = Palette(
    colorPrimary: Colors.omarchyVantablackDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF08_0808),
    colorSurface: Color(argb: 0xFF00_0000),
    colorSurfaceMuted: Color(argb: 0xFF0B_0B0B),
    colorSurfaceSunken: Color(argb: 0xFF1C_1C1C),
    colorSurfaceSubtle: Color(argb: 0x13FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF14_1414),
    colorSurfaceRaised: Color(argb: 0xFF31_3131),
    colorSurfaceOverlay: Color(argb: 0xFF0B_0B0B),
    colorSurfaceChrome: Color(argb: 0xFF0B_0B0B),
    colorSurfaceColumn: Color(argb: 0xFF06_0606),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD1_D1D1),
    colorContentNav: Color(argb: 0xFFB8_B8B8),
    colorContentMuted: Color(argb: 0xFFA3_A3A3),
    colorContentSubtle: Color(argb: 0xFF80_8080),
    colorContentFaint: Color(argb: 0xFF5C_5C5C),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF61_6161),
    colorOnAccent: Color(argb: 0xFF00_0000),
    nativeTintShade: 500
  )

  /// The `omarchy-white-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyWhiteLight = Palette(
    colorPrimary: Colors.omarchyWhiteLight,
    colorCanvas: Color(argb: 0xFFED_EDED),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF7_F7F7),
    colorSurfaceSunken: Color(argb: 0xFFE0_E0E0),
    colorSurfaceSubtle: Color(argb: 0x0F00_0000),
    colorSurfaceInset: Color(argb: 0xFFEF_EFEF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF7_F7F7),
    colorSurfaceColumn: Color(argb: 0xFFFA_FAFA),
    colorContent: Color(argb: 0xFF00_0000),
    colorContentSecondary: Color(argb: 0xFF2E_2E2E),
    colorContentNav: Color(argb: 0xFF47_4747),
    colorContentMuted: Color(argb: 0xFF5C_5C5C),
    colorContentSubtle: Color(argb: 0xFF80_8080),
    colorContentFaint: Color(argb: 0xFFA3_A3A3),
    colorBorder: Color(argb: 0x2100_0000),
    colorBorderStrong: Color(argb: 0x2B00_0000),
    colorBorderMuted: Color(argb: 0xFF9E_9E9E),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `omarchy-white-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyWhiteDark = Palette(
    colorPrimary: Colors.omarchyWhiteDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF08_0808),
    colorSurface: Color(argb: 0xFF00_0000),
    colorSurfaceMuted: Color(argb: 0xFF0B_0B0B),
    colorSurfaceSunken: Color(argb: 0xFF1C_1C1C),
    colorSurfaceSubtle: Color(argb: 0x13FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF14_1414),
    colorSurfaceRaised: Color(argb: 0xFF31_3131),
    colorSurfaceOverlay: Color(argb: 0xFF0B_0B0B),
    colorSurfaceChrome: Color(argb: 0xFF0B_0B0B),
    colorSurfaceColumn: Color(argb: 0xFF06_0606),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD1_D1D1),
    colorContentNav: Color(argb: 0xFFB8_B8B8),
    colorContentMuted: Color(argb: 0xFFA3_A3A3),
    colorContentSubtle: Color(argb: 0xFF80_8080),
    colorContentFaint: Color(argb: 0xFF5C_5C5C),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF61_6161),
    colorOnAccent: Color(argb: 0xFF00_0000),
    nativeTintShade: 500
  )

  /// The `macos27-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let macos27Light = Palette(
    colorPrimary: Colors.macos27Light,
    colorCanvas: Color(argb: 0xFFE3_E3E5),
    colorSurface: Color(argb: 0xFFF5_F5F7),
    colorSurfaceMuted: Color(argb: 0xFFED_EDEF),
    colorSurfaceSunken: Color(argb: 0xFFD7_D7D9),
    colorSurfaceSubtle: Color(argb: 0x1025_2525),
    colorSurfaceInset: Color(argb: 0xFFE5_E5E7),
    colorSurfaceRaised: Color(argb: 0xFFF5_F5F7),
    colorSurfaceOverlay: Color(argb: 0xFFF5_F5F7),
    colorSurfaceChrome: Color(argb: 0xFFED_EDEF),
    colorSurfaceColumn: Color(argb: 0xFFF0_F0F2),
    colorContent: Color(argb: 0xFF25_2525),
    colorContentSecondary: Color(argb: 0xFF4A_4A4B),
    colorContentNav: Color(argb: 0xFF5F_5F60),
    colorContentMuted: Color(argb: 0xFF6D_6D6E),
    colorContentSubtle: Color(argb: 0xFF8D_8D8E),
    colorContentFaint: Color(argb: 0xFFAA_AAAB),
    colorBorder: Color(argb: 0x1225_2525),
    colorBorderStrong: Color(argb: 0x1A25_2525),
    colorBorderMuted: Color(argb: 0xFFA6_A6A7),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `macos27-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let macos27Dark = Palette(
    colorPrimary: Colors.macos27Dark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1E_1E21),
    colorSurface: Color(argb: 0xFF23_2326),
    colorSurfaceMuted: Color(argb: 0xFF2A_2A2C),
    colorSurfaceSunken: Color(argb: 0xFF39_393B),
    colorSurfaceSubtle: Color(argb: 0x12DE_DEDF),
    colorSurfaceInset: Color(argb: 0xFF30_3032),
    colorSurfaceRaised: Color(argb: 0xFF50_5052),
    colorSurfaceOverlay: Color(argb: 0xFF2A_2A2C),
    colorSurfaceChrome: Color(argb: 0xFF2A_2A2C),
    colorSurfaceColumn: Color(argb: 0xFF20_2022),
    colorContent: Color(argb: 0xFFDE_DEDF),
    colorContentSecondary: Color(argb: 0xFFBD_BDBD),
    colorContentNav: Color(argb: 0xFFAA_AAAB),
    colorContentMuted: Color(argb: 0xFF9B_9B9C),
    colorContentSubtle: Color(argb: 0xFF81_8182),
    colorContentFaint: Color(argb: 0xFF66_6668),
    colorBorder: Color(argb: 0x14DE_DEDF),
    colorBorderStrong: Color(argb: 0x1CDE_DEDF),
    colorBorderMuted: Color(argb: 0xFF6A_6A6C),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    nativeTintShade: 500
  )

  /// The `macos15-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let macos15Light = Palette(
    colorPrimary: Colors.macos15Light,
    colorCanvas: Color(argb: 0xFFDA_DADA),
    colorSurface: Color(argb: 0xFFEC_ECEC),
    colorSurfaceMuted: Color(argb: 0xFFE4_E4E4),
    colorSurfaceSunken: Color(argb: 0xFFCE_CECE),
    colorSurfaceSubtle: Color(argb: 0x1023_2323),
    colorSurfaceInset: Color(argb: 0xFFDC_DCDC),
    colorSurfaceRaised: Color(argb: 0xFFEC_ECEC),
    colorSurfaceOverlay: Color(argb: 0xFFEC_ECEC),
    colorSurfaceChrome: Color(argb: 0xFFE4_E4E4),
    colorSurfaceColumn: Color(argb: 0xFFE7_E7E7),
    colorContent: Color(argb: 0xFF23_2323),
    colorContentSecondary: Color(argb: 0xFF47_4747),
    colorContentNav: Color(argb: 0xFF5B_5B5B),
    colorContentMuted: Color(argb: 0xFF69_6969),
    colorContentSubtle: Color(argb: 0xFF88_8888),
    colorContentFaint: Color(argb: 0xFFA4_A4A4),
    colorBorder: Color(argb: 0x1A23_2323),
    colorBorderStrong: Color(argb: 0x2123_2323),
    colorBorderMuted: Color(argb: 0xFFA0_A0A0),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `macos15-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let macos15Dark = Palette(
    colorPrimary: Colors.macos15Dark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF2D_2D2D),
    colorSurface: Color(argb: 0xFF32_3232),
    colorSurfaceMuted: Color(argb: 0xFF39_3939),
    colorSurfaceSunken: Color(argb: 0xFF49_4949),
    colorSurfaceSubtle: Color(argb: 0x13E0_E0E0),
    colorSurfaceInset: Color(argb: 0xFF40_4040),
    colorSurfaceRaised: Color(argb: 0xFF61_6161),
    colorSurfaceOverlay: Color(argb: 0xFF39_3939),
    colorSurfaceChrome: Color(argb: 0xFF39_3939),
    colorSurfaceColumn: Color(argb: 0xFF2E_2E2E),
    colorContent: Color(argb: 0xFFE0_E0E0),
    colorContentSecondary: Color(argb: 0xFFC1_C1C1),
    colorContentNav: Color(argb: 0xFFB0_B0B0),
    colorContentMuted: Color(argb: 0xFFA2_A2A2),
    colorContentSubtle: Color(argb: 0xFF89_8989),
    colorContentFaint: Color(argb: 0xFF71_7171),
    colorBorder: Color(argb: 0x1AE0_E0E0),
    colorBorderStrong: Color(argb: 0x21E0_E0E0),
    colorBorderMuted: Color(argb: 0xFF74_7474),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    nativeTintShade: 500
  )

  /// The `windows11-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let windows11Light = Palette(
    colorPrimary: Colors.windows11Light,
    colorCanvas: Color(argb: 0xFFE1_E1E1),
    colorSurface: Color(argb: 0xFFF3_F3F3),
    colorSurfaceMuted: Color(argb: 0xFFEB_EBEB),
    colorSurfaceSunken: Color(argb: 0xFFD5_D5D5),
    colorSurfaceSubtle: Color(argb: 0x0F18_1818),
    colorSurfaceInset: Color(argb: 0xFFE3_E3E3),
    colorSurfaceRaised: Color(argb: 0xFFF3_F3F3),
    colorSurfaceOverlay: Color(argb: 0xFFF3_F3F3),
    colorSurfaceChrome: Color(argb: 0xFFEB_EBEB),
    colorSurfaceColumn: Color(argb: 0xFFEE_EEEE),
    colorContent: Color(argb: 0xFF18_1818),
    colorContentSecondary: Color(argb: 0xFF3F_3F3F),
    colorContentNav: Color(argb: 0xFF55_5555),
    colorContentMuted: Color(argb: 0xFF67_6767),
    colorContentSubtle: Color(argb: 0xFF85_8585),
    colorContentFaint: Color(argb: 0xFFA4_A4A4),
    colorBorder: Color(argb: 0x1418_1818),
    colorBorderStrong: Color(argb: 0x1C18_1818),
    colorBorderMuted: Color(argb: 0xFFA0_A0A0),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `windows11-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let windows11Dark = Palette(
    colorPrimary: Colors.windows11Dark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1B_1B1B),
    colorSurface: Color(argb: 0xFF20_2020),
    colorSurfaceMuted: Color(argb: 0xFF27_2727),
    colorSurfaceSunken: Color(argb: 0xFF36_3636),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF2D_2D2D),
    colorSurfaceRaised: Color(argb: 0xFF4C_4C4C),
    colorSurfaceOverlay: Color(argb: 0xFF27_2727),
    colorSurfaceChrome: Color(argb: 0xFF27_2727),
    colorSurfaceColumn: Color(argb: 0xFF1C_1C1C),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD7_D7D7),
    colorContentNav: Color(argb: 0xFFC1_C1C1),
    colorContentMuted: Color(argb: 0xFFAF_AFAF),
    colorContentSubtle: Color(argb: 0xFF90_9090),
    colorContentFaint: Color(argb: 0xFF70_7070),
    colorBorder: Color(argb: 0x14FF_FFFF),
    colorBorderStrong: Color(argb: 0x1CFF_FFFF),
    colorBorderMuted: Color(argb: 0xFF75_7575),
    colorOnAccent: Color(argb: 0xFF00_0000),
    nativeTintShade: 500
  )

  /// The `ubuntu-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let ubuntuLight = Palette(
    colorPrimary: Colors.ubuntuLight,
    colorCanvas: Color(argb: 0xFFE8_E8E8),
    colorSurface: Color(argb: 0xFFFA_FAFA),
    colorSurfaceMuted: Color(argb: 0xFFF2_F2F2),
    colorSurfaceSunken: Color(argb: 0xFFDC_DCDC),
    colorSurfaceSubtle: Color(argb: 0x113D_3D3D),
    colorSurfaceInset: Color(argb: 0xFFEA_EAEA),
    colorSurfaceRaised: Color(argb: 0xFFFA_FAFA),
    colorSurfaceOverlay: Color(argb: 0xFFFA_FAFA),
    colorSurfaceChrome: Color(argb: 0xFFF2_F2F2),
    colorSurfaceColumn: Color(argb: 0xFFF5_F5F5),
    colorContent: Color(argb: 0xFF3D_3D3D),
    colorContentSecondary: Color(argb: 0xFF5F_5F5F),
    colorContentNav: Color(argb: 0xFF72_7272),
    colorContentMuted: Color(argb: 0xFF72_7272),
    colorContentSubtle: Color(argb: 0xFF9C_9C9C),
    colorContentFaint: Color(argb: 0xFFB6_B6B6),
    colorBorder: Color(argb: 0x1A3D_3D3D),
    colorBorderStrong: Color(argb: 0x213D_3D3D),
    colorBorderMuted: Color(argb: 0xFFB2_B2B2),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `ubuntu-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let ubuntuDark = Palette(
    colorPrimary: Colors.ubuntuDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF27_2727),
    colorSurface: Color(argb: 0xFF2C_2C2C),
    colorSurfaceMuted: Color(argb: 0xFF33_3333),
    colorSurfaceSunken: Color(argb: 0xFF43_4343),
    colorSurfaceSubtle: Color(argb: 0x12F7_F7F7),
    colorSurfaceInset: Color(argb: 0xFF3A_3A3A),
    colorSurfaceRaised: Color(argb: 0xFF5A_5A5A),
    colorSurfaceOverlay: Color(argb: 0xFF33_3333),
    colorSurfaceChrome: Color(argb: 0xFF33_3333),
    colorSurfaceColumn: Color(argb: 0xFF28_2828),
    colorContent: Color(argb: 0xFFF7_F7F7),
    colorContentSecondary: Color(argb: 0xFFD2_D2D2),
    colorContentNav: Color(argb: 0xFFBE_BEBE),
    colorContentMuted: Color(argb: 0xFFAE_AEAE),
    colorContentSubtle: Color(argb: 0xFF92_9292),
    colorContentFaint: Color(argb: 0xFF75_7575),
    colorBorder: Color(argb: 0x1AF7_F7F7),
    colorBorderStrong: Color(argb: 0x21F7_F7F7),
    colorBorderMuted: Color(argb: 0xFF79_7979),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    nativeTintShade: 500
  )

  /// The `debian-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let debianLight = Palette(
    colorPrimary: Colors.debianLight,
    colorCanvas: Color(argb: 0xFFE8_E8EA),
    colorSurface: Color(argb: 0xFFFA_FAFB),
    colorSurfaceMuted: Color(argb: 0xFFF2_F2F4),
    colorSurfaceSunken: Color(argb: 0xFFDC_DCDD),
    colorSurfaceSubtle: Color(argb: 0x1132_3237),
    colorSurfaceInset: Color(argb: 0xFFEA_EAEC),
    colorSurfaceRaised: Color(argb: 0xFFFA_FAFB),
    colorSurfaceOverlay: Color(argb: 0xFFFA_FAFB),
    colorSurfaceChrome: Color(argb: 0xFFF2_F2F4),
    colorSurfaceColumn: Color(argb: 0xFFF5_F5F7),
    colorContent: Color(argb: 0xFF32_3237),
    colorContentSecondary: Color(argb: 0xFF56_565A),
    colorContentNav: Color(argb: 0xFF6A_6A6E),
    colorContentMuted: Color(argb: 0xFF70_7074),
    colorContentSubtle: Color(argb: 0xFF96_9699),
    colorContentFaint: Color(argb: 0xFFB2_B2B4),
    colorBorder: Color(argb: 0x1A32_3237),
    colorBorderStrong: Color(argb: 0x2132_3237),
    colorBorderMuted: Color(argb: 0xFFAE_AEB1),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `debian-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let debianDark = Palette(
    colorPrimary: Colors.debianDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1E_1E21),
    colorSurface: Color(argb: 0xFF22_2226),
    colorSurfaceMuted: Color(argb: 0xFF29_292C),
    colorSurfaceSunken: Color(argb: 0xFF38_383B),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF2F_2F32),
    colorSurfaceRaised: Color(argb: 0xFF4F_4F52),
    colorSurfaceOverlay: Color(argb: 0xFF29_292C),
    colorSurfaceChrome: Color(argb: 0xFF29_292C),
    colorSurfaceColumn: Color(argb: 0xFF1F_1F22),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD7_D7D8),
    colorContentNav: Color(argb: 0xFFC1_C1C2),
    colorContentMuted: Color(argb: 0xFFAF_AFB1),
    colorContentSubtle: Color(argb: 0xFF91_9193),
    colorContentFaint: Color(argb: 0xFF72_7274),
    colorBorder: Color(argb: 0x1AFF_FFFF),
    colorBorderStrong: Color(argb: 0x21FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF76_7678),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    nativeTintShade: 500
  )

  /// The `fedora-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let fedoraLight = Palette(
    colorPrimary: Colors.fedoraLight,
    colorCanvas: Color(argb: 0xFFE8_E8EA),
    colorSurface: Color(argb: 0xFFFA_FAFB),
    colorSurfaceMuted: Color(argb: 0xFFF2_F2F4),
    colorSurfaceSunken: Color(argb: 0xFFDC_DCDD),
    colorSurfaceSubtle: Color(argb: 0x1132_3237),
    colorSurfaceInset: Color(argb: 0xFFEA_EAEC),
    colorSurfaceRaised: Color(argb: 0xFFFA_FAFB),
    colorSurfaceOverlay: Color(argb: 0xFFFA_FAFB),
    colorSurfaceChrome: Color(argb: 0xFFF2_F2F4),
    colorSurfaceColumn: Color(argb: 0xFFF5_F5F7),
    colorContent: Color(argb: 0xFF32_3237),
    colorContentSecondary: Color(argb: 0xFF56_565A),
    colorContentNav: Color(argb: 0xFF6A_6A6E),
    colorContentMuted: Color(argb: 0xFF70_7074),
    colorContentSubtle: Color(argb: 0xFF96_9699),
    colorContentFaint: Color(argb: 0xFFB2_B2B4),
    colorBorder: Color(argb: 0x1A32_3237),
    colorBorderStrong: Color(argb: 0x2132_3237),
    colorBorderMuted: Color(argb: 0xFFAE_AEB1),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `fedora-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let fedoraDark = Palette(
    colorPrimary: Colors.fedoraDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1E_1E21),
    colorSurface: Color(argb: 0xFF22_2226),
    colorSurfaceMuted: Color(argb: 0xFF29_292C),
    colorSurfaceSunken: Color(argb: 0xFF38_383B),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF2F_2F32),
    colorSurfaceRaised: Color(argb: 0xFF4F_4F52),
    colorSurfaceOverlay: Color(argb: 0xFF29_292C),
    colorSurfaceChrome: Color(argb: 0xFF29_292C),
    colorSurfaceColumn: Color(argb: 0xFF1F_1F22),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD7_D7D8),
    colorContentNav: Color(argb: 0xFFC1_C1C2),
    colorContentMuted: Color(argb: 0xFFAF_AFB1),
    colorContentSubtle: Color(argb: 0xFF91_9193),
    colorContentFaint: Color(argb: 0xFF72_7274),
    colorBorder: Color(argb: 0x1AFF_FFFF),
    colorBorderStrong: Color(argb: 0x21FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF76_7678),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    nativeTintShade: 500
  )

  /// The `kde-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let kdeLight = Palette(
    colorPrimary: Colors.kdeLight,
    colorCanvas: Color(argb: 0xFFDD_DFE0),
    colorSurface: Color(argb: 0xFFEF_F0F1),
    colorSurfaceMuted: Color(argb: 0xFFE7_E9EA),
    colorSurfaceSunken: Color(argb: 0xFFD1_D2D4),
    colorSurfaceSubtle: Color(argb: 0x1023_2629),
    colorSurfaceInset: Color(argb: 0xFFDF_E0E2),
    colorSurfaceRaised: Color(argb: 0xFFEF_F0F1),
    colorSurfaceOverlay: Color(argb: 0xFFEF_F0F1),
    colorSurfaceChrome: Color(argb: 0xFFE7_E9EA),
    colorSurfaceColumn: Color(argb: 0xFFEA_EBED),
    colorContent: Color(argb: 0xFF23_2629),
    colorContentSecondary: Color(argb: 0xFF48_4A4D),
    colorContentNav: Color(argb: 0xFF5C_5F61),
    colorContentMuted: Color(argb: 0xFF68_6B6D),
    colorContentSubtle: Color(argb: 0xFF89_8B8D),
    colorContentFaint: Color(argb: 0xFFA6_A7A9),
    colorBorder: Color(argb: 0x2123_2629),
    colorBorderStrong: Color(argb: 0x2B23_2629),
    colorBorderMuted: Color(argb: 0xFFA1_A3A5),
    colorOnAccent: Color(argb: 0xFF23_2629)
  )

  /// The `kde-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let kdeDark = Palette(
    colorPrimary: Colors.kdeDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1C_1E21),
    colorSurface: Color(argb: 0xFF20_2326),
    colorSurfaceMuted: Color(argb: 0xFF27_2A2C),
    colorSurfaceSunken: Color(argb: 0xFF36_393B),
    colorSurfaceSubtle: Color(argb: 0x12FC_FCFC),
    colorSurfaceInset: Color(argb: 0xFF2D_3032),
    colorSurfaceRaised: Color(argb: 0xFF4D_5052),
    colorSurfaceOverlay: Color(argb: 0xFF27_2A2C),
    colorSurfaceChrome: Color(argb: 0xFF27_2A2C),
    colorSurfaceColumn: Color(argb: 0xFF1D_2022),
    colorContent: Color(argb: 0xFFFC_FCFC),
    colorContentSecondary: Color(argb: 0xFFD4_D5D5),
    colorContentNav: Color(argb: 0xFFBE_BFC0),
    colorContentMuted: Color(argb: 0xFFAD_AEAF),
    colorContentSubtle: Color(argb: 0xFF8E_9091),
    colorContentFaint: Color(argb: 0xFF6F_7173),
    colorBorder: Color(argb: 0x29FC_FCFC),
    colorBorderStrong: Color(argb: 0x33FC_FCFC),
    colorBorderMuted: Color(argb: 0xFF74_7577),
    colorOnAccent: Color(argb: 0xFF20_2326),
    nativeTintShade: 500
  )

  /// The `spotify-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let spotifyLight = Palette(
    colorPrimary: Colors.spotifyLight,
    colorCanvas: Color(argb: 0xFFED_EDED),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF7_F7F7),
    colorSurfaceSunken: Color(argb: 0xFFE0_E0E0),
    colorSurfaceSubtle: Color(argb: 0x0F12_1212),
    colorSurfaceInset: Color(argb: 0xFFEF_EFEF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF7_F7F7),
    colorSurfaceColumn: Color(argb: 0xFFFA_FAFA),
    colorContent: Color(argb: 0xFF12_1212),
    colorContentSecondary: Color(argb: 0xFF3D_3D3D),
    colorContentNav: Color(argb: 0xFF54_5454),
    colorContentMuted: Color(argb: 0xFF67_6767),
    colorContentSubtle: Color(argb: 0xFF89_8989),
    colorContentFaint: Color(argb: 0xFFAA_AAAA),
    colorBorder: Color(argb: 0x1A12_1212),
    colorBorderStrong: Color(argb: 0x2112_1212),
    colorBorderMuted: Color(argb: 0xFFA5_A5A5),
    colorOnAccent: Color(argb: 0xFFFF_FFFF)
  )

  /// The `spotify-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let spotifyDark = Palette(
    colorPrimary: Colors.spotifyDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF00_0000),
    colorSurface: Color(argb: 0xFF12_1212),
    colorSurfaceMuted: Color(argb: 0xFF1F_1F1F),
    colorSurfaceSunken: Color(argb: 0xFF33_3333),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF2A_2A2A),
    colorSurfaceRaised: Color(argb: 0xFF53_5353),
    colorSurfaceOverlay: Color(argb: 0xFF28_2828),
    colorSurfaceChrome: Color(argb: 0xFF1F_1F1F),
    colorSurfaceColumn: Color(argb: 0xFF0A_0A0A),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFB3_B3B3),
    colorContentNav: Color(argb: 0xFFB3_B3B3),
    colorContentMuted: Color(argb: 0xFFB3_B3B3),
    colorContentSubtle: Color(argb: 0xFF89_8989),
    colorContentFaint: Color(argb: 0xFF67_6767),
    colorBorder: Color(argb: 0x1AFF_FFFF),
    colorBorderStrong: Color(argb: 0x21FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF6C_6C6C),
    colorOnAccent: Color(argb: 0xFF00_0000),
    nativeTintShade: 500
  )

  /// The `bright-dark` counterpart of ``brightLight``.
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
}

public extension Palette {
  /// The ds filled action colour, applied through SwiftUI tint.
  var accent: Color { colorPrimary[nativeTintShade] }
}
