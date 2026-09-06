import DazzUI
import SwiftUI

@main
struct PlaygroundApp: App {
  @NSApplicationDelegateAdaptor(AppDelegate.self) private var delegate

  var body: some Scene {
    WindowGroup("DazzUI Playground") {
      PlaygroundWindow()
    }
    .windowToolbarStyle(.unified(showsTitle: true))
    .defaultSize(width: 1180, height: 780)
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

/// The playground: the story tree on the left, one story on the canvas, and
/// the theme picker over both.
///
/// The theme is applied with `.theme(_:)` on one subtree rather than to the
/// process, which is the point worth demonstrating — themes are per-subtree
/// here, so the four can sit on screen at once (see `Tokens/Themes`).
struct PlaygroundWindow: View {
  @State private var selection = StoryIndex.first
  @State private var theme: ThemeName = .studioLight

  var body: some View {
    VStack(spacing: 0) {
      Toolbar(theme: $theme)
      DazzUI.Divider()

      HStack(spacing: 0) {
        Navigator(groups: StoryIndex.groups, selection: $selection)
        Canvas(selection: selection)
      }
    }
    .theme(theme)
    .background(ThemeBackground())
    .frame(minWidth: 960, minHeight: 640)
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

/// One story, presented the way its layout asks for.
private struct Canvas: View {
  let selection: StoryRef

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    VStack(spacing: 0) {
      Breadcrumb(selection: selection)
      DazzUI.Divider()
      stage
    }
    .frame(maxWidth: .infinity, maxHeight: .infinity)
    .background(vars.colorSurface)
  }

  @ViewBuilder private var stage: some View {
    if let story = StoryIndex.story(selection) {
      switch story.layout {
      case .centered:
        // A bare specimen sits in the middle of the canvas rather than in its
        // corner, which is what `layout: 'centered'` buys a component file.
        GeometryReader { proxy in
          ScrollView {
            story.content()
              .padding(theme.vars.spacing8)
              .frame(
                maxWidth: .infinity,
                minHeight: proxy.size.height,
                alignment: .center
              )
          }
        }
      case .page:
        // The `Page` brings its own padding, as `layout: 'fullscreen'` does
        // on an `Overview`.
        ScrollView {
          story.content()
        }
      case .fullscreen:
        // A scrim or a toast viewport is fixed to the canvas the way it would
        // be to the viewport, so it takes the whole of it, unscrolled.
        story.content()
          .frame(maxWidth: .infinity, maxHeight: .infinity)
      }
    } else {
      EmptyState("No story selected")
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
  }
}

/// Where the canvas is pointed, in words.
private struct Breadcrumb: View {
  let selection: StoryRef

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing2) {
      Text(selection.page)
        .font(vars.labelMedium.font)
        .foregroundStyle(vars.colorContent)

      Text("/")
        .font(vars.captionMedium.font)
        .foregroundStyle(vars.colorContentFaint)

      Text(selection.story)
        .font(vars.captionMedium.font)
        .foregroundStyle(vars.colorContentMuted)

      Spacer()
    }
    .padding(.horizontal, vars.spacing4)
    .frame(height: vars.frameTitlebarSize)
    .background(vars.colorSurfaceMuted)
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
