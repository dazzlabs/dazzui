import XCTest

@testable import Playground

/// The React story list is the contract's index.
///
/// `Parity/stories.json` is what `apps/storybook` renders, written by its
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
