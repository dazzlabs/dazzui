import SwiftUI

#if os(macOS)
  import AppKit
#elseif (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
  import NativeAPI
#endif

public extension SwiftUI.View {
  /// Makes a stretch of custom chrome behave like a native window titlebar:
  /// dragging bare space moves the window and double-clicking performs the
  /// platform's titlebar double-click action (zoom/minimize/maximize).
  func windowDragRegion() -> some SwiftUI.View {
    background(WindowDragRegion())
  }
}

/// An invisible region that hands pointer drags and titlebar double-clicks to
/// the enclosing native window.
public struct WindowDragRegion: SwiftUI.View {
  public init() {}

  public var body: some SwiftUI.View {
    #if os(macOS)
      MacOSWindowDragRegion()
    #elseif (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
      PortableWindowDragRegion()
    #else
      SwiftUI.Color.clear
    #endif
  }
}

#if os(macOS)
  private struct MacOSWindowDragRegion: NSViewRepresentable {
    func makeNSView(context: Context) -> WindowDragNSView { WindowDragNSView() }
    func updateNSView(_ view: WindowDragNSView, context: Context) {}
  }

  private final class WindowDragNSView: NSView {
    override func acceptsFirstMouse(for event: NSEvent?) -> Bool { true }

    override func mouseDown(with event: NSEvent) {
      guard let window else { return }
      if event.clickCount == 2 {
        Self.handleDoubleClick(window)
        return
      }
      window.performDrag(with: event)
    }

    private static func handleDoubleClick(_ window: NSWindow) {
      switch UserDefaults.standard.string(forKey: "AppleActionOnDoubleClick") {
      case "Minimize":
        window.performMiniaturize(nil)
      case "None":
        break
      default:
        window.performZoom(nil)
      }
    }
  }
#elseif (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
  private struct PortableWindowDragRegion: SwiftUI.View {
    @Environment(\.nativeWindow) private var nativeWindow
    @State private var dragging = false

    private var targetWindow: NativeAPI.Window? {
      nativeWindow ?? Application.shared.getPrimaryWindow()
    }

    var body: some SwiftUI.View {
      SwiftUI.Color.clear
        .contentShape(Rectangle())
        .onTapGesture(count: 2) {
          _ = targetWindow?.performTitleBarDoubleClick()
        }
        .gesture(
          DragGesture(minimumDistance: 2)
            .onChanged { _ in
              guard !dragging else { return }
              dragging = true
              targetWindow?.startDragging()
              #if os(Linux)
                DispatchQueue.main.async {
                  dragging = false
                }
              #endif
            }
            .onEnded { _ in
              dragging = false
            }
        )
    }
  }
#endif
