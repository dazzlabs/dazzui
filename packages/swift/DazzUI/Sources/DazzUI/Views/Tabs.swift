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
  /// What follows the last tab and stays put while they give way: the
  /// control that opens another.
  @ViewBuilder public var trailing: () -> Trailing

  @Environment(\.theme) private var theme

  public init(
    items: [TabItem<Value>],
    value: Value?,
    onValueChange: @escaping (Value) -> Void,
    onClose: ((Value) -> Void)? = nil,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.items = items
    self.value = value
    self.onValueChange = onValueChange
    self.onClose = onClose
    self.trailing = trailing
  }

  public var body: some View {
    let vars = theme.vars
    HStack(spacing: 0) {
      ViewThatFits(in: .horizontal) {
        row(items, compact: false, hidden: [])
        row(items, compact: true, hidden: [])
        // Past that, the tab that is showing stays whole and the rest go
        // behind the more-button.
        row(items.filter { $0.value == value }, compact: true, hidden: items.filter { $0.value != value })
      }
      trailing()
        .padding(.leading, vars.spacing1)
    }
    .frame(minHeight: vars.controlSmallSize)
    .fixedSize(horizontal: false, vertical: false)
  }

  private func row(_ shown: [TabItem<Value>], compact: Bool, hidden: [TabItem<Value>]) -> some View {
    HStack(spacing: 0) {
      ForEach(shown) { item in
        TabsTab(
          item: item,
          selected: item.value == value,
          compact: compact && item.value != value && item.icon != nil,
          select: { onValueChange(item.value) },
          close: onClose.map { close in { close(item.value) } }
        )
      }
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
      }
    }
    .fixedSize(horizontal: true, vertical: false)
  }
}

public extension Tabs where Trailing == EmptyView {
  init(
    items: [TabItem<Value>],
    value: Value?,
    onValueChange: @escaping (Value) -> Void,
    onClose: ((Value) -> Void)? = nil
  ) {
    self.init(items: items, value: value, onValueChange: onValueChange, onClose: onClose) { EmptyView() }
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
          if !compact {
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
      .accessibilityAddTraits(selected ? [.isSelected] : [])
      if item.closable, let close, !compact {
        // Offered on the tab that is showing and under the pointer; it keeps
        // its place either way, so a tab is one width whether it shows or not.
        IconButton(label: "Close \(item.label)", action: close) {
          Icon(.dismiss20Regular, size: 12)
        }
        .padding(.leading, -vars.spacing15)
        .padding(.trailing, vars.spacing1)
        .opacity(selected || hovered ? 1 : 0)
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
