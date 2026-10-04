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
      // source: settings.rs:24
      Text("Plugins")
        .font(theme.vars.titleMedium.font)
        .lineSpacing(theme.vars.titleMedium.lineSpacing)
      // source: settings.rs:25
      HStack(spacing: 8) {
        // source: settings.rs:26
        DazzUI.Switch(checked: model.state.showDisabledPlugins, onCheckedChange: { model.send(.setShowDisabledPlugins($0)) })
        // source: settings.rs:30
        Text("Show disabled plugins")
      }
      // source: settings.rs:32
      ForEach(model.state.visiblePlugins, id: \.id) { plugin in
        // source: settings.rs:33
        HStack(spacing: 8) {
          // source: settings.rs:34
          Text(plugin.name)
          // source: settings.rs:35
          if plugin.enabled {
            // source: settings.rs:36
            DazzUI.Button(action: { model.send(.togglePlugin(plugin.id)) }) {
              Text("Disable")
            }
          } else {
            // source: settings.rs:38
            DazzUI.Button(variant: .filled, action: { model.send(.togglePlugin(plugin.id)) }) {
              Text("Enable")
            }
          }
        }
      }
      // source: settings.rs:44
      if model.state.hasHiddenPlugins {
        // source: settings.rs:45
        Text("Disabled plugins are hidden")
          .font(theme.vars.captionMedium.font)
          .lineSpacing(theme.vars.captionMedium.lineSpacing)
      }
      // source: settings.rs:48
      Text(model.state.status)
        .font(theme.vars.captionMedium.font)
        .lineSpacing(theme.vars.captionMedium.lineSpacing)
      // source: settings.rs:49
      HStack(spacing: 8) {
        // source: settings.rs:50
        DazzUI.Button(variant: .filled, enabled: !model.state.saved, action: { model.send(.save) }) {
          Text("Save")
        }
        // source: settings.rs:54
        DazzUI.Button(action: { model.send(.reset) }) {
          Text("Reset to defaults")
        }
      }
    }
  }
}
