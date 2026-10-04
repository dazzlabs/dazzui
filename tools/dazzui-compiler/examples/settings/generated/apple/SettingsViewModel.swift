// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

import CSettings
import Combine

/// `SettingsState`, as Rust last handed it over.
public struct SettingsState: Equatable {
  public let launchAtStartup: Bool
  public let autoUpdate: Bool
  public let saved: Bool
  public let saves: UInt32
  public let status: String
}

public enum SettingsAction: Equatable {
  case setLaunchAtStartup(Bool)
  case setAutoUpdate(Bool)
  case save
  case reset
}

/// Holds the latest snapshot of the Rust state and sends actions to
/// Rust's `update`, which runs on the calling thread before `send` returns.
@MainActor
public final class SettingsViewModel: ObservableObject {
  @Published public private(set) var state: SettingsState
  private let app: OpaquePointer

  public init() {
    let app: OpaquePointer = settings_create()
    self.app = app
    state = SettingsState(reading: app)
    settings_subscribe(app, settingsStateChanged, Unmanaged.passUnretained(self).toOpaque())
  }

  deinit {
    settings_destroy(app)
  }

  public func send(_ action: SettingsAction) {
    action.withDto { settings_send(app, $0) }
  }

  fileprivate func refresh() {
    state = SettingsState(reading: app)
  }
}

/// Rust calls this back on the thread that sent the action, which for a
/// view model is the main one.
private let settingsStateChanged: @convention(c) (UnsafeMutableRawPointer?) -> Void = { context in
  guard let context else { return }
  let model = Unmanaged<SettingsViewModel>.fromOpaque(context).takeUnretainedValue()
  MainActor.assumeIsolated { model.refresh() }
}
