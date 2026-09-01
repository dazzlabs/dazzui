import SwiftUI

/// The `control.color.*` recipes that read as a label rather than a control,
/// plus the one a badge owns: `raised`, the paper chip that sits on a grey
/// card.
public enum BadgeVariant: String, CaseIterable, Sendable {
  case filled
  case tinted
  case outlined
  case raised
}

/// A badge is an annotation, not a control, so it has its own three sizes and
/// never reaches control height.
public enum BadgeSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A tag: the display cut at label weight in a tight chip-cornered capsule.
///
/// Its label has to read as an annotation rather than as body copy, which is
/// why the face and weight never soften whatever the size. A badge is not
/// interactive, so it deliberately skips the shared state machine and reads
/// the resting recipes directly — a label that changed colour under the
/// pointer would read as something you can press.
public struct Badge<Label: View>: View {
  public var size: BadgeSize
  public var variant: BadgeVariant
  /// Which colour ramp the variant resolves against.
  public var tint: Tint
  @ViewBuilder public var label: () -> Label

  @Environment(\.theme) private var theme

  public init(
    size: BadgeSize = .medium,
    variant: BadgeVariant = .tinted,
    tint: Tint = .primary,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.size = size
    self.variant = variant
    self.tint = tint
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars

    label()
      .font(.system(size: fontSize, weight: vars.labelMedium.weight))
      .lineLimit(1)
      .foregroundStyle(content)
      .padding(.vertical, paddingBlock)
      .padding(.horizontal, paddingInline)
      .background(surface)
      .clipShape(RoundedRectangle(cornerRadius: vars.radiusTiny, style: .continuous))
      // A border on every variant rather than only on `outlined`, so that one
      // does not gain a pixel per side and fall off the heights the others
      // share.
      .border(border, width: vars.spacingPx, cornerRadius: vars.radiusTiny)
  }

  // Two capsule densities on the chip corner; the type moves one step at the
  // top.
  private var paddingBlock: CGFloat {
    size == .large ? theme.vars.spacing1 : theme.vars.spacing05
  }

  private var paddingInline: CGFloat {
    size == .small ? theme.vars.spacing15 : theme.vars.spacing2
  }

  private var fontSize: CGFloat {
    size == .large ? theme.vars.labelMedium.size : theme.vars.labelSmall.size
  }

  private var recipeVariant: ControlVariant? {
    switch variant {
    case .filled: return .filled
    case .tinted: return .tinted
    case .outlined: return .outlined
    case .raised: return nil
    }
  }

  private func resting(_ part: ControlPart) -> Color? {
    guard let recipeVariant else { return nil }
    return theme.vars.resolve(
      theme.vars.descriptor(recipeVariant, part),
      tint: tint,
      state: .normal
    )
  }

  private var surface: Color {
    // The badge that sits on a grey card: paper fill, quiet ink.
    guard variant != .raised else { return theme.vars.colorSurface }
    return variant == .outlined ? .clear : resting(.surface) ?? .clear
  }

  private var content: Color {
    guard variant != .raised else { return theme.vars.colorContentMuted }
    return resting(.content) ?? theme.vars.colorContent
  }

  private var border: Color {
    variant == .outlined ? resting(.border) ?? .clear : .clear
  }
}

public extension Badge where Label == Text {
  /// A badge whose label is a plain string.
  init(
    _ title: String,
    size: BadgeSize = .medium,
    variant: BadgeVariant = .tinted,
    tint: Tint = .primary
  ) {
    self.init(size: size, variant: variant, tint: tint, label: { Text(title) })
  }
}
