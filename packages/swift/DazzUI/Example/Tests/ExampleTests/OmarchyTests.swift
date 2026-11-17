import AppKit
import SwiftUI
import XCTest

@testable import DazzUI
@testable import Example

/// The run-time derivation is `ds/scripts/themes.mjs`'s, transcribed, so it is
/// held to what that script derived at build time: every built-in Omarchy
/// theme, read back from the three colours `ds/themes/omarchy.yaml` gives it,
/// has to land on the generated instance in both of its halves.
final class OmarchyTests: XCTestCase {
  func testEveryBuiltInOmarchyThemeDerivesWhatTheBuildDerived() throws {
    let themes = try readSources()
    XCTAssertEqual(themes.count, 22)

    for (name, palette) in themes {
      for half in [palette, palette.opposite] {
        let scheme = half.colorScheme == .dark ? "dark" : "light"
        let id = "omarchy-\(name)-\(scheme)"
        let themeName = try XCTUnwrap(ThemeName(rawValue: id), id)
        let expected = themeName.variables
        let actual = half.themeVariables

        XCTAssertEqual(
          colors(actual).map(argbHex),
          colors(expected).map(argbHex),
          id
        )
        for step in [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950] {
          XCTAssertEqual(
            argbHex(actual.colorPrimary[step]),
            argbHex(expected.colorPrimary[step]),
            "\(id) primary \(step)"
          )
        }
        XCTAssertEqual(actual.radiusMedium, expected.radiusMedium, id)
        XCTAssertEqual(typeMetrics(actual), typeMetrics(expected), id)
      }
    }
  }

  func testShellStyleDefaultMatchesGeneratedOmarchyTypography() {
    XCTAssertEqual(
      typeMetrics(OmarchyShellStyle().apply(to: .brightLight)),
      typeMetrics(.omarchyTokyoNightDark)
    )
  }

  func testShellStyleFollowsBaseSize() {
    let scale = OmarchyShellStyle.fromShellToml(["[font]\nbase-size = 15"])
    XCTAssertEqual(
      [scale.caption, scale.bodySmall, scale.body, scale.subtitle],
      [12, 14, 15, 16]
    )
    let vars = scale.apply(to: .brightLight)
    XCTAssertEqual(vars.bodyMedium.size, 15)
    XCTAssertEqual(vars.bodyMedium.lineHeight, 20)
    XCTAssertEqual(vars.labelMedium.size, 15)
    XCTAssertEqual(vars.labelMedium.lineHeight, 15)
    XCTAssertEqual(vars.captionSmall.size, 12)
    // Headlines are not the shell's to move.
    XCTAssertEqual(vars.headlineSmall, ThemeVariables.brightLight.headlineSmall)
  }

  func testShellStyleTakesPinnedStepAndUsersFileOverThemes() {
    let scale = OmarchyShellStyle.fromShellToml([
      "[font]\nbase-size = 12\ntitle = 20 # big\n[spacing]\nscale = 2",
      "[font]\nbase-size = 14",
    ])
    XCTAssertEqual(scale.fontBaseSize, 14)
    XCTAssertEqual(scale.title, 20)
    XCTAssertEqual(scale.body, 14)
  }

  func testShellStyleScalesEveryLengthWithBaseSize() {
    let vars = OmarchyShellStyle.fromShellToml(["[font]\nbase-size = 16"])
      .apply(to: .omarchyTokyoNightDark)
    // 16 / 12 of the shell's own: a 28px control is 37, its 10px inset 13.
    XCTAssertEqual(vars.controlMediumSize, 37)
    XCTAssertEqual(vars.controlMediumPaddingInline, 13)
    XCTAssertEqual(vars.spacing2, 11)
    XCTAssertEqual(vars.controlMediumGap, 11)
    XCTAssertEqual(vars.iconMedium, 21)
    XCTAssertEqual(vars.spacing0, 0)
    // Not lengths the shell scales.
    XCTAssertEqual(vars.radiusFull, ThemeVariables.omarchyTokyoNightDark.radiusFull)
    XCTAssertEqual(vars.strokeControl, 1.5)
  }

  func testShellStyleTakesSpacingScaleScaleWithFontAndPinnedControls() {
    let style = OmarchyShellStyle.fromShellToml([
      "[font]\nbase-size = 16\n"
        + "[spacing]\nscale = 1.5\nscale-with-font = false\n"
        + "control-height = 40\n"
    ])
    XCTAssertEqual(style.lengthScale, 1.5)
    let vars = style.apply(to: .omarchyTokyoNightDark)
    XCTAssertEqual(vars.controlMediumSize, 40)
    XCTAssertEqual(vars.controlMediumPaddingInline, 15)
    XCTAssertEqual(vars.spacing4, 24)
  }

  func testShellStyleAtDefaultMovesNoLength() {
    let vars = OmarchyShellStyle().apply(to: .omarchyTokyoNightDark)
    XCTAssertEqual(vars.controlMediumSize, 28)
    XCTAssertEqual(vars.controlMediumPaddingInline, 10)
    XCTAssertEqual(vars.spacing3, 12)
    XCTAssertEqual(vars.frameSidebarWidth, 172)
  }

  func testShellStyleWithoutFontSectionIsDefault() {
    XCTAssertEqual(
      OmarchyShellStyle.fromShellToml(["[bar]\nsize-horizontal = 26"]),
      OmarchyShellStyle()
    )
  }

  func testColorsTomlReadsThreeColoursAndMode() throws {
    let palette = try XCTUnwrap(
      OmarchyPalette.parse(
        """
        mode = "dark"
        accent = "#b59790"
        background = "#0c0b0c"
        foreground = "#FAFCFB"
        hyprland_active_border = "rgba(8a8588ee) rgba(e2dddcee)"
        """,
        name: "last-horizon"
      )
    )
    XCTAssertEqual(argbHex(palette.background), "0xFF0C0B0C")
    XCTAssertEqual(argbHex(palette.foreground), "0xFFFAFCFB")
    XCTAssertEqual(argbHex(palette.accent), "0xFFB59790")
    XCTAssertEqual(palette.colorScheme, .dark)
    XCTAssertEqual(palette.title, "Last Horizon")
  }

  func testColorsTomlWithoutModeTakesGivenOrPaperLuminance() throws {
    let toml = "background = \"#eff1f5\"\nforeground = \"#4c4f69\"\naccent = \"#1e66f5\""
    XCTAssertEqual(try XCTUnwrap(OmarchyPalette.parse(toml)).colorScheme, .light)
    XCTAssertEqual(
      try XCTUnwrap(OmarchyPalette.parse(toml, colorScheme: .dark)).colorScheme,
      .dark
    )
    XCTAssertNil(OmarchyPalette.parse("background = \"#000000\""))
  }

  func testCurrentThemeIsReadAndFollowed() async throws {
    let current = try makeTempDirectory()
    defer { try? FileManager.default.removeItem(at: current) }
    let themeDir = current.appendingPathComponent("theme")
    try FileManager.default.createDirectory(at: themeDir, withIntermediateDirectories: true)

    func wear(_ name: String, _ background: String) throws {
      try """
      background = "\(background)"
      foreground = "#cdd6f4"
      accent = "#89b4fa"
      """.write(
        to: themeDir.appendingPathComponent("colors.toml"),
        atomically: true,
        encoding: .utf8
      )
      try "\(name)\n".write(
        to: current.appendingPathComponent("theme.name"),
        atomically: true,
        encoding: .utf8
      )
    }

    try wear("catppuccin", "#1e1e2e")
    let initial = readOmarchyPalette(directory: current.path)
    XCTAssertEqual(initial?.name, "catppuccin")
    XCTAssertEqual(initial.map { argbHex($0.background) }, "0xFF1E1E2E")
    XCTAssertEqual(initial?.shellStyle, OmarchyShellStyle())
    XCTAssertNil(readOmarchyPalette(directory: current.appendingPathComponent("missing").path))

    // Theme + user shell.toml merge, user winning.
    try "[font]\nbase-size = 13\ncaption = 9\n".write(
      to: themeDir.appendingPathComponent("shell.toml"),
      atomically: true,
      encoding: .utf8
    )
    let configDir = current.appendingPathComponent("config")
    try FileManager.default.createDirectory(at: configDir, withIntermediateDirectories: true)
    try "[font]\nbase-size = 16\n".write(
      to: configDir.appendingPathComponent("shell.toml"),
      atomically: true,
      encoding: .utf8
    )
    let merged = readOmarchyPalette(directory: current.path, config: configDir.path)
    XCTAssertEqual(
      merged?.shellStyle,
      OmarchyShellStyle(fontBaseSize: 16, fontOverrides: ["caption": 9])
    )
    XCTAssertEqual(merged?.themeVariables.bodyMedium.size, 16)

    // Watch theme swap and shell.toml edit.
    try FileManager.default.removeItem(at: configDir.appendingPathComponent("shell.toml"))
    try FileManager.default.removeItem(at: themeDir.appendingPathComponent("shell.toml"))

    let stream = watchOmarchyPalette(directory: current.path, config: configDir.path)
    var iterator = stream.makeAsyncIterator()

    let first = await iterator.next() ?? nil
    XCTAssertEqual(first?.name, "catppuccin")
    XCTAssertEqual(first?.shellStyle.fontBaseSize, 12)

    try "[font]\nbase-size = 14\n".write(
      to: configDir.appendingPathComponent("shell.toml"),
      atomically: true,
      encoding: .utf8
    )
    let afterShell = await iterator.next() ?? nil
    XCTAssertEqual(afterShell?.shellStyle.fontBaseSize, 14)

    // As `omarchy-theme-set` does it: the theme directory swapped whole, then
    // the name written.
    try FileManager.default.removeItem(at: themeDir)
    try FileManager.default.createDirectory(at: themeDir, withIntermediateDirectories: true)
    try wear("tokyo-night", "#1a1b26")

    let afterSwap = await iterator.next() ?? nil
    XCTAssertEqual(afterSwap?.name, "tokyo-night")
    XCTAssertEqual(afterSwap.map { argbHex($0.background) }, "0xFF1A1B26")
  }

  @MainActor
  func testOmarchyThemeViewAndAppViewFollowDirectory() throws {
    let current = try makeTempDirectory()
    defer { try? FileManager.default.removeItem(at: current) }
    let themeDir = current.appendingPathComponent("theme")
    try FileManager.default.createDirectory(at: themeDir, withIntermediateDirectories: true)
    try """
    mode = "dark"
    background = "#1a1b26"
    foreground = "#a9b1d6"
    accent = "#7aa2f7"
    """.write(
      to: themeDir.appendingPathComponent("colors.toml"),
      atomically: true,
      encoding: .utf8
    )
    try "tokyo-night\n".write(
      to: current.appendingPathComponent("theme.name"),
      atomically: true,
      encoding: .utf8
    )

    var capturedPalette: OmarchyPalette?
    var capturedTheme: Theme?

    struct Probe: View {
      let report: (OmarchyPalette?, Theme) -> Void
      @Environment(\.omarchyPalette) private var palette
      @Environment(\.theme) private var theme
      var body: some View {
        let _ = report(palette, theme)
        Color.clear.frame(width: 20, height: 20)
      }
    }

    let presentHost = NSHostingView(
      rootView: OmarchyTheme(fallback: .studioLight, directory: current.path) {
        Probe { capturedPalette = $0; capturedTheme = $1 }
      }
    )
    presentHost.frame = NSRect(x: 0, y: 0, width: 40, height: 40)
    let window = NSWindow(
      contentRect: presentHost.frame,
      styleMask: [.borderless],
      backing: .buffered,
      defer: false
    )
    window.isReleasedWhenClosed = false
    defer { window.close() }
    window.contentView = presentHost
    window.makeKeyAndOrderFront(nil)
    RunLoop.main.run(until: Date().addingTimeInterval(0.05))
    presentHost.layoutSubtreeIfNeeded()

    XCTAssertEqual(capturedPalette?.name, "tokyo-night")
    XCTAssertEqual(capturedPalette?.title, "Tokyo Night")
    XCTAssertEqual(capturedTheme?.colorScheme, .dark)
    XCTAssertEqual(
      capturedTheme.map { argbHex($0.vars.colorSurface) },
      argbHex(ThemeVariables.omarchyTokyoNightDark.colorSurface)
    )

    let missingHost = NSHostingView(
      rootView: OmarchyTheme(
        fallback: .studioLight,
        directory: current.appendingPathComponent("missing").path
      ) {
        Probe { capturedPalette = $0; capturedTheme = $1 }
      }
    )
    missingHost.frame = presentHost.frame
    window.contentView = missingHost
    RunLoop.main.run(until: Date().addingTimeInterval(0.05))
    missingHost.layoutSubtreeIfNeeded()

    XCTAssertNil(capturedPalette)
    XCTAssertEqual(capturedTheme, Theme(.studioLight))

    // AppView renders cleanly with a live Omarchy directory.
    let appHost = NSHostingView(
      rootView: AppView(omarchyDirectory: current.path)
        .frame(width: 960, height: 600)
    )
    appHost.frame = NSRect(x: 0, y: 0, width: 960, height: 600)
    window.setContentSize(NSSize(width: 960, height: 600))
    window.contentView = appHost
    RunLoop.main.run(until: Date().addingTimeInterval(0.05))
    appHost.layoutSubtreeIfNeeded()
    XCTAssertNotNil(appHost.bitmapImageRepForCachingDisplay(in: appHost.bounds))
  }

  private func makeTempDirectory() throws -> URL {
    let url = FileManager.default.temporaryDirectory
      .appendingPathComponent("dazzui-omarchy-\(UUID().uuidString)")
    try FileManager.default.createDirectory(at: url, withIntermediateDirectories: true)
    return url
  }

  private func argbHex(_ color: Color) -> String {
    let b = color.srgbBytes
    return String(format: "0x%02X%02X%02X%02X", b.a, b.r, b.g, b.b)
  }

  private func typeMetrics(_ v: ThemeVariables) -> [CGFloat] {
    [
      v.titleSmall.size, v.titleSmall.lineHeight,
      v.titleMedium.size, v.titleMedium.lineHeight,
      v.titleLarge.size, v.titleLarge.lineHeight,
      v.bodySmall.size, v.bodySmall.lineHeight,
      v.bodyMedium.size, v.bodyMedium.lineHeight,
      v.bodyLarge.size, v.bodyLarge.lineHeight,
      v.labelQuiet.size, v.labelQuiet.lineHeight,
      v.labelStrong.size, v.labelStrong.lineHeight,
      v.labelSmall.size, v.labelSmall.lineHeight,
      v.labelMedium.size, v.labelMedium.lineHeight,
      v.labelLarge.size, v.labelLarge.lineHeight,
      v.captionSmall.size, v.captionSmall.lineHeight,
      v.captionMedium.size, v.captionMedium.lineHeight,
      v.captionLarge.size, v.captionLarge.lineHeight,
    ]
  }

  private func colors(_ v: ThemeVariables) -> [Color] {
    [
      v.colorCanvas,
      v.colorSurface,
      v.colorSurfaceMuted,
      v.colorSurfaceSunken,
      v.colorSurfaceSubtle,
      v.colorSurfaceInset,
      v.colorSurfaceRaised,
      v.colorSurfaceOverlay,
      v.colorSurfaceChrome,
      v.colorSurfaceColumn,
      v.colorContent,
      v.colorContentSecondary,
      v.colorContentNav,
      v.colorContentMuted,
      v.colorContentSubtle,
      v.colorContentFaint,
      v.colorBorder,
      v.colorBorderStrong,
      v.colorBorderMuted,
      v.colorOnAccent,
      v.progressGradientFrom,
      v.progressGradientTo,
    ]
  }

  private func readSources() throws -> [(String, OmarchyPalette)] {
    let root = URL(fileURLWithPath: #filePath)
      .deletingLastPathComponent()
      .deletingLastPathComponent()
      .deletingLastPathComponent()
      .deletingLastPathComponent()
      .deletingLastPathComponent()
      .deletingLastPathComponent()
      .deletingLastPathComponent()
    let yamlURL = root.appendingPathComponent("ds/themes/omarchy.yaml")
    let source = try String(contentsOf: yamlURL, encoding: .utf8)

    var themes: [(String, OmarchyPalette)] = []
    var currentName: String?
    var fields: [String: String] = [:]

    func flush() throws {
      guard let name = currentName else { return }
      let toml = fields.map { "\($0.key) = \"\($0.value)\"" }.joined(separator: "\n")
      let palette = try XCTUnwrap(OmarchyPalette.parse(toml, name: name), name)
      themes.append((name, palette))
    }

    var inThemes = false
    for rawLine in source.split(separator: "\n", omittingEmptySubsequences: false) {
      let line = rawLine.trimmingCharacters(in: CharacterSet(charactersIn: "\r"))
      if line.hasPrefix("themes:") {
        inThemes = true
        continue
      }
      if !inThemes { continue }
      if line.hasPrefix("  omarchy-"), line.hasSuffix(":") {
        try flush()
        let start = line.index(line.startIndex, offsetBy: "  omarchy-".count)
        let end = line.index(before: line.endIndex)
        currentName = String(line[start..<end])
        fields = [:]
        continue
      }
      if line.hasPrefix("    "), let colon = line.firstIndex(of: ":") {
        let rawKey = line[line.index(line.startIndex, offsetBy: 4)..<colon]
          .trimmingCharacters(in: .whitespaces)
        var rawValue = line[line.index(after: colon)...]
          .trimmingCharacters(in: .whitespaces)
        if rawValue.hasPrefix("\""), rawValue.hasSuffix("\""), rawValue.count >= 2 {
          rawValue.removeFirst()
          rawValue.removeLast()
        }
        fields[rawKey == "scheme" ? "mode" : rawKey] = rawValue
      }
    }
    try flush()
    return themes
  }
}
