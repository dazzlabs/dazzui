import AppKit
import FluentIcons
import SwiftUI
import XCTest

@testable import DazzUI

@MainActor
final class DesktopPrimitivesTests: XCTestCase {
  // MARK: - P0: TypeStyle metrics cache

  func testTypeStyleNaturalLineHeightIsCachedByFamilyAndSize() {
    TypeMetricsCache.shared.resetForTesting()

    let body13 = ThemeVariables.studioLight.bodyMedium
    let first = body13.naturalLineHeight
    let second = body13.naturalLineHeight
    let spacing = body13.lineSpacing

    XCTAssertGreaterThan(first, 0)
    XCTAssertEqual(first, second)
    XCTAssertEqual(spacing, body13.lineHeight - first)
    XCTAssertEqual(TypeMetricsCache.shared.totalComputations, 1)
    XCTAssertEqual(TypeMetricsCache.shared.cachedEntryCount, 1)

    // A distinct size populates a second cache entry.
    let title17 = ThemeVariables.studioLight.titleLarge
    let titleHeight = title17.naturalLineHeight
    XCTAssertGreaterThan(titleHeight, 0)
    XCTAssertEqual(TypeMetricsCache.shared.totalComputations, 2)

    // A monospaced code family populates its own entry cleanly.
    let code11 = ThemeVariables.studioLight.codeStyle(size: 11)
    XCTAssertGreaterThan(code11.naturalLineHeight, 0)
    XCTAssertEqual(TypeMetricsCache.shared.totalComputations, 3)

    // Concurrent reads stay thread-safe and hit the cache.
    DispatchQueue.concurrentPerform(iterations: 32) { _ in
      _ = body13.naturalLineHeight
      _ = title17.lineSpacing
      _ = code11.halfLeading
    }
    XCTAssertEqual(TypeMetricsCache.shared.totalComputations, 3)
  }

  // MARK: - P1: AppearanceMode, ThemeFamily & ThemeAppearanceController

  func testAppearanceModeAndThemeFamilyResolution() {
    XCTAssertEqual(AppearanceMode.light.colorScheme, .light)
    XCTAssertEqual(AppearanceMode.dark.colorScheme, .dark)
    XCTAssertNil(AppearanceMode.system.colorScheme)

    XCTAssertEqual(AppearanceMode.system.effectiveColorScheme(system: .dark), .dark)
    XCTAssertEqual(AppearanceMode.system.effectiveColorScheme(system: .light), .light)
    XCTAssertEqual(AppearanceMode.light.effectiveColorScheme(system: .dark), .light)
    XCTAssertEqual(AppearanceMode.dark.effectiveColorScheme(system: .light), .dark)

    for family in ThemeFamily.allCases {
      let light = ThemeName.resolve(family: family, dark: false)
      let dark = ThemeName.resolve(family: family, dark: true)
      XCTAssertFalse(light.isDark, "\(family.rawValue) light must not be dark")
      XCTAssertTrue(dark.isDark, "\(family.rawValue) dark must be dark")
      XCTAssertEqual(light.family, family)
      XCTAssertEqual(dark.family, family)
      XCTAssertEqual(ThemeName.resolve(family: family.rawValue, dark: false), light)
      XCTAssertEqual(ThemeName.resolve(family: family.rawValue, dark: true), dark)
    }

    // Unknown family string falls back to Bright (`:root` default) in the requested light/dark half.
    XCTAssertEqual(ThemeName.resolve(family: "nonexistent", dark: false), .brightLight)
    XCTAssertEqual(ThemeName.resolve(family: "nonexistent", dark: true), .brightDark)
  }

  func testThemeAppearanceControllerAndStatusItemFiltering() {
    let controller = ThemeAppearanceController()
    controller.apply(.light)
    XCTAssertEqual(controller.mode, .light)
    XCTAssertEqual(controller.effectiveColorScheme, .light)
    XCTAssertEqual(controller.nsAppearance?.name, .aqua)

    let popover = NSPopover()
    controller.applyAppearance(to: popover)
    XCTAssertEqual(popover.appearance?.name, .aqua)

    controller.apply(.dark)
    XCTAssertEqual(controller.mode, .dark)
    XCTAssertEqual(controller.effectiveColorScheme, .dark)
    XCTAssertEqual(controller.nsAppearance?.name, .darkAqua)
    controller.applyAppearance(to: popover)
    XCTAssertEqual(popover.appearance?.name, .darkAqua)

    controller.apply(.system)
    XCTAssertEqual(controller.mode, .system)
    XCTAssertNil(controller.nsAppearance)
    controller.applyAppearance(to: popover)
    XCTAssertNil(popover.appearance)

    // Regular NSWindow is not treated as a status-item window.
    let regular = NSWindow(
      contentRect: NSRect(x: 0, y: 0, width: 200, height: 100),
      styleMask: .borderless,
      backing: .buffered,
      defer: false
    )
    XCTAssertFalse(regular.dazzHostsStatusItem)
  }

  // MARK: - P1: Color math & derived semantic tokens

  func testColorBlendingAndInterpolation() {
    let white = Color(argb: 0xFFFF_FFFF)
    let black = Color(argb: 0xFF00_0000)

    let halfLerp = black.lerp(to: white, t: 0.5).srgbComponents
    XCTAssertEqual(halfLerp.r, 0.5, accuracy: 0.01)
    XCTAssertEqual(halfLerp.g, 0.5, accuracy: 0.01)
    XCTAssertEqual(halfLerp.b, 0.5, accuracy: 0.01)
    XCTAssertEqual(halfLerp.a, 1.0, accuracy: 0.01)

    let blended = black.blended(over: white, alpha: 0.25).srgbComponents
    XCTAssertEqual(blended.r, 0.75, accuracy: 0.01)
    XCTAssertEqual(blended.g, 0.75, accuracy: 0.01)
    XCTAssertEqual(blended.b, 0.75, accuracy: 0.01)
    XCTAssertEqual(blended.a, 1.0, accuracy: 0.01)

    // Derived semantic colors resolve cleanly across all drawn families.
    for family in ThemeFamily.drawn {
      for dark in [false, true] {
        let vars = family.themeName(dark: dark).variables
        XCTAssertEqual(vars.accentSurface.srgbComponents.a, 1.0, accuracy: 0.01)
        XCTAssertEqual(vars.dangerSurface.srgbComponents.a, 1.0, accuracy: 0.01)
        XCTAssertEqual(vars.warnSurface.srgbComponents.a, 1.0, accuracy: 0.01)
        XCTAssertEqual(vars.successSurface.srgbComponents.a, 1.0, accuracy: 0.01)
        XCTAssertEqual(vars.railSurface.srgbComponents.a, 1.0, accuracy: 0.01)
        XCTAssertGreaterThan(vars.accentHairline.srgbComponents.a, 0)
        XCTAssertGreaterThan(vars.dangerHairline.srgbComponents.a, 0)
        XCTAssertGreaterThan(vars.warnHairline.srgbComponents.a, 0)
        XCTAssertGreaterThan(vars.selection.srgbComponents.a, 0)
        XCTAssertGreaterThan(vars.selectionUnemphasized.srgbComponents.a, 0)
        XCTAssertFalse(vars.fontCode.isEmpty)
        XCTAssertEqual(vars.codeStyle().family, vars.fontCode)
      }
    }
  }

  // MARK: - P1: Desktop Shell Chrome

  func testFullBleedHostingViewZeroesSafeAreaInsets() {
    let host = FullBleedHostingView(rootView: Text("Hello"))
    XCTAssertEqual(host.safeAreaInsets.top, 0)
    XCTAssertEqual(host.safeAreaInsets.left, 0)
    XCTAssertEqual(host.safeAreaInsets.bottom, 0)
    XCTAssertEqual(host.safeAreaInsets.right, 0)
  }

  func testLinuxDesktopStyleDetection() {
    XCTAssertEqual(
      LinuxDesktopStyle.detect(environment: ["DAZZUI_LINUX_DESKTOP": "kde"]),
      .kde
    )
    XCTAssertEqual(
      LinuxDesktopStyle.detect(environment: ["OMARCHY_PATH": "/home/user/.local/share/omarchy"]),
      .omarchy
    )
    XCTAssertEqual(
      LinuxDesktopStyle.detect(environment: ["XDG_CURRENT_DESKTOP": "KDE"]),
      .kde
    )
    XCTAssertEqual(
      LinuxDesktopStyle.detect(environment: ["XDG_CURRENT_DESKTOP": "ubuntu:GNOME"]),
      .ubuntu
    )
    XCTAssertEqual(
      LinuxDesktopStyle.detect(environment: ["XDG_CURRENT_DESKTOP": "GNOME"]),
      .gnome
    )
  }

  func testShellChromeViewsLayoutCleanly() {
    let root = VStack(spacing: 0) {
      WindowTitlebar(
        title: "Workspace",
        subtitle: "Draft",
        leadingClearance: WindowChromeMetrics.trafficLightClearance
      ) {
        Spacer(minLength: 0)
        IconButton(label: "Settings", icon: .settings20Regular, tooltipSide: .bottom) {}
      }
      WindowToolbar(title: "Details") {
        Spacer(minLength: 0)
      } trailing: {
        WindowsCaptionButtons(
          isMaximized: false,
          onMinimize: {},
          onToggleMaximize: {},
          onClose: {}
        )
      }
      HStack(spacing: 12) {
        ForEach(LinuxDesktopStyle.allCases, id: \.self) { style in
          LinuxWindowControlButtons(
            style: style,
            onMinimize: {},
            onToggleMaximize: {},
            onClose: {}
          )
        }
      }
      Spacer(minLength: 0)
      WindowFooter {
        Text("Ready").typeStyle(ThemeVariables.studioLight.captionSmall)
        Spacer(minLength: 0)
        Text("42").numericTypeStyle(ThemeVariables.studioLight.labelMedium)
      }
    }
    .theme(family: .studio, mode: .light)
    .frame(width: 640, height: 320)

    let host = NSHostingView(rootView: root)
    host.frame = NSRect(x: 0, y: 0, width: 640, height: 320)
    host.layoutSubtreeIfNeeded()
    XCTAssertEqual(host.frame.width, 640)
  }

  // MARK: - P2: Sidebar Icon Collapse

  func testSidebarIconCollapsibleModeKeepsIconRailWidth() throws {
    let width: CGFloat = 300
    let height: CGFloat = 120
    let root = HStack(spacing: 0) {
      Sidebar(
        collapsed: true,
        collapsible: .icon,
        header: { Icon(.panelLeft20Regular) },
        content: {
          SidebarGroup("Main") {
            SidebarMenuButton(
              active: true,
              tooltip: "Home",
              action: {},
              icon: { Icon(.home20Regular) },
              label: { Text("Home") }
            )
          }
        },
        footer: {
          IconButton(label: "Help", icon: .questionCircle20Regular) {}
        }
      )
      Rectangle()
        .fill(Color.red)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
    .theme(.studioLight)
    .frame(width: width, height: height)

    let host = NSHostingView(rootView: root)
    host.frame = NSRect(x: 0, y: 0, width: width, height: height)
    let window = NSWindow(
      contentRect: host.frame,
      styleMask: .borderless,
      backing: .buffered,
      defer: false
    )
    window.contentView = host
    host.layoutSubtreeIfNeeded()

    let bitmap = try XCTUnwrap(
      NSBitmapImageRep(
        bitmapDataPlanes: nil,
        pixelsWide: Int(width * 2),
        pixelsHigh: Int(height * 2),
        bitsPerSample: 8,
        samplesPerPixel: 4,
        hasAlpha: true,
        isPlanar: false,
        colorSpaceName: .calibratedRGB,
        bytesPerRow: 0,
        bitsPerPixel: 0
      )
    )
    bitmap.size = host.bounds.size
    host.cacheDisplay(in: host.bounds, to: bitmap)

    // Inside the 48pt icon rail (x = 12pt -> 24px), background is sidebar surface, not red.
    let railColor = try XCTUnwrap(
      bitmap.colorAt(x: 24, y: bitmap.pixelsHigh / 2)?.usingColorSpace(.deviceRGB)
    )
    XCTAssertFalse(
      railColor.redComponent > 0.9 && railColor.greenComponent < 0.35 && railColor.blueComponent < 0.35,
      "Icon-collapsed sidebar should occupy 48pt rail width"
    )

    // Past 48pt (x = 70pt -> 140px), the red main pane starts.
    let paneColor = try XCTUnwrap(
      bitmap.colorAt(x: 140, y: bitmap.pixelsHigh / 2)?.usingColorSpace(.deviceRGB)
    )
    XCTAssertTrue(
      paneColor.redComponent > 0.9 && paneColor.greenComponent < 0.35 && paneColor.blueComponent < 0.35,
      "Main pane should start immediately after the 48pt icon rail"
    )
  }

  // MARK: - P2: Menu Separator, Submenu Checkmarks & Center Align

  func testMenuSeparatorAndMixedCheckmarksInNativeMenuSession() throws {
    let items: [MenuItem] = [
      MenuItem("English", checked: true),
      MenuItem("Japanese", checked: false),
      .separator(),
      MenuItem(
        "More languages",
        items: [
          MenuItem("French", checked: false),
          .separator(),
          MenuItem("German", checked: false),
        ]
      ),
    ]
    XCTAssertTrue(items[2].isSeparator)
    XCTAssertNotEqual(MenuItem.separator().id, MenuItem.separator().id)

    let session = NativeMenuSession(entries: items.map(NativeMenuEntry.item), theme: .default)
    let menu = try XCTUnwrap(session.menu)
    let nativeItems = menu.allItems
    XCTAssertEqual(nativeItems.count, 4)
    XCTAssertEqual(nativeItems[0].state, .checked)
    XCTAssertEqual(nativeItems[1].state, .unchecked)
    XCTAssertEqual(nativeItems[2].type, .separator)
    XCTAssertEqual(nativeItems[3].type, .submenu)
    XCTAssertEqual(nativeItems[3].submenu?.allItems.count, 3)
    XCTAssertEqual(nativeItems[3].submenu?.allItems[1].type, .separator)
    session.cancel()

    XCTAssertTrue(MenuAlign.allCases.contains(.center))
  }

  // MARK: - P2: Toaster Hover-to-Pause & Leading Options

  func testToasterPauseAndResumeDelaysDismissal() async {
    let toaster = Toaster()
    let id = toaster.add(
      ToastOptions(
        title: "Syncing",
        tint: .info,
        timeout: 0.12,
        icon: .arrowSync20Regular,
        loading: false
      )
    )
    XCTAssertEqual(toaster.toasts.count, 1)
    XCTAssertEqual(toaster.toasts.first?.options.icon, .arrowSync20Regular)

    // Pause immediately; toast must stay alive past the original 120ms timeout.
    toaster.pause(id)
    XCTAssertTrue(toaster.isPaused(id))
    try? await Task.sleep(for: .milliseconds(180))
    XCTAssertEqual(toaster.toasts.count, 1, "Paused toast must not auto-dismiss while hovered")

    // Resuming lets it finish and auto-dismiss.
    toaster.resume(id)
    XCTAssertFalse(toaster.isPaused(id))
    try? await Task.sleep(for: .milliseconds(250))
    XCTAssertTrue(toaster.toasts.isEmpty, "Resumed toast should auto-dismiss after remaining time")

    // Loading toast defaults to persistent (timeout == 0) until closed manually.
    let loadingId = toaster.add(ToastOptions(title: "Uploading", loading: true))
    XCTAssertEqual(toaster.toasts.first?.options.resolvedTimeout, 0)
    toaster.close(loadingId)
    XCTAssertTrue(toaster.toasts.isEmpty)
  }

  // MARK: - P2: Component Initializer Compilation & Rendering Smoke

  func testNewComponentInitializersRenderCleanly() {
    var text = "Hello"
    let textBinding = Binding(get: { text }, set: { text = $0 })
    var railWidth: CGFloat = 160
    let railBinding = Binding(get: { railWidth }, set: { railWidth = $0 })

    let root = HStack(spacing: 0) {
      Rail(resizable: true, width: railBinding, surface: .rail) {
        RailGroup("Section") {
          SidebarMenuButton(
            active: true,
            showActionOnHover: true,
            action: {},
            icon: { Icon(.folder20Regular) },
            label: { Text("Projects") },
            badge: { SidebarMenuBadge("3") },
            rowAction: {
              SidebarMenuAction(showOnHover: true) {
                IconButton(label: "More", icon: .moreHorizontal16Regular) {}
              }
            }
          )
        }
      }
      SettingsPage {
        PreferenceBackButton("All Settings") {}
        PreferenceSection(label: "Account", variant: .card) {
          PreferenceRow(
            title: "Documentation",
            subtitle: "Opens in browser",
            href: URL(string: "https://example.com"),
            align: .top,
            trailing: {
              PreferenceThemeSwatches(
                options: ThemeFamily.drawn.map { PreferenceThemeOption(family: $0, dark: false) },
                value: .studio
              )
            }
          )
          PreferenceDivider()
          PreferenceRow(
            title: "Controls",
            trailing: {
              HStack(spacing: 8) {
                Button("Save", variant: .filled, loading: true) {}
                Button("Add", variant: .recessed, action: {}) { Icon(.add16Regular) }
                Toggle(
                  .pin16Regular,
                  label: "Pin",
                  pressed: true,
                  tint: .primary,
                  shape: .circle,
                  tooltip: "Pin window",
                  tooltipSide: .bottom
                ) {}
                Checkbox("Work", checked: true, accentColor: Color(argb: 0xFF3A_7BFD))
              }
            }
          )
        }
        Card(padding: .none, interactive: true) {
          Text("Card Header").typeStyle(ThemeVariables.studioLight.labelMedium)
        } content: {
          TextField(
            text: textBinding,
            placeholder: "Search",
            borderless: false,
            onSubmit: {},
            leading: { Icon(.search16Regular) },
            trailing: { Badge("⌘K", size: .small) }
          )
          TextField(
            text: textBinding,
            placeholder: "Notes",
            multiline: true,
            borderless: true,
            submitOnReturn: true,
            onSubmit: {}
          )
        }
        EmptyState(
          icon: .search20Regular,
          title: "No results",
          description: "Try a different query"
        ) {
          Button("Reset") {}
        }
        Dialog(width: 380, height: 220, onDismiss: {}) {
          DialogHeader(
            title: "Device",
            subtitle: "192.168.1.42",
            monoSubtitle: true,
            leading: { Icon(.desktop20Regular) },
            accessory: { IconButton(label: "Close", icon: .dismiss16Regular) {} }
          )
          DialogBody {
            Code("dazzui --version")
          }
          DialogFooter {
            Button("Close") {}
          }
        }
      }
      Aside(width: 200) {
        Text("Inspector")
      } content: {
        Text("Details")
      }
    }
    .theme(.studioLight)
    .windowDialog(isPresented: .constant(false)) {
      Dialog { DialogBody { Text("Modal") } }
    }
    .frame(width: 1100, height: 700)

    let host = NSHostingView(rootView: root)
    host.frame = NSRect(x: 0, y: 0, width: 1100, height: 700)
    host.layoutSubtreeIfNeeded()
    XCTAssertEqual(host.frame.width, 1100)
  }
}
