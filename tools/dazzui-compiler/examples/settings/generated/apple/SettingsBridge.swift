// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

import CSettings

extension SettingsState {
  /// Reads a snapshot from Rust and hands its memory back.
  init(reading app: OpaquePointer) {
    let dto = settings_get_state(app)
    defer { settings_state_free(dto) }
    self.init(dto: dto)
  }
}

extension Plugin {
  init(dto: SettingsPluginDto) {
    self.init(
      id: dto.id,
      name: decode(dto.name),
      enabled: dto.enabled
    )
  }
}

extension SettingsState {
  init(dto: SettingsStateDto) {
    self.init(
      launchAtStartup: dto.launch_at_startup,
      autoUpdate: dto.auto_update,
      saved: dto.saved,
      saves: dto.saves,
      status: decode(dto.status),
      showDisabledPlugins: dto.show_disabled_plugins,
      plugins: UnsafeBufferPointer(start: dto.plugins.ptr, count: dto.plugins.len).map(Plugin.init(dto:)),
      visiblePlugins: UnsafeBufferPointer(start: dto.visible_plugins.ptr, count: dto.visible_plugins.len).map(Plugin.init(dto:)),
      hasHiddenPlugins: dto.has_hidden_plugins
    )
  }
}

extension SettingsAction {
  /// Lays the action out as Rust reads it, for the length of `body`.
  func withDto(_ body: (SettingsActionDto) -> Void) {
    switch self {
    case .setLaunchAtStartup(let value):
      body(SettingsActionDto(tag: 0, payload: SettingsActionPayload(set_launch_at_startup: value)))
    case .setAutoUpdate(let value):
      body(SettingsActionDto(tag: 1, payload: SettingsActionPayload(set_auto_update: value)))
    case .save:
      body(SettingsActionDto(tag: 2, payload: SettingsActionPayload()))
    case .reset:
      body(SettingsActionDto(tag: 3, payload: SettingsActionPayload()))
    case .setShowDisabledPlugins(let value):
      body(SettingsActionDto(tag: 4, payload: SettingsActionPayload(set_show_disabled_plugins: value)))
    case .togglePlugin(let value):
      body(SettingsActionDto(tag: 5, payload: SettingsActionPayload(toggle_plugin: value)))
    }
  }
}

private func decode(_ text: NuiStr) -> String {
  guard let ptr = text.ptr, text.len > 0 else { return "" }
  return String(decoding: UnsafeBufferPointer(start: ptr, count: text.len), as: UTF8.self)
}
