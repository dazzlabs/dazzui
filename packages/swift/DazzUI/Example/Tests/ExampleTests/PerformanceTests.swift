import AppKit
import SwiftUI
import XCTest

@testable import DazzUI
@testable import Example

@MainActor
final class PerformanceTests: XCTestCase {
  private var windows: [NSWindow] = []

  override func tearDown() {
    for window in windows {
      window.orderOut(nil)
      window.contentView = nil
    }
    windows = []
    super.tearDown()
  }

  private func host<V: View>(_ root: V) -> (NSWindow, NSHostingView<AnyView>) {
    let hosting = NSHostingView(
      rootView: AnyView(root.frame(width: 400, height: 240))
    )
    hosting.frame = NSRect(x: 0, y: 0, width: 400, height: 240)
    let window = NSWindow(
      contentRect: hosting.frame,
      styleMask: [.titled, .closable],
      backing: .buffered,
      defer: false
    )
    window.isReleasedWhenClosed = false
    window.contentView = hosting
    windows.append(window)
    hosting.layoutSubtreeIfNeeded()
    RunLoop.main.run(until: Date().addingTimeInterval(0.08))
    return (window, hosting)
  }

  private func descendants(_ view: NSView) -> [NSView] {
    [view] + view.subviews.flatMap(descendants)
  }

  // MARK: - Token & metric caches

  func testColorMathCacheDeduplicatesDerivedSemanticColors() {
    ColorMathCache.shared.resetForTesting()
    let vars = ThemeVariables.studioLight

    for _ in 0..<50 {
      _ = vars.accentSurface
      _ = vars.dangerSurface
      _ = vars.warnSurface
      _ = vars.successSurface
      _ = vars.railSurface
    }

    XCTAssertEqual(ColorMathCache.shared.totalComputations, 5)
  }

  func testTypeStyleCssBaselineSharesTypeMetricsCache() {
    TypeMetricsCache.shared.resetForTesting()
    let body = ThemeVariables.studioLight.bodyMedium

    let natural = body.naturalLineHeight
    let baseline1 = body.cssBaseline
    let baseline2 = body.cssBaseline

    XCTAssertGreaterThan(natural, 0)
    XCTAssertEqual(baseline1, baseline2)
    XCTAssertEqual(TypeMetricsCache.shared.totalComputations, 1)
  }

  func testColorSwatchFastLookupMatchesAllStandardShades() {
    let swatch = Colors.ink
    for shade in [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950] {
      XCTAssertEqual(swatch[shade], swatch.shades[shade])
    }
    XCTAssertEqual(swatch[999], .clear)
    XCTAssertEqual(swatch, Colors.ink)
  }

  // MARK: - Lazy portals, tooltips, and native menus

  private final class Counter {
    var count = 0
  }

  private final class PresentationToggle: ObservableObject {
    @Published var isPresented = false
  }

  private struct FloatingProbe: View {
    @ObservedObject var toggle: PresentationToggle
    let builds: Counter

    var body: some View {
      Text("Trigger")
        .floatingPanel(isPresented: $toggle.isPresented, gap: 4, modal: false) { _ in
          builds.count += 1
          return Text("Panel").frame(width: 120, height: 40)
        }
        .theme(.studioLight)
    }
  }

  func testClosedFloatingPanelDoesNotEvaluatePanelUntilPresented() {
    let toggle = PresentationToggle()
    let builds = Counter()
    let (window, _) = host(FloatingProbe(toggle: toggle, builds: builds))

    XCTAssertEqual(builds.count, 0, "Closed floatingPanel must not evaluate its panel builder")
    XCTAssertTrue(window.childWindows?.isEmpty ?? true)

    toggle.isPresented = true
    RunLoop.main.run(until: Date().addingTimeInterval(0.08))
    XCTAssertGreaterThan(builds.count, 0)
    XCTAssertEqual(window.childWindows?.count, 1)

    toggle.isPresented = false
    RunLoop.main.run(until: Date().addingTimeInterval(0.08))
    XCTAssertTrue(window.childWindows?.isEmpty ?? true)
  }

  func testRestingTooltipsDoNotEvaluateLabelsOrMountAnchors() {
    let labelBuilds = Counter()
    let (_, hosting) = host(
      HStack {
        ForEach(0..<10, id: \.self) { index in
          Tooltip(
            presented: false,
            label: {
              labelBuilds.count += 1
              return Text("Tip \(index)")
            },
            content: {
              Text("\(index)")
            }
          )
        }
      }
      .theme(.studioLight)
    )

    XCTAssertEqual(labelBuilds.count, 0, "Resting tooltips must not evaluate their label builder")
    let tooltipAnchors = descendants(hosting).filter {
      String(describing: type(of: $0)).contains("TooltipAnchor")
    }
    XCTAssertTrue(tooltipAnchors.isEmpty, "Resting tooltips must not mount TooltipAnchor NSViews")
  }

  func testCustomModeMenusSkipNativeAnchorsAndClosedNativeMenusSkipEntryEvaluation() {
    let (_, customHost) = host(
      VStack {
        DazzUI.Menu(items: [MenuItem("Open")]) { state in
          DazzUI.Button("Menu", action: state.toggle)
        }
        DazzUI.Select(
          options: [SelectOption(value: "a", label: "Ada")],
          selection: .constant("a")
        )
      }
      .theme(.studioLight)
      .menuPresentation(.custom)
    )

    XCTAssertTrue(
      descendants(customHost).compactMap { $0 as? NativeMenuAnchorView }.isEmpty,
      "Custom menu mode must not mount NativeMenuAnchorView"
    )

    let entryBuilds = Counter()
    _ = host(
      Text("Trigger")
        .nativeMenu(
          isPresented: .constant(false),
          hasEntries: true,
          entries: {
            entryBuilds.count += 1
            return [.item(MenuItem("One"))]
          }
        )
        .theme(.studioLight)
        .menuPresentation(.native)
    )

    XCTAssertEqual(
      entryBuilds.count, 0,
      "Closed native menu must not evaluate its entries closure"
    )
  }

  // MARK: - Invalidation isolation

  private struct CountedContent: View {
    let renders: Counter
    let onReady: (OverlayStack?, Toaster?) -> Void

    @Environment(\.overlayStack) private var overlayStack
    @Environment(\.toaster) private var toaster

    var body: some View {
      renders.count += 1
      return Text("Stable")
        .onAppear { onReady(overlayStack, toaster) }
    }
  }

  func testOverlayStackAndToasterMutationsDoNotInvalidateHostContent() {
    let renders = Counter()
    var capturedStack: OverlayStack?
    var capturedToaster: Toaster?

    _ = host(
      ToastProvider {
        ZStack {
          CountedContent(renders: renders) { stack, toaster in
            capturedStack = stack
            capturedToaster = toaster
          }
          ToasterViewport()
        }
      }
      .theme(.studioLight)
      .overlayHost()
    )

    let baseline = renders.count
    XCTAssertGreaterThan(baseline, 0)
    XCTAssertNotNil(capturedStack)
    XCTAssertNotNil(capturedToaster)

    let layerId = UUID()
    capturedStack?.show(layerId, AnyView(Text("Dialog")))
    let toastId = capturedToaster?.add(ToastOptions(title: "Saved", timeout: 0))
    RunLoop.main.run(until: Date().addingTimeInterval(0.06))

    capturedStack?.hide(layerId)
    if let toastId { capturedToaster?.close(toastId) }
    RunLoop.main.run(until: Date().addingTimeInterval(0.06))

    XCTAssertEqual(
      renders.count, baseline,
      "OverlayStack and Toaster mutations must not re-evaluate wrapped content"
    )
  }

  // MARK: - System sizing ticker lifecycle

  func testDesignSizingDoesNotRegisterSystemSizingTicker() {
    let baseline = SystemSizingTicker.shared.listenerCount
    _ = host(
      Text("Design")
        .theme(.studioLight, sizing: .design)
    )
    XCTAssertEqual(
      SystemSizingTicker.shared.listenerCount,
      baseline,
      "Design sizing must not register a 1-second system-sizing timer listener"
    )

    // Verify a standalone ticker starts lazily on first listener and stops when empty.
    let ticker = SystemSizingTicker()
    XCTAssertFalse(ticker.isRunning)
    let id = UUID()
    ticker.add(id) {}
    XCTAssertTrue(ticker.isRunning)
    ticker.remove(id)
    XCTAssertFalse(ticker.isRunning)
  }

  // MARK: - COW ThemeVariables & cached controlColors

  func testThemeVariablesIsPointerSizedWithCopyOnWriteIsolation() {
    XCTAssertEqual(
      MemoryLayout<ThemeVariables>.size,
      MemoryLayout<AnyObject>.size,
      "ThemeVariables must remain pointer-sized (COW-boxed) to avoid multi-KB copies in SwiftUI bodies"
    )

    let original = ThemeVariables.studioLight
    let originalSpacing = original.spacing4
    let originalSurface = original.controlColor(.normal, .surface, tint: .primary, state: .normal)

    var copy = original
    copy.spacing4 = 999
    copy.colorSurface = .red

    XCTAssertEqual(original.spacing4, originalSpacing)
    XCTAssertEqual(copy.spacing4, 999)
    XCTAssertEqual(
      original.controlColor(.normal, .surface, tint: .primary, state: .normal),
      originalSurface
    )
    XCTAssertNotEqual(
      copy.controlColor(.normal, .surface, tint: .primary, state: .normal),
      originalSurface,
      "Mutating a color token on a ThemeVariables copy must invalidate its controlColors cache"
    )
  }

  func testCachedControlColorsMatchesUncachedResolutionAcrossAllCombinations() {
    let themes: [ThemeVariables] = [
      .brightLight,
      .brightDark,
      .studioLight,
      .studioDark,
    ]
    for vars in themes {
      for variant in ControlVariant.allCases {
        for tint in Tint.allCases {
          for state in ControlState.allCases {
            let cached = vars.controlColors(variant: variant, tint: tint, state: state)
            let expectedSurface = vars.resolve(
              vars.descriptor(variant, .surface),
              tint: tint,
              state: state
            )
            let expectedBorder = vars.resolve(
              vars.descriptor(variant, .border),
              tint: tint,
              state: state
            )
            let expectedContent = vars.resolve(
              vars.descriptor(variant, .content),
              tint: tint,
              state: state
            )
            XCTAssertEqual(cached.surface, expectedSurface)
            XCTAssertEqual(cached.border, expectedBorder)
            XCTAssertEqual(cached.content, expectedContent)
            XCTAssertEqual(
              vars.controlColor(variant, .surface, tint: tint, state: state),
              expectedSurface
            )
            XCTAssertEqual(
              vars.controlColor(variant, .border, tint: tint, state: state),
              expectedBorder
            )
            XCTAssertEqual(
              vars.controlColor(variant, .content, tint: tint, state: state),
              expectedContent
            )
          }
        }
      }
    }
  }

  func testTypeStylePrecomputedFontAndEquality() {
    let vars = ThemeVariables.studioLight
    let style = vars.bodyMedium
    XCTAssertEqual(style.font, style.resolvedFont(weight: nil))
    XCTAssertEqual(
      style,
      TypeStyle(
        family: style.family,
        size: style.size,
        weight: style.weight,
        lineHeight: style.lineHeight
      )
    )
  }

  // MARK: - Shortcut formatting & Example JSON decoding

  func testFormatShortcutZeroRegexMatchesAllKeyShapes() {
    XCTAssertEqual(
      formatShortcut(ShortcutEvent(code: "KeyT", altKey: true, shiftKey: true)),
      "⌥⇧T"
    )
    XCTAssertEqual(
      formatShortcut(ShortcutEvent(code: "Digit2", altKey: true, shiftKey: true)),
      "⌥⇧2"
    )
    XCTAssertEqual(
      formatShortcut(ShortcutEvent(code: "Numpad5", metaKey: true)),
      "⌘5"
    )
    XCTAssertEqual(formatShortcut(ShortcutEvent(code: "F1")), "F1")
    XCTAssertEqual(formatShortcut(ShortcutEvent(code: "F12", metaKey: true)), "⌘ F12")
    XCTAssertEqual(formatShortcut(ShortcutEvent(code: "F20")), "F20")
    XCTAssertNil(formatShortcut(ShortcutEvent(code: "F0")))
    XCTAssertNil(formatShortcut(ShortcutEvent(code: "F01")))
    XCTAssertNil(formatShortcut(ShortcutEvent(code: "F21")))
    XCTAssertEqual(formatShortcut(ShortcutEvent(code: "Space", altKey: true)), "⌥ Space")
    XCTAssertEqual(formatShortcut(ShortcutEvent(code: "", key: "k", metaKey: true)), "⌘K")
    XCTAssertNil(formatShortcut(ShortcutEvent(code: "KeyA", shiftKey: true)))
  }

  func testStoryValueParsePreservesJsonTypes() throws {
    let json = """
      {
        "nullVal": null,
        "boolTrue": true,
        "boolFalse": false,
        "zero": 0,
        "one": 1,
        "float": 3.5,
        "str": "hello",
        "arr": [true, false, 0, 1, "x", null],
        "nested": { "flag": true, "count": 42 }
      }
      """
    let parsed = Value.parse(json)
    let decoded = try JSONDecoder().decode(Value.self, from: Data(json.utf8))
    XCTAssertEqual(parsed, decoded)
    XCTAssertEqual(parsed["boolTrue"], .bool(true))
    XCTAssertEqual(parsed["boolFalse"], .bool(false))
    XCTAssertEqual(parsed["zero"], .number(0))
    XCTAssertEqual(parsed["one"], .number(1))
    XCTAssertEqual(parsed["float"], .number(3.5))
    XCTAssertEqual(parsed["nullVal"], .null)
  }

  // MARK: - Round 3 caches, layout & sizing isolation

  func testControlMetricsCacheAndInvalidationOnMutation() {
    let original = ThemeVariables.studioLight
    let initialSmall = original.metrics(.small)
    XCTAssertEqual(initialSmall.size, original.controlSmallSize)
    XCTAssertEqual(initialSmall.content, original.controlSmallContent)

    var copy = original
    copy.controlSmallSize = 99
    XCTAssertEqual(copy.metrics(.small).size, 99)
    XCTAssertEqual(original.metrics(.small).size, initialSmall.size)

    let systemTheme = Theme(.studioLight, sizing: .system)
    for size in DazzUI.ControlSize.allCases {
      let m1 = systemTheme.vars.metrics(size)
      let m2 = systemTheme.vars.metrics(size)
      XCTAssertEqual(m1, m2)
      XCTAssertGreaterThan(m1.size, 0)
    }
  }

  func testDesignSizingModifierDoesNotProbeSystemMetrics() {
    let probes = Counter()
    let probe: @MainActor () -> SystemSizingMetrics = {
      probes.count += 1
      return SystemSizingMetrics.current()
    }
    _ = host(
      Text("Design only")
        .modifier(SystemThemeModifier(name: .studioLight, sizing: .design, readMetrics: probe))
    )
    XCTAssertEqual(
      probes.count, 0,
      "Design sizing must not invoke readMetrics() during init or body evaluation"
    )
  }

  func testCalendarMonthGridAndDateRangeTimeOptionsCachesDeduplicate() {
    MonthGridCache.shared.resetForTesting()
    TimeOptionsCache.shared.resetForTesting()

    let math = DayMath.cached(weekStartsOn: 2)
    let sample = math.startOfMonth(Date(timeIntervalSince1970: 1_760_000_000))
    for _ in 0..<25 {
      let grid = math.cachedMonthGrid(sample)
      XCTAssertEqual(grid.days.count, 42)
      XCTAssertEqual(grid.dayNumbers.count, 42)
      XCTAssertEqual(grid.dayNumbers[0], math.day(grid.days[0]))
    }
    XCTAssertEqual(MonthGridCache.shared.totalComputations, 1)

    for _ in 0..<25 {
      let opts = TimeOptionsCache.shared.options(step: 15)
      XCTAssertEqual(opts.count, 96)
      XCTAssertEqual(opts.first?.value, "00:00")
      XCTAssertEqual(opts.last?.value, "23:45")
    }
    XCTAssertEqual(TimeOptionsCache.shared.totalComputations, 1)
  }

  func testThemeFamilyGroupsReturnPrecomputedMatches() {
    for group in DazzUI.ThemeGroup.allCases {
      let expected = DazzUI.ThemeFamily.allCases.filter { $0.group == group }
      XCTAssertEqual(DazzUI.ThemeFamily.families(in: group), expected)
    }
  }

  func testProportionalBarLayoutRendersMeterProgressAndSkeletonText() {
    _ = host(
      VStack(alignment: .leading, spacing: 8) {
        Meter(value: 65, label: "Quota", showValue: true)
        Progress(value: 40)
        SkeletonText(lines: 3)
      }
      .frame(width: 240)
      .theme(.studioLight)
    )
  }

  func testSceneIsLiveAndResolveAvoidFalsePositives() {
    let state = SceneState(["flag": .bool(true), "name": .string("Ada")])
    let ctx = Node.Context(scene: state, toaster: nil, press: nil)
    XCTAssertFalse(isLive(.string("plain")))
    XCTAssertFalse(isLive(.object(["plainKey": .number(1)])))
    XCTAssertTrue(isLive(.object(["$state": .string("flag")])))
    XCTAssertEqual(ctx.resolve(.object(["$state": .string("name")])), .string("Ada"))
    XCTAssertEqual(
      ctx.resolve(.object(["$not": .object(["$state": .string("flag")])])),
      .bool(false)
    )
    XCTAssertEqual(
      ctx.resolve(
        .object([
          "$concat": .array([.string("Hi "), .object(["$state": .string("name")])])
        ])
      ),
      .string("Hi Ada")
    )
    XCTAssertEqual(
      ctx.resolve(.object(["$arg": .number(-1)]), [.string("x")]),
      .null,
      "Negative $arg index must safely return .null instead of trapping"
    )
    XCTAssertEqual(
      ctx.resolve(.object(["$arg": .number(0)]), [.string("x")]),
      .string("x")
    )
  }

  private struct BarFrameKey: PreferenceKey {
    static var defaultValue: CGRect = .zero
    static func reduce(value: inout CGRect, nextValue: () -> CGRect) {
      value = nextValue()
    }
  }

  private final class FrameBox {
    var rect: CGRect = .zero
  }

  private func measureBarFill(
    fraction: Double,
    containerWidth: CGFloat = 200,
    containerHeight: CGFloat = 10,
    layoutDirection: LayoutDirection = .leftToRight
  ) -> CGRect {
    let box = FrameBox()
    _ = host(
      VStack {
        HStack {
          ProportionalBarLayout(fraction: fraction) {
            Color.blue
              .background(
                GeometryReader { proxy in
                  Color.clear.preference(
                    key: BarFrameKey.self,
                    value: proxy.frame(in: .named("barTrack"))
                  )
                }
              )
          }
          .frame(width: containerWidth, height: containerHeight)
          .coordinateSpace(name: "barTrack")
          .onPreferenceChange(BarFrameKey.self) { box.rect = $0 }
        }
      }
      .environment(\.layoutDirection, layoutDirection)
      .theme(.studioLight)
    )
    return box.rect
  }

  func testProportionalBarLayoutGeometryEdgeCasesAndRTL() {
    let zero = measureBarFill(fraction: 0)
    XCTAssertEqual(zero.width, 0, accuracy: 0.5)
    XCTAssertEqual(zero.height, 10, accuracy: 0.5)

    let negative = measureBarFill(fraction: -0.5)
    XCTAssertEqual(negative.width, 0, accuracy: 0.5)

    let nanFrame = measureBarFill(fraction: .nan)
    XCTAssertEqual(nanFrame.width, 0, accuracy: 0.5)
    XCTAssertFalse(nanFrame.width.isNaN)

    let infFrame = measureBarFill(fraction: .infinity)
    XCTAssertEqual(infFrame.width, 0, accuracy: 0.5)

    let quarter = measureBarFill(fraction: 0.25)
    XCTAssertEqual(quarter.minX, 0, accuracy: 0.5)
    XCTAssertEqual(quarter.width, 50, accuracy: 0.5)

    let sixty = measureBarFill(fraction: 0.6)
    XCTAssertEqual(sixty.width, 120, accuracy: 0.5)

    let full = measureBarFill(fraction: 1.0)
    XCTAssertEqual(full.width, 200, accuracy: 0.5)

    let over = measureBarFill(fraction: 1.5)
    XCTAssertEqual(over.width, 200, accuracy: 0.5)

    let rtlQuarter = measureBarFill(fraction: 0.25, layoutDirection: .rightToLeft)
    XCTAssertEqual(rtlQuarter.width, 50, accuracy: 0.5)
    XCTAssertEqual(rtlQuarter.maxX, 200, accuracy: 0.5, "RTL bar must anchor to trailing/right edge")
    XCTAssertEqual(rtlQuarter.minX, 150, accuracy: 0.5)

    // Flexible zero-fraction meter & progress inside stacks must not produce NaN.
    _ = host(
      VStack {
        HStack {
          Meter(value: 0, label: "Empty", showValue: true)
          Progress(value: 0)
        }
      }
      .frame(maxWidth: .infinity)
      .theme(.studioLight)
    )
  }

  func testDateStringCachesAvoidMidnightCollisionDeduplicateSameDayAndHandleExtremeDates() {
    CalendarStrings.dayLabelCache.resetForTesting()
    CalendarStrings.monthTitleCache.resetForTesting()
    DateRangePickerStrings.dayCache.resetForTesting()

    var gregorian = Foundation.Calendar(identifier: .gregorian)
    gregorian.timeZone = TimeZone.current

    let oct16Late = gregorian.date(
      from: DateComponents(
        year: 2026, month: 10, day: 16, hour: 23, minute: 59, second: 59, nanosecond: 600_000_000
      )
    )!
    let oct17Midnight = gregorian.date(
      from: DateComponents(year: 2026, month: 10, day: 17, hour: 0, minute: 0, second: 0)
    )!

    XCTAssertEqual(CalendarStrings.dayLabel(oct16Late), "Friday, October 16, 2026")
    XCTAssertEqual(CalendarStrings.dayLabel(oct17Midnight), "Saturday, October 17, 2026")

    XCTAssertEqual(DateRangePickerStrings.day(oct16Late), "Fri, Oct 16")
    XCTAssertEqual(DateRangePickerStrings.day(oct17Midnight), "Sat, Oct 17")

    let oct31Late = gregorian.date(
      from: DateComponents(
        year: 2026, month: 10, day: 31, hour: 23, minute: 59, second: 59, nanosecond: 600_000_000
      )
    )!
    let nov1Midnight = gregorian.date(
      from: DateComponents(year: 2026, month: 11, day: 1, hour: 0, minute: 0, second: 0)
    )!
    XCTAssertEqual(CalendarStrings.monthTitle(oct31Late), "October 2026")
    XCTAssertEqual(CalendarStrings.monthTitle(nov1Midnight), "November 2026")

    // Different times on the same local day must share one cached DateRangePickerStrings.day entry.
    DateRangePickerStrings.dayCache.resetForTesting()
    let times = [(9, 0), (9, 15), (14, 7), (23, 59)].map { hour, minute in
      gregorian.date(
        from: DateComponents(year: 2026, month: 10, day: 16, hour: hour, minute: minute)
      )!
    }
    for t in times {
      XCTAssertEqual(DateRangePickerStrings.day(t), "Fri, Oct 16")
    }
    XCTAssertEqual(
      DateRangePickerStrings.dayCache.totalComputations, 1,
      "All times on the same local day should share a single cached formatted string"
    )

    // Extreme dates must never trap on Int64 conversion.
    let extreme = Date(timeIntervalSinceReferenceDate: 1e20)
    _ = CalendarStrings.dayLabel(extreme)
    _ = CalendarStrings.monthTitle(extreme)
    _ = DateRangePickerStrings.day(extreme)
    _ = DayMath.cached(weekStartsOn: 2).cachedMonthGrid(extreme)
  }

  func testDayMathCachedMonthGridNormalizesStartOfDay() {
    let math = DayMath.cached(weekStartsOn: 1)
    let sample = Date(timeIntervalSince1970: 1_772_928_000) // March 2026 (DST month)
    let grid = math.cachedMonthGrid(sample)
    XCTAssertEqual(grid.days.count, 42)
    for day in grid.days {
      XCTAssertEqual(
        day, math.startOfDay(day),
        "Every cached month grid cell must be normalized to startOfDay"
      )
    }
  }

  private final class ParentTick: ObservableObject {
    @Published var tick = 0
    @Published var sizing: ThemeSizing = .system
    @Published var themeName: ThemeName = .studioLight
  }

  private struct SystemSizingRedrawProbe: View {
    @ObservedObject var state: ParentTick
    let readMetrics: @MainActor () -> SystemSizingMetrics

    var body: some View {
      Text("Tick \(state.tick)")
        .modifier(
          SystemThemeModifier(
            name: state.themeName,
            sizing: state.sizing,
            readMetrics: readMetrics
          )
        )
    }
  }

  func testSystemThemeModifierDoesNotReprobeMetricsOnParentRedraws() {
    let state = ParentTick()
    let probes = Counter()
    let snapshot = SystemSizingMetrics.current()
    let probe: @MainActor () -> SystemSizingMetrics = {
      probes.count += 1
      return snapshot
    }

    _ = host(SystemSizingRedrawProbe(state: state, readMetrics: probe))
    let afterMount = probes.count
    XCTAssertGreaterThan(afterMount, 0)

    for i in 1...10 {
      state.tick = i
      state.themeName = i.isMultiple(of: 2) ? .studioLight : .studioDark
      RunLoop.main.run(until: Date().addingTimeInterval(0.01))
    }

    XCTAssertEqual(
      probes.count, afterMount,
      "SystemThemeModifier must not re-probe readMetrics() on parent redraws after mount"
    )
  }

  func testDelayedPresentedTooltipDoesNotMountPortalBeforeDelayCompletes() {
    let labelBuilds = Counter()
    let (_, hosting) = host(
      Tooltip(
        delay: 10,
        presented: true,
        label: {
          labelBuilds.count += 1
          return Text("Delayed")
        },
        content: {
          Text("Anchor")
        }
      )
      .theme(.studioLight)
    )

    XCTAssertEqual(labelBuilds.count, 0)
    let anchors = descendants(hosting).filter {
      String(describing: type(of: $0)).contains("TooltipAnchor")
    }
    XCTAssertTrue(
      anchors.isEmpty,
      "Tooltip waiting on its opening delay must not prematurely mount TooltipAnchor"
    )
  }

  // MARK: - CodeBlock CRLF trimming/splitting & DayMath live timezone tracking

  func testCodeBlockTrimsAndSplitsLFAndCRLFWithoutCorruptingUnicode() {
    let lf = CodeBlock(code: "let a = 1\nlet b = 2\n\n")
    XCTAssertEqual(lf.trimmedBody, "let a = 1\nlet b = 2")
    XCTAssertEqual(lf.lineSlices, ["let a = 1", "let b = 2"])

    let crlf = CodeBlock(code: "let a = 1\r\nlet b = 2\r\n\r\n")
    XCTAssertEqual(crlf.trimmedBody, "let a = 1\r\nlet b = 2")
    XCTAssertEqual(crlf.lineSlices, ["let a = 1", "let b = 2"])

    let mixedTrailing = CodeBlock(code: "one\r\n\r\ntwo\n\r\nthree\r\n\n\r\n")
    XCTAssertEqual(mixedTrailing.trimmedBody, "one\r\n\r\ntwo\n\r\nthree")
    XCTAssertEqual(mixedTrailing.lineSlices, ["one", "", "two", "", "three"])

    let leading = CodeBlock(code: "\r\nfirst\nsecond")
    XCTAssertEqual(leading.trimmedBody, "\r\nfirst\nsecond")
    XCTAssertEqual(leading.lineSlices, ["", "first", "second"])

    let empty = CodeBlock(code: "")
    XCTAssertEqual(empty.trimmedBody, "")
    XCTAssertEqual(empty.lineSlices, [""])

    let onlyCRLF = CodeBlock(code: "\r\n\r\n")
    XCTAssertEqual(onlyCRLF.trimmedBody, "")
    XCTAssertEqual(onlyCRLF.lineSlices, [""])

    let onlyLF = CodeBlock(code: "\n\n")
    XCTAssertEqual(onlyLF.trimmedBody, "")
    XCTAssertEqual(onlyLF.lineSlices, [""])

    let bareCR = CodeBlock(code: "foo\rbar\r\nbaz\r")
    XCTAssertEqual(bareCR.trimmedBody, "foo\rbar\r\nbaz\r")
    XCTAssertEqual(bareCR.lineSlices, ["foo\rbar", "baz\r"])

    let crEdgeCases: [(String, Substring, [Substring])] = [
      ("\r", "\r", ["\r"]),
      ("\r\r\n", "\r", ["\r"]),
      ("\n\r", "\n\r", ["", "\r"]),
      ("a\r\r\nb", "a\r\r\nb", ["a\r", "b"]),
      ("a\n\u{0301}b\r\n", "a\n\u{0301}b", ["a", "\u{0301}b"]),
    ]
    for (input, expectedBody, expectedLines) in crEdgeCases {
      let block = CodeBlock(code: input)
      XCTAssertEqual(block.trimmedBody, expectedBody)
      XCTAssertEqual(block.lineSlices, expectedLines)
    }

    let unicode = CodeBlock(code: "café\r\ne\u{0301}\n🚀👨‍👩‍👧‍👦\r\n日本語\r\n")
    XCTAssertEqual(unicode.trimmedBody, "café\r\ne\u{0301}\n🚀👨‍👩‍👧‍👦\r\n日本語")
    XCTAssertEqual(unicode.lineSlices, ["café", "e\u{0301}", "🚀👨‍👩‍👧‍👦", "日本語"])

    // Bridged UTF-16 NSString backing storage with surrogate-pair emoji and CRLF.
    let utf16Units = Array("A\r\n🚀👨‍👩‍👧‍👦\r\n日本語\r\n\r\n".utf16)
    let bridged = NSString(characters: utf16Units, length: utf16Units.count) as String
    let bridgedBlock = CodeBlock(code: bridged)
    XCTAssertEqual(bridgedBlock.trimmedBody, "A\r\n🚀👨‍👩‍👧‍👦\r\n日本語")
    XCTAssertEqual(bridgedBlock.lineSlices, ["A", "🚀👨‍👩‍👧‍👦", "日本語"])

    _ = host(
      CodeBlock(code: "alpha\r\nbeta\r\ngamma\r\n", lineNumbers: true)
        .theme(.studioLight)
    )
  }

  func testDayMathTracksLiveTimeZoneChangesWhileCachingPerTimeZone() {
    let previousTZ = getenv("TZ").map { String(cString: $0) }
    DayMathCache.shared.resetForTesting()
    MonthGridCache.shared.resetForTesting()
    CalendarStrings.dayLabelCache.resetForTesting()
    CalendarStrings.monthTitleCache.resetForTesting()
    DateRangePickerStrings.dayCache.resetForTesting()
    defer {
      // Unpin CFTimeZone default (`nil`) rather than assigning `NSTimeZone.default = previous`
      // so `NSTimeZone.default` continues tracking future system timezone resets.
      CFTimeZoneSetDefault(nil)
      if let previousTZ {
        setenv("TZ", previousTZ, 1)
      } else {
        unsetenv("TZ")
      }
      NSTimeZone.resetSystemTimeZone()
      DayMathCache.shared.resetForTesting()
      MonthGridCache.shared.resetForTesting()
      CalendarStrings.dayLabelCache.resetForTesting()
      CalendarStrings.monthTitleCache.resetForTesting()
      DateRangePickerStrings.dayCache.resetForTesting()
    }

    let honolulu = TimeZone(identifier: "Pacific/Honolulu")!
    let tokyo = TimeZone(identifier: "Asia/Tokyo")!
    // 2026-10-16 20:00:00 UTC -> Oct 16 10:00 in Honolulu (UTC-10), Oct 17 05:00 in Tokyo (UTC+9).
    let instant = Date(timeIntervalSince1970: 1_792_180_800)

    // 1. Live system timezone changes via TZ + resetSystemTimeZone() (updates TimeZone.current).
    CFTimeZoneSetDefault(nil)
    setenv("TZ", "Pacific/Honolulu", 1)
    NSTimeZone.resetSystemTimeZone()
    XCTAssertEqual(TimeZone.current.identifier, honolulu.identifier)
    let sysHonolulu = DayMath.cached(weekStartsOn: 2)
    XCTAssertEqual(sysHonolulu.calendar.timeZone.identifier, honolulu.identifier)
    XCTAssertEqual(sysHonolulu.day(instant), 16)
    XCTAssertEqual(DateRangePickerStrings.day(instant), "Fri, Oct 16")
    XCTAssertEqual(DayMathCache.shared.totalBuilds, 1)

    setenv("TZ", "Asia/Tokyo", 1)
    NSTimeZone.resetSystemTimeZone()
    XCTAssertEqual(TimeZone.current.identifier, tokyo.identifier)
    let sysTokyo = DayMath.cached(weekStartsOn: 2)
    XCTAssertEqual(sysTokyo.calendar.timeZone.identifier, tokyo.identifier)
    XCTAssertEqual(sysTokyo.day(instant), 17)
    XCTAssertEqual(DateRangePickerStrings.day(instant), "Sat, Oct 17")
    XCTAssertEqual(DayMathCache.shared.totalBuilds, 2)

    // 2. Explicit NSTimeZone.default overrides stay consistent with Foundation.Calendar.
    DayMathCache.shared.resetForTesting()
    NSTimeZone.default = honolulu
    let honoluluCached = DayMath.cached(weekStartsOn: 2)
    let honoluluInit = DayMath(weekStartsOn: 2)
    XCTAssertEqual(honoluluCached.calendar.timeZone, honolulu)
    XCTAssertEqual(honoluluInit.calendar.timeZone, honolulu)
    XCTAssertEqual(honoluluCached.day(instant), 16)
    XCTAssertEqual(honoluluInit.day(instant), 16)
    XCTAssertEqual(DayMathCache.shared.totalBuilds, 1)

    for _ in 0..<10 {
      XCTAssertEqual(DayMath.cached(weekStartsOn: 2).calendar.timeZone, honolulu)
      XCTAssertEqual(DayMath(weekStartsOn: 2).calendar.timeZone, honolulu)
    }
    XCTAssertEqual(
      DayMathCache.shared.totalBuilds, 1,
      "Repeated lookups in the same timezone must reuse the cached DayMath"
    )

    NSTimeZone.default = tokyo
    let tokyoCached = DayMath.cached(weekStartsOn: 2)
    let tokyoInit = DayMath(weekStartsOn: 2)
    XCTAssertEqual(tokyoCached.calendar.timeZone, tokyo)
    XCTAssertEqual(tokyoInit.calendar.timeZone, tokyo)
    XCTAssertEqual(tokyoCached.day(instant), 17)
    XCTAssertEqual(tokyoInit.day(instant), 17)
    XCTAssertNotEqual(
      honoluluCached.startOfDay(instant),
      tokyoCached.startOfDay(instant)
    )
    XCTAssertEqual(DayMathCache.shared.totalBuilds, 2)

    for _ in 0..<10 {
      XCTAssertEqual(DayMath.cached(weekStartsOn: 2).calendar.timeZone, tokyo)
    }
    XCTAssertEqual(DayMathCache.shared.totalBuilds, 2)

    // Out-of-range firstWeekday still uses the live timezone safely and matches
    // stock Foundation.Calendar's weekday normalization.
    var expectedUncached = Foundation.Calendar(identifier: .gregorian)
    expectedUncached.firstWeekday = 8
    let uncached = DayMath(weekStartsOn: 8)
    XCTAssertEqual(uncached.calendar.timeZone, tokyo)
    XCTAssertEqual(uncached.calendar.firstWeekday, expectedUncached.firstWeekday)
    XCTAssertEqual(uncached.day(instant), 17)

    // LocalDayStringCache agrees with DayMath on the live timezone (dayLabel & monthTitle).
    // 2026-10-31 20:00:00 UTC -> Oct 31 in Honolulu, Nov 1 in Tokyo.
    let monthBoundary = Date(timeIntervalSince1970: 1_793_476_800)
    CalendarStrings.dayLabelCache.resetForTesting()
    CalendarStrings.monthTitleCache.resetForTesting()
    XCTAssertEqual(CalendarStrings.dayLabel(instant), "Saturday, October 17, 2026")
    XCTAssertEqual(CalendarStrings.monthTitle(monthBoundary), "November 2026")
    NSTimeZone.default = honolulu
    XCTAssertEqual(CalendarStrings.dayLabel(instant), "Friday, October 16, 2026")
    XCTAssertEqual(CalendarStrings.monthTitle(monthBoundary), "October 2026")

    // Month grid cache distinguishes timezones (including sub-minute fixed offsets
    // that share a rounded `.identifier` string) and stays normalized.
    let honoluluGrid = honoluluCached.cachedMonthGrid(instant)
    let tokyoGrid = tokyoCached.cachedMonthGrid(instant)
    XCTAssertEqual(honoluluGrid.days.count, 42)
    XCTAssertEqual(tokyoGrid.days.count, 42)
    XCTAssertNotEqual(honoluluGrid.monthStart, tokyoGrid.monthStart)

    let offsetA = TimeZone(secondsFromGMT: 3630)!
    let offsetB = TimeZone(secondsFromGMT: 3659)!
    let mathA = DayMathCache.shared.dayMath(weekStartsOn: 2, timeZone: offsetA)
    let mathB = DayMathCache.shared.dayMath(weekStartsOn: 2, timeZone: offsetB)
    XCTAssertEqual(mathA.calendar.timeZone.secondsFromGMT(), 3630)
    XCTAssertEqual(mathB.calendar.timeZone.secondsFromGMT(), 3659)
    XCTAssertNotEqual(
      mathA.cachedMonthGrid(instant).monthStart,
      mathB.cachedMonthGrid(instant).monthStart
    )
  }

  func testDayMathCacheIsThreadSafeAcrossConcurrentTimeZoneSwitches() {
    let cache = DayMathCache()
    let zones = [
      TimeZone(identifier: "Pacific/Honolulu")!,
      TimeZone(identifier: "UTC")!,
      TimeZone(identifier: "Asia/Tokyo")!,
      TimeZone(identifier: "America/New_York")!,
    ]
    let group = DispatchGroup()
    let queue = DispatchQueue(label: "dazzui.daymath.stress", attributes: .concurrent)
    let failures = NSLock()
    var mismatchCount = 0

    for worker in 0..<8 {
      group.enter()
      queue.async {
        defer { group.leave() }
        for iter in 0..<250 {
          let weekday = (worker + iter) % 7 + 1
          let tz = zones[(worker + iter / 10) % zones.count]
          let math = cache.dayMath(weekStartsOn: weekday, timeZone: tz)
          if math.calendar.firstWeekday != weekday || math.calendar.timeZone != tz {
            failures.lock()
            mismatchCount += 1
            failures.unlock()
          }
        }
      }
    }

    XCTAssertEqual(group.wait(timeout: .now() + 10), .success)
    XCTAssertEqual(
      mismatchCount, 0,
      "Concurrent DayMathCache lookups across timezone switches must never return a mismatched DayMath"
    )
  }
}
