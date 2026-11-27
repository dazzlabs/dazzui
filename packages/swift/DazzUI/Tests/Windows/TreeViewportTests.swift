import DazzUI
import SwiftUI
import XCTest

final class TreeViewportTests: XCTestCase {
  private struct Visits: Equatable {
    var bodies = 0
    var measures = 0
    var placements = 0
  }

  private final class Counter {
    var visits = Visits()

    func take() -> Visits {
      defer { visits = Visits() }
      return visits
    }
  }

  private struct LabelLayout: Layout {
    let counter: Counter

    func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
      counter.visits.measures += 1
      return subviews[0].sizeThatFits(proposal)
    }

    func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {
      counter.visits.placements += 1
      subviews[0].place(at: bounds.origin, anchor: .topLeading, proposal: proposal)
    }
  }

  private struct MeasuredLabel: View {
    let index: Int
    let counter: Counter

    var body: some View {
      counter.visits.bodies += 1
      return LabelLayout(counter: counter) { Text("Entry \(index)") }
    }
  }

  @MainActor
  func testExpandedTreeRowVisitsAtFixedScrollOffsets() throws {
    func measure(_ count: Int) throws -> [Visits] {
      StateRegistry.shared.reset()
      let counter = Counter()
      let leaves = (1..<count).map { index in
        TreeNode(id: "row-\(index)", label: MeasuredLabel(index: index, counter: counter), text: "Entry \(index)")
      }
      let roots = [TreeNode(id: "root", label: "Expanded folder", children: leaves)]
      let view = ScrollView { Tree(items: roots, expanded: ["root"]) }.theme(.studioLight)
      let renderer = Renderer(backend: FakeBackend(), root: AnyView(view), size: Size(width: 640, height: 480))
      renderer.mount()
      defer { renderer.unmount() }

      func scrollView(in element: Element) -> Element? {
        if element.kind == .scroll { return element }
        for child in element.children {
          if let match = scrollView(in: child) { return match }
        }
        return nil
      }

      var samples = [counter.take()]
      for step in 1...60 {
        let delta = step <= 30 ? -70 : 70
        XCTAssertTrue(renderer.input(event: 5, point: CGPoint(x: 260, y: 220), value: delta))
        let scroll = try XCTUnwrap(scrollView(in: XCTUnwrap(renderer.tree)))
        XCTAssertEqual(scroll.props.scrollOffset.y, Double(min(step, 60 - step) * 28), accuracy: 0.01)
        samples.append(counter.take())
      }
      return samples
    }

    let thousand = try measure(1_000)
    XCTAssertTrue(thousand.dropFirst().contains { $0.bodies > 0 && $0.measures > 0 && $0.placements > 0 })
    XCTAssertEqual(try measure(10_000), thousand)
  }
}
