import DazzUI
import SwiftUI

/// The page being read and the theme it is read in — the two things the
/// React example keeps in its address — and the window that shows them.
struct AppView: View {
  @State private var page: String?
  @State private var theme: ThemeChoice
  @State private var sizing: ThemeSizing
  @State private var menuPresentation: MenuPresentation = .custom

  init(page: String? = nil, theme: ThemeChoice = defaultTheme, sizing: ThemeSizing = .design) {
    _page = State(initialValue: page)
    _theme = State(initialValue: theme)
    _sizing = State(initialValue: sizing)
  }

  var body: some View {
    Shell(page: $page, theme: $theme, sizing: $sizing, menuPresentation: $menuPresentation)
      .menuPresentation(menuPresentation)
      .theme(theme.name, sizing: sizing)
      .overlayHost()
  }
}
