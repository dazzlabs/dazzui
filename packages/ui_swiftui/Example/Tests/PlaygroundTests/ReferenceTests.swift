import AppKit
import DazzUI
import SwiftUI
import XCTest

/// The React reference, measured against.
///
/// `Parity/reference/*.json` is what `apps/storybook` rendered: every
/// component root on every story, in every theme, with its `data-*` axes and
/// the box, border, corner, colours and type the browser computed. This test
/// reads those probes back as *specimens* — a class and its axes — builds
/// each one here with the matching view, hosts it under the same theme, and
/// measures what it can. Where the two disagree is where the port has
/// drifted from the standard, and it is listed rather than argued about.
///
/// What can be read here without a screen is the box — an `NSHostingView`
/// reports the size a view asks for — and, for the controls that read the
/// recipe table, the colours the theme resolves for the variant, tint and
/// state, which is what the view paints with. A width that varies between
/// the stories a specimen appears in is skipped, because it belonged to the
/// story rather than the component.
///
/// `Parity/drift.txt` is the ratchet: every check that fails today is named
/// there, held both ways, so the list can only shrink. Bringing a component
/// into line means deleting its lines in the same change.
///
/// None of this is committed: the reference is what `pnpm parity` writes,
/// and the list is recorded by the first run that finds it missing. A
/// checkout without the reference skips rather than fails.
final class ReferenceTests: XCTestCase {
  private var parity: URL {
    URL(fileURLWithPath: #filePath)
      .deletingLastPathComponent()
      .appendingPathComponent("Parity")
  }

  func testEverySpecimenThePortCanBuildMeasuresAsReactDrewIt() throws {
    let reference = parity.appendingPathComponent("reference")
    try XCTSkipUnless(
      FileManager.default.fileExists(atPath: reference.path),
      "no Parity/reference — run `pnpm parity` at the root")
    let specimens = try loadSpecimens(reference)
    let driftFile = parity.appendingPathComponent("drift.txt")
    let recorded = FileManager.default.fileExists(atPath: driftFile.path)
    let driftText = recorded ? try String(contentsOf: driftFile, encoding: .utf8) : ""
    let listedDrift = Set(
      driftText.split(separator: "\n")
        .map { $0.trimmingCharacters(in: .whitespaces) }
        .filter { !$0.isEmpty && !$0.hasPrefix("#") }
    )

    var failing = Set<String>()
    var detail: [String: String] = [:]
    var unsupported = Set<String>()
    var built = 0
    // What this port measured, written beside the drift list for the viewer
    // (`pnpm parity:view`): the value on this side of every check.
    var measuredAll: [String: [String: [String: Any]]] = [:]

    for specimen in specimens {
      guard build(specimen.probe) != nil else {
        unsupported.insert(specimen.probe.className)
        continue
      }
      built += 1

      for (themeName, expected) in specimen.themes.sorted(by: { $0.key < $1.key }) {
        guard let theme = ThemeName(rawValue: themeName) else { continue }
        let measured = measure(specimen.probe, theme: theme, width: expected.width)
        let colors = recipe(specimen.probe, vars: theme.variables)
        var row: [String: Any] = ["width": measured.width, "height": measured.height]
        if let colors {
          row["background"] = hex(colors.surface)
          row["color"] = hex(colors.content)
          row["border-color"] = hex(colors.border)
        }
        measuredAll[specimen.key, default: [:]][themeName] = row

        func note(_ field: String, _ want: String, _ got: String) {
          let check = "\(specimen.key) · \(field)"
          failing.insert(check)
          if detail[check] == nil { detail[check] = "\(themeName): React \(want), here \(got)" }
        }

        if specimen.probe.text.isEmpty, let want = expected.width,
          abs(want - measured.width) > 0.5
        {
          note("width", "\(want)", "\(measured.width)")
        }
        if let want = expected.height, abs(want - measured.height) > 0.5 {
          note("height", "\(want)", "\(measured.height)")
        }
        if let colors {
          let pairs: [(String, String?, String)] = [
            ("background", expected.background, hex(colors.surface)),
            ("color", expected.color, hex(colors.content)),
            ("border-color", expected.borderColor, hex(colors.border)),
          ]
          for (field, want, got) in pairs {
            if let want, !sameColour(want, got) { note(field, want, got) }
          }
        }
      }
    }

    let report = try JSONSerialization.data(
      withJSONObject: ["platform": "swiftui", "specimens": measuredAll],
      options: [.prettyPrinted, .sortedKeys])
    try report.write(to: parity.appendingPathComponent("measured.json"))

    if !recorded {
      let lines =
        [
          "# Checks that fail against the React reference today, one per line:",
          "# a specimen (class and axes) and the field that differs.",
          "# Recorded by the first test run; held both ways from now on, so it",
          "# can only shrink. Run the test to see the values on each side.",
        ] + failing.sorted()
      try (lines.joined(separator: "\n") + "\n").write(
        to: driftFile, atomically: true, encoding: .utf8)
      print("recorded \(failing.count) checks into Parity/drift.txt")
    }
    let drift = recorded ? listedDrift : failing

    var problems: [String] = []
    for check in failing.subtracting(drift) {
      problems.append("DRIFT  \(check) — \(detail[check] ?? "")")
    }
    for check in drift.subtracting(failing) {
      problems.append("STALE  \(check) — passes now, remove it from drift.txt")
    }
    problems.sort()

    print(
      "parity: \(specimens.count) specimens, \(built) built, \(failing.count) checks drifting, "
        + "no adapter for: \(unsupported.sorted().joined(separator: " "))"
    )
    for problem in problems { print(problem) }

    XCTAssertTrue(
      problems.isEmpty,
      "\(problems.count) checks are out of step with drift.txt; see the lines above"
    )
  }

  // MARK: - Measuring

  private struct Measured {
    let width: Double
    let height: Double
  }

  /// Host one specimen under a theme and read the size it asks for.
  private func measure(_ probe: Probe, theme: ThemeName, width: Double?) -> Measured {
    let view = build(probe)!.theme(theme)
    // A field has no width of its own; give it the one the story did.
    let sized: AnyView =
      probe.isField
      ? AnyView(view.frame(width: width ?? 240))
      : AnyView(view.fixedSize())
    let host = NSHostingView(rootView: sized)
    host.layoutSubtreeIfNeeded()
    let size = host.fittingSize
    return Measured(width: size.width, height: size.height)
  }

  /// The colours a control paints with: the recipe the theme resolves for its
  /// variant, tint and state. This is what the view reads, so it is what the
  /// browser's computed colour is held to.
  private func recipe(_ probe: Probe, vars: ThemeVariables) -> ControlColors? {
    guard probe.className == "dz-button" || probe.className == "dz-icon-button",
      let variant = ControlVariant(rawValue: probe.data["variant"] ?? ""),
      let tint = Tint(rawValue: probe.data["tint"] ?? "")
    else { return nil }
    return vars.controlColors(
      variant: variant, tint: tint, state: probe.disabled ? .disabled : .normal)
  }

  /// `#rrggbb`, or `#rrggbbaa` when translucent — the reference's spelling.
  private func hex(_ color: Color) -> String {
    guard let rgb = NSColor(color).usingColorSpace(.sRGB) else { return "?" }
    func channel(_ value: CGFloat) -> String {
      String(format: "%02x", Int((value * 255).rounded()))
    }
    let base = "#\(channel(rgb.redComponent))\(channel(rgb.greenComponent))\(channel(rgb.blueComponent))"
    return rgb.alphaComponent >= 1 ? base : base + channel(rgb.alphaComponent)
  }

  /// Equal to the eye: the same channels, with one step of slack on alpha,
  /// which is where `0.12 × 255` rounds differently on two platforms.
  private func sameColour(_ a: String, _ b: String) -> Bool {
    func alpha(_ value: String) -> Int {
      value.count == 9 ? Int(value.suffix(2), radix: 16) ?? 255 : 255
    }
    if alpha(a) == 0 && alpha(b) == 0 { return true }
    return a.prefix(7) == b.prefix(7) && abs(alpha(a) - alpha(b)) <= 1
  }
}

// MARK: - Specimens

/// One distinct thing React drew: a class and its axes, with the first text
/// it was seen with. Text is not part of the identity — a badge reading Beta
/// and one reading Failed are the same specimen.
struct Probe {
  let className: String
  let data: [String: String]
  let text: String

  var key: String {
    let axes = data.keys.sorted().map { "\($0)=\(data[$0]!)" }
    return axes.isEmpty ? className : "\(className) \(axes.joined(separator: " "))"
  }

  var disabled: Bool { data["disabled"] != nil || data["aria-disabled"] == "true" }

  var isField: Bool {
    ["dz-text-field", "dz-search-field", "dz-select", "dz-shortcut-recorder"].contains(className)
  }

  func has(_ axis: String) -> Bool { data[axis] != nil }
}

/// What React measured for a specimen in one theme. A value is nil where the
/// specimen's appearances disagreed, which means it was the story's.
struct Expected {
  var width: Double?
  var height: Double?
  var background: String?
  var color: String?
  var borderColor: String?
}

struct Specimen {
  let key: String
  let probe: Probe
  let themes: [String: Expected]
}

func loadSpecimens(_ dir: URL) throws -> [Specimen] {
  let files = try FileManager.default.contentsOfDirectory(at: dir, includingPropertiesForKeys: nil)
    .filter { $0.pathExtension == "json" }
    .sorted { $0.lastPathComponent < $1.lastPathComponent }

  var probes: [String: Probe] = [:]
  var seen: [String: [String: [[String: Any]]]] = [:]
  var order: [String] = []

  for file in files {
    let page = try JSONSerialization.jsonObject(with: Data(contentsOf: file)) as! [String: Any]
    let stories = page["stories"] as! [String: Any]
    for storyId in stories.keys.sorted() {
      let story = stories[storyId] as! [String: Any]
      let themes = story["themes"] as! [String: Any]
      for themeName in themes.keys.sorted() {
        let drawn = themes[themeName] as! [String: Any]
        for raw in drawn["probes"] as! [[String: Any]] {
          let data = (raw["data"] as! [String: String]).filter {
            !$0.key.hasPrefix("base-ui") && $0.key != "control"
          }
          let probe = Probe(
            className: raw["class"] as! String, data: data, text: raw["text"] as? String ?? "")
          let key = probe.key
          if probes[key] == nil {
            probes[key] = probe
            order.append(key)
          }
          seen[key, default: [:]][themeName, default: []].append(raw)
        }
      }
    }
  }

  return order.map { key in
    Specimen(
      key: key, probe: probes[key]!,
      themes: seen[key]!.mapValues(fold))
  }
}

/// The value every appearance agrees on, or nil.
func fold(_ probes: [[String: Any]]) -> Expected {
  func number(_ path: [String]) -> Double? {
    func read(_ probe: [String: Any]) -> Double? {
      var node: Any = probe
      for step in path {
        guard let dict = node as? [String: Any], let next = dict[step] else { return nil }
        node = next
      }
      return (node as? NSNumber)?.doubleValue
    }
    guard let first = read(probes[0]) else { return nil }
    for probe in probes.dropFirst() {
      guard let other = read(probe), abs(other - first) <= 0.5 else { return nil }
    }
    return first
  }
  func string(_ key: String) -> String? {
    func read(_ probe: [String: Any]) -> String? {
      (probe["style"] as? [String: Any])?[key] as? String
    }
    guard let first = read(probes[0]) else { return nil }
    for probe in probes.dropFirst() where read(probe) != first { return nil }
    return first
  }
  return Expected(
    width: number(["rect", "width"]),
    height: number(["rect", "height"]),
    background: string("backgroundColor"),
    color: string("color"),
    borderColor: string("borderTopColor"))
}

// MARK: - Adapters

/// The view a probe names, or nil where this port cannot build it.
///
/// One clause per React class: the `data-*` axes become the view's
/// arguments, by the names the design system gives them. Only *leaf*
/// components are built: the ones whose look is a function of their own
/// axes. A container takes its size from what it holds, so a probe of one
/// says more about the story than about the component.
func build(_ probe: Probe) -> AnyView? {
  let data = probe.data
  let text = probe.text
  let tint = Tint(rawValue: data["tint"] ?? "")
  let enabled = !probe.disabled

  switch probe.className {
  case "dz-button":
    return AnyView(
      Button(
        size: ControlSize(rawValue: data["size"] ?? "") ?? .small,
        variant: ControlVariant(rawValue: data["variant"] ?? "") ?? .recessed,
        tint: tint ?? .primary,
        fullWidth: data["full-width"] == "true",
        enabled: enabled,
        action: {}
      ) { Text(text) })
  case "dz-icon-button":
    return AnyView(
      IconButton(
        label: "Star",
        size: ControlSize(rawValue: data["size"] ?? "") ?? .tiny,
        variant: ControlVariant(rawValue: data["variant"] ?? "") ?? .plain,
        tint: tint ?? .neutral,
        enabled: enabled,
        action: {}
      ) { Icon(Image(systemName: "star"), size: 16) })
  case "dz-badge":
    return AnyView(
      Badge(
        size: BadgeSize(rawValue: data["size"] ?? "") ?? .medium,
        variant: BadgeVariant(rawValue: data["variant"] ?? "") ?? .tinted,
        tint: tint ?? .primary
      ) { Text(text) })
  case "dz-checkbox":
    return AnyView(
      Checkbox(
        checked: probe.has("checked"),
        size: CheckboxSize(rawValue: data["size"] ?? "") ?? .medium,
        tint: tint ?? .primary,
        enabled: enabled
      ) { Text(text) })
  case "dz-radio":
    return AnyView(
      Radio(
        checked: probe.has("checked"),
        size: RadioSize(rawValue: data["size"] ?? "") ?? .medium,
        tint: tint ?? .primary,
        enabled: enabled
      ) { Text(text) })
  case "dz-switch":
    return AnyView(
      Switch(
        checked: probe.has("checked"),
        size: SwitchSize(rawValue: data["size"] ?? "") ?? .medium,
        tint: tint ?? .primary,
        enabled: enabled))
  case "dz-key-cap":
    return AnyView(
      KeyCap(
        size: KeyCapSize(rawValue: data["size"] ?? "") ?? .medium,
        variant: KeyCapVariant(rawValue: data["variant"] ?? "") ?? .hint
      ) { Text(text) })
  case "dz-card":
    return AnyView(
      Card(
        size: CardSize(rawValue: data["size"] ?? "") ?? .medium,
        variant: CardVariant(rawValue: data["variant"] ?? "") ?? .raised,
        tint: tint ?? .primary
      ) { Text(text) })
  case "dz-callout":
    return AnyView(
      Callout(
        size: CalloutSize(rawValue: data["size"] ?? "") ?? .medium,
        tint: tint ?? .neutral,
        leading: { EmptyView() },
        description: { Text(text) },
        trailing: { EmptyView() }))
  case "dz-progress":
    return AnyView(
      Progress(
        value: probe.has("indeterminate") ? nil : 0.35,
        size: ProgressSize(rawValue: data["size"] ?? "") ?? .small,
        tint: tint ?? .primary))
  case "dz-spinner":
    return AnyView(
      Spinner(
        size: SpinnerSize(rawValue: data["size"] ?? "") ?? .medium,
        tint: tint ?? .primary))
  case "dz-section-label":
    return AnyView(SectionLabel(tint: tint) { Text(text) })
  case "dz-toast":
    return AnyView(
      Toast(
        tint: tint ?? .neutral,
        leading: { EmptyView() },
        message: { Text(text) },
        trailing: { EmptyView() }))
  case "dz-nav-item":
    return AnyView(
      NavItem(
        size: NavItemSize(rawValue: data["size"] ?? "") ?? .medium,
        tint: tint ?? .primary,
        enabled: enabled,
        action: {},
        icon: { EmptyView() },
        label: { Text(text) }))
  case "dz-option-card":
    return AnyView(
      OptionCard(
        tint: tint ?? .primary,
        enabled: enabled,
        action: {},
        title: { Text(text) },
        description: { EmptyView() }))
  case "dz-search-field":
    return AnyView(
      SearchField(
        value: "",
        onValueChange: { _ in },
        size: TextFieldSize(rawValue: data["size"] ?? "") ?? .medium,
        tint: tint ?? .primary
      ) { EmptyView() })
  case "dz-text-field":
    return AnyView(
      TextField(
        text: .constant(""),
        placeholder: "Placeholder",
        size: TextFieldSize(rawValue: data["size"] ?? "") ?? .medium,
        state: probe.has("invalid") ? .error : .default,
        tint: tint ?? .primary,
        mono: data["mono"] == "true",
        enabled: enabled,
        multiline: probe.has("multiline")))
  case "dz-shortcut-recorder":
    return AnyView(
      ShortcutRecorder(
        value: "⌘K",
        onValueChange: { _ in },
        disabled: !enabled,
        tint: tint ?? .primary))
  case "dz-divider":
    let divider = Divider(
      tone: DividerTone(rawValue: data["tone"] ?? "") ?? .default,
      orientation: DividerOrientation(rawValue: data["orientation"] ?? "") ?? .horizontal)
    return data["orientation"] == "vertical"
      ? AnyView(divider.frame(height: 24))
      : AnyView(divider.frame(width: 240))
  case "dz-segmented-control":
    return AnyView(
      SegmentedControl(
        items: [
          SegmentedItem(value: 0, label: "Design"),
          SegmentedItem(value: 1, label: "Code"),
        ],
        value: 0,
        size: SegmentedControlSize(rawValue: data["size"] ?? "") ?? .medium,
        tint: tint ?? .primary,
        activeStyle: SegmentedActiveStyle(rawValue: data["active-style"] ?? "") ?? .raised,
        stretch: data["stretch"] == "true"))
  case "dz-pill-tabs":
    return AnyView(
      PillTabs(
        items: [
          PillTabItem(value: 0, label: "All"),
          PillTabItem(value: 1, label: "Open"),
        ],
        value: 0,
        tint: tint ?? .primary))
  default:
    return nil
  }
}
