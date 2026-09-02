import SwiftUI

/// A key hint keeps its own three sizes: it annotates a row rather than
/// standing in one, so it never reaches control height.
public enum KeyCapSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// `hint` is bare faint text — the default, because most keys shown in an
/// interface are asides. `strong` darkens it a step. `key` alone draws the
/// recessed cap.
public enum KeyCapVariant: String, CaseIterable, Sendable {
  case hint
  case strong
  case key
}

/// One key hint or cap. A chord is several of these, composed by the caller.
///
/// Three renderings of a key, quietest first. `hint` is the `⌘F` beside an
/// empty search field; `strong` is the same a step darker, for a hint that has
/// to survive on a busy row; `key` draws the cap — a recessed chip-cornered
/// fill with no border, because a key is pressed *into* the surface rather
/// than raised off it.
///
/// Its colours are already in the token graph, so it takes no tint, and it is
/// not interactive, so it stays out of the shared state machine.
public struct KeyCap<Label: View>: View {
  public var size: KeyCapSize
  public var variant: KeyCapVariant
  @ViewBuilder public var label: () -> Label

  @Environment(\.theme) private var theme

  public init(
    size: KeyCapSize = .medium,
    variant: KeyCapVariant = .hint,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.size = size
    self.variant = variant
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars

    label()
      // The display cut whatever the variant — a key is a glyph, not copy.
      .font(.system(size: fontSize, weight: weight))
      .lineLimit(1)
      .foregroundStyle(content)
      .padding(.horizontal, variant == .key ? vars.spacing25 : 0)
      .frame(height: variant == .key ? capHeight : nil)
      .background(variant == .key ? vars.colorSurfaceInset : .clear)
      .clipShape(RoundedRectangle(cornerRadius: capRadius, style: .continuous))
  }

  private var fontSize: CGFloat {
    size == .large ? theme.vars.labelMedium.size : theme.vars.labelSmall.size
  }

  private var weight: Font.Weight {
    variant == .key ? theme.vars.labelStrong.weight : theme.vars.labelMedium.weight
  }

  private var capHeight: CGFloat {
    theme.vars.spacing5 + theme.vars.spacing05
  }

  private var capRadius: CGFloat {
    variant == .key ? theme.vars.radiusTiny : theme.vars.radiusNone
  }

  private var content: Color {
    switch variant {
    case .hint: return theme.vars.colorContentFaint
    case .strong: return theme.vars.colorContentSubtle
    case .key: return theme.vars.colorContent
    }
  }
}

public extension KeyCap where Label == Text {
  /// A key whose label is a plain string — `⌘`, `F`, `Space`.
  init(_ keys: String, size: KeyCapSize = .medium, variant: KeyCapVariant = .hint) {
    self.init(size: size, variant: variant, label: { Text(keys) })
  }
}
