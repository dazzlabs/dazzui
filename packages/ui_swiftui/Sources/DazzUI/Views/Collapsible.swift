import FluentIcons
import SwiftUI

/// The header row of a disclosure, and the panel under it.
///
/// ``Collapsible`` is one of them; an ``Accordion`` is several that know about
/// each other, and it wears this same drawing. So the two parts below are
/// shared the way a menu's panel and rows are — one drawing, not two that
/// resemble each other.
struct DisclosureHeader<Meta: View>: View {
  let title: String
  let open: Bool
  let icon: FluentIcon?
  let enabled: Bool
  let toggle: () -> Void
  @ViewBuilder let meta: () -> Meta

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  var body: some View {
    let vars = theme.vars

    Pressable(
      action: toggle,
      enabled: enabled,
      cornerRadius: vars.controlContainerRadius
    ) { state in
      HStack(spacing: vars.spacing2) {
        // The chevron points along the row when shut and down the page when
        // open — one glyph rotated rather than two swapped, so the turn is the
        // animation. It is the only moving part.
        Icon(.chevronRight12Regular, size: vars.labelSmall.size)
          .rotationEffect(.degrees(open ? 90 : 0))
          .frame(width: vars.spacing3)
          .foregroundStyle(enabled ? vars.colorContentSubtle : disabledInk)
          .animation(vars.motion, value: open)
        if let icon {
          Icon(icon, size: vars.spacing4)
            .foregroundStyle(enabled ? vars.colorContentMuted : disabledInk)
        }
        Text(title)
          .lineLimit(1)
          .frame(maxWidth: .infinity, alignment: .leading)
        // A trailing aside — a count, a duration — steps back the way a menu
        // row's shortcut does, and never takes the row's weight.
        meta()
          .font(.system(size: vars.labelSmall.size, weight: vars.labelQuiet.weight))
          .foregroundStyle(vars.colorContentFaint)
      }
      .font(vars.labelMedium.font)
      .foregroundStyle(enabled ? vars.colorContent : disabledInk)
      .padding(vars.spacing2)
      .frame(maxWidth: .infinity, alignment: .leading)
      // The row is not on the shared control state machine. `normal` is the
      // right recipe for a row's wash, but its resting surface is opaque
      // paper, and a header that painted itself would cut a card in half; so
      // the wash is read on hover alone and the row rests transparent —
      // exactly what a menu row does.
      .background(wash(state: state))
      // A header row grows with what it holds — a badge, a wrapped title — so
      // it takes the shared corner for a box that grows.
      .clipShape(
        RoundedRectangle(
          cornerRadius: vars.controlContainerRadius,
          style: .circular
        )
      )
      .animation(vars.motion, value: state.controlState)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: vars.controlContainerRadius,
      width: vars.focusWidth
    )
  }

  private func wash(state: PressableState) -> Color {
    guard state.enabled, state.hovered || state.pressed else { return .clear }
    return theme.vars.resolve(
      theme.vars.descriptor(.normal, .surface),
      tint: .neutral,
      state: .hovered
    )
  }

  private var disabledInk: Color {
    theme.vars.resolve(
      theme.vars.descriptor(.normal, .content),
      tint: .neutral,
      state: .disabled
    )
  }
}

/// What a disclosure discloses.
///
/// The content is indented to the chevron's far edge, so a nested list lines
/// up under the label rather than under the glyph.
struct DisclosurePanel<Content: View>: View {
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    content()
      .font(vars.bodySmall.font)
      .lineSpacing(vars.bodySmall.lineSpacing)
      .foregroundStyle(vars.colorContentSecondary)
      .frame(maxWidth: .infinity, alignment: .leading)
      .padding(.leading, vars.spacing2 + vars.spacing3 + vars.spacing2)
      .padding(.trailing, vars.spacing2)
      .padding(.top, vars.spacing1)
      .padding(.bottom, vars.spacing3)
  }
}

/// One thing that opens.
///
/// The header row is the affordance and the panel is what it discloses. It is
/// the smallest of the three ways this system hides something: a collapsible
/// keeps its subject in place and only folds it, where a ``Popover`` moves it
/// off the page and a ``Dialog`` stops the page.
public struct Collapsible<Content: View, Meta: View>: View {
  public var title: String
  public var icon: FluentIcon?
  /// Drive it from outside. Left `nil` the view holds its own state.
  public var open: Bool?
  public var initiallyOpen: Bool
  public var onOpenChange: ((Bool) -> Void)?
  public var enabled: Bool
  @ViewBuilder public var meta: () -> Meta
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @State private var latched: Bool

  public init(
    title: String,
    icon: FluentIcon? = nil,
    open: Bool? = nil,
    initiallyOpen: Bool = false,
    onOpenChange: ((Bool) -> Void)? = nil,
    enabled: Bool = true,
    @ViewBuilder meta: @escaping () -> Meta,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.title = title
    self.icon = icon
    self.open = open
    self.initiallyOpen = initiallyOpen
    self.onOpenChange = onOpenChange
    self.enabled = enabled
    self.meta = meta
    self.content = content
    self._latched = State(initialValue: initiallyOpen)
  }

  private var isOpen: Bool { open ?? latched }

  public var body: some View {
    VStack(spacing: 0) {
      DisclosureHeader(
        title: title,
        open: isOpen,
        icon: icon,
        enabled: enabled,
        toggle: {
          let next = !isOpen
          if open == nil { latched = next }
          onOpenChange?(next)
        },
        meta: meta
      )
      if isOpen {
        DisclosurePanel(content: content)
      }
    }
    .animation(theme.vars.motion, value: isOpen)
  }
}

public extension Collapsible where Meta == EmptyView {
  /// A disclosure with nothing trailing on its header.
  init(
    title: String,
    icon: FluentIcon? = nil,
    open: Bool? = nil,
    initiallyOpen: Bool = false,
    onOpenChange: ((Bool) -> Void)? = nil,
    enabled: Bool = true,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      title: title,
      icon: icon,
      open: open,
      initiallyOpen: initiallyOpen,
      onOpenChange: onOpenChange,
      enabled: enabled,
      meta: { EmptyView() },
      content: content
    )
  }
}
