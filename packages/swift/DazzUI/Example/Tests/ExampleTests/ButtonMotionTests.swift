import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

/// A state change crossfades a button's surface instead of swapping it.
///
/// `button.css` transitions `background-color`, `border-color` and `color`
/// over the motion token. A port that redraws the button as a new view
/// instead has no colour to interpolate between: the layer is snapshotted and
/// faded, which is what macOS 15 draws as the fill being wiped away from one
/// edge — and the button did exactly that whenever `enabled` dropped its
/// lift, because `tokenShadow` builds one `AnyView` per shadow layer.
@MainActor
final class ButtonMotionTests: XCTestCase {
  /// The enabled flag, flipped from the test — the same change a host makes
  /// when the field a control guards empties or fills.
  final class Toggle: ObservableObject {
    @Published var enabled = true
  }

  private struct Probe: View {
    @ObservedObject var toggle: Toggle

    var body: some View {
      DazzUI.Button(
        "Translate the paragraph",
        variant: .filled,
        shortcut: "⌘\u{2009}⏎",
        enabled: toggle.enabled,
        action: {}
      )
      .theme(.studioLight)
      .frame(width: 400, height: 60, alignment: .topLeading)
    }
  }

  struct Reading {
    let width: CGFloat
    let surface: NSColor
  }

  func testAFlatteningButtonCrossfadesItsSurface() throws {
    let toggle = Toggle()
    let host = NSHostingView(rootView: Probe(toggle: toggle))
    host.frame = NSRect(x: 0, y: 0, width: 400, height: 60)
    let window = NSWindow(
      contentRect: host.frame, styleMask: .borderless, backing: .buffered, defer: false)
    window.contentView = host
    window.orderFrontRegardless()
    defer { window.orderOut(nil) }
    host.layoutSubtreeIfNeeded()
    RunLoop.main.run(until: Date().addingTimeInterval(0.25))

    let enabled = try read(host)

    toggle.enabled = false
    var frames: [Reading] = []
    for _ in 0..<10 {
      RunLoop.main.run(until: Date().addingTimeInterval(0.015))
      frames.append(try read(host))
    }
    let disabled = try XCTUnwrap(frames.last)

    // The label flattens with the control, so the box does narrow — a probe
    // that measured nothing would pass whatever the component did.
    XCTAssertGreaterThan(
      enabled.width, disabled.width,
      "a flattened filled button is a narrower button")
    XCTAssertLessThan(
      enabled.surface.redComponent, disabled.surface.redComponent,
      "a flattened filled surface is a paler surface")

    // The surface crossfades: some frame in between is a colour of its own,
    // neither the enabled surface nor the disabled one. A surface that was
    // replaced rather than interpolated jumps straight from one to the other.
    let blended = frames.contains { frame in
      let value = frame.surface.redComponent
      return min(enabled.surface.redComponent, disabled.surface.redComponent) + 0.02 < value
        && value < max(enabled.surface.redComponent, disabled.surface.redComponent) - 0.02
    }
    XCTAssertTrue(
      blended,
      "the surface was swapped instead of crossfading: "
        + "\(enabled.surface.redComponent) → \(disabled.surface.redComponent)")
  }

  /// The box the button's surface fills, and that surface's colour, read off
  /// the frame the way the conformance collector reads one.
  private func read(_ host: NSView) throws -> Reading {
    let rep = try XCTUnwrap(host.bitmapImageRepForCachingDisplay(in: host.bounds))
    host.cacheDisplay(in: host.bounds, to: rep)
    var right = -1
    for x in 0..<rep.pixelsWide {
      for y in 0..<rep.pixelsHigh where (rep.colorAt(x: x, y: y)?.alphaComponent ?? 0) > 0.5 {
        right = max(right, x)
        break
      }
    }
    XCTAssertGreaterThan(right, 0, "nothing was drawn")
    // A point just under the top edge, clear of the stroke and the label.
    let surface = try XCTUnwrap(
      rep.colorAt(x: right / 2, y: 8)?.usingColorSpace(.deviceRGB), "no surface at the top edge")
    return Reading(width: CGFloat(right + 1), surface: surface)
  }
}
