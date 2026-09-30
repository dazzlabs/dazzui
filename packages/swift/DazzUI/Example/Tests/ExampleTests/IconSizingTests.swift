import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

final class IconSizingTests: XCTestCase {
  func testThemeScaleAndControlProfiles() {
    for name in ThemeName.allCases {
      let vars = name.variables
      let wide = ["windows11", "ubuntu", "debian", "fedora"].contains {
        name.rawValue.hasPrefix($0 + "-")
      }
      XCTAssertEqual(vars.iconSmall, wide ? 14 : 12, name.rawValue)
      XCTAssertEqual(vars.iconMedium, wide ? 20 : 16, name.rawValue)
      XCTAssertEqual(vars.iconLarge, wide ? 24 : 20, name.rawValue)
      XCTAssertEqual(vars.metrics(.tiny).iconSize, vars.iconSmall)
      XCTAssertEqual(vars.metrics(.small).iconSize, vars.iconMedium)
      XCTAssertEqual(vars.metrics(.medium).iconSize, vars.iconMedium)
      XCTAssertEqual(vars.metrics(.large).iconSize, vars.iconLarge)
    }
  }

  @MainActor func testRenderedIconSizeAndOverrides() {
    for name in [ThemeName.studioLight, .windows11Light, .windows11Dark, .macos15Dark] {
      let vars = name.variables
      let normal = NSHostingView(rootView: Icon(Image(systemName: "plus")).theme(name))
      XCTAssertEqual(normal.fittingSize, NSSize(width: vars.iconMedium, height: vars.iconMedium))
      let slot = NSHostingView(
        rootView: Icon(Image(systemName: "plus"))
          .environment(\.dazzIconSize, vars.iconSmall).theme(name))
      XCTAssertEqual(slot.fittingSize, NSSize(width: vars.iconSmall, height: vars.iconSmall))
      let explicit = NSHostingView(
        rootView: Icon(Image(systemName: "plus"), size: 10)
          .environment(\.dazzIconSize, vars.iconLarge).theme(name))
      XCTAssertEqual(explicit.fittingSize, NSSize(width: 10, height: 10))
    }
  }
}
