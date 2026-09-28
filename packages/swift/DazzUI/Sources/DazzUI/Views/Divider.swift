import SwiftUI

/// How hard the rule has to work.
public enum DividerTone: String, CaseIterable, Sendable {
  case `default`
  /// The heavier rule, for a separator that has to hold its own against a
  /// filled surface. It is `border-strong` — ink at a tenth — and deliberately
  /// not `border-muted`: that role is the drawn outline of a checkbox or a
  /// radio, an opaque grey around twice this weight, and a separator set in it
  /// stops reading as a hairline and starts reading as a border.
  case muted
}

public enum DividerOrientation: String, CaseIterable, Sendable {
  case horizontal
  case vertical
}

/// A rule.
///
/// Named `Divider` rather than prefixed, and reached as `DazzUI.Divider`
/// where `SwiftUI.Divider` is also in scope.
///
/// It is one *device* pixel rather than one point: a 1pt line on a Retina
/// display is twice the weight of the real thing, which is the same halving
/// `base.css` does behind its `min-resolution: 2dppx` block.
public struct Divider: View {
  public var tone: DividerTone
  public var orientation: DividerOrientation

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    tone: DividerTone = .default,
    orientation: DividerOrientation = .horizontal
  ) {
    self.tone = tone
    self.orientation = orientation
  }

  public var body: some View {
    let color = tone == .muted ? theme.vars.colorBorderStrong : theme.vars.colorBorder

    switch orientation {
    case .horizontal:
      Rectangle()
        .fill(color)
        .frame(height: hairline)
        .frame(maxWidth: .infinity)
    case .vertical:
      // `align-self: stretch` — the rule takes the height of the row it sits
      // in rather than one of its own.
      Rectangle()
        .fill(color)
        .frame(width: hairline)
        .frame(maxHeight: .infinity)
    }
  }
}
