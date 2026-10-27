import AppKit
import SwiftUI
import XCTest

@testable import DazzUI
@testable import Example

@MainActor
final class SystemSizingTests: XCTestCase {
  private func scaled(_ base: SystemSizingMetrics, by scale: CGFloat) -> SystemSizingMetrics {
    SystemSizingMetrics(bodyFontSize: base.bodyFontSize * scale, controls: base.controls.map {
      .init(fontSize: $0.fontSize * scale, height: $0.height * scale)
    })
  }

  func testReadsNativeFontsAndControlSizes() {
    let metrics = SystemSizingMetrics.read()
    let body = NSFont.preferredFont(forTextStyle: .body, options: [:]).pointSize
    XCTAssertEqual(metrics.bodyFontSize, body)
    for (index, size) in [NSControl.ControlSize.mini, .small, .regular, .large].enumerated() {
      XCTAssertEqual(metrics.controls[index].fontSize, NSFont.systemFontSize(for: size) * body / NSFont.systemFontSize)
      XCTAssertGreaterThan(metrics.controls[index].height, metrics.controls[index].fontSize)
    }
    XCTAssertEqual(Theme(.studioLight).sizing, .design)
    XCTAssertEqual(Theme(.studioLight, sizing: .system).vars.bodyMedium.size, body)
  }

  func testAllThemesResolveSystemSizingWithoutChangingPaintOrTimings() {
    let native = SystemSizingMetrics.read()
    for name in ThemeName.allCases {
      let design = name.variables
      for scale in [CGFloat(1), 1.5, 2.25] {
        let metrics = scaled(native, by: scale)
        let vars = Theme(name, sizing: .system, metrics: metrics).vars
        let layoutScale = metrics.bodyFontSize / design.bodyMedium.size
        XCTAssertEqual(vars.bodyMedium.size, metrics.bodyFontSize, accuracy: 0.001)
        XCTAssertEqual(vars.bodyMedium.lineHeight, design.bodyMedium.lineHeight * layoutScale, accuracy: 0.001)
        XCTAssertEqual(vars.spacing4, design.spacing4 * layoutScale, accuracy: 0.001)
        XCTAssertEqual(vars.controlSmallPaddingInline, design.controlSmallPaddingInline * layoutScale, accuracy: 0.001)
        XCTAssertEqual(vars.iconMedium, design.iconMedium * layoutScale, accuracy: 0.001)
        XCTAssertEqual(vars.strokeHairline, design.strokeHairline)
        XCTAssertEqual(vars.radiusMedium, design.radiusMedium)
        XCTAssertEqual(vars.focusWidth, design.focusWidth)
        XCTAssertEqual(vars.motionDuration, design.motionDuration)
        XCTAssertEqual(vars.colorSurface, design.colorSurface)
        XCTAssertEqual(vars.controlColorFilledContent, design.controlColorFilledContent)
        for size in DazzUI.ControlSize.allCases {
          XCTAssertEqual(vars.metrics(size).size, metrics.control(size).height)
          XCTAssertEqual(vars.metrics(size).content.size, metrics.control(size).fontSize, accuracy: 0.001)
          let host = NSHostingView(rootView: DazzUI.Button(size: size, action: {}) { Text("Save 保存") }
            .environment(\.theme, Theme(name, sizing: .system, metrics: metrics)))
          XCTAssertEqual(host.fittingSize.height, metrics.control(size).height, accuracy: 0.001)
        }
      }
    }
  }

  @MainActor private final class Model: ObservableObject {
    @Published var metrics = SystemSizingMetrics.read()
    @Published var sizing: ThemeSizing = .system
  }

  private struct EditingFixture: View {
    @ObservedObject var model: Model
    @State private var text = "Keep this selection"
    var body: some View {
      DazzUI.TextField(text: $text)
        .modifier(SystemThemeModifier(name: .studioLight, sizing: model.sizing, readMetrics: { model.metrics }))
        .frame(width: 320)
    }
  }

  private func descendants(_ view: NSView) -> [NSView] {
    [view] + view.subviews.flatMap { descendants($0) }
  }

  func testLiveSizingPreservesNativeEditorAndSelection() throws {
    let native = SystemSizingMetrics.read()
    let model = Model()
    let host = NSHostingView(rootView: EditingFixture(model: model))
    host.frame = NSRect(x: 0, y: 0, width: 400, height: 160)
    let window = NSWindow(contentRect: host.frame, styleMask: [.titled], backing: .buffered, defer: false)
    window.contentView = host
    window.makeKeyAndOrderFront(nil)
    defer { window.orderOut(nil); window.contentView = nil }
    host.layoutSubtreeIfNeeded()
    let field = try XCTUnwrap(descendants(host).compactMap { $0 as? NSTextField }.first)
    window.makeFirstResponder(field)
    let editor = try XCTUnwrap(field.currentEditor())
    editor.selectedRange = NSRange(location: 5, length: 4)
    for scale in [CGFloat(1.5), 2.25, 1] {
      model.metrics = scaled(native, by: scale)
      NotificationCenter.default.post(name: NSFont.fontSetChangedNotification, object: nil)
      RunLoop.main.run(until: Date().addingTimeInterval(0.05))
      host.layoutSubtreeIfNeeded()
      let current = try XCTUnwrap(descendants(host).compactMap { $0 as? NSTextField }.first)
      XCTAssertTrue(current === field)
      XCTAssertEqual(current.stringValue, "Keep this selection")
      XCTAssertEqual(editor.selectedRange, NSRange(location: 5, length: 4))
      XCTAssertEqual(current.font?.pointSize ?? 0, native.control(.medium).fontSize * scale, accuracy: 0.001)
    }
    model.sizing = .design
    RunLoop.main.run(until: Date().addingTimeInterval(0.05))
    host.layoutSubtreeIfNeeded()
    XCTAssertEqual(field.stringValue, "Keep this selection")
    XCTAssertEqual(field.font?.pointSize ?? 0, Theme.default.vars.controlMediumContent.size, accuracy: 0.001)
  }

  func testEveryStoryRendersAtLargestTestedSystemSize() throws {
    let metrics = scaled(SystemSizingMetrics.read(), by: 2.25)
    for name in [ThemeName.studioLight, .studioDark] {
      for story in Stories.all {
        let host = NSHostingView(rootView: SceneView(scene: story.preview)
          .environment(\.theme, Theme(name, sizing: .system, metrics: metrics))
          .frame(width: 960, height: 720).overlayHost())
        host.frame = NSRect(x: 0, y: 0, width: 960, height: 720)
        host.layoutSubtreeIfNeeded()
        let bitmap = try XCTUnwrap(host.bitmapImageRepForCachingDisplay(in: host.bounds), story.id)
        host.cacheDisplay(in: host.bounds, to: bitmap)
        XCTAssertGreaterThan(bitmap.pixelsWide, 0, story.id)
      }
    }
  }
}
