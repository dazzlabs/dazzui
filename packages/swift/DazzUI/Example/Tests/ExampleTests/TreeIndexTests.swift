import SwiftUI
import XCTest

@testable import DazzUI

@MainActor
final class TreeIndexTests: XCTestCase {
  func testExpandedRowsReuseTheIndexAcrossViewportUpdates() {
    let children = (1..<10_000).map { TreeNode(id: "\($0)", label: "File \($0)") }
    let roots = [TreeNode(id: "0", label: "Folder", children: children)]
    let cache = TreeRowsCache()
    let open = cache.rows(for: roots, expanded: ["0"])
    XCTAssertEqual(open.count, 10_000)
    for index in stride(from: 1, to: 10_000, by: 127) {
      let rows = cache.rows(for: roots, expanded: ["0"])
      XCTAssertTrue(rows === open)
      XCTAssertEqual(rows[index].id, "\(index)")
      XCTAssertEqual(rows[index].level, 2)
    }
    let closed = cache.rows(for: roots, expanded: [])
    XCTAssertEqual(closed.map(\.id), ["0"])
    XCTAssertFalse(closed[0].open)
    let reopened = cache.rows(for: roots, expanded: ["0"])
    XCTAssertEqual(reopened.count, 10_000)
    XCTAssertTrue(reopened[0].open)
  }

  func testEditingReorderingAndReplacingNodesWithTheSameIDRefreshesRows() {
    let branch = TreeNode(id: "folder", label: "Folder", children: [
      TreeNode(id: "file", label: Text("old"), text: "old"),
      TreeNode(id: "sibling", label: "Sibling"),
    ])
    var roots = [branch]
    let cache = TreeRowsCache()
    let original = cache.rows(for: roots, expanded: ["folder"])
    roots[0].children?[0] = TreeNode(id: "file", label: Text("edited"), text: "edited", enabled: false)
    let edited = cache.rows(for: roots, expanded: ["folder"])
    XCTAssertFalse(original === edited)
    XCTAssertEqual(original[1].node.label, "old")
    XCTAssertEqual(edited[1].node.label, "edited")
    XCTAssertFalse(edited[1].node.enabled)
    XCTAssertNotNil(edited[1].node.labelView)
    roots[0].children?.swapAt(0, 1)
    XCTAssertEqual(cache.rows(for: roots, expanded: ["folder"]).map(\.id), ["folder", "sibling", "file"])
    roots[0].children?.removeAll()
    let emptyFolder = cache.rows(for: roots, expanded: ["folder"])
    XCTAssertEqual(emptyFolder.count, 1)
    XCTAssertTrue(emptyFolder[0].open)
    roots = [TreeNode(id: "folder", label: "Now a leaf")]
    let leaf = cache.rows(for: roots, expanded: ["folder"])
    XCTAssertEqual(leaf[0].node.label, "Now a leaf")
    XCTAssertFalse(leaf[0].open)
  }
}
