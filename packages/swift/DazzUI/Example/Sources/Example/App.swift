import DazzUI
import SwiftUI

/// The page being read and the theme it is read in — the two things the
/// React example keeps in its address — and the window that shows them.
struct AppView: View {
  @State private var page: String?
  @State private var theme: ThemeChoice

  init(page: String? = nil, theme: ThemeChoice = defaultTheme) {
    _page = State(initialValue: page)
    _theme = State(initialValue: theme)
  }

  var body: some View {
    Shell(page: $page, theme: $theme)
      .theme(theme.name)
      .overlayHost()
  }
}
