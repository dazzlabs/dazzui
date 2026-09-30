import SwiftUI

// The axes a component varies along. Each is the Swift spelling of a
// `data-*` attribute in `packages/js/dazzui-react`, and the vocabulary is the ds
// one: the colour axis is `tint`, never `kind` or `secondary`.

/// The ramps a control can be tinted with — one per semantic role in
/// `tokens/global/color.yaml`.
///
/// Interactive colours are stored as recipes rather than finished colours, so
/// a control does not know its own hue until it is tinted. This is the choice
/// of ramp those recipes are resolved against.
public enum Tint: String, CaseIterable, Sendable {
  case primary
  case neutral
  case info
  case success
  case warning
  case danger
}

/// The `control.color.*` recipes, which is the whole set: `recessed` is the
/// workaday grey chip and the default, `filled` the one action a view points
/// at, `normal` the raised white neighbour, `tinted` a quiet accent chip,
/// `outlined` its drawn twin, and `plain` text alone.
public enum ControlVariant: String, CaseIterable, Sendable {
  case recessed
  case normal
  case filled
  case tinted
  case outlined
  case plain
}

/// The four control sizes, each a profile of geometry and type in
/// `tokens/global/control.yaml`.
public enum ControlSize: String, CaseIterable, Sendable {
  case tiny
  case small
  case medium
  case large
}

/// The interaction states the shared state machine resolves a recipe for.
public enum ControlState: Sendable {
  case normal
  case hovered
  case pressed
  case disabled

  /// What a pointer and an enabled flag add up to. Disabled outranks
  /// everything, which is what flattens a control to the groove.
  public init(hovered: Bool, pressed: Bool, enabled: Bool = true) {
    if !enabled {
      self = .disabled
    } else if pressed {
      self = .pressed
    } else if hovered {
      self = .hovered
    } else {
      self = .normal
    }
  }
}

/// The three parts a recipe paints.
public enum ControlPart: Sendable {
  case surface
  case content
  case border
}

/// A control's three resolved colours for one state.
public struct ControlColors: Sendable, Equatable {
  public let surface: Color
  public let content: Color
  public let border: Color
}

/// The geometry and type one `[data-size]` profile binds.
///
/// A sized control reads this once instead of restating geometry per size —
/// Button and IconButton keep no per-size branch at all, because the profile
/// carries the design's own control type.
public struct ControlMetrics: Sendable, Equatable {
  public let size: CGFloat
  public let iconSize: CGFloat
  public let gap: CGFloat
  public let paddingBlock: CGFloat
  public let paddingInline: CGFloat
  public let radius: CGFloat
  public let content: TypeStyle
}

public extension ThemeVariables {
  /// The ramp a tint selects — the Swift half of `control.css`'s
  /// `[data-tint] { --tint-600: var(--color-primary-600) }`.
  func ramp(_ tint: Tint) -> ColorSwatch {
    switch tint {
    case .primary: return colorPrimary
    case .neutral: return colorNeutral
    case .info: return colorInfo
    case .success: return colorSuccess
    case .warning: return colorWarning
    case .danger: return colorDanger
    }
  }

  /// The size profile `[data-size]` binds.
  func metrics(_ size: ControlSize) -> ControlMetrics {
    switch size {
    case .tiny:
      return ControlMetrics(
        size: controlTinySize,
        iconSize: controlTinyIconSize,
        gap: controlTinyGap,
        paddingBlock: controlTinyPaddingBlock,
        paddingInline: controlTinyPaddingInline,
        radius: controlTinyRadius,
        content: controlTinyContent
      )
    case .small:
      return ControlMetrics(
        size: controlSmallSize,
        iconSize: controlSmallIconSize,
        gap: controlSmallGap,
        paddingBlock: controlSmallPaddingBlock,
        paddingInline: controlSmallPaddingInline,
        radius: controlSmallRadius,
        content: controlSmallContent
      )
    case .medium:
      return ControlMetrics(
        size: controlMediumSize,
        iconSize: controlMediumIconSize,
        gap: controlMediumGap,
        paddingBlock: controlMediumPaddingBlock,
        paddingInline: controlMediumPaddingInline,
        radius: controlMediumRadius,
        content: controlMediumContent
      )
    case .large:
      return ControlMetrics(
        size: controlLargeSize,
        iconSize: controlLargeIconSize,
        gap: controlLargeGap,
        paddingBlock: controlLargePaddingBlock,
        paddingInline: controlLargePaddingInline,
        radius: controlLargeRadius,
        content: controlLargeContent
      )
    }
  }

  /// The recipe for one variant and part.
  func descriptor(_ variant: ControlVariant, _ part: ControlPart) -> ColorDescriptor {
    switch (variant, part) {
    case (.normal, .surface): return controlColorNormalSurface
    case (.normal, .content): return controlColorNormalContent
    case (.normal, .border): return controlColorNormalBorder
    case (.recessed, .surface): return controlColorRecessedSurface
    case (.recessed, .content): return controlColorRecessedContent
    case (.recessed, .border): return controlColorRecessedBorder
    case (.filled, .surface): return controlColorFilledSurface
    case (.filled, .content): return controlColorFilledContent
    case (.filled, .border): return controlColorFilledBorder
    case (.tinted, .surface): return controlColorTintedSurface
    case (.tinted, .content): return controlColorTintedContent
    case (.tinted, .border): return controlColorTintedBorder
    case (.outlined, .surface): return controlColorOutlinedSurface
    case (.outlined, .content): return controlColorOutlinedContent
    case (.outlined, .border): return controlColorOutlinedBorder
    case (.plain, .surface): return controlColorPlainSurface
    case (.plain, .content): return controlColorPlainContent
    case (.plain, .border): return controlColorPlainBorder
    }
  }

  /// Resolve one recipe against a ramp and a state.
  ///
  /// This is what the token build does for CSS in `plugins/control.mjs`, done
  /// at read time instead: a state names either a semantic colour, which is
  /// the same in every tint, or a shade off the tinted ramp — then the alpha
  /// applies to whichever it landed on.
  func resolve(
    _ descriptor: ColorDescriptor,
    tint: Tint,
    state: ControlState
  ) -> Color {
    let (color, shade, opacity) = descriptor.recipe(for: state)
    let seed = color ?? shade.map { ramp(tint)[$0] } ?? .clear
    return opacity.map { seed.opacity($0) } ?? seed
  }

  /// The three colours a control paints in one state — the Swift half of
  /// `--control-surface` / `--control-content` / `--control-border`.
  func controlColors(
    variant: ControlVariant,
    tint: Tint,
    state: ControlState
  ) -> ControlColors {
    ControlColors(
      surface: resolve(descriptor(variant, .surface), tint: tint, state: state),
      content: resolve(descriptor(variant, .content), tint: tint, state: state),
      border: resolve(descriptor(variant, .border), tint: tint, state: state)
    )
  }

  /// Keyboard focus: the system accent at a per-theme alpha, and never the
  /// control's own tint. macOS draws it hugging the control, offset 0.
  var focusRing: Color {
    colorPrimary[focusRingShade].opacity(focusRingAlpha)
  }

  /// The one transition the system uses — colour crossfades on interactive
  /// surfaces.
  var motion: Animation {
    motionEasing.animation(duration: motionDuration)
  }
}
