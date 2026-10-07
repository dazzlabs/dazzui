import SwiftUI

/// Hands the generated page a state and a model, as a `ListenableBuilder`
/// hands a widget its value. Here both come from the Rust model, through the
/// store `dazzui-compiler` generated over it; the page takes the model only
/// as `SettingsActions`, which anything may implement. Each snapshot is
/// printed, so a change can be seen to have come from Rust.
@main
struct SettingsExampleApp: App {
  @StateObject private var store = SettingsStore()

  var body: some Scene {
    WindowGroup("Settings") {
      SettingsPage(state: store.state, model: store.model)
        .padding(24)
        .frame(minWidth: 360, alignment: .topLeading)
        .onReceive(store.$state) { print("state:", $0) }
    }
  }
}
