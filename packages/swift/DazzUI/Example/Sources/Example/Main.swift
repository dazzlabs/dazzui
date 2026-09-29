import SwiftUI

@main
struct ExampleApp: App {
  private let location = readLocation()

  var body: some SwiftUI.Scene {
    WindowGroup("DazzUI — SwiftUI example") {
      AppView(page: location.page, theme: location.theme)
        .frame(minWidth: 960, minHeight: 600)
    }
    .defaultSize(width: 1360, height: 860)
  }
}
