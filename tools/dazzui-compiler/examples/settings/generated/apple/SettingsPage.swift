// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

import DazzUI
import SwiftUI

public struct SettingsPage: View {
  @ObservedObject public var model: SettingsViewModel

  public init(model: SettingsViewModel) {
    self.model = model
  }

  public var body: some View {
    // source: settings.rs:7
    VStack(spacing: 16) {
      // source: settings.rs:8
      Text("General")
      // source: settings.rs:9
      Text("Changes apply the next time the app starts.")
      // source: settings.rs:10
      VStack(spacing: 8) {
        // source: settings.rs:11
        DazzUI.Button(action: { model.send(.save) }) {
          Text("Save")
        }
        // source: settings.rs:12
        DazzUI.Button(action: { model.send(.reset) }) {
          Text("Reset to defaults")
        }
      }
    }
  }
}
