import SwiftUI

/// Runs the generated page against the Rust app core. Each snapshot the
/// page gets is printed, so a change can be seen to have come from Rust.
@main
struct SettingsExampleApp: App {
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
