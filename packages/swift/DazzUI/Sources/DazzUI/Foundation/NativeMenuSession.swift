#if os(macOS) || ((os(Windows) || os(Linux)) && !SWIFTUI_PORT_HARMONY)
  import Foundation
  import NativeAPI
  import SwiftUI
  #if canImport(AppKit)
    import AppKit
  #endif

  /// Owns a single popup and every item listener, including nested choices.
  /// NativeAPI opens synchronously on Windows and asynchronously on macOS/Linux.
  @MainActor
  internal final class NativeMenuSession {
    let menu: NativeAPI.Menu?
    private static var trackingCount = 0
    static var isTracking: Bool { trackingCount > 0 }
    private var listeners: [(NativeAPI.MenuItem, NativeAPI.ListenerId)] = []
    private var menuListener: NativeAPI.ListenerId?
    private var keepAlive: NativeMenuSession?
    private(set) var opened = false
    private var finished = false
    private var timer: NativeAPI.RunLoopTimer?
    #if os(Windows) || os(Linux)
      private var pendingOpen: (() -> Void)?
    #endif
    private(set) var cancelled = false
    private(set) var selected = false
    private(set) var tracking = false
    var didClose: () -> Void = {}
    var whileTracking: (() -> Void)?

    init(entries: [NativeMenuEntry], theme: Theme, scale: CGFloat = 2) {
      menu = NativeAPI.Menu.create()
      guard let menu else { return }
      fill(menu, entries: entries, theme: theme, scale: scale)
      menuListener = menu.addListener { [weak self] event in
        guard let self else { return }
        switch event {
        case .opened:
          self.opened = true
        case .closed:
          self.finish()
        default: break
        }
      }
    }

    private func fill(
      _ menu: NativeAPI.Menu, entries: [NativeMenuEntry], theme: Theme, scale: CGFloat
    ) {
      for entry in entries {
        switch entry {
        case .heading(let title):
          guard let item = NativeAPI.MenuItem.createWithLabelAndType(title, type: .normal) else {
            continue
          }
          item.setEnabled(false)
          menu.addItem(item)
        case .item(let choice):
          let children = choice.items ?? []
          let type: NativeAPI.MenuItemType =
            !children.isEmpty
            ? .submenu
            : choice.checked != nil ? .checkbox : .normal
          // A hint is text only; it must never register an accelerator.
          let label = choice.nativeShortcut.map { choice.label + "\t" + $0 } ?? choice.label
          guard let item = NativeAPI.MenuItem.createWithLabelAndType(label, type: type) else {
            continue
          }
          item.setEnabled(!choice.disabled)
          item.setState(choice.checked == true ? .checked : .unchecked)
          #if canImport(AppKit)
            if let icon = choice.icon {
              let renderer = ImageRenderer(
                content:
                  icon.environment(\.dazzIconSize, NSFont.systemFontSize)
                  .foregroundStyle(Color(nsColor: .labelColor))
                  .environment(\.theme, theme).environment(\.colorScheme, theme.colorScheme)
                  .fixedSize())
              renderer.scale = scale
              if let cgImage = renderer.cgImage,
                let data = NSBitmapImageRep(cgImage: cgImage).representation(
                  using: .png, properties: [:]),
                let image = NativeAPI.Image.fromBase64(data.base64EncodedString())
              {
                item.setIcon(image)
              }
            }
          #endif
          if !children.isEmpty, let submenu = NativeAPI.Menu.create() {
            fill(submenu, entries: children.map(NativeMenuEntry.item), theme: theme, scale: scale)
            item.setSubmenu(submenu)
          } else {
            let listener = item.addListener { [weak self] event in
              guard case .itemClicked = event, let self,
                !self.cancelled, !self.selected, !self.finished, !choice.disabled
              else { return }
              self.selected = true
              choice.onSelect?()
            }
            listeners.append((item, listener))
          }
          menu.addItem(item)
        }
      }
    }

    func present(strategy: NativeAPI.PositioningStrategy?, placement: NativeAPI.Placement) {
      guard !cancelled, !finished, !tracking else { return }
      guard let menu, let strategy else {
        finish()
        return
      }
      tracking = true
      Self.trackingCount += 1
      keepAlive = self
      timer = NativeAPI.RunLoopTimer(milliseconds: 20) { [weak self] in
        MainActor.assumeIsolated { self?.tick() }
      }
      #if canImport(AppKit)
        if !menu.open(strategy, placement: placement) { finish() }
      #else
        // Leave the render transaction before entering Win32's nested menu loop.
        pendingOpen = { [weak self] in
          guard let self else { return }
          if !menu.open(strategy, placement: placement) { self.finish() }
        }
      #endif
    }

    private func tick() {
      #if os(Windows) || os(Linux)
        if let open = pendingOpen {
          pendingOpen = nil
          if cancelled { finish() } else { open() }
          return
        }
      #endif
      if cancelled && opened { _ = menu?.close() } else if !cancelled && opened { whileTracking?() }
    }

    func cancel() {
      cancelled = true
      #if canImport(AppKit)
        _ = menu?.close()
        if !opened { finish() }
      #else
        if opened { _ = menu?.close() }
      #endif
      // No native open has been queued yet: nothing needs to keep us alive.
      if !tracking { finish() }
    }

    private func finish() {
      guard !finished else { return }
      finished = true
      if tracking { Self.trackingCount -= 1 }
      tracking = false
      #if os(Windows) || os(Linux)
        pendingOpen = nil
      #endif
      timer?.invalidate()
      timer = nil
      didClose()
      keepAlive = nil
    }

    deinit {
      if let menuListener { _ = menu?.removeListener(menuListener) }
      for (item, listener) in listeners { _ = item.removeListener(listener) }
    }
  }

  #if canImport(AppKit)
    extension NativeMenuSession {
      func present(at point: NSPoint, in view: NSView, align: MenuAlign = .start) {
        guard let window = view.window else { return }
        let screen = window.convertPoint(toScreen: view.convert(point, to: nil))
        let top = NSScreen.screens.first?.frame.maxY ?? 0
        present(
          strategy: .absolute(NativeAPI.Point(x: screen.x, y: top - screen.y)),
          placement: align == .end ? .bottomEnd : .bottomStart)
      }
    }
  #endif
#endif
