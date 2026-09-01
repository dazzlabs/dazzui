import SwiftUI

/// The micro-heading that organises a pane.
///
/// One size, sentence case, no added tracking: tone is the only axis worth
/// varying — the labelling ink by default, one step fainter for a nav group's
/// heading, and a tint's *text* grade when the label carries a status, because
/// a label is read, not pressed.
public struct SectionLabel<Label: View>: View {
  /// Leave unset for the labelling ink. Set, it resolves the quiet accent
  /// recipe against that ramp — which is why it is optional rather than
  /// defaulted: an untinted label must keep the labelling ink rather than
  /// resolve against a ramp it never chose.
  public var tint: Tint?
  /// One step quieter — a nav group's heading rather than a pane's.
  public var faint: Bool
  @ViewBuilder public var label: () -> Label

  @Environment(\.theme) private var theme

  public init(
    tint: Tint? = nil,
    faint: Bool = false,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.tint = tint
    self.faint = faint
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars

    label()
      .font(vars.labelSmall.font)
      .lineSpacing(vars.labelSmall.lineSpacing)
      .foregroundStyle(color)
  }

  private var color: Color {
    let vars = theme.vars
    // A tint outranks the faint tone, the way `[data-tint]` outranks
    // `[data-tone='faint']` further down the stylesheet.
    if let tint {
      return vars.resolve(vars.descriptor(.plain, .content), tint: tint, state: .normal)
    }
    return faint ? vars.colorContentFaint : vars.colorContentSubtle
  }
}

public extension SectionLabel where Label == Text {
  /// A section label whose text is a plain string.
  init(_ title: String, tint: Tint? = nil, faint: Bool = false) {
    self.init(tint: tint, faint: faint, label: { Text(title) })
  }
}
