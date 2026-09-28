import XCTest
import SwiftUI
import DazzMobileUI
@testable import DazzMobilePlayground

final class RenderTests: XCTestCase {
  @MainActor
  func testSharedCatalogAndNativeRendering() throws {
    continueAfterFailure = false
    let windowScene = try XCTUnwrap(UIApplication.shared.connectedScenes.compactMap { $0 as? UIWindowScene }.first)
    let url = try XCTUnwrap(Bundle(for: Self.self).url(forResource: "scenes", withExtension: "json"))
    let scenes = try XCTUnwrap(JSONSerialization.jsonObject(with: Data(contentsOf: url)) as? [String: Any])
    XCTAssertEqual(Set(mobileStories.map(\.id)), Set(scenes.keys))
    XCTAssertEqual(mobileStories.count, Set(mobileStories.map(\.id)).count)
    for family in ThemeFamily.allCases {
      for scheme in [ColorScheme.light, .dark] {
        for story in mobileStories {
          autoreleasepool {
            let host = UIHostingController(rootView: story.build().mobileTheme(family, colorScheme: scheme))
            let window = UIWindow(windowScene: windowScene)
            window.frame = CGRect(x: 0, y: 0, width: 390, height: 844)
            window.rootViewController = host
            window.makeKeyAndVisible()
            host.loadViewIfNeeded()
            host.view.frame = CGRect(x: 0, y: 0, width: 390, height: 844)
            host.view.setNeedsLayout()
            host.view.layoutIfNeeded()
            let format = UIGraphicsImageRendererFormat()
            format.scale = 1
            let image = UIGraphicsImageRenderer(size: host.view.bounds.size, format: format).image { context in
              XCTAssertTrue(window.drawHierarchy(in: window.bounds, afterScreenUpdates: true), story.id)
            }
            XCTAssertNotNil(image.cgImage, "\(story.id) \(family) \(scheme)")
            window.isHidden = true
            window.rootViewController = nil
          }
        }
      }
    }
  }
}
