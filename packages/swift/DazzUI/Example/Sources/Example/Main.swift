import DazzUI
import SwiftUI

@main
struct ExampleApp: App {
  private let location = readLocation()

  private var sizing: ThemeSizing {
    #if os(WASI)
      return .design
    #else
      return ProcessInfo.processInfo.arguments.contains("-design-sizing") ? .design : .system
    #endif
  }

  var body: some SwiftUI.Scene {
    WindowGroup("DazzUI — SwiftUI example") {
      AppView(page: location.page, theme: location.theme, sizing: sizing)
        .frame(minWidth: 960, minHeight: 600)
    }
    .defaultSize(width: 1360, height: 860)
  }
}
