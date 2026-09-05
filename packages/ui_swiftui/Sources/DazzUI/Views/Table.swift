import SwiftUI

/// Which ramp a selected row is picked out with.
///
/// In the React package the tint rides on the table's `data-tint` and reaches
/// the rows through the cascade. There is no cascade here, so it travels the
/// one way a subtree value travels in SwiftUI.
private struct TableTintKey: EnvironmentKey {
  static let defaultValue = Tint.primary
}

private extension EnvironmentValues {
  var tableTint: Tint {
    get { self[TableTintKey.self] }
    set { self[TableTintKey.self] = newValue }
  }
}

/// A stack of rows rather than a `SwiftUI.Table`: rows here are often
/// clickable and column widths are set by the design, both of which a real
/// table view fights. Named `Table`, and reached as `DazzUI.Table` where
/// `SwiftUI.Table` is also in scope.
public struct Table<Content: View>: View {
  /// Which ramp a selected row is picked out with.
  public var tint: Tint
  @ViewBuilder public var content: () -> Content

  public init(tint: Tint = .primary, @ViewBuilder content: @escaping () -> Content) {
    self.tint = tint
    self.content = content
  }

  public var body: some View {
    VStack(spacing: 0) {
      content()
    }
    .environment(\.tableTint, tint)
    .accessibilityElement(children: .contain)
  }
}

/// The column-heading row: the muted band over the body, ruled off from it.
public struct TableHead<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    HStack(spacing: 0) {
      content()
    }
    .padding(.horizontal, vars.spacing5)
    .background(vars.colorSurfaceChrome)
    .rule(vars.colorBorder, edge: .bottom, width: hairline)
  }
}

/// One row of the body.
public struct TableRow<Content: View>: View {
  public var active: Bool
  /// Makes the row selectable, and reachable from the keyboard.
  public var interactive: Bool
  public var action: (() -> Void)?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @Environment(\.tableTint) private var tint
  @FocusState private var focused: Bool

  public init(
    active: Bool = false,
    interactive: Bool = false,
    action: (() -> Void)? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.active = active
    self.interactive = interactive
    self.action = action
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    if interactive {
      Pressable(action: action, cornerRadius: vars.radiusNone) { state in
        row(hovered: state.hovered)
      }
      .accessibilityAddTraits(active ? [.isSelected] : [])
      .focused($focused)
      .focusRing(
        visible: focused,
        color: vars.focusRing,
        cornerRadius: vars.radiusNone,
        width: vars.focusWidth
      )
    } else {
      row(hovered: false)
    }
  }

  private func row(hovered: Bool) -> some View {
    let vars = theme.vars

    return HStack(spacing: 0) {
      content()
    }
    .font(vars.bodyMedium.font)
    .lineSpacing(vars.bodyMedium.lineSpacing)
    .foregroundStyle(vars.colorContent)
    .frame(maxWidth: .infinity, alignment: .leading)
    .padding(.vertical, vars.spacing3)
    .padding(.horizontal, vars.spacing5)
    .background(background(hovered: hovered))
    .rule(vars.colorBorder, edge: .bottom, width: hairline)
    .animation(vars.motion, value: hovered)
  }

  /// Hover outranks selection, the way the two rules land in the stylesheet:
  /// the pointer answers first, and the band comes back when it leaves.
  private func background(hovered: Bool) -> Color {
    let vars = theme.vars
    if hovered {
      // Neutral, because the tinted selection sits one step lower on the ramp
      // and a tinted hover would read as the heavier of the two.
      return vars.resolve(
        vars.descriptor(.normal, .surface),
        tint: tint,
        state: .hovered
      )
    }
    guard active else { return .clear }
    // The surface wash, not the `tinted` recipe's chip fill: a selected row is
    // a band the width of the table, and the chip's 12% reads as a block on it.
    return vars.ramp(tint)[600].opacity(vars.washSurface)
  }
}

/// Where a cell's content sits in its column.
public enum TableCellAlignment: String, CaseIterable, Sendable {
  case start
  case end
}

/// One cell, in a body row or — with `head` — in the heading band.
public struct TableCell<Content: View>: View {
  /// Renders as a column heading.
  public var head: Bool
  public var align: TableCellAlignment
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    head: Bool = false,
    align: TableCellAlignment = .start,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.head = head
    self.align = align
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    content()
      .modifier(HeadFace(head: head, vars: vars))
      .frame(maxWidth: .infinity, alignment: align == .end ? .trailing : .leading)
      .multilineTextAlignment(align == .end ? .trailing : .leading)
      .padding(.vertical, head ? vars.spacing2 : 0)
  }
}

/// A heading cell steps down to the labelling face and ink; a body cell keeps
/// whatever the row set.
private struct HeadFace: ViewModifier {
  let head: Bool
  let vars: ThemeVariables

  func body(content: Content) -> some View {
    if head {
      content
        .font(vars.labelSmall.font)
        .lineSpacing(vars.labelSmall.lineSpacing)
        .foregroundStyle(vars.colorContentFaint)
    } else {
      content
    }
  }
}
