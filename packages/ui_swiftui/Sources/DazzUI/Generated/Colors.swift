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

}
