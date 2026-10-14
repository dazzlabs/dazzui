import SwiftUI

/// One shape, three densities. The type never changes, only the box around
/// it — a callout that grew its type would be competing with the content it
/// interrupts.
public enum CalloutSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A tinted notice.
///
/// It always wears a tinted treatment — a callout that needed a fill louder
/// than this would be a dialog, not a callout — so `tint` is the only colour
/// axis. Fill and edge are washes of that tint rather than opaque steps, so
/// the callout sits as correctly on a card as on the paper.
///
/// The alphas are the shared surface wash rather than the `tinted` recipe's.
/// That recipe fills a chip at a chip's strength, which is right for something
/// the size of a word and twice too heavy spread across a full width. A callout
/// is read rather than pressed, so it stays out of the shared state machine
/// and takes the resting text grades directly.
public struct Callout<Leading: View, Description: View, Trailing: View>: View {
  public var size: CalloutSize
  /// Which colour ramp the washes and text grades resolve against.
  public var tint: Tint
  /// Omit for a one-line notice; the description then carries it alone.
  public var title: String?
  /// Leading slot — an icon or a ``Spinner``.
  @ViewBuilder public var leading: () -> Leading
  /// The description.
  @ViewBuilder public var description: () -> Description
  /// Trailing affordance, pushed to the far edge.
  @ViewBuilder public var trailing: () -> Trailing

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    size: CalloutSize = .medium,
    tint: Tint = .neutral,
    title: String? = nil,
    @ViewBuilder leading: @escaping () -> Leading,
    @ViewBuilder description: @escaping () -> Description,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.size = size
    self.tint = tint
    self.title = title
    self.leading = leading
    self.description = description
    self.trailing = trailing
  }

  public var body: some View {
    let vars = theme.vars

    HStack(alignment: hasDescription ? .top : .center, spacing: vars.spacing25) {
      leading()
        .environment(\.dazzIconSize, vars.iconMedium)
        .foregroundStyle(vars.ramp(tint)[600])
      VStack(alignment: .leading, spacing: vars.spacing1) {
        if let title {
          Text(title)
            .font(vars.labelMedium.font)
            .lineBox(vars.labelMedium, lineHeight: titleLeading * vars.labelMedium.size)
            .foregroundStyle(grade(.hovered))
        }
        description()
          .font(.system(size: vars.labelQuiet.size, weight: vars.captionSmall.weight))
          .lineBox(vars.labelQuiet, lineHeight: bodyLeading * vars.labelQuiet.size)
          .foregroundStyle(grade(.normal))
      }
      .frame(maxWidth: .infinity, alignment: .leading)
      trailing()
    }
    .padding(.vertical, paddingBlock + hairline)
    .padding(.horizontal, paddingInline + hairline)
    .background(vars.ramp(tint)[600].opacity(vars.washSurface))
    .clipShape(RoundedRectangle(cornerRadius: radius, style: .circular))
    .border(
      vars.ramp(tint)[600].opacity(vars.washEdge),
      width: hairline,
      cornerRadius: radius
    )
  }

  /// Once a description sits under a title, the icon belongs beside the first
  /// line rather than halfway down the block.
  private var hasDescription: Bool {
    title != nil && Description.self != EmptyView.self
  }

  /// The text grades of the tint, not its fill.
  private func grade(_ state: ControlState) -> Color {
    theme.vars.resolve(
      theme.vars.descriptor(.tinted, .content),
      tint: tint,
      state: state
    )
  }

  private var paddingBlock: CGFloat {
    switch size {
    case .small: return theme.vars.spacing2
    case .medium: return theme.vars.spacing3
    case .large: return theme.vars.spacing4
    }
  }

  private var paddingInline: CGFloat {
    switch size {
    case .small: return theme.vars.spacing3
    case .medium: return theme.vars.spacing35
    case .large: return theme.vars.spacing4
    }
  }

  private var radius: CGFloat {
    // The small size is the one whose box takes the control step; it is still a
    // box that grows, so it takes the shared corner for one.
    size == .small ? theme.vars.controlContainerRadius : theme.vars.radiusLarge
  }

  /// The CSS states these leadings as ratios of the face rather than as
  /// lengths, so they are stated the same way here.
  private let titleLeading: CGFloat = 1.4
  private let bodyLeading: CGFloat = 1.5
}

public extension Callout where Leading == EmptyView, Trailing == EmptyView {
  /// A callout with no icon and no action.
  init(
    size: CalloutSize = .medium,
    tint: Tint = .neutral,
    title: String? = nil,
    @ViewBuilder description: @escaping () -> Description
  ) {
    self.init(
      size: size,
      tint: tint,
      title: title,
      leading: { EmptyView() },
      description: description,
      trailing: { EmptyView() }
    )
  }
}

public extension Callout
where Leading == EmptyView, Description == Text, Trailing == EmptyView {
  /// A callout whose description is a plain string.
  init(
    _ description: String,
    size: CalloutSize = .medium,
    tint: Tint = .neutral,
    title: String? = nil
  ) {
    self.init(
      size: size,
      tint: tint,
      title: title,
      description: { Text(description) }
    )
  }
}
