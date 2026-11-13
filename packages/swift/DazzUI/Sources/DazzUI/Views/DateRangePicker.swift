import FluentIcons
import SwiftUI

/// What the picker chooses: when something starts, when it ends, and whether
/// the times matter. An all-day span keeps its times — switching all-day off
/// again brings them back — and a consumer reads `allDay` before them.
public struct DateRangeValue: Equatable, Sendable {
  public var start: Date
  public var end: Date
  public var allDay: Bool

  public init(start: Date, end: Date, allDay: Bool = false) {
    self.start = start
    self.end = end
    self.allDay = allDay
  }
}

/// A named span the reader can pick in one press, counted from today.
public struct DateRangePreset: Sendable {
  public var label: String
  public var range: @Sendable (_ today: Date, _ math: DayMath) -> DateRange

  public init(label: String, range: @escaping @Sendable (Date, DayMath) -> DateRange) {
    self.label = label
    self.range = range
  }
}

/// A length the end is set to, from the start.
public struct DateRangeDuration: Sendable {
  public var label: String
  public var minutes: Int

  public init(label: String, minutes: Int) {
    self.label = label
    self.minutes = minutes
  }
}

/// The strings the sheet is labelled with, for a product in another
/// language, and the way it writes a day.
public struct DateRangePickerStrings: Sendable {
  /// What the trigger says while nothing is chosen.
  public var placeholder: String
  public var presets: String
  public var start: String
  public var end: String
  public var duration: String
  public var allDay: String
  public var cancel: String
  public var done: String
  /// A day in the trigger and the column, e.g. `Fri, Oct 16`.
  public var day: @Sendable (Date) -> String
  public var calendar: CalendarStrings

  public init(
    placeholder: String = "Choose dates",
    presets: String = "Quick pick",
    start: String = "Start",
    end: String = "End",
    duration: String = "Duration",
    allDay: String = "All day",
    cancel: String = "Cancel",
    done: String = "Done",
    day: @escaping @Sendable (Date) -> String = DateRangePickerStrings.day,
    calendar: CalendarStrings = CalendarStrings()
  ) {
    self.placeholder = placeholder
    self.presets = presets
    self.start = start
    self.end = end
    self.duration = duration
    self.allDay = allDay
    self.cancel = cancel
    self.done = done
    self.day = day
    self.calendar = calendar
  }

  private static let dayFormatter: DateFormatter = {
    let formatter = DateFormatter()
    formatter.locale = Locale(identifier: "en_US_POSIX")
    formatter.dateFormat = "EEE, MMM d"
    return formatter
  }()

  @Sendable public static func day(_ date: Date) -> String {
    dayFormatter.string(from: date)
  }
}

private func weekend(_ today: Date, _ math: DayMath) -> Date {
  let until = (7 - math.weekday(today) + 7) % 7
  return math.addDays(today, until == 0 ? 7 : until)
}

private func nextMonday(_ today: Date, _ math: DayMath) -> Date {
  let until = (2 - math.weekday(today) + 7) % 7
  return math.addDays(today, until == 0 ? 7 : until)
}

/// The spans a calendar reaches for most.
public let dateRangePresets: [DateRangePreset] = [
  DateRangePreset(label: "Today") { t, _ in DateRange(start: t, end: t) },
  DateRangePreset(label: "Tomorrow") { t, m in
    DateRange(start: m.addDays(t, 1), end: m.addDays(t, 1))
  },
  DateRangePreset(label: "This weekend") { t, m in
    DateRange(start: weekend(t, m), end: m.addDays(weekend(t, m), 1))
  },
  DateRangePreset(label: "Next Monday") { t, m in
    DateRange(start: nextMonday(t, m), end: nextMonday(t, m))
  },
  DateRangePreset(label: "Next week") { t, m in
    DateRange(start: nextMonday(t, m), end: m.addDays(nextMonday(t, m), 6))
  },
  DateRangePreset(label: "This month") { t, m in
    DateRange(start: m.startOfMonth(t), end: m.addDays(m.addMonths(t, 1), -1))
  },
]

public let dateRangeDurations: [DateRangeDuration] = [
  DateRangeDuration(label: "30 min", minutes: 30),
  DateRangeDuration(label: "1 hour", minutes: 60),
  DateRangeDuration(label: "2 hours", minutes: 120),
]

/// The sheet's own rules, apart from the view so they can be read and tested
/// as data: moving the start carries the end with it, an end set before its
/// start pulls the start back an hour ahead of it, and the days the calendar
/// holds stay apart from the draft while a range is mid-pick.
struct DateRangeDraft: Equatable {
  var value: DateRangeValue
  /// The days as the calendar holds them: between the first press and the
  /// second the calendar's end is unset, and the draft reads that as a
  /// one-day span until the reader finishes.
  var days: DateRange
  let math: DayMath

  static let hour: TimeInterval = 60 * 60

  init(_ value: DateRangeValue, math: DayMath) {
    self.value = value
    self.days = DateRange(start: value.start, end: value.end)
    self.math = math
  }

  static func == (a: DateRangeDraft, b: DateRangeDraft) -> Bool {
    a.value == b.value && a.days == b.days
  }

  mutating func setDays(_ range: DateRange) {
    days = range
    let start = range.start ?? value.start
    let end = range.end ?? start
    let nextStart = math.withTime(start, value.start)
    var nextEnd = math.withTime(end, value.end)
    if nextEnd < nextStart { nextEnd = nextStart.addingTimeInterval(Self.hour) }
    value.start = nextStart
    value.end = nextEnd
  }

  mutating func setStartTime(hour: Int, minute: Int) {
    let length = value.end.timeIntervalSince(value.start)
    let start = math.setTime(value.start, hour: hour, minute: minute)
    value.start = start
    value.end = start.addingTimeInterval(length)
  }

  mutating func setEndTime(hour: Int, minute: Int) {
    let end = math.setTime(value.end, hour: hour, minute: minute)
    if end < value.start { value.start = end.addingTimeInterval(-Self.hour) }
    value.end = end
  }

  mutating func setDuration(minutes: Int) {
    value.allDay = false
    value.end = value.start.addingTimeInterval(TimeInterval(minutes * 60))
  }

  var oneDay: Bool { math.compareDay(value.start, value.end) == .orderedSame }

  func matches(_ preset: DateRangePreset, today: Date) -> Bool {
    let range = preset.range(today, math)
    return math.sameDay(range.start, value.start) && math.sameDay(range.end, value.end)
  }

  func matches(_ duration: DateRangeDuration) -> Bool {
    !value.allDay && value.end.timeIntervalSince(value.start) == TimeInterval(duration.minutes * 60)
  }
}

private func pad(_ n: Int) -> String { n < 10 ? "0\(n)" : "\(n)" }

private func clock(_ date: Date, _ math: DayMath) -> String {
  let parts = math.calendar.dateComponents([.hour, .minute], from: date)
  return "\(pad(parts.hour ?? 0)):\(pad(parts.minute ?? 0))"
}

/// A span of days and times, chosen in one sheet.
///
/// The trigger is a field — it wears the select's box — and what it opens is
/// a sheet: a month to pick the span on, two for a product whose spans mostly
/// cross one, and beside it a column that opens with a menu of the spans a
/// calendar reaches for most, then the start and end with their times and a
/// row of durations. Nothing in the sheet reaches the value until Done; an
/// outside press or Escape puts it back. That is the one way this differs
/// from every other popup here, and it is deliberate: a span is several
/// decisions, and a reader who has made two of them should not lose both to
/// a stray click.
///
/// The sheet is composition rather than behaviour: the kit's own
/// ``Calendar``, ``Select``, ``Menu`` and ``Toggle`` own the parts, and the
/// floating panel owns the outside press and the Escape. The rules of its own
/// are ``DateRangeDraft``'s.
public struct DateRangePicker: View {
  @Binding public var value: DateRangeValue?
  /// The quick-pick menu. ``dateRangePresets`` unless the product has its own.
  public var presets: [DateRangePreset]
  /// The duration chips. ``dateRangeDurations`` unless the product has its own.
  public var durations: [DateRangeDuration]
  /// Minutes between the times the lists offer.
  public var timeStep: Int
  /// Months side by side in the sheet: one, or two.
  public var months: Int
  /// Foundation's weekday: 1 is Sunday. Monday by default.
  public var weekStartsOn: Int
  public var strings: DateRangePickerStrings
  /// What the grid counts as today.
  public var today: Date?
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @FocusState private var focused: Bool
  @State private var open = false
  @State private var draft: DateRangeDraft?
  @State private var month = Date()

  public init(
    value: Binding<DateRangeValue?>,
    presets: [DateRangePreset] = dateRangePresets,
    durations: [DateRangeDuration] = dateRangeDurations,
    timeStep: Int = 15,
    months: Int = 1,
    weekStartsOn: Int = 2,
    strings: DateRangePickerStrings = DateRangePickerStrings(),
    today: Date? = nil,
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    enabled: Bool = true
  ) {
    self._value = value
    self.presets = presets
    self.durations = durations
    self.timeStep = timeStep
    self.months = months
    self.weekStartsOn = weekStartsOn
    self.strings = strings
    self.today = today
    self.size = size
    self.state = state
    self.tint = tint
    self.enabled = enabled
  }

  private var math: DayMath { DayMath(weekStartsOn: weekStartsOn) }
  private var todayDay: Date { math.startOfDay(today ?? Date()) }

  /// The next whole hour, for an hour — what a new event defaults to.
  private func blank() -> DateRangeValue {
    let now = Date()
    let day = todayDay
    let hour =
      math.sameDay(now, day)
      ? Swift.min(23, math.calendar.component(.hour, from: now) + 1) : 9
    let start = math.setTime(day, hour: hour, minute: 0)
    return DateRangeValue(start: start, end: start.addingTimeInterval(DateRangeDraft.hour))
  }

  private func toggle() {
    if open {
      open = false
    } else {
      let initial = value ?? blank()
      draft = DateRangeDraft(initial, math: math)
      month = math.startOfMonth(initial.start)
      open = true
    }
  }

  private func commit() {
    if let draft { value = draft.value }
    open = false
  }

  public var body: some View {
    let vars = theme.vars

    trigger
      .floatingPanel(isPresented: $open, gap: vars.spacing1, modal: false) {
        sheet
      }
      .onChange(of: enabled) { if !$0 { open = false } }
  }

  /// The trigger: the select's box, with the calendar glyph at its leading
  /// edge and the span where the value goes.
  private var trigger: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)

    return Pressable(action: toggle, enabled: enabled, cornerRadius: vars.controlFieldRadius) {
      _ in
      HStack(spacing: vars.spacing2) {
        // `CalendarLtr16Regular` on the web; the Swift icon package carries
        // the glyph only as `Calendar16Regular`.
        Icon(.calendar16Regular, size: vars.iconSmall)
          .foregroundStyle(enabled ? vars.colorContentSubtle : vars.colorContentFaint)
        Group {
          if let value {
            Summary(value: value, strings: strings, math: math)
          } else {
            Text(strings.placeholder).foregroundStyle(vars.colorContentFaint)
          }
        }
        .lineLimit(1)
        .truncationMode(.tail)
        .frame(maxWidth: .infinity, alignment: .leading)
        Icon(.chevronDown12Regular, size: vars.iconSmall)
          .foregroundStyle(enabled ? vars.colorContentSubtle : vars.colorContentFaint)
      }
      .font(FieldStyle.font(size: metrics.content.size, mono: false, vars: vars))
      .monospacedDigit()
      .padding(.horizontal, vars.spacing3)
      .frame(minWidth: 256)
      .frame(height: metrics.size)
      .fieldChrome(
        FieldStyle(
          vars: vars,
          tint: tintFor(state, tint),
          invalid: state == .error,
          enabled: enabled,
          focused: focused || open
        ),
        cornerRadius: vars.controlFieldRadius,
        vars: vars
      )
    }
    .focused($focused)
  }

  /// The sheet: the shared popup holding the months, the column beside them
  /// and a footer, hairlined off each other.
  @ViewBuilder private var sheet: some View {
    let vars = theme.vars

    VStack(spacing: 0) {
      HStack(alignment: .top, spacing: 0) {
        DazzUI.Calendar(
          range: Binding(
            get: { draft?.days ?? DateRange() },
            set: { draft?.setDays($0) }
          ),
          size: .small,
          months: months,
          month: $month,
          weekStartsOn: weekStartsOn,
          strings: strings.calendar,
          today: todayDay,
          tint: tint
        )
        .padding(vars.spacing2)
        side
          .padding(.top, vars.spacing2)
          .padding(.horizontal, vars.spacing3)
          .padding(.bottom, vars.spacing3)
          .frame(width: vars.dateRangePickerSideWidth, alignment: .topLeading)
          .frame(maxHeight: .infinity, alignment: .top)
          .rule(vars.colorBorder, edge: .leading, width: hairline)
      }
      .fixedSize(horizontal: false, vertical: true)
      footer
        .padding(.leading, vars.spacing3)
        .padding(.trailing, vars.spacing2)
        .padding(.vertical, vars.spacing2)
        .rule(vars.colorBorder, edge: .top, width: hairline)
    }
    .font(vars.bodySmall.font)
    .foregroundStyle(vars.colorContent)
    .background(vars.colorSurfaceOverlay)
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusLarge, style: .circular))
    .border(vars.colorBorderStrong, width: hairline, cornerRadius: vars.radiusLarge)
    .tokenShadow(vars.shadowLg)
    .fixedSize()
  }

  /// The column: the quick-pick menu's name on the first line, then a key
  /// over a date and its time, twice, then the durations.
  @ViewBuilder private var side: some View {
    let vars = theme.vars
    let current = draft?.value ?? blank()

    VStack(alignment: .leading, spacing: vars.spacing3) {
      // A plain button pads its label by the profile's inline step; pulling
      // it back by that sets the name on the column's own edge.
      DazzUI.Menu(
        items: presets.map { preset in
          MenuItem(
            preset.label,
            checked: draft?.matches(preset, today: todayDay) ?? false
          ) {
            let range = preset.range(todayDay, math)
            month = math.startOfMonth(range.start ?? todayDay)
            draft?.setDays(range)
          }
        },
        align: .start,
        tint: tint
      ) { state in
        DazzUI.Button(
          strings.presets,
          size: .tiny,
          variant: .plain,
          tint: .neutral,
          trailing: Icon(.chevronDown12Regular),
          action: state.toggle
        )
      }
      .padding(.leading, -(vars.controlTinyPaddingInline + vars.spacingPx))

      end(
        label: strings.start,
        at: current.start,
        allDay: current.allDay
      ) { draft?.setStartTime(hour: $0, minute: $1) }
      end(
        label: strings.end,
        at: current.end,
        allDay: current.allDay
      ) { draft?.setEndTime(hour: $0, minute: $1) }

      VStack(alignment: .leading, spacing: vars.spacing15) {
        key(strings.duration)
        // The chips wrap to a second line where the column is too narrow
        // for four.
        FlowRow(spacing: vars.spacing1) {
          ForEach(durations, id: \.minutes) { duration in
            DazzUI.Toggle(
              duration.label,
              pressed: draft?.matches(duration) ?? false,
              variant: .outlined,
              tint: tint,
              // A duration is a length within a day; a span across days
              // has none.
              enabled: draft?.oneDay ?? true
            ) { draft?.setDuration(minutes: duration.minutes) }
          }
          DazzUI.Toggle(
            strings.allDay,
            pressed: current.allDay,
            variant: .outlined,
            tint: tint
          ) { draft?.value.allDay.toggle() }
        }
      }
    }
  }

  private func key(_ text: String) -> some View {
    let vars = theme.vars
    return Text(text.uppercased())
      .font(.system(size: vars.captionSmall.size, weight: vars.labelSmall.weight))
      .tracking(0.06 * vars.captionSmall.size)
      .foregroundStyle(vars.colorContentSubtle)
      .frame(height: vars.captionSmall.size)
  }

  private func end(
    label: String, at date: Date, allDay: Bool, onTime: @escaping (Int, Int) -> Void
  ) -> some View {
    let vars = theme.vars
    let current = clock(date, math)
    var options = (stride(from: 0, to: 24 * 60, by: timeStep)).map { m -> SelectOption<String> in
      let label = "\(pad(m / 60)):\(pad(m % 60))"
      return SelectOption(value: label, label: label)
    }
    // A time list that starts on the step but still shows a value off it —
    // a span handed in at 14:07 keeps 14:07 until the reader moves it.
    if !options.contains(where: { $0.value == current }) {
      options.insert(SelectOption(value: current, label: current), at: 0)
    }

    return VStack(alignment: .leading, spacing: vars.spacing1) {
      key(label)
      HStack(spacing: vars.spacing2) {
        Text(strings.day(date))
          .font(.system(size: vars.labelQuiet.size, weight: vars.labelQuiet.weight))
          .monospacedDigit()
          .lineLimit(1)
          .frame(maxWidth: .infinity, alignment: .leading)
        Select(
          options: options,
          selection: Binding(
            get: { current },
            set: { time in
              let parts = time.split(separator: ":").compactMap { Int($0) }
              if parts.count == 2 { onTime(parts[0], parts[1]) }
            }
          ),
          size: .small,
          tint: tint,
          enabled: !allDay,
          width: .value
        )
      }
    }
  }

  @ViewBuilder private var footer: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing3) {
      Group {
        if let draft {
          Summary(value: draft.value, strings: strings, math: math)
        }
      }
      .font(.system(size: vars.labelSmall.size, weight: vars.labelSmall.weight))
      .monospacedDigit()
      .foregroundStyle(vars.colorContentSecondary)
      .lineLimit(1)
      .truncationMode(.tail)
      .frame(maxWidth: .infinity, alignment: .leading)
      DazzUI.Button(strings.cancel, variant: .normal) { open = false }
      DazzUI.Button(strings.done, variant: .filled, tint: tint, action: commit)
    }
  }
}

/// A span in one line: the day once when both ends share it, the times
/// unless it is all day, and the joins in the faint ink so the dates and
/// times read as the values and the arrow as punctuation.
private struct Summary: View {
  let value: DateRangeValue
  let strings: DateRangePickerStrings
  let math: DayMath

  @Environment(\.theme) private var theme

  var body: some View {
    let faint = theme.vars.colorContentFaint
    let oneDay = math.compareDay(value.start, value.end) == .orderedSame
    let startDay = Text(strings.day(value.start))
    let endDay = Text(strings.day(value.end))
    let sep = { (glyph: String) in Text(" \(glyph) ").foregroundColor(faint) }

    if value.allDay {
      if oneDay {
        startDay + sep("·") + Text(strings.allDay)
      } else {
        startDay + sep("→") + endDay
      }
    } else if oneDay {
      startDay + sep("·") + Text(clock(value.start, math)) + sep("→")
        + Text(clock(value.end, math))
    } else {
      startDay + Text(" \(clock(value.start, math))") + sep("→") + endDay
        + Text(" \(clock(value.end, math))")
    }
  }
}

/// A run of chips that wraps — CSS's `flex-wrap`, which the stacks lack.
struct FlowRow: Layout {
  var spacing: CGFloat

  func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
    arrange(proposal: proposal, subviews: subviews).size
  }

  func placeSubviews(
    in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()
  ) {
    let placed = arrange(proposal: proposal, subviews: subviews)
    for (index, origin) in placed.origins.enumerated() {
      subviews[index].place(
        at: CGPoint(x: bounds.minX + origin.x, y: bounds.minY + origin.y),
        proposal: .unspecified
      )
    }
  }

  private func arrange(proposal: ProposedViewSize, subviews: Subviews)
    -> (size: CGSize, origins: [CGPoint])
  {
    let width = proposal.width ?? .infinity
    var origins: [CGPoint] = []
    var x: CGFloat = 0
    var y: CGFloat = 0
    var rowHeight: CGFloat = 0
    var widest: CGFloat = 0
    for subview in subviews {
      let size = subview.sizeThatFits(.unspecified)
      if x > 0, x + size.width > width {
        x = 0
        y += rowHeight + spacing
        rowHeight = 0
      }
      origins.append(CGPoint(x: x, y: y))
      x += size.width + spacing
      rowHeight = Swift.max(rowHeight, size.height)
      widest = Swift.max(widest, x - spacing)
    }
    return (CGSize(width: widest, height: y + rowHeight), origins)
  }
}
