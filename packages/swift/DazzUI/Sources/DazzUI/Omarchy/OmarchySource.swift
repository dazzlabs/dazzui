import Foundation
import SwiftUI
#if !os(WASI)
  import Dispatch
#endif
#if canImport(Darwin)
  import Darwin
#elseif os(Linux) && !SWIFTUI_PORT_HARMONY
  import Glibc

  @_silgen_name("inotify_init")
  private func linux_inotify_init() -> CInt

  @_silgen_name("inotify_add_watch")
  private func linux_inotify_add_watch(
    _ fd: CInt,
    _ pathname: UnsafePointer<CChar>,
    _ mask: UInt32
  ) -> CInt
#endif

/// The palette the machine is wearing now, or `nil` when this is not an
/// Omarchy machine or its current theme has no `colors.toml`.
///
/// Omarchy keeps its current theme in a directory it swaps whole on
/// `omarchy theme set`: `$XDG_STATE_HOME/omarchy/current/` (by default
/// `~/.local/state`), holding the staged `theme/` and a `theme.name`; before
/// that it was `~/.config/omarchy/current/`, whose `theme` is a symlink into
/// the theme itself. The first of those that holds a readable palette is the
/// one read. `directory` reads that one directory instead.
///
/// The type scale is read the way the shell reads it: the theme's
/// `shell.toml`, then the user's own in `~/.config/omarchy/` — or in `config`,
/// when `directory` is given — whose keys win.
public func readOmarchyPalette(
  directory: String? = nil,
  config: String? = nil
) -> OmarchyPalette? {
  #if os(WASI)
    return nil
  #else
    let user = omarchyConfigDirectory(directory: directory, config: config)
    for current in omarchyCandidates(directory) {
      if let palette = readPalette(in: current, config: user) {
        return palette
      }
    }
    return nil
  #endif
}

/// The current palette, and again every time the machine changes theme.
///
/// Omarchy replaces `current/theme` outright and then writes
/// `current/theme.name`, so what is watched is `current/` itself, beside the
/// user's config directory for their own `shell.toml`; a burst of events is
/// read once, after it settles, and a palette the same as the last one is not
/// emitted twice.
public func watchOmarchyPalette(
  directory: String? = nil,
  config: String? = nil
) -> AsyncStream<OmarchyPalette?> {
  #if os(WASI)
    return AsyncStream { continuation in
      continuation.yield(nil)
      continuation.finish()
    }
  #else
    let candidates = omarchyCandidates(directory)
    let userConfig = omarchyConfigDirectory(directory: directory, config: config)
    guard !candidates.isEmpty || userConfig != nil else {
      return AsyncStream { continuation in
        continuation.yield(nil)
        continuation.finish()
      }
    }
    return AsyncStream { continuation in
      let initial = readOmarchyPalette(directory: directory, config: config)
      let watcher = OmarchyDirectoryWatcher(
        directory: directory,
        config: config,
        candidates: candidates,
        userConfig: userConfig,
        initial: initial
      ) { next in
        continuation.yield(next)
      }
      watcher.start()
      continuation.yield(initial)
      continuation.onTermination = { _ in
        watcher.stop()
      }
    }
  #endif
}

#if !os(WASI)
  private func omarchyCandidates(_ directory: String?) -> [String] {
    if let directory { return [directory] }
    #if os(Linux) && !SWIFTUI_PORT_HARMONY
      let env = ProcessInfo.processInfo.environment
      let home = env["HOME"]
      let xdgState = env["XDG_STATE_HOME"].flatMap { $0.isEmpty ? nil : $0 }
      let state = xdgState ?? home.map { "\($0)/.local/state" }
      var paths: [String] = []
      if let state { paths.append("\(state)/omarchy/current") }
      if let home { paths.append("\(home)/.config/omarchy/current") }
      return paths
    #else
      return []
    #endif
  }

  private func omarchyConfigDirectory(directory: String?, config: String?) -> String? {
    if config != nil || directory != nil { return config }
    #if os(Linux) && !SWIFTUI_PORT_HARMONY
      guard let home = ProcessInfo.processInfo.environment["HOME"], !home.isEmpty else {
        return nil
      }
      return "\(home)/.config/omarchy"
    #else
      return nil
    #endif
  }

  private func readPalette(in current: String, config: String?) -> OmarchyPalette? {
    let fm = FileManager.default
    let colorsPath = "\(current)/theme/colors.toml"
    guard fm.fileExists(atPath: colorsPath),
      let colorsToml = try? String(contentsOfFile: colorsPath, encoding: .utf8)
    else {
      // Caught mid-swap or absent: the next event reads it whole.
      return nil
    }
    let lightMode = fm.fileExists(atPath: "\(current)/theme/light.mode")
    var shellSources: [String] = []
    for shellPath in ["\(current)/theme/shell.toml", config.map { "\($0)/shell.toml" }]
      .compactMap({ $0 })
    {
      if fm.fileExists(atPath: shellPath),
        let content = try? String(contentsOfFile: shellPath, encoding: .utf8)
      {
        shellSources.append(content)
      }
    }
    return OmarchyPalette.parse(
      colorsToml,
      name: readThemeName(in: current),
      colorScheme: lightMode ? .light : nil,
      shellStyle: OmarchyShellStyle.fromShellToml(shellSources)
    )
  }

  private func readThemeName(in current: String) -> String? {
    let namePath = "\(current)/theme.name"
    if let content = try? String(contentsOfFile: namePath, encoding: .utf8) {
      let trimmed = content.trimmingCharacters(in: .whitespacesAndNewlines)
      if !trimmed.isEmpty { return trimmed }
    }
    let linkPath = "\(current)/theme"
    if let target = try? FileManager.default.destinationOfSymbolicLink(atPath: linkPath),
      let last = target.split(separator: "/").last, !last.isEmpty
    {
      return String(last)
    }
    return nil
  }

  private final class OmarchyDirectoryWatcher: @unchecked Sendable {
    private let directory: String?
    private let config: String?
    private let candidates: [String]
    private let userConfig: String?
    private let emit: @Sendable (OmarchyPalette?) -> Void
    private let queue = DispatchQueue(label: "dev.dazzlabs.dazzui.omarchy-watcher")

    private var last: OmarchyPalette?
    private var stopped = false
    private var settle: DispatchWorkItem?

    #if canImport(Darwin)
      private struct Entry {
        let dev: Int
        let ino: UInt64
        let source: any DispatchSourceFileSystemObject
      }
      private var entries: [String: Entry] = [:]
    #elseif os(Linux) && !SWIFTUI_PORT_HARMONY
      private var inotifyFd: CInt = -1
      private var readSource: (any DispatchSourceRead)?
      private var fallbackTimer: (any DispatchSourceTimer)?
    #else
      private var pollTimer: (any DispatchSourceTimer)?
    #endif

    init(
      directory: String?,
      config: String?,
      candidates: [String],
      userConfig: String?,
      initial: OmarchyPalette?,
      emit: @escaping @Sendable (OmarchyPalette?) -> Void
    ) {
      self.directory = directory
      self.config = config
      self.candidates = candidates
      self.userConfig = userConfig
      self.last = initial
      self.emit = emit
    }

    func start() {
      queue.sync {
        guard !stopped else { return }
        #if canImport(Darwin)
          syncDarwinWatches()
        #elseif os(Linux) && !SWIFTUI_PORT_HARMONY
          startLinuxWatches()
        #else
          startPolling()
        #endif
      }
    }

    func stop() {
      queue.async { [self] in
        guard !stopped else { return }
        stopped = true
        settle?.cancel()
        settle = nil
        #if canImport(Darwin)
          for entry in entries.values {
            entry.source.cancel()
          }
          entries.removeAll()
        #elseif os(Linux) && !SWIFTUI_PORT_HARMONY
          readSource?.cancel()
          readSource = nil
          fallbackTimer?.cancel()
          fallbackTimer = nil
        #else
          pollTimer?.cancel()
          pollTimer = nil
        #endif
      }
    }

    private func scheduleRefresh() {
      guard !stopped else { return }
      settle?.cancel()
      let item = DispatchWorkItem { [weak self] in
        self?.refresh()
      }
      settle = item
      queue.asyncAfter(deadline: .now() + .milliseconds(150), execute: item)
    }

    private func refresh() {
      guard !stopped else { return }
      #if canImport(Darwin)
        syncDarwinWatches()
      #elseif os(Linux) && !SWIFTUI_PORT_HARMONY
        syncLinuxWatches()
      #endif
      let palette = readOmarchyPalette(directory: directory, config: config)
      guard palette != last else { return }
      last = palette
      emit(palette)
    }

    #if canImport(Darwin)
      private func watchedPaths() -> [String] {
        var paths: [String] = []
        for dir in candidates {
          paths.append(dir)
          paths.append("\(dir)/theme")
          paths.append("\(dir)/theme/colors.toml")
          paths.append("\(dir)/theme/shell.toml")
          paths.append("\(dir)/theme/light.mode")
          paths.append("\(dir)/theme.name")
        }
        if let userConfig {
          paths.append(userConfig)
          paths.append("\(userConfig)/shell.toml")
        }
        return paths
      }

      private func syncDarwinWatches() {
        let wanted = Set(watchedPaths())
        for (path, entry) in entries where !wanted.contains(path) {
          entry.source.cancel()
          entries.removeValue(forKey: path)
        }
        for path in wanted {
          guard
            let attrs = try? FileManager.default.attributesOfItem(atPath: path),
            let ino = (attrs[.systemFileNumber] as? NSNumber)?.uint64Value
          else {
            if let old = entries.removeValue(forKey: path) {
              old.source.cancel()
            }
            continue
          }
          let dev = (attrs[.systemNumber] as? NSNumber)?.intValue ?? 0
          if let existing = entries[path] {
            if existing.dev == dev && existing.ino == ino { continue }
            existing.source.cancel()
            entries.removeValue(forKey: path)
          }
          let fd = Darwin.open(path, O_EVTONLY)
          guard fd >= 0 else { continue }
          let source = DispatchSource.makeFileSystemObjectSource(
            fileDescriptor: fd,
            eventMask: [.write, .delete, .rename, .attrib, .extend, .link],
            queue: queue
          )
          source.setEventHandler { [weak self, weak source] in
            guard let self, !self.stopped else { return }
            let flags = source?.data ?? []
            if flags.contains(.delete) || flags.contains(.rename) {
              if let removed = self.entries.removeValue(forKey: path) {
                removed.source.cancel()
              }
            }
            self.syncDarwinWatches()
            self.scheduleRefresh()
          }
          source.setCancelHandler {
            Darwin.close(fd)
          }
          entries[path] = Entry(dev: dev, ino: ino, source: source)
          source.resume()
        }
      }
    #elseif os(Linux) && !SWIFTUI_PORT_HARMONY
      // Universal Linux UAPI `<linux/inotify.h>` event mask bits.
      private static let inotifyMask: UInt32 =
        0x0000_0002  // IN_MODIFY
        | 0x0000_0004  // IN_ATTRIB
        | 0x0000_0008  // IN_CLOSE_WRITE
        | 0x0000_0040  // IN_MOVED_FROM
        | 0x0000_0080  // IN_MOVED_TO
        | 0x0000_0100  // IN_CREATE
        | 0x0000_0200  // IN_DELETE
        | 0x0000_0400  // IN_DELETE_SELF
        | 0x0000_0800  // IN_MOVE_SELF

      private func startLinuxWatches() {
        let fd = linux_inotify_init()
        guard fd >= 0 else {
          let timer = DispatchSource.makeTimerSource(queue: queue)
          timer.schedule(deadline: .now() + .milliseconds(250), repeating: .milliseconds(250))
          timer.setEventHandler { [weak self] in self?.refresh() }
          fallbackTimer = timer
          timer.resume()
          return
        }
        let flags = Glibc.fcntl(fd, F_GETFL, 0)
        if flags >= 0 { _ = Glibc.fcntl(fd, F_SETFL, flags | O_NONBLOCK) }
        let fdFlags = Glibc.fcntl(fd, F_GETFD, 0)
        if fdFlags >= 0 { _ = Glibc.fcntl(fd, F_SETFD, fdFlags | FD_CLOEXEC) }

        inotifyFd = fd
        syncLinuxWatches()

        let source = DispatchSource.makeReadSource(fileDescriptor: fd, queue: queue)
        source.setEventHandler { [weak self] in
          guard let self, !self.stopped, self.inotifyFd >= 0 else { return }
          var buffer = [UInt8](repeating: 0, count: 4096)
          while Glibc.read(self.inotifyFd, &buffer, buffer.count) > 0 {}
          self.syncLinuxWatches()
          self.scheduleRefresh()
        }
        source.setCancelHandler { [weak self] in
          if let fd = self?.inotifyFd, fd >= 0 {
            _ = Glibc.close(fd)
            self?.inotifyFd = -1
          }
        }
        readSource = source
        source.resume()
      }

      private func syncLinuxWatches() {
        guard inotifyFd >= 0 else { return }
        var dirs: [String] = []
        for dir in candidates {
          dirs.append(dir)
          dirs.append("\(dir)/theme")
        }
        if let userConfig { dirs.append(userConfig) }
        for dir in dirs where FileManager.default.fileExists(atPath: dir) {
          dir.withCString { cPath in
            _ = linux_inotify_add_watch(inotifyFd, cPath, Self.inotifyMask)
          }
        }
      }
    #else
      private func startPolling() {
        let timer = DispatchSource.makeTimerSource(queue: queue)
        timer.schedule(deadline: .now() + .milliseconds(150), repeating: .milliseconds(150))
        timer.setEventHandler { [weak self] in self?.refresh() }
        pollTimer = timer
        timer.resume()
      }
    #endif
  }
#endif
