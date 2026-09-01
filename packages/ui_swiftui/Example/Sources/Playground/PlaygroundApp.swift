import DazzUI
import FluentIcons
import SwiftUI

@main
struct PlaygroundApp: App {
  @NSApplicationDelegateAdaptor(AppDelegate.self) private var delegate

  var body: some Scene {
    WindowGroup("DazzUI Playground") {
      PlaygroundWindow()
    }
    .windowToolbarStyle(.unified(showsTitle: true))
    .defaultSize(width: 1080, height: 720)
  }
}

/// Only matters when the playground is built as a bare SwiftPM executable
/// rather than through `Playground.xcodeproj`: an unbundled binary launches
/// with no dock presence and behind whatever window has focus. In the app
/// bundle this is already the policy, so both calls are no-ops.
final class AppDelegate: NSObject, NSApplicationDelegate {
  func applicationDidFinishLaunching(_ notification: Notification) {
    NSApp.setActivationPolicy(.regular)
    NSApp.activate(ignoringOtherApps: true)
  }

  func applicationShouldTerminateAfterLastWindowClosed(_ app: NSApplication) -> Bool {
    true
  }
}

/// The playground itself: the component gallery on the left, the selected
/// page on the right, and the theme picker over both.
///
/// The theme is applied with `.theme(_:)` on one subtree rather than to the
/// process, which is the point worth demonstrating — themes are per-subtree
/// here, so the four can sit on screen at once (see ``ThemesPage``).
struct PlaygroundWindow: View {
  @State private var page: Page = .controls
  @State private var theme: ThemeName = .studioLight
  @State private var dialog = false
  @State private var toast = false

  var body: some View {
    VStack(spacing: 0) {
      Toolbar(theme: $theme)
      Divider()

      HStack(spacing: 0) {
        Navigator(page: $page)
        Divider(orientation: .vertical)
        PageHost(page: page, dialog: $dialog, toast: $toast)
      }
    }
    // The overlays sit outside the scrolled column so the scrim covers the
    // window, which is what a portal target buys the React package.
    .overlay(FeedbackOverlays(dialog: $dialog, toast: $toast))
    .theme(theme)
    .background(ThemeBackground())
    .frame(minWidth: 900, minHeight: 600)
  }
}

/// The canvas the whole window sits on, read from the active theme.
private struct ThemeBackground: View {
  @Environment(\.theme) private var theme

  var body: some View {
    theme.vars.colorCanvas.ignoresSafeArea()
  }
}

private struct Toolbar: View {
  @Binding var theme: ThemeName
  @Environment(\.theme) private var active

  var body: some View {
    let vars = active.vars

    HStack(spacing: vars.spacing3) {
      Text("DazzUI")
        .font(vars.titleSmall.font)
        .foregroundStyle(vars.colorContent)

      Badge("SwiftUI", size: .small, variant: .tinted)

      Spacer()

      SegmentedControl(
        items: ThemeName.allCases.map {
          SegmentedItem(value: $0, label: $0.title)
        },
        value: theme,
        onValueChange: { theme = $0 },
        size: .small
      )
    }
    .padding(.horizontal, vars.spacing4)
    .frame(height: vars.frameTitlebarSize)
    .background(vars.colorSurfaceMuted)
  }
}

private struct Navigator: View {
  @Binding var page: Page
  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    Sidebar {
      SidebarGroup {
        Text("Components")
      } content: {
        ForEach(Page.allCases) { item in
          NavItem(
            item.title,
            icon: item.icon,
            active: page == item,
            action: { page = item }
          )
        }
      }
    }
    .frame(width: vars.frameSidebarWidth)
  }
}

private struct PageHost: View {
  let page: Page
  @Binding var dialog: Bool
  @Binding var toast: Bool

  @Environment(\.theme) private var theme

  var body: some View {
    ScrollView {
      VStack(alignment: .leading, spacing: theme.vars.spacing6) {
        switch page {
        case .controls: ControlsPage()
        case .forms: FormsPage()
        case .data: DataPage()
        case .feedback: FeedbackPage(dialog: $dialog, toast: $toast)
        case .settings: SettingsPage()
        case .themes: ThemesPage()
        }
      }
      .frame(maxWidth: .infinity, alignment: .leading)
      .padding(theme.vars.spacing6)
    }
    .frame(maxWidth: .infinity, maxHeight: .infinity)
    .background(theme.vars.colorSurface)
  }
}

enum Page: String, CaseIterable, Identifiable {
  case controls
  case forms
  case data
  case feedback
  case settings
  case themes

  var id: String { rawValue }

  var title: String {
    switch self {
    case .controls: return "Controls"
    case .forms: return "Forms"
    case .data: return "Data"
    case .feedback: return "Feedback"
    case .settings: return "Settings"
    case .themes: return "Themes"
    }
  }

  var icon: FluentIcon {
    switch self {
    case .controls: return .squareMultiple16Regular
    case .forms: return .checkboxChecked16Regular
    case .data: return .table16Regular
    case .feedback: return .alert16Regular
    case .settings: return .settings16Regular
    case .themes: return .paintBrush16Regular
    }
  }
}

extension ThemeName {
  var title: String {
    switch self {
    case .studioLight: return "Studio"
    case .studioDark: return "Studio Dark"
    case .brightLight: return "Bright"
    case .brightDark: return "Bright Dark"
    }
  }
}
