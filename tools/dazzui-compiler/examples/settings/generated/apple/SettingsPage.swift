// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

import DazzUI
import SwiftUI

public struct SettingsPage: View {
  @ObservedObject public var model: SettingsViewModel
  @Environment(\.theme) private var theme

  public init(model: SettingsViewModel) {
    self.model = model
  }

  public var body: some View {
    // source: settings.rs:7
    VStack(spacing: 16) {
      // source: settings.rs:8
      Text("General")
        .font(theme.vars.titleMedium.font)
        .lineSpacing(theme.vars.titleMedium.lineSpacing)
      // source: settings.rs:9
      HStack(spacing: 8) {
        // source: settings.rs:10
        DazzUI.Switch(checked: model.state.launchAtStartup, onCheckedChange: { model.send(.setLaunchAtStartup($0)) })
        // source: settings.rs:14
        Text("Launch at startup")
      }
      // source: settings.rs:16
      HStack(spacing: 8) {
        // source: settings.rs:17
        DazzUI.Switch(checked: model.state.autoUpdate, onCheckedChange: { model.send(.setAutoUpdate($0)) })
        // source: settings.rs:21
        Text("Update automatically")
      }
      // source: settings.rs:23
      Text(model.state.status)
        .font(theme.vars.captionMedium.font)
        .lineSpacing(theme.vars.captionMedium.lineSpacing)
      // source: settings.rs:24
      HStack(spacing: 8) {
        // source: settings.rs:25
        DazzUI.Button(variant: .filled, enabled: !model.state.saved, action: { model.send(.save) }) {
          Text("Save")
        }
        // source: settings.rs:29
        DazzUI.Button(action: { model.send(.reset) }) {
          Text("Reset to defaults")
        }
      }
    }
  }
}
