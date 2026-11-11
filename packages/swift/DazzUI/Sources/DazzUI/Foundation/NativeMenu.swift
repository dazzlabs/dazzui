import SwiftUI

#if canImport(AppKit)
  import AppKit
#endif

/// Headings and choices retain their original order, including mixed Select groups.
internal enum NativeMenuEntry {
  case item(MenuItem)
  case heading(String)
}

extension View {
  func nativeMenu(
    isPresented: Binding<Bool>, entries: [NativeMenuEntry],
    align: MenuAlign = .start, point: CGPoint? = nil,
    bar: NativeMenuBar? = nil, menuID: String? = nil,
    switchMenu: @escaping () -> Void = {}
  ) -> some View {
    modifier(
      NativeMenuModifier(
        isPresented: isPresented, entries: entries,
        align: align, point: point, bar: bar, menuID: menuID, switchMenu: switchMenu))
  }
}

private struct NativeMenuModifier: ViewModifier {
  @Binding var isPresented: Bool
  let entries: [NativeMenuEntry]
  let align: MenuAlign
  let point: CGPoint?
  let bar: NativeMenuBar?
  let menuID: String?
  let switchMenu: () -> Void
  @Environment(\.menuPresentation) private var presentation
  @Environment(\.theme) private var theme

  func body(content: Content) -> some View {
    content
      .background {
        #if canImport(AppKit)
          NativeMenuAnchor(
            isPresented: $isPresented, presentation: presentation,
            entries: entries, theme: theme, align: align, point: point,
            bar: bar, menuID: menuID, switchMenu: switchMenu)
        #endif
      }
      .onChange(of: presentation) { _ in isPresented = false }
      .onDisappear { isPresented = false }
  }
}

/// One coordinator per in-window menubar, never the application's main menu.
internal final class NativeMenuBar: ObservableObject {
  #if canImport(AppKit)
    private let anchors = NSHashTable<NativeMenuAnchorView>.weakObjects()
    func add(_ anchor: NativeMenuAnchorView) { anchors.add(anchor) }
    @MainActor func switchAtPointer(from current: NativeMenuAnchorView) {
      let point = NSEvent.mouseLocation
      guard
        let next = anchors.allObjects.first(where: { anchor in
          guard anchor !== current, anchor.canSwitch, let window = anchor.window else {
            return false
          }
          return window.convertToScreen(anchor.convert(anchor.bounds, to: nil)).contains(point)
        })
      else { return }
      current.cancel()
      next.switchMenu()
    }
  #endif
}

#if canImport(AppKit)
  /// Native rows keep AppKit's highlighting, keyboard navigation and accessibility.
  /// Slots become images within those rows, never a whole-row `NSMenuItem.view`.
  @MainActor
  internal final class NativeMenuSession: NSObject, NSMenuDelegate {
    let menu = NSMenu()
    private static var trackingCount = 0
    static var isTracking: Bool { trackingCount > 0 }
    private var actions: [Int: () -> Void] = [:]
    private(set) var cancelled = false
    private(set) var selected = false
    private(set) var tracking = false
    var didClose: () -> Void = {}
    var whileTracking: (() -> Void)?

    init(entries: [NativeMenuEntry], theme: Theme, scale: CGFloat = 2) {
      super.init()
      menu.autoenablesItems = false
      menu.delegate = self
      var nextTag = 0
      Self.fill(menu, from: entries, theme: theme, scale: scale, tags: &nextTag, into: self)
    }

    /// Builds the rows, descending into an item's `items` as an
    /// `NSMenuItem.submenu` — the nesting is AppKit's own, highlight and
    /// keyboard traversal included. Tags run across the whole tree, since
    /// every row reports its choice through this one session.
    private static func fill(
      _ menu: NSMenu, from entries: [NativeMenuEntry], theme: Theme,
      scale: CGFloat, tags: inout Int, into session: NativeMenuSession
    ) {
      for entry in entries {
        switch entry {
        case .heading(let title):
          if #available(macOS 14, *) {
            let heading = NSMenuItem.sectionHeader(title: title)
            heading.isEnabled = false
            menu.addItem(heading)
          } else {
            let heading = NSMenuItem(title: title, action: nil, keyEquivalent: "")
            heading.isEnabled = false
            menu.addItem(heading)
          }
        case .item(let choice):
          let item = NSMenuItem(
            title: choice.label, action: #selector(NativeMenuSession.choose(_:)),
            keyEquivalent: "")
          item.target = session
          item.tag = tags
          tags += 1
          item.isEnabled = !choice.disabled
          item.state = choice.checked == true ? .on : .off
          if let icon = choice.icon {
            item.image = Self.image(
              icon.environment(\.dazzIconSize, NSFont.systemFontSize)
                .foregroundStyle(Color(nsColor: .labelColor)), theme: theme, scale: scale)
            item.image?.isTemplate = true
          }
          if let shortcut = choice.shortcut,
            let image = Self.image(
              shortcut.font(.system(size: NSFont.systemFontSize)).foregroundStyle(
                Color(nsColor: .secondaryLabelColor)),
              theme: theme, scale: scale)
          {
            let title = NSMutableAttributedString(string: choice.label + "    ")
            let attachment = NSTextAttachment()
            attachment.image = image
            title.append(NSAttributedString(attachment: attachment))
            item.attributedTitle = title
            item.setAccessibilityLabel(choice.label)
          }
          if let children = choice.items, !children.isEmpty {
            let submenu = NSMenu(title: choice.label)
            submenu.autoenablesItems = false
            fill(submenu, from: children.map(NativeMenuEntry.item), theme: theme,
                 scale: scale, tags: &tags, into: session)
            item.submenu = submenu
          }
          session.actions[item.tag] = choice.onSelect ?? {}
          menu.addItem(item)
        }
      }
    }

    private static func image<V: View>(_ slot: V, theme: Theme, scale: CGFloat) -> NSImage? {
      let renderer = ImageRenderer(
        content: slot.fixedSize()
          .environment(\.theme, theme).environment(\.colorScheme, theme.name.colorScheme))
      renderer.scale = scale
      guard let cgImage = renderer.cgImage else { return nil }
      return NSImage(
        cgImage: cgImage,
        size: NSSize(width: CGFloat(cgImage.width) / scale, height: CGFloat(cgImage.height) / scale)
      )
    }

    @objc private func choose(_ sender: NSMenuItem) {
      // The row may sit in any level of the tree, so the session — not the
      // root menu — is what vouches for it.
      guard !cancelled, !selected, sender.isEnabled, sender.target === self,
        actions[sender.tag] != nil
      else { return }
      selected = true
      actions[sender.tag]?()
    }

    func present(at point: NSPoint, in view: NSView) {
      guard !cancelled, view.window != nil else { return }
      tracking = true
      Self.trackingCount += 1
      // SwiftUI updates and menubar transitions must keep running in AppKit's
      // nested tracking loop, including changes made from other windows.
      let timer = Timer(timeInterval: 0.02, repeats: true) { [weak self] _ in
        MainActor.assumeIsolated { self?.whileTracking?() }
      }
      RunLoop.main.add(timer, forMode: .eventTracking)
      menu.popUp(positioning: nil, at: point, in: view)
      timer.invalidate()
      tracking = false
      Self.trackingCount -= 1
      didClose()
    }

    func cancel() {
      cancelled = true
      if tracking { menu.cancelTrackingWithoutAnimation() }
    }
  }

  private struct NativeMenuAnchor: NSViewRepresentable {
    @Binding var isPresented: Bool
    let presentation: MenuPresentation
    let entries: [NativeMenuEntry]
    let theme: Theme
    let align: MenuAlign
    let point: CGPoint?
    let bar: NativeMenuBar?
    let menuID: String?
    let switchMenu: () -> Void

    func makeNSView(context: Context) -> NativeMenuAnchorView { NativeMenuAnchorView() }
    func updateNSView(_ view: NativeMenuAnchorView, context: Context) {
      view.canSwitch = !entries.isEmpty && menuID != nil
      view.switchMenu = switchMenu
      bar?.add(view)
      view.update(
        shown: isPresented, presentation: presentation, entries: entries,
        theme: theme, align: align, point: point, bar: bar,
        dismiss: { isPresented = false })
    }
    static func dismantleNSView(_ view: NativeMenuAnchorView, coordinator: ()) { view.cancel() }
  }

  @MainActor
  internal final class NativeMenuAnchorView: NSView {
    var canSwitch = false
    var switchMenu: () -> Void = {}
    private(set) var session: NativeMenuSession?
    private var generation = 0
    private var presentation: MenuPresentation?
    private var closeObserver: NSObjectProtocol?

    override func hitTest(_ point: NSPoint) -> NSView? { nil }
    override func viewDidMoveToWindow() {
      super.viewDidMoveToWindow()
      if window == nil { cancel() }
    }

    func update(
      shown: Bool, presentation: MenuPresentation, entries: [NativeMenuEntry],
      theme: Theme, align: MenuAlign, point: CGPoint?, bar: NativeMenuBar?,
      dismiss: @escaping () -> Void
    ) {
      let changed = self.presentation != nil && self.presentation != presentation
      self.presentation = presentation
      guard !changed, shown, presentation.usesNativeMenus else {
        cancel()
        if changed && shown { DispatchQueue.main.async { dismiss() } }
        return
      }
      guard session == nil else { return }
      generation += 1
      let token = generation
      let session = NativeMenuSession(
        entries: entries, theme: theme, scale: window?.backingScaleFactor ?? 2)
      self.session = session
      session.didClose = { [weak self] in
        guard let self, self.generation == token else { return }
        self.cleanUp()
        dismiss()
      }
      session.whileTracking = { [weak self, weak bar] in
        guard let self else { return }
        bar?.switchAtPointer(from: self)
      }
      DispatchQueue.main.async { [weak self] in
        guard let self, self.generation == token else { return }
        guard let window = self.window else {
          self.cancel()
          dismiss()
          return
        }
        self.closeObserver = NotificationCenter.default.addObserver(
          forName: NSWindow.willCloseNotification, object: window, queue: .main
        ) { [weak self] _ in
          MainActor.assumeIsolated {
            self?.cancel()
            dismiss()
          }
        }
        var location = NSPoint(x: 0, y: 0)
        if let point {
          location = NSPoint(x: point.x, y: self.bounds.height - point.y)
        } else if align == .end {
          location.x = self.bounds.width - session.menu.size.width
        }
        session.present(at: location, in: self)
      }
    }

    private func cleanUp() {
      if let closeObserver { NotificationCenter.default.removeObserver(closeObserver) }
      closeObserver = nil
      session = nil
    }

    func cancel() {
      generation += 1
      session?.cancel()
      cleanUp()
    }

    deinit {
      if let closeObserver { NotificationCenter.default.removeObserver(closeObserver) }
    }
  }
#endif
