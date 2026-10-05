import SwiftUI

/// Hands the generated page a state and a `send`. Here both come from the
/// Rust model, through the store `dazzui-compiler` generated over it; the
/// page itself would take them from anywhere. Each snapshot is printed, so
/// a change can be seen to have come from Rust.
@main
struct SettingsExampleApp: App {
  @StateObject private var store = SettingsStore()

  var body: some Scene {
    WindowGroup("Settings") {
      SettingsPage(state: store.state, send: store.send)
        .padding(24)
        .frame(minWidth: 360, alignment: .topLeading)
        .onReceive(store.$state) { print("state:", $0) }
    }
  }
}
