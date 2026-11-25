import Foundation
import SwiftUI

#if os(macOS)
  import AppKit
#elseif (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
  import NativeAPI
  #if os(Windows) && canImport(WinSDK)
    import WinSDK
  #elseif os(Linux) && canImport(CGTK3)
    import CGTK3
  #endif
#endif

// MARK: - AppearanceMode

/// Orthogonal light/dark preference paired with a theme family.
public enum AppearanceMode: String, CaseIterable, Sendable, Hashable, Codable {
  case light
  case dark
  case system

  /// Explicit SwiftUI color scheme, or `nil` when following the OS.
  public var colorScheme: ColorScheme? {
    switch self {
    case .light: return .light
    case .dark: return .dark
    case .system: return nil
    }
  }

  /// Resolves the active color scheme given the current system scheme.
  public func effectiveColorScheme(system: ColorScheme) -> ColorScheme {
    colorScheme ?? system
  }
}

// MARK: - ThemeName helpers

public extension ThemeName {
  /// Whether this theme is a dark palette.
  var isDark: Bool {
    colorScheme == .dark
  }

  /// Resolves a typed ``ThemeFamily`` and light/dark flag into a ``ThemeName``.
  static func resolve(family: ThemeFamily, dark: Bool) -> ThemeName {
    family.themeName(dark: dark)
  }

  /// Resolves a typed ``ThemeFamily`` against a SwiftUI `ColorScheme`.
  static func resolve(family: ThemeFamily, scheme: ColorScheme) -> ThemeName {
    family.themeName(for: scheme)
  }

  /// Resolves a theme family identifier (e.g. `"bright"`, `"studio"`, `"windows11"`,
  /// `"omarchy-tokyo-night"`) and light/dark flag into a `ThemeName`.
  static func resolve(
    family: String,
    dark: Bool,
    fallback: ThemeName = .brightLight
  ) -> ThemeName {
    if let typed = ThemeFamily(rawValue: family) {
      return typed.themeName(dark: dark)
    }
    let suffix = dark ? "dark" : "light"
    if let paired = ThemeName(rawValue: "\(family)-\(suffix)") {
      return paired
    }
    if let exact = ThemeName(rawValue: family) {
      return exact
    }
    return fallback.family.themeName(dark: dark)
  }

  /// Resolves a theme family identifier against a SwiftUI `ColorScheme`.
  static func resolve(
    family: String,
    scheme: ColorScheme,
    fallback: ThemeName = .brightLight
  ) -> ThemeName {
    resolve(family: family, dark: scheme == .dark, fallback: fallback)
  }
}

// MARK: - ThemeAppearanceController

/// Synchronizes application/window native appearance with an `AppearanceMode`
/// and tracks live OS light/dark changes across macOS, Windows, and Linux.
///
/// On macOS, window appearance synchronization explicitly filters out
/// `NSStatusBarWindow` (windows hosting `NSStatusBarButton`) so forcing Light
/// mode never paints dark menu-bar status items unreadable.
@MainActor
public final class ThemeAppearanceController: ObservableObject {
  public static let shared = ThemeAppearanceController()

  @Published public private(set) var mode: AppearanceMode = .system
  @Published public private(set) var systemColorScheme: ColorScheme

  /// The effective color scheme after applying `mode` over `systemColorScheme`.
  public var effectiveColorScheme: ColorScheme {
    mode.effectiveColorScheme(system: systemColorScheme)
  }

  /// Whether the effective appearance is dark.
  public var isDark: Bool {
    effectiveColorScheme == .dark
  }

  #if os(macOS)
    private var distributedObserver: NSObjectProtocol?
    private var activeObserver: NSObjectProtocol?
  #elseif (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
    private var appListenerId: ListenerId?
  #endif

  public init() {
    self.systemColorScheme = Self.detectSystemColorScheme()
    startObserving()
  }

  /// Updates the active appearance mode and synchronizes native windows.
  public func apply(_ mode: AppearanceMode) {
    self.mode = mode
    refreshSystemAppearance()

    #if os(macOS)
      guard let app = NSApp else { return }
      let appearance = Self.nsAppearance(for: mode)
      app.appearance = appearance
      for window in Self.applicationWindows(in: app) {
        window.appearance = appearance
      }
    #elseif (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
      let brightness: Brightness
      switch mode {
      case .light: brightness = .light
      case .dark: brightness = .dark
      case .system: brightness = .system
      }
      _ = Application.shared.setBrightness(brightness)
    #endif
  }

  /// Re-reads the operating system's current light/dark setting.
  public func refreshSystemAppearance() {
    let detected = Self.detectSystemColorScheme()
    if systemColorScheme != detected {
      systemColorScheme = detected
    }
  }

  #if os(macOS)
    /// The `NSAppearance` corresponding to the active `mode`, or `nil` for `.system`.
    public var nsAppearance: NSAppearance? {
      Self.nsAppearance(for: mode)
    }

    /// Maps an `AppearanceMode` to its `NSAppearance` override (`nil` follows system).
    public static func nsAppearance(for mode: AppearanceMode) -> NSAppearance? {
      switch mode {
      case .light: return NSAppearance(named: .aqua)
      case .dark: return NSAppearance(named: .darkAqua)
      case .system: return nil
      }
    }

    /// Synchronizes an individual `NSWindow` with the active appearance mode,
    /// leaving menu bar status-item windows untouched.
    public func applyAppearance(to window: NSWindow) {
      guard !window.dazzHostsStatusItem else { return }
      window.appearance = nsAppearance
    }

    /// Synchronizes an `NSPopover` with the active appearance mode.
    public func applyAppearance(to popover: NSPopover) {
      popover.appearance = nsAppearance
    }

    /// Application windows excluding `NSStatusBarWindow` status-item hosts.
    public static func applicationWindows(in app: NSApplication? = NSApp) -> [NSWindow] {
      guard let app else { return [] }
      return app.windows.filter { !$0.dazzHostsStatusItem }
    }
  #endif

  #if (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
    /// Synchronizes a portable `NativeAPI.Window`'s background and titlebar colors.
    public static func syncWindowChrome(
      _ window: NativeAPI.Window?,
      surface: SwiftUI.Color,
      chrome: SwiftUI.Color,
      content: SwiftUI.Color
    ) {
      guard let target = window ?? Application.shared.getPrimaryWindow() else { return }
      target.setBackgroundColor(nativeColor(surface))
      _ = target.setTitleBarColors(nativeColor(chrome), foreground: nativeColor(content))
    }

    private static func nativeColor(_ color: SwiftUI.Color) -> NativeAPI.Color {
      let c = color.srgbComponents
      func byte(_ channel: CGFloat) -> UInt8 {
        UInt8(min(255, max(0, (channel * 255).rounded())))
      }
      return NativeAPI.Color(r: byte(c.r), g: byte(c.g), b: byte(c.b), a: byte(c.a))
    }
  #endif

  private func startObserving() {
    #if os(macOS)
      distributedObserver = DistributedNotificationCenter.default().addObserver(
        forName: NSNotification.Name("AppleInterfaceThemeChangedNotification"),
        object: nil,
        queue: .main
      ) { [weak self] _ in
        MainActor.assumeIsolated {
          self?.refreshSystemAppearance()
        }
      }
      activeObserver = NotificationCenter.default.addObserver(
        forName: NSApplication.didBecomeActiveNotification,
        object: nil,
        queue: .main
      ) { [weak self] _ in
        MainActor.assumeIsolated {
          self?.refreshSystemAppearance()
        }
      }
    #elseif (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
      appListenerId = Application.shared.addListener { [weak self] event in
        guard case .activated = event else { return }
        MainActor.assumeIsolated {
          self?.refreshSystemAppearance()
        }
      }
    #endif
  }

  /// Reads the current operating system color scheme.
  public static func detectSystemColorScheme() -> ColorScheme {
    #if os(macOS)
      // Global AppleInterfaceStyle reflects the true OS appearance even when
      // NSApp.appearance is temporarily overridden to aqua/darkAqua.
      if let globalDomain = UserDefaults.standard.persistentDomain(forName: UserDefaults.globalDomain),
        let style = globalDomain["AppleInterfaceStyle"] as? String
      {
        return style.caseInsensitiveCompare("Dark") == .orderedSame ? .dark : .light
      }
      if let app = NSApp, app.appearance == nil {
        return app.effectiveAppearance.bestMatch(from: [.darkAqua, .aqua]) == .darkAqua
          ? .dark : .light
      }
      return .light
    #elseif os(Windows) && !CROSS_SWIFTUI_HARMONY
      #if canImport(WinSDK)
        var light: DWORD = 1
        var size = DWORD(MemoryLayout<DWORD>.size)
        let hkcu = HKEY(bitPattern: UInt(bitPattern: Int(Int32(bitPattern: 0x8000_0001))))
        let status = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"
          .withCString(encodedAs: UTF16.self) { subkey in
            "AppsUseLightTheme".withCString(encodedAs: UTF16.self) { name in
              RegGetValueW(hkcu, subkey, name, DWORD(RRF_RT_REG_DWORD), nil, &light, &size)
            }
          }
        return (status == ERROR_SUCCESS && light == 0) ? .dark : .light
      #else
        return .light
      #endif
    #elseif os(Linux) && !CROSS_SWIFTUI_HARMONY
      #if canImport(CGTK3)
        if let settings = gtk_settings_get_default() {
          let object = UnsafeMutableRawPointer(settings).assumingMemoryBound(to: GObject.self)
          var preferDark = GValue()
          g_value_init(&preferDark, g_type_from_name("gboolean"))
          g_object_get_property(object, "gtk-application-prefer-dark-theme", &preferDark)
          let isDark = g_value_get_boolean(&preferDark) != 0
          g_value_unset(&preferDark)
          if isDark { return .dark }

          var themeValue = GValue()
          g_value_init(&themeValue, g_type_from_name("gchararray"))
          g_object_get_property(object, "gtk-theme-name", &themeValue)
          let themeName = g_value_get_string(&themeValue).map { String(cString: $0) }
          g_value_unset(&themeValue)
          if let themeName, themeName.lowercased().contains("dark") {
            return .dark
          }
        }
      #endif
      if let gtkTheme = ProcessInfo.processInfo.environment["GTK_THEME"],
        gtkTheme.lowercased().contains("dark")
      {
        return .dark
      }
      return .light
    #else
      return .light
    #endif
  }

  deinit {
    #if os(macOS)
      if let distributedObserver {
        DistributedNotificationCenter.default().removeObserver(distributedObserver)
      }
      if let activeObserver {
        NotificationCenter.default.removeObserver(activeObserver)
      }
    #elseif (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
      if let appListenerId {
        _ = Application.shared.removeListener(appListenerId)
      }
    #endif
  }
}

#if os(macOS)
  public extension NSWindow {
    /// Whether this window is an AppKit `NSStatusBarWindow` hosting an
    /// `NSStatusItem` button. Checked via the public `NSStatusBarButton` type
    /// rather than private class names.
    var dazzHostsStatusItem: Bool {
      guard let contentView else { return false }
      return contentView.dazzContainsStatusBarButton
    }
  }

  private extension NSView {
    var dazzContainsStatusBarButton: Bool {
      self is NSStatusBarButton || subviews.contains { $0.dazzContainsStatusBarButton }
    }
  }
#endif

// MARK: - Reactive Omarchy helper & View modifiers

/// Observable holder for the live Omarchy palette on Linux, falling back to
/// `nil` off Omarchy.
@MainActor
public final class OmarchyPaletteObserver: ObservableObject {
  public static let shared = OmarchyPaletteObserver()

  @Published public private(set) var palette: OmarchyPalette?

  public var isAvailable: Bool { palette != nil }

  private var watchTask: Task<Void, Never>?

  public init(directory: String? = nil, config: String? = nil, watch: Bool = true) {
    let initial = readOmarchyPalette(directory: directory, config: config)
    self.palette = initial
    guard watch, initial != nil else { return }
    watchTask = Task { [weak self] in
      for await next in watchOmarchyPalette(directory: directory, config: config) {
        if Task.isCancelled { break }
        if let next {
          self?.palette = next
        }
      }
    }
  }

  /// Resolves the live Omarchy palette against an optional forced color scheme.
  public func resolvedTheme(for scheme: ColorScheme? = nil) -> Theme? {
    guard let palette else { return nil }
    if let scheme, scheme != palette.colorScheme {
      return palette.opposite.theme
    }
    return palette.theme
  }

  deinit {
    watchTask?.cancel()
  }
}

public extension SwiftUI.View {
  /// Scopes a theme resolved from a typed ``ThemeFamily`` and ``AppearanceMode``.
  ///
  /// Observes `ThemeAppearanceController` so system appearance switches update
  /// the scoped theme automatically. When `followOmarchy` is true and an
  /// Omarchy palette is present on Linux, the live Omarchy palette takes
  /// precedence.
  @MainActor
  func theme(
    family: ThemeFamily,
    mode: AppearanceMode = .system,
    sizing: ThemeSizing = .design,
    followOmarchy: Bool = false,
    controller: ThemeAppearanceController = .shared
  ) -> some SwiftUI.View {
    theme(
      family: family.rawValue,
      mode: mode,
      sizing: sizing,
      followOmarchy: followOmarchy,
      controller: controller
    )
  }

  /// Scopes a theme resolved from a family identifier and ``AppearanceMode``.
  @MainActor
  func theme(
    family: String,
    mode: AppearanceMode = .system,
    sizing: ThemeSizing = .design,
    followOmarchy: Bool = false,
    controller: ThemeAppearanceController = .shared
  ) -> some SwiftUI.View {
    modifier(
      FamilyAppearanceThemeModifier(
        family: family,
        mode: mode,
        sizing: sizing,
        followOmarchy: followOmarchy,
        controller: controller
      )
    )
  }

  /// Paints a `ThemeVariables` color behind this view, resolved from the
  /// nearest scoped `@Environment(\.theme)`.
  func themeBackground(_ color: KeyPath<ThemeVariables, SwiftUI.Color>) -> some SwiftUI.View {
    modifier(ThemeBackgroundModifier(color: color))
  }
}

@MainActor
private struct FamilyAppearanceThemeModifier: ViewModifier {
  let family: String
  let mode: AppearanceMode
  let sizing: ThemeSizing
  let followOmarchy: Bool
  @ObservedObject var controller: ThemeAppearanceController
  @Environment(\.colorScheme) private var envScheme

  @ViewBuilder
  func body(content: Content) -> some SwiftUI.View {
    let systemScheme: ColorScheme = {
      #if os(macOS) || os(Windows) || os(Linux)
        return controller.systemColorScheme
      #else
        return envScheme
      #endif
    }()
    let effective = mode.effectiveColorScheme(system: systemScheme)
    let resolvedName = ThemeName.resolve(family: family, scheme: effective)
    if followOmarchy {
      content.omarchyTheme(fallback: resolvedName, colorScheme: mode.colorScheme)
    } else if sizing == .system {
      content.theme(resolvedName, sizing: .system)
    } else {
      content.theme(resolvedName)
    }
  }
}

private struct ThemeBackgroundModifier: ViewModifier {
  let color: KeyPath<ThemeVariables, SwiftUI.Color>
  @Environment(\.theme) private var theme

  func body(content: Content) -> some SwiftUI.View {
    content.background(theme.vars[keyPath: color])
  }
}
