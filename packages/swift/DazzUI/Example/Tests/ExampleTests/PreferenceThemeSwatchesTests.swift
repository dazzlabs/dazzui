import AppKit
import DazzUI
import SwiftUI
import XCTest

@testable import Example

@MainActor
final class PreferenceThemeSwatchesTests: XCTestCase {
  private let options = [
    PreferenceThemeOption(value: "paper", label: "Studio", theme: .studioLight),
    PreferenceThemeOption(value: "night", label: "Nocturne", theme: .nocturneDark),
  ]

  private func click(_ point: NSPoint, in window: NSWindow) {
    for type in [NSEvent.EventType.leftMouseDown, .leftMouseUp] {
      let event = NSEvent.mouseEvent(with: type, location: point, modifierFlags: [],
        timestamp: ProcessInfo.processInfo.systemUptime, windowNumber: window.windowNumber,
        context: nil, eventNumber: 0, clickCount: 1, pressure: 1)!
      NSApp.sendEvent(event)
    }
    RunLoop.current.run(until: Date().addingTimeInterval(0.05))
  }

  private func bitmap(_ host: NSView) throws -> NSBitmapImageRep {
    host.layoutSubtreeIfNeeded()
    let bitmap = try XCTUnwrap(host.bitmapImageRepForCachingDisplay(in: host.bounds))
    host.cacheDisplay(in: host.bounds, to: bitmap)
    return bitmap
  }

  func testPalettesSelectionAndPageThemeChanges() throws {
    var changed: String?
    let control = PreferenceThemeSwatches(options: options, defaultValue: "paper",
      onValueChange: { changed = $0 })
    func content(_ theme: ThemeName) -> some View {
      HStack(spacing: 0) {
        control.theme(theme).frame(width: 100, height: 40)
        ThemeName.studioLight.variables.colorSurfaceColumn.frame(width: 10)
        ThemeName.nocturneDark.variables.colorSurfaceColumn.frame(width: 10)
      }.frame(height: 40)
    }
    let host = NSHostingView(rootView: content(.studioDark))
    let window = NSWindow(contentRect: NSRect(x: 0, y: 0, width: 120, height: 40),
      styleMask: .borderless, backing: .buffered, defer: false)
    window.isReleasedWhenClosed = false
    defer { window.close() }
    window.contentView = host
    window.makeKeyAndOrderFront(nil)
    RunLoop.current.run(until: Date().addingTimeInterval(0.05))
    let before = try bitmap(host)
    click(host.convert(NSPoint(x: 70, y: 20), to: nil), in: window)
    XCTAssertEqual(changed, "night")
    host.rootView = content(.brightLight)
    let after = try bitmap(host)
    // Each preview's column keeps its own palette when the page changes.
    for (point, reference) in [(CGPoint(x: 15, y: 20), CGFloat(105)),
      (CGPoint(x: 57, y: 20), CGFloat(115))] {
      let x = Int(point.x * CGFloat(before.pixelsWide) / host.bounds.width)
      let y = Int(point.y * CGFloat(before.pixelsHigh) / host.bounds.height)
      let first = try XCTUnwrap(before.colorAt(x: x, y: y)?.usingColorSpace(.deviceRGB))
      let next = try XCTUnwrap(after.colorAt(x: x, y: y)?.usingColorSpace(.deviceRGB))
      // Compare within one rendered bitmap so the display's color profile
      // applies equally to the Canvas swatch and the reference Color view.
      let referenceX = Int(reference * CGFloat(before.pixelsWide) / host.bounds.width)
      let expected = try XCTUnwrap(before.colorAt(x: referenceX, y: y)?.usingColorSpace(.deviceRGB))
      XCTAssertEqual(first.redComponent, expected.redComponent, accuracy: 0.01)
      XCTAssertEqual(first.greenComponent, expected.greenComponent, accuracy: 0.01)
      XCTAssertEqual(first.blueComponent, expected.blueComponent, accuracy: 0.01)
      XCTAssertEqual(first, next)
    }
    XCTAssertEqual(changed, "night")
    XCTAssertTrue(Stories.story("preferences")!.examples.contains { $0.title == "Choosing a theme" })
  }

  func testControlledSelectionAndNarrowPreferenceRow() throws {
    var changed: String?
    let control = PreferenceThemeSwatches(options: options, value: "paper",
      onValueChange: { changed = $0 })
    let host = NSHostingView(rootView: AnyView(control.frame(width: 100, height: 40).theme(.studioLight)))
    let window = NSWindow(contentRect: NSRect(x: 0, y: 0, width: 100, height: 40),
      styleMask: .borderless, backing: .buffered, defer: false)
    window.isReleasedWhenClosed = false
    defer { window.close() }
    window.contentView = host
    window.makeKeyAndOrderFront(nil)
    RunLoop.current.run(until: Date().addingTimeInterval(0.05))
    click(host.convert(NSPoint(x: 70, y: 20), to: nil), in: window)
    XCTAssertEqual(changed, "night")
    host.rootView = AnyView(PreferenceRow(title: "Theme", trailing: {
      PreferenceThemeSwatches(options: self.options, value: "paper")
    }).frame(width: 105).theme(.studioLight))
    let size = host.fittingSize
    host.frame = NSRect(origin: .zero, size: size)
    XCTAssertGreaterThan(size.height, ThemeName.studioLight.variables.controlSmallSize)
    XCTAssertNotNil(try bitmap(host))
  }
}
