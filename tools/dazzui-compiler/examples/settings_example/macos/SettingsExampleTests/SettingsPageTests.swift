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

    store.model.setShowDisabledPlugins(on: true)
    XCTAssertEqual(store.state.visiblePlugins.map(\.name), ["Spell check", "Grammar", "Translate"])
    XCTAssertFalse(store.state.hasHiddenPlugins)

    store.model.togglePlugin(id: 2)
    XCTAssertEqual(store.state.visiblePlugins[1], Plugin(id: 2, name: "Grammar", enabled: true))
  }

  /// A switch calls the model's method with its new value; the model
  /// changes as the method runs and notifies on this thread, so the store
  /// shows it at once.
  func testAMethodChangesTheRustState() {
    let store = SettingsStore()
    store.model.setAutoUpdate(on: true)
    XCTAssertTrue(store.state.autoUpdate)
    XCTAssertFalse(store.state.saved)
    XCTAssertEqual(store.state.status, "Unsaved changes")

    store.model.reset()
    XCTAssertFalse(store.state.autoUpdate)
    XCTAssertTrue(store.state.saved)
  }

  /// A save finishes on a Rust thread of its own; the store hears of it
  /// there and reads the value again on the main one.
  func testAChangeFromAnotherThreadReachesTheStore() {
    let store = SettingsStore()
    store.model.setAutoUpdate(on: true)
    store.model.save()
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
    let host = NSHostingView(rootView: SettingsPage(state: store.state, model: store.model))
    host.frame = NSRect(x: 0, y: 0, width: 360, height: 240)
    host.layoutSubtreeIfNeeded()
    XCTAssertGreaterThan(host.fittingSize.height, 0)
  }

  /// The page takes the model only as the protocol of its methods: one
  /// written in Swift, which records the calls, is all it needs.
  func testThePageNeedsNoRustModel() {
    final class Recorder: SettingsActions {
      var calls: [String] = []
      func setLaunchAtStartup(on: Bool) { calls.append("launch \(on)") }
      func setAutoUpdate(on: Bool) { calls.append("update \(on)") }
      func setShowDisabledPlugins(on: Bool) { calls.append("show \(on)") }
      func togglePlugin(id: UInt32) { calls.append("toggle \(id)") }
      func save() { calls.append("save") }
      func reset() { calls.append("reset") }
    }
    let recorder = Recorder()
    let page = SettingsPage(state: SettingsStore().state, model: recorder)
    page.model.save()
    page.model.togglePlugin(id: 3)
    XCTAssertEqual(recorder.calls, ["save", "toggle 3"])
  }
}
