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
  @Environment(\.font) private var font
  @Environment(\.locale) private var locale
  @Environment(\.displayScale) private var displayScale
  @Environment(\.layoutDirection) private var layoutDirection
  #if !os(Windows) && !os(Linux) && !os(WASI) && canImport(Darwin)
    @Environment(\.dynamicTypeSize) private var dynamicTypeSize
    @Environment(\.legibilityWeight) private var legibilityWeight
    @Environment(\.textCase) private var textCase
  #endif
  /// What the ruler and the row measured: each tab whole and as its icon,
  /// what trails, and the room the row was given.
  @State private var measured = TabsMeasure()

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
    let active = items.firstIndex { $0.value == value }
    let ruler = TabsRulerPlan(items: items, active: active, canClose: onClose != nil)
    var signature = TabsRulerSignature(
      probes: ruler.probes, label: vars.labelMedium, caption: vars.captionSmall,
      selectedWeight: vars.labelStrong.weight, padding: vars.spacing25, gap: vars.spacing15,
      closePadding: vars.spacing1, controlWidth: vars.metrics(.tiny).size,
      font: font, locale: locale, displayScale: displayScale, direction: layoutDirection)
    #if !os(Windows) && !os(Linux) && !os(WASI) && canImport(Darwin)
      signature.dynamicTypeSize = dynamicTypeSize
      signature.legibilityWeight = legibilityWeight
      signature.textCase = textCase
    #endif
    let hasWidths = measured.signature == signature
    // Short tab bars can keep the ruler mounted. Long document lists show
    // the selection while the first measurement settles.
    let large = items.count > 32
    let fit = hasWidths
      ? TabsFit.resolve(ruler.slots, active: active, widths: measured.widths,
                        moreWidth: vars.metrics(.tiny).size)
      : TabsFit(compact: large, shown: large ? 1 : items.count, basis: 0, floor: 0)
    let probes = large && hasWidths ? ruler.liveProbes : ruler.probes

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
    // Measure the offered width inside the intrinsic cap. A row of discrete
    // buttons usually leaves some spare space; using its drawn width would
    // feed that smaller value back into the next fit.
    .frame(maxWidth: .infinity, alignment: .leading)
    .background {
      GeometryReader { proxy in
        let width = proxy.size.width
        // Keep the reported width stable while the same tabs fit. A new
        // selection, ruler or trailing control checks the actual width again.
        let reported = hasWidths && TabsFit.resolve(
          ruler.slots, active: active, widths: measured.widths,
          moreWidth: vars.metrics(.tiny).size, room: width) == fit
          ? measured.widths[.room] ?? width : width
        Color.clear.preference(key: TabsMeasureKey.self, value: [.room: reported])
      }
    }
    // What the row would take whole: it is that wide when there is room and
    // gives way, down to the tab that is showing, when there is not.
    .frame(
      minWidth: fit.floor > 0 ? fit.floor : nil,
      idealWidth: fit.basis > 0 ? fit.basis : nil,
      maxWidth: fit.basis > 0 ? fit.basis : (large ? .infinity : nil),
      alignment: .leading
    )
    .frame(minHeight: vars.controlSmallSize, alignment: .leading)
    .clipped()
    // Intrinsic widths come from real labels laid out independently of the
    // visible row. Equal label geometry needs only one measurement.
    .background(alignment: .topLeading) {
      measure(probes).frame(width: 0, alignment: .leading).clipped()
    }
    .onPreferenceChange(TabsMeasureKey.self) { reading in
      let updated = measured.applying(reading, signature: signature, probes: probes)
      if updated != measured { measured = updated }
    }
  }

  private func measure(_ probes: [TabsRulerPlan.Probe]) -> some View {
    HStack(spacing: 0) {
      ForEach(probes) { probe in
        TabsTab(
          item: items[probe.index],
          selected: probe.selected,
          compact: probe.compact,
          select: {},
          close: onClose.map { _ in {} },
          isRuler: true
        )
        .fixedSize()
        .background(TabsMeasureReport(key: probe.id))
      }
    }
    .fixedSize()
    .opacity(0)
    .allowsHitTesting(false)
    .accessibilityHidden(true)
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
struct TabsFit: Equatable {
  var compact: Bool
  var shown: Int
  /// The row's width whole, and the least it is ever given.
  var basis: CGFloat
  var floor: CGFloat

  static func resolve(_ slots: [TabsRulerPlan.Slots], active: Int?, widths: [TabsMeasureSlot: CGFloat],
                      moreWidth: CGFloat, room: CGFloat? = nil) -> Self {
    let count = slots.count
    let unmeasured = Self(compact: false, shown: count, basis: 0, floor: 0)
    guard count > 0, let room = room ?? widths[.room],
          let activeWidth = widths[slots[active ?? 0].whole] else { return unmeasured }

    let trailing = widths[.trailing] ?? 0
    // Half a point of slack for subpixel measurements at the fit boundary.
    let available = room - trailing + 0.5
    let activeIndex = active ?? -1
    var wholeWidth: CGFloat = 0
    var prefix: CGFloat = 0
    var shown = 1
    for (index, pair) in slots.enumerated() {
      guard let whole = widths[pair.whole], let small = widths[pair.compact] else {
        return unmeasured
      }
      wholeWidth += whole
      let width = index == activeIndex ? whole : small
      // If the active tab would fall behind the menu it takes the last
      // visible place, at its full width. Each candidate reuses the prefix.
      let last = activeIndex > index ? activeWidth : width
      let more = index + 1 < count ? moreWidth : 0
      if prefix + last + more <= available { shown = index + 1 }
      prefix += width
    }
    let basis = wholeWidth + trailing
    let floor = min(basis, activeWidth + (count > 1 ? moreWidth : 0) + trailing)
    let compact = wholeWidth > available
    return Self(compact: compact, shown: compact ? shown : count, basis: basis, floor: floor)
  }
}

enum TabsMeasureSlot: Hashable {
  case whole(Int)
  case compact(Int)
  case trailing
  case room
}

/// Only inputs which affect the intrinsic width participate. A compact icon
/// has a fixed footprint; its label, detail and glyph identity don't affect it.
struct TabsRulerPlan {
  struct Slots {
    let whole: TabsMeasureSlot
    let compact: TabsMeasureSlot
  }

  struct Probe: Identifiable, Equatable {
    let id: TabsMeasureSlot
    let index: Int
    let selected: Bool
    let compact: Bool
    fileprivate let shape: Shape
  }

  fileprivate struct Shape: Hashable {
    let label: String?
    let detail: String?
    let count: Int?
    let icon: Bool
    let dot: Bool
    let strong: Bool
    let close: Bool
  }

  let slots: [Slots]
  let probes: [Probe]
  // Keep a real regular, selected and compact label participating in layout.
  // This also detects inherited typesetting changes outside the theme.
  let liveProbes: [Probe]

  init<Value>(items: [TabItem<Value>], active: Int?, canClose: Bool) {
    var measured: [Shape: TabsMeasureSlot] = [:]
    var probes: [Probe] = []
    var slots: [Slots] = []
    slots.reserveCapacity(items.count)
    func probe(_ index: Int, compact: Bool) -> TabsMeasureSlot {
      let item = items[index]
      let showsText = !compact || item.icon == nil
      let selected = !compact && index == active
      let shape = Shape(label: showsText ? item.label : nil, detail: showsText ? item.detail : nil,
                        count: item.count, icon: item.icon != nil, dot: item.status != nil,
                        strong: selected && showsText, close: !compact && canClose && item.closable)
      if let previous = measured[shape] { return previous }
      let id: TabsMeasureSlot = compact ? .compact(index) : .whole(index)
      measured[shape] = id
      probes.append(Probe(id: id, index: index, selected: selected, compact: compact, shape: shape))
      return id
    }
    for index in items.indices {
      slots.append(Slots(whole: probe(index, compact: false), compact: probe(index, compact: true)))
    }
    self.slots = slots
    self.probes = probes
    self.liveProbes = [
      probes.first { !$0.compact && !$0.selected },
      probes.first { $0.selected },
      probes.first { $0.compact },
      probes.first { $0.shape.count != nil },
    ].compactMap { $0 }.reduce(into: []) { result, probe in
      if !result.contains(where: { $0.id == probe.id }) { result.append(probe) }
    }
  }
}

private struct TabsRulerSignature: Equatable {
  let probes: [TabsRulerPlan.Probe]
  let label: TypeStyle
  let caption: TypeStyle
  let selectedWeight: Font.Weight
  let padding: CGFloat
  let gap: CGFloat
  let closePadding: CGFloat
  let controlWidth: CGFloat
  let font: Font?
  let locale: Locale
  let displayScale: CGFloat
  let direction: LayoutDirection
  #if !os(Windows) && !os(Linux) && !os(WASI) && canImport(Darwin)
    var dynamicTypeSize: DynamicTypeSize = .large
    var legibilityWeight: LegibilityWeight?
    var textCase: Text.Case?
  #endif
}

private struct TabsMeasure: Equatable {
  var signature: TabsRulerSignature?
  var widths: [TabsMeasureSlot: CGFloat] = [:]

  func applying(_ reading: [TabsMeasureSlot: CGFloat], signature: TabsRulerSignature,
                probes: [TabsRulerPlan.Probe]) -> Self {
    var next = self
    next.widths[.room] = reading[.room]
    next.widths[.trailing] = reading[.trailing]
    guard probes.allSatisfy({ reading[$0.id]?.isFinite == true }) else { return next }
    if probes.count == signature.probes.count {
      next.signature = signature
      next.widths = reading
    } else if probes.contains(where: { reading[$0.id] != widths[$0.id] }) {
      // The text system changed the width of a live label. Reattach the full
      // ruler before using the other saved widths.
      next.signature = nil
    }
    return next
  }
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

private let tabGlyphSize: CGFloat = 14

/// One tab: the button, and its close beside it.
private struct TabsTab<Value: Hashable>: View {
  let item: TabItem<Value>
  let selected: Bool
  let compact: Bool
  let select: () -> Void
  let close: (() -> Void)?
  var isRuler: Bool = false

  @Environment(\.theme) private var theme
  @State private var hovered = false
  @State private var pulse = false

  var body: some View {
    let vars = theme.vars
    let labelStack = HStack(spacing: vars.spacing15) {
      if let status = item.status {
        Circle()
          .fill(dot(status, vars))
          .frame(width: vars.spacing15, height: vars.spacing15)
          .opacity(item.busy && pulse ? 0.35 : 1)
          .onAppear {
            guard !isRuler, item.busy else { return }
            withAnimation(.easeInOut(duration: 0.7).repeatForever(autoreverses: true)) { pulse = true }
          }
      }
      if let icon = item.icon {
        if isRuler {
          Color.clear.frame(width: tabGlyphSize, height: tabGlyphSize)
        } else {
          Icon(icon, size: tabGlyphSize)
        }
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

    if isRuler {
      HStack(spacing: 0) {
        labelStack
        if item.closable, close != nil, !compact {
          let closeSize = vars.metrics(.tiny).size
          Color.clear
            .frame(width: closeSize, height: closeSize)
            .padding(.leading, -vars.spacing15)
            .padding(.trailing, vars.spacing1)
        }
      }
      .frame(maxHeight: .infinity)
    } else {
      HStack(spacing: 0) {
        SwiftUI.Button(action: select) {
          labelStack
            .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
        .disabled(item.disabled)
        .opacity(item.disabled ? 0.5 : 1)
        .modifier(OptionalHelpModifier(help: item.title ?? (compact ? item.label : nil)))
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
            .fill(vars.controlColor(.filled, .surface, tint: .primary, state: .normal))
            .frame(height: vars.strokeControl)
            .padding(.horizontal, vars.spacing25)
        }
      }
      .onHover { hovered = $0 }
      .animation(vars.motion, value: hovered)
    }
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
