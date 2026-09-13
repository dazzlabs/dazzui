import XCTest
import SwiftUI
import AppKit
import DazzUI

@testable import Playground

/// The React story list is the contract's index.
///
/// `Parity/stories.json` is what `packages/ui_react/example` renders, written by its
/// parity extractor and distributed here; every id in it is a story this
/// playground is expected to draw under the same name. A story missing here
/// is a gap in the port, and a story here that React does not have is a name
/// the port invented — both are the kind of drift that used to be invisible.
///
/// The two lists beside it are the ratchet: `pending.txt` names the React
/// stories not drawn yet, `extra.txt` the ones drawn here that React has no
/// counterpart for. Each is held both ways, so an entry cannot go stale — a
/// pending story that lands has to leave the list in the same change.
///
/// None of this is committed: the data is what `pnpm parity` writes, and the
/// lists are recorded by the first run that finds them missing. A checkout
/// without the data skips rather than fails.
final class StoriesTests: XCTestCase {
  @MainActor func testEmptyStateWithoutActionHasNoPhantomActionGap() {
    let vars = ThemeName.studioLight.variables
    let actual = NSHostingView(rootView: EmptyState("Nothing")
      .theme(.studioLight).fixedSize(horizontal: false, vertical: true).frame(width: 320))
    let expected = NSHostingView(rootView: Text("Nothing")
      .font(vars.bodyMedium.font).lineSpacing(vars.bodyMedium.lineSpacing)
      .padding(.vertical, vars.spacing14).padding(.horizontal, vars.spacing8)
      .frame(width: 320))
    XCTAssertEqual(actual.fittingSize.height, expected.fittingSize.height, accuracy: 0.5)
  }

  @MainActor func testGeneratedScenesRenderInEveryTheme() throws {
    for entry in GeneratedStories.entries {
      for theme in ThemeName.allCases {
        let view = try XCTUnwrap(GeneratedStories.content("\(entry.group)/\(entry.page)/\(entry.name)"))
        let host = NSHostingView(rootView: view.theme(theme).frame(width: 960, height: 640))
        host.frame = NSRect(x: 0, y: 0, width: 960, height: 640)
        host.layoutSubtreeIfNeeded()
        let bitmap = try XCTUnwrap(host.bitmapImageRepForCachingDisplay(in: host.bounds))
        host.cacheDisplay(in: host.bounds, to: bitmap)
        XCTAssertGreaterThan(bitmap.pixelsWide, 0, "\(entry.id) / \(theme.rawValue)")
      }
    }
  }

  func testGeneratedStoriesCreateTheirOwnCatalog() {
    let groups = GeneratedStories.apply([])
    let ids = groups.flatMap(\.pages).flatMap(\.stories).compactMap(\.sourceId)
    XCTAssertEqual(Set(ids), Set(GeneratedStories.entries.map(\.id)))
    XCTAssertEqual(ids.count, Set(ids).count)
    XCTAssertEqual(GeneratedStories.apply(groups).flatMap(\.pages).flatMap(\.stories).count, ids.count)
  }

  func testGeneratedStoriesPreserveExtensionsAndReplaceMatchingEntries() {
    let input = [StoryGroup("Components", pages: [StoryPage("Button", stories: [
      Story("Normal") { EmptyView() },
      Story("Local extension") { EmptyView() },
    ])])]
    let output = GeneratedStories.apply(input)
    let stories = output[0].pages.first(where: { $0.name == "Button" })!.stories
    XCTAssertEqual(stories.filter { $0.name == "Normal" }.count, 1)
    XCTAssertEqual(stories.first(where: { $0.name == "Normal" })?.sourceId, "components-button--normal")
    XCTAssertTrue(stories.contains { $0.name == "Local extension" })
    XCTAssertEqual(input[0].pages[0].stories.count, 2)
  }

  private var parity: URL {
    URL(fileURLWithPath: #filePath)
      .deletingLastPathComponent()
      .deletingLastPathComponent()
      .deletingLastPathComponent()
      .appendingPathComponent("Parity")
  }

  private func listed(_ file: String) throws -> Set<String> {
    let text = try String(contentsOf: parity.appendingPathComponent(file), encoding: .utf8)
    return Set(
      text.split(separator: "\n")
        .map { $0.trimmingCharacters(in: .whitespaces) }
        .filter { !$0.isEmpty && !$0.hasPrefix("#") }
    )
  }

  /// The list as it stands, or — on a first run — the list as it is now,
  /// written down so the next run is held to it.
  private func ratchet(_ file: String, note: String, current: Set<String>) throws -> Set<String> {
    let url = parity.appendingPathComponent(file)
    if FileManager.default.fileExists(atPath: url.path) { return try listed(file) }
    let lines =
      [
        "# \(note)",
        "# Recorded by the first test run; held both ways from now on, so it",
        "# can only shrink. Shrink it; never grow it to pass.",
      ] + current.sorted()
    try (lines.joined(separator: "\n") + "\n").write(to: url, atomically: true, encoding: .utf8)
    print("recorded \(current.count) entries into \(file)")
    return current
  }

  private struct Manifest: Decodable {
    struct Entry: Decodable { let id: String }
    let stories: [Entry]
  }

  func testEveryStoryIdIsUnique() {
    let ids = StoryIndex.ids
    XCTAssertEqual(Set(ids).count, ids.count, "two stories share an id")
  }

  func testEveryReactStoryIsDrawnHereOrListedAsPending() throws {
    let stories = parity.appendingPathComponent("stories.json")
    try XCTSkipUnless(
      FileManager.default.fileExists(atPath: stories.path),
      "no Parity/stories.json — run `pnpm parity` at the root")
    let data = try Data(contentsOf: stories)
    let react = Set(try JSONDecoder().decode(Manifest.self, from: data).stories.map(\.id))
    let drawn = Set(StoryIndex.ids)

    let missing = react.subtracting(drawn)
    let surplus = drawn.subtracting(react)
    let pending = try ratchet(
      "pending.txt",
      note: "React stories this playground does not draw yet, one id per line.",
      current: missing)
    let extra = try ratchet(
      "extra.txt",
      note: "Stories this playground draws that React has no counterpart for.",
      current: surplus)

    var problems: [String] = []
    for id in missing.subtracting(pending) {
      problems.append("React draws \(id) and this playground does not — draw it, or add it to pending.txt")
    }
    for id in pending.subtracting(missing) {
      problems.append("\(id) is in pending.txt but is drawn now, or gone from React — remove it")
    }
    for id in surplus.subtracting(extra) {
      problems.append("\(id) is drawn here but is not a React story — rename it, or add it to extra.txt")
    }
    for id in extra.subtracting(surplus) {
      problems.append("\(id) is in extra.txt but is no longer drawn — remove it")
    }

    XCTAssertTrue(problems.isEmpty, problems.sorted().joined(separator: "\n"))
  }
}
