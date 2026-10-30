import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

/// A tooltip is a remark about a control, not part of what the control lays
/// out: it has to be whole even where the layout clips, and on top even where
/// a sibling paints over the control.
@MainActor
final class TooltipPresentationTests: XCTestCase {
  /// A label whose control sits in a box that clips, the shape a tooltip in a
  /// sidebar or a scrolling pane has.
  private struct ClippedBox: View {
    var presented: Bool

    var body: some View {
      VStack(spacing: 0) {
        ZStack(alignment: .topLeading) {
          Color.white
          Tooltip(
            "A label that is deliberately longer than the box it hangs off",
            side: .top,
            delay: 0,
            presented: presented
          ) {
            Color.blue.frame(width: 40, height: 20)
          }
        }
        .frame(width: 120, height: 40)
        .clipped()
        Spacer(minLength: 0)
      }
      .frame(width: 400, height: 300)
      .background(Color.gray)
    }
  }

  /// A control that already owns a hint, wrapped in another one.
  private struct Nested: View {
    var body: some View {
      Tooltip(
        "An outer label that is deliberately much wider than the inner one",
        delay: 0,
        presented: true
      ) {
        Tooltip("In", delay: 0, presented: true) {
          Color.blue.frame(width: 40, height: 20)
        }
      }
      .frame(width: 400, height: 300)
    }
  }

  private func host(
    _ view: some View
  ) throws -> (NSWindow, NSHostingView<some View>) {
    let host = NSHostingView(rootView: view)
    host.frame = NSRect(x: 0, y: 0, width: 400, height: 300)
    let window = NSWindow(
      contentRect: host.frame, styleMask: .borderless, backing: .buffered,
      defer: false)
    window.contentView = host
    host.layoutSubtreeIfNeeded()
    return (window, host)
  }

  /// The label's window: it hangs off the window the control lives in, so it
  /// is above that window's content and outside anything that clips it.
  private func labelPanels(of window: NSWindow) -> [NSPanel] {
    (window.childWindows ?? [])
      .compactMap { $0 as? NSPanel }
      .filter { $0.contentView is NSHostingView<AnyView> && $0.isVisible }
  }

  /// Runs the run loop until the label is on screen, or gives up.
  private func settle(
    _ window: NSWindow, for seconds: TimeInterval = 1.5
  ) -> [NSPanel] {
    let deadline = Date().addingTimeInterval(seconds)
    while Date() < deadline {
      let panels = labelPanels(of: window)
      if !panels.isEmpty { return panels }
      RunLoop.current.run(
        mode: .default, before: Date().addingTimeInterval(0.02))
    }
    return labelPanels(of: window)
  }

  func testTheLabelIsPresentedAboveTheWindow() throws {
    let (window, _) = try host(ClippedBox(presented: true))
    let panels = settle(window)

    XCTAssertEqual(panels.count, 1, "one label, in a window of its own")
    let panel = try XCTUnwrap(panels.first)
    XCTAssertTrue(
      window.childWindows?.contains(panel) == true,
      "the label rides with the window it belongs to")
    // The box is 120 points wide and clips; the label is wider than that and
    // is not cut by it.
    let inset = Theme(.studioLight).vars.spacing3
    let label = panel.frame.insetBy(dx: inset, dy: inset)
    XCTAssertGreaterThan(label.width, 120, "the label is not cut to the box")
    XCTAssertGreaterThan(label.height, 0)
  }

  func testTheLabelStaysInsideTheWindow() throws {
    let (window, _) = try host(ClippedBox(presented: true))
    let panel = try XCTUnwrap(settle(window).first)
    let inset = Theme(.studioLight).vars.spacing3
    let label = panel.frame.insetBy(dx: inset, dy: inset)
    XCTAssertTrue(
      window.frame.contains(label),
      "a label beside a control at the window's edge is flipped, not cut")
  }

  func testAnOuterLabelYieldsToTheOneInsideIt() throws {
    let (window, _) = try host(Nested())
    RunLoop.current.run(until: Date().addingTimeInterval(0.5))
    let panels = labelPanels(of: window)
    XCTAssertEqual(panels.count, 1, "one label, not two for one control")
    let inset = Theme(.studioLight).vars.spacing3
    let label = try XCTUnwrap(panels.first).frame.insetBy(dx: inset, dy: inset)
    XCTAssertLessThan(label.width, 60, "the label the control owns, not the outer one")
  }

  func testTheLabelIsNotPresentedWhileTheControlRests() throws {
    let (window, _) = try host(ClippedBox(presented: false))
    RunLoop.current.run(until: Date().addingTimeInterval(0.2))
    XCTAssertTrue(labelPanels(of: window).isEmpty)
  }
}
