import SwiftUI

import XCTest

@testable import DazzUI

final class RenderingTests: XCTestCase {
  private func nodes(_ node: Element) -> [Element] { [node] + node.children.flatMap(nodes) }
  @MainActor
  private final class Documents: ObservableObject {
    @Published var selection = 255
    var choices: [Int] = []
  }

  private struct DocumentTabs: View {
    @ObservedObject var documents: Documents
    let items: [TabItem<Int>]

    init(documents: Documents, count: Int = 256) {
      self.documents = documents
      self.items = (0..<count).map { TabItem(value: $0, label: "Document \($0)", icon: .document20Regular) }
    }

    var body: some View {
      DazzUI.Tabs(items: items, value: documents.selection) { value in
        documents.selection = value
        documents.choices.append(value)
      }
      .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
      .menuPresentation(.custom)
      .theme(.studioLight)
      .overlayHost()
    }
  }

  @MainActor
  private func settledTree(_ renderer: Renderer) throws -> Element {
    // Geometry preferences and the fit they determine settle on separate turns.
    for _ in 0..<12 {
      RunLoop.main.run(until: Date().addingTimeInterval(0.01))
      renderer.renderIfNeeded()
    }
    return try XCTUnwrap(renderer.tree)
  }

  private func buttons(_ root: Element) -> [Element] {
    nodes(root).filter { $0.kind == .styledButton || $0.kind == .button }
  }

  @MainActor
  func testShortTabsKeepTheirLeadingEdgeAndIntrinsicWidthOnResize() throws {
    StateRegistry.shared.reset()
    let documents = Documents()
    documents.selection = 7
    let renderer = Renderer(backend: FakeBackend(), root: AnyView(DocumentTabs(documents: documents, count: 8)),
                            size: Size(width: 1360, height: 600))
    renderer.mount()
    defer { renderer.unmount() }

    let original = buttons(try settledTree(renderer)).map(\.frame)
    XCTAssertEqual(original.count, 8)
    let first = try XCTUnwrap(original.first)
    XCTAssertEqual(first.minX, 0, accuracy: 0.5)
    XCTAssertLessThan(try XCTUnwrap(original.last).maxX, 1040)

    for width in [1040.0, 1207.0, 1360.0] {
      renderer.resize(to: Size(width: width, height: 600))
      XCTAssertEqual(buttons(try settledTree(renderer)).map(\.frame), original)
    }
  }

  @MainActor
  func testLargeTabsUsesAvailableWidthAndPromotesTheOverflowSelection() throws {
    StateRegistry.shared.reset()
    let documents = Documents()
    let backend = FakeBackend()
    let renderer = Renderer(backend: backend, root: AnyView(DocumentTabs(documents: documents)),
                            size: Size(width: 1360, height: 600))
    renderer.mount()
    defer { renderer.unmount() }

    let wide = buttons(try settledTree(renderer)).count
    XCTAssertGreaterThan(wide, 10)
    XCTAssertLessThan(wide, 256)
    renderer.resize(to: Size(width: 700, height: 600))
    let narrow = buttons(try settledTree(renderer)).count
    XCTAssertLessThan(narrow, wide)
    XCTAssertGreaterThan(narrow, 3)
    renderer.resize(to: Size(width: 1360, height: 600))
    let restored = buttons(try settledTree(renderer))
    XCTAssertEqual(restored.count, wide)
    let active = try XCTUnwrap(restored.flatMap(nodes).first { $0.props.text == "Document 255" })
    XCTAssertGreaterThan(active.frame.width, 50)
    let more = try XCTUnwrap(restored.max { $0.frame.minX < $1.frame.minX })
    backend.tap(more.id)
    let menuItem = try XCTUnwrap(nodes(try settledTree(renderer)).first { $0.props.text == "Document 200" })
    backend.tap(menuItem.id)
    let promoted = buttons(try settledTree(renderer))
    XCTAssertEqual(documents.choices, [200])
    XCTAssertLessThan(promoted.count, 256)
    let label = try XCTUnwrap(promoted.flatMap(nodes).first { $0.props.text == "Document 200" })
    XCTAssertGreaterThan(label.frame.width, 50)
  }

  func testSharedControlsInEveryTheme() {
    for name in ThemeName.allCases {
      StateRegistry.shared.reset()
      let backend = FakeBackend()
      let view = VStack(alignment: .leading) {
        DazzUI.Button("Action", variant: .filled) {}
        Badge("Status", tint: .success)
        Card { KeyCap("Enter", variant: .key) }
        DazzUI.Divider()
        SectionLabel("Section")
      }.theme(name)
      let renderer = Renderer(
        backend: backend, root: AnyView(view), size: Size(width: 640, height: 480))
      renderer.mount()
      let tree = nodes(renderer.tree!)
      XCTAssertEqual(tree.filter { $0.kind == .styledButton }.count, 1, "\(name)")
      for text in ["Action", "Status", "Enter", "Section"] {
        XCTAssertTrue(
          tree.contains { $0.kind == .text && $0.props.text == text && $0.frame.width > 0 },
          "\(name): \(text)")
      }
      XCTAssertTrue(
        tree.allSatisfy {
          $0.frame.width.isFinite && $0.frame.height.isFinite && $0.frame.width >= 0
            && $0.frame.height >= 0
        }, "\(name)")
      let button = tree.first { $0.kind == .styledButton }!
      XCTAssertEqual(
        button.frame.height, Double(name.variables.metrics(.small).size), accuracy: 0.01, "\(name)")
      let colors = name.variables.controlColors(variant: .filled, tint: .primary, state: .normal)
      XCTAssertTrue(
        nodes(button).contains { $0.kind == .shape && $0.props.background == colors.surface },
        "\(name): button fill uses shared recipe")
    }
  }
}
