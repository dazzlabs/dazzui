import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

/// A column put away takes no width, and the pane beside it has the window:
/// the collapse is a width the column travels to rather than a view that
/// vanishes and leaves a gap where it stood. Its content is clipped rather
/// than laid out again, which is what keeps a row from rewriting itself on the
/// way out.
@MainActor
final class SidebarCollapseTests: XCTestCase {
  private let width: CGFloat = 300
  private let height: CGFloat = 100

  /// The window at 2x: a column, and a red pane taking what is left of it.
  private func window(collapsed: Bool) throws -> NSBitmapImageRep {
    let root = HStack(spacing: 0) {
      Sidebar(collapsed: collapsed) {
        EmptyView()
      }
      Rectangle()
        .fill(Color.red)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
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

  /// Red is the pane's own colour and nothing the column is drawn in, so a
  /// leading pixel that is red is a window the column has given up.
  private func isPane(_ bitmap: NSBitmapImageRep, atX x: Int) throws -> Bool {
    let color = try XCTUnwrap(
      bitmap.colorAt(x: x, y: bitmap.pixelsHigh / 2)?.usingColorSpace(.deviceRGB))
    // System red is not 1,0,0 in the display's colour space; what makes this
    // the pane is the green and blue channels being far below the column's.
    return color.redComponent > 0.9 && color.greenComponent < 0.35
      && color.blueComponent < 0.35
  }

  func testAnOpenColumnTakesItsWidth() throws {
    let bitmap = try window(collapsed: false)
    XCTAssertFalse(try isPane(bitmap, atX: 2), "the column's own surface")
    XCTAssertTrue(try isPane(bitmap, atX: bitmap.pixelsWide - 4), "the pane beside it")
  }

  func testAColumnPutAwayTakesNone() throws {
    let bitmap = try window(collapsed: true)
    // The pane starts at the window's leading edge, not a column's width in.
    XCTAssertTrue(try isPane(bitmap, atX: 2))
    XCTAssertTrue(try isPane(bitmap, atX: bitmap.pixelsWide - 4))
  }
}
