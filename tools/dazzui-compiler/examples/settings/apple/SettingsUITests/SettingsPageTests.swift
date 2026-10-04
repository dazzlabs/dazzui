import AppKit
import SwiftUI
import XCTest

@testable import SettingsUI

@MainActor
final class SettingsPageTests: XCTestCase {
  func testTheViewModelForwardsWhatThePageSends() {
    var sent: [SettingsAction] = []
    let model = SettingsViewModel { sent.append($0) }
    model.send(.save)
    model.send(.setAutoUpdate(true))
    XCTAssertEqual(sent, [.save, .setAutoUpdate(true)])
  }

  func testThePageLaysOut() {
    let host = NSHostingView(rootView: SettingsPage(model: SettingsViewModel { _ in }))
    host.frame = NSRect(x: 0, y: 0, width: 360, height: 240)
    host.layoutSubtreeIfNeeded()
    XCTAssertGreaterThan(host.fittingSize.height, 0)
  }
}
