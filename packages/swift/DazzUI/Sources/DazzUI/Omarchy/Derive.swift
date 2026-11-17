import Foundation
import SwiftUI

/// `ds/scripts/themes.mjs`'s `derive`, in Swift: every colour an imported
/// theme has, from the three its source decides everything else from.
///
/// The build derives Omarchy's shipped themes once, into the generated
/// `ThemeVariables.omarchy*` instances; this does the same at run time, for
/// whatever palette the machine is wearing now — a user's own theme as much
/// as a built-in one. It is a transcription, step for step, rather than a
/// second reading, and `OmarchyTests` holds it to the generated instances to
/// the digit. A change to the script is a change here.
///
/// Everything that is not a colour — Hyprland's square corner, the dark
/// shadows, the shades a recipe picks — is the same for every Omarchy theme
/// of one scheme, so it is taken from a generated Omarchy instance of that
/// scheme and only the derived colours are laid over it.
public func deriveOmarchyThemeVariables(
  background: Color,
  foreground: Color,
  accent: Color,
  dark: Bool
) -> ThemeVariables {
  let paper = rgb(background)
  let ink = parse(foreground, over: paper)
  let accentRgb = parse(accent, over: paper)
  let steps = dark ? ladderDark : ladderLight
  let anchor = dark ? 500 : 600
  let ramp = rampThrough(accentRgb, anchor: anchor, paper: paper, dark: dark)

  func off(_ dL: Double) -> RGB {
    stepOff(from: paper, to: ink, dL: dL) ?? ink
  }
  func under(_ dL: Double) -> RGB {
    stepOff(from: paper, to: RGB(r: 0, g: 0, b: 0), dL: dL) ?? off(dL)
  }

  let muted = off(steps.muted)
  let column = dark ? under(steps.column) : off(steps.column)
  let inset = off(steps.inset)
  let sunken = off(steps.sunken)
  let raised = dark ? off(steps.raised) : paper
  let canvas = dark ? under(steps.canvas) : off(steps.canvas)
  let overlay = dark ? muted : paper

  let washOver: [(RGB, Double)] = [
    (paper, dark ? 5.5 : 4.5),
    (muted, dark ? 5.0 : 4.0),
    (column, dark ? 5.0 : 4.0),
    (overlay, dark ? 5.0 : 4.5),
  ]
  let subtle = washOver
    .map { alphaFor(ink: ink, ground: $0.0, dL: $0.1) }
    .reduce(dark ? 0.07 : 0.06, max)

  let faint = mix(paper, ink, 0.36)
  let subtleInk = mix(paper, ink, 0.5)
  let mutedT = legible(paper: paper, ink: ink, t: 0.64, min: 4.6)
  let navT = max(legible(paper: paper, ink: ink, t: 0.72, min: 4.6), mutedT)
  let secondaryT = max(legible(paper: paper, ink: ink, t: 0.82, min: 4.6), navT)

  let separator = dark ? 0.16 : 0.13
  let onAccent = [paper, ink].first { contrast($0, accentRgb) >= 3 } ?? paper

  var shades: [Int: Color] = [:]
  shades.reserveCapacity(rampSteps.count)
  for step in rampSteps {
    shades[step] = color(ramp[step]!)
  }
  let swatch = ColorSwatch(shades)

  var vars = dark ? ThemeVariables.omarchyTokyoNightDark : .omarchyTokyoNightLight
  vars.colorPrimary = swatch
  vars.colorCanvas = color(canvas)
  vars.colorSurface = color(paper)
  vars.colorSurfaceMuted = color(muted)
  vars.colorSurfaceSunken = color(sunken)
  vars.colorSurfaceSubtle = color(ink, alpha: subtle)
  vars.colorSurfaceInset = color(inset)
  vars.colorSurfaceRaised = color(raised)
  vars.colorSurfaceOverlay = color(overlay)
  vars.colorSurfaceChrome = color(muted)
  vars.colorSurfaceColumn = color(column)
  vars.colorContent = color(ink)
  vars.colorContentSecondary = color(mix(paper, ink, secondaryT))
  vars.colorContentNav = color(mix(paper, ink, navT))
  vars.colorContentMuted = color(mix(paper, ink, mutedT))
  vars.colorContentSubtle = color(subtleInk)
  vars.colorContentFaint = color(faint)
  vars.colorBorder = color(ink, alpha: separator)
  vars.colorBorderStrong = color(ink, alpha: separator + 0.04)
  vars.colorBorderMuted = color(mix(paper, ink, 0.38))
  vars.colorOnAccent = color(onAccent)
  vars.progressGradientFrom = swatch[anchor]
  vars.progressGradientTo = swatch[300]
  return vars
}

/// `bothModes`' other half: the foreground becomes the paper, lifted towards
/// white or pressed towards black, the paper becomes the ink, and the accent
/// is pressed or lifted so a label printed in it still reads.
public func oppositeOmarchyColors(
  background: Color,
  foreground: Color,
  accent: Color,
  dark: Bool
) -> (background: Color, foreground: Color, accent: Color) {
  let white = Color(argb: 0xFFFF_FFFF)
  let black = Color(argb: 0xFF00_0000)
  return dark
    ? (
      background: mixHex(foreground, white, 0.78),
      foreground: background,
      accent: mixHex(accent, black, 0.38)
    )
    : (
      background: mixHex(foreground, black, 0.55),
      foreground: background,
      accent: mixHex(accent, white, 0.38)
    )
}

internal func omarchyLuminance(_ color: Color) -> Double {
  luminance(rgb(color))
}

private struct RGB {
  let r: Double
  let g: Double
  let b: Double
}

private let rampSteps = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950]

private struct Ladder {
  let muted: Double
  let column: Double
  let inset: Double
  let sunken: Double
  let raised: Double
  let canvas: Double
}

private let ladderLight = Ladder(
  muted: 2.5,
  column: 1.5,
  inset: 5.5,
  sunken: 10.5,
  raised: 0,
  canvas: 6
)

private let ladderDark = Ladder(
  muted: 3,
  column: 1.5,
  inset: 6,
  sunken: 10,
  raised: 20,
  canvas: 2
)

/// `a` mixed `t` of the way towards `b`, in sRGB bytes.
private func mixHex(_ a: Color, _ b: Color, _ t: Double) -> Color {
  let ab = a.srgbBytes
  let bb = b.srgbBytes
  func channel(_ from: UInt8, _ to: UInt8) -> UInt32 {
    let f = Double(from)
    return UInt32(clamping: omarchyRound(f + (Double(to) - f) * t))
  }
  return Color(
    argb: 0xFF00_0000
      | (channel(ab.r, bb.r) << 16)
      | (channel(ab.g, bb.g) << 8)
      | channel(ab.b, bb.b)
  )
}

/* The ramp ----------------------------------------------------------------- */

private let shape: [Int: (Double, Double)] = [
  50: (0.975, 0.12),
  100: (0.945, 0.25),
  200: (0.89, 0.45),
  300: (0.82, 0.7),
  400: (0.72, 0.9),
  500: (0.64, 1),
  600: (0.56, 1),
  700: (0.49, 0.92),
  800: (0.42, 0.82),
  900: (0.36, 0.7),
  950: (0.27, 0.55),
]

private func rampThrough(_ accent: RGB, anchor: Int, paper: RGB, dark: Bool) -> [Int: RGB] {
  let (L, C, h) = oklch(accent)
  let (aL, aC) = shape[anchor]!
  let light = shape[50]!.0
  let deep = shape[950]!.0

  func lightnessOf(_ step: Int) -> Double {
    let sL = shape[step]!.0
    return sL > aL
      ? L + ((max(light, L) - L) * (sL - aL)) / (light - aL)
      : L - ((L - min(deep, L)) * (aL - sL)) / (aL - deep)
  }

  var ramp: [Int: RGB] = [:]
  ramp.reserveCapacity(rampSteps.count)
  for step in rampSteps {
    ramp[step] =
      step == anchor
      ? accent
      : inGamut(lightnessOf(step), (C * shape[step]!.1) / aC, h)
  }
  for step in dark ? [300, 200] : [700, 800] {
    var sL = oklch(ramp[step]!).0
    let chroma = (C * shape[step]!.1) / aC
    while contrast(ramp[step]!, paper) < 4.5 && sL > 0.02 && sL < 0.99 {
      sL += dark ? 0.01 : -0.01
      ramp[step] = inGamut(sL, chroma, h)
    }
  }
  return ramp
}

private func inGamut(_ L: Double, _ C: Double, _ h: Double) -> RGB {
  var c = C
  for _ in 0..<40 {
    let candidate = fromOklch(L, c, h)
    if candidate.r >= -1e-4 && candidate.r <= 1 + 1e-4
      && candidate.g >= -1e-4 && candidate.g <= 1 + 1e-4
      && candidate.b >= -1e-4 && candidate.b <= 1 + 1e-4
    {
      return RGB(r: clamp01(candidate.r), g: clamp01(candidate.g), b: clamp01(candidate.b))
    }
    c *= 0.9
  }
  let grey = fromOklch(L, 0, h)
  return RGB(r: clamp01(grey.r), g: clamp01(grey.g), b: clamp01(grey.b))
}

/* The ladders -------------------------------------------------------------- */

private func stepOff(from: RGB, to: RGB, dL: Double) -> RGB? {
  let l0 = lightness(from)
  if abs(lightness(to) - l0) < dL { return nil }
  var lo = 0.0
  var hi = 1.0
  for _ in 0..<40 {
    let t = (lo + hi) / 2
    if abs(lightness(mix(from, to, t)) - l0) < dL {
      lo = t
    } else {
      hi = t
    }
  }
  return mix(from, to, hi)
}

private func alphaFor(ink: RGB, ground: RGB, dL: Double) -> Double {
  let l0 = lightness(ground)
  var lo = 0.0
  var hi = 1.0
  for _ in 0..<40 {
    let a = (lo + hi) / 2
    if abs(lightness(mix(ground, ink, a)) - l0) < dL {
      lo = a
    } else {
      hi = a
    }
  }
  return ceil(hi * 255) / 255
}

private func legible(paper: RGB, ink: RGB, t: Double, min minContrast: Double) -> Double {
  var u = t
  while u < 1 && contrast(mix(paper, ink, u), paper) < minContrast {
    u += 0.01
  }
  return min(u, 1)
}

/* Colour ------------------------------------------------------------------- */

private func rgb(_ c: Color) -> RGB {
  let bytes = c.srgbBytes
  return RGB(
    r: Double(bytes.r) / 255,
    g: Double(bytes.g) / 255,
    b: Double(bytes.b) / 255
  )
}

/// A colour; one with an alpha is composited over `ground` first.
private func parse(_ c: Color, over ground: RGB) -> RGB {
  let bytes = c.srgbBytes
  let alpha = Double(bytes.a) / 255
  let fg = RGB(r: Double(bytes.r) / 255, g: Double(bytes.g) / 255, b: Double(bytes.b) / 255)
  return RGB(
    r: fg.r * alpha + ground.r * (1 - alpha),
    g: fg.g * alpha + ground.g * (1 - alpha),
    b: fg.b * alpha + ground.b * (1 - alpha)
  )
}

/// Hex precision is what ships, so every value is rounded to it here, as the
/// script's `hex` does.
private func color(_ rgb: RGB, alpha: Double = 1) -> Color {
  func byte(_ v: Double) -> UInt32 {
    UInt32(clamping: omarchyRound(clamp01(v) * 255))
  }
  let a = alpha < 1 ? byte(alpha) : 255
  return Color(argb: (a << 24) | (byte(rgb.r) << 16) | (byte(rgb.g) << 8) | byte(rgb.b))
}

private func mix(_ a: RGB, _ b: RGB, _ t: Double) -> RGB {
  RGB(
    r: Double(omarchyRound((a.r + (b.r - a.r) * t) * 255)) / 255,
    g: Double(omarchyRound((a.g + (b.g - a.g) * t) * 255)) / 255,
    b: Double(omarchyRound((a.b + (b.b - a.b) * t) * 255)) / 255
  )
}

/// JavaScript's `Math.round`: halves go up, never away from zero.
internal func omarchyRound(_ v: Double) -> Int {
  Int(floor(v + 0.5))
}

private func clamp01(_ v: Double) -> Double {
  min(1, max(0, v))
}

private func cbrt(_ v: Double) -> Double {
  v < 0 ? -pow(-v, 1.0 / 3.0) : pow(v, 1.0 / 3.0)
}

private func linear(_ v: Double) -> Double {
  v <= 0.04045 ? v / 12.92 : pow((v + 0.055) / 1.055, 2.4)
}

private func gamma(_ v: Double) -> Double {
  v <= 0.0031308
    ? 12.92 * v
    : 1.055 * pow(v, 1.0 / 2.4) - 0.055
}

private func luminance(_ rgb: RGB) -> Double {
  0.2126 * linear(rgb.r) + 0.7152 * linear(rgb.g) + 0.0722 * linear(rgb.b)
}

private func lightness(_ rgb: RGB) -> Double {
  let y = luminance(rgb)
  return y > 0.008856 ? 116 * cbrt(y) - 16 : 903.3 * y
}

private func contrast(_ a: RGB, _ b: RGB) -> Double {
  let la = luminance(a)
  let lb = luminance(b)
  return (max(la, lb) + 0.05) / (min(la, lb) + 0.05)
}

private func oklch(_ rgb: RGB) -> (Double, Double, Double) {
  let r = linear(rgb.r)
  let g = linear(rgb.g)
  let b = linear(rgb.b)
  let l = cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b)
  let m = cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b)
  let s = cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b)
  let L = 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s
  let A = 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s
  let B = 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s
  return (L, sqrt(A * A + B * B), atan2(B, A))
}

private func fromOklch(_ L: Double, _ C: Double, _ h: Double) -> RGB {
  let A = C * cos(h)
  let B = C * sin(h)
  func cube(_ v: Double) -> Double { v * v * v }
  let l = cube(L + 0.3963377774 * A + 0.2158037573 * B)
  let m = cube(L - 0.1055613458 * A - 0.0638541728 * B)
  let s = cube(L - 0.0894841775 * A - 1.291485548 * B)
  return RGB(
    r: gamma(4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s),
    g: gamma(-1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s),
    b: gamma(-0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s)
  )
}
