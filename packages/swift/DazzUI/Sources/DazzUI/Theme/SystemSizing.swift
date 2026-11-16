import Foundation
import SwiftUI

/// Runtime sizing is independent from the selected color family.
#if os(macOS)
  import AppKit
#endif
#if canImport(Combine)
  import Combine
#endif

public enum ThemeSizing: String, CaseIterable, Sendable {
  case design
  case system
}
internal struct SystemSizingMetrics: Sendable, Hashable {
  struct Control: Sendable, Hashable {
    let fontSize: CGFloat
    let height: CGFloat
  }

  let bodyFontSize: CGFloat
  let controls: [Control]

  func control(_ size: ControlSize) -> Control {
    controls[ControlSize.allCases.firstIndex(of: size)!]
  }

  func content(_ size: ControlSize, design: TypeStyle) -> TypeStyle {
    design.scaled(by: control(size).fontSize / design.size)
  }

  /// Measures the platform afresh: eight native buttons laid out on macOS.
  /// Views go through ``current()`` instead.
  @MainActor static func read() -> SystemSizingMetrics {
    #if os(macOS)
      let body = NSFont.preferredFont(forTextStyle: .body, options: [:]).pointSize
      let scale = body / NSFont.systemFontSize
      let controls = [NSControl.ControlSize.mini, .small, .regular, .large].map { size in
        let fontSize = NSFont.systemFontSize(for: size) * scale
        let button = NSButton(title: "Ag 保存", target: nil, action: nil)
        button.bezelStyle = .rounded
        button.controlSize = size
        button.font = .systemFont(ofSize: fontSize)
        button.sizeToFit()
        // Native bezels can retain their base height at larger text sizes.
        // Scale their native floor together with the preferred reading font.
        let base = NSButton(title: "Ag 保存", target: nil, action: nil)
        base.bezelStyle = .rounded
        base.controlSize = size
        base.font = .systemFont(ofSize: NSFont.systemFontSize(for: size))
        base.sizeToFit()
        return Control(
          fontSize: fontSize, height: max(button.frame.height, base.frame.height * scale))
      }
      return SystemSizingMetrics(bodyFontSize: body, controls: controls)
    #else
      let vars = Theme.default.vars
      return SystemSizingMetrics(
        bodyFontSize: vars.bodyMedium.size,
        controls: ControlSize.allCases.map {
          Control(fontSize: vars.metrics($0).content.size, height: vars.metrics($0).size)
        })
    #endif
  }

  /// The process's last reading, measured again only when the preferred body
  /// size has moved or a font, accessibility or activation notification has
  /// cleared it. Checking the body size is one font lookup; the buttons are
  /// laid out once per change rather than once per view, redraw or tick.
  @MainActor static func current() -> SystemSizingMetrics {
    let cache = SystemSizingCache.shared
    cache.observe()
    #if os(macOS)
      let body = NSFont.preferredFont(forTextStyle: .body, options: [:]).pointSize
      if let cached = cache.value, cached.bodyFontSize == body { return cached }
    #else
      if let cached = cache.value { return cached }
    #endif
    let fresh = read()
    cache.value = fresh
    return fresh
  }
}

private final class SystemSizingCache: @unchecked Sendable {
  static let shared = SystemSizingCache()

  private let lock = NSLock()
  private var stored: SystemSizingMetrics?
  /// Touched only on the main actor.
  private var observing = false

  var value: SystemSizingMetrics? {
    get {
      lock.lock()
      defer { lock.unlock() }
      return stored
    }
    set {
      lock.lock()
      stored = newValue
      lock.unlock()
    }
  }

  /// Clears the reading the moment the platform says it may be stale. The
  /// observers are synchronous, so they run before any modifier's own
  /// subscription — delivered on the next run-loop pass — asks again.
  @MainActor func observe() {
    guard !observing else { return }
    observing = true
    #if os(macOS)
      let clear: @Sendable (Notification) -> Void = { _ in SystemSizingCache.shared.value = nil }
      let center = NotificationCenter.default
      _ = center.addObserver(
        forName: NSFont.fontSetChangedNotification, object: nil, queue: nil, using: clear)
      _ = center.addObserver(
        forName: NSApplication.didBecomeActiveNotification, object: nil, queue: nil, using: clear)
      _ = NSWorkspace.shared.notificationCenter.addObserver(
        forName: NSWorkspace.accessibilityDisplayOptionsDidChangeNotification, object: nil,
        queue: nil, using: clear)
    #endif
  }
}
#if os(Windows) || os(Linux) || os(WASI)
  // AppKit system metrics are not available through SwiftUIPort. The desktop port uses
  // design metrics; native display scaling is applied by the backend.
  internal struct SystemThemeModifier: ViewModifier {
    let name: ThemeName
    let sizing: ThemeSizing
    func body(content: Content) -> some View {
      content.environment(\.theme, Theme(name))
        .font(name.variables.bodyMedium.font)
        .environment(\.colorScheme, name.colorScheme)
    }
  }
#else
  @MainActor
  internal struct SystemThemeModifier: ViewModifier {
    let name: ThemeName
    let sizing: ThemeSizing
    private let readMetrics: @MainActor () -> SystemSizingMetrics
    @State private var metrics: SystemSizingMetrics
    @Environment(\.scenePhase) private var scenePhase

    /// One timer for every modifier in the process, rather than a publisher
    /// made afresh each time SwiftUI rebuilds this struct.
    private static let ticks = Timer.publish(every: 1, on: .main, in: .common)
      .autoconnect()
      .share()

    init(
      name: ThemeName, sizing: ThemeSizing,
      readMetrics: @escaping @MainActor () -> SystemSizingMetrics = {
        SystemSizingMetrics.current()
      }
    ) {
      self.name = name
      self.sizing = sizing
      self.readMetrics = readMetrics
      // SwiftUI evaluates this on every rebuild and keeps only the first;
      // `current()` makes the rest a cache hit.
      _metrics = State(initialValue: readMetrics())
    }

    func body(content: Content) -> some View {
      let resolved = Theme(name, sizing: sizing, metrics: metrics)
      content
        .environment(\.theme, resolved)
        .font(resolved.vars.bodyMedium.font)
        .environment(\.colorScheme, name.colorScheme)
        .onAppear(perform: refresh)
        .onChange(of: sizing) { _ in refresh() }
        .onReceive(Self.ticks) { _ in
          if scenePhase == .active { refresh() }
        }
        #if os(macOS)
          .onReceive(
            NotificationCenter.default.publisher(for: NSApplication.didBecomeActiveNotification)
              .receive(on: RunLoop.main)
          ) { _ in refresh() }
          .onReceive(
            NotificationCenter.default.publisher(for: NSFont.fontSetChangedNotification).receive(
              on: RunLoop.main)
          ) { _ in refresh() }
          .onReceive(
            NSWorkspace.shared.notificationCenter.publisher(
              for: NSWorkspace.accessibilityDisplayOptionsDidChangeNotification
            ).receive(on: RunLoop.main)
          ) { _ in refresh() }
        #endif
    }

    private func refresh() {
      guard sizing == .system else { return }
      let updated = readMetrics()
      if updated != metrics { metrics = updated }
    }
  }

#endif
