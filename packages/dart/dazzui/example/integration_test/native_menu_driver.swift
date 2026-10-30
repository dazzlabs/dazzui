import AppKit

// Debug-only driver called by native_menu_test.dart. Uses AppKit's event queue
// rather than a mocked presenter; no accessibility permission is required.
#if DEBUG
  private var activeMenu: NSMenu?
  private var observers: [NSObjectProtocol] = []
  private let commandsLock = NSLock()
  private var commands: [UInt16] = []
  private var pointerCommand: NSPoint?
  private var originalPointer: CGPoint?
  private var driverTimer: Timer?

  @_cdecl("dazzui_observe_menus")
  public func observeMenus() {
    DispatchQueue.main.async {
      NSApp.activate(ignoringOtherApps: true)
      NSApp.windows.first?.makeKeyAndOrderFront(nil)
      observers.append(
        NotificationCenter.default.addObserver(
          forName: NSMenu.didBeginTrackingNotification, object: nil, queue: .main
        ) { activeMenu = $0.object as? NSMenu })
      observers.append(
        NotificationCenter.default.addObserver(
          forName: NSMenu.didEndTrackingNotification, object: nil, queue: .main
        ) { _ in activeMenu = nil })
      let timer = Timer(timeInterval: 0.02, repeats: true) { _ in
        commandsLock.lock()
        let next = commands
        let pointer = pointerCommand
        pointerCommand = nil
        commands.removeAll()
        commandsLock.unlock()
        if let pointer, let window = NSApp.mainWindow, let view = window.contentView {
          if originalPointer == nil { originalPointer = CGEvent(source: nil)?.location }
          let point = window.convertPoint(
            toScreen: view.convert(
              NSPoint(x: pointer.x, y: view.bounds.height - pointer.y), to: nil))
          let height = NSScreen.screens.first?.frame.maxY ?? 0
          CGWarpMouseCursorPosition(CGPoint(x: point.x, y: height - point.y))
        }
        for code in next {
          if code == 1000, let menu = activeMenu {
            menu.performActionForItem(at: 0)
            menu.cancelTrackingWithoutAnimation()
          } else {
            postKey(code)
          }
        }
      }
      RunLoop.main.add(timer, forMode: .common)
      RunLoop.main.add(timer, forMode: .eventTracking)
      driverTimer = timer
    }
  }

  @_cdecl("dazzui_stop_menu_driver")
  public func stopDriver() {
    let stop = {
      driverTimer?.invalidate()
      driverTimer = nil
      activeMenu?.cancelTrackingWithoutAnimation()
      activeMenu = nil
      for observer in observers { NotificationCenter.default.removeObserver(observer) }
      observers.removeAll()
      if let originalPointer { CGWarpMouseCursorPosition(originalPointer) }
      originalPointer = nil
    }
    if Thread.isMainThread { stop() } else { DispatchQueue.main.sync(execute: stop) }
  }

  @_cdecl("dazzui_menu_count")
  public func menuCount() -> Int32 {
    var count: Int32 = 0
    let read = { count = Int32(activeMenu?.numberOfItems ?? 0) }
    if Thread.isMainThread { read() } else { DispatchQueue.main.sync(execute: read) }
    return count
  }

  @_cdecl("dazzui_move_pointer")
  public func movePointer(_ x: Double, _ y: Double) {
    commandsLock.lock()
    pointerCommand = NSPoint(x: x, y: y)
    commandsLock.unlock()
  }

  @_cdecl("dazzui_menu_key")
  public func menuKey(_ keyCode: UInt16) {
    commandsLock.lock()
    commands.append(keyCode)
    commandsLock.unlock()
  }

  private func postKey(_ keyCode: UInt16) {
    let characters: String
    switch keyCode {
    case 125: characters = String(UnicodeScalar(NSDownArrowFunctionKey)!)
    case 126: characters = String(UnicodeScalar(NSUpArrowFunctionKey)!)
    case 36: characters = "\r"
    default: characters = "\u{1b}"
    }
    for type in [NSEvent.EventType.keyDown] {
      if let event = NSEvent.keyEvent(
        with: type, location: .zero,
        modifierFlags: [], timestamp: ProcessInfo.processInfo.systemUptime,
        windowNumber: NSApp.keyWindow?.windowNumber ?? 0, context: nil,
        characters: characters, charactersIgnoringModifiers: characters,
        isARepeat: false, keyCode: keyCode)
      {
        NSApp.postEvent(event, atStart: false)
      }
    }
  }
#endif
