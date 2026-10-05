// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

import Combine

/// Holds the latest snapshot of the Rust state and sends actions to Rust's
/// `update`, which runs on the calling thread before `send` returns.
@MainActor
public final class SettingsViewModel: ObservableObject {
  @Published public private(set) var state: SettingsState
  private let app = SettingsApp()

  public init() {
    state = app.state()
    app.subscribe(listener: Listener(model: self))
  }

  public func send(_ action: SettingsAction) {
    app.send(action: action)
  }

  fileprivate func refresh() {
    state = app.state()
  }
}

/// Rust calls this back on the thread that sent the action, which for a
/// view model is the main one. It holds the model weakly: Rust holds it.
private final class Listener: SettingsListener, @unchecked Sendable {
  weak var model: SettingsViewModel?

  init(model: SettingsViewModel) {
    self.model = model
  }

  func stateChanged() {
    MainActor.assumeIsolated { model?.refresh() }
  }
}
