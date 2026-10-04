// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

import CSettings

extension SettingsState {
  /// Reads a snapshot from Rust and hands its memory back.
  init(reading app: OpaquePointer) {
    let dto = settings_get_state(app)
    defer { settings_state_free(dto) }
    self.init(
      launchAtStartup: dto.launch_at_startup,
      autoUpdate: dto.auto_update,
      saved: dto.saved,
      saves: dto.saves,
      status: decode(dto.status)
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
    }
  }
}

private func decode(_ text: NuiStr) -> String {
  guard let ptr = text.ptr, text.len > 0 else { return "" }
  return String(decoding: UnsafeBufferPointer(start: ptr, count: text.len), as: UTF8.self)
}
