import XCTest
import SwiftUI
@testable import DazzMobileUI

final class PaletteTests: XCTestCase {
  func testEveryFamilyHasLightAndDarkPalettes() {
    for family in ThemeFamily.allCases {
      let light = family.palette(for: .light)
      let dark = family.palette(for: .dark)
      XCTAssertNotEqual(light.colorSurface, dark.colorSurface, family.rawValue)
      XCTAssertNotEqual(light.colorContent, dark.colorContent, family.rawValue)
      XCTAssertFalse(light.isDark, family.rawValue)
      XCTAssertTrue(dark.isDark, family.rawValue)
      for palette in [light, dark] {
        for ramp in [palette.colorPrimary, palette.colorNeutral, palette.colorInfo,
                     palette.colorSuccess, palette.colorWarning, palette.colorDanger] {
          XCTAssertEqual(Set(ramp.shades.keys), Set([50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950]))
        }
        XCTAssertNotNil(palette.colorPrimary.shades[palette.nativeTintShade])
        XCTAssertNotEqual(palette.accent, .clear)
      }
    }
  }

  func testBrightChangesItsAccentRampInDarkMode() {
    XCTAssertNotEqual(Palette.brightLight.colorPrimary, Palette.brightDark.colorPrimary)
  }

  func testARGBPreservesAlphaAndChannelOrder() {
    XCTAssertEqual(Color(argb: 0x80402010), Color(.sRGB,
      red: 64.0 / 255, green: 32.0 / 255, blue: 16.0 / 255, opacity: 128.0 / 255))
  }

  func testAppearanceModeMappingAndCodable() throws {
    XCTAssertEqual(AppearanceMode.allCases, [.system, .light, .dark])
    XCTAssertNil(AppearanceMode.system.colorScheme)
    XCTAssertEqual(AppearanceMode.light.colorScheme, .light)
    XCTAssertEqual(AppearanceMode.dark.colorScheme, .dark)

    for mode in AppearanceMode.allCases {
      XCTAssertEqual(mode.id, mode.rawValue)
      let data = try JSONEncoder().encode(mode)
      let decoded = try JSONDecoder().decode(AppearanceMode.self, from: data)
      XCTAssertEqual(decoded, mode)
    }
  }

  func testThemeFamilyPaletteResolvesAppearanceMode() {
    for family in ThemeFamily.drawn {
      let light = family.palette(for: .light)
      let dark = family.palette(for: .dark)
      XCTAssertEqual(
        family.palette(for: .system, systemColorScheme: .light).colorSurface,
        light.colorSurface
      )
      XCTAssertEqual(
        family.palette(for: .system, systemColorScheme: .dark).colorSurface,
        dark.colorSurface
      )
      XCTAssertEqual(
        family.palette(for: .light, systemColorScheme: .dark).colorSurface,
        light.colorSurface
      )
      XCTAssertEqual(
        family.palette(for: .dark, systemColorScheme: .light).colorSurface,
        dark.colorSurface
      )
    }
  }

  func testDerivedSemanticColorsAcrossAllFamilies() {
    for family in ThemeFamily.allCases {
      for scheme in [ColorScheme.light, ColorScheme.dark] {
        let palette = family.palette(for: scheme)
        let textShade = scheme == .dark ? 300 : 700

        XCTAssertEqual(palette.onAccent, palette.colorOnAccent)
        XCTAssertEqual(palette.accentText, palette.colorPrimary[textShade])
        XCTAssertEqual(palette.accentSurface, palette.accent.opacity(0.12))
        XCTAssertEqual(palette.accentHairline, palette.accent.opacity(0.25))

        XCTAssertEqual(palette.danger, palette.colorDanger[palette.nativeTintShade])
        XCTAssertEqual(palette.dangerText, palette.colorDanger[textShade])
        XCTAssertEqual(palette.dangerSurface, palette.danger.opacity(0.12))
        XCTAssertEqual(palette.dangerHairline, palette.danger.opacity(0.25))

        XCTAssertEqual(palette.warn, palette.colorWarning[palette.nativeTintShade])
        XCTAssertEqual(palette.warnText, palette.colorWarning[textShade])
        XCTAssertEqual(palette.warnSurface, palette.warn.opacity(0.12))
        XCTAssertEqual(palette.warnHairline, palette.warn.opacity(0.25))

        XCTAssertEqual(palette.success, palette.colorSuccess[palette.nativeTintShade])
        XCTAssertEqual(palette.successText, palette.colorSuccess[textShade])
        XCTAssertEqual(palette.successSurface, palette.success.opacity(0.12))

        XCTAssertEqual(palette.info, palette.colorInfo[palette.nativeTintShade])
        XCTAssertEqual(palette.infoText, palette.colorInfo[textShade])
        XCTAssertEqual(palette.infoSurface, palette.info.opacity(0.12))

        XCTAssertEqual(palette.separator, palette.colorBorder)
        XCTAssertEqual(palette.hairlineStrong, palette.colorBorderStrong)
        XCTAssertEqual(palette.grouped, palette.colorSurface)
        XCTAssertEqual(palette.cell, palette.colorSurfaceMuted)
      }
    }
  }

  func testSwatchColorsAndMetadata() {
    XCTAssertEqual(
      ThemeFamily.drawn,
      [.bright, .studio, .frost, .graphite, .ember, .nocturne]
    )
    for family in ThemeFamily.allCases {
      XCTAssertFalse(family.displayName.isEmpty, family.rawValue)
      XCTAssertEqual(family.isDrawn, ThemeFamily.drawn.contains(family))

      let lightSwatch = family.swatchColors(for: .light)
      let darkSwatch = family.swatchColors(for: .dark)
      XCTAssertNotEqual(lightSwatch, darkSwatch, family.rawValue)
      XCTAssertEqual(
        family.swatchColors(for: .system, systemColorScheme: .dark),
        darkSwatch
      )
      XCTAssertEqual(
        family.swatchColors(for: .light, systemColorScheme: .dark),
        lightSwatch
      )
    }
  }

  func testMobileThemeViewModifierOverloadsCompile() {
    let defaultThemed = Text("Hello").mobileTheme(.studio)
    let schemeThemed = Text("Hello").mobileTheme(.studio, colorScheme: .dark)
    let modeThemed = Text("Hello").mobileTheme(.studio, mode: .system)
    XCTAssertNotNil(defaultThemed)
    XCTAssertNotNil(schemeThemed)
    XCTAssertNotNil(modeThemed)
  }
}
