import DazzUI
import SwiftUI

/// The page being read and the theme it is read in — the two things the
/// React example keeps in its address — and the window that shows them.
struct AppView: View {
  @State private var page: String?
  @State private var theme: ThemeChoice
  @State private var sizing: ThemeSizing
  @State private var menuPresentation: MenuPresentation = .custom

  /// On an Omarchy machine the window wears the machine's theme, and follows
  /// it, until another is picked.
  @State private var omarchy: OmarchyPalette?
  @State private var followingOmarchy: Bool

  private let omarchyDirectory: String?
  private let omarchyConfig: String?

  init(
    page: String? = nil,
    theme: ThemeChoice? = nil,
    sizing: ThemeSizing = .design,
    omarchyDirectory: String? = nil,
    omarchyConfig: String? = nil
  ) {
    _page = State(initialValue: page)
    _theme = State(initialValue: theme ?? defaultTheme)
    _sizing = State(initialValue: sizing)
    _followingOmarchy = State(initialValue: theme == nil)
    self.omarchyDirectory = omarchyDirectory
    self.omarchyConfig = omarchyConfig
  }

  private var activeOmarchy: Theme? {
    guard let omarchy, followingOmarchy else { return nil }
    let shipsDark = omarchy.colorScheme == .dark
    return (shipsDark == theme.dark ? omarchy : omarchy.opposite).theme
  }

  private func applyOmarchy(_ palette: OmarchyPalette?) {
    omarchy = palette
    if let palette, followingOmarchy {
      theme.dark = palette.colorScheme == .dark
    }
  }

  var body: some View {
    let source = OmarchyLocation(directory: omarchyDirectory, config: omarchyConfig)
    Shell(
      page: $page,
      theme: $theme,
      sizing: $sizing,
      menuPresentation: $menuPresentation,
      omarchy: omarchy,
      followingOmarchy: omarchy != nil && followingOmarchy,
      onFollowOmarchy: { following in
        followingOmarchy = following
        if following, let omarchy {
          theme.dark = omarchy.colorScheme == .dark
        }
      }
    )
    .menuPresentation(menuPresentation)
    .modifier(OmarchyOverrideModifier(override: activeOmarchy))
    .theme(theme.name, sizing: sizing)
    .overlayHost()
    .onAppear {
      applyOmarchy(readOmarchyPalette(directory: omarchyDirectory, config: omarchyConfig))
    }
    .onChange(of: source) { next in
      applyOmarchy(readOmarchyPalette(directory: next.directory, config: next.config))
    }
    .task(id: source) {
      for await palette in watchOmarchyPalette(directory: omarchyDirectory, config: omarchyConfig) {
        if Task.isCancelled { break }
        applyOmarchy(palette)
      }
    }
  }
}

private struct OmarchyLocation: Equatable {
  let directory: String?
  let config: String?
}

private struct OmarchyOverrideModifier: ViewModifier {
  let override: Theme?

  @Environment(\.theme) private var inherited
  @Environment(\.colorScheme) private var inheritedScheme

  func body(content: Content) -> some View {
    let active = override ?? inherited
    content
      .environment(\.theme, active)
      .font(active.vars.bodyMedium.font)
      .environment(\.colorScheme, override?.colorScheme ?? inheritedScheme)
  }
}
