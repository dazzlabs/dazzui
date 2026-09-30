import FluentIcons
import SwiftUI

/// What a step has to say about itself: finished, running, or still ahead.
public enum StepStatus: String, CaseIterable, Sendable {
  case done
  case active
  case idle
}

/// One stage of a pipeline, carrying its own status.
///
/// The hierarchy runs label-over-meta, and only the active row takes the label
/// weight — done and idle rows rest at the quiet weight so the column reads as
/// one live step among finished and pending ones.
public struct Step<Label: View, Meta: View>: View {
  public var status: StepStatus
  public var tint: Tint
  @ViewBuilder public var label: () -> Label
  /// Trailing detail — a count, a duration.
  @ViewBuilder public var meta: () -> Meta

  @Environment(\.theme) private var theme

  public init(
    status: StepStatus,
    tint: Tint = .primary,
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder meta: @escaping () -> Meta
  ) {
    self.status = status
    self.tint = tint
    self.label = label
    self.meta = meta
  }

  public var body: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing25) {
      if status == .active {
        Spinner(size: .medium, tint: tint)
      } else {
        marker
      }
      label()
      Spacer(minLength: 0)
      meta()
        .font(.system(size: vars.labelSmall.size, weight: vars.captionSmall.weight))
        .foregroundStyle(metaInk)
    }
    .font(.system(size: vars.labelQuiet.size, weight: weight))
    .lineSpacing(vars.labelQuiet.lineSpacing)
    .foregroundStyle(ink)
    // Not started yet: the whole row recedes, marker and meta included — the
    // plan is visible, the past and present are what read.
    .opacity(status == .idle ? Self.idleOpacity : 1)
  }

  /// The step the stylesheet drops an unstarted row by. Not a token: it is a
  /// recession applied to the whole row rather than a colour of its own.
  private static var idleOpacity: CGFloat { 0.8 }

  /// The wash a finished marker fills with — the stylesheet's own 14%, which
  /// no shared wash covers.
  private static var doneWash: CGFloat { 0.14 }

  private var marker: some View {
    let vars = theme.vars

    // `Checkmark12Filled`, the glyph `step-list.tsx` names.
    return Icon(.checkmark12Filled, size: vars.iconSmall)
      .opacity(status == .done ? 1 : 0)
      .foregroundStyle(vars.colorSuccess[600])
      .frame(width: vars.iconMedium, height: vars.iconMedium)
      // Done is success wherever the list is tinted: a finished step is not a
      // variation on the accent, it is the one fixed meaning in the column.
      // The fill is a wash of the hue rather than an opaque chip, so it sits
      // as correctly on a card or a dark canvas as on the paper.
      .background(
        status == .done ? vars.colorSuccess[600].opacity(Self.doneWash) : .clear
      )
      .clipShape(Circle())
      .overlay {
        if status == .idle {
          Circle().strokeBorder(vars.colorBorderMuted, lineWidth: vars.strokeControl)
        }
      }
  }

  private var ink: Color {
    switch status {
    case .done: return theme.vars.colorContentMuted
    case .active: return theme.vars.colorContent
    case .idle: return theme.vars.colorContentSubtle
    }
  }

  private var weight: Font.Weight {
    status == .active ? theme.vars.labelMedium.weight : theme.vars.labelQuiet.weight
  }

  /// The trailing detail runs two ink steps back; only the active row's meta
  /// takes the accent — in its text grade, not the fill.
  private var metaInk: Color {
    let vars = theme.vars
    guard status == .active else { return vars.colorContentFaint }
    return vars.resolve(vars.descriptor(.plain, .content), tint: tint, state: .normal)
  }
}

public extension Step where Meta == EmptyView {
  /// A step with no trailing detail.
  init(
    status: StepStatus,
    tint: Tint = .primary,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.init(status: status, tint: tint, label: label, meta: { EmptyView() })
  }
}

public extension Step where Label == Text, Meta == EmptyView {
  /// A step whose label is a plain string.
  init(_ label: String, status: StepStatus, tint: Tint = .primary) {
    self.init(status: status, tint: tint, label: { Text(label) }, meta: { EmptyView() })
  }
}

public extension Step where Label == Text, Meta == Text {
  /// A step whose label and trailing detail are plain strings.
  init(_ label: String, meta: String, status: StepStatus, tint: Tint = .primary) {
    self.init(status: status, tint: tint, label: { Text(label) }, meta: { Text(meta) })
  }
}

/// The column the steps sit in. An ordered list, because the steps are one:
/// the markers carry the status, so the numbering is dropped rather than the
/// semantics.
public struct StepList<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    VStack(alignment: .leading, spacing: theme.vars.spacing2) {
      content()
    }
    .accessibilityElement(children: .contain)
  }
}
