import SwiftUI

import XCTest

@testable import DazzUI

final class RenderingTests: XCTestCase {
  private func nodes(_ node: Element) -> [Element] { [node] + node.children.flatMap(nodes) }
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
