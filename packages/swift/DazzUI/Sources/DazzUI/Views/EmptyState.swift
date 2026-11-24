import FluentIcons
import SwiftUI

/// What a pane shows before it has content: a muted line, optional icon and
/// supporting description, and at most one row of actions.
public struct EmptyState<Leading: View, Action: View>: View {
  public var title: String
  public var description: String?
  /// Optional leading icon or illustration shown above the title.
  public var icon: (() -> Leading)?
  /// The way out of the empty state — usually one button.
  @ViewBuilder public var action: () -> Action

  @Environment(\.theme) private var theme

  public init(
    title: String,
    description: String? = nil,
    @ViewBuilder icon: @escaping () -> Leading,
    @ViewBuilder action: @escaping () -> Action
  ) {
    self.title = title
    self.description = description
    self.icon = icon
    self.action = action
  }

  init(
    title: String,
    description: String?,
    iconBuilder: (() -> Leading)?,
    action: @escaping () -> Action
  ) {
    self.title = title
    self.description = description
    self.icon = iconBuilder
    self.action = action
  }

  public var body: some View {
    let vars = theme.vars

    VStack(spacing: vars.spacing3) {
      if let icon {
        icon()
          .environment(\.dazzIconSize, vars.iconLarge)
          .foregroundStyle(vars.colorContentFaint)
      }
      VStack(spacing: vars.spacing1) {
        Text(title)
          .font(description != nil ? vars.titleSmall.font : vars.bodyMedium.font)
          .lineBox(description != nil ? vars.titleSmall : vars.bodyMedium)
          .multilineTextAlignment(.center)
          .foregroundStyle(description != nil ? vars.colorContent : vars.colorContentSubtle)
        if let description {
          Text(description)
            .font(vars.bodySmall.font)
            .lineBox(vars.bodySmall)
            .multilineTextAlignment(.center)
            .foregroundStyle(vars.colorContentSubtle)
        }
      }
      // A row of controls, so it takes the control gap rather than the block
      // gap.
      if Action.self != EmptyView.self {
        HStack(spacing: vars.controlMediumGap) { action() }
      }
    }
    .padding(.vertical, vars.spacing14)
    .padding(.horizontal, vars.spacing8)
    // `flex: 1`: it takes the room its container gives, and with none to
    // give it is its own height — the padding and one line.
    .frame(maxWidth: .infinity, maxHeight: .infinity)
  }
}

public extension EmptyState where Leading == EmptyView {
  /// An empty state with a title, optional description, and action.
  init(
    title: String,
    description: String? = nil,
    @ViewBuilder action: @escaping () -> Action
  ) {
    self.init(title: title, description: description, iconBuilder: nil, action: action)
  }
}

public extension EmptyState where Leading == Icon {
  /// An empty state with a Fluent icon, title, optional description, and action.
  init(
    icon: FluentIcon,
    title: String,
    description: String? = nil,
    @ViewBuilder action: @escaping () -> Action
  ) {
    self.init(
      title: title,
      description: description,
      iconBuilder: { Icon(icon) },
      action: action
    )
  }
}

public extension EmptyState where Leading == Icon, Action == EmptyView {
  /// An empty state with a Fluent icon, title, and optional description.
  init(
    icon: FluentIcon,
    title: String,
    description: String? = nil
  ) {
    self.init(
      title: title,
      description: description,
      iconBuilder: { Icon(icon) },
      action: { EmptyView() }
    )
  }
}

public extension EmptyState where Action == EmptyView {
  /// An empty state with a custom icon view, title, and optional description.
  init(
    _ title: String,
    description: String? = nil,
    @ViewBuilder icon: @escaping () -> Leading
  ) {
    self.init(
      title: title,
      description: description,
      iconBuilder: icon,
      action: { EmptyView() }
    )
  }
}

public extension EmptyState where Leading == EmptyView, Action == EmptyView {
  /// An empty state with no way out — a pane that fills itself.
  init(_ title: String, description: String? = nil) {
    self.init(title: title, description: description, iconBuilder: nil, action: { EmptyView() })
  }
}
