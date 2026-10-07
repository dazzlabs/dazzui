// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/model.rs

import Combine
import Foundation

extension SettingsModel: SettingsActions {}

/// The latest value of a `SettingsModel`, observable, and the model itself: what
/// a page takes, as `ListenableBuilder` hands a widget. Whatever thread the
/// model changes on, the value is read again on the main one.
@MainActor
public final class SettingsStore: ObservableObject {
  @Published public private(set) var state: SettingsState
  public let model: SettingsModel

  public init(model: SettingsModel = SettingsModel()) {
    self.model = model
    state = model.value()
    _ = model.addListener(listener: Listener(store: self))
  }

  fileprivate func refresh() {
    state = model.value()
  }
}

/// Rust calls this back on the thread that changed the value. On the main
/// one, the store reads it at once, so a model that changes as a method
/// runs is shown changed when the call returns; from any other, it reads it
/// there next. It holds the store weakly: Rust holds it.
private final class Listener: SettingsListener, @unchecked Sendable {
  weak var store: SettingsStore?

  init(store: SettingsStore) {
    self.store = store
  }

  func changed() {
    if Thread.isMainThread {
      MainActor.assumeIsolated { store?.refresh() }
    } else {
      DispatchQueue.main.async { [weak self] in
        MainActor.assumeIsolated { self?.store?.refresh() }
      }
    }
  }
}
