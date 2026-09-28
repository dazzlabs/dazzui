import XCTest

final class InteractionTests: XCTestCase {
  // The actions and expectations are the same declarations used by React.
  // Timed checkpoints require a virtual native clock and are covered separately.
  @MainActor
  func testSharedInputAndToggleCheckpoints() throws {
    continueAfterFailure = false
    let url = try XCTUnwrap(Bundle(for: Self.self).url(forResource: "scenes", withExtension: "json"))
    let scenes = try XCTUnwrap(JSONSerialization.jsonObject(with: Data(contentsOf: url)) as? [String: [String: Any]])
    for (id, scene) in scenes.sorted(by: { $0.key < $1.key }) {
      guard scene["timers"] == nil, let checkpoints = scene["checkpoints"] as? [[String: Any]] else { continue }
      for checkpoint in checkpoints {
        let app = XCUIApplication()
        app.launchEnvironment["PLAYGROUND_STORY"] = id
        let fills = (checkpoint["steps"] as? [[String: Any]] ?? []).filter { $0["action"] as? String == "fill" }
        XCTAssertLessThanOrEqual(fills.count, 1, "Extend clipboard automation for multiple fills in one checkpoint")
        if let text = fills.first?["value"] as? String {
          app.launchEnvironment["PLAYGROUND_TEST_CLIPBOARD"] = text
        }
        app.launch()
        for step in checkpoint["steps"] as? [[String: Any]] ?? [] {
          let target = try XCTUnwrap(step["target"] as? String)
          let element = app.descendants(matching: .any).matching(identifier: target).firstMatch
          XCTAssertTrue(element.waitForExistence(timeout: 5), "\(id) / \(target)")
          if step["action"] as? String == "click" {
            element.tap()
          } else if step["action"] as? String == "fill" {
            let input = app.textFields[target]
            input.tap()
            let old = input.value as? String ?? ""
            input.typeText(String(repeating: XCUIKeyboardKey.delete.rawValue, count: old.count))
            let value = try XCTUnwrap(step["value"] as? String)
            if !value.isEmpty {
              input.press(forDuration: 1)
              let paste = app.descendants(matching: .any).matching(NSPredicate(format: "label == %@", "Paste")).firstMatch
              XCTAssertTrue(paste.waitForExistence(timeout: 3), id)
              paste.tap()
            }
          } else { XCTFail("Unsupported checkpoint action") }
        }
        for (target, disabled) in checkpoint["expectDisabled"] as? [String: Bool] ?? [:] {
          XCTAssertEqual(app.buttons[target].isEnabled, !disabled, "\(id) / \(checkpoint)")
        }
        for (target, expected) in checkpoint["expectText"] as? [String: String] ?? [:] {
          let element = app.descendants(matching: .any).matching(identifier: target).firstMatch
          let labels = element.descendants(matching: .staticText).allElementsBoundByIndex.map(\.label)
          XCTAssertEqual(labels.isEmpty ? element.label : labels.joined(), expected, "\(id) / \(target)")
        }
        var observedStateKeys = Set<String>()
        func assertBindings(_ raw: Any) throws {
          guard let node = raw as? [String: Any] else { return }
          let expected = checkpoint["expect"] as? [String: Any] ?? [:]
          if let binding = node["bind"] as? [String: String], let target = node["id"] as? String {
            if let key = binding["checked"], let value = expected[key] as? Bool {
              observedStateKeys.insert(key)
              XCTAssertEqual(app.switches[target].value as? String, value ? "1" : "0", id)
            }
            if let key = binding["value"], let value = expected[key] as? String {
              observedStateKeys.insert(key)
              XCTAssertEqual(app.textFields[target].value as? String, value, id)
            }
          }
          if node["component"] as? String == "Dialog", let binding = node["bind"] as? [String: String], let key = binding["open"], let value = expected[key] as? Bool {
            observedStateKeys.insert(key)
            XCTAssertEqual(app.descendants(matching: .any).matching(identifier: "scene-dialog").firstMatch.waitForExistence(timeout: 2), value, id)
          }
          for child in node["children"] as? [Any] ?? [] { try assertBindings(child) }
          for child in (node["slots"] as? [String: Any] ?? [:]).values { try assertBindings(child) }
        }
        try assertBindings(try XCTUnwrap(scene["content"]))
        XCTAssertEqual(observedStateKeys, Set((checkpoint["expect"] as? [String: Any] ?? [:]).keys), "Every state expectation must be observed: \(id)")
        app.terminate()
      }
    }
  }
}
