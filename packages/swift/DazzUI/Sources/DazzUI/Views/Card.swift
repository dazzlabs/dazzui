import SwiftUI

/// The three card densities. A card is not a control, so it has its own scale
/// rather than the control profiles.
public enum CardSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// `raised` is the paper panel, `sunken` the workaday card one step off it,
/// `outlined` the drawn box, and `tinted` the washed one.
public enum CardVariant: String, CaseIterable, Sendable {
  case raised
  case sunken
  case outlined
  case tinted
}

/// Interior padding posture for ``Card``.
public enum CardPadding: String, CaseIterable, Sendable {
  /// Standard size-scaled card padding.
  case `default`
  /// Zero interior padding for full-bleed media, lists, or custom inner chrome.
  case none
}

/// The filled or bordered box the panels in a layout are built from.
///
/// By default a card stays non-interactive and reads resting surfaces directly.
/// Pass `interactive: true` on cards that act as clickable tiles or list panels
/// to enable a subtle hover border lift.
public struct Card<Header: View, Content: View>: View {
  public var size: CardSize
  public var variant: CardVariant
  /// Which ramp `tinted` resolves against; ignored by the other variants.
  public var tint: Tint
  public var padding: CardPadding
  /// Highlights the card border on hover for clickable card tiles.
  public var interactive: Bool
  /// Optional pinned header band separated from the body by a bottom hairline.
  public var header: (() -> Header)?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @State private var hovered = false

  public init(
    size: CardSize = .medium,
    variant: CardVariant = .raised,
    tint: Tint = .primary,
    padding: CardPadding = .default,
    interactive: Bool = false,
    @ViewBuilder header: @escaping () -> Header,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.size = size
    self.variant = variant
    self.tint = tint
    self.padding = padding
    self.interactive = interactive
    self.header = header
    self.content = content
  }

  init(
    size: CardSize,
    variant: CardVariant,
    tint: Tint,
    padding: CardPadding,
    interactive: Bool,
    headerBuilder: (() -> Header)?,
    content: @escaping () -> Content
  ) {
    self.size = size
    self.variant = variant
    self.tint = tint
    self.padding = padding
    self.interactive = interactive
    self.header = headerBuilder
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars
    let hairline = vars.strokeHairline
    let box = Group {
      if let header {
        VStack(alignment: .leading, spacing: 0) {
          header()
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.vertical, padding == .none ? 0 : paddingBlock)
            .padding(.horizontal, padding == .none ? 0 : paddingInline)
            .rule(vars.colorBorder, edge: .bottom, width: hairline)
          VStack(alignment: .leading, spacing: gap) {
            content()
          }
          .frame(maxWidth: .infinity, alignment: .leading)
          .padding(.vertical, padding == .none ? 0 : paddingBlock)
          .padding(.horizontal, padding == .none ? 0 : paddingInline)
        }
        .padding(hairline)
      } else {
        VStack(alignment: .leading, spacing: gap) {
          content()
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(.vertical, padding == .none ? 0 : paddingBlock + hairline)
        .padding(.horizontal, padding == .none ? 0 : paddingInline + hairline)
      }
    }
    .background(surface)
    .clipShape(RoundedRectangle(cornerRadius: radius, style: .circular))
    // Drawn on every variant rather than only where it shows, so `outlined`
    // and `tinted` do not gain a pixel per side and sit a hair larger than
    // the filled ones.
    .border(border, width: hairline, cornerRadius: radius)

    if interactive {
      box
        .onHover { hovered = $0 }
        .animation(vars.motion, value: hovered)
    } else {
      box
    }
  }

  private var gap: CGFloat {
    switch size {
    case .small: return theme.vars.spacing2
    case .medium: return theme.vars.spacing3
    case .large: return theme.vars.spacing4
    }
  }

  private var paddingBlock: CGFloat {
    switch size {
    case .small: return theme.vars.spacing3
    case .medium: return theme.vars.spacing3
    case .large: return theme.vars.spacing4
    }
  }

  /// The medium card's padding is asymmetric — a hair more inline than block —
  /// which keeps a one-line row inside it optically centred.
  private var paddingInline: CGFloat {
    switch size {
    case .small: return theme.vars.spacing3
    case .medium: return theme.vars.spacing35
    case .large: return theme.vars.spacing4
    }
  }

  /// The container corner at every size: a card grows with what it holds,
  /// and Bright's pill on the control steps is wrong for that.
  private var radius: CGFloat { theme.vars.radiusLarge }

  private var surface: Color {
    let vars = theme.vars
    switch variant {
    case .raised: return vars.colorSurface
    // The workaday card: one step off the paper, the surface most content
    // sits on.
    case .sunken: return vars.colorSurfaceMuted
    case .outlined: return .clear
    // Tinted takes the surface wash rather than the `tinted` recipe's chip
    // fill: a card is the largest thing a tint washes here, and the chip's
    // 12% fills it instead of tinting it.
    case .tinted: return vars.ramp(tint)[600].opacity(vars.washSurface)
    }
  }

  private var border: Color {
    let vars = theme.vars
    if interactive && hovered {
      switch variant {
      case .raised, .sunken, .outlined: return vars.colorBorderMuted
      case .tinted: return vars.ramp(tint)[600].opacity(min(1, vars.washEdge * 1.5))
      }
    }
    switch variant {
    case .raised, .sunken, .outlined: return vars.colorBorder
    // The matching wash, so the pair moves together when either is retuned.
    case .tinted: return vars.ramp(tint)[600].opacity(vars.washEdge)
    }
  }
}

public extension Card where Header == EmptyView {
  /// A card without a separate header band.
  init(
    size: CardSize = .medium,
    variant: CardVariant = .raised,
    tint: Tint = .primary,
    padding: CardPadding = .default,
    interactive: Bool = false,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      size: size,
      variant: variant,
      tint: tint,
      padding: padding,
      interactive: interactive,
      headerBuilder: nil,
      content: content
    )
  }
}
