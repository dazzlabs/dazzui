import SwiftUI

/// A switch carries its own three sizes: it is a proportion — track,
/// padding and thumb — rather than a control that has to line up with a
/// button's height.
public enum SwitchSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A switch: the track carries the state, the thumb slides to the side that
/// says which.
///
/// The thumb is laid out by alignment rather than by a translate, so it lands
/// on the track's padding whatever the size is.
public struct Switch: View {
  public var checked: Bool
  public var onCheckedChange: ((Bool) -> Void)?
  public var size: SwitchSize
  /// Which colour ramp the filled track is drawn from.
  public var tint: Tint
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    checked: Bool,
    onCheckedChange: ((Bool) -> Void)? = nil,
    size: SwitchSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true
  ) {
    self.checked = checked
    self.onCheckedChange = onCheckedChange
    self.size = size
    self.tint = tint
    self.enabled = enabled
  }

  public var body: some View {
    let vars = theme.vars

    Pressable(
      action: { onCheckedChange?(!checked) },
      enabled: enabled,
      cornerRadius: vars.radiusFull
    ) { state in
      RoundedRectangle(cornerRadius: vars.radiusFull, style: .continuous)
        .fill(thumbColor)
        .frame(width: thumb, height: thumb)
        .frame(maxWidth: .infinity, alignment: checked ? .trailing : .leading)
        .padding(vars.spacing05)
        .frame(width: width, height: height)
        .background(trackColor)
        .clipShape(RoundedRectangle(cornerRadius: vars.radiusFull, style: .continuous))
        .opacity(state.enabled ? 1 : Self.disabledOpacity)
        .animation(vars.motion, value: checked)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: vars.radiusFull,
      width: vars.focusWidth
    )
    .accessibilityAddTraits(checked ? [.isSelected] : [])
  }

  /// A toggle has no hover or press colour — the state it shows is the state
  /// it is in — so it reads the resting `filled` surface directly rather than
  /// going through the shared state machine.
  private var trackColor: Color {
    guard checked else { return theme.vars.colorSurfaceSunken }
    return theme.vars.resolve(
      theme.vars.descriptor(.filled, .surface),
      tint: tint,
      state: .normal
    )
  }

  /// Faint ink while off — a grey thumb in a grey groove, deliberately
  /// low-contrast, "off" — and only the raised surface once the track fills.
  /// A white thumb in the off groove reads nearly as loud as the on state, and
  /// `surface-raised` rather than the paper because on a dark canvas the paper
  /// is darker than the groove and a thumb painted with it would sink.
  private var thumbColor: Color {
    checked ? theme.vars.colorSurfaceRaised : theme.vars.colorContentFaint
  }

  private var width: CGFloat {
    switch size {
    case .small: return theme.vars.switchSmallWidth
    case .medium: return theme.vars.switchMediumWidth
    case .large: return theme.vars.switchLargeWidth
    }
  }

  private var height: CGFloat {
    switch size {
    case .small: return theme.vars.switchSmallHeight
    case .medium: return theme.vars.switchMediumHeight
    case .large: return theme.vars.switchLargeHeight
    }
  }

  private var thumb: CGFloat {
    switch size {
    case .small: return theme.vars.switchSmallThumb
    case .medium: return theme.vars.switchMediumThumb
    case .large: return theme.vars.switchLargeThumb
    }
  }

  /// `:disabled { opacity: 0.6 }` — the stylesheet states the number outright,
  /// so there is no token to read it from.
  private static var disabledOpacity: CGFloat { 0.6 }
}
