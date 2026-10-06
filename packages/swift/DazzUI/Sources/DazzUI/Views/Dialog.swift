import SwiftUI

/// A dialog's edge colour. `danger` is the only deviation the sheet allows
/// itself — a destructive confirmation earns a tinted rule, nothing more.
public enum DialogTone: String, CaseIterable, Sendable {
  case `default`
  case danger
}

/// The sheet a modal form or a confirmation is built in.
///
/// It takes the window corner and the popover shadow rather than a panel's,
/// because a dialog is a window that floats rather than a panel that hovers at
/// the window's own elevation.
///
/// The React sheet lets its body scroll while the header and footer stay put.
/// Here that is the host's call: `ScrollView` is greedy along its axis, so
/// wrapping ``DialogBody`` in one unconditionally would make every dialog grow
/// to its maximum height — the opposite of what the CSS does. A body that
/// genuinely overflows puts a `ScrollView` inside its own content.
public struct Dialog<Content: View>: View {
  /// Overrides the token width for a sheet that needs more or less room.
  public var width: CGFloat?
  public var tone: DialogTone
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    width: CGFloat? = nil,
    tone: DialogTone = .default,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.width = width
    self.tone = tone
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(spacing: 0) {
      content()
    }
    .frame(width: width ?? vars.dialogWidth)
    .background(vars.colorSurfaceOverlay)
    .clipShape(RoundedRectangle(cornerRadius: vars.frameWindowRadius, style: .circular))
    .border(border, width: hairline, cornerRadius: vars.frameWindowRadius)
    .tokenShadow(vars.shadowLg)
    .accessibilityAddTraits(.isModal)
  }

  private var border: Color {
    guard tone == .danger else { return theme.vars.colorBorderStrong }
    // The CSS states this wash inline rather than as a token: a third of the
    // danger ramp's action step, which reads as a warning at a hairline
    // without the sheet gaining a red frame.
    return theme.vars.colorDanger[600].opacity(dangerEdgeAlpha)
  }

  private let dangerEdgeAlpha: CGFloat = 0.34
}

/// The chrome above the body: a title, an optional subtitle, and whatever the
/// host hangs beside them.
public struct DialogHeader<Accessory: View>: View {
  public var title: String
  public var subtitle: String?
  @ViewBuilder public var accessory: () -> Accessory

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    title: String,
    subtitle: String? = nil,
    @ViewBuilder accessory: @escaping () -> Accessory
  ) {
    self.title = title
    self.subtitle = subtitle
    self.accessory = accessory
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing1) {
      Text(title)
        // The title takes the label's weight at the title face's size, and
        // tightens a hundredth of an em the way a heading is set.
        .font(.system(size: vars.titleSmall.size, weight: vars.labelStrong.weight))
        .tracking(titleTracking * vars.titleSmall.size)
        .foregroundStyle(vars.colorContent)
      if let subtitle {
        Text(subtitle)
          .font(vars.captionSmall.font)
          .lineBox(vars.captionSmall)
          .foregroundStyle(vars.colorContentSubtle)
      }
      accessory()
    }
    .frame(maxWidth: .infinity, alignment: .leading)
    // Floored so a subtitle-less header still balances the footer.
    .padding(.vertical, vars.spacing3)
    .padding(.horizontal, vars.spacing5)
    .frame(minHeight: vars.spacing11)
    .rule(vars.colorBorder, edge: .bottom, width: hairline)
  }

  /// The CSS `letter-spacing: -0.01em`, which is a ratio of the face rather
  /// than a length, so it is stated the same way here.
  private let titleTracking: CGFloat = -0.01
}

public extension DialogHeader where Accessory == EmptyView {
  /// A header that is only its title and subtitle.
  init(_ title: String, subtitle: String? = nil) {
    self.init(title: title, subtitle: subtitle, accessory: { EmptyView() })
  }
}

/// The dialog's content.
///
/// It pads a step and a half — deeper than the chrome above and below it — so
/// the content sits clear of both rules rather than level with their text.
public struct DialogBody<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing4) {
      content()
    }
    .frame(maxWidth: .infinity, alignment: .leading)
    .padding(.vertical, vars.spacing4 + vars.spacing05)
    .padding(.horizontal, vars.spacing5)
  }
}

/// The chrome below the body — a row of controls on the window's bar fill.
public struct DialogFooter<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    // The actions sit at the trailing edge, the confirming one last.
    HStack(spacing: vars.spacing2) {
      content()
    }
    .frame(maxWidth: .infinity, alignment: .trailing)
    .padding(.vertical, vars.spacing3)
    .padding(.horizontal, vars.spacing5)
    .background(vars.colorSurfaceChrome)
    .rule(vars.colorBorder, edge: .top, width: hairline)
  }
}

/// The dimmed backdrop a ``Dialog`` is centred in.
///
/// It covers the whole surface it is placed on, the way a modal covers the
/// viewport. A scrim that has to stay inside a frame instead — a sheet over
/// one window of a desktop shell — is that shell's job, and the shell
/// components are not in the system yet.
public struct DialogScrim<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    ZStack {
      content()
        .padding(vars.spacing6)
    }
    .frame(maxWidth: .infinity, maxHeight: .infinity)
    .background(
      vars.dialogScrimColor
        .opacity(vars.dialogScrimAlpha)
        .ignoresSafeArea()
    )
  }
}
