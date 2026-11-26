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
  /// Optional fixed height for dialogs with internally scrolling layouts.
  public var height: CGFloat?
  public var tone: DialogTone
  /// Optional Escape-to-dismiss handler invoked on cancel command.
  public var onDismiss: (() -> Void)?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    width: CGFloat? = nil,
    height: CGFloat? = nil,
    tone: DialogTone = .default,
    onDismiss: (() -> Void)? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.width = width
    self.height = height
    self.tone = tone
    self.onDismiss = onDismiss
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars
    let box = VStack(spacing: 0) {
      content()
    }
    .frame(width: width ?? vars.dialogWidth, height: height)
    .background(vars.colorSurfaceOverlay)
    .clipShape(RoundedRectangle(cornerRadius: vars.frameWindowRadius, style: .circular))
    .border(border, width: vars.strokeHairline, cornerRadius: vars.frameWindowRadius)
    .tokenShadow(vars.shadowLg)
    .accessibilityAddTraits(.isModal)

    if let onDismiss {
      #if os(macOS)
        box.onExitCommand { onDismiss() }
      #else
        box.background {
          SwiftUI.Button("") { onDismiss() }
            .keyboardShortcut(.cancelAction)
            .opacity(0)
            .frame(width: 0, height: 0)
            .accessibilityHidden(true)
        }
      #endif
    } else {
      box
    }
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

/// The chrome above the body: an optional leading visual (icon or thumbnail),
/// a title, an optional subtitle (proportional or monospaced), and an optional
/// accessory view.
public struct DialogHeader<Leading: View, Accessory: View>: View {
  public var title: String
  public var subtitle: String?
  /// Renders `subtitle` in the monospaced code face (for IDs, addresses, paths).
  public var monoSubtitle: Bool
  /// Optional leading view (thumbnail, avatar, or glyph) beside the title stack.
  public var leading: (() -> Leading)?
  @ViewBuilder public var accessory: () -> Accessory

  @Environment(\.theme) private var theme

  public init(
    title: String,
    subtitle: String? = nil,
    monoSubtitle: Bool = false,
    @ViewBuilder leading: @escaping () -> Leading,
    @ViewBuilder accessory: @escaping () -> Accessory
  ) {
    self.title = title
    self.subtitle = subtitle
    self.monoSubtitle = monoSubtitle
    self.leading = leading
    self.accessory = accessory
  }

  init(
    title: String,
    subtitle: String?,
    monoSubtitle: Bool,
    leadingBuilder: (() -> Leading)?,
    accessory: @escaping () -> Accessory
  ) {
    self.title = title
    self.subtitle = subtitle
    self.monoSubtitle = monoSubtitle
    self.leading = leadingBuilder
    self.accessory = accessory
  }

  public var body: some View {
    let vars = theme.vars

    Group {
      if let leading {
        HStack(alignment: .center, spacing: vars.spacing35) {
          leading()
          titleStack(vars: vars)
          Spacer(minLength: 0)
          accessory()
        }
      } else {
        VStack(alignment: .leading, spacing: vars.spacing1) {
          titleStack(vars: vars)
          accessory()
        }
      }
    }
    .frame(maxWidth: .infinity, alignment: .leading)
    // Floored so a subtitle-less header still balances the footer.
    .padding(.vertical, leading != nil ? vars.spacing4 : vars.spacing3)
    .padding(.horizontal, vars.spacing5)
    .frame(minHeight: vars.spacing11)
    .rule(vars.colorBorder, edge: .bottom, width: vars.strokeHairline)
  }

  @ViewBuilder
  private func titleStack(vars: ThemeVariables) -> some View {
    VStack(alignment: .leading, spacing: vars.spacing1) {
      Text(title)
        // The title takes the label's weight at the title face's size, and
        // tightens a hundredth of an em the way a heading is set.
        .font(.system(size: vars.titleSmall.size, weight: vars.labelStrong.weight))
        .tracking(titleTracking * vars.titleSmall.size)
        .foregroundStyle(vars.colorContent)
      if let subtitle {
        if monoSubtitle {
          Text(subtitle)
            .font(vars.codeFont(size: vars.captionSmall.size))
            .foregroundStyle(vars.colorContentSubtle)
        } else {
          Text(subtitle)
            .font(vars.captionSmall.font)
            .lineBox(vars.captionSmall)
            .foregroundStyle(vars.colorContentSubtle)
        }
      }
    }
  }

  /// The CSS `letter-spacing: -0.01em`, which is a ratio of the face rather
  /// than a length, so it is stated the same way here.
  private let titleTracking: CGFloat = -0.01
}

public extension DialogHeader where Leading == EmptyView {
  /// A header with a title, optional subtitle, and trailing/below accessory.
  init(
    title: String,
    subtitle: String? = nil,
    monoSubtitle: Bool = false,
    @ViewBuilder accessory: @escaping () -> Accessory
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      monoSubtitle: monoSubtitle,
      leadingBuilder: nil,
      accessory: accessory
    )
  }
}

public extension DialogHeader where Accessory == EmptyView {
  /// A header with a leading view, title, and optional subtitle.
  init(
    _ title: String,
    subtitle: String? = nil,
    monoSubtitle: Bool = false,
    @ViewBuilder leading: @escaping () -> Leading
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      monoSubtitle: monoSubtitle,
      leadingBuilder: leading,
      accessory: { EmptyView() }
    )
  }
}

public extension DialogHeader where Leading == EmptyView, Accessory == EmptyView {
  /// A header that is only its title and subtitle.
  init(_ title: String, subtitle: String? = nil, monoSubtitle: Bool = false) {
    self.init(
      title: title,
      subtitle: subtitle,
      monoSubtitle: monoSubtitle,
      leadingBuilder: nil,
      accessory: { EmptyView() }
    )
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
    .rule(vars.colorBorder, edge: .top, width: vars.strokeHairline)
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

public extension View {
  /// Presents a window-scoped modal dialog directly over this view on a
  /// ``DialogScrim`` without requiring a separate `.overlayHost()` ancestor.
  ///
  /// Clicking the scrim or pressing Escape dismisses the dialog when
  /// `dismissible` is true, and `\.closeOverlay` is injected into `content`.
  func windowDialog<Panel: View>(
    isPresented: Binding<Bool>,
    dismissible: Bool = true,
    @ViewBuilder content: @escaping () -> Panel
  ) -> some View {
    modifier(
      WindowDialogModifier(
        isPresented: isPresented,
        dismissible: dismissible,
        panel: content
      )
    )
  }

  /// Presents a window-scoped modal dialog whenever `item` is non-nil.
  func windowDialog<Item: Identifiable, Panel: View>(
    item: Binding<Item?>,
    dismissible: Bool = true,
    @ViewBuilder content: @escaping (Item) -> Panel
  ) -> some View {
    let presented = Binding<Bool>(
      get: { item.wrappedValue != nil },
      set: { if !$0 { item.wrappedValue = nil } }
    )
    return windowDialog(isPresented: presented, dismissible: dismissible) {
      if let value = item.wrappedValue {
        content(value)
      }
    }
  }
}

private struct WindowDialogModifier<Panel: View>: ViewModifier {
  @Binding var isPresented: Bool
  let dismissible: Bool
  @ViewBuilder let panel: () -> Panel

  @Environment(\.theme) private var theme

  func body(content: Content) -> some View {
    content.overlay {
      if isPresented {
        let close = { isPresented = false }
        let dialogLayer = DialogScrim {
          ZStack {
            Color.clear
              .contentShape(Rectangle())
              .onTapGesture { if dismissible { close() } }
            panel()
          }
        }
        .environment(\.closeOverlay, CloseOverlayAction(close))
        .transition(.opacity)

        #if os(macOS)
          dialogLayer.onExitCommand { if dismissible { close() } }
        #else
          dialogLayer.background {
            if dismissible {
              SwiftUI.Button("") { close() }
                .keyboardShortcut(.cancelAction)
                .opacity(0)
                .frame(width: 0, height: 0)
                .accessibilityHidden(true)
            }
          }
        #endif
      }
    }
    .animation(theme.vars.motion, value: isPresented)
  }
}
