import AppKit
import DazzUI
import SwiftUI
import XCTest

@testable import Example

/// Draws every story's preview and every example, in light and dark — the
/// counterpart of the Flutter example's `stories_test.dart`. A component or
/// glyph the interpreter cannot draw stops here rather than in a window.
@MainActor
final class StoriesTests: XCTestCase {
  /// Lays a view out at the stage's size and draws it once.
  private func draw<Content: View>(_ view: Content, _ theme: ThemeName) {
    let host = NSHostingView(
      rootView: view
        .frame(width: 720, height: 480)
        .theme(theme)
        .overlayHost()
    )
    host.frame = NSRect(x: 0, y: 0, width: 720, height: 480)
    host.layoutSubtreeIfNeeded()
    let image = host.bitmapImageRepForCachingDisplay(in: host.bounds)
    XCTAssertNotNil(image)
    if let image { host.cacheDisplay(in: host.bounds, to: image) }
  }

  func testEveryStoryIsCompiledFromReact() {
    XCTAssertEqual(Stories.all.count, 51)
    XCTAssertEqual(Stories.all.map(\.name), Stories.all.map(\.name).sorted {
      $0.localizedCompare($1) == .orderedAscending
    })
    for story in Stories.all {
      XCTAssertFalse(story.examples.isEmpty, story.id)
    }
  }

  func testEveryPreviewAndExampleDraws() {
    for theme in [ThemeName.studioLight, .studioDark] {
      for story in Stories.all {
        draw(SceneView(scene: story.preview), theme)
        for example in story.examples {
          draw(SceneView(scene: example.demo), theme)
        }
      }
    }
  }

  func testTheWindowDrawsTheOverviewAndAStory() {
    draw(AppView(), .studioLight)
    draw(AppView(page: "button", theme: ThemeChoice(family: "nocturne", dark: true)), .nocturneDark)
  }

  func testEveryThemeThePanelOffersHasANameInTheKit() {
    for group in themeFamilyGroups {
      for family in group.families {
        for dark in [false, true] {
          let choice = ThemeChoice(family: family.value, dark: dark)
          XCTAssertEqual(choice.name.rawValue, choice.id)
        }
      }
    }
  }

  func testAnEventWritesASlotAnotherComponentReads() {
    let story = Stories.story("option-card")!
    let state = SceneState(story.preview.state)
    let context = Node.Context(scene: state, toaster: nil, press: nil)
    let free = story.preview.content["children"]!.array!.first!
    let node = Node(free, context)
    XCTAssertEqual(state.slots["s0"], .string("team"))
    XCTAssertFalse(node.flag("selected"))
    node.action("onClick")?()
    XCTAssertEqual(state.slots["s0"], .string("free"))
    XCTAssertTrue(node.flag("selected"))
  }

  func testAToastIsRaisedThroughItsProvider() {
    let toaster = Toaster()
    let context = Node.Context(scene: SceneState([:]), toaster: toaster, press: nil)
    context.run(.object(["$do": .array([.object(["toast": .object(["title": .string("Saved")])])])]))
    XCTAssertEqual(toaster.toasts.map(\.options.title), ["Saved"])
  }
}
