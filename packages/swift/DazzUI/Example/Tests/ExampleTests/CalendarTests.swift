import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

/// `Calendar` and `DateRangePicker` — the month grid's geometry, and the
/// sheet's own rules, which are data rather than drawing.
@MainActor
final class CalendarTests: XCTestCase {
  private let math = DayMath()
  private var today: Date {
    math.calendar.date(from: DateComponents(year: 2026, month: 10, day: 16))!
  }

  private func at(_ day: Int, _ hour: Int, _ minute: Int = 0) -> Date {
    math.calendar.date(
      from: DateComponents(year: 2026, month: 10, day: day, hour: hour, minute: minute))!
  }

  func testAMonthIsSevenCellsAndSixGapsWide() {
    for name in [ThemeName.studioLight, .studioDark, .windows11Light] {
      let vars = name.variables
      let single = NSHostingView(
        rootView: DazzUI.Calendar(selection: .constant(today), today: today).theme(name))
      XCTAssertEqual(
        single.fittingSize.width, 7 * vars.calendarMediumCellSize + 6 * vars.calendarCellGap,
        name.rawValue)

      let two = NSHostingView(
        rootView: DazzUI.Calendar(
          range: .constant(DateRange(start: today, end: today)), size: .small, months: 2,
          today: today
        ).theme(name))
      XCTAssertEqual(
        two.fittingSize.width,
        2 * (7 * vars.calendarSmallCellSize + 6 * vars.calendarCellGap) + vars.spacing4,
        name.rawValue)
    }
  }

  func testTheGridIsSixWeeksWhateverTheMonth() {
    let grid = math.monthGrid(today)
    XCTAssertEqual(grid.count, 42)
    XCTAssertEqual(math.weekday(grid[0]), 2)
    XCTAssertEqual(math.day(grid[0]), 28)
    XCTAssertEqual(math.day(grid[41]), 8)
  }

  func testTheTriggerIsAField() {
    for name in [ThemeName.studioLight, .macos27Dark] {
      let vars = name.variables
      let picker = NSHostingView(
        rootView: DateRangePicker(
          value: .constant(DateRangeValue(start: at(16, 14, 30), end: at(16, 15, 30))),
          today: today
        ).theme(name))
      XCTAssertEqual(picker.fittingSize.height, vars.controlMediumSize, name.rawValue)
    }
  }

  func testTheDraftKeepsTheSheetsRules() {
    var draft = DateRangeDraft(
      DateRangeValue(start: at(16, 14, 30), end: at(16, 15, 30)), math: math)

    // The first press leaves the calendar's end unset and the draft on one
    // day; the second finishes the span and the times ride along.
    draft.setDays(DateRange(start: at(20, 0)))
    XCTAssertNil(draft.days.end)
    XCTAssertEqual(draft.value.start, at(20, 14, 30))
    XCTAssertEqual(draft.value.end, at(20, 15, 30))
    draft.setDays(DateRange(start: at(20, 0), end: at(24, 0)))
    XCTAssertEqual(draft.value.end, at(24, 15, 30))
    XCTAssertFalse(draft.oneDay)

    // Moving the start carries the end with it.
    draft.setDays(DateRange(start: at(20, 0), end: at(20, 0)))
    draft.setStartTime(hour: 9, minute: 0)
    XCTAssertEqual(draft.value.end, at(20, 10, 0))

    // An end set before its start pulls the start back an hour ahead of it.
    draft.setEndTime(hour: 8, minute: 0)
    XCTAssertEqual(draft.value.start, at(20, 7, 0))
    XCTAssertEqual(draft.value.end, at(20, 8, 0))

    // A duration is a length from the start, and clears all-day.
    draft.value.allDay = true
    draft.setDuration(minutes: 120)
    XCTAssertFalse(draft.value.allDay)
    XCTAssertEqual(draft.value.end, at(20, 9, 0))
    XCTAssertTrue(draft.matches(dateRangeDurations[2]))
  }

  func testThePresetsCountFromToday() {
    let labels = dateRangePresets.map(\.label)
    XCTAssertEqual(
      labels, ["Today", "Tomorrow", "This weekend", "Next Monday", "Next week", "This month"])
    let weekend = dateRangePresets[2].range(today, math)
    XCTAssertEqual(math.day(weekend.start!), 17)
    XCTAssertEqual(math.day(weekend.end!), 18)
    let nextWeek = dateRangePresets[4].range(today, math)
    XCTAssertEqual(math.day(nextWeek.start!), 19)
    XCTAssertEqual(math.day(nextWeek.end!), 25)
    let month = dateRangePresets[5].range(today, math)
    XCTAssertEqual(math.day(month.start!), 1)
    XCTAssertEqual(math.day(month.end!), 31)
  }
}
