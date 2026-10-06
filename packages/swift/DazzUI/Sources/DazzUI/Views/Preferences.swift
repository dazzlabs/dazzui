import SwiftUI

// The settings column.
//
// The spacing is the whole design here. Rows sit close together, a heading
// sits further from the section above it than from its own rows, a group
// title stands further still, and one group stands furthest from the next —
// so a heading always reads as belonging to what follows it rather than
// floating between two blocks. The ladder is decided in ds — one
// `preferences*Gap` per level, and the pad a row's text wears — and nothing
// here restates a step. One thing the React sheet does that a `VStack`
// cannot: a rule between two children is drawn in the middle of the run
// rather than given the run on each side, since a view cannot be told from
// its siblings here. A story that draws one carries the negative padding
// itself.

/// The settings column, and the root the rest of this file sits in.
///
/// It owns the two things no group or section can: the run between groups,
/// which is the widest step of the spacing ladder this column climbs, and the
/// width. The column is capped rather than filling its pane — a row puts its
/// label on one edge and its control on the other, and past
/// `preferencesWidth` the two stop reading as one line.
public struct Preferences<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.preferencesGroupGap) {
      content()
    }
    // Full width up to the cap, so it fills a narrow pane and stops in a wide
    // one: `width: 100%; max-width` in the stylesheet.
    .frame(maxWidth: vars.preferencesWidth, alignment: .leading)
  }
}

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
        .padding(.bottom, vars.preferencesHeadingGap)
      }

      VStack(alignment: .leading, spacing: vars.preferencesRowGap) {
        content()
      }

      if let footer {
        Text(footer)
          .font(vars.captionSmall.font)
          .lineSpacing(vars.captionSmall.size * PreferenceLeading.note - vars.captionSmall.size)
          .foregroundStyle(vars.colorContentSubtle)
          .padding(.top, vars.preferencesFooterGap)
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
        // Half a step: the title's leading and the description's already
        // open the pair.
        VStack(alignment: .leading, spacing: vars.spacing05) {
          // The group outranks its sections typographically, and it has to
          // outrank the rows by more than a weight: a row's title is the same
          // 12px, so a title one grade heavier read as one more row.
          // `titleMedium` — two sizes up, at the label weight — is the first
          // face that stands over a row. A section heading stays a
          // SectionLabel.
          Text(title)
            .font(vars.titleMedium.font)
            .lineBox(vars.titleMedium)
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
      .padding(.bottom, vars.preferencesTitleGap)

      VStack(alignment: .leading, spacing: vars.preferencesSectionGap) {
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
      // The row's corner is the shared one for a box that grows: the control
      // step everywhere, the container step in Bright.
      Pressable(action: onOpen, cornerRadius: vars.controlContainerRadius) { state in
        row
          .padding(.horizontal, vars.spacing2)
          .background(background(hovered: state.hovered || state.pressed))
          .clipShape(
            RoundedRectangle(
              cornerRadius: vars.controlContainerRadius,
              style: .circular
            )
          )
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
        cornerRadius: vars.controlContainerRadius,
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
        .environment(\.dazzIconSize, vars.iconMedium)
      VStack(alignment: .leading, spacing: vars.spacing1) {
        // A row rests at `labelQuiet`, the way a list row does: every title at
        // the label weight was a column of bold with nothing left to outrank
        // it.
        Text(title)
          .font(vars.labelQuiet.font)
          .lineBox(vars.labelQuiet)
          .foregroundStyle(vars.colorContent)
          .lineLimit(1)
          .truncationMode(.tail)
        if let subtitle {
          Text(subtitle)
            .font(vars.captionSmall.font)
            .lineBox(vars.captionSmall)
            .foregroundStyle(vars.colorContentSubtle)
            .lineLimit(1)
            .truncationMode(.tail)
        }
      }
      // The pad rides on the text, so a row that grows a subtitle keeps air
      // above the title and below it. A single line padded this way is shorter
      // than the control height, and the row's minimum still centres it.
      .padding(.vertical, vars.preferencesRowPadding)
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
