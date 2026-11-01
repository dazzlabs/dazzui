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
  public let shades: [Int: Color]

  public init(_ shades: [Int: Color]) {
    self.shades = shades
  }

  /// The step at `shade`. A ramp is generated with every step present, so a
  /// missing one is a generator bug rather than a case to design around —
  /// it resolves to clear instead of trapping.
  public subscript(shade: Int) -> Color {
    shades[shade] ?? .clear
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

  public init(family: [String], size: CGFloat, weight: Font.Weight, lineHeight: CGFloat) {
    self.family = family
    self.size = size
    self.weight = weight
    self.lineHeight = lineHeight
  }

  internal func scaled(by scale: CGFloat) -> TypeStyle {
    TypeStyle(family: family, size: size * scale, weight: weight, lineHeight: lineHeight * scale)
  }

  /// The font this face resolves to.
  ///
  /// Every face in the graph leads its stack with `-apple-system`, which on
  /// this platform is a request for the system face rather than a family that
  /// can be looked up by name — so the stack resolves to `.system` and only a
  /// face naming a real family falls through to `.custom`.
  public var font: Font {
    guard let first = family.first, !first.hasPrefix("-apple-system") else {
      return .system(size: size, weight: weight)
    }
    return .custom(first, size: size)
  }

  /// How tall the platform sets one line of this face: its ascender, its
  /// descender and its own leading — what a `Text` is before any spacing.
  public var naturalLineHeight: CGFloat {
    #if os(macOS)
      let face = family.first.flatMap { name in
        name.hasPrefix("-apple-system") ? nil : NSFont(name: name, size: size)
      } ?? NSFont.systemFont(ofSize: size)
      // A `Text` sets its line on whole points above and below the baseline:
      // a 13pt system face is 12.6 up and 2.7 down, and a line of it is 16.
      return face.ascender.rounded(.up) + (-face.descender).rounded(.up) + face.leading
    #elseif canImport(UIKit)
      let face = family.first.flatMap { name in
        name.hasPrefix("-apple-system") ? nil : UIFont(name: name, size: size)
      } ?? UIFont.systemFont(ofSize: size)
      return face.ascender.rounded(.up) + (-face.descender).rounded(.up) + face.leading
    #else
      return size
    #endif
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

public extension View {
  /// Draws a token shadow.
  ///
  /// One `.shadow` per layer, and every layer given counts: a list that
  /// *shrinks* between states takes modifiers away, and a view whose
  /// structure changes is replaced rather than updated. A replaced view
  /// cannot crossfade the colours drawn inside it — it is snapshotted and
  /// faded, which is what macOS 15 draws as a button's fill being wiped away
  /// from one edge. A control that has no shadow in this state therefore
  /// keeps its layer and draws ``TokenShadow/none`` in it.
  func tokenShadow(_ shadows: [TokenShadow]) -> some View {
    shadows.reduce(AnyView(self)) { view, shadow in
      AnyView(
        view.shadow(color: shadow.color, radius: shadow.radius, x: shadow.x, y: shadow.y)
      )
    }
  }
}
