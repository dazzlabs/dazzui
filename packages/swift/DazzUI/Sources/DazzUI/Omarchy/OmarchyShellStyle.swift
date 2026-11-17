import Foundation
import SwiftUI

/// The sizes Omarchy's shell sets its text and lays out its controls at, as a
/// machine's `shell.toml` decides them.
///
/// The shell (`shell/Commons/Style.qml`) derives every size from two roots.
/// Type comes from `[font] base-size`, 12 by default: each step is
/// `round(base ×` its multiplier`)`, and any step can be pinned in px under
/// its own key. Lengths — margins, gaps, padding, control heights, panel
/// dimensions — are the px they are at the default, times `[spacing] scale`,
/// and times `base-size / 12` too while `scale-with-font` is on, as it is by
/// default; `control-height` and `control-padding-x` can be pinned in px.
///
/// The generated `omarchy-*` themes carry all of it at the default, from the
/// `omarchy` metrics in `ds/themes/omarchy.yaml`; this follows a machine that
/// moved it. The shell's type steps land on DazzUI's the way that file states:
/// `caption` on caption, `body-small` on small, `body` on body, `subtitle` on
/// emphasis, `title` on title, and a control's label at `body`, which is what
/// the shell sets a button in.
public struct OmarchyShellStyle: Sendable, Hashable, CustomStringConvertible {
  /// `[font] base-size`, the root every type step is a multiple of.
  public let fontBaseSize: Int

  /// Type steps pinned in px, by the shell's key (`caption`, `body-small`, …).
  public let fontOverrides: [String: Int]

  /// `[spacing] scale`.
  public let spacingScale: Double

  /// `[spacing] scale-with-font`: whether lengths grow with the type too.
  public let spacingScaleWithFont: Bool

  /// Lengths pinned in px, by the shell's key (`control-height`, …).
  public let spacingOverrides: [String: Double]

  public init(
    fontBaseSize: Int = 12,
    fontOverrides: [String: Int] = [:],
    spacingScale: Double = 1,
    spacingScaleWithFont: Bool = true,
    spacingOverrides: [String: Double] = [:]
  ) {
    self.fontBaseSize = fontBaseSize
    self.fontOverrides = fontOverrides
    self.spacingScale = spacingScale
    self.spacingScaleWithFont = spacingScaleWithFont
    self.spacingOverrides = spacingOverrides
  }

  /// The style out of the `shell.toml` files the shell reads, in the order it
  /// reads them — the theme's, then the user's — a later file's key winning
  /// over an earlier one's, as the shell's does.
  public static func fromShellToml<S: Sequence>(
    _ sources: S
  ) -> OmarchyShellStyle where S.Element == String {
    var values: [String: String] = [:]
    for source in sources {
      for (key, value) in readOmarchyToml(source) {
        values[key] = value
      }
    }

    var fontBaseSize = 12
    var fontOverrides: [String: Int] = [:]
    var spacingScale = 1.0
    var spacingScaleWithFont = true
    var spacingOverrides: [String: Double] = [:]

    for (key, value) in values {
      if key.hasPrefix("font.") {
        guard let n = parseLeadingInt(value) else { continue }
        let step = String(key.dropFirst("font.".count))
        if step == "base-size" {
          fontBaseSize = n
        } else if n > 0 {
          fontOverrides[step] = n
        }
      } else if key.hasPrefix("spacing.") {
        let token = String(key.dropFirst("spacing.".count))
        if token == "scale-with-font" {
          if let parsed = parseBoolToken(value) {
            spacingScaleWithFont = parsed
          }
          continue
        }
        guard let n = Double(value.trimmingCharacters(in: .whitespacesAndNewlines)),
          n.isFinite
        else { continue }
        if token == "scale" {
          spacingScale = n
        } else if n >= 0 {
          spacingOverrides[token] = n
        }
      }
    }

    return OmarchyShellStyle(
      fontBaseSize: max(1, fontBaseSize),
      fontOverrides: fontOverrides,
      spacingScale: spacingScale < 0 ? 1 : spacingScale,
      spacingScaleWithFont: spacingScaleWithFont,
      spacingOverrides: spacingOverrides
    )
  }

  /// What every length is multiplied by, as the shell's
  /// `effectiveSpacingScale` is.
  public var lengthScale: Double {
    spacingScale * (spacingScaleWithFont ? Double(fontBaseSize) / 12 : 1)
  }

  private func step(_ key: String, _ multiplier: Double) -> Int {
    fontOverrides[key] ?? max1(omarchyRound(Double(fontBaseSize) * multiplier))
  }

  public var caption: Int { step("caption", 0.833) }
  public var bodySmall: Int { step("body-small", 0.917) }
  public var body: Int { step("body", 1.0) }
  public var subtitle: Int { step("subtitle", 1.083) }
  public var title: Int { step("title", 1.167) }

  /// `vars` — an Omarchy theme, at the shell's default sizes — at this
  /// machine's.
  ///
  /// Every face set at one of the type steps moves to it, keeping its family
  /// and weight; the headlines are not the chrome's and are left alone. A face
  /// that never wraps keeps its leading at its size, and those at the small
  /// step are a control's label.
  ///
  /// Every length is scaled as the shell's `space(px)` scales one, and with
  /// the spacing ladder go the paddings, gaps and boxes DazzUI derives from
  /// it. Corners, strokes and the focus ring are not lengths the shell scales,
  /// and stay.
  public func apply(to vars: ThemeVariables) -> ThemeVariables {
    let caption = CGFloat(self.caption)
    let small = CGFloat(bodySmall)
    let body = CGFloat(self.body)
    let emphasis = CGFloat(subtitle)
    let title = CGFloat(self.title)
    let label = body
    let scale = lengthScale

    func space(_ px: CGFloat) -> CGFloat {
      px <= 0 ? 0 : CGFloat(max1(omarchyRound(Double(px) * scale)))
    }
    func token(_ key: String, _ px: CGFloat) -> CGFloat {
      if let pinned = spacingOverrides[key] {
        return CGFloat(omarchyRound(pinned))
      }
      return space(px)
    }

    var copy = vars
    copy.titleSmall = vars.titleSmall.sized(body, lineHeight: leading(body))
    copy.titleMedium = vars.titleMedium.sized(emphasis, lineHeight: leading(emphasis))
    copy.titleLarge = vars.titleLarge.sized(title, lineHeight: leading(title))
    copy.bodySmall = vars.bodySmall.sized(small, lineHeight: leading(small))
    copy.bodyMedium = vars.bodyMedium.sized(body, lineHeight: leading(body))
    copy.bodyLarge = vars.bodyLarge.sized(emphasis, lineHeight: leading(emphasis))
    copy.labelQuiet = vars.labelQuiet.sized(label, lineHeight: label)
    copy.labelStrong = vars.labelStrong.sized(label, lineHeight: label)
    copy.labelSmall = vars.labelSmall.sized(caption, lineHeight: caption)
    copy.labelMedium = vars.labelMedium.sized(label, lineHeight: label)
    copy.labelLarge = vars.labelLarge.sized(body, lineHeight: body)
    copy.captionSmall = vars.captionSmall.sized(caption, lineHeight: leading(caption))
    copy.captionMedium = vars.captionMedium.sized(small, lineHeight: leading(small))
    copy.captionLarge = vars.captionLarge.sized(body, lineHeight: leading(body))

    copy.spacingPx = space(vars.spacingPx)
    copy.spacing05 = space(vars.spacing05)
    copy.spacing1 = space(vars.spacing1)
    copy.spacing15 = space(vars.spacing15)
    copy.spacing2 = space(vars.spacing2)
    copy.spacing25 = space(vars.spacing25)
    copy.spacing3 = space(vars.spacing3)
    copy.spacing35 = space(vars.spacing35)
    copy.spacing4 = space(vars.spacing4)
    copy.spacing5 = space(vars.spacing5)
    copy.spacing6 = space(vars.spacing6)
    copy.spacing7 = space(vars.spacing7)
    copy.spacing8 = space(vars.spacing8)
    copy.spacing9 = space(vars.spacing9)
    copy.spacing10 = space(vars.spacing10)
    copy.spacing11 = space(vars.spacing11)
    copy.spacing12 = space(vars.spacing12)
    copy.spacing14 = space(vars.spacing14)
    copy.spacing16 = space(vars.spacing16)
    copy.spacing20 = space(vars.spacing20)

    copy.iconSmall = space(vars.iconSmall)
    copy.iconMedium = space(vars.iconMedium)
    copy.iconLarge = space(vars.iconLarge)

    copy.controlTinySize = space(vars.controlTinySize)
    copy.controlSmallSize = space(vars.controlSmallSize)
    copy.controlMediumSize = token("control-height", vars.controlMediumSize)
    copy.controlLargeSize = space(vars.controlLargeSize)
    copy.controlMediumPaddingInline = token("control-padding-x", vars.controlMediumPaddingInline)

    copy.frameTitlebarSize = space(vars.frameTitlebarSize)
    copy.frameSidebarWidth = space(vars.frameSidebarWidth)
    copy.frameSidebarIconWidth = space(vars.frameSidebarIconWidth)
    copy.frameRailWidth = space(vars.frameRailWidth)
    copy.frameAsideWidth = space(vars.frameAsideWidth)
    copy.frameNavGap = space(vars.frameNavGap)

    copy.dialogWidth = space(vars.dialogWidth)
    copy.drawerSize = space(vars.drawerSize)
    copy.menuMinWidth = space(vars.menuMinWidth)
    copy.menuItemPadding = space(vars.menuItemPadding)
    copy.preferencesWidth = space(vars.preferencesWidth)
    copy.previewCardWidth = space(vars.previewCardWidth)
    copy.segmentedControlInset = space(vars.segmentedControlInset)
    copy.shortcutRecorderWidth = space(vars.shortcutRecorderWidth)
    copy.switchMediumWidth = space(vars.switchMediumWidth)
    copy.switchMediumHeight = space(vars.switchMediumHeight)
    copy.switchMediumThumb = space(vars.switchMediumThumb)
    copy.toastMaxWidth = space(vars.toastMaxWidth)
    return copy
  }

  public var description: String {
    "OmarchyShellStyle(font: \(fontBaseSize) \(fontOverrides), "
      + "spacing: ×\(spacingScale)\(spacingScaleWithFont ? " with font" : "") "
      + "\(spacingOverrides))"
  }
}

/// The shell leaves leading to Qt; DazzUI's is a third over the size, rounded
/// — the same rule the scale in `omarchy.yaml` is written by.
private func leading(_ size: CGFloat) -> CGFloat {
  CGFloat(omarchyRound(Double(size) * 4 / 3))
}

private func max1(_ v: Int) -> Int {
  v < 1 ? 1 : v
}

/// QML's `parseInt`: the leading integer, so `12.5` reads as 12.
private func parseLeadingInt(_ value: String) -> Int? {
  let trimmed = value.drop { $0.isWhitespace }
  guard !trimmed.isEmpty else { return nil }
  var cursor = trimmed.startIndex
  if trimmed[cursor] == "+" || trimmed[cursor] == "-" {
    cursor = trimmed.index(after: cursor)
  }
  let digitsStart = cursor
  while cursor < trimmed.endIndex, trimmed[cursor].isASCII, trimmed[cursor].isNumber {
    cursor = trimmed.index(after: cursor)
  }
  guard cursor > digitsStart else { return nil }
  return Int(trimmed[..<cursor])
}

/// The shell's `boolToken`.
private func parseBoolToken(_ value: String) -> Bool? {
  switch value.trimmingCharacters(in: .whitespacesAndNewlines).lowercased() {
  case "true", "1", "yes", "on":
    return true
  case "false", "0", "no", "off":
    return false
  default:
    return nil
  }
}
