import FluentIcons
import SwiftUI

/// A span of days. `end` is nil while the first day has been picked and the
/// second has not — a range mid-pick is still a value, because the picker
/// around the calendar wants to show the start while the reader chooses the
/// end.
public struct DateRange: Equatable, Sendable {
  public var start: Date?
  public var end: Date?

  public init(start: Date? = nil, end: Date? = nil) {
    self.start = start
    self.end = end
  }
}

/// The calendar arithmetic a month grid and a range picker need, on local
/// time. A day is a `Date` at local midnight; a time of day rides on the same
/// value. The kit draws a calendar, it does not do calendars, so this is the
/// Gregorian one with the week starting where the host says.
public struct DayMath: Sendable {
  public let calendar: Foundation.Calendar

  /// `weekStartsOn` is Foundation's weekday: 1 is Sunday, 2 Monday.
  public init(weekStartsOn: Int = 2) {
    var calendar = Foundation.Calendar(identifier: .gregorian)
    calendar.firstWeekday = weekStartsOn
    self.calendar = calendar
  }

  public func startOfDay(_ date: Date) -> Date { calendar.startOfDay(for: date) }

  public func startOfMonth(_ date: Date) -> Date {
    calendar.date(from: calendar.dateComponents([.year, .month], from: date)) ?? date
  }

  public func addDays(_ date: Date, _ days: Int) -> Date {
    calendar.date(byAdding: .day, value: days, to: date) ?? date
  }

  /// The first of the month `months` away, so the day never overflows.
  public func addMonths(_ date: Date, _ months: Int) -> Date {
    calendar.date(byAdding: .month, value: months, to: startOfMonth(date)) ?? date
  }

  public func sameDay(_ a: Date?, _ b: Date?) -> Bool {
    guard let a, let b else { return false }
    return calendar.isDate(a, inSameDayAs: b)
  }

  public func sameMonth(_ a: Date, _ b: Date) -> Bool {
    calendar.isDate(a, equalTo: b, toGranularity: .month)
  }

  /// Day order, ignoring the time of day.
  public func compareDay(_ a: Date, _ b: Date) -> ComparisonResult {
    startOfDay(a).compare(startOfDay(b))
  }

  /// `day`'s date carrying `time`'s hours and minutes.
  public func withTime(_ day: Date, _ time: Date) -> Date {
    let clock = calendar.dateComponents([.hour, .minute], from: time)
    return calendar.date(
      bySettingHour: clock.hour ?? 0, minute: clock.minute ?? 0, second: 0, of: day
    ) ?? day
  }

  public func setTime(_ date: Date, hour: Int, minute: Int) -> Date {
    calendar.date(bySettingHour: hour, minute: minute, second: 0, of: date) ?? date
  }

  public func day(_ date: Date) -> Int { calendar.component(.day, from: date) }

  /// Foundation's weekday, 1 is Sunday.
  public func weekday(_ date: Date) -> Int { calendar.component(.weekday, from: date) }

  /// The six weeks a month grid draws — always forty-two days, so the grid
  /// never changes height between months.
  public func monthGrid(_ month: Date) -> [Date] {
    let first = startOfMonth(month)
    let offset = (weekday(first) - calendar.firstWeekday + 7) % 7
    let start = addDays(first, -offset)
    return (0..<42).map { addDays(start, $0) }
  }
}

/// The words a calendar is labelled with. English unless the product says
/// otherwise; the kit carries no locale of its own, and a product hands it
/// formatters for the one it speaks.
public struct CalendarStrings: Sendable {
  /// The month's name over its grid, e.g. `October 2026`.
  public var monthTitle: @Sendable (Date) -> String
  /// The narrow name over a column, from Foundation's weekday (1 is Sunday).
  public var weekday: @Sendable (Int) -> String
  /// A day's accessible name, e.g. `Friday, October 16, 2026`.
  public var dayLabel: @Sendable (Date) -> String
  public var previousMonth: String
  public var nextMonth: String

  public init(
    monthTitle: @escaping @Sendable (Date) -> String = CalendarStrings.monthTitle,
    weekday: @escaping @Sendable (Int) -> String = CalendarStrings.weekday,
    dayLabel: @escaping @Sendable (Date) -> String = CalendarStrings.dayLabel,
    previousMonth: String = "Previous month",
    nextMonth: String = "Next month"
  ) {
    self.monthTitle = monthTitle
    self.weekday = weekday
    self.dayLabel = dayLabel
    self.previousMonth = previousMonth
    self.nextMonth = nextMonth
  }

  private static let english = Locale(identifier: "en_US_POSIX")

  @Sendable public static func monthTitle(_ date: Date) -> String {
    let formatter = DateFormatter()
    formatter.locale = english
    formatter.dateFormat = "MMMM yyyy"
    return formatter.string(from: date)
  }

  @Sendable public static func weekday(_ weekday: Int) -> String {
    ["S", "M", "T", "W", "T", "F", "S"][(weekday - 1 + 7) % 7]
  }

  @Sendable public static func dayLabel(_ date: Date) -> String {
    let formatter = DateFormatter()
    formatter.locale = english
    formatter.dateFormat = "EEEE, MMMM d, yyyy"
    return formatter.string(from: date)
  }
}

/// The regular control height per day, or the small step for a panel that
/// holds two months.
public enum CalendarSize: String, CaseIterable, Sendable {
  case small
  case medium
}

/// A month grid, in one of two modes: one day, or a span of them.
///
/// SwiftUI has no calendar this kit can wear — `DatePicker`'s graphical
/// style is the platform's own drawing — so this is one of the few views that
/// writes its behaviour out, and the reason is written here: a grid's
/// keyboard is not a list's. The grid is one focus stop; the arrow keys move
/// by a day and by a week, Page Up and Page Down by a month, Home and End to
/// the ends of the week, and Return or Space picks. In range mode the first
/// press sets the start and the second the end, in either order, and while
/// the end is pending the day under the pointer or the keyboard previews the
/// span.
///
/// Every day is a control, so it wears the shared recipes: `tinted` inside a
/// range — the system's one drawing for a selection — and `filled` for the
/// day or the two days actually picked. At rest it is drawn as a row, in ink
/// with the neutral hover wash, the way a menu row is.
///
/// Named `Calendar` as the React component is, and reached as
/// `DazzUI.Calendar` where `Foundation.Calendar` is also in scope.
public struct Calendar: View {
  public enum Mode: Sendable { case single, range }

  public var mode: Mode
  @Binding public var selection: Date?
  @Binding public var range: DateRange
  public var size: CalendarSize
  /// How many months stand side by side: one or two.
  public var months: Int
  /// The month shown — the first of two. Left nil the view holds its own.
  public var month: Binding<Date>?
  /// Days before `min` and after `max` are drawn but cannot be picked.
  public var min: Date?
  public var max: Date?
  /// Foundation's weekday: 1 is Sunday. Monday by default, as ISO counts the
  /// week.
  public var weekStartsOn: Int
  public var strings: CalendarStrings
  /// What the grid counts as today. The clock's, unless a test says
  /// otherwise.
  public var today: Date?
  /// Whether the head carries the chevrons that move the month. Off for a
  /// calendar whose host moves it, which then shows the month's name alone.
  public var navigation: Bool
  public var tint: Tint

  @Environment(\.theme) private var theme
  @State private var ownMonth: Date?
  @State private var focused: Date?
  @State private var hovered: Date?
  @FocusState private var gridFocused: Bool

  /// One day.
  public init(
    selection: Binding<Date?>,
    size: CalendarSize = .medium,
    months: Int = 1,
    month: Binding<Date>? = nil,
    min: Date? = nil,
    max: Date? = nil,
    weekStartsOn: Int = 2,
    strings: CalendarStrings = CalendarStrings(),
    today: Date? = nil,
    navigation: Bool = true,
    tint: Tint = .primary
  ) {
    self.mode = .single
    self._selection = selection
    self._range = .constant(DateRange())
    self.size = size
    self.months = months
    self.month = month
    self.min = min
    self.max = max
    self.weekStartsOn = weekStartsOn
    self.strings = strings
    self.today = today
    self.navigation = navigation
    self.tint = tint
  }

  /// A span of days.
  public init(
    range: Binding<DateRange>,
    size: CalendarSize = .medium,
    months: Int = 1,
    month: Binding<Date>? = nil,
    min: Date? = nil,
    max: Date? = nil,
    weekStartsOn: Int = 2,
    strings: CalendarStrings = CalendarStrings(),
    today: Date? = nil,
    navigation: Bool = true,
    tint: Tint = .primary
  ) {
    self.mode = .range
    self._selection = .constant(nil)
    self._range = range
    self.size = size
    self.months = months
    self.month = month
    self.min = min
    self.max = max
    self.weekStartsOn = weekStartsOn
    self.strings = strings
    self.today = today
    self.navigation = navigation
    self.tint = tint
  }

  private var math: DayMath { DayMath(weekStartsOn: weekStartsOn) }

  private var todayDay: Date { math.startOfDay(today ?? Date()) }

  /// The value, read as a range either way: a single day is a range whose two
  /// ends are the same day.
  private var span: DateRange {
    mode == .range ? range : DateRange(start: selection, end: selection)
  }

  private var pending: Bool { mode == .range && range.start != nil && range.end == nil }

  private var firstMonth: Date {
    math.startOfMonth(month?.wrappedValue ?? ownMonth ?? span.start ?? todayDay)
  }

  private func setMonth(_ next: Date) {
    let first = math.startOfMonth(next)
    if let month { month.wrappedValue = first } else { ownMonth = first }
  }

  private func visible(_ day: Date) -> Bool {
    (0..<months).contains { math.sameMonth(math.addMonths(firstMonth, $0), day) }
  }

  private func disabled(_ day: Date) -> Bool {
    if let min, math.compareDay(day, min) == .orderedAscending { return true }
    if let max, math.compareDay(day, max) == .orderedDescending { return true }
    return false
  }

  private func pick(_ day: Date) {
    guard !disabled(day) else { return }
    focused = day
    if mode == .range {
      if range.start == nil || range.end != nil {
        range = DateRange(start: day)
      } else if let start = range.start, math.compareDay(day, start) == .orderedAscending {
        range = DateRange(start: day, end: start)
      } else {
        range = DateRange(start: range.start, end: day)
      }
    } else {
      selection = day
    }
  }

  private func moveFocus(_ day: Date) {
    focused = day
    if !visible(day) {
      // Reached past the shown months: slide them so the day is in the first
      // one going back, or the last one going forward.
      let forward = math.compareDay(day, firstMonth) == .orderedDescending
      setMonth(forward ? math.addMonths(day, 1 - months) : day)
    }
  }

  private var focusedDay: Date { focused ?? span.start ?? todayDay }

  private func move(_ direction: MoveCommandDirection) {
    let day = focusedDay
    switch direction {
    case .left: moveFocus(math.addDays(day, -1))
    case .right: moveFocus(math.addDays(day, 1))
    case .up: moveFocus(math.addDays(day, -7))
    case .down: moveFocus(math.addDays(day, 7))
    @unknown default: break
    }
  }

  /// The keys `onMoveCommand` does not carry. Returns whether it was one.
  private func key(_ key: KeyEquivalent, shift: Bool) -> Bool {
    let day = focusedDay
    let offset = (math.weekday(day) - weekStartsOn + 7) % 7
    // Compared by character: `KeyEquivalent` is only `Equatable` from macOS
    // 14, and this runs under the package's floor.
    switch key.character {
    case KeyEquivalent.pageUp.character:
      moveFocus(sameDayIn(math.addMonths(day, shift ? -12 : -1), as: day))
    case KeyEquivalent.pageDown.character:
      moveFocus(sameDayIn(math.addMonths(day, shift ? 12 : 1), as: day))
    case KeyEquivalent.home.character: moveFocus(math.addDays(day, -offset))
    case KeyEquivalent.end.character: moveFocus(math.addDays(day, 6 - offset))
    case KeyEquivalent.return.character, KeyEquivalent.space.character: pick(day)
    default: return false
    }
    return true
  }

  private func sameDayIn(_ month: Date, as day: Date) -> Date {
    let last = math.calendar.range(of: .day, in: .month, for: month)?.count ?? 28
    return math.addDays(month, Swift.min(math.day(day), last) - 1)
  }

  public var body: some View {
    let vars = theme.vars
    let cell = size == .small ? vars.calendarSmallCellSize : vars.calendarMediumCellSize

    HStack(alignment: .top, spacing: vars.spacing4) {
      ForEach(0..<months, id: \.self) { index in
        monthView(index: index, cell: cell, vars: vars)
      }
    }
    .onHover { if !$0 { hovered = nil } }
    // One focus stop for the whole grid, which lands on the day last reached.
    .focusable()
    .focused($gridFocused)
    .onMoveCommand(perform: move)
    .modifier(CalendarKeys(handle: key))
    .accessibilityElement(children: .contain)
  }

  private func monthView(index: Int, cell: CGFloat, vars: ThemeVariables) -> some View {
    let current = math.addMonths(firstMonth, index)
    let days = math.monthGrid(current)
    let width = 7 * cell + 6 * vars.calendarCellGap

    return VStack(spacing: vars.spacing1) {
      // The bar: a chevron at each end of the month's name, in the tiny glyph
      // square. A middle month of a run has neither, and keeps the room.
      HStack(spacing: vars.spacing1) {
        if navigation {
          if index == 0 {
            IconButton(label: strings.previousMonth, icon: .chevronLeft12Regular) {
              setMonth(math.addMonths(firstMonth, -1))
            }
          } else {
            Color.clear.frame(width: vars.controlTinySize, height: vars.controlTinySize)
          }
        }
        Text(strings.monthTitle(current))
          .font(.system(size: vars.labelMedium.size, weight: vars.labelMedium.weight))
          .monospacedDigit()
          .foregroundStyle(vars.colorContent)
          .lineLimit(1)
          .frame(maxWidth: .infinity)
        if navigation {
          if index == months - 1 {
            IconButton(label: strings.nextMonth, icon: .chevronRight12Regular) {
              setMonth(math.addMonths(firstMonth, 1))
            }
          } else {
            Color.clear.frame(width: vars.controlTinySize, height: vars.controlTinySize)
          }
        }
      }
      HStack(spacing: vars.calendarCellGap) {
        ForEach(0..<7, id: \.self) { i in
          Text(strings.weekday((weekStartsOn - 1 + i) % 7 + 1))
            .font(.system(size: vars.captionSmall.size, weight: vars.labelSmall.weight))
            .foregroundStyle(vars.colorContentSubtle)
            .frame(width: cell, height: vars.spacing5)
        }
      }
      VStack(spacing: vars.calendarCellGap) {
        ForEach(0..<6, id: \.self) { row in
          HStack(spacing: vars.calendarCellGap) {
            ForEach(days[row * 7..<row * 7 + 7], id: \.self) { day in
              dayView(day, current: current, cell: cell, vars: vars)
            }
          }
        }
      }
    }
    .frame(width: width)
  }

  private func dayView(_ day: Date, current: Date, cell: CGFloat, vars: ThemeVariables)
    -> some View
  {
    let value = span
    let isStart = math.sameDay(day, value.start)
    let isEnd = math.sameDay(day, value.end)
    let picked = mode == .range ? isStart || isEnd : isStart
    // What the span to draw is: the two picked ends, or the one picked end
    // and the day being previewed against it.
    let preview = pending ? (hovered ?? focusedDay) : nil
    var lo: Date?
    var hi: Date?
    if mode == .range, let a = value.start, let b = pending ? preview : value.end {
      lo = math.compareDay(a, b) != .orderedDescending ? a : b
      hi = math.compareDay(a, b) != .orderedDescending ? b : a
    }
    let inSpan =
      lo != nil && hi != nil && math.compareDay(day, lo!) != .orderedAscending
      && math.compareDay(day, hi!) != .orderedDescending
    let spanStart = inSpan && math.sameDay(day, lo)
    let spanEnd = inSpan && math.sameDay(day, hi)
    let previewed = pending && preview != nil && math.sameDay(day, preview) && !isStart
    let outside = !math.sameMonth(day, current)
    let isToday = math.sameDay(day, todayDay)
    let off = disabled(day)
    let isFocused = gridFocused && math.sameDay(day, focusedDay)
    let radius = vars.radiusSmall

    return Pressable(action: { pick(day) }, enabled: !off, cornerRadius: radius) { state in
      let colors: (surface: Color, content: Color) = {
        if picked || inSpan {
          let drawn: ControlVariant = picked ? .filled : .tinted
          let c = vars.controlColors(variant: drawn, tint: tint, state: state.controlState)
          return (c.surface, c.content)
        }
        // At rest: the row's drawing, not the quiet accent control's.
        let surface =
          state.enabled && (state.hovered || state.pressed)
          ? vars.resolve(vars.descriptor(.normal, .surface), tint: tint, state: .hovered)
          : Color.clear
        let content =
          off || outside
          ? vars.colorContentFaint : (isToday ? vars.ramp(tint)[600] : vars.colorContent)
        return (surface, content)
      }()
      // Today is a weight heavier, and a picked day the selection's weight —
      // the half of a selection that survives on a monochrome theme.
      let weight =
        isToday
        ? vars.labelStrong.weight : (picked ? vars.labelMedium.weight : vars.labelQuiet.weight)

      Text("\(math.day(day))")
        .font(.system(size: vars.labelQuiet.size, weight: weight))
        .monospacedDigit()
        .foregroundStyle(colors.content)
        .frame(width: cell, height: cell)
        .background(
          DayBand(radius: radius, start: !inSpan || spanStart, end: !inSpan || spanEnd)
            .fill(colors.surface)
        )
        .overlay(alignment: .bottom) {
          // Today's dot, in whatever ink the recipe gave the label, so today
          // is never lost under a selection.
          if isToday {
            Circle()
              .fill(colors.content)
              .frame(width: vars.spacing05, height: vars.spacing05)
              .padding(.bottom, vars.spacing05)
          }
        }
        .overlay {
          // The end a pending range is previewing against: dashed, because
          // it is where the span would end, not where it does.
          if previewed {
            RoundedRectangle(cornerRadius: radius, style: .circular)
              .strokeBorder(
                vars.ramp(tint)[600],
                style: StrokeStyle(lineWidth: vars.strokeHairline, dash: [3, 2])
              )
          }
        }
        .animation(vars.motion, value: state.controlState)
    }
    // The grid holds the focus; a day of its own would be a second stop.
    .focusable(false)
    .onHover { over in
      guard mode == .range else { return }
      if over { hovered = day }
    }
    .focusRing(
      visible: isFocused, color: vars.focusRing, cornerRadius: radius, width: vars.focusWidth
    )
    .accessibilityLabel(strings.dayLabel(day))
    .accessibilityAddTraits(picked || inSpan ? .isSelected : [])
  }
}

/// A day's background: a rounded square alone, and inside a range one band
/// whose days square their inner corners and whose two ends keep the corner
/// only on the outer side, so the band's edge stays concentric with the
/// day's.
struct DayBand: Shape {
  let radius: CGFloat
  let start: Bool
  let end: Bool

  func path(in rect: CGRect) -> Path {
    var path = Path()
    let r = Swift.min(radius, rect.width / 2, rect.height / 2)
    path.move(to: CGPoint(x: rect.minX + (start ? r : 0), y: rect.minY))
    path.addLine(to: CGPoint(x: rect.maxX - (end ? r : 0), y: rect.minY))
    if end {
      path.addArc(
        center: CGPoint(x: rect.maxX - r, y: rect.minY + r), radius: r,
        startAngle: .degrees(-90), endAngle: .degrees(0), clockwise: false)
    }
    path.addLine(to: CGPoint(x: rect.maxX, y: rect.maxY - (end ? r : 0)))
    if end {
      path.addArc(
        center: CGPoint(x: rect.maxX - r, y: rect.maxY - r), radius: r,
        startAngle: .degrees(0), endAngle: .degrees(90), clockwise: false)
    }
    path.addLine(to: CGPoint(x: rect.minX + (start ? r : 0), y: rect.maxY))
    if start {
      path.addArc(
        center: CGPoint(x: rect.minX + r, y: rect.maxY - r), radius: r,
        startAngle: .degrees(90), endAngle: .degrees(180), clockwise: false)
    }
    path.addLine(to: CGPoint(x: rect.minX, y: rect.minY + (start ? r : 0)))
    if start {
      path.addArc(
        center: CGPoint(x: rect.minX + r, y: rect.minY + r), radius: r,
        startAngle: .degrees(180), endAngle: .degrees(270), clockwise: false)
    }
    path.closeSubpath()
    return path
  }
}

/// The keys a move command does not carry — Page Up and Page Down, Home and
/// End, Return and Space — on the platforms that report them. Below macOS 14
/// the arrows alone move the focus.
private struct CalendarKeys: ViewModifier {
  let handle: (KeyEquivalent, Bool) -> Bool

  func body(content: Content) -> some View {
    if #available(macOS 14.0, *) {
      content.onKeyPress(phases: [.down, .repeat]) { press in
        handle(press.key, press.modifiers.contains(.shift)) ? .handled : .ignored
      }
    } else {
      content
    }
  }
}
