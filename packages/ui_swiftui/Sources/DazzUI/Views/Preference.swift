import SwiftUI

// The settings column.
//
// The spacing is the whole design here. Rows sit close together, a heading
// sits further from the section above it than from its own rows, and a group
// title stands further still — so a heading always reads as belonging to what
// follows it rather than floating between two blocks.

/// A labelled run of preference rows.
public struct PreferenceSection<Content: View, Action: View>: View {
  /// The heading over the rows. Omit for an unlabelled run.
  public var label: String?
  /// A muted note under the rows, for what the section cannot fit.
  public var footer: String?
  /// A control on the heading line, pushed to the trailing edge.
  @ViewBuilder public var action: () -> Action
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    label: String? = nil,
    footer: String? = nil,
    @ViewBuilder action: @escaping () -> Action,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.label = label
    self.footer = footer
    self.action = action
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: 0) {
      if label != nil || Action.self != EmptyView.self {
        HStack(spacing: vars.spacing4) {
          if let label {
            SectionLabel(label)
          }
          Spacer(minLength: 0)
          HeadingAction { action() }
        }
        .padding(.bottom, vars.spacing3)
      }

      VStack(alignment: .leading, spacing: vars.spacing1) {
        content()
      }

      if let footer {
        Text(footer)
          .font(vars.captionSmall.font)
          .lineSpacing(vars.captionSmall.size * PreferenceLeading.note - vars.captionSmall.size)
          .foregroundStyle(vars.colorContentSubtle)
          .padding(.top, vars.spacing2)
      }
    }
  }
}

public extension PreferenceSection where Action == EmptyView {
  /// A section whose heading carries no control.
  init(
    label: String? = nil,
    footer: String? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(label: label, footer: footer, action: { EmptyView() }, content: content)
  }
}

/// A titled run of sections, for a page whose sections come in bunches.
///
/// Sections stay one size everywhere; the level above them is a title that
/// outranks them, not a section shrunk to fit underneath one. A heading then
/// means the same thing wherever it is met.
public struct PreferenceGroup<Content: View, Action: View>: View {
  public var title: String
  /// The line under the title, for what the group is for.
  public var description: String?
  public var action: () -> Action
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    title: String,
    description: String? = nil,
    @ViewBuilder action: @escaping () -> Action,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.title = title
    self.description = description
    self.action = action
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: 0) {
      HStack(spacing: vars.spacing4) {
        VStack(alignment: .leading, spacing: vars.spacing15) {
          // The group outranks its sections typographically: the display cut
          // at the strong weight, where a section heading is a SectionLabel.
          Text(title)
            .font(vars.labelStrong.font)
            .lineSpacing(vars.labelStrong.lineSpacing)
            .foregroundStyle(vars.colorContent)
          if let description {
            Text(description)
              .font(vars.captionSmall.font)
              .lineSpacing(
                vars.captionSmall.size * PreferenceLeading.note - vars.captionSmall.size
              )
              .foregroundStyle(vars.colorContentSubtle)
          }
        }
        Spacer(minLength: 0)
        HeadingAction { action() }
      }
      .padding(.bottom, vars.spacing3)

      VStack(alignment: .leading, spacing: vars.spacing4 + vars.spacing05) {
        content()
      }
    }
  }
}

public extension PreferenceGroup where Action == EmptyView {
  /// A group whose heading carries no control.
  init(
    title: String,
    description: String? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(title: title, description: description, action: { EmptyView() }, content: content)
  }
}

/// One preference row: title and optional subtitle left, control right.
///
/// A row's height is a minimum plus a pad on its text, never a fixed height.
/// The minimum keeps the text column in one rhythm whatever control sits on
/// the right; the pad rides on the text so a row grows for content — a
/// subtitle, a wrapped title — and never for a control.
public struct PreferenceRow<Icon: View, Trailing: View>: View {
  public var title: String
  /// The second line, for what the title cannot say in three words.
  public var subtitle: String?
  /// Makes the row open something. It grows the hover wash that says so; rows
  /// that only carry a control leave this off and stay inert.
  public var onOpen: (() -> Void)?
  public var tint: Tint
  /// A glyph or avatar before the title, for rows that name a thing.
  @ViewBuilder public var icon: () -> Icon
  @ViewBuilder public var trailing: () -> Trailing

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    title: String,
    subtitle: String? = nil,
    onOpen: (() -> Void)? = nil,
    tint: Tint = .neutral,
    @ViewBuilder icon: @escaping () -> Icon,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.title = title
    self.subtitle = subtitle
    self.onOpen = onOpen
    self.tint = tint
    self.icon = icon
    self.trailing = trailing
  }

  public var body: some View {
    let vars = theme.vars

    if let onOpen {
      Pressable(action: onOpen, cornerRadius: vars.radiusSmall) { state in
        row
          .padding(.horizontal, vars.spacing2)
          .background(background(hovered: state.hovered || state.pressed))
          .clipShape(RoundedRectangle(cornerRadius: vars.radiusSmall, style: .circular))
          .animation(vars.motion, value: state.controlState)
      }
      // Pulled out and pushed back in, so the wash is wider than the text
      // while the row still starts on the same left edge as the inert rows
      // above it.
      .padding(.horizontal, -vars.spacing2)
      .focused($focused)
      .focusRing(
        visible: focused,
        color: vars.focusRing,
        cornerRadius: vars.radiusSmall,
        width: vars.focusWidth
      )
    } else {
      row
    }
  }

  private var row: some View {
    let vars = theme.vars

    return HStack(spacing: vars.spacing25) {
      // Uncoloured on purpose: a Badge or a Switch in these slots keeps its
      // own ink rather than inheriting a wash.
      icon()
      VStack(alignment: .leading, spacing: vars.spacing1) {
        Text(title)
          .font(vars.labelMedium.font)
          .lineSpacing(vars.labelMedium.lineSpacing)
          .foregroundStyle(vars.colorContent)
          .lineLimit(1)
          .truncationMode(.tail)
        if let subtitle {
          Text(subtitle)
            .font(vars.captionSmall.font)
            .lineSpacing(vars.captionSmall.lineSpacing)
            .foregroundStyle(vars.colorContentSubtle)
            .lineLimit(1)
            .truncationMode(.tail)
        }
      }
      .padding(.vertical, vars.spacing1)
      Spacer(minLength: 0)
      if Trailing.self != EmptyView.self || onOpen != nil {
        HStack(spacing: vars.spacing25) {
          trailing()
          if onOpen != nil {
            Text("›")
          }
        }
      }
    }
    .multilineTextAlignment(.leading)
    .frame(maxWidth: .infinity, minHeight: vars.controlMediumSize, alignment: .leading)
  }

  /// The wash previews selection — the accent at 8% — because an interactive
  /// row here opens something; a neutral wash is for rows that merely light.
  private func background(hovered: Bool) -> Color {
    guard hovered else { return .clear }
    let vars = theme.vars
    return vars.resolve(vars.descriptor(.plain, .surface), tint: tint, state: .hovered)
  }
}

public extension PreferenceRow where Icon == EmptyView {
  /// A row with no leading glyph.
  init(
    title: String,
    subtitle: String? = nil,
    onOpen: (() -> Void)? = nil,
    tint: Tint = .neutral,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      onOpen: onOpen,
      tint: tint,
      icon: { EmptyView() },
      trailing: trailing
    )
  }
}

public extension PreferenceRow where Trailing == EmptyView {
  /// A row that carries no control on the right.
  init(
    title: String,
    subtitle: String? = nil,
    onOpen: (() -> Void)? = nil,
    tint: Tint = .neutral,
    @ViewBuilder icon: @escaping () -> Icon
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      onOpen: onOpen,
      tint: tint,
      icon: icon,
      trailing: { EmptyView() }
    )
  }
}

public extension PreferenceRow where Icon == EmptyView, Trailing == EmptyView {
  /// A row that is text alone.
  init(
    title: String,
    subtitle: String? = nil,
    onOpen: (() -> Void)? = nil,
    tint: Tint = .neutral
  ) {
    self.init(
      title: title,
      subtitle: subtitle,
      onOpen: onOpen,
      tint: tint,
      icon: { EmptyView() },
      trailing: { EmptyView() }
    )
  }
}

/// A heading's control sits in a zero-height slot it overhangs on both sides.
///
/// Letting a 32pt control size the line would make a heading with a button
/// taller than one without, and two sections on the same page would then start
/// at different heights. Zero rather than a negative inset, so it holds for
/// any control height and any heading size.
private struct HeadingAction<Content: View>: View {
  @ViewBuilder let content: () -> Content

  var body: some View {
    content().frame(height: 0)
  }
}

/// The leading the notes under a heading are set at. A ratio rather than a
/// token, because that is how the stylesheet states it.
private enum PreferenceLeading {
  static let note: CGFloat = 1.7
}
