import SettingsUI
import SwiftUI

/// Runs the generated page. Until the bridge to Rust is generated, an
/// action is only printed.
@main
struct SettingsHostApp: App {
  @StateObject private var model = SettingsViewModel { action in
    print("action:", action)
  }

  var body: some Scene {
    WindowGroup("Settings") {
      SettingsPage(model: model)
        .padding(24)
        .frame(minWidth: 360, alignment: .topLeading)
    }
  }
}
