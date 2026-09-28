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
}
