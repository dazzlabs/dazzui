import Foundation
import SwiftUI

// MARK: - Windows Caption Buttons

/// Windows 11-style window caption controls (Minimize, Maximize/Restore, Close)
/// for frameless / hidden-titlebar windows.
public struct WindowsCaptionButtons: View {
  public var isMaximized: Bool
  public var isFocused: Bool
  public var isFullScreen: Bool
  public var height: CGFloat?
  public var buttonWidth: CGFloat
  public var minimizeLabel: String
  public var maximizeLabel: String
  public var restoreLabel: String
  public var closeLabel: String
  public var onMinimize: () -> Void
  public var onToggleMaximize: () -> Void
  public var onClose: () -> Void

  @Environment(\.theme) private var theme

  public init(
    isMaximized: Bool = false,
    isFocused: Bool = true,
    isFullScreen: Bool = false,
    height: CGFloat? = nil,
    buttonWidth: CGFloat = WindowChromeMetrics.windowsCaptionButtonWidth,
    minimizeLabel: String = "Minimize window",
    maximizeLabel: String = "Maximize window",
    restoreLabel: String = "Restore window",
    closeLabel: String = "Close window",
    onMinimize: @escaping () -> Void,
    onToggleMaximize: @escaping () -> Void,
    onClose: @escaping () -> Void
  ) {
    self.isMaximized = isMaximized
    self.isFocused = isFocused
    self.isFullScreen = isFullScreen
    self.height = height
    self.buttonWidth = buttonWidth
    self.minimizeLabel = minimizeLabel
    self.maximizeLabel = maximizeLabel
    self.restoreLabel = restoreLabel
    self.closeLabel = closeLabel
    self.onMinimize = onMinimize
    self.onToggleMaximize = onToggleMaximize
    self.onClose = onClose
  }

  public var body: some View {
    if isFullScreen {
      EmptyView()
    } else {
      let resolvedHeight = max(0, (height ?? theme.vars.frameTitlebarSize) - theme.vars.strokeHairline)
      HStack(spacing: 0) {
        captionButton(
          kind: .minimize,
          label: minimizeLabel,
          closeRole: false,
          height: resolvedHeight,
          action: onMinimize
        )
        captionButton(
          kind: isMaximized ? .restore : .maximize,
          label: isMaximized ? restoreLabel : maximizeLabel,
          closeRole: false,
          height: resolvedHeight,
          action: onToggleMaximize
        )
        captionButton(
          kind: .close,
          label: closeLabel,
          closeRole: true,
          height: resolvedHeight,
          action: onClose
        )
      }
      .frame(height: resolvedHeight, alignment: .top)
    }
  }

  private func captionButton(
    kind: WindowsCaptionGlyph.Kind,
    label: String,
    closeRole: Bool,
    height: CGFloat,
    action: @escaping () -> Void
  ) -> some View {
    let vars = theme.vars
    let closeHover = Color(red: 0xc4 / 255.0, green: 0x2b / 255.0, blue: 0x1c / 255.0)
    let closePress = Color(red: 0xb1 / 255.0, green: 0x26 / 255.0, blue: 0x18 / 255.0)

    return Pressable(action: action, cornerRadius: 0) { state in
      let lit = state.hovered || state.pressed
      let foreground: Color =
        closeRole && lit
        ? .white
        : (isFocused || lit ? vars.colorContent : vars.colorContentFaint)
      let background: Color =
        !lit
        ? .clear
        : closeRole
          ? (state.pressed ? closePress : closeHover)
          : (state.pressed ? vars.colorSurfaceMuted : vars.colorSurfaceSubtle)

      WindowsCaptionGlyph(kind: kind)
        .stroke(foreground, lineWidth: 1)
        .frame(width: 10, height: 10)
        .frame(width: buttonWidth, height: height)
        .background(background)
        .animation(vars.motion, value: lit)
    }
    .accessibilityLabel(label)
  }
}

/// Standalone Windows 11 close button for modal / utility windows without
/// minimize or maximize controls.
public struct WindowsCaptionCloseButton: View {
  public var height: CGFloat?
  public var buttonWidth: CGFloat
  public var label: String
  public var action: () -> Void

  @Environment(\.theme) private var theme

  public init(
    height: CGFloat? = nil,
    buttonWidth: CGFloat = WindowChromeMetrics.windowsCaptionButtonWidth,
    label: String = "Close window",
    action: @escaping () -> Void
  ) {
    self.height = height
    self.buttonWidth = buttonWidth
    self.label = label
    self.action = action
  }

  public var body: some View {
    let vars = theme.vars
    let resolvedHeight = max(0, (height ?? vars.frameTitlebarSize) - vars.strokeHairline)
    let closeHover = Color(red: 0xc4 / 255.0, green: 0x2b / 255.0, blue: 0x1c / 255.0)
    let closePress = Color(red: 0xb1 / 255.0, green: 0x26 / 255.0, blue: 0x18 / 255.0)

    Pressable(action: action, cornerRadius: 0) { state in
      let lit = state.hovered || state.pressed
      WindowsCaptionGlyph(kind: .close)
        .stroke(lit ? .white : vars.colorContent, lineWidth: 1)
        .frame(width: 10, height: 10)
        .frame(width: buttonWidth, height: resolvedHeight)
        .background(lit ? (state.pressed ? closePress : closeHover) : .clear)
        .animation(vars.motion, value: lit)
    }
    .accessibilityLabel(label)
  }
}

private struct WindowsCaptionGlyph: Shape {
  enum Kind: Equatable {
    case minimize
    case maximize
    case restore
    case close
  }

  let kind: Kind

  func path(in rect: CGRect) -> Path {
    let x = rect.minX
    let y = rect.minY
    var path = Path()
    switch kind {
    case .minimize:
      path.move(to: CGPoint(x: x, y: y + 5.5))
      path.addLine(to: CGPoint(x: x + 10, y: y + 5.5))
    case .maximize:
      path.addPath(
        Path(
          roundedRect: CGRect(x: x + 0.5, y: y + 0.5, width: 9, height: 9),
          cornerRadius: 1
        )
      )
    case .restore:
      path.addPath(
        Path(
          roundedRect: CGRect(x: x + 0.5, y: y + 2.5, width: 7, height: 7),
          cornerRadius: 1
        )
      )
      path.move(to: CGPoint(x: x + 2.5, y: y + 2.5))
      path.addLine(to: CGPoint(x: x + 2.5, y: y + 1.5))
      path.addArc(
        center: CGPoint(x: x + 3.5, y: y + 1.5),
        radius: 1,
        startAngle: .degrees(180),
        endAngle: .degrees(270),
        clockwise: false
      )
      path.addLine(to: CGPoint(x: x + 8.5, y: y + 0.5))
      path.addArc(
        center: CGPoint(x: x + 8.5, y: y + 1.5),
        radius: 1,
        startAngle: .degrees(270),
        endAngle: .degrees(360),
        clockwise: false
      )
      path.addLine(to: CGPoint(x: x + 9.5, y: y + 6.5))
      path.addArc(
        center: CGPoint(x: x + 8.5, y: y + 6.5),
        radius: 1,
        startAngle: .degrees(0),
        endAngle: .degrees(90),
        clockwise: false
      )
      path.addLine(to: CGPoint(x: x + 7.5, y: y + 7.5))
    case .close:
      path.move(to: CGPoint(x: x + 0.5, y: y + 0.5))
      path.addLine(to: CGPoint(x: x + 9.5, y: y + 9.5))
      path.move(to: CGPoint(x: x + 9.5, y: y + 0.5))
      path.addLine(to: CGPoint(x: x + 0.5, y: y + 9.5))
    }
    return path
  }
}

// MARK: - Linux Window Control Buttons

/// Linux desktop environment window-control glyph and geometry styles.
public enum LinuxDesktopStyle: String, CaseIterable, Equatable, Sendable {
  case ubuntu
  case gnome
  case kde
  case omarchy

  /// Detects the active Linux desktop style from environment variables and
  /// `/etc/os-release`.
  public static func detect(
    environment: [String: String] = ProcessInfo.processInfo.environment
  ) -> LinuxDesktopStyle {
    if let override = environment["DAZZUI_LINUX_DESKTOP"]?
      .trimmingCharacters(in: .whitespacesAndNewlines)
      .lowercased(),
      let parsed = LinuxDesktopStyle(rawValue: override)
    {
      return parsed
    }

    let desktopTokens = [
      environment["XDG_CURRENT_DESKTOP"],
      environment["XDG_SESSION_DESKTOP"],
      environment["DESKTOP_SESSION"],
      environment["GDMSESSION"],
    ]
    .compactMap { $0?.lowercased() }
    .joined(separator: ":")

    let osReleaseID = readOSReleaseID()

    if environment["OMARCHY_PATH"] != nil
      || environment["OMARCHY_STATE_DIR"] != nil
      || desktopTokens.contains("omarchy")
      || desktopTokens.contains("hyprland")
      || environment["HYPRLAND_INSTANCE_SIGNATURE"] != nil
      || osReleaseID == "omarchy"
    {
      return .omarchy
    }

    if environment["KDE_FULL_SESSION"] != nil
      || desktopTokens.contains("kde")
      || desktopTokens.contains("plasma")
    {
      return .kde
    }

    if desktopTokens.contains("ubuntu") || osReleaseID == "ubuntu" {
      return .ubuntu
    }

    return .gnome
  }

  private static func readOSReleaseID() -> String? {
    guard let contents = try? String(contentsOfFile: "/etc/os-release", encoding: .utf8) else {
      return nil
    }
    for line in contents.split(whereSeparator: \.isNewline) {
      let trimmed = line.trimmingCharacters(in: .whitespaces)
      guard trimmed.hasPrefix("ID=") else { continue }
      let value = trimmed.dropFirst(3).trimmingCharacters(in: CharacterSet(charactersIn: "\"' "))
      return value.lowercased()
    }
    return nil
  }
}

/// Linux window control buttons adapting glyphs, circular/square geometry,
/// and hover treatments across Adwaita/GNOME, Ubuntu, KDE Breeze, and Omarchy.
public struct LinuxWindowControlButtons: View {
  public var style: LinuxDesktopStyle
  public var isMaximized: Bool
  public var isFocused: Bool
  public var isFullScreen: Bool
  public var height: CGFloat?
  public var minimizeLabel: String
  public var maximizeLabel: String
  public var restoreLabel: String
  public var closeLabel: String
  public var onMinimize: () -> Void
  public var onToggleMaximize: () -> Void
  public var onClose: () -> Void

  @Environment(\.theme) private var theme

  public init(
    style: LinuxDesktopStyle = .detect(),
    isMaximized: Bool = false,
    isFocused: Bool = true,
    isFullScreen: Bool = false,
    height: CGFloat? = nil,
    minimizeLabel: String = "Minimize window",
    maximizeLabel: String = "Maximize window",
    restoreLabel: String = "Restore window",
    closeLabel: String = "Close window",
    onMinimize: @escaping () -> Void,
    onToggleMaximize: @escaping () -> Void,
    onClose: @escaping () -> Void
  ) {
    self.style = style
    self.isMaximized = isMaximized
    self.isFocused = isFocused
    self.isFullScreen = isFullScreen
    self.height = height
    self.minimizeLabel = minimizeLabel
    self.maximizeLabel = maximizeLabel
    self.restoreLabel = restoreLabel
    self.closeLabel = closeLabel
    self.onMinimize = onMinimize
    self.onToggleMaximize = onToggleMaximize
    self.onClose = onClose
  }

  public var body: some View {
    if isFullScreen {
      EmptyView()
    } else {
      let resolvedHeight = max(0, (height ?? theme.vars.frameTitlebarSize) - theme.vars.strokeHairline)
      HStack(spacing: buttonSpacing) {
        controlButton(
          kind: .minimize,
          label: minimizeLabel,
          closeRole: false,
          action: onMinimize
        )
        controlButton(
          kind: isMaximized ? .restore : .maximize,
          label: isMaximized ? restoreLabel : maximizeLabel,
          closeRole: false,
          action: onToggleMaximize
        )
        controlButton(
          kind: .close,
          label: closeLabel,
          closeRole: true,
          action: onClose
        )
      }
      .padding(.trailing, trailingInset)
      .frame(height: resolvedHeight)
    }
  }

  private var buttonSpacing: CGFloat {
    switch style {
    case .omarchy, .kde: return 6
    case .ubuntu, .gnome: return 12
    }
  }

  private var trailingInset: CGFloat {
    switch style {
    case .omarchy: return 10
    case .kde: return 8
    case .ubuntu, .gnome: return 12
    }
  }

  private var buttonSize: CGFloat {
    switch style {
    case .kde: return 20
    case .omarchy, .ubuntu, .gnome: return 24
    }
  }

  private var buttonCornerRadius: CGFloat {
    switch style {
    case .omarchy: return 0
    case .ubuntu, .gnome, .kde: return buttonSize / 2
    }
  }

  private var strokeWidth: CGFloat {
    switch style {
    case .omarchy: return 1.25
    case .ubuntu, .gnome, .kde: return 1.5
    }
  }

  private func controlButton(
    kind: LinuxWindowControlGlyph.Kind,
    label: String,
    closeRole: Bool,
    action: @escaping () -> Void
  ) -> some View {
    let vars = theme.vars
    let radius = buttonCornerRadius

    return Pressable(action: action, cornerRadius: radius) { state in
      let lit = state.hovered || state.pressed
      let foreground = foregroundColor(vars: vars, state: state, closeRole: closeRole)
      let background = backgroundColor(
        vars: vars, state: state, foreground: foreground, closeRole: closeRole
      )

      LinuxWindowControlGlyph(kind: kind, style: style)
        .stroke(foreground, lineWidth: strokeWidth)
        .frame(width: 16, height: 16)
        .frame(width: buttonSize, height: buttonSize)
        .background(background)
        .clipShape(RoundedRectangle(cornerRadius: radius, style: .circular))
        .animation(vars.motion, value: lit)
    }
    .accessibilityLabel(label)
  }

  private func foregroundColor(
    vars: ThemeVariables,
    state: PressableState,
    closeRole: Bool
  ) -> Color {
    let lit = state.hovered || state.pressed
    switch style {
    case .omarchy:
      if closeRole && lit { return vars.dangerText }
      if lit { return vars.colorContent }
      return isFocused ? vars.colorContentSubtle : vars.colorContentFaint
    case .ubuntu, .gnome, .kde:
      return isFocused || lit ? vars.colorContent : vars.colorContentFaint
    }
  }

  private func backgroundColor(
    vars: ThemeVariables,
    state: PressableState,
    foreground: Color,
    closeRole: Bool
  ) -> Color {
    let lit = state.hovered || state.pressed
    switch style {
    case .omarchy:
      guard lit else { return .clear }
      if closeRole {
        return state.pressed ? vars.dangerText.opacity(0.22) : vars.dangerSurface
      }
      return state.pressed ? vars.colorSurfaceMuted : vars.colorSurfaceSubtle
    case .kde:
      guard lit else { return .clear }
      return foreground.opacity(state.pressed ? 0.24 : 0.16)
    case .ubuntu, .gnome:
      if !lit { return vars.colorSurfaceSubtle }
      return foreground.opacity(state.pressed ? 0.24 : 0.16)
    }
  }
}

private struct LinuxWindowControlGlyph: Shape {
  enum Kind: Equatable {
    case minimize
    case maximize
    case restore
    case close
  }

  let kind: Kind
  let style: LinuxDesktopStyle

  func path(in rect: CGRect) -> Path {
    let x = rect.minX
    let y = rect.minY
    var path = Path()
    switch style {
    case .omarchy:
      switch kind {
      case .minimize:
        path.move(to: CGPoint(x: x + 4, y: y + 8))
        path.addLine(to: CGPoint(x: x + 12, y: y + 8))
      case .maximize:
        path.addRect(CGRect(x: x + 4.5, y: y + 4.5, width: 7, height: 7))
      case .restore:
        path.addRect(CGRect(x: x + 4.5, y: y + 6.0, width: 5.5, height: 5.5))
        path.move(to: CGPoint(x: x + 6.5, y: y + 6.0))
        path.addLine(to: CGPoint(x: x + 6.5, y: y + 4.5))
        path.addLine(to: CGPoint(x: x + 11.5, y: y + 4.5))
        path.addLine(to: CGPoint(x: x + 11.5, y: y + 9.5))
        path.addLine(to: CGPoint(x: x + 10.0, y: y + 9.5))
      case .close:
        path.move(to: CGPoint(x: x + 4.5, y: y + 4.5))
        path.addLine(to: CGPoint(x: x + 11.5, y: y + 11.5))
        path.move(to: CGPoint(x: x + 11.5, y: y + 4.5))
        path.addLine(to: CGPoint(x: x + 4.5, y: y + 11.5))
      }
    case .kde:
      switch kind {
      case .minimize:
        path.move(to: CGPoint(x: x + 4, y: y + 6))
        path.addLine(to: CGPoint(x: x + 8, y: y + 10))
        path.addLine(to: CGPoint(x: x + 12, y: y + 6))
      case .maximize:
        path.move(to: CGPoint(x: x + 4, y: y + 10))
        path.addLine(to: CGPoint(x: x + 8, y: y + 6))
        path.addLine(to: CGPoint(x: x + 12, y: y + 10))
      case .restore:
        path.move(to: CGPoint(x: x + 8, y: y + 4.5))
        path.addLine(to: CGPoint(x: x + 11.5, y: y + 8))
        path.addLine(to: CGPoint(x: x + 8, y: y + 11.5))
        path.addLine(to: CGPoint(x: x + 4.5, y: y + 8))
        path.closeSubpath()
      case .close:
        path.move(to: CGPoint(x: x + 4.5, y: y + 4.5))
        path.addLine(to: CGPoint(x: x + 11.5, y: y + 11.5))
        path.move(to: CGPoint(x: x + 11.5, y: y + 4.5))
        path.addLine(to: CGPoint(x: x + 4.5, y: y + 11.5))
      }
    case .ubuntu, .gnome:
      switch kind {
      case .minimize:
        path.move(to: CGPoint(x: x + 4.5, y: y + 11.25))
        path.addLine(to: CGPoint(x: x + 11.5, y: y + 11.25))
      case .maximize:
        Self.addRoundedRect(
          CGRect(x: x + 4.75, y: y + 4.75, width: 6.5, height: 6.5), radius: 1, to: &path
        )
      case .restore:
        Self.addRoundedRect(
          CGRect(x: x + 4.75, y: y + 6.25, width: 5.0, height: 5.0), radius: 0.8, to: &path
        )
        path.move(to: CGPoint(x: x + 6.75, y: y + 6.25))
        path.addLine(to: CGPoint(x: x + 6.75, y: y + 4.75))
        path.addLine(to: CGPoint(x: x + 11.25, y: y + 4.75))
        path.addLine(to: CGPoint(x: x + 11.25, y: y + 9.25))
        path.addLine(to: CGPoint(x: x + 9.75, y: y + 9.25))
      case .close:
        path.move(to: CGPoint(x: x + 5, y: y + 5))
        path.addLine(to: CGPoint(x: x + 11, y: y + 11))
        path.move(to: CGPoint(x: x + 11, y: y + 5))
        path.addLine(to: CGPoint(x: x + 5, y: y + 11))
      }
    }
    return path
  }

  private static func addRoundedRect(_ rect: CGRect, radius: CGFloat, to path: inout Path) {
    let r = min(radius, rect.width / 2, rect.height / 2)
    guard r > 0 else {
      path.addRect(rect)
      return
    }
    let inset = r * (1 - 4.0 / 3.0 * (sqrt(2.0) - 1.0))
    let x = rect.minX
    let y = rect.minY
    let midX = rect.midX
    let right = rect.maxX
    let bottom = rect.maxY
    path.move(to: CGPoint(x: midX, y: y))
    path.addLine(to: CGPoint(x: right - r, y: y))
    path.addCurve(
      to: CGPoint(x: right, y: y + r),
      control1: CGPoint(x: right - inset, y: y),
      control2: CGPoint(x: right, y: y + inset)
    )
    path.addLine(to: CGPoint(x: right, y: bottom - r))
    path.addCurve(
      to: CGPoint(x: right - r, y: bottom),
      control1: CGPoint(x: right, y: bottom - inset),
      control2: CGPoint(x: right - inset, y: bottom)
    )
    path.addLine(to: CGPoint(x: x + r, y: bottom))
    path.addCurve(
      to: CGPoint(x: x, y: bottom - r),
      control1: CGPoint(x: x + inset, y: bottom),
      control2: CGPoint(x: x, y: bottom - inset)
    )
    path.addLine(to: CGPoint(x: x, y: y + r))
    path.addCurve(
      to: CGPoint(x: x + r, y: y),
      control1: CGPoint(x: x, y: y + inset),
      control2: CGPoint(x: x + inset, y: y)
    )
    path.closeSubpath()
  }
}
