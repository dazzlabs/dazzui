#if os(macOS)
  import AppKit

  public extension NSWindow {
    private static weak var dazzWindowFollowingPointer: NSWindow?

    /// True while ``followPointer(from:)`` is actively moving this window.
    ///
    /// Custom `NSWindow` subclasses can return the proposed frame unchanged
    /// from `constrainFrameRect(_:to:)` while this holds, allowing smooth
    /// drags across displays with differing backing scales; the top edge is
    /// settled below the menu bar on release.
    var isFollowingPointer: Bool {
      NSWindow.dazzWindowFollowingPointer === self
    }

    /// Moves the window directly with the pointer until mouse-up, keeping
    /// AppKit's frame authoritative across mixed-DPI display transitions.
    func followPointer(from event: NSEvent) {
      NSWindow.dazzWindowFollowingPointer = self
      defer { NSWindow.dazzWindowFollowingPointer = nil }

      let pointerStart = NSEvent.mouseLocation
      let leftStart = frame.minX
      let topStart = frame.maxY

      trackEvents(
        matching: [.leftMouseDragged, .leftMouseUp],
        timeout: NSEvent.foreverDuration,
        mode: .eventTracking
      ) { nextEvent, stop in
        guard let nextEvent, nextEvent.type == .leftMouseDragged else {
          stop.pointee = true
          return
        }
        let pointer = NSEvent.mouseLocation
        let size = self.frame.size
        self.setFrame(
          NSRect(
            x: leftStart + pointer.x - pointerStart.x,
            y: topStart + pointer.y - pointerStart.y - size.height,
            width: size.width,
            height: size.height
          ),
          display: false
        )
      }

      keepTopEdgeBelowMenuBar()
    }

    private func keepTopEdgeBelowMenuBar() {
      let top = NSPoint(x: frame.midX, y: frame.maxY - 1)
      guard let screen = NSScreen.screens.first(where: { $0.frame.contains(top) }) ?? screen else {
        return
      }
      let limit = screen.visibleFrame.maxY
      guard frame.maxY > limit else { return }
      setFrame(frame.offsetBy(dx: 0, dy: limit - frame.maxY), display: false)
    }
  }
#endif
