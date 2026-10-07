// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/model.rs

/// The methods of `SettingsModel` a page calls. The Rust model's native object has
/// them; a host, or a test, may implement them itself.
public protocol SettingsActions {
  func setLaunchAtStartup(on: Bool)
  func setAutoUpdate(on: Bool)
  func setShowDisabledPlugins(on: Bool)
  func togglePlugin(id: UInt32)
  func save()
  func reset()
}
