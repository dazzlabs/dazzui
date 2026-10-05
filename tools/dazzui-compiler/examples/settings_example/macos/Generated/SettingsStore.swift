// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/model.rs

import Combine
import Foundation

/// The latest snapshot of the model `settings` makes, and a way to send it
/// actions: what a page takes. Whatever thread the model changes on, the
/// snapshot is read again on the main one.
@MainActor
public final class SettingsStore: ObservableObject {
  @Published public private(set) var state: SettingsState
  private let source: SettingsSource

  public init(source: SettingsSource = SettingsSource()) {
    self.source = source
    state = source.state()
    source.subscribe(listener: Listener(store: self))
  }

  public func send(_ action: SettingsAction) {
    source.send(action: action)
  }

  fileprivate func refresh() {
    state = source.state()
  }
}

/// Rust calls this back on the thread that changed the state. On the main
/// one, the store reads it at once, so a model that changes as it takes an
/// action is shown changed when `send` returns; from any other, it reads it
/// there next. It holds the store weakly: Rust holds it.
private final class Listener: SettingsListener, @unchecked Sendable {
  weak var store: SettingsStore?

  init(store: SettingsStore) {
    self.store = store
  }

  func stateChanged() {
    if Thread.isMainThread {
      MainActor.assumeIsolated { store?.refresh() }
    } else {
      DispatchQueue.main.async { [weak self] in
        MainActor.assumeIsolated { self?.store?.refresh() }
      }
    }
  }
}
