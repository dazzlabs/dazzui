import DazzUI
import SwiftUI
import XCTest

@testable import Example

final class GalleryTests: XCTestCase {
  private func nodes(_ tree: Element) -> [Element] { [tree] + tree.children.flatMap(nodes) }
  private func render<V: View>(
    _ view: V, size: SwiftUIRuntime.Size = .init(width: 1360, height: 860)
  ) -> Renderer {
    StateRegistry.shared.reset()
    let renderer = Renderer(backend: FakeBackend(), root: AnyView(view), size: size)
    renderer.mount()
    return renderer
  }
  func testEveryGeneratedSceneHasFiniteVisibleContent() throws {
    var count = 0
    XCTAssertGreaterThan(Stories.all.count, 40)
    for story in Stories.all {
      for scene in [story.preview] + story.examples.map({ $0.demo }) {
        let renderer = render(
          SceneView(scene: scene).theme(.brightLight).overlayHost(),
          size: .init(width: 720, height: 600))
        let all = nodes(try XCTUnwrap(renderer.tree))
        XCTAssertTrue(all.contains { $0.frame.width > 0 && $0.frame.height > 0 }, story.id)
        for node in all {
          if node.kind == .image, !node.props.imagePath.isEmpty {
            XCTAssertTrue(
              FileManager.default.fileExists(atPath: node.props.imagePath), node.props.imagePath)
          }
          XCTAssertTrue(
            node.frame.width.isFinite && node.frame.height.isFinite && node.frame.minX.isFinite
              && node.frame.minY.isFinite, story.id)
        }
        count += 1
      }
    }
    print(
      "Gallery coverage: " + String(Stories.all.count) + " stories, " + String(count)
        + " preview/example scenes")
  }
  func testEveryPageAndThemeRenders() throws {
    for story in Stories.all {
      let renderer = render(AppView(page: story.id))
      XCTAssertTrue(
        nodes(try XCTUnwrap(renderer.tree)).contains { $0.props.text == story.summary }, story.id)
    }
    for family in themeFamilyGroups.flatMap({ $0.families }) {
      for dark in [false, true] {
        let renderer = render(AppView(theme: ThemeChoice(family: family.value, dark: dark)))
        let all = nodes(try XCTUnwrap(renderer.tree))
        XCTAssertTrue(all.contains { $0.props.text == "Overview" }, family.value)
        XCTAssertTrue(
          all.contains { $0.kind == .image && !$0.props.imagePath.isEmpty }, family.value)
      }
    }
  }
  func testGallerySearchNavigationAndTheme() throws {
    let renderer = render(AppView())
    let search = try XCTUnwrap(
      nodes(renderer.tree!).first { $0.kind == .textField && $0.props.placeholder == "Search" })
    renderer.input(id: search.id, event: 7, text: "calendar")
    XCTAssertFalse(nodes(renderer.tree!).contains { $0.props.text == "Checkbox" })
    let calendar = try XCTUnwrap(
      nodes(renderer.tree!).first {
        $0.kind == .styledButton && nodes($0).contains { $0.props.text == "Calendar" }
      })
    renderer.performTap(calendar.id)
    XCTAssertTrue(
      nodes(renderer.tree!).contains { $0.props.text == Stories.story("calendar")?.summary })
    let themeControl = try XCTUnwrap(
      nodes(renderer.tree!).first {
        $0.props.accessibilityLabel == "Switch to dark"
      })
    let theme = try XCTUnwrap(nodes(themeControl).first { $0.kind == .styledButton })
    renderer.performTap(theme.id)
    XCTAssertTrue(
      nodes(renderer.tree!).contains { $0.props.accessibilityLabel == "Switch to light" })
  }

  func testThemePreviewInkIsStableAcrossCardHover() throws {
    let renderer = render(AppView(page: "button"))
    let theme = try XCTUnwrap(nodes(renderer.tree!).first {
      $0.kind == .styledButton && nodes($0).contains { $0.props.text == "Tokyo Night" }
    })
    renderer.performTap(theme.id)
    let scroll = try XCTUnwrap(nodes(renderer.tree!).first { $0.kind == .scroll && $0.frame.minX > 1000 })
    renderer.input(event: 5, point: CGPoint(x: scroll.frame.minX + 70, y: scroll.frame.minY + 150), value: -1200)
    let card = try XCTUnwrap(nodes(renderer.tree!).first {
      $0.kind == .styledButton && nodes($0).contains { $0.props.text == "Tokyo Night" }
    })
    func previewInk() throws -> [Element] {
      let current = try XCTUnwrap(nodes(renderer.tree!).first { $0.id == card.id })
      let preview = try XCTUnwrap(nodes(current).first { $0.kind == .geometry })
      return nodes(preview).filter {
        $0.kind == .shape && $0.frame.height > 0 && $0.frame.height <= 3 && $0.frame.width > 3
      }
    }
    let initial = try previewInk()
    XCTAssertEqual(initial.count, 2)
    XCTAssertEqual(initial.last?.props.opacity, 0.4)
    let other = try XCTUnwrap(
      nodes(renderer.tree!).first {
        $0.kind == .styledButton && nodes($0).contains { $0.props.text == "filled" }
      })
    for target in [card, other] {
      for hovering in [true, false, true, false] {
        renderer.input(
          event: hovering ? 1 : 10,
          point: CGPoint(
            x: target.frame.minX + target.frame.width / 2,
            y: target.frame.minY + target.frame.height / 2))
        let all = nodes(renderer.tree!)
        XCTAssertFalse(all.contains { $0.id.rawValue == 0 })
        XCTAssertEqual(Set(all.map(\.id)).count, all.count)
        let ink = try previewInk()
        XCTAssertEqual(ink.map { $0.props.background }, initial.map { $0.props.background })
        XCTAssertEqual(ink.map { $0.props.opacity }, initial.map { $0.props.opacity })
        XCTAssertEqual(ink.map { $0.frame }, initial.map { $0.frame })
      }
    }
  }
}
