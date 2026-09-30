import FluentIcons
import SwiftUI

/// The recipes a tag can wear — the ``Badge``'s set, since it is the badge's
/// capsule.
public enum TagVariant: String, CaseIterable, Sendable {
  case filled
  case tinted
  case outlined
  /// The tag that sits on a grey card: paper fill, quiet ink.
  case raised
}

/// A ``Badge`` you can act on.
///
/// The two are one drawing and two views rather than one view with an
/// `interactive` flag, because the difference is not cosmetic: a badge reads
/// its recipes at rest and deliberately stays out of the state machine, since a
/// label that moves under the pointer reads as something you can press. A tag
/// *is* something you can press, and taking the machine is the whole reason it
/// exists.
///
/// The dismiss button is its own control rather than a second job for the
/// label: a chip that removes itself when clicked has no way to be selected,
/// and one that does both from the same target removes things by accident.
///
/// The capsule's pads and type mirror ``Badge`` — the same box at the same three
/// densities. React shares them in one stylesheet; here they are written twice,
/// and the parity reference test is what holds the two together.
public struct Tag: View {
  public var title: String
  public var size: BadgeSize
  public var variant: TagVariant
  public var tint: Tint
  /// Latched. Passing it at all is what makes the label pressable, and a
  /// selected tag is drawn with the `tinted` recipe and the tint's edge
  /// whatever `variant` says.
  public var selected: Bool?
  public var enabled: Bool
  public var onSelect: ((Bool) -> Void)?
  /// Draws the dismiss button, and is what it does.
  public var onRemove: (() -> Void)?
  public var removeLabel: String

  @Environment(\.theme) private var theme

  public init(
    _ title: String,
    size: BadgeSize = .medium,
    variant: TagVariant = .tinted,
    tint: Tint = .primary,
    selected: Bool? = nil,
    enabled: Bool = true,
    removeLabel: String = "Remove",
    onSelect: ((Bool) -> Void)? = nil,
    onRemove: (() -> Void)? = nil
  ) {
    self.title = title
    self.size = size
    self.variant = variant
    self.tint = tint
    self.selected = selected
    self.enabled = enabled
    self.removeLabel = removeLabel
    self.onSelect = onSelect
    self.onRemove = onRemove
  }

  private var pressable: Bool { selected != nil }

  private var drawn: TagVariant { selected == true ? .tinted : variant }

  public var body: some View {
    if pressable {
      Pressable(
        action: { onSelect?(!(selected ?? false)) },
        enabled: enabled,
        cornerRadius: theme.vars.radiusTiny
      ) { state in
        capsule(state: state.controlState)
      }
    } else {
      // At rest, whatever `enabled` says. A tag that cannot be pressed is a
      // label, and a label has no disabled state — only the dismiss button
      // inside it does, and it flattens itself. React draws it the same way,
      // because a static tag never joins the state machine at all.
      capsule(state: .normal)
    }
  }

  private func capsule(state: ControlState) -> some View {
    let vars = theme.vars

    return HStack(spacing: vars.spacing1) {
      Text(title)
      if let onRemove {
        Dismiss(label: removeLabel, ink: content(state: state), enabled: enabled, action: onRemove)
      }
    }
    .font(.system(size: fontSize, weight: vars.labelMedium.weight))
    .lineLimit(1)
    // Match the single CSS line box before adding padding and border.
    .frame(height: fontSize * 1.4)
    .foregroundStyle(content(state: state))
    .padding(.vertical, paddingBlock + vars.spacingPx)
    .padding(.horizontal, paddingInline + vars.spacingPx)
    .background(surface(state: state))
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusTiny, style: .circular))
    // A border on every variant rather than only on `outlined`, so that one
    // does not gain a pixel per side and fall off the heights the others share.
    .border(border(state: state), width: vars.spacingPx, cornerRadius: vars.radiusTiny)
    .animation(vars.motion, value: state)
  }

  // Two capsule densities on the chip corner; the type moves one step at the
  // top. Badge's, because this is Badge's capsule.
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
    switch drawn {
    case .filled: return .filled
    case .tinted: return .tinted
    case .outlined: return .outlined
    case .raised: return nil
    }
  }

  private func recipe(_ part: ControlPart, state: ControlState) -> Color? {
    guard let recipeVariant else { return nil }
    return theme.vars.resolve(
      theme.vars.descriptor(recipeVariant, part),
      tint: tint,
      state: state
    )
  }

  private func surface(state: ControlState) -> Color {
    guard drawn != .raised else { return theme.vars.colorSurface }
    return drawn == .outlined ? .clear : recipe(.surface, state: state) ?? .clear
  }

  private func content(state: ControlState) -> Color {
    guard drawn != .raised else { return theme.vars.colorContentMuted }
    return recipe(.content, state: state) ?? theme.vars.colorContent
  }

  private func border(state: ControlState) -> Color {
    // Selected is drawn twice over: the tinted fill the variant swaps to, and
    // this edge. One alone is not enough — a tinted tag that was already tinted
    // would look untouched, and an edge alone is a pixel to spot in a row of
    // forty.
    if selected == true {
      return theme.vars.resolve(
        theme.vars.descriptor(.outlined, .border),
        tint: tint,
        state: .normal
      )
    }
    return drawn == .outlined ? recipe(.border, state: state) ?? .clear : .clear
  }
}

/// The ✕ inside a tag.
///
/// Faint at rest and only resolved under the pointer: an ✕ at full strength on
/// every chip turns a list of tags into a list of buttons. Its wash is an alpha
/// of the capsule's own ink rather than a recipe, because it sits on four
/// variants across twelve themes and the only colour guaranteed to read on all
/// of them is the one the label is already set in.
private struct Dismiss: View {
  let label: String
  let ink: Color
  let enabled: Bool
  let action: () -> Void

  @Environment(\.theme) private var theme

  private static let hoverAlpha: Double = 0.16
  private static let restOpacity: Double = 0.6
  private static let deadOpacity: Double = 0.4

  var body: some View {
    let vars = theme.vars

    Pressable(action: action, enabled: enabled, cornerRadius: vars.radiusFull, help: label) {
      state in
      Icon(.dismiss12Regular, size: vars.iconSmall)
        .foregroundStyle(ink)
        .opacity(
          !state.enabled
            ? Self.deadOpacity
            : (state.hovered ? 1 : Self.restOpacity)
        )
        .background(
          state.hovered && state.enabled ? ink.opacity(Self.hoverAlpha) : .clear
        )
        .clipShape(Circle())
        .animation(vars.motion, value: state.controlState)
    }
    .accessibilityLabel(label)
  }
}
