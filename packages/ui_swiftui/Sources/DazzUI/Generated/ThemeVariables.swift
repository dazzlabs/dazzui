import SwiftUI

/// The complete token set for one theme.
///
/// DO NOT EDIT - This file is auto-generated
public struct ThemeVariables: Sendable {
  public init(
    colorPrimary: ColorSwatch = Colors.brand,
    colorNeutral: ColorSwatch = Colors.neutral,
    colorInfo: ColorSwatch = Colors.sky,
    colorSuccess: ColorSwatch = Colors.green,
    colorWarning: ColorSwatch = Colors.amber,
    colorDanger: ColorSwatch = Colors.red,
    colorCanvas: Color = Color(argb: 0xFFEE_ECF6),
    colorSurface: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color = Color(argb: 0xFFF7_F7FA),
    colorSurfaceSunken: Color = Color(argb: 0xFFE3_E3EC),
    colorSurfaceSubtle: Color = Color(argb: 0xFFF5_F5F9),
    colorSurfaceInset: Color = Color(argb: 0xFFF0_F0F5),
    colorSurfaceRaised: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color = Color(argb: 0xFFF7_F7FA),
    colorContent: Color = Color(argb: 0xFF12_142A),
    colorContentSecondary: Color = Color(argb: 0xFF3C_405C),
    colorContentNav: Color = Color(argb: 0xFF4A_4F6B),
    colorContentMuted: Color = Color(argb: 0xFF56_5B78),
    colorContentSubtle: Color = Color(argb: 0xFF8C_92AA),
    colorContentFaint: Color = Color(argb: 0xFFA2_A7BD),
    colorBorder: Color = Color(argb: 0x1214_162A),
    colorBorderStrong: Color = Color(argb: 0x1A14_162A),
    colorBorderMuted: Color = Color(argb: 0xFFCF_D2DE),
    colorOnAccent: Color = Color(argb: 0xFFFF_FFFF),
    controlPressedAlpha: CGFloat = 0.9,
    controlTinySize: CGFloat = 24.0,
    controlSmallSize: CGFloat = 26.0,
    controlMediumSize: CGFloat = 28.0,
    controlLargeSize: CGFloat = 32.0,
    shadow2xs: [TokenShadow] = [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow] = [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow] = [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow] = [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow] = [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow] = [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow] = [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    focusWidth: CGFloat = 3.0,
    focusOffset: CGFloat = 0.0,
    focusGlowShade: Int = 600,
    focusGlowAlpha: CGFloat = 0.14,
    focusRingShade: Int = 600,
    focusRingAlpha: CGFloat = 0.45,
    frameWindowRadius: CGFloat = 18.0,
    framePopoverRadius: CGFloat = 16.0,
    frameTitlebarSize: CGFloat = 52.0,
    frameSidebarWidth: CGFloat = 172.0,
    frameSidebarIconWidth: CGFloat = 48.0,
    frameRailWidth: CGFloat = 150.0,
    frameAsideWidth: CGFloat = 214.0,
    frameNavGap: CGFloat = 3.0,
    motionDuration: Double = 0.15,
    motionEasing: BezierCurve = BezierCurve(0.4, 0.0, 0.2, 1.0),
    radiusNone: CGFloat = 0.0,
    radiusTiny: CGFloat = 7.0,
    radiusSmall: CGFloat = 8.0,
    radiusMedium: CGFloat = 10.0,
    radiusLarge: CGFloat = 12.0,
    radiusBig: CGFloat = 16.0,
    radiusFull: CGFloat = 9999.0,
    spacingPx: CGFloat = 1.0,
    spacing0: CGFloat = 0.0,
    spacing05: CGFloat = 2.0,
    spacing1: CGFloat = 4.0,
    spacing15: CGFloat = 6.0,
    spacing2: CGFloat = 8.0,
    spacing25: CGFloat = 10.0,
    spacing3: CGFloat = 12.0,
    spacing35: CGFloat = 14.0,
    spacing4: CGFloat = 16.0,
    spacing5: CGFloat = 20.0,
    spacing6: CGFloat = 24.0,
    spacing7: CGFloat = 28.0,
    spacing8: CGFloat = 32.0,
    spacing9: CGFloat = 36.0,
    spacing10: CGFloat = 40.0,
    spacing11: CGFloat = 44.0,
    spacing12: CGFloat = 48.0,
    spacing14: CGFloat = 56.0,
    spacing16: CGFloat = 64.0,
    spacing20: CGFloat = 80.0,
    strokeHairline: CGFloat = 1.0,
    strokeControl: CGFloat = 1.5,
    headlineSmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 26.0),
    headlineMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: 24.0, weight: .semibold, lineHeight: 30.0),
    headlineLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: 28.0, weight: .semibold, lineHeight: 34.0),
    titleSmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 15.0, weight: .semibold, lineHeight: 22.0),
    titleLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 17.0, weight: .semibold, lineHeight: 22.0),
    bodySmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 18.0),
    bodyMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 15.0, weight: .regular, lineHeight: 22.0),
    labelQuiet: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 11.0, weight: .semibold, lineHeight: 11.0),
    labelMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    captionSmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    captionMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 18.0),
    captionLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 20.0),
    washSurface: CGFloat = 0.06,
    washEdge: CGFloat = 0.2,
    checkboxRadius: CGFloat = 5.0,
    dialogWidth: CGFloat = 440.0,
    dialogScrimAlpha: CGFloat = 0.25,
    menuMinWidth: CGFloat = 176.0,
    menuItemPadding: CGFloat = 7.0,
    preferencesWidth: CGFloat = 480.0,
    progressGradientFrom: Color = Colors.brand[600],
    progressGradientTo: Color = Color(argb: 0xFFA0_8CFF),
    segmentedControlInset: CGFloat = 3.0,
    shortcutRecorderWidth: CGFloat = 132.0,
    switchMediumWidth: CGFloat = 32.0,
    switchMediumHeight: CGFloat = 18.0,
    switchMediumThumb: CGFloat = 14.0,
    toastMaxWidth: CGFloat = 420.0,
    controlColorRecessedBorder: ColorDescriptor = ColorDescriptor(
    normalColor: Colors.transparent,
    hoveredColor: Colors.transparent,
    pressedColor: Colors.transparent,
    disabledColor: Colors.transparent
  ),
    controlColorFilledSurfaceNormalShade: Int = 600,
    controlColorFilledSurfaceHoveredShade: Int = 700,
    controlColorFilledSurfacePressedShade: Int = 700,
    controlColorFilledBorder: ColorDescriptor = ColorDescriptor(
    normalColor: Colors.transparent,
    hoveredColor: Colors.transparent,
    pressedColor: Colors.transparent,
    disabledColor: Colors.transparent
  ),
    controlColorTintedContentNormalShade: Int = 700,
    controlColorTintedContentHoveredShade: Int = 800,
    controlColorTintedContentPressedShade: Int = 800,
    controlColorTintedBorder: ColorDescriptor = ColorDescriptor(
    normalColor: Colors.transparent,
    hoveredColor: Colors.transparent,
    pressedColor: Colors.transparent,
    disabledColor: Colors.transparent
  ),
    controlColorOutlinedContentNormalShade: Int = 700,
    controlColorOutlinedContentHoveredShade: Int = 800,
    controlColorOutlinedContentPressedShade: Int = 800,
    controlColorOutlinedBorder: ColorDescriptor = ColorDescriptor(
    normalShade: 600,
    normalOpacity: 1.0,
    hoveredShade: 700,
    hoveredOpacity: 1.0,
    pressedShade: 700,
    pressedOpacity: 0.9,
    disabledColor: Colors.transparent
  ),
    controlColorPlainSurface: ColorDescriptor = ColorDescriptor(
    normalColor: Colors.transparent,
    hoveredShade: 600,
    hoveredOpacity: 0.08,
    pressedShade: 600,
    pressedOpacity: 0.12,
    disabledColor: Colors.transparent
  ),
    controlColorPlainContentNormalShade: Int = 700,
    controlColorPlainContentHoveredShade: Int = 800,
    controlColorPlainContentPressedShade: Int = 800,
    controlColorPlainBorder: ColorDescriptor = ColorDescriptor(
    normalColor: Colors.transparent,
    hoveredColor: Colors.transparent,
    pressedColor: Colors.transparent,
    disabledColor: Colors.transparent
  )
  ) {
    self.colorPrimary = colorPrimary
    self.colorNeutral = colorNeutral
    self.colorInfo = colorInfo
    self.colorSuccess = colorSuccess
    self.colorWarning = colorWarning
    self.colorDanger = colorDanger
    self.colorCanvas = colorCanvas
    self.colorSurface = colorSurface
    self.colorSurfaceMuted = colorSurfaceMuted
    self.colorSurfaceSunken = colorSurfaceSunken
    self.colorSurfaceSubtle = colorSurfaceSubtle
    self.colorSurfaceInset = colorSurfaceInset
    self.colorSurfaceRaised = colorSurfaceRaised
    self.colorSurfaceChrome = colorSurfaceChrome
    self.colorContent = colorContent
    self.colorContentSecondary = colorContentSecondary
    self.colorContentNav = colorContentNav
    self.colorContentMuted = colorContentMuted
    self.colorContentSubtle = colorContentSubtle
    self.colorContentFaint = colorContentFaint
    self.colorBorder = colorBorder
    self.colorBorderStrong = colorBorderStrong
    self.colorBorderMuted = colorBorderMuted
    self.colorOnAccent = colorOnAccent
    self.controlPressedAlpha = controlPressedAlpha
    self.controlTinySize = controlTinySize
    self.controlSmallSize = controlSmallSize
    self.controlMediumSize = controlMediumSize
    self.controlLargeSize = controlLargeSize
    self.shadow2xs = shadow2xs
    self.shadowXs = shadowXs
    self.shadowSm = shadowSm
    self.shadowMd = shadowMd
    self.shadowLg = shadowLg
    self.shadowXl = shadowXl
    self.shadow2xl = shadow2xl
    self.focusWidth = focusWidth
    self.focusOffset = focusOffset
    self.focusGlowShade = focusGlowShade
    self.focusGlowAlpha = focusGlowAlpha
    self.focusRingShade = focusRingShade
    self.focusRingAlpha = focusRingAlpha
    self.frameWindowRadius = frameWindowRadius
    self.framePopoverRadius = framePopoverRadius
    self.frameTitlebarSize = frameTitlebarSize
    self.frameSidebarWidth = frameSidebarWidth
    self.frameSidebarIconWidth = frameSidebarIconWidth
    self.frameRailWidth = frameRailWidth
    self.frameAsideWidth = frameAsideWidth
    self.frameNavGap = frameNavGap
    self.motionDuration = motionDuration
    self.motionEasing = motionEasing
    self.radiusNone = radiusNone
    self.radiusTiny = radiusTiny
    self.radiusSmall = radiusSmall
    self.radiusMedium = radiusMedium
    self.radiusLarge = radiusLarge
    self.radiusBig = radiusBig
    self.radiusFull = radiusFull
    self.spacingPx = spacingPx
    self.spacing0 = spacing0
    self.spacing05 = spacing05
    self.spacing1 = spacing1
    self.spacing15 = spacing15
    self.spacing2 = spacing2
    self.spacing25 = spacing25
    self.spacing3 = spacing3
    self.spacing35 = spacing35
    self.spacing4 = spacing4
    self.spacing5 = spacing5
    self.spacing6 = spacing6
    self.spacing7 = spacing7
    self.spacing8 = spacing8
    self.spacing9 = spacing9
    self.spacing10 = spacing10
    self.spacing11 = spacing11
    self.spacing12 = spacing12
    self.spacing14 = spacing14
    self.spacing16 = spacing16
    self.spacing20 = spacing20
    self.strokeHairline = strokeHairline
    self.strokeControl = strokeControl
    self.headlineSmall = headlineSmall
    self.headlineMedium = headlineMedium
    self.headlineLarge = headlineLarge
    self.titleSmall = titleSmall
    self.titleMedium = titleMedium
    self.titleLarge = titleLarge
    self.bodySmall = bodySmall
    self.bodyMedium = bodyMedium
    self.bodyLarge = bodyLarge
    self.labelQuiet = labelQuiet
    self.labelStrong = labelStrong
    self.labelSmall = labelSmall
    self.labelMedium = labelMedium
    self.labelLarge = labelLarge
    self.captionSmall = captionSmall
    self.captionMedium = captionMedium
    self.captionLarge = captionLarge
    self.washSurface = washSurface
    self.washEdge = washEdge
    self.checkboxRadius = checkboxRadius
    self.dialogWidth = dialogWidth
    self.dialogScrimAlpha = dialogScrimAlpha
    self.menuMinWidth = menuMinWidth
    self.menuItemPadding = menuItemPadding
    self.preferencesWidth = preferencesWidth
    self.progressGradientFrom = progressGradientFrom
    self.progressGradientTo = progressGradientTo
    self.segmentedControlInset = segmentedControlInset
    self.shortcutRecorderWidth = shortcutRecorderWidth
    self.switchMediumWidth = switchMediumWidth
    self.switchMediumHeight = switchMediumHeight
    self.switchMediumThumb = switchMediumThumb
    self.toastMaxWidth = toastMaxWidth
    self.controlColorRecessedBorder = controlColorRecessedBorder
    self.controlColorFilledSurfaceNormalShade = controlColorFilledSurfaceNormalShade
    self.controlColorFilledSurfaceHoveredShade = controlColorFilledSurfaceHoveredShade
    self.controlColorFilledSurfacePressedShade = controlColorFilledSurfacePressedShade
    self.controlColorFilledBorder = controlColorFilledBorder
    self.controlColorTintedContentNormalShade = controlColorTintedContentNormalShade
    self.controlColorTintedContentHoveredShade = controlColorTintedContentHoveredShade
    self.controlColorTintedContentPressedShade = controlColorTintedContentPressedShade
    self.controlColorTintedBorder = controlColorTintedBorder
    self.controlColorOutlinedContentNormalShade = controlColorOutlinedContentNormalShade
    self.controlColorOutlinedContentHoveredShade = controlColorOutlinedContentHoveredShade
    self.controlColorOutlinedContentPressedShade = controlColorOutlinedContentPressedShade
    self.controlColorOutlinedBorder = controlColorOutlinedBorder
    self.controlColorPlainSurface = controlColorPlainSurface
    self.controlColorPlainContentNormalShade = controlColorPlainContentNormalShade
    self.controlColorPlainContentHoveredShade = controlColorPlainContentHoveredShade
    self.controlColorPlainContentPressedShade = controlColorPlainContentPressedShade
    self.controlColorPlainBorder = controlColorPlainBorder
  }

  public let colorPrimary: ColorSwatch
  public let colorNeutral: ColorSwatch
  public let colorInfo: ColorSwatch
  public let colorSuccess: ColorSwatch
  public let colorWarning: ColorSwatch
  public let colorDanger: ColorSwatch
  public let colorCanvas: Color
  public let colorSurface: Color
  public let colorSurfaceMuted: Color
  public let colorSurfaceSunken: Color
  public let colorSurfaceSubtle: Color
  public let colorSurfaceInset: Color
  public let colorSurfaceRaised: Color
  public let colorSurfaceChrome: Color
  public let colorContent: Color
  public let colorContentSecondary: Color
  public let colorContentNav: Color
  public let colorContentMuted: Color
  public let colorContentSubtle: Color
  public let colorContentFaint: Color
  public let colorBorder: Color
  public let colorBorderStrong: Color
  public let colorBorderMuted: Color
  public let colorOnAccent: Color
  public let controlPressedAlpha: CGFloat
  public let controlTinySize: CGFloat
  public let controlSmallSize: CGFloat
  public let controlMediumSize: CGFloat
  public let controlLargeSize: CGFloat
  public let shadow2xs: [TokenShadow]
  public let shadowXs: [TokenShadow]
  public let shadowSm: [TokenShadow]
  public let shadowMd: [TokenShadow]
  public let shadowLg: [TokenShadow]
  public let shadowXl: [TokenShadow]
  public let shadow2xl: [TokenShadow]
  public let focusWidth: CGFloat
  public let focusOffset: CGFloat
  public let focusGlowShade: Int
  public let focusGlowAlpha: CGFloat
  public let focusRingShade: Int
  public let focusRingAlpha: CGFloat
  public let frameWindowRadius: CGFloat
  public let framePopoverRadius: CGFloat
  public let frameTitlebarSize: CGFloat
  public let frameSidebarWidth: CGFloat
  public let frameSidebarIconWidth: CGFloat
  public let frameRailWidth: CGFloat
  public let frameAsideWidth: CGFloat
  public let frameNavGap: CGFloat
  public let motionDuration: Double
  public let motionEasing: BezierCurve
  public let radiusNone: CGFloat
  public let radiusTiny: CGFloat
  public let radiusSmall: CGFloat
  public let radiusMedium: CGFloat
  public let radiusLarge: CGFloat
  public let radiusBig: CGFloat
  public let radiusFull: CGFloat
  public let spacingPx: CGFloat
  public let spacing0: CGFloat
  public let spacing05: CGFloat
  public let spacing1: CGFloat
  public let spacing15: CGFloat
  public let spacing2: CGFloat
  public let spacing25: CGFloat
  public let spacing3: CGFloat
  public let spacing35: CGFloat
  public let spacing4: CGFloat
  public let spacing5: CGFloat
  public let spacing6: CGFloat
  public let spacing7: CGFloat
  public let spacing8: CGFloat
  public let spacing9: CGFloat
  public let spacing10: CGFloat
  public let spacing11: CGFloat
  public let spacing12: CGFloat
  public let spacing14: CGFloat
  public let spacing16: CGFloat
  public let spacing20: CGFloat
  public let strokeHairline: CGFloat
  public let strokeControl: CGFloat
  public let headlineSmall: TypeStyle
  public let headlineMedium: TypeStyle
  public let headlineLarge: TypeStyle
  public let titleSmall: TypeStyle
  public let titleMedium: TypeStyle
  public let titleLarge: TypeStyle
  public let bodySmall: TypeStyle
  public let bodyMedium: TypeStyle
  public let bodyLarge: TypeStyle
  public let labelQuiet: TypeStyle
  public let labelStrong: TypeStyle
  public let labelSmall: TypeStyle
  public let labelMedium: TypeStyle
  public let labelLarge: TypeStyle
  public let captionSmall: TypeStyle
  public let captionMedium: TypeStyle
  public let captionLarge: TypeStyle
  public let washSurface: CGFloat
  public let washEdge: CGFloat
  public let checkboxRadius: CGFloat
  public let dialogWidth: CGFloat
  public let dialogScrimAlpha: CGFloat
  public let menuMinWidth: CGFloat
  public let menuItemPadding: CGFloat
  public let preferencesWidth: CGFloat
  public let progressGradientFrom: Color
  public let progressGradientTo: Color
  public let segmentedControlInset: CGFloat
  public let shortcutRecorderWidth: CGFloat
  public let switchMediumWidth: CGFloat
  public let switchMediumHeight: CGFloat
  public let switchMediumThumb: CGFloat
  public let toastMaxWidth: CGFloat
  public let controlColorRecessedBorder: ColorDescriptor
  /// The ramp step `controlColorFilledSurface` picks when normal; a theme re-points it.
  public let controlColorFilledSurfaceNormalShade: Int
  /// The ramp step `controlColorFilledSurface` picks when hovered; a theme re-points it.
  public let controlColorFilledSurfaceHoveredShade: Int
  /// The ramp step `controlColorFilledSurface` picks when pressed; a theme re-points it.
  public let controlColorFilledSurfacePressedShade: Int
  public let controlColorFilledBorder: ColorDescriptor
  /// The ramp step `controlColorTintedContent` picks when normal; a theme re-points it.
  public let controlColorTintedContentNormalShade: Int
  /// The ramp step `controlColorTintedContent` picks when hovered; a theme re-points it.
  public let controlColorTintedContentHoveredShade: Int
  /// The ramp step `controlColorTintedContent` picks when pressed; a theme re-points it.
  public let controlColorTintedContentPressedShade: Int
  public let controlColorTintedBorder: ColorDescriptor
  /// The ramp step `controlColorOutlinedContent` picks when normal; a theme re-points it.
  public let controlColorOutlinedContentNormalShade: Int
  /// The ramp step `controlColorOutlinedContent` picks when hovered; a theme re-points it.
  public let controlColorOutlinedContentHoveredShade: Int
  /// The ramp step `controlColorOutlinedContent` picks when pressed; a theme re-points it.
  public let controlColorOutlinedContentPressedShade: Int
  public let controlColorOutlinedBorder: ColorDescriptor
  public let controlColorPlainSurface: ColorDescriptor
  /// The ramp step `controlColorPlainContent` picks when normal; a theme re-points it.
  public let controlColorPlainContentNormalShade: Int
  /// The ramp step `controlColorPlainContent` picks when hovered; a theme re-points it.
  public let controlColorPlainContentHoveredShade: Int
  /// The ramp step `controlColorPlainContent` picks when pressed; a theme re-points it.
  public let controlColorPlainContentPressedShade: Int
  public let controlColorPlainBorder: ColorDescriptor

  public var controlTinyGap: CGFloat { spacing1 }
  public var controlTinyPaddingBlock: CGFloat { spacing1 }
  public var controlTinyPaddingInline: CGFloat { spacing25 }
  public var controlTinyRadius: CGFloat { radiusSmall }
  public var controlTinyContent: TypeStyle { labelSmall }
  public var controlSmallGap: CGFloat { spacing15 }
  public var controlSmallPaddingBlock: CGFloat { spacing1 }
  public var controlSmallPaddingInline: CGFloat { spacing3 }
  public var controlSmallRadius: CGFloat { radiusSmall }
  public var controlSmallContent: TypeStyle { labelMedium }
  public var controlMediumGap: CGFloat { spacing2 }
  public var controlMediumPaddingBlock: CGFloat { spacing15 }
  public var controlMediumPaddingInline: CGFloat { spacing4 }
  public var controlMediumRadius: CGFloat { radiusSmall }
  public var controlMediumContent: TypeStyle { labelMedium }
  public var controlLargeGap: CGFloat { spacing2 }
  public var controlLargePaddingBlock: CGFloat { spacing15 }
  public var controlLargePaddingInline: CGFloat { spacing4 }
  public var controlLargeRadius: CGFloat { radiusMedium }
  public var controlLargeContent: TypeStyle { labelMedium }
  public var checkboxThickness: CGFloat { strokeControl }
  public var checkboxSmallBox: CGFloat { spacing35 }
  public var checkboxMediumBox: CGFloat { spacing4 }
  public var checkboxLargeBox: CGFloat { spacing5 }
  public var dialogScrimColor: Color { colorContent }
  public var radioThickness: CGFloat { strokeControl }
  public var radioSmallBox: CGFloat { spacing35 }
  public var radioSmallDot: CGFloat { spacing15 }
  public var radioMediumBox: CGFloat { spacing4 }
  public var radioMediumDot: CGFloat { spacing2 }
  public var radioLargeBox: CGFloat { spacing5 }
  public var radioLargeDot: CGFloat { spacing25 }
  public var switchSmallWidth: CGFloat { spacing7 }
  public var switchSmallHeight: CGFloat { spacing4 }
  public var switchSmallThumb: CGFloat { spacing3 }
  public var switchLargeWidth: CGFloat { spacing11 }
  public var switchLargeHeight: CGFloat { spacing6 }
  public var switchLargeThumb: CGFloat { spacing5 }
  public var controlColorNormalSurface: ColorDescriptor { ColorDescriptor(
    normalColor: colorSurface,
    hoveredColor: colorSurfaceSubtle,
    pressedColor: colorSurfaceSubtle,
    pressedOpacity: 0.9,
    disabledColor: colorSurfaceSunken
  ) }
  public var controlColorNormalContent: ColorDescriptor { ColorDescriptor(
    normalColor: colorContent,
    hoveredColor: colorContent,
    pressedColor: colorContent,
    pressedOpacity: 0.9,
    disabledColor: colorContentFaint
  ) }
  public var controlColorNormalBorder: ColorDescriptor { ColorDescriptor(
    normalColor: colorBorderStrong,
    hoveredColor: colorBorderStrong,
    pressedColor: colorBorderStrong,
    pressedOpacity: 0.9,
    disabledColor: Colors.transparent
  ) }
  public var controlColorRecessedSurface: ColorDescriptor { ColorDescriptor(
    normalColor: colorSurfaceInset,
    hoveredColor: colorSurfaceSunken,
    pressedColor: colorSurfaceSunken,
    pressedOpacity: 0.9,
    disabledColor: colorSurfaceSunken
  ) }
  public var controlColorRecessedContent: ColorDescriptor { ColorDescriptor(
    normalColor: colorContent,
    hoveredColor: colorContent,
    pressedColor: colorContent,
    pressedOpacity: 0.9,
    disabledColor: colorContentFaint
  ) }
  public var controlColorFilledSurface: ColorDescriptor { ColorDescriptor(
    normalShade: controlColorFilledSurfaceNormalShade,
    normalOpacity: 1.0,
    hoveredShade: controlColorFilledSurfaceHoveredShade,
    hoveredOpacity: 1.0,
    pressedShade: controlColorFilledSurfacePressedShade,
    pressedOpacity: 0.9,
    disabledColor: colorSurfaceSunken
  ) }
  public var controlColorFilledContent: ColorDescriptor { ColorDescriptor(
    normalColor: colorOnAccent,
    hoveredColor: colorOnAccent,
    pressedColor: colorOnAccent,
    pressedOpacity: 0.9,
    disabledColor: colorContentFaint
  ) }
  public var controlColorTintedSurface: ColorDescriptor { ColorDescriptor(
    normalShade: 600,
    normalOpacity: 0.12,
    hoveredShade: 600,
    hoveredOpacity: 0.2,
    pressedShade: 600,
    pressedOpacity: 0.24,
    disabledColor: colorSurfaceSunken
  ) }
  public var controlColorTintedContent: ColorDescriptor { ColorDescriptor(
    normalShade: controlColorTintedContentNormalShade,
    normalOpacity: 1.0,
    hoveredShade: controlColorTintedContentHoveredShade,
    hoveredOpacity: 1.0,
    pressedShade: controlColorTintedContentPressedShade,
    pressedOpacity: 0.9,
    disabledColor: colorContentFaint
  ) }
  public var controlColorOutlinedSurface: ColorDescriptor { ColorDescriptor(
    normalColor: Colors.transparent,
    hoveredShade: 600,
    hoveredOpacity: 0.08,
    pressedShade: 600,
    pressedOpacity: 0.12,
    disabledColor: colorSurfaceSunken
  ) }
  public var controlColorOutlinedContent: ColorDescriptor { ColorDescriptor(
    normalShade: controlColorOutlinedContentNormalShade,
    normalOpacity: 1.0,
    hoveredShade: controlColorOutlinedContentHoveredShade,
    hoveredOpacity: 1.0,
    pressedShade: controlColorOutlinedContentPressedShade,
    pressedOpacity: 0.9,
    disabledColor: colorContentFaint
  ) }
  public var controlColorPlainContent: ColorDescriptor { ColorDescriptor(
    normalShade: controlColorPlainContentNormalShade,
    normalOpacity: 1.0,
    hoveredShade: controlColorPlainContentHoveredShade,
    hoveredOpacity: 1.0,
    pressedShade: controlColorPlainContentPressedShade,
    pressedOpacity: 0.9,
    disabledColor: colorContentFaint
  ) }
}

public extension ThemeVariables {
  /// Studio Light — the baseline every other theme is written against.
  static let studioLight = ThemeVariables()

  /// The `bright-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let brightLight = ThemeVariables(
    colorPrimary: Colors.ink,
    colorCanvas: Color(argb: 0xFFE6_EADE),
    colorSurface: Color(argb: 0xFFFB_FAF7),
    colorSurfaceMuted: Color(argb: 0xFFF4_F3EE),
    colorSurfaceSunken: Color(argb: 0xFFE5_E3DB),
    colorSurfaceSubtle: Color(argb: 0xFFF6_F5F1),
    colorSurfaceInset: Color(argb: 0xFFF0_EFE9),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFFF_FFFF),
    colorContent: Color(argb: 0xFF11_1C2E),
    colorContentSecondary: Color(argb: 0xC711_1C2E),
    colorContentNav: Color(argb: 0xB311_1C2E),
    colorContentMuted: Color(argb: 0xA811_1C2E),
    colorContentSubtle: Color(argb: 0x7311_1C2E),
    colorContentFaint: Color(argb: 0x6111_1C2E),
    colorBorder: Color(argb: 0x1411_1C2E),
    colorBorderStrong: Color(argb: 0x1F11_1C2E),
    colorBorderMuted: Color(argb: 0x4011_1C2E),
    colorOnAccent: Color(argb: 0xFFD6_FF3F),
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1F11_1C2E))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x2411_1C2E))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2411_1C2E))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x2E11_1C2E))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x2E11_1C2E))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3311_1C2E))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x3D11_1C2E))],
    frameWindowRadius: 16.0,
    framePopoverRadius: 14.0,
    radiusSmall: 9999.0,
    radiusMedium: 9999.0,
    radiusBig: 14.0,
    progressGradientFrom: Colors.ink[600],
    progressGradientTo: Colors.ink[300]
  )

  /// The `bright-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let brightDark = ThemeVariables(
    colorPrimary: Colors.acid,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF06_0B12),
    colorSurface: Color(argb: 0xFF0C_141E),
    colorSurfaceMuted: Color(argb: 0xFF0A_111A),
    colorSurfaceSunken: Color(argb: 0xFF1C_2734),
    colorSurfaceSubtle: Color(argb: 0xFF10_1923),
    colorSurfaceInset: Color(argb: 0xFF16_202C),
    colorSurfaceRaised: Color(argb: 0xFF2A_3644),
    colorSurfaceChrome: Color(argb: 0xFF11_1A26),
    colorContent: Color(argb: 0xFFF2_F4EF),
    colorContentSecondary: Color(argb: 0xC7F2_F4EF),
    colorContentNav: Color(argb: 0xB3F2_F4EF),
    colorContentMuted: Color(argb: 0xA8F2_F4EF),
    colorContentSubtle: Color(argb: 0x73F2_F4EF),
    colorContentFaint: Color(argb: 0x61F2_F4EF),
    colorBorder: Color(argb: 0x14F2_F4EF),
    colorBorderStrong: Color(argb: 0x24F2_F4EF),
    colorBorderMuted: Color(argb: 0xFF35_414F),
    colorOnAccent: Color(argb: 0xFF11_1C2E),
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.55,
    frameWindowRadius: 16.0,
    framePopoverRadius: 14.0,
    radiusSmall: 9999.0,
    radiusMedium: 9999.0,
    radiusBig: 14.0,
    progressGradientFrom: Colors.acid[500],
    progressGradientTo: Colors.acid[500],
    controlColorFilledSurfaceNormalShade: 500,
    controlColorFilledSurfaceHoveredShade: 600,
    controlColorFilledSurfacePressedShade: 600,
    controlColorTintedContentNormalShade: 300,
    controlColorTintedContentHoveredShade: 200,
    controlColorTintedContentPressedShade: 200,
    controlColorOutlinedContentNormalShade: 300,
    controlColorOutlinedContentHoveredShade: 200,
    controlColorOutlinedContentPressedShade: 200,
    controlColorOutlinedBorder: ColorDescriptor(
      normalShade: 400,
      normalOpacity: 1.0,
      hoveredShade: 300,
      hoveredOpacity: 1.0,
      pressedShade: 300,
      pressedOpacity: 0.9,
      disabledColor: Colors.transparent
    ),
    controlColorPlainContentNormalShade: 300,
    controlColorPlainContentHoveredShade: 200,
    controlColorPlainContentPressedShade: 200
  )

  /// The `frost-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let frostLight = ThemeVariables(
    colorPrimary: Colors.frost,
    colorCanvas: Color(argb: 0xFFE5_EDF0),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF2_F6F8),
    colorSurfaceSunken: Color(argb: 0xFFDB_E6EA),
    colorSurfaceSubtle: Color(argb: 0xFFF4_F8F9),
    colorSurfaceInset: Color(argb: 0xFFEC_F2F4),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF2_F6F8),
    colorContent: Color(argb: 0xFF0E_1F26),
    colorContentSecondary: Color(argb: 0xFF2F_4C58),
    colorContentNav: Color(argb: 0xFF3A_5B68),
    colorContentMuted: Color(argb: 0xFF45_6573),
    colorContentSubtle: Color(argb: 0xFF77_94A0),
    colorContentFaint: Color(argb: 0xFF9A_B1BA),
    colorBorder: Color(argb: 0x140E_1F26),
    colorBorderStrong: Color(argb: 0x1F0E_1F26),
    colorBorderMuted: Color(argb: 0xFFC6_D5DB),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    progressGradientFrom: Colors.frost[600],
    progressGradientTo: Colors.frost[300]
  )

  /// The `frost-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let frostDark = ThemeVariables(
    colorPrimary: Colors.frost,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF07_0D11),
    colorSurface: Color(argb: 0xFF0D_151A),
    colorSurfaceMuted: Color(argb: 0xFF11_1A20),
    colorSurfaceSunken: Color(argb: 0xFF23_333D),
    colorSurfaceSubtle: Color(argb: 0xFF14_2027),
    colorSurfaceInset: Color(argb: 0xFF18_262E),
    colorSurfaceRaised: Color(argb: 0xFF37_4B57),
    colorSurfaceChrome: Color(argb: 0xFF11_1A20),
    colorContent: Color(argb: 0xFFEA_F3F7),
    colorContentSecondary: Color(argb: 0xFFBC_D0D9),
    colorContentNav: Color(argb: 0xFF93_AAB5),
    colorContentMuted: Color(argb: 0xFF93_AAB5),
    colorContentSubtle: Color(argb: 0xFF7F_99A4),
    colorContentFaint: Color(argb: 0xFF62_808D),
    colorBorder: Color(argb: 0x0FFF_FFFF),
    colorBorderStrong: Color(argb: 0x1AFF_FFFF),
    colorBorderMuted: Color(argb: 0xFF32_454F),
    colorOnAccent: Color(argb: 0xFF04_171D),
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    progressGradientFrom: Colors.frost[500],
    progressGradientTo: Colors.frost[300],
    controlColorFilledSurfaceNormalShade: 500,
    controlColorFilledSurfaceHoveredShade: 600,
    controlColorFilledSurfacePressedShade: 600,
    controlColorTintedContentNormalShade: 300,
    controlColorTintedContentHoveredShade: 200,
    controlColorTintedContentPressedShade: 200,
    controlColorOutlinedContentNormalShade: 300,
    controlColorOutlinedContentHoveredShade: 200,
    controlColorOutlinedContentPressedShade: 200,
    controlColorOutlinedBorder: ColorDescriptor(
      normalShade: 400,
      normalOpacity: 1.0,
      hoveredShade: 300,
      hoveredOpacity: 1.0,
      pressedShade: 300,
      pressedOpacity: 0.9,
      disabledColor: Colors.transparent
    ),
    controlColorPlainContentNormalShade: 300,
    controlColorPlainContentHoveredShade: 200,
    controlColorPlainContentPressedShade: 200
  )

  /// The `graphite-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let graphiteLight = ThemeVariables(
    colorPrimary: Colors.graphite,
    colorCanvas: Color(argb: 0xFFE8_E8EC),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF6_F6F8),
    colorSurfaceSunken: Color(argb: 0xFFE2_E2E7),
    colorSurfaceSubtle: Color(argb: 0xFFF4_F4F6),
    colorSurfaceInset: Color(argb: 0xFFEF_EFF2),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF6_F6F8),
    colorContent: Color(argb: 0xFF17_171B),
    colorContentSecondary: Color(argb: 0xFF3D_3D45),
    colorContentNav: Color(argb: 0xFF4C_4C56),
    colorContentMuted: Color(argb: 0xFF57_575F),
    colorContentSubtle: Color(argb: 0xFF8A_8A93),
    colorContentFaint: Color(argb: 0xFFA3_A3AB),
    colorBorder: Color(argb: 0x1217_171B),
    colorBorderStrong: Color(argb: 0x1F17_171B),
    colorBorderMuted: Color(argb: 0xFFD5_D5DA),
    colorOnAccent: Color(argb: 0xFFFA_FAFA),
    frameWindowRadius: 16.0,
    framePopoverRadius: 12.0,
    radiusSmall: 7.0,
    radiusMedium: 8.0,
    radiusLarge: 10.0,
    radiusBig: 12.0,
    progressGradientFrom: Colors.graphite[600],
    progressGradientTo: Colors.graphite[400]
  )

  /// The `graphite-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let graphiteDark = ThemeVariables(
    colorPrimary: Colors.graphiteDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF08_080A),
    colorSurface: Color(argb: 0xFF0F_0F12),
    colorSurfaceMuted: Color(argb: 0xFF14_1417),
    colorSurfaceSunken: Color(argb: 0xFF27_272C),
    colorSurfaceSubtle: Color(argb: 0xFF17_171B),
    colorSurfaceInset: Color(argb: 0xFF1C_1C21),
    colorSurfaceRaised: Color(argb: 0xFF3B_3B43),
    colorSurfaceChrome: Color(argb: 0xFF14_1417),
    colorContent: Color(argb: 0xFFF4_F4F6),
    colorContentSecondary: Color(argb: 0xFFC9_C9D0),
    colorContentNav: Color(argb: 0xFF9D_9DA6),
    colorContentMuted: Color(argb: 0xFF9D_9DA6),
    colorContentSubtle: Color(argb: 0xFF8A_8A93),
    colorContentFaint: Color(argb: 0xFF6A_6A73),
    colorBorder: Color(argb: 0x0FFF_FFFF),
    colorBorderStrong: Color(argb: 0x1AFF_FFFF),
    colorBorderMuted: Color(argb: 0xFF38_383F),
    colorOnAccent: Color(argb: 0xFF13_1316),
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 16.0,
    framePopoverRadius: 12.0,
    radiusSmall: 7.0,
    radiusMedium: 8.0,
    radiusLarge: 10.0,
    radiusBig: 12.0,
    progressGradientFrom: Colors.graphiteDark[500],
    progressGradientTo: Colors.graphiteDark[400],
    controlColorFilledSurfaceNormalShade: 500,
    controlColorFilledSurfaceHoveredShade: 600,
    controlColorFilledSurfacePressedShade: 600,
    controlColorTintedContentNormalShade: 300,
    controlColorTintedContentHoveredShade: 200,
    controlColorTintedContentPressedShade: 200,
    controlColorOutlinedContentNormalShade: 300,
    controlColorOutlinedContentHoveredShade: 200,
    controlColorOutlinedContentPressedShade: 200,
    controlColorOutlinedBorder: ColorDescriptor(
      normalShade: 400,
      normalOpacity: 1.0,
      hoveredShade: 300,
      hoveredOpacity: 1.0,
      pressedShade: 300,
      pressedOpacity: 0.9,
      disabledColor: Colors.transparent
    ),
    controlColorPlainContentNormalShade: 300,
    controlColorPlainContentHoveredShade: 200,
    controlColorPlainContentPressedShade: 200
  )

  /// The `ember-light` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let emberLight = ThemeVariables(
    colorPrimary: Colors.ember,
    colorCanvas: Color(argb: 0xFFEA_E2D4),
    colorSurface: Color(argb: 0xFFFF_FDF9),
    colorSurfaceMuted: Color(argb: 0xFFF6_F1E8),
    colorSurfaceSunken: Color(argb: 0xFFE6_DDCD),
    colorSurfaceSubtle: Color(argb: 0xFFF9_F4EC),
    colorSurfaceInset: Color(argb: 0xFFF2_EBDF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FDF9),
    colorSurfaceChrome: Color(argb: 0xFFF6_F1E8),
    colorContent: Color(argb: 0xFF24_1A11),
    colorContentSecondary: Color(argb: 0xFF4B_3A2B),
    colorContentNav: Color(argb: 0xFF5B_4839),
    colorContentMuted: Color(argb: 0xFF6A_5747),
    colorContentSubtle: Color(argb: 0xFF96_867A),
    colorContentFaint: Color(argb: 0xFFB0_A294),
    colorBorder: Color(argb: 0x1424_1A11),
    colorBorderStrong: Color(argb: 0x1F24_1A11),
    colorBorderMuted: Color(argb: 0xFFDD_D1C0),
    colorOnAccent: Color(argb: 0xFFFF_FAF3),
    progressGradientFrom: Colors.ember[600],
    progressGradientTo: Colors.ember[300]
  )

  /// The `ember-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let emberDark = ThemeVariables(
    colorPrimary: Colors.emberDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0C_0906),
    colorSurface: Color(argb: 0xFF14_100B),
    colorSurfaceMuted: Color(argb: 0xFF19_1410),
    colorSurfaceSunken: Color(argb: 0xFF2F_271E),
    colorSurfaceSubtle: Color(argb: 0xFF1D_1813),
    colorSurfaceInset: Color(argb: 0xFF23_1D16),
    colorSurfaceRaised: Color(argb: 0xFF45_3A2D),
    colorSurfaceChrome: Color(argb: 0xFF19_1410),
    colorContent: Color(argb: 0xFFF6_EFE4),
    colorContentSecondary: Color(argb: 0xFFD5_C6B4),
    colorContentNav: Color(argb: 0xFFAB_9886),
    colorContentMuted: Color(argb: 0xFFAB_9886),
    colorContentSubtle: Color(argb: 0xFF94_806D),
    colorContentFaint: Color(argb: 0xFF71_604F),
    colorBorder: Color(argb: 0x0FFF_FFFF),
    colorBorderStrong: Color(argb: 0x1AFF_FFFF),
    colorBorderMuted: Color(argb: 0xFF44_3A2D),
    colorOnAccent: Color(argb: 0xFF24_1206),
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    progressGradientFrom: Colors.emberDark[500],
    progressGradientTo: Colors.emberDark[300],
    controlColorFilledSurfaceNormalShade: 500,
    controlColorFilledSurfaceHoveredShade: 600,
    controlColorFilledSurfacePressedShade: 600,
    controlColorTintedContentNormalShade: 300,
    controlColorTintedContentHoveredShade: 200,
    controlColorTintedContentPressedShade: 200,
    controlColorOutlinedContentNormalShade: 300,
    controlColorOutlinedContentHoveredShade: 200,
    controlColorOutlinedContentPressedShade: 200,
    controlColorOutlinedBorder: ColorDescriptor(
      normalShade: 400,
      normalOpacity: 1.0,
      hoveredShade: 300,
      hoveredOpacity: 1.0,
      pressedShade: 300,
      pressedOpacity: 0.9,
      disabledColor: Colors.transparent
    ),
    controlColorPlainContentNormalShade: 300,
    controlColorPlainContentHoveredShade: 200,
    controlColorPlainContentPressedShade: 200
  )

  /// The `studio-dark` counterpart of ``studioLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let studioDark = ThemeVariables(
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0A_0B10),
    colorSurface: Color(argb: 0xFF0D_0F14),
    colorSurfaceMuted: Color(argb: 0xFF12_141A),
    colorSurfaceSunken: Color(argb: 0xFF26_2A34),
    colorSurfaceSubtle: Color(argb: 0xFF15_1821),
    colorSurfaceInset: Color(argb: 0xFF1B_1E26),
    colorSurfaceRaised: Color(argb: 0xFF3A_3F4D),
    colorSurfaceChrome: Color(argb: 0xFF12_141A),
    colorContent: Color(argb: 0xFFF2_F3FA),
    colorContentSecondary: Color(argb: 0xFFC3_C8DC),
    colorContentNav: Color(argb: 0xFF9A_A1BB),
    colorContentMuted: Color(argb: 0xFF9A_A1BB),
    colorContentSubtle: Color(argb: 0xFF8B_93B0),
    colorContentFaint: Color(argb: 0xFF6A_7090),
    colorBorder: Color(argb: 0x0FFF_FFFF),
    colorBorderStrong: Color(argb: 0x1AFF_FFFF),
    colorBorderMuted: Color(argb: 0xFF36_3C4D),
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    progressGradientFrom: Colors.brand[500],
    progressGradientTo: Colors.brand[300],
    controlColorFilledSurfaceNormalShade: 500,
    controlColorFilledSurfaceHoveredShade: 600,
    controlColorFilledSurfacePressedShade: 600,
    controlColorTintedContentNormalShade: 300,
    controlColorTintedContentHoveredShade: 200,
    controlColorTintedContentPressedShade: 200,
    controlColorOutlinedContentNormalShade: 300,
    controlColorOutlinedContentHoveredShade: 200,
    controlColorOutlinedContentPressedShade: 200,
    controlColorOutlinedBorder: ColorDescriptor(
      normalShade: 400,
      normalOpacity: 1.0,
      hoveredShade: 300,
      hoveredOpacity: 1.0,
      pressedShade: 300,
      pressedOpacity: 0.9,
      disabledColor: Colors.transparent
    ),
    controlColorPlainContentNormalShade: 300,
    controlColorPlainContentHoveredShade: 200,
    controlColorPlainContentPressedShade: 200
  )
}
