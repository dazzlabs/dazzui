import SwiftUI
import DazzMobileUI

/// The playground's root view, built entirely from native SwiftUI controls.
struct NativeControls: View {
  @State private var family: ThemeFamily = .studio
  @State private var appearance = Appearance.system

  var body: some View {
    ControlsForm(family: $family, appearance: $appearance)
      .mobileTheme(family, colorScheme: appearance.colorScheme)
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
  @Environment(\.mobilePalette) private var palette
  @Binding var family: ThemeFamily
  @Binding var appearance: Appearance
  @State private var name = ""
  @State private var notifications = true
  @State private var volume = 0.5
  @State private var date = Date()
  @State private var count = 1
  @State private var showSheet = false
  @State private var showAlert = false

  var body: some View {
    NavigationStack {
      Form {
        Section("Theme") {
          Picker("Family", selection: $family) {
            ForEach(ThemeFamily.allCases) { family in
              Text(family.rawValue.capitalized).tag(family)
            }
          }
          Picker("Appearance", selection: $appearance) {
            ForEach(Appearance.allCases) { appearance in
              Text(appearance.rawValue.capitalized).tag(appearance)
            }
          }
        }
        Section("Controls") {
          TextField("Name", text: $name)
          Toggle("Notifications", isOn: $notifications)
          Slider(value: $volume) { Text("Volume") }
          Stepper("Copies: \(count)", value: $count, in: 1...10)
          DatePicker("Date", selection: $date, displayedComponents: .date)
          ProgressView("Progress", value: volume)
        }
        Section("Actions") {
          Button("Show sheet") { showSheet = true }
          Button("Save") { showAlert = true }
            .buttonStyle(.borderedProminent)
          Button("Unavailable") {}.disabled(true)
          Button("Delete", role: .destructive) { showAlert = true }
          Menu("More") {
            Button("Duplicate") { count += 1 }
            ShareLink(item: "DazzUI Mobile")
          }
          NavigationLink("Details") {
            Text("System navigation and Dynamic Type.")
              .navigationTitle("Details")
          }
        }
        Section("DS colours") {
          Label("Semantic content", systemImage: "paintpalette")
            .foregroundStyle(palette.colorContent)
            .listRowBackground(palette.colorSurfaceMuted)
        }
      }
      .navigationTitle("Native controls")
      .sheet(isPresented: $showSheet) {
        NavigationStack {
          Form {
            Toggle("Notifications", isOn: $notifications)
            Text("System sheet presentation")
          }
          .navigationTitle("Preferences")
          .toolbar {
            ToolbarItem(placement: .confirmationAction) {
              Button("Done") { showSheet = false }
            }
          }
        }
      }
      .alert("Saved", isPresented: $showAlert) {
        Button("OK", role: .cancel) {}
      }
    }
  }
}

#Preview {
  NativeControls()
}
