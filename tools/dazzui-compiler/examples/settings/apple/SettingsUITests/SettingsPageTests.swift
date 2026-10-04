import AppKit
import SwiftUI
import XCTest

@testable import SettingsUI

@MainActor
final class SettingsPageTests: XCTestCase {
  func testTheFirstSnapshotIsRustsDefault() {
    let model = SettingsViewModel()
    XCTAssertEqual(
      model.state,
      SettingsState(
        launchAtStartup: false, autoUpdate: false, saved: true, saves: 0, status: "No changes")
    )
  }

  /// A toggle sends its new value; Rust's `update` changes the state and
  /// calls back; the next snapshot is what the page shows.
  func testAnActionChangesTheRustState() {
    let model = SettingsViewModel()
    model.send(.setAutoUpdate(true))
    XCTAssertTrue(model.state.autoUpdate)
    XCTAssertFalse(model.state.saved)
    XCTAssertEqual(model.state.status, "Unsaved changes")

    model.send(.save)
    model.send(.save)
    XCTAssertEqual(model.state.saves, 2)
    XCTAssertEqual(model.state.status, "Saved 2 times")

    model.send(.reset)
    XCTAssertFalse(model.state.autoUpdate)
    XCTAssertEqual(model.state.saves, 0)
  }

  func testThePageLaysOut() {
    let host = NSHostingView(rootView: SettingsPage(model: SettingsViewModel()))
    host.frame = NSRect(x: 0, y: 0, width: 360, height: 240)
    host.layoutSubtreeIfNeeded()
    XCTAssertGreaterThan(host.fittingSize.height, 0)
  }
}
