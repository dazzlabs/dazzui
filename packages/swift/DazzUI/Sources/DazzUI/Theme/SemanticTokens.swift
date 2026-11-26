import Foundation
import SwiftUI
#if os(macOS)
  import AppKit
#elseif canImport(UIKit)
  import UIKit
#endif

// MARK: - Color math

public extension Color {
  /// Extracts normalized sRGB RGBA components in `0...1`.
  internal var srgbComponents: (r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) {
    #if os(Windows) || os(Linux) || os(WASI)
      return (CGFloat(red), CGFloat(green), CGFloat(blue), CGFloat(opacity))
    #elseif os(macOS)
      guard let c = NSColor(self).usingColorSpace(.sRGB) else { return (0, 0, 0, 0) }
      return (c.redComponent, c.greenComponent, c.blueComponent, c.alphaComponent)
    #elseif canImport(UIKit)
      var r: CGFloat = 0
      var g: CGFloat = 0
      var b: CGFloat = 0
      var a: CGFloat = 0
      UIColor(self).getRed(&r, green: &g, blue: &b, alpha: &a)
      return (r, g, b, a)
    #else
      let bytes = srgbBytes
      return (
        CGFloat(bytes.r) / 255,
        CGFloat(bytes.g) / 255,
        CGFloat(bytes.b) / 255,
        CGFloat(bytes.a) / 255
      )
    #endif
  }

  /// Source-over compositing of `self` onto `background` in sRGB.
  ///
  /// When `alpha` is provided, it replaces the foreground opacity before
  /// compositing — useful for baking a translucent wash onto a theme surface
  /// so surfaces look identical over raised panels and windows alike.
  func blended(over background: Color, alpha: CGFloat? = nil) -> Color {
    #if os(macOS) || canImport(UIKit)
      let key = ColorMathCacheKey(
        op: .blend,
        a: self,
        b: background,
        param: alpha.map { UInt64($0.bitPattern) } ?? UInt64.max
      )
      return ColorMathCache.shared.resolve(key) {
        self.computeBlended(over: background, alpha: alpha)
      }
    #else
      return computeBlended(over: background, alpha: alpha)
    #endif
  }

  /// Channel-wise sRGB linear interpolation from `self` to `other`, alpha included.
  func lerp(to other: Color, t: CGFloat) -> Color {
    #if os(macOS) || canImport(UIKit)
      let key = ColorMathCacheKey(
        op: .lerp,
        a: self,
        b: other,
        param: UInt64(t.bitPattern)
      )
      return ColorMathCache.shared.resolve(key) {
        self.computeLerp(to: other, t: t)
      }
    #else
      return computeLerp(to: other, t: t)
    #endif
  }

  private func computeBlended(over background: Color, alpha: CGFloat?) -> Color {
    let fRaw = srgbComponents
    let fAlpha = min(1, max(0, alpha ?? fRaw.a))
    guard fAlpha > 0 else { return background }
    let b = background.srgbComponents
    let inv = 1 - fAlpha
    if b.a >= 1 {
      return Self.srgbColor(
        r: fRaw.r * fAlpha + b.r * inv,
        g: fRaw.g * fAlpha + b.g * inv,
        b: fRaw.b * fAlpha + b.b * inv,
        a: 1
      )
    }
    let back = b.a * inv
    let outAlpha = fAlpha + back
    guard outAlpha > 0 else { return .clear }
    return Self.srgbColor(
      r: (fRaw.r * fAlpha + b.r * back) / outAlpha,
      g: (fRaw.g * fAlpha + b.g * back) / outAlpha,
      b: (fRaw.b * fAlpha + b.b * back) / outAlpha,
      a: outAlpha
    )
  }

  private func computeLerp(to other: Color, t: CGFloat) -> Color {
    let clamped = min(1, max(0, t))
    let a = srgbComponents
    let b = other.srgbComponents
    return Self.srgbColor(
      r: a.r + (b.r - a.r) * clamped,
      g: a.g + (b.g - a.g) * clamped,
      b: a.b + (b.b - a.b) * clamped,
      a: a.a + (b.a - a.a) * clamped
    )
  }

  private static func srgbColor(r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> Color {
    #if os(Windows) || os(Linux) || os(WASI)
      Color(red: Double(r), green: Double(g), blue: Double(b), opacity: Double(a))
    #else
      Color(.sRGB, red: Double(r), green: Double(g), blue: Double(b), opacity: Double(a))
    #endif
  }
}

#if os(macOS) || canImport(UIKit)
  private struct ColorMathCacheKey: Hashable, Sendable {
    enum Op: UInt8, Hashable, Sendable {
      case blend
      case lerp
    }

    let op: Op
    let a: Color
    let b: Color
    let param: UInt64
  }

  internal final class ColorMathCache: @unchecked Sendable {
    static let shared = ColorMathCache()
    private static let limit = 1024

    private let lock = NSLock()
    private var entries: [ColorMathCacheKey: Color] = [:]
    private var computeCount = 0

    fileprivate func resolve(_ key: ColorMathCacheKey, compute: () -> Color) -> Color {
      lock.lock()
      if let hit = entries[key] {
        lock.unlock()
        return hit
      }
      lock.unlock()

      let computed = compute()

      lock.lock()
      defer { lock.unlock() }
      if let raced = entries[key] {
        return raced
      }
      if entries.count >= Self.limit {
        entries.removeAll(keepingCapacity: true)
      }
      entries[key] = computed
      computeCount += 1
      return computed
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
#endif

// MARK: - Derived semantic colors

public extension ThemeVariables {
  /// Primary accent fill — the resting shade a filled primary control paints with.
  var accent: Color {
    colorPrimary[controlColorFilledSurfaceNormalShade]
  }

  /// Primary accent fill under pointer hover.
  var accentHover: Color {
    colorPrimary[controlColorFilledSurfaceHoveredShade]
  }

  /// Primary accent text — readable on the theme's surface.
  var accentText: Color {
    colorPrimary[controlColorPlainContentNormalShade]
  }

  /// Emphasized primary accent text (hover / active).
  var accentTextStrong: Color {
    colorPrimary[controlColorPlainContentHoveredShade]
  }

  /// Opaque accent-tinted surface resolved over `colorSurface`.
  var accentSurface: Color {
    colorPrimary[600].blended(over: colorSurface, alpha: washSurface)
  }

  /// Hairline edge pairing with `accentSurface`.
  var accentHairline: Color {
    colorPrimary[600].opacity(washEdge)
  }

  /// Danger fill color.
  var danger: Color {
    colorDanger[controlColorFilledSurfaceNormalShade]
  }

  /// Danger text color.
  var dangerText: Color {
    colorDanger[controlColorPlainContentNormalShade]
  }

  /// Opaque danger-tinted surface resolved over `colorSurface`.
  var dangerSurface: Color {
    colorDanger[600].blended(over: colorSurface, alpha: washSurface)
  }

  /// Hairline edge pairing with `dangerSurface`.
  var dangerHairline: Color {
    colorDanger[600].opacity(washEdge)
  }

  /// Warning fill color.
  var warn: Color {
    colorWarning[controlColorFilledSurfaceNormalShade]
  }

  /// Warning text color.
  var warnText: Color {
    colorWarning[controlColorPlainContentNormalShade]
  }

  /// Opaque warning-tinted surface resolved over `colorSurface`.
  var warnSurface: Color {
    colorWarning[600].blended(over: colorSurface, alpha: washSurface)
  }

  /// Hairline edge pairing with `warnSurface`.
  var warnHairline: Color {
    colorWarning[600].opacity(washEdge)
  }

  /// Success fill color.
  var success: Color {
    colorSuccess[controlColorFilledSurfaceNormalShade]
  }

  /// Success text color.
  var successText: Color {
    colorSuccess[controlColorPlainContentNormalShade]
  }

  /// Opaque success-tinted surface resolved over `colorSurface`.
  var successSurface: Color {
    colorSuccess[600].blended(over: colorSurface, alpha: washSurface)
  }

  /// Hairline edge pairing with `successSurface`.
  var successHairline: Color {
    colorSuccess[600].opacity(washEdge)
  }

  /// Second-column navigation rail surface: halfway between `colorSurfaceColumn`
  /// and `colorSurface`, reading as one step closer to the content pane.
  var railSurface: Color {
    colorSurfaceColumn.lerp(to: colorSurface, t: 0.5)
  }

  /// Emphasized selection highlight for active list rows and selected blocks.
  var selection: Color {
    controlColor(.tinted, .surface, tint: .primary, state: .normal)
  }

  /// Unemphasized selection fill when the enclosing window or pane is inactive.
  var selectionUnemphasized: Color {
    controlColor(.recessed, .surface, tint: .neutral, state: .normal)
  }
}

// MARK: - Typography ergonomics

public extension TypeStyle {
  /// Creates a custom UI `TypeStyle` using the platform's system font stack
  /// when a layout needs a size outside the semantic type scale.
  static func system(
    size: CGFloat,
    weight: Font.Weight = .regular,
    lineHeight: CGFloat? = nil
  ) -> TypeStyle {
    TypeStyle(
      family: ["-apple-system"],
      size: size,
      weight: weight,
      lineHeight: lineHeight ?? size
    )
  }

  /// Creates a monospaced code `TypeStyle`.
  static func code(
    size: CGFloat,
    weight: Font.Weight = .regular,
    lineHeight: CGFloat? = nil
  ) -> TypeStyle {
    TypeStyle(
      family: ["ui-monospace"],
      size: size,
      weight: weight,
      lineHeight: lineHeight ?? size
    )
  }
}

public extension ThemeVariables {
  /// Monospaced code font resolved from the semantic `fontCode` token.
  func codeFont(size: CGFloat? = nil, weight: Font.Weight = .regular) -> Font {
    codeStyle(size: size, weight: weight).font
  }

  /// Monospaced code `TypeStyle` built from the semantic `fontCode` token.
  func codeStyle(
    size: CGFloat? = nil,
    weight: Font.Weight = .regular,
    lineHeight: CGFloat? = nil
  ) -> TypeStyle {
    let resolvedSize = size ?? captionSmall.size
    return TypeStyle(
      family: fontCode,
      size: resolvedSize,
      weight: weight,
      lineHeight: lineHeight ?? resolvedSize * 1.4
    )
  }
}

public extension View {
  /// Applies a `TypeStyle`'s font, cached line spacing, optional ink, optional
  /// weight override, and tracking in a single modifier.
  func typeStyle(
    _ style: TypeStyle,
    color: Color? = nil,
    weight: Font.Weight? = nil,
    tracking: CGFloat = 0
  ) -> some View {
    modifier(
      TypeStyleModifier(
        style: style,
        color: color,
        weight: weight,
        tracking: tracking,
        monospacedDigit: false
      )
    )
  }

  /// Applies a `TypeStyle` with tabular (monospaced) digits so aligned numeric
  /// columns do not jitter as values change.
  func numericTypeStyle(
    _ style: TypeStyle,
    color: Color? = nil,
    weight: Font.Weight? = nil,
    tracking: CGFloat = 0
  ) -> some View {
    modifier(
      TypeStyleModifier(
        style: style,
        color: color,
        weight: weight,
        tracking: tracking,
        monospacedDigit: true
      )
    )
  }

  /// Applies the theme's monospaced code font with optional size, weight, and color.
  func codeStyle(
    size: CGFloat? = nil,
    weight: Font.Weight = .regular,
    color: Color? = nil
  ) -> some View {
    modifier(CodeStyleModifier(size: size, weight: weight, color: color))
  }
}

private struct TypeStyleModifier: ViewModifier {
  let style: TypeStyle
  let color: Color?
  let weight: Font.Weight?
  let tracking: CGFloat
  let monospacedDigit: Bool

  @ViewBuilder
  func body(content: Content) -> some View {
    let base = content
      .font(style.resolvedFont(weight: weight, monospacedDigit: monospacedDigit))
      .tracking(tracking)
      .lineSpacing(style.lineSpacing)
    if let color {
      base.foregroundStyle(color)
    } else {
      base
    }
  }
}

private struct CodeStyleModifier: ViewModifier {
  let size: CGFloat?
  let weight: Font.Weight
  let color: Color?

  @Environment(\.theme) private var theme

  @ViewBuilder
  func body(content: Content) -> some View {
    let base = content.font(theme.vars.codeFont(size: size, weight: weight))
    if let color {
      base.foregroundStyle(color)
    } else {
      base
    }
  }
}
