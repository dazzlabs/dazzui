import FluentIcons
import SwiftUI

/// A checkbox has its own three sizes rather than the control profile's four:
/// only the gap comes off the shared vocabulary, and the box keeps a per-size
/// square of its own — that is the point of `checkbox.<size>.box`.
public enum CheckboxSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A box and a label on one row.
///
/// The box defines the row — there is no minimum height, so a stacked
/// checklist sits at the density the boxes set — and the label carries the
/// selection signal twice over: it rests quiet, one ink step back, and takes
/// the full ink and the label weight only once its box is checked. A checked
/// row you cannot pick out of the list by its label alone is not signalling.
public struct Checkbox<Label: View>: View {
  public var checked: Bool
  public var onCheckedChange: ((Bool) -> Void)?
  public var size: CheckboxSize
  /// Which colour ramp the checked box fills from.
  public var tint: Tint
  /// A de-emphasised aside on the label — a count, a size.
  public var note: String?
  public var enabled: Bool
  @ViewBuilder public var label: () -> Label

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    checked: Bool,
    onCheckedChange: ((Bool) -> Void)? = nil,
    size: CheckboxSize = .medium,
    tint: Tint = .primary,
    note: String? = nil,
    enabled: Bool = true,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.checked = checked
    self.onCheckedChange = onCheckedChange
    self.size = size
    self.tint = tint
    self.note = note
    self.enabled = enabled
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars

    Pressable(
      action: { onCheckedChange?(!checked) },
      enabled: enabled,
      cornerRadius: vars.radiusNone
    ) { state in
      HStack(alignment: .center, spacing: vars.spacing25) {
        box
        // A trailing note rides on the label rather than on the row, so it
        // wraps with it instead of being pushed to the far edge the way a
        // control would be.
        HStack(alignment: .firstTextBaseline, spacing: vars.spacing0) {
          label()
          if let note {
            Text(" \(note)").foregroundStyle(vars.colorContentFaint)
          }
        }
      }
      .font(.system(size: vars.labelQuiet.size, weight: labelWeight))
      .foregroundStyle(checked ? vars.colorContent : vars.colorContentMuted)
      .multilineTextAlignment(.leading)
      .opacity(state.enabled ? 1 : Self.disabledOpacity)
      .animation(vars.motion, value: checked)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: vars.radiusNone,
      width: vars.focusWidth
    )
    .accessibilityAddTraits(checked ? [.isSelected] : [])
  }

  /// The row's own box: neutral while empty — an unchecked box is an outline,
  /// not a quiet accent — and the filled recipe's surface once checked, border
  /// and all, so the square reads as one solid rather than as a fill inside a
  /// grey ring.
  private var box: some View {
    let vars = theme.vars
    let filled = checked ? recipe(.surface) : Color.clear

    // `Checkmark12Filled`, the glyph `checkbox.tsx` names. It sits a step
    // under the label so it clears the box's corners.
    return Icon(.checkmark12Filled, size: vars.labelSmall.size)
      .opacity(checked ? 1 : 0)
      .foregroundStyle(recipe(.content))
      .frame(width: boxSize, height: boxSize)
      .background(filled)
      .clipShape(RoundedRectangle(cornerRadius: vars.checkboxRadius, style: .circular))
      .border(
        checked ? filled : vars.colorBorderMuted,
        width: vars.checkboxThickness,
        cornerRadius: vars.checkboxRadius
      )
  }

  /// The checked box paints the resting `filled` recipe. A checkbox has no
  /// hover or press colour of its own, so it stays out of the shared state
  /// machine and reads the `-normal` recipes directly.
  private func recipe(_ part: ControlPart) -> Color {
    theme.vars.resolve(theme.vars.descriptor(.filled, part), tint: tint, state: .normal)
  }

  /// The box corner is the component's own token: any shared step doubles as a
  /// pill under Bright, and a checkbox that goes round is a radio.
  private var boxSize: CGFloat {
    switch size {
    case .small: return theme.vars.checkboxSmallBox
    case .medium: return theme.vars.checkboxMediumBox
    case .large: return theme.vars.checkboxLargeBox
    }
  }

  private var labelWeight: Font.Weight {
    checked ? theme.vars.labelMedium.weight : theme.vars.labelQuiet.weight
  }

  /// `:disabled { opacity: 0.6 }` — the stylesheet states the number outright,
  /// so there is no token to read it from.
  private static var disabledOpacity: CGFloat { 0.6 }
}

public extension Checkbox where Label == Text {
  /// A checkbox whose label is a plain string.
  init(
    _ title: String,
    checked: Bool,
    onCheckedChange: ((Bool) -> Void)? = nil,
    size: CheckboxSize = .medium,
    tint: Tint = .primary,
    note: String? = nil,
    enabled: Bool = true
  ) {
    self.init(
      checked: checked,
      onCheckedChange: onCheckedChange,
      size: size,
      tint: tint,
      note: note,
      enabled: enabled,
      label: { Text(title) }
    )
  }
}
