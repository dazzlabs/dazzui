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
    VStack(spacing: 24) {
      // source: settings.rs:8
      DazzUI.Preferences {
        // source: settings.rs:9
        DazzUI.PreferenceSection(label: "General") {
          // source: settings.rs:10
          DazzUI.PreferenceRow(title: "Launch at startup", trailing: {
            // source: settings.rs:12
            DazzUI.Switch(checked: model.state.launchAtStartup, onCheckedChange: { model.send(.setLaunchAtStartup($0)) })
          })
          // source: settings.rs:17
          DazzUI.PreferenceRow(title: "Update automatically", subtitle: "Download new versions in the background", trailing: {
            // source: settings.rs:20
            DazzUI.Switch(checked: model.state.autoUpdate, onCheckedChange: { model.send(.setAutoUpdate($0)) })
          })
        }
        // source: settings.rs:26
        DazzUI.PreferenceSection(label: "Plugins", footer: "Plugins run in a sandbox and can be turned off at any time.") {
          // source: settings.rs:27
          DazzUI.PreferenceRow(title: "Show disabled plugins", trailing: {
            // source: settings.rs:29
            DazzUI.Switch(checked: model.state.showDisabledPlugins, onCheckedChange: { model.send(.setShowDisabledPlugins($0)) })
          })
        }
      }
      // source: settings.rs:37
      VStack(spacing: 8) {
        // source: settings.rs:38
        ForEach(model.state.visiblePlugins, id: \.id) { plugin in
          // source: settings.rs:39
          HStack(spacing: 8) {
            // source: settings.rs:40
            Text(plugin.name)
            // source: settings.rs:41
            if plugin.enabled {
              // source: settings.rs:42
              DazzUI.Button(action: { model.send(.togglePlugin(plugin.id)) }) {
                Text("Disable")
              }
            } else {
              // source: settings.rs:44
              DazzUI.Button(variant: .filled, action: { model.send(.togglePlugin(plugin.id)) }) {
                Text("Enable")
              }
            }
          }
        }
        // source: settings.rs:50
        if model.state.hasHiddenPlugins {
          // source: settings.rs:51
          Text("Disabled plugins are hidden")
            .font(theme.vars.captionMedium.font)
            .lineSpacing(theme.vars.captionMedium.lineSpacing)
        }
      }
      // source: settings.rs:55
      Text(model.state.status)
        .font(theme.vars.captionMedium.font)
        .lineSpacing(theme.vars.captionMedium.lineSpacing)
      // source: settings.rs:56
      HStack(spacing: 8) {
        // source: settings.rs:57
        DazzUI.Button(variant: .filled, enabled: !model.state.saved, action: { model.send(.save) }) {
          Text("Save")
        }
        // source: settings.rs:61
        DazzUI.Button(action: { model.send(.reset) }) {
          Text("Reset to defaults")
        }
      }
    }
  }
}
