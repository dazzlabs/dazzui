import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

/// The trailing column collapses the way the leading one does, mirrored: a
/// panel put away takes no width and the document beside it has the window.
@MainActor
final class AsideCollapseTests: XCTestCase {
  private let width: CGFloat = 300
  private let height: CGFloat = 100

  /// The window at 2x: a red document, and the panel at its trailing edge.
  private func window(collapsed: Bool) throws -> NSBitmapImageRep {
    let root = HStack(spacing: 0) {
      Rectangle()
        .fill(Color.red)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
      Aside(width: 100, collapsed: collapsed) {
        Color.clear.frame(maxWidth: .infinity, maxHeight: .infinity)
      }
    }
    .theme(.studioLight)
    .frame(width: width, height: height)

    let host = NSHostingView(rootView: root)
    host.frame = NSRect(x: 0, y: 0, width: width, height: height)
    let panel = NSWindow(
      contentRect: host.frame, styleMask: .borderless, backing: .buffered, defer: false)
    panel.contentView = host
    host.layoutSubtreeIfNeeded()
    let bitmap = try XCTUnwrap(
      NSBitmapImageRep(
        bitmapDataPlanes: nil,
        pixelsWide: Int(width * 2),
        pixelsHigh: Int(height * 2),
        bitsPerSample: 8,
        samplesPerPixel: 4,
        hasAlpha: true,
        isPlanar: false,
        colorSpaceName: .calibratedRGB,
        bytesPerRow: 0,
        bitsPerPixel: 0
      ))
    bitmap.size = host.bounds.size
    host.cacheDisplay(in: host.bounds, to: bitmap)
    return bitmap
  }

  /// Red is the document's own colour and nothing the panel is drawn in, so a
  /// trailing pixel that is red is a window the panel has given up.
  private func isDocument(_ bitmap: NSBitmapImageRep) throws -> Bool {
    let color = try XCTUnwrap(
      bitmap.colorAt(x: bitmap.pixelsWide - 4, y: bitmap.pixelsHigh / 2)?
        .usingColorSpace(.deviceRGB))
    return color.redComponent > 0.9 && color.greenComponent < 0.35
      && color.blueComponent < 0.35
  }

  func testAnOpenPanelTakesItsWidth() throws {
    let bitmap = try window(collapsed: false)
    XCTAssertFalse(try isDocument(bitmap), "the panel's own surface at the edge")
  }

  func testAPanelPutAwayTakesNone() throws {
    let bitmap = try window(collapsed: true)
    XCTAssertTrue(try isDocument(bitmap), "the document runs to the window's edge")
  }
}
