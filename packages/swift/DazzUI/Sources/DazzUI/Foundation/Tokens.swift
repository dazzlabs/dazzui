import Foundation
import SwiftUI
#if os(macOS)
  import AppKit
#elseif canImport(UIKit)
  import UIKit
#endif

// The hand-written half of the token vocabulary: the types `Generated/` is
// written against. Nothing here holds a design value — every value lives in
// `ds` and arrives through the generator.

/// A colour ramp — a role ladder indexed by shade.
///
/// Reading it as a lightness ramp is the mistake it invites: Bright's ink
/// ladder puts a *lighter* colour at 700 than at 600, because its primary
/// action is ink navy whose hover lifts. A step only ever means the role the
/// recipe reaching for it gives it.
public struct ColorSwatch: Sendable, Equatable {
  private final class Storage: Sendable {
    let shades: [Int: Color]
    let s50: Color?
    let s100: Color?
    let s200: Color?
    let s300: Color?
    let s400: Color?
    let s500: Color?
    let s600: Color?
    let s700: Color?
    let s800: Color?
    let s900: Color?
    let s950: Color?

    init(_ shades: [Int: Color]) {
      self.shades = shades
      self.s50 = shades[50]
      self.s100 = shades[100]
      self.s200 = shades[200]
      self.s300 = shades[300]
      self.s400 = shades[400]
      self.s500 = shades[500]
      self.s600 = shades[600]
      self.s700 = shades[700]
      self.s800 = shades[800]
      self.s900 = shades[900]
      self.s950 = shades[950]
    }
  }

  private let storage: Storage

  public var shades: [Int: Color] { storage.shades }

  public init(_ shades: [Int: Color]) {
    self.storage = Storage(shades)
  }

  /// The step at `shade`. A ramp is generated with every step present, so a
  /// missing one is a generator bug rather than a case to design around —
  /// it resolves to clear instead of trapping.
  public subscript(shade: Int) -> Color {
    switch shade {
    case 50: return storage.s50 ?? .clear
    case 100: return storage.s100 ?? .clear
    case 200: return storage.s200 ?? .clear
    case 300: return storage.s300 ?? .clear
    case 400: return storage.s400 ?? .clear
    case 500: return storage.s500 ?? .clear
    case 600: return storage.s600 ?? .clear
    case 700: return storage.s700 ?? .clear
    case 800: return storage.s800 ?? .clear
    case 900: return storage.s900 ?? .clear
    case 950: return storage.s950 ?? .clear
    default: return storage.shades[shade] ?? .clear
    }
  }

  public static func == (lhs: ColorSwatch, rhs: ColorSwatch) -> Bool {
    lhs.storage === rhs.storage || lhs.storage.shades == rhs.storage.shades
  }
}

/// An interactive colour, stored as a *recipe* rather than a finished colour.
///
/// A control does not know its own hue until it is tinted, so each state
/// carries either a literal colour (a semantic role, the same in every tint)
/// or a shade to pick off whichever ramp the control is tinted with, plus an
/// alpha to apply. `ControlColor.resolve` is where the two are combined.
public struct ColorDescriptor: Sendable, Equatable {
  public var normalColor: Color?
  public var normalShade: Int?
  public var normalOpacity: CGFloat?

  public var hoveredColor: Color?
  public var hoveredShade: Int?
  public var hoveredOpacity: CGFloat?

  public var pressedColor: Color?
  public var pressedShade: Int?
  public var pressedOpacity: CGFloat?

  public var disabledColor: Color?
  public var disabledShade: Int?
  public var disabledOpacity: CGFloat?

  public init(
    normalColor: Color? = nil,
    normalShade: Int? = nil,
    normalOpacity: CGFloat? = nil,
    hoveredColor: Color? = nil,
    hoveredShade: Int? = nil,
    hoveredOpacity: CGFloat? = nil,
    pressedColor: Color? = nil,
    pressedShade: Int? = nil,
    pressedOpacity: CGFloat? = nil,
    disabledColor: Color? = nil,
    disabledShade: Int? = nil,
    disabledOpacity: CGFloat? = nil
  ) {
    self.normalColor = normalColor
    self.normalShade = normalShade
    self.normalOpacity = normalOpacity
    self.hoveredColor = hoveredColor
    self.hoveredShade = hoveredShade
    self.hoveredOpacity = hoveredOpacity
    self.pressedColor = pressedColor
    self.pressedShade = pressedShade
    self.pressedOpacity = pressedOpacity
    self.disabledColor = disabledColor
    self.disabledShade = disabledShade
    self.disabledOpacity = disabledOpacity
  }

  /// The recipe for one interaction state.
  public func recipe(for state: ControlState) -> (Color?, Int?, CGFloat?) {
    switch state {
    case .normal: return (normalColor, normalShade, normalOpacity)
    case .hovered: return (hoveredColor, hoveredShade, hoveredOpacity)
    case .pressed: return (pressedColor, pressedShade, pressedOpacity)
    case .disabled: return (disabledColor, disabledShade, disabledOpacity)
    }
  }
}

/// A type face: a family stack, a size, a weight, and an absolute leading.
///
/// The leading is kept absolute rather than as a multiple because that is how
/// the design states it — a label face sets its line-height to its own size
/// so its text sits on the centre of a 26pt box, and a multiple would round
/// that away.
public struct TypeStyle: Sendable, Equatable {
  public let family: [String]
  public let size: CGFloat
  public let weight: Font.Weight
  public let lineHeight: CGFloat
  /// The font this face resolves to, precomputed once at initialisation so
  /// `.font(style.font)` avoids repeating family inspection and `Font`
  /// construction on every SwiftUI body evaluation.
  ///
  /// Every face in the graph leads its stack with `-apple-system` (or
  /// `ui-monospace` for code), which on this platform is a request for the
  /// system face rather than a family that can be looked up by name — so the
  /// stack resolves to `.system` and only a face naming a real family falls
  /// through to `.custom`.
  public let font: Font

  public init(family: [String], size: CGFloat, weight: Font.Weight, lineHeight: CGFloat) {
    self.family = family
    self.size = size
    self.weight = weight
    self.lineHeight = lineHeight
    self.font = Self.makeFont(family: family, size: size, weight: weight, overrideWeight: nil)
  }

  public static func == (lhs: TypeStyle, rhs: TypeStyle) -> Bool {
    lhs.size == rhs.size && lhs.weight == rhs.weight && lhs.lineHeight == rhs.lineHeight
      && lhs.family == rhs.family
  }

  internal func scaled(by scale: CGFloat) -> TypeStyle {
    TypeStyle(family: family, size: size * scale, weight: weight, lineHeight: lineHeight * scale)
  }

  internal func sized(_ size: CGFloat, lineHeight: CGFloat) -> TypeStyle {
    TypeStyle(family: family, size: size, weight: weight, lineHeight: lineHeight)
  }

  /// Resolves the font with an optional weight override and optional tabular digits.
  public func resolvedFont(weight overrideWeight: Font.Weight? = nil, monospacedDigit: Bool = false) -> Font {
    let base: Font
    if overrideWeight == nil {
      base = font
    } else {
      base = Self.makeFont(family: family, size: size, weight: weight, overrideWeight: overrideWeight)
    }
    #if os(macOS) || os(iOS) || os(tvOS) || os(watchOS)
      return monospacedDigit ? base.monospacedDigit() : base
    #else
      return base
    #endif
  }

  private static func makeFont(
    family: [String],
    size: CGFloat,
    weight: Font.Weight,
    overrideWeight: Font.Weight?
  ) -> Font {
    let effectiveWeight = overrideWeight ?? weight
    if let first = family.first {
      if first.hasPrefix("-apple-system") {
        return .system(size: size, weight: effectiveWeight)
      } else if first == "ui-monospace" || first == "monospace" {
        return .system(size: size, weight: effectiveWeight, design: .monospaced)
      } else if let overrideWeight {
        return .custom(first, size: size).weight(overrideWeight)
      } else {
        return .custom(first, size: size)
      }
    } else {
      return .system(size: size, weight: effectiveWeight)
    }
  }

  /// How tall the platform sets one line of this face: its ascender, its
  /// descender and its own leading — what a `Text` is before any spacing.
  ///
  /// Cached by `(family, size)` behind a lock so SwiftUI redraws do not query
  /// CoreText/AppKit font metrics on every `Text` evaluation.
  public var naturalLineHeight: CGFloat {
    cachedMetrics.naturalLineHeight
  }

  /// Where CSS sets the baseline in this face's line box: the face's ascent
  /// below half of what the box has over its ascent and descent.
  internal var cssBaseline: CGFloat {
    let metrics = cachedMetrics
    return (lineHeight - metrics.ascent - metrics.descent) / 2 + metrics.ascent
  }

  private var cachedMetrics: CachedTypeMetrics {
    let familyKey = family.first ?? "-apple-system"
    let sizeKey = Int((size * 100).rounded())
    let key = TypeMetricsCacheKey(family: familyKey, size: sizeKey)
    return TypeMetricsCache.shared.metrics(for: key) {
      #if os(macOS)
        let face: NSFont
        if familyKey.hasPrefix("-apple-system") {
          face = NSFont.systemFont(ofSize: size)
        } else if familyKey == "ui-monospace" || familyKey == "monospace" {
          face = NSFont.monospacedSystemFont(ofSize: size, weight: .regular)
        } else {
          face = NSFont(name: familyKey, size: size) ?? NSFont.systemFont(ofSize: size)
        }
        // A `Text` sets its line on whole points above and below the baseline:
        // a 13pt system face is 12.6 up and 2.7 down, and a line of it is 16.
        let ascent = face.ascender
        let descent = -face.descender
        let natural = ascent.rounded(.up) + descent.rounded(.up) + face.leading
        return CachedTypeMetrics(naturalLineHeight: natural, ascent: ascent, descent: descent)
      #elseif canImport(UIKit)
        let face: UIFont
        if familyKey.hasPrefix("-apple-system") {
          face = UIFont.systemFont(ofSize: size)
        } else if familyKey == "ui-monospace" || familyKey == "monospace" {
          face = UIFont.monospacedSystemFont(ofSize: size, weight: .regular)
        } else {
          face = UIFont(name: familyKey, size: size) ?? UIFont.systemFont(ofSize: size)
        }
        let ascent = face.ascender
        let descent = -face.descender
        let natural = ascent.rounded(.up) + descent.rounded(.up) + face.leading
        return CachedTypeMetrics(naturalLineHeight: natural, ascent: ascent, descent: descent)
      #else
        return CachedTypeMetrics(naturalLineHeight: size, ascent: size * 0.8, descent: size * 0.2)
      #endif
    }
  }

  /// SwiftUI spaces lines by the gap between them, where CSS states the whole
  /// line box. The gap is what the line box has over the face's own line —
  /// not over its size, which would set every line the face's leading apart
  /// on top of the design's.
  public var lineSpacing: CGFloat {
    lineHeight - naturalLineHeight
  }

  /// CSS's half-leading: what a line box puts above and below its line.
  public var halfLeading: CGFloat {
    lineSpacing / 2
  }
}

internal struct TypeMetricsCacheKey: Hashable, Sendable {
  let family: String
  let size: Int
}

internal struct CachedTypeMetrics: Sendable {
  let naturalLineHeight: CGFloat
  let ascent: CGFloat
  let descent: CGFloat
}

internal final class TypeMetricsCache: @unchecked Sendable {
  static let shared = TypeMetricsCache()
  private static let limit = 512

  private let lock = NSLock()
  private var entries: [TypeMetricsCacheKey: CachedTypeMetrics] = [:]
  private var computeCount = 0

  func metrics(for key: TypeMetricsCacheKey, compute: () -> CachedTypeMetrics) -> CachedTypeMetrics {
    lock.lock()
    if let hit = entries[key] {
      lock.unlock()
      return hit
    }
    lock.unlock()

    let measured = compute()

    lock.lock()
    defer { lock.unlock() }
    if let raced = entries[key] {
      return raced
    }
    if entries.count >= Self.limit {
      entries.removeAll(keepingCapacity: true)
    }
    entries[key] = measured
    computeCount += 1
    return measured
  }

  internal var cachedEntryCount: Int {
    lock.lock()
    defer { lock.unlock() }
    return entries.count
  }

  internal var totalComputations: Int {
    lock.lock()
    defer { lock.unlock() }
    return computeCount
  }

  internal func resetForTesting() {
    lock.lock()
    entries.removeAll()
    computeCount = 0
    lock.unlock()
  }
}

public extension View {
  /// Sets text in `style`'s line box, as CSS does: lines `lineHeight` apart,
  /// and half the leading above the first and below the last, so one line is
  /// `lineHeight` tall rather than the face's own height. For a run of text;
  /// on a row of controls the padding would land on the row.
  func lineBox(_ style: TypeStyle, lineHeight: CGFloat? = nil) -> some View {
    let leading = (lineHeight ?? style.lineHeight) - style.naturalLineHeight
    return lineSpacing(leading)
      .padding(.vertical, leading / 2)
  }
}

private struct StretchedControlKey: EnvironmentKey {
  static let defaultValue = false
}

public extension EnvironmentValues {
  /// Whether an enclosing column stretches `width: auto` controls across its
  /// cross axis, the way CSS's `align-items: stretch` does.
  var dazzStretchedControl: Bool {
    get { self[StretchedControlKey.self] }
    set { self[StretchedControlKey.self] = newValue }
  }
}

/// One shadow layer.
///
/// Named against the token rather than `SwiftUI.ShadowStyle`, which is a
/// different thing: a `ShadowStyle` is a fill effect, and this is the layer a
/// `shadow` token describes, blur diameter and all.
public struct TokenShadow: Sendable, Equatable {
  public let x: CGFloat
  public let y: CGFloat
  public let blur: CGFloat
  public let spread: CGFloat
  public let color: Color

  public init(x: CGFloat, y: CGFloat, blur: CGFloat, spread: CGFloat, color: Color) {
    self.x = x
    self.y = y
    self.blur = blur
    self.spread = spread
    self.color = color
  }

  /// CSS quotes a blur *diameter*; SwiftUI's modifier wants the radius.
  public var radius: CGFloat { blur / 2 }

  /// A layer that paints nothing, so a control can drop its lift without
  /// dropping the layer the lift is drawn on. See ``SwiftUI/View/tokenShadow(_:)``.
  public static let none = TokenShadow(x: 0, y: 0, blur: 0, spread: 0, color: .clear)
}

/// A CSS `cubic-bezier(…)`, kept as data so the motion token can name one.
public struct BezierCurve: Sendable, Equatable {
  public let x1: CGFloat
  public let y1: CGFloat
  public let x2: CGFloat
  public let y2: CGFloat

  public init(_ x1: CGFloat, _ y1: CGFloat, _ x2: CGFloat, _ y2: CGFloat) {
    self.x1 = x1
    self.y1 = y1
    self.x2 = x2
    self.y2 = y2
  }

  public func animation(duration: Double) -> Animation {
    .timingCurve(x1, y1, x2, y2, duration: duration)
  }
}

public extension Color {
  /// `0xAARRGGBB`, which is how the generator writes a colour.
  init(argb: UInt32) {
    self.init(
      .sRGB,
      red: Double((argb >> 16) & 0xff) / 255,
      green: Double((argb >> 8) & 0xff) / 255,
      blue: Double(argb & 0xff) / 255,
      opacity: Double((argb >> 24) & 0xff) / 255
    )
  }
}

internal extension Color {
  var srgbBytes: (a: UInt8, r: UInt8, g: UInt8, b: UInt8) {
    func byte(_ v: Double) -> UInt8 {
      let clamped = min(1, max(0, v))
      return UInt8(clamping: Int(floor(clamped * 255 + 0.5)))
    }
    #if os(Windows) || os(Linux) || os(WASI)
      return (byte(opacity), byte(red), byte(green), byte(blue))
    #elseif os(macOS)
      guard let c = NSColor(self).usingColorSpace(.sRGB) else { return (255, 0, 0, 0) }
      return (
        byte(Double(c.alphaComponent)),
        byte(Double(c.redComponent)),
        byte(Double(c.greenComponent)),
        byte(Double(c.blueComponent))
      )
    #elseif canImport(UIKit)
      var r: CGFloat = 0
      var g: CGFloat = 0
      var b: CGFloat = 0
      var a: CGFloat = 0
      UIColor(self).getRed(&r, green: &g, blue: &b, alpha: &a)
      return (byte(Double(a)), byte(Double(r)), byte(Double(g)), byte(Double(b)))
    #else
      return (255, 0, 0, 0)
    #endif
  }
}

public extension View {
  /// Draws a token shadow.
  ///
  /// One `.shadow` per layer without erasing the view to `AnyView` for the
  /// common 0–3 layer cases. Every layer given counts: a list that *shrinks*
  /// between states takes modifiers away, and a view whose structure changes is
  /// replaced rather than updated. A replaced view cannot crossfade the colours
  /// drawn inside it — it is snapshotted and faded, which is what macOS 15
  /// draws as a button's fill being wiped away from one edge. A control that
  /// has no shadow in this state therefore keeps its layer and draws
  /// ``TokenShadow/none`` in it.
  func tokenShadow(_ shadows: [TokenShadow]) -> some View {
    modifier(TokenShadowModifier(shadows: shadows))
  }
}

private struct TokenShadowModifier: ViewModifier {
  let shadows: [TokenShadow]

  @ViewBuilder
  func body(content: Content) -> some View {
    switch shadows.count {
    case 0:
      content
    case 1:
      let s0 = shadows[0]
      content.shadow(color: s0.color, radius: s0.radius, x: s0.x, y: s0.y)
    case 2:
      let s0 = shadows[0]
      let s1 = shadows[1]
      content
        .shadow(color: s0.color, radius: s0.radius, x: s0.x, y: s0.y)
        .shadow(color: s1.color, radius: s1.radius, x: s1.x, y: s1.y)
    case 3:
      let s0 = shadows[0]
      let s1 = shadows[1]
      let s2 = shadows[2]
      content
        .shadow(color: s0.color, radius: s0.radius, x: s0.x, y: s0.y)
        .shadow(color: s1.color, radius: s1.radius, x: s1.x, y: s1.y)
        .shadow(color: s2.color, radius: s2.radius, x: s2.x, y: s2.y)
    default:
      shadows.reduce(AnyView(content)) { view, shadow in
        AnyView(
          view.shadow(color: shadow.color, radius: shadow.radius, x: shadow.x, y: shadow.y)
        )
      }
    }
  }
}
