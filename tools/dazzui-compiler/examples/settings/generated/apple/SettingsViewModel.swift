// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

import Combine

public enum SettingsAction: Equatable {
  case setLaunchAtStartup(Bool)
  case setAutoUpdate(Bool)
  case save
  case reset
}

/// Holds what the page reads and forwards what it sends. Until the
/// bridge to Rust is generated, the host supplies where actions go.
@MainActor
public final class SettingsViewModel: ObservableObject {
  private let sink: (SettingsAction) -> Void

  public init(send: @escaping (SettingsAction) -> Void) {
    self.sink = send
  }

  public func send(_ action: SettingsAction) {
    sink(action)
  }
}
