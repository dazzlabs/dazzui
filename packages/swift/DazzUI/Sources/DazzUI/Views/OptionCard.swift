import SwiftUI

/// One choice in a radio group of cards — the shape a small set of mutually
/// exclusive choices takes when each needs a line of explanation and a radio
/// row would not carry it.
///
/// The type runs title-over-description in the strong/caption pair: the
/// display cut at 700 over a caption a step down. The card is one of the two
/// places the hierarchy is carried by *both* size and weight, because the two
/// lines sit closer than anywhere else in the system.
public struct OptionCard<Title: View, Description: View>: View {
  public var selected: Bool
  public var tint: Tint
  public var enabled: Bool
  public var action: () -> Void
  @ViewBuilder public var title: () -> Title
  @ViewBuilder public var description: () -> Description
  /// The inset of the card's content. A card whose title is a picture of
  /// the option takes zero, and the picture runs to the card's edge.
  public var padding: EdgeInsets?
  /// A corner fixed by the host rather than the theme's own radius.
  public var cornerRadius: CGFloat?

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @FocusState private var focused: Bool

  public init(
    selected: Bool = false,
    tint: Tint = .primary,
    enabled: Bool = true,
    padding: EdgeInsets? = nil,
    cornerRadius: CGFloat? = nil,
    action: @escaping () -> Void,
    @ViewBuilder title: @escaping () -> Title,
    @ViewBuilder description: @escaping () -> Description
  ) {
    self.padding = padding
    self.cornerRadius = cornerRadius
    self.selected = selected
    self.tint = tint
    self.enabled = enabled
    self.action = action
    self.title = title
    self.description = description
  }

  public var body: some View {
    let vars = theme.vars
    let radius = cornerRadius ?? vars.radiusLarge

    Pressable(action: action, enabled: enabled, cornerRadius: radius) { state in
      VStack(alignment: .leading, spacing: padding == nil ? vars.spacing15 : 0) {
        title()
          .font(.system(size: vars.labelStrong.size, weight: vars.labelStrong.weight))
          .lineBox(vars.labelStrong)
          .foregroundStyle(vars.colorContent)
        description()
          .font(.system(size: vars.labelSmall.size, weight: vars.captionSmall.weight))
          // The caption sets its own leading here rather than taking the
          // caption face's, at the roomier ratio a wrapped explanation wants.
          .lineBox(vars.labelSmall, lineHeight: vars.labelSmall.size * Self.descriptionLeading)
          .foregroundStyle(descriptionInk)
          .animation(vars.motion, value: selected)
      }
      .frame(maxWidth: .infinity, alignment: .leading)
      .multilineTextAlignment(.leading)
      // The disabled ink the two lines then override, exactly as the
      // stylesheet's `:disabled { color }` is overridden by the spans.
      .foregroundStyle(
        state.enabled
          ? vars.colorContent
          : vars.resolve(vars.descriptor(.normal, .content), tint: tint, state: .disabled)
      )
      .padding(
        padding
          ?? EdgeInsets(
            top: vars.spacing35,
            leading: vars.spacing3,
            bottom: vars.spacing35,
            trailing: vars.spacing3
          )
      )
      .padding(hairline)
      .background(surface(state: state))
      .clipShape(RoundedRectangle(cornerRadius: radius, style: .circular))
      // Selection *thickens* the border as well as colouring it, which is what
      // makes the chosen card survive a glance across its neighbours.
      .border(
        border,
        width: selected ? vars.strokeControl : hairline,
        cornerRadius: radius
      )
      .animation(vars.motion, value: state.controlState)
    }
    .accessibilityAddTraits(selected ? [.isSelected] : [])
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: radius,
      width: vars.focusWidth
    )
  }

  /// The 1.5 ratio the stylesheet sets on the description, kept as a ratio
  /// because SwiftUI wants the gap between lines rather than the line box.
  private static var descriptionLeading: CGFloat { 1.5 }

  private func surface(state: PressableState) -> Color {
    let vars = theme.vars
    guard state.enabled else {
      return vars.resolve(vars.descriptor(.normal, .surface), tint: tint, state: .disabled)
    }
    // The wash, not the `tinted` recipe's chip fill: a card is a surface, and
    // the chip's 12% fills it rather than tinting it. The border is what says
    // chosen; the fill only has to agree with it.
    if selected { return vars.ramp(tint)[600].opacity(vars.washSurface) }
    if state.hovered || state.pressed { return vars.colorSurfaceSubtle }
    return vars.colorSurfaceMuted
  }

  private var border: Color {
    let vars = theme.vars
    guard selected else { return vars.colorBorder }
    return vars.resolve(vars.descriptor(.outlined, .border), tint: tint, state: .normal)
  }

  /// A chosen card's explanation steps up an ink grade — it is being read
  /// rather than skimmed past.
  private var descriptionInk: Color {
    selected ? theme.vars.colorContentMuted : theme.vars.colorContentSubtle
  }
}

public extension OptionCard where Title == Text, Description == Text {
  /// A card whose two lines are plain strings.
  init(
    _ title: String,
    description: String,
    selected: Bool = false,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: @escaping () -> Void
  ) {
    self.init(
      selected: selected,
      tint: tint,
      enabled: enabled,
      action: action,
      title: { Text(title) },
      description: { Text(description) }
    )
  }
}

public extension OptionCard where Title == Text, Description == EmptyView {
  /// A card that is a title alone.
  init(
    _ title: String,
    selected: Bool = false,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: @escaping () -> Void
  ) {
    self.init(
      selected: selected,
      tint: tint,
      enabled: enabled,
      action: action,
      title: { Text(title) },
      description: { EmptyView() }
    )
  }
}
