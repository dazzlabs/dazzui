// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

import DazzUI
import SwiftUI

public struct SettingsPage: View {
  public let state: SettingsState
  public let model: any SettingsActions
  @Environment(\.theme) private var theme

  public init(state: SettingsState, model: any SettingsActions) {
    self.state = state
    self.model = model
  }

  public var body: some View {
    // source: settings.rs:7
    VStack(alignment: .leading, spacing: theme.vars.spacing6) {
      // source: settings.rs:8
      DazzUI.Preferences {
        // source: settings.rs:9
        DazzUI.PreferenceSection(label: "General") {
          // source: settings.rs:10
          DazzUI.PreferenceRow(title: "Launch at startup", trailing: {
            // source: settings.rs:12
            DazzUI.Switch(checked: state.launchAtStartup, onCheckedChange: { model.setLaunchAtStartup(on: $0) })
          })
          // source: settings.rs:17
          DazzUI.PreferenceRow(title: "Update automatically", subtitle: "Download new versions in the background", trailing: {
            // source: settings.rs:20
            DazzUI.Switch(checked: state.autoUpdate, onCheckedChange: { model.setAutoUpdate(on: $0) })
          })
        }
        // source: settings.rs:23
        DazzUI.PreferenceSection(label: "Plugins", footer: "Plugins run in a sandbox and can be turned off at any time.") {
          // source: settings.rs:27
          DazzUI.PreferenceRow(title: "Show disabled plugins", trailing: {
            // source: settings.rs:29
            DazzUI.Switch(checked: state.showDisabledPlugins, onCheckedChange: { model.setShowDisabledPlugins(on: $0) })
          })
        }
      }
      // source: settings.rs:37
      VStack(alignment: .leading, spacing: theme.vars.spacing2) {
        // source: settings.rs:38
        ForEach(state.visiblePlugins, id: \.id) { plugin in
          // source: settings.rs:39
          HStack(spacing: theme.vars.spacing2) {
            // source: settings.rs:40
            Text(plugin.name)
            // source: settings.rs:41
            if plugin.enabled {
              // source: settings.rs:42
              DazzUI.Button(action: { model.togglePlugin(id: plugin.id) }) {
                // source: settings.rs:42
                Text("Disable")
              }
            } else {
              // source: settings.rs:44
              DazzUI.Button(variant: .filled, action: { model.togglePlugin(id: plugin.id) }) {
                // source: settings.rs:47
                Text("Enable")
              }
            }
          }
        }
        // source: settings.rs:52
        if state.hasHiddenPlugins {
          // source: settings.rs:53
          Text("Disabled plugins are hidden")
            .foregroundStyle(theme.vars.colorContentSecondary)
        }
      }
        .frame(maxWidth: .infinity, alignment: .leading)
      // source: settings.rs:56
      Text(state.status)
        .foregroundStyle(theme.vars.colorContentSecondary)
      // source: settings.rs:57
      HStack(spacing: theme.vars.spacing2) {
        // source: settings.rs:58
        DazzUI.Button(variant: .filled, enabled: !state.saveDisabled, action: { model.save() }) {
          // source: settings.rs:62
          Text("Save")
        }
        // source: settings.rs:64
        DazzUI.Button(action: { model.reset() }) {
          // source: settings.rs:64
          Text("Reset to defaults")
        }
      }
    }
      .frame(maxWidth: .infinity, alignment: .leading)
  }
}
