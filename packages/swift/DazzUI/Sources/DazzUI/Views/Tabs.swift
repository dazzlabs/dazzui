import FluentIcons
import SwiftUI

/// What a tab's dot says: nothing is drawn without it.
public enum TabStatus: String, CaseIterable, Sendable {
  case neutral
  case primary
  case success
  case warning
  case danger
}

/// One tab of ``Tabs``.
public struct TabItem<Value: Hashable>: Identifiable {
  public var value: Value
  /// The tab's name. It is also the tooltip when the tab is drawn as its icon alone.
  public var label: String
  public var icon: FluentIcon?
  /// Trailing count, in the label's own line.
  public var count: Int?
  /// A dot before the label: a state the reader cannot see while another tab is showing.
  public var status: TabStatus?
  /// Whether the dot pulses — something is running.
  public var busy: Bool
  /// A quiet second word after the label, e.g. the machine a terminal is on.
  public var detail: String?
  /// Whether the tab can be closed; `onClose` on the tabs is what closes it.
  public var closable: Bool
  /// The tooltip; the label by default.
  public var title: String?
  public var disabled: Bool

  public var id: Value { value }

  public init(
    value: Value,
    label: String,
    icon: FluentIcon? = nil,
    count: Int? = nil,
    status: TabStatus? = nil,
    busy: Bool = false,
    detail: String? = nil,
    closable: Bool = false,
    title: String? = nil,
    disabled: Bool = false
  ) {
    self.value = value
    self.label = label
    self.icon = icon
    self.count = count
    self.status = status
    self.busy = busy
    self.detail = detail
    self.closable = closable
    self.title = title
    self.disabled = disabled
  }
}

/// Tabs — `tabs.tsx`: the navigation of a view or a toolbar. Each is its name
/// in quiet ink, and the one showing is in full ink over a rule along the
/// row's foot — no fill, so a row of them sits in a head without outweighing
/// its title. Where ``PillTabs`` are filters standing on a page, these are
/// where you are.
///
/// A tab can lead with an icon, carry a count, a status dot and a close
/// button, so fixed sections (Pull Requests, Issues) and open documents
/// (editors, terminals) are the same control.
///
/// The row never scrolls and never wraps. When the tabs do not fit, the ones
/// not showing give up their names and keep their icons; when they still do
/// not fit, the rest go behind a more-button. The tab that is showing is
/// always whole.
public struct Tabs<Value: Hashable, Trailing: View>: View {
  public var items: [TabItem<Value>]
  public var value: Value?
  public var onValueChange: (Value) -> Void
  /// Closes a `closable` tab.
  public var onClose: ((Value) -> Void)?
  /// What the row is called — React's `aria-label` on the tab list.
  public var accessibilityLabel: String?
  /// What follows the last tab and stays put while they give way: the
  /// control that opens another.
  @ViewBuilder public var trailing: () -> Trailing

  @Environment(\.theme) private var theme
  /// What the ruler and the row measured: each tab whole and as its icon,
  /// what trails, and the room the row was given.
  @State private var measured = TabsMeasure()

  /// What the more-button takes of the row; reserved once anything has to go
  /// behind it.
  private static var moreWidth: CGFloat { 28 }

  public init(
    items: [TabItem<Value>],
    value: Value?,
    onValueChange: @escaping (Value) -> Void,
    onClose: ((Value) -> Void)? = nil,
    accessibilityLabel: String? = nil,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.items = items
    self.value = value
    self.onValueChange = onValueChange
    self.onClose = onClose
    self.accessibilityLabel = accessibilityLabel
    self.trailing = trailing
  }

  public var body: some View {
    let vars = theme.vars
    let fit = solve()
    let active = items.firstIndex { $0.value == value }

    // The tab that is showing is never the one behind the more-button: it
    // takes the last place in the row.
    var order = Array(items.indices)
    if let active, active >= fit.shown, fit.shown > 0 {
      order.remove(at: active)
      order.insert(active, at: fit.shown - 1)
    }
    let shown = order.prefix(fit.shown).map { items[$0] }
    let hidden = order.dropFirst(fit.shown).map { items[$0] }

    return HStack(spacing: 0) {
      HStack(spacing: 0) {
        ForEach(shown) { item in
          let selected = item.value == value
          TabsTab(
            item: item,
            selected: selected,
            compact: fit.compact && !selected,
            select: { onValueChange(item.value) },
            close: onClose.map { close in { close(item.value) } }
          )
        }
      }
      .fixedSize(horizontal: true, vertical: false)
      .accessibilityElement(children: .contain)
      .modifier(TabsListLabel(label: accessibilityLabel))

      if !hidden.isEmpty {
        Menu(
          items: hidden.map { item in
            MenuItem(
              item.count.map { "\(item.label) · \($0)" } ?? item.label,
              icon: item.icon,
              disabled: item.disabled,
              onSelect: { onValueChange(item.value) }
            )
          },
          align: .end
        ) { state in
          IconButton(label: "\(hidden.count) more", action: state.toggle) {
            Icon(.moreHorizontal20Regular, size: 14)
          }
        }
        .fixedSize()
      }

      if Trailing.self != EmptyView.self {
        trailing()
          .padding(.leading, vars.spacing1)
          .fixedSize()
          .background(TabsMeasureReport(key: .trailing))
      }
    }
    // What the row would take whole: it is that wide when there is room and
    // gives way, down to the tab that is showing, when there is not.
    .frame(
      minWidth: fit.floor > 0 ? fit.floor : nil,
      idealWidth: fit.basis > 0 ? fit.basis : nil,
      maxWidth: fit.basis > 0 ? fit.basis : nil,
      alignment: .leading
    )
    .frame(minHeight: vars.controlSmallSize)
    .background(TabsMeasureReport(key: .room))
    .clipped()
    // The ruler: every tab whole, then every tab as its icon, out of sight,
    // so what fits is arithmetic on widths that do not depend on what is
    // currently drawn — measuring the visible row would chase itself.
    .background(alignment: .topLeading) { ruler.frame(width: 0, alignment: .leading).clipped() }
    .onPreferenceChange(TabsMeasureKey.self) { reading in
      if reading != measured.widths { measured.widths = reading }
    }
  }

  private var ruler: some View {
    let rulerItems = items.map { item in
      // A dot that pulses out of sight is work for nothing.
      var still = item
      still.busy = false
      return still
    }
    return VStack(alignment: .leading, spacing: 0) {
      HStack(spacing: 0) {
        ForEach(Array(rulerItems.enumerated()), id: \.offset) { index, item in
          // The tab that is showing is measured as it is drawn, in the
          // heavier weight, so the row it is reserved does not clip it.
          TabsTab(
            item: item,
            selected: item.value == value,
            compact: false,
            select: {},
            close: onClose.map { _ in {} }
          )
            .fixedSize()
            .background(TabsMeasureReport(key: .whole(index)))
        }
      }
      HStack(spacing: 0) {
        ForEach(Array(rulerItems.enumerated()), id: \.offset) { index, item in
          TabsTab(item: item, selected: false, compact: true, select: {}, close: onClose.map { _ in {} })
            .fixedSize()
            .background(TabsMeasureReport(key: .compact(index)))
        }
      }
    }
    .fixedSize()
    .opacity(0)
    .allowsHitTesting(false)
    .accessibilityHidden(true)
  }

  /// `tabs.tsx`'s arithmetic, on what was measured.
  private func solve() -> TabsFit {
    let widths = measured.widths
    let whole = items.indices.compactMap { widths[.whole($0)] }
    let small = items.indices.compactMap { widths[.compact($0)] }
    guard whole.count == items.count, small.count == items.count, let room = widths[.room] else {
      return TabsFit(compact: false, shown: items.count, basis: 0, floor: 0)
    }
    let trailingWidth = widths[.trailing] ?? 0
    let basis = whole.reduce(0, +) + trailingWidth
    let space = room - trailingWidth
    let active = items.firstIndex { $0.value == value }
    // The least the row is ever given: the tab that is showing, whole, the
    // way to the rest and what trails.
    let floor = min(
      basis,
      whole[active ?? 0] + (items.count > 1 ? Self.moreWidth : 0) + trailingWidth
    )
    func sum(_ widths: [CGFloat], _ count: Int) -> CGFloat {
      widths.prefix(count).reduce(0, +)
    }
    // Half a point of slack, so a row that fits to the rounding is not
    // folded by it.
    guard sum(whole, items.count) > space + 0.5 else {
      return TabsFit(compact: false, shown: items.count, basis: basis, floor: floor)
    }
    // The tab that is showing keeps its name; the others are their icons.
    let mixed = small.indices.map { $0 == active ? whole[$0] : small[$0] }
    var shown = items.count
    while shown > 1,
          sum(mixed, shown) + (shown < items.count ? Self.moreWidth : 0) > space + 0.5 {
      shown -= 1
    }
    return TabsFit(compact: true, shown: shown, basis: basis, floor: floor)
  }
}

public extension Tabs where Trailing == EmptyView {
  init(
    items: [TabItem<Value>],
    value: Value?,
    onValueChange: @escaping (Value) -> Void,
    onClose: ((Value) -> Void)? = nil,
    accessibilityLabel: String? = nil
  ) {
    self.init(
      items: items,
      value: value,
      onValueChange: onValueChange,
      onClose: onClose,
      accessibilityLabel: accessibilityLabel
    ) { EmptyView() }
  }
}

/// How the row is drawn: whether the tabs not showing are their icons, and
/// how many are in the row rather than behind the more-button.
private struct TabsFit {
  var compact: Bool
  var shown: Int
  /// The row's width whole, and the least it is ever given.
  var basis: CGFloat
  var floor: CGFloat
}

private enum TabsMeasureSlot: Hashable {
  case whole(Int)
  case compact(Int)
  case trailing
  case room
}

private struct TabsMeasure {
  var widths: [TabsMeasureSlot: CGFloat] = [:]
}

private struct TabsMeasureKey: PreferenceKey {
  static let defaultValue: [TabsMeasureSlot: CGFloat] = [:]

  static func reduce(
    value: inout [TabsMeasureSlot: CGFloat],
    nextValue: () -> [TabsMeasureSlot: CGFloat]
  ) {
    value.merge(nextValue()) { _, next in next }
  }
}

/// Reports the width of the view it is the background of.
private struct TabsMeasureReport: View {
  let key: TabsMeasureSlot

  var body: some View {
    GeometryReader { proxy in
      Color.clear.preference(key: TabsMeasureKey.self, value: [key: proxy.size.width])
    }
  }
}

private struct TabsListLabel: ViewModifier {
  let label: String?

  func body(content: Content) -> some View {
    if let label {
      content.accessibilityLabel(label)
    } else {
      content
    }
  }
}

/// One tab: the button, and its close beside it.
private struct TabsTab<Value: Hashable>: View {
  let item: TabItem<Value>
  let selected: Bool
  let compact: Bool
  let select: () -> Void
  let close: (() -> Void)?

  @Environment(\.theme) private var theme
  @State private var hovered = false
  @State private var pulse = false

  var body: some View {
    let vars = theme.vars
    HStack(spacing: 0) {
      SwiftUI.Button(action: select) {
        HStack(spacing: vars.spacing15) {
          if let status = item.status {
            Circle()
              .fill(dot(status, vars))
              .frame(width: vars.spacing15, height: vars.spacing15)
              .opacity(item.busy && pulse ? 0.35 : 1)
              .onAppear {
                guard item.busy else { return }
                withAnimation(.easeInOut(duration: 0.7).repeatForever(autoreverses: true)) { pulse = true }
              }
          }
          if let icon = item.icon {
            Icon(icon, size: 14)
          }
          if !(compact && item.icon != nil) {
            Text(item.label).lineLimit(1).truncationMode(.tail)
            if let detail = item.detail {
              Text(detail).fontWeight(vars.labelMedium.weight).foregroundStyle(vars.colorContentSubtle)
            }
          }
          if let count = item.count {
            Text("\(count)")
              .font(vars.captionSmall.font)
              .fontWeight(vars.labelMedium.weight)
              .monospacedDigit()
              .foregroundStyle(vars.colorContentSubtle)
          }
        }
        .font(vars.labelMedium.font)
        .fontWeight(selected ? vars.labelStrong.weight : vars.labelMedium.weight)
        .foregroundStyle(selected || hovered ? vars.colorContent : vars.colorContentSubtle)
        .padding(.horizontal, vars.spacing25)
        .frame(maxHeight: .infinity)
        .contentShape(Rectangle())
      }
      .buttonStyle(.plain)
      .disabled(item.disabled)
      .opacity(item.disabled ? 0.5 : 1)
      .help(item.title ?? (compact ? item.label : ""))
      // Drawn as its icon alone, the tab is still called by its name.
      .modifier(TabsListLabel(label: compact && item.icon != nil ? item.label : nil))
      .accessibilityAddTraits(selected ? [.isSelected] : [])
      if item.closable, let close, !compact {
        // Offered on the tab that is showing and under the pointer; it keeps
        // its place either way, so a tab is one width whether it shows or not.
        IconButton(label: "Close \(item.label)", action: close) {
          Icon(.dismiss20Regular, size: 12)
        }
        .padding(.leading, -vars.spacing15)
        .padding(.trailing, vars.spacing1)
        // `visibility: hidden`, not only unseen: a close that cannot be seen
        // cannot be pressed either.
        .opacity(closeShown ? 1 : 0)
        .allowsHitTesting(closeShown)
        .accessibilityHidden(!closeShown)
      }
    }
    .frame(maxHeight: .infinity)
    // The rule under the tab that is showing, on the row's foot and inset to the label.
    .overlay(alignment: .bottom) {
      if selected {
        Capsule()
          .fill(vars.resolve(vars.descriptor(.filled, .surface), tint: .primary, state: .normal))
          .frame(height: vars.strokeControl)
          .padding(.horizontal, vars.spacing25)
      }
    }
    .onHover { hovered = $0 }
    .animation(vars.motion, value: hovered)
  }

  private var closeShown: Bool { selected || hovered }

  private func dot(_ status: TabStatus, _ vars: ThemeVariables) -> Color {
    switch status {
    case .neutral: vars.colorBorderStrong
    case .primary: vars.colorPrimary[500]
    case .success: vars.colorSuccess[500]
    case .warning: vars.colorWarning[400]
    case .danger: vars.colorDanger[500]
    }
  }
}
