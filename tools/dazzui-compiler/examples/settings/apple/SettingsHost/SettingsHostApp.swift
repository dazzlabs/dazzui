import AppKit
import SettingsUI
import SwiftUI

/// Runs the generated page against the Rust app core. Each snapshot the
/// page gets is printed, so a change can be seen to have come from Rust.
@main
struct SettingsHostApp: App {
  @NSApplicationDelegateAdaptor private var delegate: HostDelegate
  @StateObject private var model = SettingsViewModel()

  var body: some Scene {
    WindowGroup("Settings") {
      SettingsPage(model: model)
        .padding(24)
        .frame(minWidth: 360, alignment: .topLeading)
        .onReceive(model.$state) { print("state:", $0) }
    }
  }
}

/// A bare SwiftPM executable is not an app bundle, so macOS neither gives
/// it a Dock icon nor brings it forward when it launches: its window opens
/// behind whatever was in front. This asks for both.
final class HostDelegate: NSObject, NSApplicationDelegate {
  func applicationDidFinishLaunching(_ notification: Notification) {
    NSApp.setActivationPolicy(.regular)
    NSApp.activate(ignoringOtherApps: true)
  }
}
