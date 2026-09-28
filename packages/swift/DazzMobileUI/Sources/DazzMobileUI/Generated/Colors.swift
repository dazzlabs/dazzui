import SwiftUI

/// Primitive color tokens.
///
/// DO NOT EDIT - This file is auto-generated
public enum Colors {
  /// Completely invisible.
  public static let transparent = Color(argb: 0x0000_0000)

  /// Completely opaque black.
  public static let black = Color(argb: 0xFF00_0000)

  /// Completely opaque white.
  public static let white = Color(argb: 0xFFFF_FFFF)

  public static let acid = ColorSwatch([
    50: Color(argb: 0xFFF6_FAE2),
    100: Color(argb: 0xFFF1_F7D2),
    200: Color(argb: 0xFFE9_FF8F),
    300: Color(argb: 0xFFD6_FF3F),
    400: Color(argb: 0xFFB8_E034),
    500: Color(argb: 0xFFD6_FF3F),
    600: Color(argb: 0xFFC2_EA2C),
    700: Color(argb: 0xFFA8_CC22),
    800: Color(argb: 0xFF4A_5C14),
    900: Color(argb: 0xFF24_310F),
    950: Color(argb: 0xFF1A_2410),
  ])

  public static let amber = ColorSwatch([
    50: Color(argb: 0xFFFD_F6EA),
    100: Color(argb: 0xFFFE_F1DE),
    200: Color(argb: 0xFFFF_E6C7),
    300: Color(argb: 0xFFF7_D1A0),
    400: Color(argb: 0xFFF0_BC78),
    500: Color(argb: 0xFFE8_A651),
    600: Color(argb: 0xFFE0_912A),
    700: Color(argb: 0xFFB9_6F12),
    800: Color(argb: 0xFF8A_5210),
    900: Color(argb: 0xFF57_3409),
    950: Color(argb: 0xFF3D_2506),
  ])

  public static let amberDark = ColorSwatch([
    50: Color(argb: 0xFFFD_F6EA),
    100: Color(argb: 0xFFFE_F1DE),
    200: Color(argb: 0xFFFF_E6C7),
    300: Color(argb: 0xFFFF_CF9C),
    400: Color(argb: 0xFFF0_BC78),
    500: Color(argb: 0xFFFF_B86B),
    600: Color(argb: 0xFFE0_912A),
    700: Color(argb: 0xFFB9_6F12),
    800: Color(argb: 0xFF8A_5210),
    900: Color(argb: 0xFF57_3409),
    950: Color(argb: 0xFF3D_2506),
  ])

  public static let brand = ColorSwatch([
    50: Color(argb: 0xFFF4_F1FE),
    100: Color(argb: 0xFFE6_DFFF),
    200: Color(argb: 0xFFCD_C0FF),
    300: Color(argb: 0xFFB9_A8FF),
    400: Color(argb: 0xFF9A_82FF),
    500: Color(argb: 0xFF7C_5CFF),
    600: Color(argb: 0xFF6B_4DFF),
    700: Color(argb: 0xFF5B_3FE0),
    800: Color(argb: 0xFF4C_33CC),
    900: Color(argb: 0xFF3D_2AA3),
    950: Color(argb: 0xFF2A_1C6B),
  ])

  public static let debianDark = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFE0_EDFE),
    200: Color(argb: 0xFFC1_DBFC),
    300: Color(argb: 0xFF98_C3FC),
    400: Color(argb: 0xFF5B_A0F8),
    500: Color(argb: 0xFF35_84E4),
    600: Color(argb: 0xFF1A_6DCB),
    700: Color(argb: 0xFF0B_5AAE),
    800: Color(argb: 0xFF02_4891),
    900: Color(argb: 0xFF03_3A75),
    950: Color(argb: 0xFF05_264D),
  ])

  public static let debianLight = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFE4_EFFF),
    200: Color(argb: 0xFFCA_E1FF),
    300: Color(argb: 0xFFAA_CDFC),
    400: Color(argb: 0xFF7A_B2FA),
    500: Color(argb: 0xFF4C_9AFC),
    600: Color(argb: 0xFF35_84E4),
    700: Color(argb: 0xFF23_6BC1),
    800: Color(argb: 0xFF13_539D),
    900: Color(argb: 0xFF0B_407D),
    950: Color(argb: 0xFF05_264D),
  ])

  public static let ember = ColorSwatch([
    50: Color(argb: 0xFFFD_F4EC),
    100: Color(argb: 0xFFFA_E4D0),
    200: Color(argb: 0xFFF3_C8A0),
    300: Color(argb: 0xFFE9_A969),
    400: Color(argb: 0xFFD9_863A),
    500: Color(argb: 0xFFC4_6A1E),
    600: Color(argb: 0xFFAD_5717),
    700: Color(argb: 0xFF8C_4415),
    800: Color(argb: 0xFF6D_3513),
    900: Color(argb: 0xFF51_2810),
    950: Color(argb: 0xFF33_190B),
  ])

  public static let emberDark = ColorSwatch([
    50: Color(argb: 0xFFFD_F2E4),
    100: Color(argb: 0xFFFA_E0C4),
    200: Color(argb: 0xFFF6_CF9F),
    300: Color(argb: 0xFFF0_B877),
    400: Color(argb: 0xFFE9_A45C),
    500: Color(argb: 0xFFD9_832E),
    600: Color(argb: 0xFFE0_954A),
    700: Color(argb: 0xFFB9_6A20),
    800: Color(argb: 0xFF7D_4718),
    900: Color(argb: 0xFF4A_2B11),
    950: Color(argb: 0xFF2C_1A0C),
  ])

  public static let fedoraDark = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFE0_EDFE),
    200: Color(argb: 0xFFC1_DBFC),
    300: Color(argb: 0xFF98_C3FC),
    400: Color(argb: 0xFF5B_A0F8),
    500: Color(argb: 0xFF35_84E4),
    600: Color(argb: 0xFF1A_6DCB),
    700: Color(argb: 0xFF0B_5AAE),
    800: Color(argb: 0xFF02_4891),
    900: Color(argb: 0xFF03_3A75),
    950: Color(argb: 0xFF05_264D),
  ])

  public static let fedoraLight = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFE4_EFFF),
    200: Color(argb: 0xFFCA_E1FF),
    300: Color(argb: 0xFFAA_CDFC),
    400: Color(argb: 0xFF7A_B2FA),
    500: Color(argb: 0xFF4C_9AFC),
    600: Color(argb: 0xFF35_84E4),
    700: Color(argb: 0xFF23_6BC1),
    800: Color(argb: 0xFF13_539D),
    900: Color(argb: 0xFF0B_407D),
    950: Color(argb: 0xFF05_264D),
  ])

  public static let frost = ColorSwatch([
    50: Color(argb: 0xFFEE_F8FA),
    100: Color(argb: 0xFFD7_EFF4),
    200: Color(argb: 0xFFB2_E0EA),
    300: Color(argb: 0xFF74_C9DC),
    400: Color(argb: 0xFF35_ABC4),
    500: Color(argb: 0xFF16_90A9),
    600: Color(argb: 0xFF0F_7A92),
    700: Color(argb: 0xFF0C_6277),
    800: Color(argb: 0xFF0B_4F61),
    900: Color(argb: 0xFF0D_3C4A),
    950: Color(argb: 0xFF08_262F),
  ])

  public static let graphite = ColorSwatch([
    50: Color(argb: 0xFFF6_F6F7),
    100: Color(argb: 0xFFED_EDEF),
    200: Color(argb: 0xFFE2_E2E6),
    300: Color(argb: 0xFFC9_C9CF),
    400: Color(argb: 0xFF8F_8F99),
    500: Color(argb: 0xFF52_525B),
    600: Color(argb: 0xFF3F_3F46),
    700: Color(argb: 0xFF2B_2B31),
    800: Color(argb: 0xFF1C_1C21),
    900: Color(argb: 0xFF13_1317),
    950: Color(argb: 0xFF0B_0B0E),
  ])

  public static let graphiteDark = ColorSwatch([
    50: Color(argb: 0xFF1C_1C20),
    100: Color(argb: 0xFF26_262B),
    200: Color(argb: 0xFFFA_FAFA),
    300: Color(argb: 0xFFE6_E6EA),
    400: Color(argb: 0xFFA1_A1AA),
    500: Color(argb: 0xFFED_EDF0),
    600: Color(argb: 0xFFD4_D4D8),
    700: Color(argb: 0xFFA1_A1AA),
    800: Color(argb: 0xFF71_717A),
    900: Color(argb: 0xFF52_525B),
    950: Color(argb: 0xFF3F_3F46),
  ])

  public static let green = ColorSwatch([
    50: Color(argb: 0xFFE9_F5EF),
    100: Color(argb: 0xFFD7_EDE2),
    200: Color(argb: 0xFFB2_DDC7),
    300: Color(argb: 0xFF8D_CDAD),
    400: Color(argb: 0xFF68_BD93),
    500: Color(argb: 0xFF44_AD78),
    600: Color(argb: 0xFF1F_9D5E),
    700: Color(argb: 0xFF17_7D4A),
    800: Color(argb: 0xFF10_5B37),
    900: Color(argb: 0xFF09_3924),
    950: Color(argb: 0xFF06_281A),
  ])

  public static let greenDark = ColorSwatch([
    50: Color(argb: 0xFFE9_F5EF),
    100: Color(argb: 0xFFD7_EDE2),
    200: Color(argb: 0xFFB2_DDC7),
    300: Color(argb: 0xFF7F_D7A8),
    400: Color(argb: 0xFF68_BD93),
    500: Color(argb: 0xFF34_D399),
    600: Color(argb: 0xFF1F_9D5E),
    700: Color(argb: 0xFF17_7D4A),
    800: Color(argb: 0xFF10_5B37),
    900: Color(argb: 0xFF09_3924),
    950: Color(argb: 0xFF06_281A),
  ])

  public static let ink = ColorSwatch([
    50: Color(argb: 0xFFF6_FAE2),
    100: Color(argb: 0xFFF1_F7D2),
    200: Color(argb: 0xFFE6_F77A),
    300: Color(argb: 0xFFD6_FF3F),
    400: Color(argb: 0xFF8F_A06A),
    500: Color(argb: 0xFF3D_5470),
    600: Color(argb: 0xFF11_1C2E),
    700: Color(argb: 0xFF1C_2A41),
    800: Color(argb: 0xFF26_3449),
    900: Color(argb: 0xFF0B_1420),
    950: Color(argb: 0xFF06_0B12),
  ])

  public static let kdeDark = ColorSwatch([
    50: Color(argb: 0xFFEF_F8FE),
    100: Color(argb: 0xFFDF_F2FF),
    200: Color(argb: 0xFFC4_E7FD),
    300: Color(argb: 0xFF9A_D8FE),
    400: Color(argb: 0xFF63_C1F7),
    500: Color(argb: 0xFF3D_AEE9),
    600: Color(argb: 0xFF08_90C9),
    700: Color(argb: 0xFF14_75A3),
    800: Color(argb: 0xFF16_5C7F),
    900: Color(argb: 0xFF0D_4764),
    950: Color(argb: 0xFF05_2A3D),
  ])

  public static let kdeLight = ColorSwatch([
    50: Color(argb: 0xFFEF_F8FE),
    100: Color(argb: 0xFFE2_F3FF),
    200: Color(argb: 0xFFCB_EAFE),
    300: Color(argb: 0xFFAD_DEFD),
    400: Color(argb: 0xFF7A_CDFE),
    500: Color(argb: 0xFF50_BEFA),
    600: Color(argb: 0xFF3D_AEE9),
    700: Color(argb: 0xFF10_73A1),
    800: Color(argb: 0xFF1B_6B93),
    900: Color(argb: 0xFF0B_5070),
    950: Color(argb: 0xFF05_2A3D),
  ])

  public static let macos15Dark = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFE1_EEFF),
    200: Color(argb: 0xFFC2_DCFE),
    300: Color(argb: 0xFF99_C5FE),
    400: Color(argb: 0xFF5D_A2FC),
    500: Color(argb: 0xFF0A_84FF),
    600: Color(argb: 0xFF1C_6FCF),
    700: Color(argb: 0xFF0D_5CB1),
    800: Color(argb: 0xFF02_4993),
    900: Color(argb: 0xFF03_3A77),
    950: Color(argb: 0xFF04_264D),
  ])

  public static let macos15Light = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFE4_EFFE),
    200: Color(argb: 0xFFCB_DFFD),
    300: Color(argb: 0xFFA8_CCFE),
    400: Color(argb: 0xFF77_AEFD),
    500: Color(argb: 0xFF53_96F6),
    600: Color(argb: 0xFF00_7AFF),
    700: Color(argb: 0xFF0E_65CE),
    800: Color(argb: 0xFF12_50A1),
    900: Color(argb: 0xFF0A_3E80),
    950: Color(argb: 0xFF00_2454),
  ])

  public static let macos27Dark = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FE),
    100: Color(argb: 0xFFE1_EFFE),
    200: Color(argb: 0xFFC4_DFFD),
    300: Color(argb: 0xFF9C_CAFE),
    400: Color(argb: 0xFF60_ACFC),
    500: Color(argb: 0xFF00_91FF),
    600: Color(argb: 0xFF1B_79CE),
    700: Color(argb: 0xFF07_63B0),
    800: Color(argb: 0xFF10_508A),
    900: Color(argb: 0xFF0B_3F6F),
    950: Color(argb: 0xFF02_274A),
  ])

  public static let macos27Light = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFE4_F0FF),
    200: Color(argb: 0xFFCC_E2FD),
    300: Color(argb: 0xFFAB_D0FE),
    400: Color(argb: 0xFF7C_B6FD),
    500: Color(argb: 0xFF58_A0F7),
    600: Color(argb: 0xFF00_88FF),
    700: Color(argb: 0xFF0F_6FCE),
    800: Color(argb: 0xFF12_579F),
    900: Color(argb: 0xFF09_437E),
    950: Color(argb: 0xFF03_264C),
  ])

  public static let neutral = ColorSwatch([
    50: Color(argb: 0xFFFA_FAFA),
    100: Color(argb: 0xFFF5_F5F5),
    200: Color(argb: 0xFFE5_E5E5),
    300: Color(argb: 0xFFD4_D4D4),
    400: Color(argb: 0xFFA1_A1A1),
    500: Color(argb: 0xFF73_7373),
    600: Color(argb: 0xFF52_5252),
    700: Color(argb: 0xFF40_4040),
    800: Color(argb: 0xFF26_2626),
    900: Color(argb: 0xFF17_1717),
    950: Color(argb: 0xFF0A_0A0A),
  ])

  public static let nocturne = ColorSwatch([
    50: Color(argb: 0xFFF5_F4FF),
    100: Color(argb: 0xFFE7_E5FE),
    200: Color(argb: 0xFFD2_CEFD),
    300: Color(argb: 0xFFB5_ABFC),
    400: Color(argb: 0xFFA0_94E8),
    500: Color(argb: 0xFF91_84D9),
    600: Color(argb: 0xFF83_75D1),
    700: Color(argb: 0xFF6F_60C6),
    800: Color(argb: 0xFF5B_4BB4),
    900: Color(argb: 0xFF45_3897),
    950: Color(argb: 0xFF2D_2465),
  ])

  public static let omarchyCatppuccinDark = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FE),
    100: Color(argb: 0xFFE9_F1FF),
    200: Color(argb: 0xFFD7_E6FE),
    300: Color(argb: 0xFFC1_D8FD),
    400: Color(argb: 0xFFA1_C4FE),
    500: Color(argb: 0xFF89_B4FA),
    600: Color(argb: 0xFF69_92D6),
    700: Color(argb: 0xFF51_76B2),
    800: Color(argb: 0xFF3C_5B8E),
    900: Color(argb: 0xFF2C_456F),
    950: Color(argb: 0xFF14_2644),
  ])

  public static let omarchyCatppuccinLatteDark = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FE),
    100: Color(argb: 0xFFE7_EFFF),
    200: Color(argb: 0xFFD3_E1FD),
    300: Color(argb: 0xFFB7_CFFE),
    400: Color(argb: 0xFF90_B5FE),
    500: Color(argb: 0xFF74_A0F9),
    600: Color(argb: 0xFF58_82D8),
    700: Color(argb: 0xFF44_69B6),
    800: Color(argb: 0xFF33_5294),
    900: Color(argb: 0xFF25_3F75),
    950: Color(argb: 0xFF12_244B),
  ])

  public static let omarchyCatppuccinLatteLight = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FF),
    100: Color(argb: 0xFFE4_EDFE),
    200: Color(argb: 0xFFC9_DBFC),
    300: Color(argb: 0xFFA7_C5FB),
    400: Color(argb: 0xFF76_A3F7),
    500: Color(argb: 0xFF4B_86F9),
    600: Color(argb: 0xFF1E_66F5),
    700: Color(argb: 0xFF11_52D3),
    800: Color(argb: 0xFF07_40AF),
    900: Color(argb: 0xFF05_338E),
    950: Color(argb: 0xFF00_1E61),
  ])

  public static let omarchyCatppuccinLight = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FD),
    100: Color(argb: 0xFFE5_EDF9),
    200: Color(argb: 0xFFCD_DBF0),
    300: Color(argb: 0xFFAE_C3E5),
    400: Color(argb: 0xFF88_A2CC),
    500: Color(argb: 0xFF6D_89B5),
    600: Color(argb: 0xFF55_709B),
    700: Color(argb: 0xFF45_5D83),
    800: Color(argb: 0xFF36_4B6C),
    900: Color(argb: 0xFF2B_3C57),
    950: Color(argb: 0xFF1A_273B),
  ])

  public static let omarchyEtherealDark = ColorSwatch([
    50: Color(argb: 0xFFF5_F6FF),
    100: Color(argb: 0xFFE9_EBFF),
    200: Color(argb: 0xFFD3_D8FE),
    300: Color(argb: 0xFFB8_BEFF),
    400: Color(argb: 0xFF96_9CEC),
    500: Color(argb: 0xFF7D_82D9),
    600: Color(argb: 0xFF66_6ABE),
    700: Color(argb: 0xFF54_57A2),
    800: Color(argb: 0xFF43_4485),
    900: Color(argb: 0xFF34_366B),
    950: Color(argb: 0xFF20_2148),
  ])

  public static let omarchyEtherealLight = ColorSwatch([
    50: Color(argb: 0xFFF5_F6FE),
    100: Color(argb: 0xFFE7_E9F9),
    200: Color(argb: 0xFFCE_D2EE),
    300: Color(argb: 0xFFB0_B4DF),
    400: Color(argb: 0xFF87_8CC1),
    500: Color(argb: 0xFF69_6EA6),
    600: Color(argb: 0xFF4E_5187),
    700: Color(argb: 0xFF43_4576),
    800: Color(argb: 0xFF38_3A64),
    900: Color(argb: 0xFF2F_3154),
    950: Color(argb: 0xFF22_233D),
  ])

  public static let omarchyEverforestDark = ColorSwatch([
    50: Color(argb: 0xFFF1_F8F7),
    100: Color(argb: 0xFFE5_F4F1),
    200: Color(argb: 0xFFD0_EAE6),
    300: Color(argb: 0xFFB5_DED8),
    400: Color(argb: 0xFF95_CBC4),
    500: Color(argb: 0xFF7F_BBB3),
    600: Color(argb: 0xFF5F_9B93),
    700: Color(argb: 0xFF49_7E78),
    800: Color(argb: 0xFF35_635D),
    900: Color(argb: 0xFF26_4C47),
    950: Color(argb: 0xFF10_2C29),
  ])

  public static let omarchyEverforestLight = ColorSwatch([
    50: Color(argb: 0xFFF3_F8F7),
    100: Color(argb: 0xFFE5_EFED),
    200: Color(argb: 0xFFCB_DDDA),
    300: Color(argb: 0xFFAC_C7C3),
    400: Color(argb: 0xFF85_A7A2),
    500: Color(argb: 0xFF68_8E88),
    600: Color(argb: 0xFF4F_746F),
    700: Color(argb: 0xFF40_615D),
    800: Color(argb: 0xFF33_4F4B),
    900: Color(argb: 0xFF28_403D),
    950: Color(argb: 0xFF19_2A28),
  ])

  public static let omarchyFlexokiLightDark = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FD),
    100: Color(argb: 0xFFE5_EFFB),
    200: Color(argb: 0xFFCE_E0F6),
    300: Color(argb: 0xFFB1_CDEE),
    400: Color(argb: 0xFF8E_B1DA),
    500: Color(argb: 0xFF75_9BC8),
    600: Color(argb: 0xFF5B_80AC),
    700: Color(argb: 0xFF48_6990),
    800: Color(argb: 0xFF37_5374),
    900: Color(argb: 0xFF2A_415C),
    950: Color(argb: 0xFF16_273B),
  ])

  public static let omarchyFlexokiLightLight = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFDF_ECFD),
    200: Color(argb: 0xFFBC_D8FD),
    300: Color(argb: 0xFF92_BEF5),
    400: Color(argb: 0xFF61_98DD),
    500: Color(argb: 0xFF3E_7BC5),
    600: Color(argb: 0xFF20_5EA6),
    700: Color(argb: 0xFF17_5090),
    800: Color(argb: 0xFF11_427A),
    900: Color(argb: 0xFF0F_3765),
    950: Color(argb: 0xFF09_2649),
  ])

  public static let omarchyGruvboxDark = ColorSwatch([
    50: Color(argb: 0xFFF2_F8F7),
    100: Color(argb: 0xFFE6_F2EF),
    200: Color(argb: 0xFFD0_E6E1),
    300: Color(argb: 0xFFB5_D8CF),
    400: Color(argb: 0xFF94_C1B6),
    500: Color(argb: 0xFF7D_AEA3),
    600: Color(argb: 0xFF60_9086),
    700: Color(argb: 0xFF4B_776D),
    800: Color(argb: 0xFF38_5D56),
    900: Color(argb: 0xFF2A_4942),
    950: Color(argb: 0xFF15_2B27),
  ])

  public static let omarchyGruvboxLight = ColorSwatch([
    50: Color(argb: 0xFFF4_F8F7),
    100: Color(argb: 0xFFE5_EEEB),
    200: Color(argb: 0xFFCC_DBD7),
    300: Color(argb: 0xFFAD_C3BE),
    400: Color(argb: 0xFF85_A19A),
    500: Color(argb: 0xFF68_877F),
    600: Color(argb: 0xFF4E_6C65),
    700: Color(argb: 0xFF40_5B55),
    800: Color(argb: 0xFF34_4B46),
    900: Color(argb: 0xFF2A_3D39),
    950: Color(argb: 0xFF1C_2A26),
  ])

  public static let omarchyHackermanDark = ColorSwatch([
    50: Color(argb: 0xFFEE_FBEF),
    100: Color(argb: 0xFFE2_FDE5),
    200: Color(argb: 0xFFD1_FED7),
    300: Color(argb: 0xFFBD_FDC7),
    400: Color(argb: 0xFF94_FFA9),
    500: Color(argb: 0xFF82_FB9C),
    600: Color(argb: 0xFF53_CE72),
    700: Color(argb: 0xFF33_A654),
    800: Color(argb: 0xFF15_7F39),
    900: Color(argb: 0xFF14_5D2A),
    950: Color(argb: 0xFF00_300F),
  ])

  public static let omarchyHackermanLight = ColorSwatch([
    50: Color(argb: 0xFFF1_FAF2),
    100: Color(argb: 0xFFE2_F4E4),
    200: Color(argb: 0xFFC8_EACD),
    300: Color(argb: 0xFFA8_DCB0),
    400: Color(argb: 0xFF81_C58D),
    500: Color(argb: 0xFF66_B175),
    600: Color(argb: 0xFF51_9C61),
    700: Color(argb: 0xFF3D_804C),
    800: Color(argb: 0xFF2A_6538),
    900: Color(argb: 0xFF1E_4E2A),
    950: Color(argb: 0xFF0A_2F14),
  ])

  public static let omarchyKanagawaDark = ColorSwatch([
    50: Color(argb: 0xFFF7_F7F3),
    100: Color(argb: 0xFFF5_F4ED),
    200: Color(argb: 0xFFF1_EFE2),
    300: Color(argb: 0xFFEC_E8D4),
    400: Color(argb: 0xFFE3_DFC4),
    500: Color(argb: 0xFFDC_D7BA),
    600: Color(argb: 0xFFB2_AD91),
    700: Color(argb: 0xFF8E_8A71),
    800: Color(argb: 0xFF6C_6853),
    900: Color(argb: 0xFF50_4D3C),
    950: Color(argb: 0xFF29_271B),
  ])

  public static let omarchyKanagawaLight = ColorSwatch([
    50: Color(argb: 0xFFF7_F7F4),
    100: Color(argb: 0xFFEF_EEE9),
    200: Color(argb: 0xFFE0_DFD6),
    300: Color(argb: 0xFFCD_CBBE),
    400: Color(argb: 0xFFB2_B09F),
    500: Color(argb: 0xFF9D_9A88),
    600: Color(argb: 0xFF88_8573),
    700: Color(argb: 0xFF6F_6D5D),
    800: Color(argb: 0xFF57_5547),
    900: Color(argb: 0xFF44_4237),
    950: Color(argb: 0xFF28_271F),
  ])

  public static let omarchyLastHorizonDark = ColorSwatch([
    50: Color(argb: 0xFFFA_F6F5),
    100: Color(argb: 0xFFF5_ECEB),
    200: Color(argb: 0xFFEB_DCD9),
    300: Color(argb: 0xFFDE_C8C2),
    400: Color(argb: 0xFFC8_ACA6),
    500: Color(argb: 0xFFB5_9790),
    600: Color(argb: 0xFF98_7B74),
    700: Color(argb: 0xFF7E_645E),
    800: Color(argb: 0xFF64_4E49),
    900: Color(argb: 0xFF4E_3C38),
    950: Color(argb: 0xFF30_2320),
  ])

  public static let omarchyLastHorizonLight = ColorSwatch([
    50: Color(argb: 0xFFF9_F6F5),
    100: Color(argb: 0xFFEF_EAE8),
    200: Color(argb: 0xFFDE_D4D2),
    300: Color(argb: 0xFFC7_B9B5),
    400: Color(argb: 0xFFA6_948F),
    500: Color(argb: 0xFF8B_7873),
    600: Color(argb: 0xFF70_5E59),
    700: Color(argb: 0xFF5F_4F4B),
    800: Color(argb: 0xFF4F_413D),
    900: Color(argb: 0xFF41_3532),
    950: Color(argb: 0xFF2D_2422),
  ])

  public static let omarchyLumonDark = ColorSwatch([
    50: Color(argb: 0xFFF1_F8FC),
    100: Color(argb: 0xFFE5_F4FD),
    200: Color(argb: 0xFFD2_EDFD),
    300: Color(argb: 0xFFB9_E4FC),
    400: Color(argb: 0xFF9E_D5F4),
    500: Color(argb: 0xFF8B_C9EB),
    600: Color(argb: 0xFF67_A4C5),
    700: Color(argb: 0xFF4E_84A2),
    800: Color(argb: 0xFF37_667F),
    900: Color(argb: 0xFF26_4D61),
    950: Color(argb: 0xFF0D_2A38),
  ])

  public static let omarchyLumonLight = ColorSwatch([
    50: Color(argb: 0xFFF3_F8FA),
    100: Color(argb: 0xFFE5_EFF5),
    200: Color(argb: 0xFFCC_DFE9),
    300: Color(argb: 0xFFAE_CADA),
    400: Color(argb: 0xFF88_ACC0),
    500: Color(argb: 0xFF6D_95AA),
    600: Color(argb: 0xFF56_7D92),
    700: Color(argb: 0xFF45_687A),
    800: Color(argb: 0xFF35_5363),
    900: Color(argb: 0xFF29_414F),
    950: Color(argb: 0xFF17_2933),
  ])

  public static let omarchyLupineDark = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FF),
    100: Color(argb: 0xFFE9_EFFE),
    200: Color(argb: 0xFFD5_E1FE),
    300: Color(argb: 0xFFBD_CFFB),
    400: Color(argb: 0xFF99_B4F9),
    500: Color(argb: 0xFF80_9FF3),
    600: Color(argb: 0xFF64_81D3),
    700: Color(argb: 0xFF4F_69B1),
    800: Color(argb: 0xFF3C_518F),
    900: Color(argb: 0xFF2D_3F71),
    950: Color(argb: 0xFF17_2449),
  ])

  public static let omarchyLupineLight = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FE),
    100: Color(argb: 0xFFE4_EDFE),
    200: Color(argb: 0xFFCA_DBFD),
    300: Color(argb: 0xFFA7_C3FD),
    400: Color(argb: 0xFF78_A0FA),
    500: Color(argb: 0xFF4F_81FC),
    600: Color(argb: 0xFF32_64EB),
    700: Color(argb: 0xFF25_51CA),
    800: Color(argb: 0xFF1B_3FA8),
    900: Color(argb: 0xFF14_3288),
    950: Color(argb: 0xFF09_1E5E),
  ])

  public static let omarchyMatteBlackDark = ColorSwatch([
    50: Color(argb: 0xFFFE_F5EC),
    100: Color(argb: 0xFFFF_ECD9),
    200: Color(argb: 0xFFFD_DBBA),
    300: Color(argb: 0xFFFD_C489),
    400: Color(argb: 0xFFF5_A448),
    500: Color(argb: 0xFFE6_8E0D),
    600: Color(argb: 0xFFBA_761F),
    700: Color(argb: 0xFF9B_5E06),
    800: Color(argb: 0xFF79_4A0D),
    900: Color(argb: 0xFF5F_3806),
    950: Color(argb: 0xFF39_2002),
  ])

  public static let omarchyMatteBlackLight = ColorSwatch([
    50: Color(argb: 0xFFFD_F5EE),
    100: Color(argb: 0xFFF9_E9D9),
    200: Color(argb: 0xFFEE_D2B6),
    300: Color(argb: 0xFFE0_B589),
    400: Color(argb: 0xFFC4_8F55),
    500: Color(argb: 0xFFAB_722E),
    600: Color(argb: 0xFF8F_5808),
    700: Color(argb: 0xFF77_4B12),
    800: Color(argb: 0xFF62_3C0A),
    900: Color(argb: 0xFF51_3109),
    950: Color(argb: 0xFF38_2104),
  ])

  public static let omarchyMiasmaDark = ColorSwatch([
    50: Color(argb: 0xFFF6_F8F1),
    100: Color(argb: 0xFFEA_EDDF),
    200: Color(argb: 0xFFD4_DAC0),
    300: Color(argb: 0xFFB9_C19A),
    400: Color(argb: 0xFF94_9E6C),
    500: Color(argb: 0xFF78_824B),
    600: Color(argb: 0xFF64_6E37),
    700: Color(argb: 0xFF54_5C2B),
    800: Color(argb: 0xFF44_4C21),
    900: Color(argb: 0xFF37_3D1B),
    950: Color(argb: 0xFF25_290F),
  ])

  public static let omarchyMiasmaLight = ColorSwatch([
    50: Color(argb: 0xFFF6_F7F3),
    100: Color(argb: 0xFFE8_EBE1),
    200: Color(argb: 0xFFCF_D4C2),
    300: Color(argb: 0xFFB1_B79D),
    400: Color(argb: 0xFF87_8F6E),
    500: Color(argb: 0xFF68_6F4C),
    600: Color(argb: 0xFF4A_512F),
    700: Color(argb: 0xFF41_4728),
    800: Color(argb: 0xFF38_3D22),
    900: Color(argb: 0xFF30_351F),
    950: Color(argb: 0xFF25_2818),
  ])

  public static let omarchyNordDark = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FB),
    100: Color(argb: 0xFFE7_F0F8),
    200: Color(argb: 0xFFD2_E1F1),
    300: Color(argb: 0xFFB8_CFE7),
    400: Color(argb: 0xFF98_B6D3),
    500: Color(argb: 0xFF81_A1C1),
    600: Color(argb: 0xFF66_85A4),
    700: Color(argb: 0xFF51_6D88),
    800: Color(argb: 0xFF3E_566D),
    900: Color(argb: 0xFF2F_4356),
    950: Color(argb: 0xFF1A_2836),
  ])

  public static let omarchyNordLight = ColorSwatch([
    50: Color(argb: 0xFFF4_F7FA),
    100: Color(argb: 0xFFE6_ECF2),
    200: Color(argb: 0xFFCE_D8E2),
    300: Color(argb: 0xFFAF_BECE),
    400: Color(argb: 0xFF87_9BAE),
    500: Color(argb: 0xFF6A_7F94),
    600: Color(argb: 0xFF50_6478),
    700: Color(argb: 0xFF43_5567),
    800: Color(argb: 0xFF36_4655),
    900: Color(argb: 0xFF2D_3946),
    950: Color(argb: 0xFF1E_2731),
  ])

  public static let omarchyOsakaJadeDark = ColorSwatch([
    50: Color(argb: 0xFFF1_F9F5),
    100: Color(argb: 0xFFE0_F1E8),
    200: Color(argb: 0xFFC3_E1D1),
    300: Color(argb: 0xFF9E_CDB5),
    400: Color(argb: 0xFF71_AD90),
    500: Color(argb: 0xFF50_9475),
    600: Color(argb: 0xFF39_7E60),
    700: Color(argb: 0xFF2B_694F),
    800: Color(argb: 0xFF20_563F),
    900: Color(argb: 0xFF19_4532),
    950: Color(argb: 0xFF0C_2D20),
  ])

  public static let omarchyOsakaJadeLight = ColorSwatch([
    50: Color(argb: 0xFFF3_F8F5),
    100: Color(argb: 0xFFE2_EDE7),
    200: Color(argb: 0xFFC4_D8CE),
    300: Color(argb: 0xFF9F_BEAE),
    400: Color(argb: 0xFF71_9885),
    500: Color(argb: 0xFF4F_7A66),
    600: Color(argb: 0xFF32_5C49),
    700: Color(argb: 0xFF2A_503F),
    800: Color(argb: 0xFF23_4435),
    900: Color(argb: 0xFF1F_3A2E),
    950: Color(argb: 0xFF17_2B22),
  ])

  public static let omarchyRetro82Dark = ColorSwatch([
    50: Color(argb: 0xFFFF_F5ED),
    100: Color(argb: 0xFFFE_EEE3),
    200: Color(argb: 0xFFFE_E2CE),
    300: Color(argb: 0xFFFD_D3B3),
    400: Color(argb: 0xFFFE_BB88),
    500: Color(argb: 0xFFFA_A968),
    600: Color(argb: 0xFFD3_8543),
    700: Color(argb: 0xFFAE_692C),
    800: Color(argb: 0xFF89_4E18),
    900: Color(argb: 0xFF6A_3A0B),
    950: Color(argb: 0xFF3C_1D02),
  ])

  public static let omarchyRetro82Light = ColorSwatch([
    50: Color(argb: 0xFFFC_F5F0),
    100: Color(argb: 0xFFF8_EADF),
    200: Color(argb: 0xFFEF_D6C2),
    300: Color(argb: 0xFFE3_BC9E),
    400: Color(argb: 0xFFCA_9A75),
    500: Color(argb: 0xFFB4_8057),
    600: Color(argb: 0xFF9B_6940),
    700: Color(argb: 0xFF82_5631),
    800: Color(argb: 0xFF6A_4424),
    900: Color(argb: 0xFF55_351B),
    950: Color(argb: 0xFF37_200D),
  ])

  public static let omarchyRistrettoDark = ColorSwatch([
    50: Color(argb: 0xFFFE_F4F1),
    100: Color(argb: 0xFFFE_EBE6),
    200: Color(argb: 0xFFFF_DAD0),
    300: Color(argb: 0xFFFC_C5B5),
    400: Color(argb: 0xFFFA_A58D),
    500: Color(argb: 0xFFF3_8D70),
    600: Color(argb: 0xFFD0_6D51),
    700: Color(argb: 0xFFAD_553D),
    800: Color(argb: 0xFF8A_402A),
    900: Color(argb: 0xFF6C_2F1E),
    950: Color(argb: 0xFF43_170A),
  ])

  public static let omarchyRistrettoLight = ColorSwatch([
    50: Color(argb: 0xFFFE_F4F2),
    100: Color(argb: 0xFFFB_E7E2),
    200: Color(argb: 0xFFF2_D0C6),
    300: Color(argb: 0xFFE5_B3A4),
    400: Color(argb: 0xFFCB_8D7B),
    500: Color(argb: 0xFFB3_705D),
    600: Color(argb: 0xFF97_5745),
    700: Color(argb: 0xFF81_4737),
    800: Color(argb: 0xFF6A_392B),
    900: Color(argb: 0xFF56_2E22),
    950: Color(argb: 0xFF3B_1D15),
  ])

  public static let omarchyRosePineDark = ColorSwatch([
    50: Color(argb: 0xFFF3_F8F8),
    100: Color(argb: 0xFFE9_F3F4),
    200: Color(argb: 0xFFD8_EAEC),
    300: Color(argb: 0xFFC2_DEE2),
    400: Color(argb: 0xFFA8_CCD1),
    500: Color(argb: 0xFF96_BDC3),
    600: Color(argb: 0xFF75_9BA1),
    700: Color(argb: 0xFF5C_7E83),
    800: Color(argb: 0xFF45_6266),
    900: Color(argb: 0xFF33_4A4E),
    950: Color(argb: 0xFF18_2A2C),
  ])

  public static let omarchyRosePineLight = ColorSwatch([
    50: Color(argb: 0xFFF1_F8FA),
    100: Color(argb: 0xFFE3_F2F5),
    200: Color(argb: 0xFFCA_E5EA),
    300: Color(argb: 0xFFAA_D5DD),
    400: Color(argb: 0xFF85_BDC7),
    500: Color(argb: 0xFF6A_A9B4),
    600: Color(argb: 0xFF56_949F),
    700: Color(argb: 0xFF3E_7680),
    800: Color(argb: 0xFF2F_5F68),
    900: Color(argb: 0xFF22_4950),
    950: Color(argb: 0xFF0D_2B30),
  ])

  public static let omarchySolitudeDark = ColorSwatch([
    50: Color(argb: 0xFFF6_F7F8),
    100: Color(argb: 0xFFEA_ECED),
    200: Color(argb: 0xFFD4_D8DA),
    300: Color(argb: 0xFFB9_BFC3),
    400: Color(argb: 0xFF95_9CA1),
    500: Color(argb: 0xFF79_8186),
    600: Color(argb: 0xFF65_6C71),
    700: Color(argb: 0xFF54_5B5F),
    800: Color(argb: 0xFF44_494D),
    900: Color(argb: 0xFF36_3B3E),
    950: Color(argb: 0xFF23_2729),
  ])

  public static let omarchySolitudeLight = ColorSwatch([
    50: Color(argb: 0xFFF6_F7F7),
    100: Color(argb: 0xFFE8_EAEB),
    200: Color(argb: 0xFFD0_D2D4),
    300: Color(argb: 0xFFB1_B5B7),
    400: Color(argb: 0xFF88_8D90),
    500: Color(argb: 0xFF68_6E71),
    600: Color(argb: 0xFF4B_5053),
    700: Color(argb: 0xFF41_4648),
    800: Color(argb: 0xFF38_3C3E),
    900: Color(argb: 0xFF30_3335),
    950: Color(argb: 0xFF24_2728),
  ])

  public static let omarchyTokyoNightDark = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FF),
    100: Color(argb: 0xFFE8_EFFE),
    200: Color(argb: 0xFFD4_E2FC),
    300: Color(argb: 0xFFBA_D0FD),
    400: Color(argb: 0xFF95_B7FC),
    500: Color(argb: 0xFF7A_A2F7),
    600: Color(argb: 0xFF5E_84D6),
    700: Color(argb: 0xFF49_6BB4),
    800: Color(argb: 0xFF37_5392),
    900: Color(argb: 0xFF29_4073),
    950: Color(argb: 0xFF14_244A),
  ])

  public static let omarchyTokyoNightLight = ColorSwatch([
    50: Color(argb: 0xFFF3_F7FE),
    100: Color(argb: 0xFFE4_ECFB),
    200: Color(argb: 0xFFCA_D8F3),
    300: Color(argb: 0xFFAA_BEE7),
    400: Color(argb: 0xFF82_9ACD),
    500: Color(argb: 0xFF65_7FB6),
    600: Color(argb: 0xFF4C_6499),
    700: Color(argb: 0xFF3F_5483),
    800: Color(argb: 0xFF32_456D),
    900: Color(argb: 0xFF29_385A),
    950: Color(argb: 0xFF1A_263F),
  ])

  public static let omarchyVantablackDark = ColorSwatch([
    50: Color(argb: 0xFFF7_F7F7),
    100: Color(argb: 0xFFED_EDED),
    200: Color(argb: 0xFFDB_DBDB),
    300: Color(argb: 0xFFC4_C4C4),
    400: Color(argb: 0xFFA5_A5A5),
    500: Color(argb: 0xFF8D_8D8D),
    600: Color(argb: 0xFF75_7575),
    700: Color(argb: 0xFF61_6161),
    800: Color(argb: 0xFF4D_4D4D),
    900: Color(argb: 0xFF3D_3D3D),
    950: Color(argb: 0xFF26_2626),
  ])

  public static let omarchyVantablackLight = ColorSwatch([
    50: Color(argb: 0xFFF7_F7F7),
    100: Color(argb: 0xFFEA_EAEA),
    200: Color(argb: 0xFFD4_D4D4),
    300: Color(argb: 0xFFB8_B8B8),
    400: Color(argb: 0xFF91_9191),
    500: Color(argb: 0xFF73_7373),
    600: Color(argb: 0xFF57_5757),
    700: Color(argb: 0xFF4B_4B4B),
    800: Color(argb: 0xFF3F_3F3F),
    900: Color(argb: 0xFF35_3535),
    950: Color(argb: 0xFF26_2626),
  ])

  public static let omarchyWhiteDark = ColorSwatch([
    50: Color(argb: 0xFFF7_F7F7),
    100: Color(argb: 0xFFEF_EFEF),
    200: Color(argb: 0xFFE1_E1E1),
    300: Color(argb: 0xFFD0_D0D0),
    400: Color(argb: 0xFFB8_B8B8),
    500: Color(argb: 0xFFA5_A5A5),
    600: Color(argb: 0xFF87_8787),
    700: Color(argb: 0xFF6E_6E6E),
    800: Color(argb: 0xFF56_5656),
    900: Color(argb: 0xFF42_4242),
    950: Color(argb: 0xFF26_2626),
  ])

  public static let omarchyWhiteLight = ColorSwatch([
    50: Color(argb: 0xFFF7_F7F7),
    100: Color(argb: 0xFFEC_ECEC),
    200: Color(argb: 0xFFD9_D9D9),
    300: Color(argb: 0xFFC1_C1C1),
    400: Color(argb: 0xFFA0_A0A0),
    500: Color(argb: 0xFF87_8787),
    600: Color(argb: 0xFF6E_6E6E),
    700: Color(argb: 0xFF5C_5C5C),
    800: Color(argb: 0xFF4A_4A4A),
    900: Color(argb: 0xFF3B_3B3B),
    950: Color(argb: 0xFF26_2626),
  ])

  public static let red = ColorSwatch([
    50: Color(argb: 0xFFFD_F2F2),
    100: Color(argb: 0xFFF9_E2E2),
    200: Color(argb: 0xFFF2_C1C1),
    300: Color(argb: 0xFFEB_A1A1),
    400: Color(argb: 0xFFE4_8181),
    500: Color(argb: 0xFFDD_6060),
    600: Color(argb: 0xFFD6_4040),
    700: Color(argb: 0xFFC4_342F),
    800: Color(argb: 0xFF7E_1F1C),
    900: Color(argb: 0xFF4B_1210),
    950: Color(argb: 0xFF31_0B0A),
  ])

  public static let redDark = ColorSwatch([
    50: Color(argb: 0xFFFD_F2F2),
    100: Color(argb: 0xFFF9_E2E2),
    200: Color(argb: 0xFFF2_C1C1),
    300: Color(argb: 0xFFFF_8F8F),
    400: Color(argb: 0xFFE4_8181),
    500: Color(argb: 0xFFFF_6B6B),
    600: Color(argb: 0xFFD6_4040),
    700: Color(argb: 0xFFC4_342F),
    800: Color(argb: 0xFF7E_1F1C),
    900: Color(argb: 0xFF4B_1210),
    950: Color(argb: 0xFF31_0B0A),
  ])

  public static let sky = ColorSwatch([
    50: Color(argb: 0xFFF0_F9FF),
    100: Color(argb: 0xFFDF_F2FE),
    200: Color(argb: 0xFFB8_E6FE),
    300: Color(argb: 0xFF74_D4FF),
    400: Color(argb: 0xFF00_BCFF),
    500: Color(argb: 0xFF00_A6F4),
    600: Color(argb: 0xFF00_84D1),
    700: Color(argb: 0xFF00_69A8),
    800: Color(argb: 0xFF00_598A),
    900: Color(argb: 0xFF02_4A70),
    950: Color(argb: 0xFF05_2F4A),
  ])

  public static let ubuntuDark = ColorSwatch([
    50: Color(argb: 0xFFFF_F4F1),
    100: Color(argb: 0xFFFD_E7E1),
    200: Color(argb: 0xFFFE_CFC0),
    300: Color(argb: 0xFFFD_AE96),
    400: Color(argb: 0xFFFD_774D),
    500: Color(argb: 0xFFE9_5420),
    600: Color(argb: 0xFFC5_4313),
    700: Color(argb: 0xFFA7_3201),
    800: Color(argb: 0xFF84_2B0B),
    900: Color(argb: 0xFF6B_2108),
    950: Color(argb: 0xFF47_1101),
  ])

  public static let ubuntuLight = ColorSwatch([
    50: Color(argb: 0xFFFF_F4F1),
    100: Color(argb: 0xFFFE_EAE4),
    200: Color(argb: 0xFFFD_D6CA),
    300: Color(argb: 0xFFFD_BCA8),
    400: Color(argb: 0xFFF9_9678),
    500: Color(argb: 0xFFF8_7248),
    600: Color(argb: 0xFFE9_5420),
    700: Color(argb: 0xFFC3_3D06),
    800: Color(argb: 0xFF97_320D),
    900: Color(argb: 0xFF77_2304),
    950: Color(argb: 0xFF47_1101),
  ])

  public static let windows11Dark = ColorSwatch([
    50: Color(argb: 0xFFED_F9FF),
    100: Color(argb: 0xFFE4_F5FE),
    200: Color(argb: 0xFFCF_EEFF),
    300: Color(argb: 0xFFB3_E5FF),
    400: Color(argb: 0xFF8E_D7FC),
    500: Color(argb: 0xFF60_CDFF),
    600: Color(argb: 0xFF35_A8D8),
    700: Color(argb: 0xFF18_88B3),
    800: Color(argb: 0xFF16_6889),
    900: Color(argb: 0xFF08_4F69),
    950: Color(argb: 0xFF06_2A3A),
  ])

  public static let windows11Light = ColorSwatch([
    50: Color(argb: 0xFFF2_F7FF),
    100: Color(argb: 0xFFDE_EDFF),
    200: Color(argb: 0xFFBD_D9FD),
    300: Color(argb: 0xFF8F_BFFC),
    400: Color(argb: 0xFF55_9AEE),
    500: Color(argb: 0xFF2C_7CD7),
    600: Color(argb: 0xFF00_5FB8),
    700: Color(argb: 0xFF0E_5198),
    800: Color(argb: 0xFF08_4380),
    900: Color(argb: 0xFF08_376A),
    950: Color(argb: 0xFF05_264C),
  ])

}
