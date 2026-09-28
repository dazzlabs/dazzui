import SwiftUI
import UIKit

@main
struct PlaygroundApp: App {
  init() {
    // UI automation pastes the declared string, preserving characters that
    // XCTest keyboard synthesis drops (for example U+FEFF).
    if let text = ProcessInfo.processInfo.environment["PLAYGROUND_TEST_CLIPBOARD"] {
      UIPasteboard.general.string = text
    }
  }

  var body: some Scene {
    WindowGroup {
      NativeControls()
    }
  }
}
