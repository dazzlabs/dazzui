import SwiftUI

/// Standard layout measurements for custom desktop window chrome.
public enum WindowChromeMetrics {
  /// Width occupied by the macOS traffic-light buttons at the top-left of a
  /// hidden-titlebar window.
  public static let trafficLightInset: CGFloat = 56

  /// Recommended leading clearance inside a full-bleed titlebar/toolbar band
  /// so the first control starts clear of the macOS traffic-light hit frames.
  public static let trafficLightClearance: CGFloat = 78

  /// Platform-appropriate leading clearance (`78pt` on macOS, `0pt` elsewhere).
  public static var platformTrafficLightClearance: CGFloat {
    #if os(macOS)
      return trafficLightClearance
    #else
      return 0
    #endif
  }

  /// Standard width of a single Windows 11 caption button (Minimize/Maximize/Close).
  public static let windowsCaptionButtonWidth: CGFloat = 46
}

/// The titlebar's window title: 13pt bold display face tightened by `-0.01em`.
public struct WindowTitle: View {
  public var text: String

  @Environment(\.theme) private var theme

  public init(_ text: String) {
    self.text = text
  }

  public var body: some View {
    Text(text)
      .font(.system(size: 13, weight: .bold))
      .tracking(-0.13)
      .foregroundStyle(theme.vars.colorContent)
      .lineLimit(1)
      .fixedSize()
  }
}

/// De-emphasised context label placed after a ``WindowTitle``.
public struct WindowSubtitle: View {
  public var text: String

  @Environment(\.theme) private var theme

  public init(_ text: String) {
    self.text = text
  }

  public var body: some View {
    Text(text)
      .font(theme.vars.bodySmall.font)
      .foregroundStyle(theme.vars.colorContentSubtle)
      .lineLimit(1)
      .truncationMode(.tail)
  }
}

/// Custom window titlebar band built from DazzUI's chrome surface, titlebar
/// height (`frameTitlebarSize`), and bottom hairline.
public struct WindowTitlebar<Leading: View, Content: View>: View {
  public var title: String?
  public var subtitle: String?
  /// Extra leading clearance reserved for native window controls (for example,
  /// ``WindowChromeMetrics/trafficLightClearance`` on macOS).
  public var leadingClearance: CGFloat
  /// Overrides the height (`vars.frameTitlebarSize` by default).
  public var height: CGFloat?
  /// Enables native titlebar drag and double-click behaviour on bare chrome.
  public var draggable: Bool
  @ViewBuilder public var leading: () -> Leading
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    title: String? = nil,
    subtitle: String? = nil,
    leadingClearance: CGFloat = 0,
    height: CGFloat? = nil,
    draggable: Bool = true,
    @ViewBuilder leading: @escaping () -> Leading,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.title = title
    self.subtitle = subtitle
    self.leadingClearance = leadingClearance
    self.height = height
    self.draggable = draggable
    self.leading = leading
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars
    let bar = HStack(spacing: vars.spacing35) {
      if leadingClearance > 0 {
        Spacer().frame(width: leadingClearance)
      }
      leading()
      if let title {
        WindowTitle(title)
      }
      if let subtitle {
        WindowSubtitle(subtitle)
      }
      content()
    }
    .padding(.horizontal, vars.spacing4)
    .frame(height: height ?? vars.frameTitlebarSize)
    .frame(maxWidth: .infinity, alignment: .leading)

    Group {
      if draggable {
        bar.windowDragRegion()
      } else {
        bar
      }
    }
    .background(vars.colorSurfaceChrome)
    .rule(vars.colorBorder, edge: .bottom, width: hairline)
  }
}

public extension WindowTitlebar where Leading == EmptyView {
  init(
    title: String? = nil,
    subtitle: String? = nil,
    leadingClearance: CGFloat = 0,
    height: CGFloat? = nil,
    draggable: Bool = true,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      leadingClearance: leadingClearance,
      height: height,
      draggable: draggable,
      leading: { EmptyView() },
      content: content
    )
  }
}

public extension WindowTitlebar where Leading == EmptyView, Content == EmptyView {
  init(
    title: String? = nil,
    subtitle: String? = nil,
    leadingClearance: CGFloat = 0,
    height: CGFloat? = nil,
    draggable: Bool = true
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      leadingClearance: leadingClearance,
      height: height,
      draggable: draggable,
      leading: { EmptyView() },
      content: { EmptyView() }
    )
  }
}

/// Content-pane toolbar strip that matches ``WindowTitlebar``'s height, chrome
/// surface, and bottom hairline, with an optional trailing slot (for actions
/// or custom window caption buttons on Windows/Linux).
public struct WindowToolbar<Leading: View, Content: View, Trailing: View>: View {
  public var title: String?
  public var subtitle: String?
  public var leadingClearance: CGFloat
  public var height: CGFloat?
  public var draggable: Bool
  @ViewBuilder public var leading: () -> Leading
  @ViewBuilder public var content: () -> Content
  @ViewBuilder public var trailing: () -> Trailing

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    title: String? = nil,
    subtitle: String? = nil,
    leadingClearance: CGFloat = 0,
    height: CGFloat? = nil,
    draggable: Bool = true,
    @ViewBuilder leading: @escaping () -> Leading,
    @ViewBuilder content: @escaping () -> Content,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.title = title
    self.subtitle = subtitle
    self.leadingClearance = leadingClearance
    self.height = height
    self.draggable = draggable
    self.leading = leading
    self.content = content
    self.trailing = trailing
  }

  public var body: some View {
    let vars = theme.vars
    let main = HStack(spacing: vars.spacing35) {
      if leadingClearance > 0 {
        Spacer().frame(width: leadingClearance)
      }
      leading()
      if let title {
        WindowTitle(title)
      }
      if let subtitle {
        WindowSubtitle(subtitle)
      }
      content()
    }
    .padding(.horizontal, vars.spacing4)
    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .leading)

    HStack(spacing: 0) {
      if draggable {
        main.windowDragRegion()
      } else {
        main
      }
      trailing()
    }
    .frame(height: height ?? vars.frameTitlebarSize)
    .frame(maxWidth: .infinity, alignment: .leading)
    .background(vars.colorSurfaceChrome)
    .rule(vars.colorBorder, edge: .bottom, width: hairline)
  }
}

public extension WindowToolbar where Leading == EmptyView, Trailing == EmptyView {
  init(
    title: String? = nil,
    subtitle: String? = nil,
    leadingClearance: CGFloat = 0,
    height: CGFloat? = nil,
    draggable: Bool = true,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      leadingClearance: leadingClearance,
      height: height,
      draggable: draggable,
      leading: { EmptyView() },
      content: content,
      trailing: { EmptyView() }
    )
  }
}

public extension WindowToolbar where Leading == EmptyView, Content == EmptyView, Trailing == EmptyView {
  init(
    title: String? = nil,
    subtitle: String? = nil,
    leadingClearance: CGFloat = 0,
    height: CGFloat? = nil,
    draggable: Bool = true
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      leadingClearance: leadingClearance,
      height: height,
      draggable: draggable,
      leading: { EmptyView() },
      content: { EmptyView() },
      trailing: { EmptyView() }
    )
  }
}

public extension WindowToolbar where Leading == EmptyView {
  init(
    title: String? = nil,
    subtitle: String? = nil,
    leadingClearance: CGFloat = 0,
    height: CGFloat? = nil,
    draggable: Bool = true,
    @ViewBuilder content: @escaping () -> Content,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      leadingClearance: leadingClearance,
      height: height,
      draggable: draggable,
      leading: { EmptyView() },
      content: content,
      trailing: trailing
    )
  }
}

/// Footer strip along the bottom of a window pane — summary on the left,
/// actions pushed right with a `Spacer`.
public struct WindowFooter<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing4) {
      content()
    }
    .padding(.horizontal, vars.spacing5)
    .padding(.vertical, vars.spacing3)
    .frame(maxWidth: .infinity, alignment: .leading)
    .background(vars.colorSurfaceChrome)
    .rule(vars.colorBorder, edge: .top, width: hairline)
  }
}
