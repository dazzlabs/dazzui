import SwiftUI
import DazzMobileUI

/// The playground's root view, built entirely from native SwiftUI controls.
struct NativeControls: View {
  @State private var family: ThemeFamily = .bright
  @State private var appearance = Appearance.system

  var body: some View {
    Group {
      if let id = ProcessInfo.processInfo.environment["PLAYGROUND_STORY"],
         let story = mobileStories.first(where: { $0.id == id }) {
        ScrollView { story.build().padding() }.id(story.id)
      } else {
        ControlsForm(family: $family, appearance: $appearance)
      }
    }.mobileTheme(family, colorScheme: appearance.colorScheme)
  }
}

private enum Appearance: String, CaseIterable, Identifiable {
  case system, light, dark
  var id: String { rawValue }
  var colorScheme: ColorScheme? {
    switch self {
    case .system: return nil
    case .light: return .light
    case .dark: return .dark
    }
  }
}

private struct ControlsForm: View {
  @Binding var family: ThemeFamily
  @Binding var appearance: Appearance
  @State private var search = ""

  var body: some View {
    NavigationStack {
      List {
        Section("Theme") {
          Picker("Family", selection: $family) {
            ForEach(ThemeFamily.allCases) { Text($0.rawValue.capitalized).tag($0) }
          }
          Picker("Appearance", selection: $appearance) {
            ForEach(Appearance.allCases) { Text($0.rawValue.capitalized).tag($0) }
          }
        }
        ForEach(Array(Set(mobileStories.map(\.title))).sorted(), id: \.self) { title in
          Section(title) {
            ForEach(mobileStories.filter { $0.title == title && (search.isEmpty || "\($0.title) \($0.name)".localizedCaseInsensitiveContains(search)) }) { story in
              NavigationLink(story.name) {
                ScrollView { story.build().padding() }
                  .id(story.id)
                  .navigationTitle(story.name)
              }.accessibilityIdentifier(story.id)
            }
          }
        }
      }
      .searchable(text: $search, prompt: "Find a story")
      .navigationTitle("Playground")
    }
  }
}
