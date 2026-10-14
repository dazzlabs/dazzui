import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

/// Inspect rendered corners: a token alias alone cannot prove that a view
/// used it, or that an oversized radius stayed circular when clipped.
@MainActor
final class CornerRenderingTests: XCTestCase {
  func testButtonCornersInEveryThemeAndSize() throws {
    for name in ThemeName.allCases {
      for size in DazzUI.ControlSize.allCases {
        let metrics = name.variables.metrics(size)
        let width: CGFloat = 120
        let height = metrics.size
        let root = DazzUI.Button(size: size, variant: .filled, fullWidth: true, action: {}) {
          SwiftUI.Text("")
        }
        .theme(name)
        .frame(width: width)
        let host = NSHostingView(rootView: root)
        host.frame = NSRect(x: 0, y: 0, width: width, height: height)
        let window = NSWindow(contentRect: host.frame, styleMask: .borderless, backing: .buffered, defer: false)
        window.contentView = host
        host.layoutSubtreeIfNeeded()
        let bitmap = try XCTUnwrap(NSBitmapImageRep(
          bitmapDataPlanes: nil, pixelsWide: Int(width * 2), pixelsHigh: Int(height * 2),
          bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true, isPlanar: false,
          colorSpaceName: .calibratedRGB, bytesPerRow: 0, bitsPerPixel: 0
        ))
        bitmap.size = host.bounds.size
        host.cacheDisplay(in: host.bounds, to: bitmap)
        let fill = try XCTUnwrap(bitmap.colorAt(x: Int(width), y: Int(height))?.usingColorSpace(.deviceRGB))
        func filled(_ x: Int, _ y: Int) -> Bool {
          guard let color = bitmap.colorAt(x: x, y: y)?.usingColorSpace(.deviceRGB) else { return false }
          return color.alphaComponent > 0.8
            && abs(color.redComponent - fill.redComponent) < 0.04
            && abs(color.greenComponent - fill.greenComponent) < 0.04
            && abs(color.blueComponent - fill.blueComponent) < 0.04
        }
        let radius = min(metrics.radius, min(width, height) / 2)
        let label = "\(name.rawValue) \(size)"
        XCTAssertTrue(filled(Int(width), Int(height)), label)
        if radius == 0 {
          XCTAssertTrue(filled(1, 1), label)
        } else {
          for x in 0..<Int(ceil(radius * 2)) {
            for y in 0..<Int(ceil(radius * 2)) {
              let distance = hypot(radius - (CGFloat(x) + 0.5) / 2, radius - (CGFloat(y) + 0.5) / 2)
              // Exclude the antialiased edge; compare the shape on all four corners.
              guard abs(distance - radius) > 0.75 else { continue }
              let expected = distance < radius
              for (px, py) in [
                (x, y), (bitmap.pixelsWide - x - 1, y),
                (x, bitmap.pixelsHigh - y - 1),
                (bitmap.pixelsWide - x - 1, bitmap.pixelsHigh - y - 1),
              ] {
                XCTAssertEqual(filled(px, py), expected, "\(label) corner at \(px),\(py)")
              }
            }
          }
        }
        window.contentView = nil
      }
    }
  }
}
