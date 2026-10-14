import AppKit
import DazzUI
import SwiftUI
import XCTest

@testable import Example

/// The SwiftUI collector: draws every scene in scenes.json at 2x, in every
/// theme asked for, and writes readings.json, the scene images and a crop
/// per specimen. See specs/conformance.md. It runs only when
/// `tool/conformance.sh` hands it these, and is skipped otherwise:
///
/// - `DAZZUI_CONFORMANCE_OUTPUT` — where the readings go;
/// - `DAZZUI_CONFORMANCE_CONTRACT` — scenes.json;
/// - `DAZZUI_CONFORMANCE_THEMES` — `drawn`, `all` or a list;
/// - `DAZZUI_CONFORMANCE_SCENES` — stories to keep, by id;
/// - `DAZZUI_CONFORMANCE_IMAGES` — `false` to skip pictures;
/// - `DAZZUI_CONFORMANCE_COMMIT` — the commit being measured.
///
/// A SwiftUI view is a value: it can be laid out and drawn, but it cannot
/// say what colour it painted or what padding it kept. So this reads each
/// specimen's box, from the frame it reports under the `$at` codegen wrote
/// into its node, and leaves every other property unread.
@MainActor
final class ConformanceCollector: XCTestCase {
  private static let scale: CGFloat = 2

  /// Where the frames a drawn scene reports are kept.
  private final class Frames: @unchecked Sendable {
    var byIndex: [Int: CGRect] = [:]
  }

  func testCollect() throws {
    let env = ProcessInfo.processInfo.environment
    func value(_ name: String) -> String? {
      env["DAZZUI_CONFORMANCE_\(name)"].flatMap { $0.isEmpty ? nil : $0 }
    }
    guard let output = value("OUTPUT") else {
      throw XCTSkip("Run tool/conformance.sh")
    }
    let out = URL(fileURLWithPath: output)
    let images = value("IMAGES") != "false"
    let only = value("SCENES")?.split(separator: ",").map(String.init)
    let contractPath = try XCTUnwrap(value("CONTRACT"), "No scenes.json given")
    let contract = try XCTUnwrap(
      JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: contractPath)))
        as? [String: Any]
    )

    func halves(_ families: [ThemeFamily]) -> [ThemeChoice] {
      families.flatMap { [ThemeChoice(family: $0.value), ThemeChoice(family: $0.value, dark: true)] }
    }
    let themes: [ThemeChoice] =
      switch value("THEMES") ?? "drawn" {
      case "drawn": halves(themeFamilyGroups[0].families)
      case "all": halves(themeFamilyGroups.flatMap(\.families))
      case let list:
        list.split(separator: ",").map { id in
          let cut = id.lastIndex(of: "-")!
          return ThemeChoice(family: String(id[..<cut]), dark: id[id.index(after: cut)...] == "dark")
        }
      }

    // Inferred: `Scene` is both this module's type and SwiftUI's.
    let drawable = Dictionary(
      uniqueKeysWithValues: Stories.all.flatMap { story in
        [("\(story.id)/preview", story.preview)]
          + story.examples.enumerated().map { ("\(story.id)/\($0.offset)", $0.element.demo) }
      }
    )
    let stories = (contract["stories"] as? [[String: Any]] ?? [])
    let scenes = stories.flatMap { $0["scenes"] as? [[String: Any]] ?? [] }
      .filter { scene in
        let id = scene["id"] as? String ?? ""
        return only == nil || only!.contains { id == $0 || id.hasPrefix($0 + "/") }
      }

    try? FileManager.default.removeItem(at: out)
    try FileManager.default.createDirectory(at: out, withIntermediateDirectories: true)
    var readingsThemes: [String: Any] = [:]
    var readingsScenes: [String: Any] = [:]
    var failures: [String] = []

    for choice in themes {
      let vars = choice.name.variables
      var readingsOf: [String: Any] = [:]
      var scenesOf: [String: Any] = [:]
      let imageDir = out.appendingPathComponent("images/\(choice.id)")
      if images { try FileManager.default.createDirectory(at: imageDir, withIntermediateDirectories: true) }

      for contracted in scenes {
        let id = contracted["id"] as? String ?? ""
        let specimens = contracted["specimens"] as? [[String: Any]] ?? []
        guard let scene = drawable[id] else {
          failures.append("\(id): the example has no such scene")
          continue
        }
        // Codegen addressed each specimen; the scene must carry the same.
        let carried = Self.addressed(scene.content)
        let want = specimens.map { $0["component"] as? String ?? "" }
        let got = want.indices.map { carried[$0].map(Self.kebab) ?? "nothing" }
        if want != got || carried.count != want.count {
          failures.append("\(id): codegen addressed \(want.joined(separator: " ")), the scene carries \(got.joined(separator: " "))")
          continue
        }

        let frames = Frames()
        // The frame every collector draws a scene in: 40rem wide, spacing-6
        // around it, on the surface, its content centred.
        let root = SceneView(scene: scene)
          .frame(maxWidth: .infinity)
          .padding(vars.spacing6)
          .frame(width: 640)
          .fixedSize(horizontal: false, vertical: true)
          .background(vars.colorSurface)
          .coordinateSpace(name: specimenSpace)
          .font(vars.bodyMedium.font)
          .foregroundStyle(vars.colorContent)
          .environment(\.collectsSpecimens, true)
          .onPreferenceChange(SpecimenFrames.self) { frames.byIndex = $0 }
          .theme(choice.name)
          .overlayHost()
        let host = NSHostingView(rootView: root)
        host.frame = NSRect(x: 0, y: 0, width: 640, height: 10)
        host.frame.size = host.fittingSize
        // In a window, layout rounds to the screen's pixels — half points on
        // a 2x display, as React's page is read — rather than whole ones.
        let window = NSWindow(
          contentRect: host.frame, styleMask: .borderless, backing: .buffered, defer: false
        )
        window.contentView = host
        host.layoutSubtreeIfNeeded()
        // Let the preferences the layout produced reach their handler.
        RunLoop.main.run(until: Date().addingTimeInterval(0.02))
        host.layoutSubtreeIfNeeded()

        // A frame is all there is to go on here, and a container exactly
        // its one child's size is common, so a wrapper that draws nothing
        // of its own is not told apart: its reading is simply one the
        // reference has none for, and nothing holds it.
        let rects: [CGRect?] = specimens.indices.map { index in
          guard let rect = frames.byIndex[index], rect.width > 0 || rect.height > 0 else { return nil }
          return rect
        }

        let file = id.replacingOccurrences(of: "/", with: "-")
        var crops: [String?] = rects.map { _ in nil }
        if images, let picture = Self.draw(host) {
          try Self.png(picture, to: imageDir.appendingPathComponent("\(file).png"))
          scenesOf[id] = "images/\(choice.id)/\(file).png"
          for (index, rect) in rects.enumerated() {
            guard let rect else { continue }
            let pixels = CGRect(
              x: rect.minX * Self.scale, y: rect.minY * Self.scale,
              width: rect.width * Self.scale, height: rect.height * Self.scale
            ).integral.intersection(CGRect(x: 0, y: 0, width: picture.width, height: picture.height))
            guard !pixels.isEmpty, let crop = picture.cropping(to: pixels) else { continue }
            let name = "\(file)-\(index).png"
            try Self.png(crop, to: imageDir.appendingPathComponent(name))
            crops[index] = name
          }
        }

        for (index, rect) in rects.enumerated() {
          guard let rect, let address = specimens[index]["address"] as? String else { continue }
          var reading: [String: Any] = [
            "box": ["width": Self.round(rect.width), "height": Self.round(rect.height)],
            // Nothing else a SwiftUI view drew can be read back from it.
            "values": [String: Any](),
          ]
          if let crop = crops[index] { reading["image"] = "images/\(choice.id)/\(crop)" }
          readingsOf[address] = reading
        }
      }
      readingsThemes[choice.id] = readingsOf
      readingsScenes[choice.id] = scenesOf
    }

    let readings: [String: Any] = [
      "platform": "swiftui",
      "title": "SwiftUI",
      "role": "implementation",
      "collected": ISO8601DateFormatter().string(from: Date()),
      "commit": value("COMMIT") ?? "",
      "scale": Self.scale,
      // The same system face as React's, and no embedded test face.
      "font": NSNull(),
      "themes": readingsThemes,
      "scenes": readingsScenes,
    ]
    try JSONSerialization.data(withJSONObject: readings)
      .write(to: out.appendingPathComponent("readings.json"))
    if !failures.isEmpty {
      try (failures.joined(separator: "\n") + "\n")
        .write(to: out.appendingPathComponent("failures.txt"), atomically: true, encoding: .utf8)
    }
  }

  /// Every `$at` a scene's JSON carries, and the component at it.
  private static func addressed(_ value: Value) -> [Int: String] {
    var found: [Int: String] = [:]
    func visit(_ value: Value) {
      switch value {
      case .array(let items): items.forEach(visit)
      case .object(let object):
        if let at = object["$at"]?.number, let component = object["$el"]?.string {
          found[Int(at)] = component
        }
        object.values.forEach(visit)
      default: break
      }
    }
    visit(value)
    return found
  }

  /// `SegmentedControl` → `segmented-control`, as scenes.json names it.
  private static func kebab(_ name: String) -> String {
    name.replacingOccurrences(of: "([a-z0-9])([A-Z])", with: "$1-$2", options: .regularExpression)
      .lowercased()
  }

  private static func round(_ n: CGFloat) -> Double { (Double(n) * 100).rounded() / 100 }

  /// The host drawn at the collector's scale, whatever the screen's.
  private static func draw(_ host: NSView) -> CGImage? {
    let size = host.bounds.size
    guard size.width >= 1, size.height >= 1,
      let rep = NSBitmapImageRep(
        bitmapDataPlanes: nil,
        pixelsWide: Int((size.width * scale).rounded()),
        pixelsHigh: Int((size.height * scale).rounded()),
        bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true, isPlanar: false,
        colorSpaceName: .calibratedRGB, bytesPerRow: 0, bitsPerPixel: 0
      )
    else { return nil }
    rep.size = size
    host.cacheDisplay(in: host.bounds, to: rep)
    return rep.cgImage
  }

  private static func png(_ image: CGImage, to url: URL) throws {
    let data = try XCTUnwrap(NSBitmapImageRep(cgImage: image).representation(using: .png, properties: [:]))
    try data.write(to: url)
  }
}
