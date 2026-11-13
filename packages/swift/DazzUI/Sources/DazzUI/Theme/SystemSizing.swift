import SwiftUI

/// Runtime sizing is independent from the selected color family.
#if os(macOS)
  import AppKit
#endif

public enum ThemeSizing: String, CaseIterable, Sendable {
  case design
  case system
}
internal struct SystemSizingMetrics: Sendable, Equatable {
  struct Control: Sendable, Equatable {
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
}
#if os(Windows) || os(Linux)
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
    private let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    init(
      name: ThemeName, sizing: ThemeSizing,
      readMetrics: @escaping @MainActor () -> SystemSizingMetrics = { SystemSizingMetrics.read() }
    ) {
      self.name = name
      self.sizing = sizing
      self.readMetrics = readMetrics
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
        .onReceive(timer) { _ in
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
