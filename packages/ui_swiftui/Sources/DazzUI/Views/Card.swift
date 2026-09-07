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

/// The filled or bordered box the panels in a layout are built from.
///
/// A card is not interactive, so it stays out of the shared state machine
/// entirely and reads the resting surfaces directly — a panel that lit up
/// under the pointer would read as something you can press.
public struct Card<Content: View>: View {
  public var size: CardSize
  public var variant: CardVariant
  /// Which ramp `tinted` resolves against; ignored by the other variants.
  public var tint: Tint
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    size: CardSize = .medium,
    variant: CardVariant = .raised,
    tint: Tint = .primary,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.size = size
    self.variant = variant
    self.tint = tint
    self.content = content
  }

  public var body: some View {
    VStack(alignment: .leading, spacing: gap) {
      content()
    }
    .padding(.vertical, paddingBlock)
    .padding(.horizontal, paddingInline)
    .background(surface)
    .clipShape(RoundedRectangle(cornerRadius: radius, style: .circular))
    // Drawn on every variant rather than only where it shows, so `outlined`
    // and `tinted` do not gain a pixel per side and sit a hair larger than
    // the filled ones.
    .border(border, width: hairline, cornerRadius: radius)
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
    switch variant {
    case .raised, .sunken, .outlined: return vars.colorBorder
    // The matching wash, so the pair moves together when either is retuned.
    case .tinted: return vars.ramp(tint)[600].opacity(vars.washEdge)
    }
  }
}
