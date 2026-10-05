import AppKit
import Combine
import SwiftUI
import XCTest

@testable import SettingsExample

@MainActor
final class SettingsPageTests: XCTestCase {
  func testTheFirstSnapshotIsRustsDefault() {
    let state = SettingsStore().state
    XCTAssertEqual(state.saved, true)
    XCTAssertEqual(state.status, "No changes")
    XCTAssertEqual(state.plugins.map(\.name), ["Spell check", "Grammar", "Translate"])
  }

  /// A selector crosses as a field, recomputed in Rust for every snapshot.
  func testSelectorsAndListsCross() {
    let store = SettingsStore()
    XCTAssertEqual(store.state.visiblePlugins.map(\.id), [1, 3])
    XCTAssertTrue(store.state.hasHiddenPlugins)

    store.send(.setShowDisabledPlugins(true))
    XCTAssertEqual(store.state.visiblePlugins.map(\.name), ["Spell check", "Grammar", "Translate"])
    XCTAssertFalse(store.state.hasHiddenPlugins)

    store.send(.togglePlugin(2))
    XCTAssertEqual(store.state.visiblePlugins[1], Plugin(id: 2, name: "Grammar", enabled: true))
  }

  /// A toggle sends its new value; the model changes the state as it takes
  /// it and calls back on this thread, so the store shows it at once.
  func testAnActionChangesTheRustState() {
    let store = SettingsStore()
    store.send(.setAutoUpdate(true))
    XCTAssertTrue(store.state.autoUpdate)
    XCTAssertFalse(store.state.saved)
    XCTAssertEqual(store.state.status, "Unsaved changes")

    store.send(.reset)
    XCTAssertFalse(store.state.autoUpdate)
    XCTAssertTrue(store.state.saved)
  }

  /// A save finishes on a Rust thread of its own; the store hears of it
  /// there and reads the state again on the main one.
  func testAChangeFromAnotherThreadReachesTheStore() {
    let store = SettingsStore()
    store.send(.setAutoUpdate(true))
    store.send(.save)
    XCTAssertTrue(store.state.saving)
    XCTAssertTrue(store.state.saveDisabled)

    let saved = expectation(description: "the save finished")
    let watching = store.$state.sink { state in
      if state.saved {
        XCTAssertTrue(Thread.isMainThread)
        saved.fulfill()
      }
    }
    wait(for: [saved], timeout: 5)
    watching.cancel()
    XCTAssertEqual(store.state.saves, 1)
    XCTAssertEqual(store.state.status, "Saved")
    XCTAssertFalse(store.state.saving)
  }

  func testThePageLaysOut() {
    let store = SettingsStore()
    let host = NSHostingView(rootView: SettingsPage(state: store.state, send: store.send))
    host.frame = NSRect(x: 0, y: 0, width: 360, height: 240)
    host.layoutSubtreeIfNeeded()
    XCTAssertGreaterThan(host.fittingSize.height, 0)
  }

  /// The page holds nothing: a state made in Swift, and a closure that
  /// collects what it sends, are all it needs.
  func testThePageNeedsNoModel() {
    var sent: [SettingsAction] = []
    let state = SettingsStore().state
    let page = SettingsPage(state: state, send: { sent.append($0) })
    page.send(.save)
    XCTAssertEqual(sent, [.save])
  }
}
