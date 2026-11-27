import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

@MainActor
final class TreeTabsPerformanceTests: XCTestCase {
  func testTabFitMatchesTheWidthsOfTheTabsActuallyShown() {
    let whole: [CGFloat] = [70, 103.25, 90, 159.5]
    let small: [CGFloat] = [20, 32, 60.5, 38]
    var widths: [TabsMeasureSlot: CGFloat] = [.trailing: 12]
    for index in whole.indices {
      widths[.whole(index)] = whole[index]
      widths[.compact(index)] = small[index]
    }
    let slots = whole.indices.map { TabsRulerPlan.Slots(whole: .whole($0), compact: .compact($0)) }
    for active in [nil, 0, 1, 2, 3] as [Int?] {
      for room in stride(from: CGFloat(20), through: 480, by: 0.25) {
        widths[.room] = room
        let fit = TabsFit.resolve(slots, active: active, widths: widths, moreWidth: 28)
        let basis = whole.reduce(12, +)
        XCTAssertEqual(fit.basis, basis)
        XCTAssertEqual(fit.floor, min(basis, whole[active ?? 0] + 28 + 12))
        XCTAssertEqual(fit.compact, basis > room + 0.5)

        // Exhaustively lay out each possible prefix, including the full-width
        // active tab that replaces the final compact tab when it is promoted.
        var expected = whole.count
        if fit.compact {
          expected = 1
          for count in 1...whole.count {
            var indices = Array(whole.indices.prefix(count))
            if let active, active >= count { indices[count - 1] = active }
            let drawn = indices.reduce(CGFloat(0)) { $0 + ($1 == active ? whole[$1] : small[$1]) }
            let more: CGFloat = count == whole.count ? 0 : 28
            if drawn + more + 12 <= room + 0.5 { expected = count }
          }
        }
        XCTAssertEqual(fit.shown, expected, "active=\(String(describing: active)), room=\(room)")
      }
    }
  }

  func testTabFitWaitsForCompleteMeasurements() {
    let widths: [TabsMeasureSlot: CGFloat] = [.room: 50, .whole(0): 72, .compact(0): 20]
    let slots = (0..<2).map { TabsRulerPlan.Slots(whole: .whole($0), compact: .compact($0)) }
    XCTAssertEqual(
      TabsFit.resolve([], active: nil, widths: [:], moreWidth: 28),
      TabsFit(compact: false, shown: 0, basis: 0, floor: 0))
    XCTAssertEqual(
      TabsFit.resolve(slots, active: 0, widths: widths, moreWidth: 28),
      TabsFit(compact: false, shown: 2, basis: 0, floor: 0))
    XCTAssertEqual(
      TabsFit.resolve([slots[0]], active: 0, widths: widths, moreWidth: 28),
      TabsFit(compact: true, shown: 1, basis: 72, floor: 72))
  }

  func testRulerMeasuresSharedChromeOnceAndKeepsDifferentTextAndBadges() {
    var items = (0..<256).map { TabItem(value: $0, label: "Document \($0)", icon: .document20Regular) }
    let plain = TabsRulerPlan(items: items, active: 255, canClose: false)
    XCTAssertEqual(plain.probes.count, 257)
    XCTAssertEqual(plain.liveProbes.count, 3)
    XCTAssertEqual(Set(plain.slots.map(\.compact)).count, 1)
    items[0].icon = nil
    items[1].detail = "detail"
    items[2].count = 1
    items[3].status = .success
    items[4].closable = true
    let varied = TabsRulerPlan(items: items, active: 255, canClose: true)
    XCTAssertEqual(varied.slots[0].whole, varied.slots[0].compact)
    XCTAssertEqual(varied.slots[1].compact, varied.slots[4].compact)
    XCTAssertNotEqual(varied.slots[2].compact, varied.slots[3].compact)
  }

  private final class TabDocument: ObservableObject {
    @Published var changedLabel = false
    @Published var theme = Theme(.studioLight)
  }

  private struct MeasuredDocumentTabs: View {
    @ObservedObject var document: TabDocument

    var body: some View {
      let items = (0..<48).map { index in
        let label = index == 20 && document.changedLabel
          ? String(repeating: "W", count: 40) : "Document \(index)"
        return TabItem(value: index, label: label, icon: .document20Regular)
      }
      DazzUI.Tabs(items: items, value: 47) { _ in }
        .fixedSize(horizontal: true, vertical: false)
        .background { TabBounds().allowsHitTesting(false) }
        .theme(document.theme)
        .frame(maxWidth: .infinity, alignment: .leading)
    }
  }

  func testLargeTabsRemeasureAfterDocumentAndFontChanges() throws {
    let document = TabDocument()
    let hosting = NSHostingView(rootView: MeasuredDocumentTabs(document: document))
    let rect = NSRect(x: 0, y: 0, width: 16000, height: 40)
    let window = NSWindow(contentRect: rect, styleMask: [.borderless],
                          backing: .buffered, defer: false)
    window.isReleasedWhenClosed = false
    hosting.sizingOptions = []
    hosting.frame = rect
    window.contentView = hosting
    defer { window.close() }
    func width() throws -> CGFloat {
      hosting.layoutSubtreeIfNeeded()
      RunLoop.main.run(until: Date().addingTimeInterval(0.12))
      hosting.layoutSubtreeIfNeeded()
      func marker(_ view: NSView) -> NSView? {
        if view.identifier?.rawValue == "tabs" { return view }
        return view.subviews.lazy.compactMap(marker).first
      }
      return try XCTUnwrap(marker(hosting)).bounds.width
    }
    let original = try width()
    XCTAssertGreaterThan(original, 1000)
    document.changedLabel = true
    let renamed = try width()
    XCTAssertGreaterThan(renamed, original + 100)
    var vars = document.theme.vars
    vars.labelMedium = vars.labelMedium.scaled(by: 1.5)
    document.theme = Theme(vars: vars, colorScheme: .light)
    XCTAssertGreaterThan(try width(), renamed * 1.15)
    document.changedLabel = false
    document.theme = Theme(.studioLight)
    XCTAssertEqual(try width(), original, accuracy: 1)
  }

  private final class OverflowSelection: ObservableObject {
    @Published var value = 35
    @Published var presentation = MenuPresentation.native
    var presses: [Int] = []
  }

  private struct OverflowTabs: View {
    @ObservedObject var selection: OverflowSelection
    let items = (0..<36).map { TabItem(value: $0, label: "Document \($0)", icon: .document20Regular) }

    var body: some View {
      DazzUI.Tabs(items: items, value: selection.value) { id in
        selection.value = id
        selection.presses.append(id)
      }
      .frame(height: 32)
      .menuPresentation(selection.presentation)
      .theme(.studioLight)
    }
  }

  func testSelectingAnOverflowDocumentPromotesItsButton() throws {
    let selection = OverflowSelection()
    let rect = NSRect(x: 100, y: 200, width: 820, height: 80)
    let window = NSPanel(contentRect: rect, styleMask: [.borderless, .nonactivatingPanel],
                         backing: .buffered, defer: false)
    window.isReleasedWhenClosed = false
    window.hidesOnDeactivate = false
    window.ignoresMouseEvents = true
    let host = NSHostingView(rootView: OverflowTabs(selection: selection))
    host.sizingOptions = []
    host.frame = NSRect(origin: .zero, size: rect.size)
    window.contentView = host
    window.orderBack(nil)
    defer {
      window.orderOut(nil)
      window.close()
    }
    func settle() {
      host.layoutSubtreeIfNeeded()
      RunLoop.main.run(until: Date().addingTimeInterval(0.12))
    }
    func anchor(_ view: NSView) -> NativeMenuAnchorView? {
      if let view = view as? NativeMenuAnchorView { return view }
      return view.subviews.lazy.compactMap(anchor).first
    }
    func click(_ point: NSPoint, in target: NSWindow) throws {
      for type in [NSEvent.EventType.leftMouseDown, .leftMouseUp] {
        let event = try XCTUnwrap(NSEvent.mouseEvent(
          with: type, location: point, modifierFlags: [], timestamp: ProcessInfo.processInfo.systemUptime,
          windowNumber: target.windowNumber, context: nil, eventNumber: 1, clickCount: 1,
          pressure: type == .leftMouseDown ? 1 : 0))
        target.sendEvent(event)
      }
      settle()
    }
    settle()
    // Both presentations share the same trigger. Its native anchor gives
    // the exact hit point without involving the mouse or the active app.
    let more = try XCTUnwrap(anchor(host))
    let center = more.convert(NSPoint(x: more.bounds.midX, y: more.bounds.midY), to: nil)
    selection.presentation = .custom
    settle()
    try click(center, in: window)
    let menu = try XCTUnwrap(window.childWindows?.first)
    defer { menu.orderOut(nil) }
    menu.ignoresMouseEvents = true
    let content = try XCTUnwrap(menu.contentView)
    try click(content.convert(NSPoint(x: content.bounds.midX, y: content.bounds.midY), to: nil), in: menu)
    let chosen = try XCTUnwrap(selection.presses.first)
    XCTAssertNotEqual(chosen, 35)
    XCTAssertEqual(selection.presses.count, 1)
    XCTAssertFalse(window.childWindows?.contains(menu) == true)
    // The selection takes the complete final tab location just before More.
    selection.presentation = .native
    settle()
    let newAnchor = try XCTUnwrap(anchor(host))
    let edge = newAnchor.convert(newAnchor.bounds, to: nil)
    try click(NSPoint(x: edge.minX - 30, y: edge.midY), in: window)
    XCTAssertEqual(selection.presses, [chosen, chosen])
  }

  private final class TreeInput: ObservableObject {
    @Published var expanded: [String] = []
    @Published var target: String?
    var built = Set<String>()
  }

  private struct LabelProbe: View {
    let id: String
    let input: TreeInput

    var body: some View {
      let _ = input.built.insert(id)
      Text(id)
    }
  }

  private struct ScrollingTree: View {
    @ObservedObject var input: TreeInput
    let items: [TreeNode]

    var body: some View {
      ScrollViewReader { proxy in
        ScrollView {
          DazzUI.Tree(items: items, expanded: input.expanded)
        }
        .onChange(of: input.target) { target in
          if let target { proxy.scrollTo(target) }
        }
      }
      .theme(.studioLight)
    }
  }

  func testTreeScrollsToExpandedDescendantInParentViewport() {
    let input = TreeInput()
    let children = (0..<100).map { index in
      let id = "child-\(index)"
      return TreeNode(id: id, label: LabelProbe(id: id, input: input), text: id)
    }
    let folder = TreeNode(
      id: "folder", label: LabelProbe(id: "folder", input: input), children: children)
    let hosting = NSHostingView(rootView: ScrollingTree(input: input, items: [folder]))
    let window = NSWindow(contentRect: NSRect(x: 0, y: 0, width: 300, height: 140),
                          styleMask: [.titled], backing: .buffered, defer: false)
    window.isReleasedWhenClosed = false
    hosting.sizingOptions = []
    hosting.frame = NSRect(x: 0, y: 0, width: 300, height: 140)
    window.contentView = hosting
    defer { window.close() }
    func settle() {
      hosting.layoutSubtreeIfNeeded()
      RunLoop.main.run(until: Date().addingTimeInterval(0.1))
    }
    settle()
    XCTAssertEqual(input.built, ["folder"])
    input.expanded = ["folder"]
    settle()
    XCTAssertTrue(input.built.contains("child-0"))
    XCTAssertFalse(input.built.contains("child-99"))
    input.target = "child-99"
    settle()
    XCTAssertTrue(input.built.contains("child-99"))
  }


  private final class Selection: ObservableObject {
    @Published var reversed = false
    @Published var tab = "Alpha tab"
    @Published var file: String?
    var tabPresses: [String] = []
    var filePresses: [String] = []
  }

  // A real, focusable AppKit control is allowed in a tree's rich label slot.
  private final class FocusableTitle: NSTextField {
    override var acceptsFirstResponder: Bool { true }
  }

  private struct FileTitle: NSViewRepresentable {
    let name: String

    func makeNSView(context: Context) -> FocusableTitle {
      let field = FocusableTitle(labelWithString: name)
      field.identifier = NSUserInterfaceItemIdentifier(name)
      return field
    }

    func updateNSView(_ field: FocusableTitle, context: Context) { field.stringValue = name }
  }

  private struct TabBounds: NSViewRepresentable {
    func makeNSView(context: Context) -> NSView {
      let view = NSView()
      view.identifier = NSUserInterfaceItemIdentifier("tabs")
      return view
    }
    func updateNSView(_ view: NSView, context: Context) {}
  }

  private struct ReorderedControls: View {
    @ObservedObject var state: Selection

    var body: some View {
      let tabs = state.reversed ? ["Beta tab", "Alpha tab"] : ["Alpha tab", "Beta tab"]
      let files = state.reversed ? ["Second file", "First file"] : ["First file", "Second file"]
      VStack {
        DazzUI.Tabs(items: tabs.map { TabItem(value: $0, label: $0) }, value: state.tab) { id in
          state.tab = id
          state.tabPresses.append(id)
        }
        .background { TabBounds().allowsHitTesting(false) }
        .frame(height: 36)
        ScrollView {
          DazzUI.Tree(items: files.map { TreeNode(id: $0, label: FileTitle(name: $0), text: $0) }, selected: state.file,
                      onSelect: { id in
                        state.file = id
                        state.filePresses.append(id)
                      })
        }
      }
      .theme(.studioLight)
    }
  }

  func testSelectionAndFocusedTreeRowFollowStableIDsAfterReordering() throws {
    let state = Selection()
    let hosting = NSHostingView(rootView: ReorderedControls(state: state))
    let rect = NSRect(x: 0, y: 0, width: 400, height: 240)
    let window = NSPanel(contentRect: rect, styleMask: [.borderless, .nonactivatingPanel],
                         backing: .buffered, defer: false)
    window.isReleasedWhenClosed = false
    window.hidesOnDeactivate = false
    window.ignoresMouseEvents = true
    hosting.sizingOptions = []
    hosting.frame = rect
    window.contentView = hosting
    window.orderBack(nil)
    defer {
      window.orderOut(nil)
      window.contentView = nil
      window.close()
    }
    func settle() {
      hosting.layoutSubtreeIfNeeded()
      RunLoop.main.run(until: Date().addingTimeInterval(0.05))
    }
    func named(_ name: String, under view: NSView) -> NSView? {
      if view.identifier?.rawValue == name { return view }
      for child in view.subviews {
        if let match = named(name, under: child) { return match }
      }
      return nil
    }
    func control(_ name: String) throws -> NSView {
      try XCTUnwrap(named(name, under: hosting), "Missing hosted view: \(name)")
    }
    func click(_ point: NSPoint) throws {
      for type in [NSEvent.EventType.leftMouseDown, .leftMouseUp] {
        let event = try XCTUnwrap(NSEvent.mouseEvent(
          with: type, location: point, modifierFlags: [], timestamp: ProcessInfo.processInfo.systemUptime,
          windowNumber: window.windowNumber, context: nil, eventNumber: 1, clickCount: 1,
          pressure: type == .leftMouseDown ? 1 : 0))
        window.sendEvent(event)
      }
      settle()
    }
    func clickSecondTab() throws {
      let tabs = try control("tabs")
      let frame = tabs.convert(tabs.bounds, to: nil)
      try click(NSPoint(x: frame.minX + frame.width * 0.75, y: frame.midY))
    }
    func clickFile(_ name: String) throws {
      let row = try control(name)
      let frame = row.convert(row.bounds, to: nil)
      // Use the row's leading padding, outside the native label.
      try click(NSPoint(x: 4, y: frame.midY))
    }

    settle()
    try clickFile("Second file")
    XCTAssertEqual(state.filePresses, ["Second file"])
    try clickSecondTab()
    XCTAssertEqual(state.tabPresses, ["Beta tab"])
    let focused = try control("Second file")
    XCTAssertTrue(window.makeFirstResponder(focused))
    let originalY = focused.convert(focused.bounds, to: nil).midY
    state.reversed = true
    settle()
    XCTAssertEqual(state.tab, "Beta tab")
    XCTAssertEqual(state.file, "Second file")
    XCTAssertTrue(try control("Second file") === focused)
    XCTAssertTrue(window.firstResponder === focused)
    XCTAssertGreaterThan(focused.convert(focused.bounds, to: nil).midY, originalY)
    try clickFile("First file")
    try clickSecondTab()
    XCTAssertEqual(state.filePresses, ["Second file", "First file"])
    XCTAssertEqual(state.tabPresses, ["Beta tab", "Alpha tab"])
  }
}
