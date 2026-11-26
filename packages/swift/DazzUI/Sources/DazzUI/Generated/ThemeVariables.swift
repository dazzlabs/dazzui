import Foundation
import SwiftUI
#if canImport(os)
  import os
#endif

/// The complete token set for one theme.
///
/// DO NOT EDIT - This file is auto-generated
public struct ThemeVariables: Sendable {
  /// Reference-counted backing storage so passing and reading `ThemeVariables`
  /// through SwiftUI view bodies copies one 8-byte pointer instead of the
  /// entire token table, while preserving value semantics via copy-on-write.
  private final class Storage: @unchecked Sendable {
    var systemSizing: SystemSizingMetrics?
    var colorPrimary: ColorSwatch
    var colorNeutral: ColorSwatch
    var colorInfo: ColorSwatch
    var colorSuccess: ColorSwatch
    var colorWarning: ColorSwatch
    var colorDanger: ColorSwatch
    var colorCanvas: Color
    var colorSurface: Color
    var colorSurfaceMuted: Color
    var colorSurfaceSunken: Color
    var colorSurfaceSubtle: Color
    var colorSurfaceInset: Color
    var colorSurfaceRaised: Color
    var colorSurfaceOverlay: Color
    var colorSurfaceChrome: Color
    var colorSurfaceColumn: Color
    var colorContent: Color
    var colorContentSecondary: Color
    var colorContentNav: Color
    var colorContentMuted: Color
    var colorContentSubtle: Color
    var colorContentFaint: Color
    var colorBorder: Color
    var colorBorderStrong: Color
    var colorBorderMuted: Color
    var colorOnAccent: Color
    var controlPressedAlpha: CGFloat
    var controlFieldRadius: CGFloat
    var controlContainerRadius: CGFloat
    var controlTinySize: CGFloat
    var controlSmallSize: CGFloat
    var controlMediumPaddingInline: CGFloat
    var controlMediumSize: CGFloat
    var controlLargeSize: CGFloat
    var shadow2xs: [TokenShadow]
    var shadowXs: [TokenShadow]
    var shadowSm: [TokenShadow]
    var shadowMd: [TokenShadow]
    var shadowLg: [TokenShadow]
    var shadowXl: [TokenShadow]
    var shadow2xl: [TokenShadow]
    var focusWidth: CGFloat
    var focusOffset: CGFloat
    var focusGlowShade: Int
    var focusGlowAlpha: CGFloat
    var focusRingShade: Int
    var focusRingAlpha: CGFloat
    var frameWindowRadius: CGFloat
    var frameWindowWidth: CGFloat
    var framePopoverRadius: CGFloat
    var frameTitlebarSize: CGFloat
    var frameSidebarWidth: CGFloat
    var frameSidebarIconWidth: CGFloat
    var frameRailWidth: CGFloat
    var frameAsideWidth: CGFloat
    var frameNavGap: CGFloat
    var iconSmall: CGFloat
    var iconMedium: CGFloat
    var iconLarge: CGFloat
    var motionReducedDuration: Double
    var motionDuration: Double
    var motionEasing: BezierCurve
    var motionLoopEasing: BezierCurve
    var radiusNone: CGFloat
    var radiusTiny: CGFloat
    var radiusSmall: CGFloat
    var radiusMedium: CGFloat
    var radiusLarge: CGFloat
    var radiusBig: CGFloat
    var radiusFull: CGFloat
    var spacingPx: CGFloat
    var spacing0: CGFloat
    var spacing05: CGFloat
    var spacing1: CGFloat
    var spacing15: CGFloat
    var spacing2: CGFloat
    var spacing25: CGFloat
    var spacing3: CGFloat
    var spacing35: CGFloat
    var spacing4: CGFloat
    var spacing5: CGFloat
    var spacing6: CGFloat
    var spacing7: CGFloat
    var spacing8: CGFloat
    var spacing9: CGFloat
    var spacing10: CGFloat
    var spacing11: CGFloat
    var spacing12: CGFloat
    var spacing14: CGFloat
    var spacing16: CGFloat
    var spacing20: CGFloat
    var strokeHairline: CGFloat
    var strokeControl: CGFloat
    var fontUi: [String]
    var fontDisplay: [String]
    var fontCode: [String]
    var headlineSmall: TypeStyle
    var headlineMedium: TypeStyle
    var headlineLarge: TypeStyle
    var titleSmall: TypeStyle
    var titleMedium: TypeStyle
    var titleLarge: TypeStyle
    var bodySmall: TypeStyle
    var bodyMedium: TypeStyle
    var bodyLarge: TypeStyle
    var labelQuiet: TypeStyle
    var labelStrong: TypeStyle
    var labelSmall: TypeStyle
    var labelMedium: TypeStyle
    var labelLarge: TypeStyle
    var captionSmall: TypeStyle
    var captionMedium: TypeStyle
    var captionLarge: TypeStyle
    var washSurface: CGFloat
    var washEdge: CGFloat
    var calendarCellGap: CGFloat
    var checkboxRadius: CGFloat
    var codeBlockCopyFeedbackDuration: Double
    var dateRangePickerSideWidth: CGFloat
    var dateRangePickerTriggerMinWidth: CGFloat
    var dateRangePickerTimeMinWidth: CGFloat
    var dateRangePickerKeyTracking: CGFloat
    var dialogTransitionScale: CGFloat
    var dialogWidth: CGFloat
    var dialogScrimAlpha: CGFloat
    var dialogDangerBorderAlpha: CGFloat
    var drawerSize: CGFloat
    var spinnerDuration: Double
    var spinnerReducedDuration: Double
    var spinnerTrackAlpha: CGFloat
    var spinnerOnAccentTrackAlpha: CGFloat
    var menuTypeaheadResetDuration: Double
    var menuMinWidth: CGFloat
    var menuItemPadding: CGFloat
    var preferencesWidth: CGFloat
    var previewCardOpenDelay: Double
    var previewCardCloseDelay: Double
    var previewCardWidth: CGFloat
    var progressSweepDuration: Double
    var progressReducedSweepDuration: Double
    var progressSweepWidth: CGFloat
    var progressGradientFrom: Color
    var progressGradientTo: Color
    var searchFieldInputWidth: CGFloat
    var segmentedControlInset: CGFloat
    var shortcutRecorderWidth: CGFloat
    var shortcutRecorderRejectDuration: Double
    var sidebarGroupLabelHeight: CGFloat
    var skeletonSweepDuration: Double
    var skeletonLastLineWidth: CGFloat
    var sliderMediumTrack: CGFloat
    var sliderMediumThumb: CGFloat
    var sliderThumbInk: Int
    var sliderThumbLift: Int
    var sliderThumbEdgeWidth: CGFloat
    var sliderThumbHotScale: CGFloat
    var splitterKeyStep: Int
    var splitterKeyCoarseStep: Int
    var stepListDoneAlpha: CGFloat
    var stepListIdleOpacity: CGFloat
    var switchMediumWidth: CGFloat
    var switchMediumHeight: CGFloat
    var switchMediumThumb: CGFloat
    var tabsPulseDuration: Double
    var tabsPulseOpacity: CGFloat
    var badgeLineHeight: CGFloat
    var calloutTitleLeading: CGFloat
    var calloutDescriptionLeading: CGFloat
    var textFieldMultilineLeading: CGFloat
    var optionCardDescriptionLeading: CGFloat
    var formFieldHintLeading: CGFloat
    var preferencesDescriptionLeading: CGFloat
    var toastMessageLineHeight: CGFloat
    var toastNoticeDuration: Double
    var toastActionDuration: Double
    var toastTransitionDuration: Double
    var toastTransitionX1: CGFloat
    var toastTransitionY1: CGFloat
    var toastTransitionX2: CGFloat
    var toastTransitionY2: CGFloat
    var toastTransitionScale: CGFloat
    var toastMaxWidth: CGFloat
    var tooltipOpenDelay: Double
    var tooltipCloseDelay: Double
    var treeTypeaheadWindow: Double
    var controlColorRecessedBorder: ColorDescriptor
    var controlColorFilledSurfaceNormalShade: Int
    var controlColorFilledSurfaceHoveredShade: Int
    var controlColorFilledSurfacePressedShade: Int
    var controlColorFilledBorder: ColorDescriptor
    var controlColorTintedContentNormalShade: Int
    var controlColorTintedContentHoveredShade: Int
    var controlColorTintedContentPressedShade: Int
    var controlColorTintedBorder: ColorDescriptor
    var controlColorOutlinedContentNormalShade: Int
    var controlColorOutlinedContentHoveredShade: Int
    var controlColorOutlinedContentPressedShade: Int
    var controlColorOutlinedBorder: ColorDescriptor
    var controlColorPlainSurface: ColorDescriptor
    var controlColorPlainContentNormalShade: Int
    var controlColorPlainContentHoveredShade: Int
    var controlColorPlainContentPressedShade: Int
    var controlColorPlainBorder: ColorDescriptor
    private struct CacheState: Sendable {
      var controlColors: [ControlColors]?
      var controlMetrics: ControlMetricsTable?
    }

    #if canImport(os)
      private let cache = OSAllocatedUnfairLock(initialState: CacheState())

      func cachedControlColor(at index: Int, _ build: () -> [ControlColors]) -> ControlColors {
        if let hit = cache.withLock({ $0.controlColors?[index] }) {
          return hit
        }
        let built = build()
        return cache.withLock { state in
          if let raced = state.controlColors { return raced[index] }
          state.controlColors = built
          return built[index]
        }
      }

      func cachedControlMetrics(
        for size: ControlSize,
        _ build: () -> ControlMetricsTable
      ) -> ControlMetrics {
        if let hit = cache.withLock({ $0.controlMetrics?[size] }) {
          return hit
        }
        let built = build()
        return cache.withLock { state in
          if let raced = state.controlMetrics { return raced[size] }
          state.controlMetrics = built
          return built[size]
        }
      }

      func invalidateCache() {
        cache.withLock { state in
          state.controlColors = nil
          state.controlMetrics = nil
        }
      }
    #else
      private let cacheLock = NSLock()
      private var cacheState = CacheState()

      func cachedControlColor(at index: Int, _ build: () -> [ControlColors]) -> ControlColors {
        cacheLock.lock()
        if let hit = cacheState.controlColors?[index] {
          cacheLock.unlock()
          return hit
        }
        cacheLock.unlock()
        let built = build()
        cacheLock.lock()
        defer { cacheLock.unlock() }
        if let raced = cacheState.controlColors { return raced[index] }
        cacheState.controlColors = built
        return built[index]
      }

      func cachedControlMetrics(
        for size: ControlSize,
        _ build: () -> ControlMetricsTable
      ) -> ControlMetrics {
        cacheLock.lock()
        if let hit = cacheState.controlMetrics?[size] {
          cacheLock.unlock()
          return hit
        }
        cacheLock.unlock()
        let built = build()
        cacheLock.lock()
        defer { cacheLock.unlock() }
        if let raced = cacheState.controlMetrics { return raced[size] }
        cacheState.controlMetrics = built
        return built[size]
      }

      func invalidateCache() {
        cacheLock.lock()
        cacheState.controlColors = nil
        cacheState.controlMetrics = nil
        cacheLock.unlock()
      }
    #endif

    init(
      colorPrimary: ColorSwatch,
      colorNeutral: ColorSwatch,
      colorInfo: ColorSwatch,
      colorSuccess: ColorSwatch,
      colorWarning: ColorSwatch,
      colorDanger: ColorSwatch,
      colorCanvas: Color,
      colorSurface: Color,
      colorSurfaceMuted: Color,
      colorSurfaceSunken: Color,
      colorSurfaceSubtle: Color,
      colorSurfaceInset: Color,
      colorSurfaceRaised: Color,
      colorSurfaceOverlay: Color,
      colorSurfaceChrome: Color,
      colorSurfaceColumn: Color,
      colorContent: Color,
      colorContentSecondary: Color,
      colorContentNav: Color,
      colorContentMuted: Color,
      colorContentSubtle: Color,
      colorContentFaint: Color,
      colorBorder: Color,
      colorBorderStrong: Color,
      colorBorderMuted: Color,
      colorOnAccent: Color,
      controlPressedAlpha: CGFloat,
      controlFieldRadius: CGFloat,
      controlContainerRadius: CGFloat,
      controlTinySize: CGFloat,
      controlSmallSize: CGFloat,
      controlMediumPaddingInline: CGFloat,
      controlMediumSize: CGFloat,
      controlLargeSize: CGFloat,
      shadow2xs: [TokenShadow],
      shadowXs: [TokenShadow],
      shadowSm: [TokenShadow],
      shadowMd: [TokenShadow],
      shadowLg: [TokenShadow],
      shadowXl: [TokenShadow],
      shadow2xl: [TokenShadow],
      focusWidth: CGFloat,
      focusOffset: CGFloat,
      focusGlowShade: Int,
      focusGlowAlpha: CGFloat,
      focusRingShade: Int,
      focusRingAlpha: CGFloat,
      frameWindowRadius: CGFloat,
      frameWindowWidth: CGFloat,
      framePopoverRadius: CGFloat,
      frameTitlebarSize: CGFloat,
      frameSidebarWidth: CGFloat,
      frameSidebarIconWidth: CGFloat,
      frameRailWidth: CGFloat,
      frameAsideWidth: CGFloat,
      frameNavGap: CGFloat,
      iconSmall: CGFloat,
      iconMedium: CGFloat,
      iconLarge: CGFloat,
      motionReducedDuration: Double,
      motionDuration: Double,
      motionEasing: BezierCurve,
      motionLoopEasing: BezierCurve,
      radiusNone: CGFloat,
      radiusTiny: CGFloat,
      radiusSmall: CGFloat,
      radiusMedium: CGFloat,
      radiusLarge: CGFloat,
      radiusBig: CGFloat,
      radiusFull: CGFloat,
      spacingPx: CGFloat,
      spacing0: CGFloat,
      spacing05: CGFloat,
      spacing1: CGFloat,
      spacing15: CGFloat,
      spacing2: CGFloat,
      spacing25: CGFloat,
      spacing3: CGFloat,
      spacing35: CGFloat,
      spacing4: CGFloat,
      spacing5: CGFloat,
      spacing6: CGFloat,
      spacing7: CGFloat,
      spacing8: CGFloat,
      spacing9: CGFloat,
      spacing10: CGFloat,
      spacing11: CGFloat,
      spacing12: CGFloat,
      spacing14: CGFloat,
      spacing16: CGFloat,
      spacing20: CGFloat,
      strokeHairline: CGFloat,
      strokeControl: CGFloat,
      fontUi: [String],
      fontDisplay: [String],
      fontCode: [String],
      headlineSmall: TypeStyle,
      headlineMedium: TypeStyle,
      headlineLarge: TypeStyle,
      titleSmall: TypeStyle,
      titleMedium: TypeStyle,
      titleLarge: TypeStyle,
      bodySmall: TypeStyle,
      bodyMedium: TypeStyle,
      bodyLarge: TypeStyle,
      labelQuiet: TypeStyle,
      labelStrong: TypeStyle,
      labelSmall: TypeStyle,
      labelMedium: TypeStyle,
      labelLarge: TypeStyle,
      captionSmall: TypeStyle,
      captionMedium: TypeStyle,
      captionLarge: TypeStyle,
      washSurface: CGFloat,
      washEdge: CGFloat,
      calendarCellGap: CGFloat,
      checkboxRadius: CGFloat,
      codeBlockCopyFeedbackDuration: Double,
      dateRangePickerSideWidth: CGFloat,
      dateRangePickerTriggerMinWidth: CGFloat,
      dateRangePickerTimeMinWidth: CGFloat,
      dateRangePickerKeyTracking: CGFloat,
      dialogTransitionScale: CGFloat,
      dialogWidth: CGFloat,
      dialogScrimAlpha: CGFloat,
      dialogDangerBorderAlpha: CGFloat,
      drawerSize: CGFloat,
      spinnerDuration: Double,
      spinnerReducedDuration: Double,
      spinnerTrackAlpha: CGFloat,
      spinnerOnAccentTrackAlpha: CGFloat,
      menuTypeaheadResetDuration: Double,
      menuMinWidth: CGFloat,
      menuItemPadding: CGFloat,
      preferencesWidth: CGFloat,
      previewCardOpenDelay: Double,
      previewCardCloseDelay: Double,
      previewCardWidth: CGFloat,
      progressSweepDuration: Double,
      progressReducedSweepDuration: Double,
      progressSweepWidth: CGFloat,
      progressGradientFrom: Color,
      progressGradientTo: Color,
      searchFieldInputWidth: CGFloat,
      segmentedControlInset: CGFloat,
      shortcutRecorderWidth: CGFloat,
      shortcutRecorderRejectDuration: Double,
      sidebarGroupLabelHeight: CGFloat,
      skeletonSweepDuration: Double,
      skeletonLastLineWidth: CGFloat,
      sliderMediumTrack: CGFloat,
      sliderMediumThumb: CGFloat,
      sliderThumbInk: Int,
      sliderThumbLift: Int,
      sliderThumbEdgeWidth: CGFloat,
      sliderThumbHotScale: CGFloat,
      splitterKeyStep: Int,
      splitterKeyCoarseStep: Int,
      stepListDoneAlpha: CGFloat,
      stepListIdleOpacity: CGFloat,
      switchMediumWidth: CGFloat,
      switchMediumHeight: CGFloat,
      switchMediumThumb: CGFloat,
      tabsPulseDuration: Double,
      tabsPulseOpacity: CGFloat,
      badgeLineHeight: CGFloat,
      calloutTitleLeading: CGFloat,
      calloutDescriptionLeading: CGFloat,
      textFieldMultilineLeading: CGFloat,
      optionCardDescriptionLeading: CGFloat,
      formFieldHintLeading: CGFloat,
      preferencesDescriptionLeading: CGFloat,
      toastMessageLineHeight: CGFloat,
      toastNoticeDuration: Double,
      toastActionDuration: Double,
      toastTransitionDuration: Double,
      toastTransitionX1: CGFloat,
      toastTransitionY1: CGFloat,
      toastTransitionX2: CGFloat,
      toastTransitionY2: CGFloat,
      toastTransitionScale: CGFloat,
      toastMaxWidth: CGFloat,
      tooltipOpenDelay: Double,
      tooltipCloseDelay: Double,
      treeTypeaheadWindow: Double,
      controlColorRecessedBorder: ColorDescriptor,
      controlColorFilledSurfaceNormalShade: Int,
      controlColorFilledSurfaceHoveredShade: Int,
      controlColorFilledSurfacePressedShade: Int,
      controlColorFilledBorder: ColorDescriptor,
      controlColorTintedContentNormalShade: Int,
      controlColorTintedContentHoveredShade: Int,
      controlColorTintedContentPressedShade: Int,
      controlColorTintedBorder: ColorDescriptor,
      controlColorOutlinedContentNormalShade: Int,
      controlColorOutlinedContentHoveredShade: Int,
      controlColorOutlinedContentPressedShade: Int,
      controlColorOutlinedBorder: ColorDescriptor,
      controlColorPlainSurface: ColorDescriptor,
      controlColorPlainContentNormalShade: Int,
      controlColorPlainContentHoveredShade: Int,
      controlColorPlainContentPressedShade: Int,
      controlColorPlainBorder: ColorDescriptor
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
      self.colorSurfaceOverlay = colorSurfaceOverlay
      self.colorSurfaceChrome = colorSurfaceChrome
      self.colorSurfaceColumn = colorSurfaceColumn
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
      self.controlFieldRadius = controlFieldRadius
      self.controlContainerRadius = controlContainerRadius
      self.controlTinySize = controlTinySize
      self.controlSmallSize = controlSmallSize
      self.controlMediumPaddingInline = controlMediumPaddingInline
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
      self.frameWindowWidth = frameWindowWidth
      self.framePopoverRadius = framePopoverRadius
      self.frameTitlebarSize = frameTitlebarSize
      self.frameSidebarWidth = frameSidebarWidth
      self.frameSidebarIconWidth = frameSidebarIconWidth
      self.frameRailWidth = frameRailWidth
      self.frameAsideWidth = frameAsideWidth
      self.frameNavGap = frameNavGap
      self.iconSmall = iconSmall
      self.iconMedium = iconMedium
      self.iconLarge = iconLarge
      self.motionReducedDuration = motionReducedDuration
      self.motionDuration = motionDuration
      self.motionEasing = motionEasing
      self.motionLoopEasing = motionLoopEasing
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
      self.fontUi = fontUi
      self.fontDisplay = fontDisplay
      self.fontCode = fontCode
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
      self.calendarCellGap = calendarCellGap
      self.checkboxRadius = checkboxRadius
      self.codeBlockCopyFeedbackDuration = codeBlockCopyFeedbackDuration
      self.dateRangePickerSideWidth = dateRangePickerSideWidth
      self.dateRangePickerTriggerMinWidth = dateRangePickerTriggerMinWidth
      self.dateRangePickerTimeMinWidth = dateRangePickerTimeMinWidth
      self.dateRangePickerKeyTracking = dateRangePickerKeyTracking
      self.dialogTransitionScale = dialogTransitionScale
      self.dialogWidth = dialogWidth
      self.dialogScrimAlpha = dialogScrimAlpha
      self.dialogDangerBorderAlpha = dialogDangerBorderAlpha
      self.drawerSize = drawerSize
      self.spinnerDuration = spinnerDuration
      self.spinnerReducedDuration = spinnerReducedDuration
      self.spinnerTrackAlpha = spinnerTrackAlpha
      self.spinnerOnAccentTrackAlpha = spinnerOnAccentTrackAlpha
      self.menuTypeaheadResetDuration = menuTypeaheadResetDuration
      self.menuMinWidth = menuMinWidth
      self.menuItemPadding = menuItemPadding
      self.preferencesWidth = preferencesWidth
      self.previewCardOpenDelay = previewCardOpenDelay
      self.previewCardCloseDelay = previewCardCloseDelay
      self.previewCardWidth = previewCardWidth
      self.progressSweepDuration = progressSweepDuration
      self.progressReducedSweepDuration = progressReducedSweepDuration
      self.progressSweepWidth = progressSweepWidth
      self.progressGradientFrom = progressGradientFrom
      self.progressGradientTo = progressGradientTo
      self.searchFieldInputWidth = searchFieldInputWidth
      self.segmentedControlInset = segmentedControlInset
      self.shortcutRecorderWidth = shortcutRecorderWidth
      self.shortcutRecorderRejectDuration = shortcutRecorderRejectDuration
      self.sidebarGroupLabelHeight = sidebarGroupLabelHeight
      self.skeletonSweepDuration = skeletonSweepDuration
      self.skeletonLastLineWidth = skeletonLastLineWidth
      self.sliderMediumTrack = sliderMediumTrack
      self.sliderMediumThumb = sliderMediumThumb
      self.sliderThumbInk = sliderThumbInk
      self.sliderThumbLift = sliderThumbLift
      self.sliderThumbEdgeWidth = sliderThumbEdgeWidth
      self.sliderThumbHotScale = sliderThumbHotScale
      self.splitterKeyStep = splitterKeyStep
      self.splitterKeyCoarseStep = splitterKeyCoarseStep
      self.stepListDoneAlpha = stepListDoneAlpha
      self.stepListIdleOpacity = stepListIdleOpacity
      self.switchMediumWidth = switchMediumWidth
      self.switchMediumHeight = switchMediumHeight
      self.switchMediumThumb = switchMediumThumb
      self.tabsPulseDuration = tabsPulseDuration
      self.tabsPulseOpacity = tabsPulseOpacity
      self.badgeLineHeight = badgeLineHeight
      self.calloutTitleLeading = calloutTitleLeading
      self.calloutDescriptionLeading = calloutDescriptionLeading
      self.textFieldMultilineLeading = textFieldMultilineLeading
      self.optionCardDescriptionLeading = optionCardDescriptionLeading
      self.formFieldHintLeading = formFieldHintLeading
      self.preferencesDescriptionLeading = preferencesDescriptionLeading
      self.toastMessageLineHeight = toastMessageLineHeight
      self.toastNoticeDuration = toastNoticeDuration
      self.toastActionDuration = toastActionDuration
      self.toastTransitionDuration = toastTransitionDuration
      self.toastTransitionX1 = toastTransitionX1
      self.toastTransitionY1 = toastTransitionY1
      self.toastTransitionX2 = toastTransitionX2
      self.toastTransitionY2 = toastTransitionY2
      self.toastTransitionScale = toastTransitionScale
      self.toastMaxWidth = toastMaxWidth
      self.tooltipOpenDelay = tooltipOpenDelay
      self.tooltipCloseDelay = tooltipCloseDelay
      self.treeTypeaheadWindow = treeTypeaheadWindow
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

    func copy() -> Storage {
      let duplicate = Storage(
        colorPrimary: colorPrimary,
        colorNeutral: colorNeutral,
        colorInfo: colorInfo,
        colorSuccess: colorSuccess,
        colorWarning: colorWarning,
        colorDanger: colorDanger,
        colorCanvas: colorCanvas,
        colorSurface: colorSurface,
        colorSurfaceMuted: colorSurfaceMuted,
        colorSurfaceSunken: colorSurfaceSunken,
        colorSurfaceSubtle: colorSurfaceSubtle,
        colorSurfaceInset: colorSurfaceInset,
        colorSurfaceRaised: colorSurfaceRaised,
        colorSurfaceOverlay: colorSurfaceOverlay,
        colorSurfaceChrome: colorSurfaceChrome,
        colorSurfaceColumn: colorSurfaceColumn,
        colorContent: colorContent,
        colorContentSecondary: colorContentSecondary,
        colorContentNav: colorContentNav,
        colorContentMuted: colorContentMuted,
        colorContentSubtle: colorContentSubtle,
        colorContentFaint: colorContentFaint,
        colorBorder: colorBorder,
        colorBorderStrong: colorBorderStrong,
        colorBorderMuted: colorBorderMuted,
        colorOnAccent: colorOnAccent,
        controlPressedAlpha: controlPressedAlpha,
        controlFieldRadius: controlFieldRadius,
        controlContainerRadius: controlContainerRadius,
        controlTinySize: controlTinySize,
        controlSmallSize: controlSmallSize,
        controlMediumPaddingInline: controlMediumPaddingInline,
        controlMediumSize: controlMediumSize,
        controlLargeSize: controlLargeSize,
        shadow2xs: shadow2xs,
        shadowXs: shadowXs,
        shadowSm: shadowSm,
        shadowMd: shadowMd,
        shadowLg: shadowLg,
        shadowXl: shadowXl,
        shadow2xl: shadow2xl,
        focusWidth: focusWidth,
        focusOffset: focusOffset,
        focusGlowShade: focusGlowShade,
        focusGlowAlpha: focusGlowAlpha,
        focusRingShade: focusRingShade,
        focusRingAlpha: focusRingAlpha,
        frameWindowRadius: frameWindowRadius,
        frameWindowWidth: frameWindowWidth,
        framePopoverRadius: framePopoverRadius,
        frameTitlebarSize: frameTitlebarSize,
        frameSidebarWidth: frameSidebarWidth,
        frameSidebarIconWidth: frameSidebarIconWidth,
        frameRailWidth: frameRailWidth,
        frameAsideWidth: frameAsideWidth,
        frameNavGap: frameNavGap,
        iconSmall: iconSmall,
        iconMedium: iconMedium,
        iconLarge: iconLarge,
        motionReducedDuration: motionReducedDuration,
        motionDuration: motionDuration,
        motionEasing: motionEasing,
        motionLoopEasing: motionLoopEasing,
        radiusNone: radiusNone,
        radiusTiny: radiusTiny,
        radiusSmall: radiusSmall,
        radiusMedium: radiusMedium,
        radiusLarge: radiusLarge,
        radiusBig: radiusBig,
        radiusFull: radiusFull,
        spacingPx: spacingPx,
        spacing0: spacing0,
        spacing05: spacing05,
        spacing1: spacing1,
        spacing15: spacing15,
        spacing2: spacing2,
        spacing25: spacing25,
        spacing3: spacing3,
        spacing35: spacing35,
        spacing4: spacing4,
        spacing5: spacing5,
        spacing6: spacing6,
        spacing7: spacing7,
        spacing8: spacing8,
        spacing9: spacing9,
        spacing10: spacing10,
        spacing11: spacing11,
        spacing12: spacing12,
        spacing14: spacing14,
        spacing16: spacing16,
        spacing20: spacing20,
        strokeHairline: strokeHairline,
        strokeControl: strokeControl,
        fontUi: fontUi,
        fontDisplay: fontDisplay,
        fontCode: fontCode,
        headlineSmall: headlineSmall,
        headlineMedium: headlineMedium,
        headlineLarge: headlineLarge,
        titleSmall: titleSmall,
        titleMedium: titleMedium,
        titleLarge: titleLarge,
        bodySmall: bodySmall,
        bodyMedium: bodyMedium,
        bodyLarge: bodyLarge,
        labelQuiet: labelQuiet,
        labelStrong: labelStrong,
        labelSmall: labelSmall,
        labelMedium: labelMedium,
        labelLarge: labelLarge,
        captionSmall: captionSmall,
        captionMedium: captionMedium,
        captionLarge: captionLarge,
        washSurface: washSurface,
        washEdge: washEdge,
        calendarCellGap: calendarCellGap,
        checkboxRadius: checkboxRadius,
        codeBlockCopyFeedbackDuration: codeBlockCopyFeedbackDuration,
        dateRangePickerSideWidth: dateRangePickerSideWidth,
        dateRangePickerTriggerMinWidth: dateRangePickerTriggerMinWidth,
        dateRangePickerTimeMinWidth: dateRangePickerTimeMinWidth,
        dateRangePickerKeyTracking: dateRangePickerKeyTracking,
        dialogTransitionScale: dialogTransitionScale,
        dialogWidth: dialogWidth,
        dialogScrimAlpha: dialogScrimAlpha,
        dialogDangerBorderAlpha: dialogDangerBorderAlpha,
        drawerSize: drawerSize,
        spinnerDuration: spinnerDuration,
        spinnerReducedDuration: spinnerReducedDuration,
        spinnerTrackAlpha: spinnerTrackAlpha,
        spinnerOnAccentTrackAlpha: spinnerOnAccentTrackAlpha,
        menuTypeaheadResetDuration: menuTypeaheadResetDuration,
        menuMinWidth: menuMinWidth,
        menuItemPadding: menuItemPadding,
        preferencesWidth: preferencesWidth,
        previewCardOpenDelay: previewCardOpenDelay,
        previewCardCloseDelay: previewCardCloseDelay,
        previewCardWidth: previewCardWidth,
        progressSweepDuration: progressSweepDuration,
        progressReducedSweepDuration: progressReducedSweepDuration,
        progressSweepWidth: progressSweepWidth,
        progressGradientFrom: progressGradientFrom,
        progressGradientTo: progressGradientTo,
        searchFieldInputWidth: searchFieldInputWidth,
        segmentedControlInset: segmentedControlInset,
        shortcutRecorderWidth: shortcutRecorderWidth,
        shortcutRecorderRejectDuration: shortcutRecorderRejectDuration,
        sidebarGroupLabelHeight: sidebarGroupLabelHeight,
        skeletonSweepDuration: skeletonSweepDuration,
        skeletonLastLineWidth: skeletonLastLineWidth,
        sliderMediumTrack: sliderMediumTrack,
        sliderMediumThumb: sliderMediumThumb,
        sliderThumbInk: sliderThumbInk,
        sliderThumbLift: sliderThumbLift,
        sliderThumbEdgeWidth: sliderThumbEdgeWidth,
        sliderThumbHotScale: sliderThumbHotScale,
        splitterKeyStep: splitterKeyStep,
        splitterKeyCoarseStep: splitterKeyCoarseStep,
        stepListDoneAlpha: stepListDoneAlpha,
        stepListIdleOpacity: stepListIdleOpacity,
        switchMediumWidth: switchMediumWidth,
        switchMediumHeight: switchMediumHeight,
        switchMediumThumb: switchMediumThumb,
        tabsPulseDuration: tabsPulseDuration,
        tabsPulseOpacity: tabsPulseOpacity,
        badgeLineHeight: badgeLineHeight,
        calloutTitleLeading: calloutTitleLeading,
        calloutDescriptionLeading: calloutDescriptionLeading,
        textFieldMultilineLeading: textFieldMultilineLeading,
        optionCardDescriptionLeading: optionCardDescriptionLeading,
        formFieldHintLeading: formFieldHintLeading,
        preferencesDescriptionLeading: preferencesDescriptionLeading,
        toastMessageLineHeight: toastMessageLineHeight,
        toastNoticeDuration: toastNoticeDuration,
        toastActionDuration: toastActionDuration,
        toastTransitionDuration: toastTransitionDuration,
        toastTransitionX1: toastTransitionX1,
        toastTransitionY1: toastTransitionY1,
        toastTransitionX2: toastTransitionX2,
        toastTransitionY2: toastTransitionY2,
        toastTransitionScale: toastTransitionScale,
        toastMaxWidth: toastMaxWidth,
        tooltipOpenDelay: tooltipOpenDelay,
        tooltipCloseDelay: tooltipCloseDelay,
        treeTypeaheadWindow: treeTypeaheadWindow,
        controlColorRecessedBorder: controlColorRecessedBorder,
        controlColorFilledSurfaceNormalShade: controlColorFilledSurfaceNormalShade,
        controlColorFilledSurfaceHoveredShade: controlColorFilledSurfaceHoveredShade,
        controlColorFilledSurfacePressedShade: controlColorFilledSurfacePressedShade,
        controlColorFilledBorder: controlColorFilledBorder,
        controlColorTintedContentNormalShade: controlColorTintedContentNormalShade,
        controlColorTintedContentHoveredShade: controlColorTintedContentHoveredShade,
        controlColorTintedContentPressedShade: controlColorTintedContentPressedShade,
        controlColorTintedBorder: controlColorTintedBorder,
        controlColorOutlinedContentNormalShade: controlColorOutlinedContentNormalShade,
        controlColorOutlinedContentHoveredShade: controlColorOutlinedContentHoveredShade,
        controlColorOutlinedContentPressedShade: controlColorOutlinedContentPressedShade,
        controlColorOutlinedBorder: controlColorOutlinedBorder,
        controlColorPlainSurface: controlColorPlainSurface,
        controlColorPlainContentNormalShade: controlColorPlainContentNormalShade,
        controlColorPlainContentHoveredShade: controlColorPlainContentHoveredShade,
        controlColorPlainContentPressedShade: controlColorPlainContentPressedShade,
        controlColorPlainBorder: controlColorPlainBorder
      )
      duplicate.systemSizing = systemSizing
      return duplicate
    }
  }

  private var storage: Storage

  private mutating func ensureUnique() {
    if !isKnownUniquelyReferenced(&storage) {
      storage = storage.copy()
    } else {
      storage.invalidateCache()
    }
  }

  internal var systemSizing: SystemSizingMetrics? {
    get { storage.systemSizing }
    set {
      ensureUnique()
      storage.systemSizing = newValue
    }
  }

  internal func cachedControlColor(at index: Int, _ build: () -> [ControlColors]) -> ControlColors {
    storage.cachedControlColor(at: index, build)
  }

  internal func cachedControlMetrics(
    for size: ControlSize,
    _ build: () -> ControlMetricsTable
  ) -> ControlMetrics {
    storage.cachedControlMetrics(for: size, build)
  }

  internal var storageIdentity: ObjectIdentifier {
    ObjectIdentifier(storage)
  }
  public init(
    colorPrimary: ColorSwatch = Colors.ink,
    colorNeutral: ColorSwatch = Colors.neutral,
    colorInfo: ColorSwatch = Colors.sky,
    colorSuccess: ColorSwatch = Colors.green,
    colorWarning: ColorSwatch = Colors.amber,
    colorDanger: ColorSwatch = Colors.red,
    colorCanvas: Color = Color(argb: 0xFFE6_EADE),
    colorSurface: Color = Color(argb: 0xFFFB_FAF7),
    colorSurfaceMuted: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceSunken: Color = Color(argb: 0xFFE5_E3DB),
    colorSurfaceSubtle: Color = Color(argb: 0x0F11_1C2E),
    colorSurfaceInset: Color = Color(argb: 0xFFF0_EFE9),
    colorSurfaceRaised: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color = Color(argb: 0xFFFB_FAF7),
    colorSurfaceChrome: Color = Color(argb: 0xFFFF_FFFF),
    colorSurfaceColumn: Color = Color(argb: 0xFFF4_F3EE),
    colorContent: Color = Color(argb: 0xFF11_1C2E),
    colorContentSecondary: Color = Color(argb: 0xC711_1C2E),
    colorContentNav: Color = Color(argb: 0xB311_1C2E),
    colorContentMuted: Color = Color(argb: 0xA811_1C2E),
    colorContentSubtle: Color = Color(argb: 0x7311_1C2E),
    colorContentFaint: Color = Color(argb: 0x6111_1C2E),
    colorBorder: Color = Color(argb: 0x1411_1C2E),
    colorBorderStrong: Color = Color(argb: 0x1711_1C2E),
    colorBorderMuted: Color = Color(argb: 0x4011_1C2E),
    colorOnAccent: Color = Color(argb: 0xFFD6_FF3F),
    controlPressedAlpha: CGFloat = 0.9,
    controlFieldRadius: CGFloat = 9999.0,
    controlContainerRadius: CGFloat = 14.0,
    controlTinySize: CGFloat = 24.0,
    controlSmallSize: CGFloat = 26.0,
    controlMediumPaddingInline: CGFloat = 16.0,
    controlMediumSize: CGFloat = 28.0,
    controlLargeSize: CGFloat = 32.0,
    shadow2xs: [TokenShadow] = [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1F11_1C2E))],
    shadowXs: [TokenShadow] = [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x2411_1C2E))],
    shadowSm: [TokenShadow] = [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2411_1C2E))],
    shadowMd: [TokenShadow] = [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x2E11_1C2E))],
    shadowLg: [TokenShadow] = [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x2E11_1C2E))],
    shadowXl: [TokenShadow] = [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3311_1C2E))],
    shadow2xl: [TokenShadow] = [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x3D11_1C2E))],
    focusWidth: CGFloat = 3.0,
    focusOffset: CGFloat = 0.0,
    focusGlowShade: Int = 600,
    focusGlowAlpha: CGFloat = 0.14,
    focusRingShade: Int = 600,
    focusRingAlpha: CGFloat = 0.45,
    frameWindowRadius: CGFloat = 16.0,
    frameWindowWidth: CGFloat = 840.0,
    framePopoverRadius: CGFloat = 14.0,
    frameTitlebarSize: CGFloat = 52.0,
    frameSidebarWidth: CGFloat = 172.0,
    frameSidebarIconWidth: CGFloat = 48.0,
    frameRailWidth: CGFloat = 150.0,
    frameAsideWidth: CGFloat = 214.0,
    frameNavGap: CGFloat = 3.0,
    iconSmall: CGFloat = 12.0,
    iconMedium: CGFloat = 16.0,
    iconLarge: CGFloat = 20.0,
    motionReducedDuration: Double = 0.00001,
    motionDuration: Double = 0.15,
    motionEasing: BezierCurve = BezierCurve(0.4, 0.0, 0.2, 1.0),
    motionLoopEasing: BezierCurve = BezierCurve(0.42, 0.0, 0.58, 1.0),
    radiusNone: CGFloat = 0.0,
    radiusTiny: CGFloat = 7.0,
    radiusSmall: CGFloat = 9999.0,
    radiusMedium: CGFloat = 9999.0,
    radiusLarge: CGFloat = 12.0,
    radiusBig: CGFloat = 14.0,
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
    fontUi: [String] = ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"],
    fontDisplay: [String] = ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"],
    fontCode: [String] = ["ui-monospace", "SF Mono", "Menlo", "monospace"],
    headlineSmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 26.0),
    headlineMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 24.0, weight: .semibold, lineHeight: 30.0),
    headlineLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 28.0, weight: .semibold, lineHeight: 34.0),
    titleSmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .semibold, lineHeight: 22.0),
    titleLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 17.0, weight: .semibold, lineHeight: 22.0),
    bodySmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 18.0),
    bodyMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .regular, lineHeight: 22.0),
    labelQuiet: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .semibold, lineHeight: 11.0),
    labelMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    captionSmall: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    captionMedium: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 18.0),
    captionLarge: TypeStyle = TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 20.0),
    washSurface: CGFloat = 0.06,
    washEdge: CGFloat = 0.2,
    calendarCellGap: CGFloat = 2.0,
    checkboxRadius: CGFloat = 5.0,
    codeBlockCopyFeedbackDuration: Double = 1.6,
    dateRangePickerSideWidth: CGFloat = 192.0,
    dateRangePickerTriggerMinWidth: CGFloat = 256.0,
    dateRangePickerTimeMinWidth: CGFloat = 80.0,
    dateRangePickerKeyTracking: CGFloat = 0.06,
    dialogTransitionScale: CGFloat = 0.97,
    dialogWidth: CGFloat = 440.0,
    dialogScrimAlpha: CGFloat = 0.25,
    dialogDangerBorderAlpha: CGFloat = 0.34,
    drawerSize: CGFloat = 352.0,
    spinnerDuration: Double = 1.0,
    spinnerReducedDuration: Double = 2.4,
    spinnerTrackAlpha: CGFloat = 0.25,
    spinnerOnAccentTrackAlpha: CGFloat = 0.35,
    menuTypeaheadResetDuration: Double = 0.5,
    menuMinWidth: CGFloat = 176.0,
    menuItemPadding: CGFloat = 7.0,
    preferencesWidth: CGFloat = 480.0,
    previewCardOpenDelay: Double = 0.6,
    previewCardCloseDelay: Double = 0.3,
    previewCardWidth: CGFloat = 320.0,
    progressSweepDuration: Double = 1.2,
    progressReducedSweepDuration: Double = 3.0,
    progressSweepWidth: CGFloat = 0.4,
    progressGradientFrom: Color = Colors.ink[600],
    progressGradientTo: Color = Colors.ink[300],
    searchFieldInputWidth: CGFloat = 136.0,
    segmentedControlInset: CGFloat = 3.0,
    shortcutRecorderWidth: CGFloat = 132.0,
    shortcutRecorderRejectDuration: Double = 0.36,
    sidebarGroupLabelHeight: CGFloat = 24.0,
    skeletonSweepDuration: Double = 1.6,
    skeletonLastLineWidth: CGFloat = 0.6,
    sliderMediumTrack: CGFloat = 6.0,
    sliderMediumThumb: CGFloat = 16.0,
    sliderThumbInk: Int = 0,
    sliderThumbLift: Int = 1,
    sliderThumbEdgeWidth: CGFloat = 1.0,
    sliderThumbHotScale: CGFloat = 1.0,
    splitterKeyStep: Int = 1,
    splitterKeyCoarseStep: Int = 10,
    stepListDoneAlpha: CGFloat = 0.14,
    stepListIdleOpacity: CGFloat = 0.8,
    switchMediumWidth: CGFloat = 32.0,
    switchMediumHeight: CGFloat = 18.0,
    switchMediumThumb: CGFloat = 14.0,
    tabsPulseDuration: Double = 1.4,
    tabsPulseOpacity: CGFloat = 0.35,
    badgeLineHeight: CGFloat = 1.4,
    calloutTitleLeading: CGFloat = 1.4,
    calloutDescriptionLeading: CGFloat = 1.5,
    textFieldMultilineLeading: CGFloat = 1.6,
    optionCardDescriptionLeading: CGFloat = 1.5,
    formFieldHintLeading: CGFloat = 1.6,
    preferencesDescriptionLeading: CGFloat = 1.7,
    toastMessageLineHeight: CGFloat = 1.4,
    toastNoticeDuration: Double = 5.0,
    toastActionDuration: Double = 8.0,
    toastTransitionDuration: Double = 0.26,
    toastTransitionX1: CGFloat = 0.21,
    toastTransitionY1: CGFloat = 0.9,
    toastTransitionX2: CGFloat = 0.35,
    toastTransitionY2: CGFloat = 1.05,
    toastTransitionScale: CGFloat = 0.98,
    toastMaxWidth: CGFloat = 420.0,
    tooltipOpenDelay: Double = 0.6,
    tooltipCloseDelay: Double = 0.0,
    treeTypeaheadWindow: Double = 0.6,
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
    self.storage = Storage(
      colorPrimary: colorPrimary,
      colorNeutral: colorNeutral,
      colorInfo: colorInfo,
      colorSuccess: colorSuccess,
      colorWarning: colorWarning,
      colorDanger: colorDanger,
      colorCanvas: colorCanvas,
      colorSurface: colorSurface,
      colorSurfaceMuted: colorSurfaceMuted,
      colorSurfaceSunken: colorSurfaceSunken,
      colorSurfaceSubtle: colorSurfaceSubtle,
      colorSurfaceInset: colorSurfaceInset,
      colorSurfaceRaised: colorSurfaceRaised,
      colorSurfaceOverlay: colorSurfaceOverlay,
      colorSurfaceChrome: colorSurfaceChrome,
      colorSurfaceColumn: colorSurfaceColumn,
      colorContent: colorContent,
      colorContentSecondary: colorContentSecondary,
      colorContentNav: colorContentNav,
      colorContentMuted: colorContentMuted,
      colorContentSubtle: colorContentSubtle,
      colorContentFaint: colorContentFaint,
      colorBorder: colorBorder,
      colorBorderStrong: colorBorderStrong,
      colorBorderMuted: colorBorderMuted,
      colorOnAccent: colorOnAccent,
      controlPressedAlpha: controlPressedAlpha,
      controlFieldRadius: controlFieldRadius,
      controlContainerRadius: controlContainerRadius,
      controlTinySize: controlTinySize,
      controlSmallSize: controlSmallSize,
      controlMediumPaddingInline: controlMediumPaddingInline,
      controlMediumSize: controlMediumSize,
      controlLargeSize: controlLargeSize,
      shadow2xs: shadow2xs,
      shadowXs: shadowXs,
      shadowSm: shadowSm,
      shadowMd: shadowMd,
      shadowLg: shadowLg,
      shadowXl: shadowXl,
      shadow2xl: shadow2xl,
      focusWidth: focusWidth,
      focusOffset: focusOffset,
      focusGlowShade: focusGlowShade,
      focusGlowAlpha: focusGlowAlpha,
      focusRingShade: focusRingShade,
      focusRingAlpha: focusRingAlpha,
      frameWindowRadius: frameWindowRadius,
      frameWindowWidth: frameWindowWidth,
      framePopoverRadius: framePopoverRadius,
      frameTitlebarSize: frameTitlebarSize,
      frameSidebarWidth: frameSidebarWidth,
      frameSidebarIconWidth: frameSidebarIconWidth,
      frameRailWidth: frameRailWidth,
      frameAsideWidth: frameAsideWidth,
      frameNavGap: frameNavGap,
      iconSmall: iconSmall,
      iconMedium: iconMedium,
      iconLarge: iconLarge,
      motionReducedDuration: motionReducedDuration,
      motionDuration: motionDuration,
      motionEasing: motionEasing,
      motionLoopEasing: motionLoopEasing,
      radiusNone: radiusNone,
      radiusTiny: radiusTiny,
      radiusSmall: radiusSmall,
      radiusMedium: radiusMedium,
      radiusLarge: radiusLarge,
      radiusBig: radiusBig,
      radiusFull: radiusFull,
      spacingPx: spacingPx,
      spacing0: spacing0,
      spacing05: spacing05,
      spacing1: spacing1,
      spacing15: spacing15,
      spacing2: spacing2,
      spacing25: spacing25,
      spacing3: spacing3,
      spacing35: spacing35,
      spacing4: spacing4,
      spacing5: spacing5,
      spacing6: spacing6,
      spacing7: spacing7,
      spacing8: spacing8,
      spacing9: spacing9,
      spacing10: spacing10,
      spacing11: spacing11,
      spacing12: spacing12,
      spacing14: spacing14,
      spacing16: spacing16,
      spacing20: spacing20,
      strokeHairline: strokeHairline,
      strokeControl: strokeControl,
      fontUi: fontUi,
      fontDisplay: fontDisplay,
      fontCode: fontCode,
      headlineSmall: headlineSmall,
      headlineMedium: headlineMedium,
      headlineLarge: headlineLarge,
      titleSmall: titleSmall,
      titleMedium: titleMedium,
      titleLarge: titleLarge,
      bodySmall: bodySmall,
      bodyMedium: bodyMedium,
      bodyLarge: bodyLarge,
      labelQuiet: labelQuiet,
      labelStrong: labelStrong,
      labelSmall: labelSmall,
      labelMedium: labelMedium,
      labelLarge: labelLarge,
      captionSmall: captionSmall,
      captionMedium: captionMedium,
      captionLarge: captionLarge,
      washSurface: washSurface,
      washEdge: washEdge,
      calendarCellGap: calendarCellGap,
      checkboxRadius: checkboxRadius,
      codeBlockCopyFeedbackDuration: codeBlockCopyFeedbackDuration,
      dateRangePickerSideWidth: dateRangePickerSideWidth,
      dateRangePickerTriggerMinWidth: dateRangePickerTriggerMinWidth,
      dateRangePickerTimeMinWidth: dateRangePickerTimeMinWidth,
      dateRangePickerKeyTracking: dateRangePickerKeyTracking,
      dialogTransitionScale: dialogTransitionScale,
      dialogWidth: dialogWidth,
      dialogScrimAlpha: dialogScrimAlpha,
      dialogDangerBorderAlpha: dialogDangerBorderAlpha,
      drawerSize: drawerSize,
      spinnerDuration: spinnerDuration,
      spinnerReducedDuration: spinnerReducedDuration,
      spinnerTrackAlpha: spinnerTrackAlpha,
      spinnerOnAccentTrackAlpha: spinnerOnAccentTrackAlpha,
      menuTypeaheadResetDuration: menuTypeaheadResetDuration,
      menuMinWidth: menuMinWidth,
      menuItemPadding: menuItemPadding,
      preferencesWidth: preferencesWidth,
      previewCardOpenDelay: previewCardOpenDelay,
      previewCardCloseDelay: previewCardCloseDelay,
      previewCardWidth: previewCardWidth,
      progressSweepDuration: progressSweepDuration,
      progressReducedSweepDuration: progressReducedSweepDuration,
      progressSweepWidth: progressSweepWidth,
      progressGradientFrom: progressGradientFrom,
      progressGradientTo: progressGradientTo,
      searchFieldInputWidth: searchFieldInputWidth,
      segmentedControlInset: segmentedControlInset,
      shortcutRecorderWidth: shortcutRecorderWidth,
      shortcutRecorderRejectDuration: shortcutRecorderRejectDuration,
      sidebarGroupLabelHeight: sidebarGroupLabelHeight,
      skeletonSweepDuration: skeletonSweepDuration,
      skeletonLastLineWidth: skeletonLastLineWidth,
      sliderMediumTrack: sliderMediumTrack,
      sliderMediumThumb: sliderMediumThumb,
      sliderThumbInk: sliderThumbInk,
      sliderThumbLift: sliderThumbLift,
      sliderThumbEdgeWidth: sliderThumbEdgeWidth,
      sliderThumbHotScale: sliderThumbHotScale,
      splitterKeyStep: splitterKeyStep,
      splitterKeyCoarseStep: splitterKeyCoarseStep,
      stepListDoneAlpha: stepListDoneAlpha,
      stepListIdleOpacity: stepListIdleOpacity,
      switchMediumWidth: switchMediumWidth,
      switchMediumHeight: switchMediumHeight,
      switchMediumThumb: switchMediumThumb,
      tabsPulseDuration: tabsPulseDuration,
      tabsPulseOpacity: tabsPulseOpacity,
      badgeLineHeight: badgeLineHeight,
      calloutTitleLeading: calloutTitleLeading,
      calloutDescriptionLeading: calloutDescriptionLeading,
      textFieldMultilineLeading: textFieldMultilineLeading,
      optionCardDescriptionLeading: optionCardDescriptionLeading,
      formFieldHintLeading: formFieldHintLeading,
      preferencesDescriptionLeading: preferencesDescriptionLeading,
      toastMessageLineHeight: toastMessageLineHeight,
      toastNoticeDuration: toastNoticeDuration,
      toastActionDuration: toastActionDuration,
      toastTransitionDuration: toastTransitionDuration,
      toastTransitionX1: toastTransitionX1,
      toastTransitionY1: toastTransitionY1,
      toastTransitionX2: toastTransitionX2,
      toastTransitionY2: toastTransitionY2,
      toastTransitionScale: toastTransitionScale,
      toastMaxWidth: toastMaxWidth,
      tooltipOpenDelay: tooltipOpenDelay,
      tooltipCloseDelay: tooltipCloseDelay,
      treeTypeaheadWindow: treeTypeaheadWindow,
      controlColorRecessedBorder: controlColorRecessedBorder,
      controlColorFilledSurfaceNormalShade: controlColorFilledSurfaceNormalShade,
      controlColorFilledSurfaceHoveredShade: controlColorFilledSurfaceHoveredShade,
      controlColorFilledSurfacePressedShade: controlColorFilledSurfacePressedShade,
      controlColorFilledBorder: controlColorFilledBorder,
      controlColorTintedContentNormalShade: controlColorTintedContentNormalShade,
      controlColorTintedContentHoveredShade: controlColorTintedContentHoveredShade,
      controlColorTintedContentPressedShade: controlColorTintedContentPressedShade,
      controlColorTintedBorder: controlColorTintedBorder,
      controlColorOutlinedContentNormalShade: controlColorOutlinedContentNormalShade,
      controlColorOutlinedContentHoveredShade: controlColorOutlinedContentHoveredShade,
      controlColorOutlinedContentPressedShade: controlColorOutlinedContentPressedShade,
      controlColorOutlinedBorder: controlColorOutlinedBorder,
      controlColorPlainSurface: controlColorPlainSurface,
      controlColorPlainContentNormalShade: controlColorPlainContentNormalShade,
      controlColorPlainContentHoveredShade: controlColorPlainContentHoveredShade,
      controlColorPlainContentPressedShade: controlColorPlainContentPressedShade,
      controlColorPlainBorder: controlColorPlainBorder
    )
  }

  public var colorPrimary: ColorSwatch {
    get { storage.colorPrimary }
    set {
      ensureUnique()
      storage.colorPrimary = newValue
    }
  }
  public var colorNeutral: ColorSwatch {
    get { storage.colorNeutral }
    set {
      ensureUnique()
      storage.colorNeutral = newValue
    }
  }
  public var colorInfo: ColorSwatch {
    get { storage.colorInfo }
    set {
      ensureUnique()
      storage.colorInfo = newValue
    }
  }
  public var colorSuccess: ColorSwatch {
    get { storage.colorSuccess }
    set {
      ensureUnique()
      storage.colorSuccess = newValue
    }
  }
  public var colorWarning: ColorSwatch {
    get { storage.colorWarning }
    set {
      ensureUnique()
      storage.colorWarning = newValue
    }
  }
  public var colorDanger: ColorSwatch {
    get { storage.colorDanger }
    set {
      ensureUnique()
      storage.colorDanger = newValue
    }
  }
  public var colorCanvas: Color {
    get { storage.colorCanvas }
    set {
      ensureUnique()
      storage.colorCanvas = newValue
    }
  }
  public var colorSurface: Color {
    get { storage.colorSurface }
    set {
      ensureUnique()
      storage.colorSurface = newValue
    }
  }
  public var colorSurfaceMuted: Color {
    get { storage.colorSurfaceMuted }
    set {
      ensureUnique()
      storage.colorSurfaceMuted = newValue
    }
  }
  public var colorSurfaceSunken: Color {
    get { storage.colorSurfaceSunken }
    set {
      ensureUnique()
      storage.colorSurfaceSunken = newValue
    }
  }
  public var colorSurfaceSubtle: Color {
    get { storage.colorSurfaceSubtle }
    set {
      ensureUnique()
      storage.colorSurfaceSubtle = newValue
    }
  }
  public var colorSurfaceInset: Color {
    get { storage.colorSurfaceInset }
    set {
      ensureUnique()
      storage.colorSurfaceInset = newValue
    }
  }
  public var colorSurfaceRaised: Color {
    get { storage.colorSurfaceRaised }
    set {
      ensureUnique()
      storage.colorSurfaceRaised = newValue
    }
  }
  public var colorSurfaceOverlay: Color {
    get { storage.colorSurfaceOverlay }
    set {
      ensureUnique()
      storage.colorSurfaceOverlay = newValue
    }
  }
  public var colorSurfaceChrome: Color {
    get { storage.colorSurfaceChrome }
    set {
      ensureUnique()
      storage.colorSurfaceChrome = newValue
    }
  }
  public var colorSurfaceColumn: Color {
    get { storage.colorSurfaceColumn }
    set {
      ensureUnique()
      storage.colorSurfaceColumn = newValue
    }
  }
  public var colorContent: Color {
    get { storage.colorContent }
    set {
      ensureUnique()
      storage.colorContent = newValue
    }
  }
  public var colorContentSecondary: Color {
    get { storage.colorContentSecondary }
    set {
      ensureUnique()
      storage.colorContentSecondary = newValue
    }
  }
  public var colorContentNav: Color {
    get { storage.colorContentNav }
    set {
      ensureUnique()
      storage.colorContentNav = newValue
    }
  }
  public var colorContentMuted: Color {
    get { storage.colorContentMuted }
    set {
      ensureUnique()
      storage.colorContentMuted = newValue
    }
  }
  public var colorContentSubtle: Color {
    get { storage.colorContentSubtle }
    set {
      ensureUnique()
      storage.colorContentSubtle = newValue
    }
  }
  public var colorContentFaint: Color {
    get { storage.colorContentFaint }
    set {
      ensureUnique()
      storage.colorContentFaint = newValue
    }
  }
  public var colorBorder: Color {
    get { storage.colorBorder }
    set {
      ensureUnique()
      storage.colorBorder = newValue
    }
  }
  public var colorBorderStrong: Color {
    get { storage.colorBorderStrong }
    set {
      ensureUnique()
      storage.colorBorderStrong = newValue
    }
  }
  public var colorBorderMuted: Color {
    get { storage.colorBorderMuted }
    set {
      ensureUnique()
      storage.colorBorderMuted = newValue
    }
  }
  public var colorOnAccent: Color {
    get { storage.colorOnAccent }
    set {
      ensureUnique()
      storage.colorOnAccent = newValue
    }
  }
  public var controlPressedAlpha: CGFloat {
    get { storage.controlPressedAlpha }
    set {
      ensureUnique()
      storage.controlPressedAlpha = newValue
    }
  }
  public var controlFieldRadius: CGFloat {
    get { storage.controlFieldRadius }
    set {
      ensureUnique()
      storage.controlFieldRadius = newValue
    }
  }
  public var controlContainerRadius: CGFloat {
    get { storage.controlContainerRadius }
    set {
      ensureUnique()
      storage.controlContainerRadius = newValue
    }
  }
  public var controlTinySize: CGFloat {
    get { storage.controlTinySize }
    set {
      ensureUnique()
      storage.controlTinySize = newValue
    }
  }
  public var controlSmallSize: CGFloat {
    get { storage.controlSmallSize }
    set {
      ensureUnique()
      storage.controlSmallSize = newValue
    }
  }
  public var controlMediumPaddingInline: CGFloat {
    get { storage.controlMediumPaddingInline }
    set {
      ensureUnique()
      storage.controlMediumPaddingInline = newValue
    }
  }
  public var controlMediumSize: CGFloat {
    get { storage.controlMediumSize }
    set {
      ensureUnique()
      storage.controlMediumSize = newValue
    }
  }
  public var controlLargeSize: CGFloat {
    get { storage.controlLargeSize }
    set {
      ensureUnique()
      storage.controlLargeSize = newValue
    }
  }
  public var shadow2xs: [TokenShadow] {
    get { storage.shadow2xs }
    set {
      ensureUnique()
      storage.shadow2xs = newValue
    }
  }
  public var shadowXs: [TokenShadow] {
    get { storage.shadowXs }
    set {
      ensureUnique()
      storage.shadowXs = newValue
    }
  }
  public var shadowSm: [TokenShadow] {
    get { storage.shadowSm }
    set {
      ensureUnique()
      storage.shadowSm = newValue
    }
  }
  public var shadowMd: [TokenShadow] {
    get { storage.shadowMd }
    set {
      ensureUnique()
      storage.shadowMd = newValue
    }
  }
  public var shadowLg: [TokenShadow] {
    get { storage.shadowLg }
    set {
      ensureUnique()
      storage.shadowLg = newValue
    }
  }
  public var shadowXl: [TokenShadow] {
    get { storage.shadowXl }
    set {
      ensureUnique()
      storage.shadowXl = newValue
    }
  }
  public var shadow2xl: [TokenShadow] {
    get { storage.shadow2xl }
    set {
      ensureUnique()
      storage.shadow2xl = newValue
    }
  }
  public var focusWidth: CGFloat {
    get { storage.focusWidth }
    set {
      ensureUnique()
      storage.focusWidth = newValue
    }
  }
  public var focusOffset: CGFloat {
    get { storage.focusOffset }
    set {
      ensureUnique()
      storage.focusOffset = newValue
    }
  }
  public var focusGlowShade: Int {
    get { storage.focusGlowShade }
    set {
      ensureUnique()
      storage.focusGlowShade = newValue
    }
  }
  public var focusGlowAlpha: CGFloat {
    get { storage.focusGlowAlpha }
    set {
      ensureUnique()
      storage.focusGlowAlpha = newValue
    }
  }
  public var focusRingShade: Int {
    get { storage.focusRingShade }
    set {
      ensureUnique()
      storage.focusRingShade = newValue
    }
  }
  public var focusRingAlpha: CGFloat {
    get { storage.focusRingAlpha }
    set {
      ensureUnique()
      storage.focusRingAlpha = newValue
    }
  }
  public var frameWindowRadius: CGFloat {
    get { storage.frameWindowRadius }
    set {
      ensureUnique()
      storage.frameWindowRadius = newValue
    }
  }
  public var frameWindowWidth: CGFloat {
    get { storage.frameWindowWidth }
    set {
      ensureUnique()
      storage.frameWindowWidth = newValue
    }
  }
  public var framePopoverRadius: CGFloat {
    get { storage.framePopoverRadius }
    set {
      ensureUnique()
      storage.framePopoverRadius = newValue
    }
  }
  public var frameTitlebarSize: CGFloat {
    get { storage.frameTitlebarSize }
    set {
      ensureUnique()
      storage.frameTitlebarSize = newValue
    }
  }
  public var frameSidebarWidth: CGFloat {
    get { storage.frameSidebarWidth }
    set {
      ensureUnique()
      storage.frameSidebarWidth = newValue
    }
  }
  public var frameSidebarIconWidth: CGFloat {
    get { storage.frameSidebarIconWidth }
    set {
      ensureUnique()
      storage.frameSidebarIconWidth = newValue
    }
  }
  public var frameRailWidth: CGFloat {
    get { storage.frameRailWidth }
    set {
      ensureUnique()
      storage.frameRailWidth = newValue
    }
  }
  public var frameAsideWidth: CGFloat {
    get { storage.frameAsideWidth }
    set {
      ensureUnique()
      storage.frameAsideWidth = newValue
    }
  }
  public var frameNavGap: CGFloat {
    get { storage.frameNavGap }
    set {
      ensureUnique()
      storage.frameNavGap = newValue
    }
  }
  public var iconSmall: CGFloat {
    get { storage.iconSmall }
    set {
      ensureUnique()
      storage.iconSmall = newValue
    }
  }
  public var iconMedium: CGFloat {
    get { storage.iconMedium }
    set {
      ensureUnique()
      storage.iconMedium = newValue
    }
  }
  public var iconLarge: CGFloat {
    get { storage.iconLarge }
    set {
      ensureUnique()
      storage.iconLarge = newValue
    }
  }
  /// Finish decorative motion once under the reduced-motion clamp.
  public var motionReducedDuration: Double {
    get { storage.motionReducedDuration }
    set {
      ensureUnique()
      storage.motionReducedDuration = newValue
    }
  }
  public var motionDuration: Double {
    get { storage.motionDuration }
    set {
      ensureUnique()
      storage.motionDuration = newValue
    }
  }
  public var motionEasing: BezierCurve {
    get { storage.motionEasing }
    set {
      ensureUnique()
      storage.motionEasing = newValue
    }
  }
  /// CSS ease-in-out for the loading sweeps.
  public var motionLoopEasing: BezierCurve {
    get { storage.motionLoopEasing }
    set {
      ensureUnique()
      storage.motionLoopEasing = newValue
    }
  }
  public var radiusNone: CGFloat {
    get { storage.radiusNone }
    set {
      ensureUnique()
      storage.radiusNone = newValue
    }
  }
  public var radiusTiny: CGFloat {
    get { storage.radiusTiny }
    set {
      ensureUnique()
      storage.radiusTiny = newValue
    }
  }
  public var radiusSmall: CGFloat {
    get { storage.radiusSmall }
    set {
      ensureUnique()
      storage.radiusSmall = newValue
    }
  }
  public var radiusMedium: CGFloat {
    get { storage.radiusMedium }
    set {
      ensureUnique()
      storage.radiusMedium = newValue
    }
  }
  public var radiusLarge: CGFloat {
    get { storage.radiusLarge }
    set {
      ensureUnique()
      storage.radiusLarge = newValue
    }
  }
  public var radiusBig: CGFloat {
    get { storage.radiusBig }
    set {
      ensureUnique()
      storage.radiusBig = newValue
    }
  }
  public var radiusFull: CGFloat {
    get { storage.radiusFull }
    set {
      ensureUnique()
      storage.radiusFull = newValue
    }
  }
  public var spacingPx: CGFloat {
    get { storage.spacingPx }
    set {
      ensureUnique()
      storage.spacingPx = newValue
    }
  }
  public var spacing0: CGFloat {
    get { storage.spacing0 }
    set {
      ensureUnique()
      storage.spacing0 = newValue
    }
  }
  public var spacing05: CGFloat {
    get { storage.spacing05 }
    set {
      ensureUnique()
      storage.spacing05 = newValue
    }
  }
  public var spacing1: CGFloat {
    get { storage.spacing1 }
    set {
      ensureUnique()
      storage.spacing1 = newValue
    }
  }
  public var spacing15: CGFloat {
    get { storage.spacing15 }
    set {
      ensureUnique()
      storage.spacing15 = newValue
    }
  }
  public var spacing2: CGFloat {
    get { storage.spacing2 }
    set {
      ensureUnique()
      storage.spacing2 = newValue
    }
  }
  public var spacing25: CGFloat {
    get { storage.spacing25 }
    set {
      ensureUnique()
      storage.spacing25 = newValue
    }
  }
  public var spacing3: CGFloat {
    get { storage.spacing3 }
    set {
      ensureUnique()
      storage.spacing3 = newValue
    }
  }
  public var spacing35: CGFloat {
    get { storage.spacing35 }
    set {
      ensureUnique()
      storage.spacing35 = newValue
    }
  }
  public var spacing4: CGFloat {
    get { storage.spacing4 }
    set {
      ensureUnique()
      storage.spacing4 = newValue
    }
  }
  public var spacing5: CGFloat {
    get { storage.spacing5 }
    set {
      ensureUnique()
      storage.spacing5 = newValue
    }
  }
  public var spacing6: CGFloat {
    get { storage.spacing6 }
    set {
      ensureUnique()
      storage.spacing6 = newValue
    }
  }
  public var spacing7: CGFloat {
    get { storage.spacing7 }
    set {
      ensureUnique()
      storage.spacing7 = newValue
    }
  }
  public var spacing8: CGFloat {
    get { storage.spacing8 }
    set {
      ensureUnique()
      storage.spacing8 = newValue
    }
  }
  public var spacing9: CGFloat {
    get { storage.spacing9 }
    set {
      ensureUnique()
      storage.spacing9 = newValue
    }
  }
  public var spacing10: CGFloat {
    get { storage.spacing10 }
    set {
      ensureUnique()
      storage.spacing10 = newValue
    }
  }
  public var spacing11: CGFloat {
    get { storage.spacing11 }
    set {
      ensureUnique()
      storage.spacing11 = newValue
    }
  }
  public var spacing12: CGFloat {
    get { storage.spacing12 }
    set {
      ensureUnique()
      storage.spacing12 = newValue
    }
  }
  public var spacing14: CGFloat {
    get { storage.spacing14 }
    set {
      ensureUnique()
      storage.spacing14 = newValue
    }
  }
  public var spacing16: CGFloat {
    get { storage.spacing16 }
    set {
      ensureUnique()
      storage.spacing16 = newValue
    }
  }
  public var spacing20: CGFloat {
    get { storage.spacing20 }
    set {
      ensureUnique()
      storage.spacing20 = newValue
    }
  }
  public var strokeHairline: CGFloat {
    get { storage.strokeHairline }
    set {
      ensureUnique()
      storage.strokeHairline = newValue
    }
  }
  public var strokeControl: CGFloat {
    get { storage.strokeControl }
    set {
      ensureUnique()
      storage.strokeControl = newValue
    }
  }
  public var fontUi: [String] {
    get { storage.fontUi }
    set {
      ensureUnique()
      storage.fontUi = newValue
    }
  }
  public var fontDisplay: [String] {
    get { storage.fontDisplay }
    set {
      ensureUnique()
      storage.fontDisplay = newValue
    }
  }
  public var fontCode: [String] {
    get { storage.fontCode }
    set {
      ensureUnique()
      storage.fontCode = newValue
    }
  }
  public var headlineSmall: TypeStyle {
    get { storage.headlineSmall }
    set {
      ensureUnique()
      storage.headlineSmall = newValue
    }
  }
  public var headlineMedium: TypeStyle {
    get { storage.headlineMedium }
    set {
      ensureUnique()
      storage.headlineMedium = newValue
    }
  }
  public var headlineLarge: TypeStyle {
    get { storage.headlineLarge }
    set {
      ensureUnique()
      storage.headlineLarge = newValue
    }
  }
  public var titleSmall: TypeStyle {
    get { storage.titleSmall }
    set {
      ensureUnique()
      storage.titleSmall = newValue
    }
  }
  public var titleMedium: TypeStyle {
    get { storage.titleMedium }
    set {
      ensureUnique()
      storage.titleMedium = newValue
    }
  }
  public var titleLarge: TypeStyle {
    get { storage.titleLarge }
    set {
      ensureUnique()
      storage.titleLarge = newValue
    }
  }
  public var bodySmall: TypeStyle {
    get { storage.bodySmall }
    set {
      ensureUnique()
      storage.bodySmall = newValue
    }
  }
  public var bodyMedium: TypeStyle {
    get { storage.bodyMedium }
    set {
      ensureUnique()
      storage.bodyMedium = newValue
    }
  }
  public var bodyLarge: TypeStyle {
    get { storage.bodyLarge }
    set {
      ensureUnique()
      storage.bodyLarge = newValue
    }
  }
  public var labelQuiet: TypeStyle {
    get { storage.labelQuiet }
    set {
      ensureUnique()
      storage.labelQuiet = newValue
    }
  }
  public var labelStrong: TypeStyle {
    get { storage.labelStrong }
    set {
      ensureUnique()
      storage.labelStrong = newValue
    }
  }
  public var labelSmall: TypeStyle {
    get { storage.labelSmall }
    set {
      ensureUnique()
      storage.labelSmall = newValue
    }
  }
  public var labelMedium: TypeStyle {
    get { storage.labelMedium }
    set {
      ensureUnique()
      storage.labelMedium = newValue
    }
  }
  public var labelLarge: TypeStyle {
    get { storage.labelLarge }
    set {
      ensureUnique()
      storage.labelLarge = newValue
    }
  }
  public var captionSmall: TypeStyle {
    get { storage.captionSmall }
    set {
      ensureUnique()
      storage.captionSmall = newValue
    }
  }
  public var captionMedium: TypeStyle {
    get { storage.captionMedium }
    set {
      ensureUnique()
      storage.captionMedium = newValue
    }
  }
  public var captionLarge: TypeStyle {
    get { storage.captionLarge }
    set {
      ensureUnique()
      storage.captionLarge = newValue
    }
  }
  public var washSurface: CGFloat {
    get { storage.washSurface }
    set {
      ensureUnique()
      storage.washSurface = newValue
    }
  }
  public var washEdge: CGFloat {
    get { storage.washEdge }
    set {
      ensureUnique()
      storage.washEdge = newValue
    }
  }
  public var calendarCellGap: CGFloat {
    get { storage.calendarCellGap }
    set {
      ensureUnique()
      storage.calendarCellGap = newValue
    }
  }
  public var checkboxRadius: CGFloat {
    get { storage.checkboxRadius }
    set {
      ensureUnique()
      storage.checkboxRadius = newValue
    }
  }
  /// How long the copy action shows its successful state.
  public var codeBlockCopyFeedbackDuration: Double {
    get { storage.codeBlockCopyFeedbackDuration }
    set {
      ensureUnique()
      storage.codeBlockCopyFeedbackDuration = newValue
    }
  }
  public var dateRangePickerSideWidth: CGFloat {
    get { storage.dateRangePickerSideWidth }
    set {
      ensureUnique()
      storage.dateRangePickerSideWidth = newValue
    }
  }
  public var dateRangePickerTriggerMinWidth: CGFloat {
    get { storage.dateRangePickerTriggerMinWidth }
    set {
      ensureUnique()
      storage.dateRangePickerTriggerMinWidth = newValue
    }
  }
  public var dateRangePickerTimeMinWidth: CGFloat {
    get { storage.dateRangePickerTimeMinWidth }
    set {
      ensureUnique()
      storage.dateRangePickerTimeMinWidth = newValue
    }
  }
  public var dateRangePickerKeyTracking: CGFloat {
    get { storage.dateRangePickerKeyTracking }
    set {
      ensureUnique()
      storage.dateRangePickerKeyTracking = newValue
    }
  }
  public var dialogTransitionScale: CGFloat {
    get { storage.dialogTransitionScale }
    set {
      ensureUnique()
      storage.dialogTransitionScale = newValue
    }
  }
  public var dialogWidth: CGFloat {
    get { storage.dialogWidth }
    set {
      ensureUnique()
      storage.dialogWidth = newValue
    }
  }
  public var dialogScrimAlpha: CGFloat {
    get { storage.dialogScrimAlpha }
    set {
      ensureUnique()
      storage.dialogScrimAlpha = newValue
    }
  }
  public var dialogDangerBorderAlpha: CGFloat {
    get { storage.dialogDangerBorderAlpha }
    set {
      ensureUnique()
      storage.dialogDangerBorderAlpha = newValue
    }
  }
  public var drawerSize: CGFloat {
    get { storage.drawerSize }
    set {
      ensureUnique()
      storage.drawerSize = newValue
    }
  }
  public var spinnerDuration: Double {
    get { storage.spinnerDuration }
    set {
      ensureUnique()
      storage.spinnerDuration = newValue
    }
  }
  public var spinnerReducedDuration: Double {
    get { storage.spinnerReducedDuration }
    set {
      ensureUnique()
      storage.spinnerReducedDuration = newValue
    }
  }
  public var spinnerTrackAlpha: CGFloat {
    get { storage.spinnerTrackAlpha }
    set {
      ensureUnique()
      storage.spinnerTrackAlpha = newValue
    }
  }
  public var spinnerOnAccentTrackAlpha: CGFloat {
    get { storage.spinnerOnAccentTrackAlpha }
    set {
      ensureUnique()
      storage.spinnerOnAccentTrackAlpha = newValue
    }
  }
  public var menuTypeaheadResetDuration: Double {
    get { storage.menuTypeaheadResetDuration }
    set {
      ensureUnique()
      storage.menuTypeaheadResetDuration = newValue
    }
  }
  public var menuMinWidth: CGFloat {
    get { storage.menuMinWidth }
    set {
      ensureUnique()
      storage.menuMinWidth = newValue
    }
  }
  public var menuItemPadding: CGFloat {
    get { storage.menuItemPadding }
    set {
      ensureUnique()
      storage.menuItemPadding = newValue
    }
  }
  public var preferencesWidth: CGFloat {
    get { storage.preferencesWidth }
    set {
      ensureUnique()
      storage.preferencesWidth = newValue
    }
  }
  public var previewCardOpenDelay: Double {
    get { storage.previewCardOpenDelay }
    set {
      ensureUnique()
      storage.previewCardOpenDelay = newValue
    }
  }
  public var previewCardCloseDelay: Double {
    get { storage.previewCardCloseDelay }
    set {
      ensureUnique()
      storage.previewCardCloseDelay = newValue
    }
  }
  public var previewCardWidth: CGFloat {
    get { storage.previewCardWidth }
    set {
      ensureUnique()
      storage.previewCardWidth = newValue
    }
  }
  public var progressSweepDuration: Double {
    get { storage.progressSweepDuration }
    set {
      ensureUnique()
      storage.progressSweepDuration = newValue
    }
  }
  public var progressReducedSweepDuration: Double {
    get { storage.progressReducedSweepDuration }
    set {
      ensureUnique()
      storage.progressReducedSweepDuration = newValue
    }
  }
  public var progressSweepWidth: CGFloat {
    get { storage.progressSweepWidth }
    set {
      ensureUnique()
      storage.progressSweepWidth = newValue
    }
  }
  public var progressGradientFrom: Color {
    get { storage.progressGradientFrom }
    set {
      ensureUnique()
      storage.progressGradientFrom = newValue
    }
  }
  public var progressGradientTo: Color {
    get { storage.progressGradientTo }
    set {
      ensureUnique()
      storage.progressGradientTo = newValue
    }
  }
  public var searchFieldInputWidth: CGFloat {
    get { storage.searchFieldInputWidth }
    set {
      ensureUnique()
      storage.searchFieldInputWidth = newValue
    }
  }
  public var segmentedControlInset: CGFloat {
    get { storage.segmentedControlInset }
    set {
      ensureUnique()
      storage.segmentedControlInset = newValue
    }
  }
  public var shortcutRecorderWidth: CGFloat {
    get { storage.shortcutRecorderWidth }
    set {
      ensureUnique()
      storage.shortcutRecorderWidth = newValue
    }
  }
  public var shortcutRecorderRejectDuration: Double {
    get { storage.shortcutRecorderRejectDuration }
    set {
      ensureUnique()
      storage.shortcutRecorderRejectDuration = newValue
    }
  }
  public var sidebarGroupLabelHeight: CGFloat {
    get { storage.sidebarGroupLabelHeight }
    set {
      ensureUnique()
      storage.sidebarGroupLabelHeight = newValue
    }
  }
  /// A slow sweep behind content that has not arrived.
  public var skeletonSweepDuration: Double {
    get { storage.skeletonSweepDuration }
    set {
      ensureUnique()
      storage.skeletonSweepDuration = newValue
    }
  }
  /// The final paragraph bar occupies three fifths of its measure.
  public var skeletonLastLineWidth: CGFloat {
    get { storage.skeletonLastLineWidth }
    set {
      ensureUnique()
      storage.skeletonLastLineWidth = newValue
    }
  }
  public var sliderMediumTrack: CGFloat {
    get { storage.sliderMediumTrack }
    set {
      ensureUnique()
      storage.sliderMediumTrack = newValue
    }
  }
  public var sliderMediumThumb: CGFloat {
    get { storage.sliderMediumThumb }
    set {
      ensureUnique()
      storage.sliderMediumThumb = newValue
    }
  }
  public var sliderThumbInk: Int {
    get { storage.sliderThumbInk }
    set {
      ensureUnique()
      storage.sliderThumbInk = newValue
    }
  }
  public var sliderThumbLift: Int {
    get { storage.sliderThumbLift }
    set {
      ensureUnique()
      storage.sliderThumbLift = newValue
    }
  }
  public var sliderThumbEdgeWidth: CGFloat {
    get { storage.sliderThumbEdgeWidth }
    set {
      ensureUnique()
      storage.sliderThumbEdgeWidth = newValue
    }
  }
  public var sliderThumbHotScale: CGFloat {
    get { storage.sliderThumbHotScale }
    set {
      ensureUnique()
      storage.sliderThumbHotScale = newValue
    }
  }
  public var splitterKeyStep: Int {
    get { storage.splitterKeyStep }
    set {
      ensureUnique()
      storage.splitterKeyStep = newValue
    }
  }
  public var splitterKeyCoarseStep: Int {
    get { storage.splitterKeyCoarseStep }
    set {
      ensureUnique()
      storage.splitterKeyCoarseStep = newValue
    }
  }
  /// The success wash behind a completed step's marker.
  public var stepListDoneAlpha: CGFloat {
    get { storage.stepListDoneAlpha }
    set {
      ensureUnique()
      storage.stepListDoneAlpha = newValue
    }
  }
  /// The opacity of a step that has not started.
  public var stepListIdleOpacity: CGFloat {
    get { storage.stepListIdleOpacity }
    set {
      ensureUnique()
      storage.stepListIdleOpacity = newValue
    }
  }
  public var switchMediumWidth: CGFloat {
    get { storage.switchMediumWidth }
    set {
      ensureUnique()
      storage.switchMediumWidth = newValue
    }
  }
  public var switchMediumHeight: CGFloat {
    get { storage.switchMediumHeight }
    set {
      ensureUnique()
      storage.switchMediumHeight = newValue
    }
  }
  public var switchMediumThumb: CGFloat {
    get { storage.switchMediumThumb }
    set {
      ensureUnique()
      storage.switchMediumThumb = newValue
    }
  }
  public var tabsPulseDuration: Double {
    get { storage.tabsPulseDuration }
    set {
      ensureUnique()
      storage.tabsPulseDuration = newValue
    }
  }
  public var tabsPulseOpacity: CGFloat {
    get { storage.tabsPulseOpacity }
    set {
      ensureUnique()
      storage.tabsPulseOpacity = newValue
    }
  }
  public var badgeLineHeight: CGFloat {
    get { storage.badgeLineHeight }
    set {
      ensureUnique()
      storage.badgeLineHeight = newValue
    }
  }
  public var calloutTitleLeading: CGFloat {
    get { storage.calloutTitleLeading }
    set {
      ensureUnique()
      storage.calloutTitleLeading = newValue
    }
  }
  public var calloutDescriptionLeading: CGFloat {
    get { storage.calloutDescriptionLeading }
    set {
      ensureUnique()
      storage.calloutDescriptionLeading = newValue
    }
  }
  public var textFieldMultilineLeading: CGFloat {
    get { storage.textFieldMultilineLeading }
    set {
      ensureUnique()
      storage.textFieldMultilineLeading = newValue
    }
  }
  public var optionCardDescriptionLeading: CGFloat {
    get { storage.optionCardDescriptionLeading }
    set {
      ensureUnique()
      storage.optionCardDescriptionLeading = newValue
    }
  }
  public var formFieldHintLeading: CGFloat {
    get { storage.formFieldHintLeading }
    set {
      ensureUnique()
      storage.formFieldHintLeading = newValue
    }
  }
  public var preferencesDescriptionLeading: CGFloat {
    get { storage.preferencesDescriptionLeading }
    set {
      ensureUnique()
      storage.preferencesDescriptionLeading = newValue
    }
  }
  public var toastMessageLineHeight: CGFloat {
    get { storage.toastMessageLineHeight }
    set {
      ensureUnique()
      storage.toastMessageLineHeight = newValue
    }
  }
  public var toastNoticeDuration: Double {
    get { storage.toastNoticeDuration }
    set {
      ensureUnique()
      storage.toastNoticeDuration = newValue
    }
  }
  public var toastActionDuration: Double {
    get { storage.toastActionDuration }
    set {
      ensureUnique()
      storage.toastActionDuration = newValue
    }
  }
  public var toastTransitionDuration: Double {
    get { storage.toastTransitionDuration }
    set {
      ensureUnique()
      storage.toastTransitionDuration = newValue
    }
  }
  public var toastTransitionX1: CGFloat {
    get { storage.toastTransitionX1 }
    set {
      ensureUnique()
      storage.toastTransitionX1 = newValue
    }
  }
  public var toastTransitionY1: CGFloat {
    get { storage.toastTransitionY1 }
    set {
      ensureUnique()
      storage.toastTransitionY1 = newValue
    }
  }
  public var toastTransitionX2: CGFloat {
    get { storage.toastTransitionX2 }
    set {
      ensureUnique()
      storage.toastTransitionX2 = newValue
    }
  }
  public var toastTransitionY2: CGFloat {
    get { storage.toastTransitionY2 }
    set {
      ensureUnique()
      storage.toastTransitionY2 = newValue
    }
  }
  public var toastTransitionScale: CGFloat {
    get { storage.toastTransitionScale }
    set {
      ensureUnique()
      storage.toastTransitionScale = newValue
    }
  }
  public var toastMaxWidth: CGFloat {
    get { storage.toastMaxWidth }
    set {
      ensureUnique()
      storage.toastMaxWidth = newValue
    }
  }
  public var tooltipOpenDelay: Double {
    get { storage.tooltipOpenDelay }
    set {
      ensureUnique()
      storage.tooltipOpenDelay = newValue
    }
  }
  public var tooltipCloseDelay: Double {
    get { storage.tooltipCloseDelay }
    set {
      ensureUnique()
      storage.tooltipCloseDelay = newValue
    }
  }
  public var treeTypeaheadWindow: Double {
    get { storage.treeTypeaheadWindow }
    set {
      ensureUnique()
      storage.treeTypeaheadWindow = newValue
    }
  }
  public var controlColorRecessedBorder: ColorDescriptor {
    get { storage.controlColorRecessedBorder }
    set {
      ensureUnique()
      storage.controlColorRecessedBorder = newValue
    }
  }
  /// The ramp step `controlColorFilledSurface` picks when normal; a theme re-points it.
  public var controlColorFilledSurfaceNormalShade: Int {
    get { storage.controlColorFilledSurfaceNormalShade }
    set {
      ensureUnique()
      storage.controlColorFilledSurfaceNormalShade = newValue
    }
  }
  /// The ramp step `controlColorFilledSurface` picks when hovered; a theme re-points it.
  public var controlColorFilledSurfaceHoveredShade: Int {
    get { storage.controlColorFilledSurfaceHoveredShade }
    set {
      ensureUnique()
      storage.controlColorFilledSurfaceHoveredShade = newValue
    }
  }
  /// The ramp step `controlColorFilledSurface` picks when pressed; a theme re-points it.
  public var controlColorFilledSurfacePressedShade: Int {
    get { storage.controlColorFilledSurfacePressedShade }
    set {
      ensureUnique()
      storage.controlColorFilledSurfacePressedShade = newValue
    }
  }
  public var controlColorFilledBorder: ColorDescriptor {
    get { storage.controlColorFilledBorder }
    set {
      ensureUnique()
      storage.controlColorFilledBorder = newValue
    }
  }
  /// The ramp step `controlColorTintedContent` picks when normal; a theme re-points it.
  public var controlColorTintedContentNormalShade: Int {
    get { storage.controlColorTintedContentNormalShade }
    set {
      ensureUnique()
      storage.controlColorTintedContentNormalShade = newValue
    }
  }
  /// The ramp step `controlColorTintedContent` picks when hovered; a theme re-points it.
  public var controlColorTintedContentHoveredShade: Int {
    get { storage.controlColorTintedContentHoveredShade }
    set {
      ensureUnique()
      storage.controlColorTintedContentHoveredShade = newValue
    }
  }
  /// The ramp step `controlColorTintedContent` picks when pressed; a theme re-points it.
  public var controlColorTintedContentPressedShade: Int {
    get { storage.controlColorTintedContentPressedShade }
    set {
      ensureUnique()
      storage.controlColorTintedContentPressedShade = newValue
    }
  }
  public var controlColorTintedBorder: ColorDescriptor {
    get { storage.controlColorTintedBorder }
    set {
      ensureUnique()
      storage.controlColorTintedBorder = newValue
    }
  }
  /// The ramp step `controlColorOutlinedContent` picks when normal; a theme re-points it.
  public var controlColorOutlinedContentNormalShade: Int {
    get { storage.controlColorOutlinedContentNormalShade }
    set {
      ensureUnique()
      storage.controlColorOutlinedContentNormalShade = newValue
    }
  }
  /// The ramp step `controlColorOutlinedContent` picks when hovered; a theme re-points it.
  public var controlColorOutlinedContentHoveredShade: Int {
    get { storage.controlColorOutlinedContentHoveredShade }
    set {
      ensureUnique()
      storage.controlColorOutlinedContentHoveredShade = newValue
    }
  }
  /// The ramp step `controlColorOutlinedContent` picks when pressed; a theme re-points it.
  public var controlColorOutlinedContentPressedShade: Int {
    get { storage.controlColorOutlinedContentPressedShade }
    set {
      ensureUnique()
      storage.controlColorOutlinedContentPressedShade = newValue
    }
  }
  public var controlColorOutlinedBorder: ColorDescriptor {
    get { storage.controlColorOutlinedBorder }
    set {
      ensureUnique()
      storage.controlColorOutlinedBorder = newValue
    }
  }
  public var controlColorPlainSurface: ColorDescriptor {
    get { storage.controlColorPlainSurface }
    set {
      ensureUnique()
      storage.controlColorPlainSurface = newValue
    }
  }
  /// The ramp step `controlColorPlainContent` picks when normal; a theme re-points it.
  public var controlColorPlainContentNormalShade: Int {
    get { storage.controlColorPlainContentNormalShade }
    set {
      ensureUnique()
      storage.controlColorPlainContentNormalShade = newValue
    }
  }
  /// The ramp step `controlColorPlainContent` picks when hovered; a theme re-points it.
  public var controlColorPlainContentHoveredShade: Int {
    get { storage.controlColorPlainContentHoveredShade }
    set {
      ensureUnique()
      storage.controlColorPlainContentHoveredShade = newValue
    }
  }
  /// The ramp step `controlColorPlainContent` picks when pressed; a theme re-points it.
  public var controlColorPlainContentPressedShade: Int {
    get { storage.controlColorPlainContentPressedShade }
    set {
      ensureUnique()
      storage.controlColorPlainContentPressedShade = newValue
    }
  }
  public var controlColorPlainBorder: ColorDescriptor {
    get { storage.controlColorPlainBorder }
    set {
      ensureUnique()
      storage.controlColorPlainBorder = newValue
    }
  }

  public var controlLabelGlyphShrink: CGFloat { spacing05 }
  public var controlLabelGlyphInset: CGFloat { spacing05 }
  public var controlTinyIconSize: CGFloat { iconSmall }
  public var controlTinyGap: CGFloat { spacing1 }
  public var controlTinyPaddingBlock: CGFloat { spacing1 }
  public var controlTinyPaddingInline: CGFloat { spacing25 }
  public var controlTinyRadius: CGFloat { radiusSmall }
  public var controlTinyContent: TypeStyle { systemSizing?.content(.tiny, design: labelSmall) ?? labelSmall }
  public var controlSmallIconSize: CGFloat { iconMedium }
  public var controlSmallGap: CGFloat { spacing15 }
  public var controlSmallPaddingBlock: CGFloat { spacing1 }
  public var controlSmallPaddingInline: CGFloat { spacing3 }
  public var controlSmallRadius: CGFloat { radiusSmall }
  public var controlSmallContent: TypeStyle { systemSizing?.content(.small, design: labelMedium) ?? labelMedium }
  public var controlMediumIconSize: CGFloat { iconMedium }
  public var controlMediumGap: CGFloat { spacing2 }
  public var controlMediumPaddingBlock: CGFloat { spacing15 }
  public var controlMediumRadius: CGFloat { radiusSmall }
  public var controlMediumContent: TypeStyle { systemSizing?.content(.medium, design: labelMedium) ?? labelMedium }
  public var controlLargeIconSize: CGFloat { iconLarge }
  public var controlLargeGap: CGFloat { spacing2 }
  public var controlLargePaddingBlock: CGFloat { spacing15 }
  public var controlLargePaddingInline: CGFloat { spacing4 }
  public var controlLargeRadius: CGFloat { radiusMedium }
  public var controlLargeContent: TypeStyle { systemSizing?.content(.large, design: labelMedium) ?? labelMedium }
  public var calendarMediumCellSize: CGFloat { controlMediumSize }
  public var calendarSmallCellSize: CGFloat { controlSmallSize }
  public var checkboxThickness: CGFloat { strokeControl }
  public var checkboxSmallBox: CGFloat { spacing35 }
  public var checkboxMediumBox: CGFloat { spacing4 }
  public var checkboxLargeBox: CGFloat { spacing5 }
  public var dialogScrimColor: Color { colorContent }
  public var iconButtonTinyIconSize: CGFloat { spacing4 }
  public var preferencesRowPadding: CGFloat { spacing15 }
  public var preferencesRowGap: CGFloat { spacing1 }
  public var preferencesHeadingGap: CGFloat { spacing2 }
  public var preferencesFooterGap: CGFloat { spacing1 }
  public var preferencesTitleGap: CGFloat { spacing4 }
  public var preferencesSectionGap: CGFloat { spacing6 }
  public var preferencesGroupGap: CGFloat { spacing10 }
  public var radioThickness: CGFloat { strokeControl }
  public var radioSmallBox: CGFloat { spacing35 }
  public var radioSmallDot: CGFloat { spacing15 }
  public var radioMediumBox: CGFloat { spacing4 }
  public var radioMediumDot: CGFloat { spacing2 }
  public var radioLargeBox: CGFloat { spacing5 }
  public var radioLargeDot: CGFloat { spacing25 }
  public var sliderSmallTrack: CGFloat { spacing1 }
  public var sliderSmallThumb: CGFloat { spacing35 }
  public var sliderLargeTrack: CGFloat { spacing2 }
  public var sliderLargeThumb: CGFloat { spacing5 }
  public var switchSmallWidth: CGFloat { spacing7 }
  public var switchSmallHeight: CGFloat { spacing4 }
  public var switchSmallThumb: CGFloat { spacing3 }
  public var switchLargeWidth: CGFloat { spacing11 }
  public var switchLargeHeight: CGFloat { spacing6 }
  public var switchLargeThumb: CGFloat { spacing5 }
  public var tabsIconSize: CGFloat { spacing35 }
  public var tabsCloseIconSize: CGFloat { spacing3 }
  public var toastTransitionOffset: CGFloat { spacing25 }
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

  /// Resolves runtime system metrics without changing palette or recipe data.
  internal func resolvingSystemSizing(_ metrics: SystemSizingMetrics) -> ThemeVariables {
    let scale = metrics.bodyFontSize / bodyMedium.size
    var result = ThemeVariables(
      colorPrimary: colorPrimary,
      colorNeutral: colorNeutral,
      colorInfo: colorInfo,
      colorSuccess: colorSuccess,
      colorWarning: colorWarning,
      colorDanger: colorDanger,
      colorCanvas: colorCanvas,
      colorSurface: colorSurface,
      colorSurfaceMuted: colorSurfaceMuted,
      colorSurfaceSunken: colorSurfaceSunken,
      colorSurfaceSubtle: colorSurfaceSubtle,
      colorSurfaceInset: colorSurfaceInset,
      colorSurfaceRaised: colorSurfaceRaised,
      colorSurfaceOverlay: colorSurfaceOverlay,
      colorSurfaceChrome: colorSurfaceChrome,
      colorSurfaceColumn: colorSurfaceColumn,
      colorContent: colorContent,
      colorContentSecondary: colorContentSecondary,
      colorContentNav: colorContentNav,
      colorContentMuted: colorContentMuted,
      colorContentSubtle: colorContentSubtle,
      colorContentFaint: colorContentFaint,
      colorBorder: colorBorder,
      colorBorderStrong: colorBorderStrong,
      colorBorderMuted: colorBorderMuted,
      colorOnAccent: colorOnAccent,
      controlPressedAlpha: controlPressedAlpha,
      controlFieldRadius: controlFieldRadius,
      controlContainerRadius: controlContainerRadius,
      controlTinySize: metrics.control(.tiny).height,
      controlSmallSize: metrics.control(.small).height,
      controlMediumPaddingInline: controlMediumPaddingInline * scale,
      controlMediumSize: metrics.control(.medium).height,
      controlLargeSize: metrics.control(.large).height,
      shadow2xs: shadow2xs,
      shadowXs: shadowXs,
      shadowSm: shadowSm,
      shadowMd: shadowMd,
      shadowLg: shadowLg,
      shadowXl: shadowXl,
      shadow2xl: shadow2xl,
      focusWidth: focusWidth,
      focusOffset: focusOffset,
      focusGlowShade: focusGlowShade,
      focusGlowAlpha: focusGlowAlpha,
      focusRingShade: focusRingShade,
      focusRingAlpha: focusRingAlpha,
      frameWindowRadius: frameWindowRadius,
      frameWindowWidth: frameWindowWidth * scale,
      framePopoverRadius: framePopoverRadius,
      frameTitlebarSize: frameTitlebarSize * scale,
      frameSidebarWidth: frameSidebarWidth * scale,
      frameSidebarIconWidth: frameSidebarIconWidth * scale,
      frameRailWidth: frameRailWidth * scale,
      frameAsideWidth: frameAsideWidth * scale,
      frameNavGap: frameNavGap * scale,
      iconSmall: iconSmall * scale,
      iconMedium: iconMedium * scale,
      iconLarge: iconLarge * scale,
      motionReducedDuration: motionReducedDuration,
      motionDuration: motionDuration,
      motionEasing: motionEasing,
      motionLoopEasing: motionLoopEasing,
      radiusNone: radiusNone,
      radiusTiny: radiusTiny,
      radiusSmall: radiusSmall,
      radiusMedium: radiusMedium,
      radiusLarge: radiusLarge,
      radiusBig: radiusBig,
      radiusFull: radiusFull,
      spacingPx: spacingPx,
      spacing0: spacing0 * scale,
      spacing05: spacing05 * scale,
      spacing1: spacing1 * scale,
      spacing15: spacing15 * scale,
      spacing2: spacing2 * scale,
      spacing25: spacing25 * scale,
      spacing3: spacing3 * scale,
      spacing35: spacing35 * scale,
      spacing4: spacing4 * scale,
      spacing5: spacing5 * scale,
      spacing6: spacing6 * scale,
      spacing7: spacing7 * scale,
      spacing8: spacing8 * scale,
      spacing9: spacing9 * scale,
      spacing10: spacing10 * scale,
      spacing11: spacing11 * scale,
      spacing12: spacing12 * scale,
      spacing14: spacing14 * scale,
      spacing16: spacing16 * scale,
      spacing20: spacing20 * scale,
      strokeHairline: strokeHairline,
      strokeControl: strokeControl,
      fontUi: fontUi,
      fontDisplay: fontDisplay,
      fontCode: fontCode,
      headlineSmall: headlineSmall.scaled(by: scale),
      headlineMedium: headlineMedium.scaled(by: scale),
      headlineLarge: headlineLarge.scaled(by: scale),
      titleSmall: titleSmall.scaled(by: scale),
      titleMedium: titleMedium.scaled(by: scale),
      titleLarge: titleLarge.scaled(by: scale),
      bodySmall: bodySmall.scaled(by: scale),
      bodyMedium: bodyMedium.scaled(by: scale),
      bodyLarge: bodyLarge.scaled(by: scale),
      labelQuiet: labelQuiet.scaled(by: scale),
      labelStrong: labelStrong.scaled(by: scale),
      labelSmall: labelSmall.scaled(by: scale),
      labelMedium: labelMedium.scaled(by: scale),
      labelLarge: labelLarge.scaled(by: scale),
      captionSmall: captionSmall.scaled(by: scale),
      captionMedium: captionMedium.scaled(by: scale),
      captionLarge: captionLarge.scaled(by: scale),
      washSurface: washSurface,
      washEdge: washEdge,
      calendarCellGap: calendarCellGap * scale,
      checkboxRadius: checkboxRadius,
      codeBlockCopyFeedbackDuration: codeBlockCopyFeedbackDuration,
      dateRangePickerSideWidth: dateRangePickerSideWidth * scale,
      dateRangePickerTriggerMinWidth: dateRangePickerTriggerMinWidth * scale,
      dateRangePickerTimeMinWidth: dateRangePickerTimeMinWidth * scale,
      dateRangePickerKeyTracking: dateRangePickerKeyTracking,
      dialogTransitionScale: dialogTransitionScale,
      dialogWidth: dialogWidth * scale,
      dialogScrimAlpha: dialogScrimAlpha,
      dialogDangerBorderAlpha: dialogDangerBorderAlpha,
      drawerSize: drawerSize * scale,
      spinnerDuration: spinnerDuration,
      spinnerReducedDuration: spinnerReducedDuration,
      spinnerTrackAlpha: spinnerTrackAlpha,
      spinnerOnAccentTrackAlpha: spinnerOnAccentTrackAlpha,
      menuTypeaheadResetDuration: menuTypeaheadResetDuration,
      menuMinWidth: menuMinWidth * scale,
      menuItemPadding: menuItemPadding * scale,
      preferencesWidth: preferencesWidth * scale,
      previewCardOpenDelay: previewCardOpenDelay,
      previewCardCloseDelay: previewCardCloseDelay,
      previewCardWidth: previewCardWidth * scale,
      progressSweepDuration: progressSweepDuration,
      progressReducedSweepDuration: progressReducedSweepDuration,
      progressSweepWidth: progressSweepWidth,
      progressGradientFrom: progressGradientFrom,
      progressGradientTo: progressGradientTo,
      searchFieldInputWidth: searchFieldInputWidth * scale,
      segmentedControlInset: segmentedControlInset * scale,
      shortcutRecorderWidth: shortcutRecorderWidth * scale,
      shortcutRecorderRejectDuration: shortcutRecorderRejectDuration,
      sidebarGroupLabelHeight: sidebarGroupLabelHeight * scale,
      skeletonSweepDuration: skeletonSweepDuration,
      skeletonLastLineWidth: skeletonLastLineWidth,
      sliderMediumTrack: sliderMediumTrack * scale,
      sliderMediumThumb: sliderMediumThumb * scale,
      sliderThumbInk: sliderThumbInk,
      sliderThumbLift: sliderThumbLift,
      sliderThumbEdgeWidth: sliderThumbEdgeWidth,
      sliderThumbHotScale: sliderThumbHotScale,
      splitterKeyStep: splitterKeyStep,
      splitterKeyCoarseStep: splitterKeyCoarseStep,
      stepListDoneAlpha: stepListDoneAlpha,
      stepListIdleOpacity: stepListIdleOpacity,
      switchMediumWidth: switchMediumWidth * scale,
      switchMediumHeight: switchMediumHeight * scale,
      switchMediumThumb: switchMediumThumb * scale,
      tabsPulseDuration: tabsPulseDuration,
      tabsPulseOpacity: tabsPulseOpacity,
      badgeLineHeight: badgeLineHeight,
      calloutTitleLeading: calloutTitleLeading,
      calloutDescriptionLeading: calloutDescriptionLeading,
      textFieldMultilineLeading: textFieldMultilineLeading,
      optionCardDescriptionLeading: optionCardDescriptionLeading,
      formFieldHintLeading: formFieldHintLeading,
      preferencesDescriptionLeading: preferencesDescriptionLeading,
      toastMessageLineHeight: toastMessageLineHeight,
      toastNoticeDuration: toastNoticeDuration,
      toastActionDuration: toastActionDuration,
      toastTransitionDuration: toastTransitionDuration,
      toastTransitionX1: toastTransitionX1,
      toastTransitionY1: toastTransitionY1,
      toastTransitionX2: toastTransitionX2,
      toastTransitionY2: toastTransitionY2,
      toastTransitionScale: toastTransitionScale,
      toastMaxWidth: toastMaxWidth * scale,
      tooltipOpenDelay: tooltipOpenDelay,
      tooltipCloseDelay: tooltipCloseDelay,
      treeTypeaheadWindow: treeTypeaheadWindow,
      controlColorRecessedBorder: controlColorRecessedBorder,
      controlColorFilledSurfaceNormalShade: controlColorFilledSurfaceNormalShade,
      controlColorFilledSurfaceHoveredShade: controlColorFilledSurfaceHoveredShade,
      controlColorFilledSurfacePressedShade: controlColorFilledSurfacePressedShade,
      controlColorFilledBorder: controlColorFilledBorder,
      controlColorTintedContentNormalShade: controlColorTintedContentNormalShade,
      controlColorTintedContentHoveredShade: controlColorTintedContentHoveredShade,
      controlColorTintedContentPressedShade: controlColorTintedContentPressedShade,
      controlColorTintedBorder: controlColorTintedBorder,
      controlColorOutlinedContentNormalShade: controlColorOutlinedContentNormalShade,
      controlColorOutlinedContentHoveredShade: controlColorOutlinedContentHoveredShade,
      controlColorOutlinedContentPressedShade: controlColorOutlinedContentPressedShade,
      controlColorOutlinedBorder: controlColorOutlinedBorder,
      controlColorPlainSurface: controlColorPlainSurface,
      controlColorPlainContentNormalShade: controlColorPlainContentNormalShade,
      controlColorPlainContentHoveredShade: controlColorPlainContentHoveredShade,
      controlColorPlainContentPressedShade: controlColorPlainContentPressedShade,
      controlColorPlainBorder: controlColorPlainBorder
    )
    result.systemSizing = metrics
    return result
  }
}

public extension ThemeVariables {
  /// Bright Light — the default every other theme is written against.
  static let brightLight = ThemeVariables()

  /// The `studio-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let studioLight = ThemeVariables(
    colorPrimary: Colors.brand,
    colorCanvas: Color(argb: 0xFFEE_ECF6),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF7_F7FA),
    colorSurfaceSunken: Color(argb: 0xFFE3_E3EC),
    colorSurfaceSubtle: Color(argb: 0x0F14_162A),
    colorSurfaceInset: Color(argb: 0xFFF0_F0F5),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF7_F7FA),
    colorSurfaceColumn: Color(argb: 0xFFFA_FAFC),
    colorContent: Color(argb: 0xFF12_142A),
    colorContentSecondary: Color(argb: 0xFF3C_405C),
    colorContentNav: Color(argb: 0xFF4A_4F6B),
    colorContentMuted: Color(argb: 0xFF56_5B78),
    colorContentSubtle: Color(argb: 0xFF8C_92AA),
    colorContentFaint: Color(argb: 0xFFA2_A7BD),
    colorBorder: Color(argb: 0x1214_162A),
    colorBorderStrong: Color(argb: 0x1714_162A),
    colorBorderMuted: Color(argb: 0xFF5F_6478),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 10.0,
    controlContainerRadius: 10.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 18.0,
    framePopoverRadius: 16.0,
    radiusSmall: 8.0,
    radiusMedium: 10.0,
    radiusBig: 16.0,
    progressGradientFrom: Colors.brand[600],
    progressGradientTo: Color(argb: 0xFFA0_8CFF)
  )

  /// The `studio-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let studioDark = ThemeVariables(
    colorPrimary: Colors.brand,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0A_0B10),
    colorSurface: Color(argb: 0xFF0D_0F14),
    colorSurfaceMuted: Color(argb: 0xFF12_141A),
    colorSurfaceSunken: Color(argb: 0xFF26_2A34),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF1B_1E26),
    colorSurfaceRaised: Color(argb: 0xFF3A_3F4D),
    colorSurfaceOverlay: Color(argb: 0xFF12_141A),
    colorSurfaceChrome: Color(argb: 0xFF12_141A),
    colorSurfaceColumn: Color(argb: 0xFF0A_0C11),
    colorContent: Color(argb: 0xFFF2_F3FA),
    colorContentSecondary: Color(argb: 0xFFC3_C8DC),
    colorContentNav: Color(argb: 0xFF9A_A1BB),
    colorContentMuted: Color(argb: 0xFF9A_A1BB),
    colorContentSubtle: Color(argb: 0xFF8B_93B0),
    colorContentFaint: Color(argb: 0xFF6A_7090),
    colorBorder: Color(argb: 0x0FFF_FFFF),
    colorBorderStrong: Color(argb: 0x17FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF36_3C4D),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 10.0,
    controlContainerRadius: 10.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 18.0,
    framePopoverRadius: 16.0,
    radiusSmall: 8.0,
    radiusMedium: 10.0,
    radiusBig: 16.0,
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

  /// The `frost-light` counterpart of ``brightLight``.
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
    colorSurfaceSubtle: Color(argb: 0x0F0E_1F26),
    colorSurfaceInset: Color(argb: 0xFFEC_F2F4),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF2_F6F8),
    colorSurfaceColumn: Color(argb: 0xFFF2_F6F8),
    colorContent: Color(argb: 0xFF0E_1F26),
    colorContentSecondary: Color(argb: 0xFF2F_4C58),
    colorContentNav: Color(argb: 0xFF3A_5B68),
    colorContentMuted: Color(argb: 0xFF45_6573),
    colorContentSubtle: Color(argb: 0xFF77_94A0),
    colorContentFaint: Color(argb: 0xFF9A_B1BA),
    colorBorder: Color(argb: 0x210E_1F26),
    colorBorderStrong: Color(argb: 0x2B0E_1F26),
    colorBorderMuted: Color(argb: 0xFF45_6573),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 10.0,
    controlContainerRadius: 10.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 18.0,
    framePopoverRadius: 16.0,
    radiusSmall: 8.0,
    radiusMedium: 10.0,
    radiusBig: 16.0,
    progressGradientFrom: Colors.frost[600],
    progressGradientTo: Colors.frost[300]
  )

  /// The `frost-dark` counterpart of ``brightLight``.
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
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF18_262E),
    colorSurfaceRaised: Color(argb: 0xFF37_4B57),
    colorSurfaceOverlay: Color(argb: 0xFF11_1A20),
    colorSurfaceChrome: Color(argb: 0xFF11_1A20),
    colorSurfaceColumn: Color(argb: 0xFF11_1A20),
    colorContent: Color(argb: 0xFFEA_F3F7),
    colorContentSecondary: Color(argb: 0xFFBC_D0D9),
    colorContentNav: Color(argb: 0xFF93_AAB5),
    colorContentMuted: Color(argb: 0xFF93_AAB5),
    colorContentSubtle: Color(argb: 0xFF7F_99A4),
    colorContentFaint: Color(argb: 0xFF62_808D),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF32_454F),
    colorOnAccent: Color(argb: 0xFF04_171D),
    controlFieldRadius: 10.0,
    controlContainerRadius: 10.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 18.0,
    framePopoverRadius: 16.0,
    radiusSmall: 8.0,
    radiusMedium: 10.0,
    radiusBig: 16.0,
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

  /// The `graphite-light` counterpart of ``brightLight``.
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
    colorSurfaceSubtle: Color(argb: 0x0F17_171B),
    colorSurfaceInset: Color(argb: 0xFFEF_EFF2),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF6_F6F8),
    colorSurfaceColumn: Color(argb: 0xFFF6_F6F8),
    colorContent: Color(argb: 0xFF17_171B),
    colorContentSecondary: Color(argb: 0xFF3D_3D45),
    colorContentNav: Color(argb: 0xFF4C_4C56),
    colorContentMuted: Color(argb: 0xFF57_575F),
    colorContentSubtle: Color(argb: 0xFF8A_8A93),
    colorContentFaint: Color(argb: 0xFFA3_A3AB),
    colorBorder: Color(argb: 0x2117_171B),
    colorBorderStrong: Color(argb: 0x2B17_171B),
    colorBorderMuted: Color(argb: 0xFF57_575F),
    colorOnAccent: Color(argb: 0xFFFA_FAFA),
    controlFieldRadius: 8.0,
    controlContainerRadius: 8.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 16.0,
    framePopoverRadius: 12.0,
    radiusSmall: 7.0,
    radiusMedium: 8.0,
    radiusLarge: 10.0,
    radiusBig: 12.0,
    progressGradientFrom: Colors.graphite[600],
    progressGradientTo: Colors.graphite[400]
  )

  /// The `graphite-dark` counterpart of ``brightLight``.
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
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF1C_1C21),
    colorSurfaceRaised: Color(argb: 0xFF3B_3B43),
    colorSurfaceOverlay: Color(argb: 0xFF14_1417),
    colorSurfaceChrome: Color(argb: 0xFF14_1417),
    colorSurfaceColumn: Color(argb: 0xFF14_1417),
    colorContent: Color(argb: 0xFFF4_F4F6),
    colorContentSecondary: Color(argb: 0xFFC9_C9D0),
    colorContentNav: Color(argb: 0xFF9D_9DA6),
    colorContentMuted: Color(argb: 0xFF9D_9DA6),
    colorContentSubtle: Color(argb: 0xFF8A_8A93),
    colorContentFaint: Color(argb: 0xFF6A_6A73),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF38_383F),
    colorOnAccent: Color(argb: 0xFF13_1316),
    controlFieldRadius: 8.0,
    controlContainerRadius: 8.0,
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

  /// The `ember-light` counterpart of ``brightLight``.
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
    colorSurfaceSubtle: Color(argb: 0x0F24_1A11),
    colorSurfaceInset: Color(argb: 0xFFF2_EBDF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FDF9),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FDF9),
    colorSurfaceChrome: Color(argb: 0xFFF6_F1E8),
    colorSurfaceColumn: Color(argb: 0xFFF6_F1E8),
    colorContent: Color(argb: 0xFF24_1A11),
    colorContentSecondary: Color(argb: 0xFF4B_3A2B),
    colorContentNav: Color(argb: 0xFF5B_4839),
    colorContentMuted: Color(argb: 0xFF6A_5747),
    colorContentSubtle: Color(argb: 0xFF96_867A),
    colorContentFaint: Color(argb: 0xFFB0_A294),
    colorBorder: Color(argb: 0x2124_1A11),
    colorBorderStrong: Color(argb: 0x2B24_1A11),
    colorBorderMuted: Color(argb: 0xFF6A_5747),
    colorOnAccent: Color(argb: 0xFFFF_FAF3),
    controlFieldRadius: 10.0,
    controlContainerRadius: 10.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 18.0,
    framePopoverRadius: 16.0,
    radiusSmall: 8.0,
    radiusMedium: 10.0,
    radiusBig: 16.0,
    progressGradientFrom: Colors.ember[600],
    progressGradientTo: Colors.ember[300]
  )

  /// The `ember-dark` counterpart of ``brightLight``.
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
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF23_1D16),
    colorSurfaceRaised: Color(argb: 0xFF45_3A2D),
    colorSurfaceOverlay: Color(argb: 0xFF19_1410),
    colorSurfaceChrome: Color(argb: 0xFF19_1410),
    colorSurfaceColumn: Color(argb: 0xFF19_1410),
    colorContent: Color(argb: 0xFFF6_EFE4),
    colorContentSecondary: Color(argb: 0xFFD5_C6B4),
    colorContentNav: Color(argb: 0xFFAB_9886),
    colorContentMuted: Color(argb: 0xFFAB_9886),
    colorContentSubtle: Color(argb: 0xFF94_806D),
    colorContentFaint: Color(argb: 0xFF71_604F),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF44_3A2D),
    colorOnAccent: Color(argb: 0xFF24_1206),
    controlFieldRadius: 10.0,
    controlContainerRadius: 10.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 18.0,
    framePopoverRadius: 16.0,
    radiusSmall: 8.0,
    radiusMedium: 10.0,
    radiusBig: 16.0,
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

  /// The `nocturne-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let nocturneLight = ThemeVariables(
    colorPrimary: Colors.nocturne,
    colorCanvas: Color(argb: 0xFFE7_E7EF),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF6_F6FA),
    colorSurfaceSunken: Color(argb: 0xFFE2_E2EC),
    colorSurfaceSubtle: Color(argb: 0x0F23_242E),
    colorSurfaceInset: Color(argb: 0xFFEE_EEF5),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF6_F6FA),
    colorSurfaceColumn: Color(argb: 0xFFF6_F6FA),
    colorContent: Color(argb: 0xFF23_242E),
    colorContentSecondary: Color(argb: 0xFF38_3A49),
    colorContentNav: Color(argb: 0xFF4C_4E61),
    colorContentMuted: Color(argb: 0xFF63_667D),
    colorContentSubtle: Color(argb: 0xFF85_889E),
    colorContentFaint: Color(argb: 0xFFAD_AFC1),
    colorBorder: Color(argb: 0x2123_242E),
    colorBorderStrong: Color(argb: 0x2B23_242E),
    colorBorderMuted: Color(argb: 0xFFD8_D9E4),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 8.0,
    controlContainerRadius: 8.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 16.0,
    framePopoverRadius: 12.0,
    radiusSmall: 7.0,
    radiusMedium: 8.0,
    radiusLarge: 10.0,
    radiusBig: 12.0,
    progressGradientFrom: Colors.nocturne[600],
    progressGradientTo: Colors.nocturne[300]
  )

  /// The `nocturne-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let nocturneDark = ThemeVariables(
    colorPrimary: Colors.nocturne,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF10_1120),
    colorSurface: Color(argb: 0xFF16_1826),
    colorSurfaceMuted: Color(argb: 0xFF23_2532),
    colorSurfaceSunken: Color(argb: 0xFF2F_313C),
    colorSurfaceSubtle: Color(argb: 0x12E9_E9ED),
    colorSurfaceInset: Color(argb: 0xFF29_2B31),
    colorSurfaceRaised: Color(argb: 0xFF3F_424D),
    colorSurfaceOverlay: Color(argb: 0xFF23_2532),
    colorSurfaceChrome: Color(argb: 0xFF1C_1E2C),
    colorSurfaceColumn: Color(argb: 0xFF23_2532),
    colorContent: Color(argb: 0xFFE9_E9ED),
    colorContentSecondary: Color(argb: 0xFFCF_D3E5),
    colorContentNav: Color(argb: 0xFFB2_B6CA),
    colorContentMuted: Color(argb: 0xFF93_97AB),
    colorContentSubtle: Color(argb: 0xFF75_798C),
    colorContentFaint: Color(argb: 0xFF59_5D6C),
    colorBorder: Color(argb: 0x29E9_E9ED),
    colorBorderStrong: Color(argb: 0x33E9_E9ED),
    colorBorderMuted: Color(argb: 0xFF3F_424D),
    colorOnAccent: Color(argb: 0xFF16_1826),
    controlFieldRadius: 8.0,
    controlContainerRadius: 8.0,
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
    progressGradientFrom: Colors.nocturne[500],
    progressGradientTo: Colors.nocturne[300],
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

  /// The `omarchy-tokyo-night-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyTokyoNightLight = ThemeVariables(
    colorPrimary: Colors.omarchyTokyoNightLight,
    colorCanvas: Color(argb: 0xFFDB_DCE5),
    colorSurface: Color(argb: 0xFFEC_EEF6),
    colorSurfaceMuted: Color(argb: 0xFFE5_E6EF),
    colorSurfaceSunken: Color(argb: 0xFFCF_D0D9),
    colorSurfaceSubtle: Color(argb: 0x101A_1B26),
    colorSurfaceInset: Color(argb: 0xFFDD_DEE7),
    colorSurfaceRaised: Color(argb: 0xFFEC_EEF6),
    colorSurfaceOverlay: Color(argb: 0xFFEC_EEF6),
    colorSurfaceChrome: Color(argb: 0xFFE5_E6EF),
    colorSurfaceColumn: Color(argb: 0xFFE8_E9F2),
    colorContent: Color(argb: 0xFF1A_1B26),
    colorContentSecondary: Color(argb: 0xFF40_414B),
    colorContentNav: Color(argb: 0xFF55_5660),
    colorContentMuted: Color(argb: 0xFF66_6771),
    colorContentSubtle: Color(argb: 0xFF83_848E),
    colorContentFaint: Color(argb: 0xFFA0_A2AB),
    colorBorder: Color(argb: 0x211A_1B26),
    colorBorderStrong: Color(argb: 0x2B1A_1B26),
    colorBorderMuted: Color(argb: 0xFF9C_9EA7),
    colorOnAccent: Color(argb: 0xFFEC_EEF6),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyTokyoNightLight[600],
    progressGradientTo: Colors.omarchyTokyoNightLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-tokyo-night-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyTokyoNightDark = ThemeVariables(
    colorPrimary: Colors.omarchyTokyoNightDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF16_1721),
    colorSurface: Color(argb: 0xFF1A_1B26),
    colorSurfaceMuted: Color(argb: 0xFF20_212D),
    colorSurfaceSunken: Color(argb: 0xFF2E_303E),
    colorSurfaceSubtle: Color(argb: 0x14A9_B1D6),
    colorSurfaceInset: Color(argb: 0xFF26_2835),
    colorSurfaceRaised: Color(argb: 0xFF43_4659),
    colorSurfaceOverlay: Color(argb: 0xFF20_212D),
    colorSurfaceChrome: Color(argb: 0xFF20_212D),
    colorSurfaceColumn: Color(argb: 0xFF17_1822),
    colorContent: Color(argb: 0xFFA9_B1D6),
    colorContentSecondary: Color(argb: 0xFF8F_96B6),
    colorContentNav: Color(argb: 0xFF81_87A5),
    colorContentMuted: Color(argb: 0xFF7E_84A1),
    colorContentSubtle: Color(argb: 0xFF62_667E),
    colorContentFaint: Color(argb: 0xFF4D_5165),
    colorBorder: Color(argb: 0x29A9_B1D6),
    colorBorderStrong: Color(argb: 0x33A9_B1D6),
    colorBorderMuted: Color(argb: 0xFF50_5469),
    colorOnAccent: Color(argb: 0xFF1A_1B26),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyTokyoNightDark[500],
    progressGradientTo: Colors.omarchyTokyoNightDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-catppuccin-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyCatppuccinLight = ThemeVariables(
    colorPrimary: Colors.omarchyCatppuccinLight,
    colorCanvas: Color(argb: 0xFFE3_E4EC),
    colorSurface: Color(argb: 0xFFF4_F6FD),
    colorSurfaceMuted: Color(argb: 0xFFED_EEF6),
    colorSurfaceSunken: Color(argb: 0xFFD6_D8E0),
    colorSurfaceSubtle: Color(argb: 0x0F1E_1E2E),
    colorSurfaceInset: Color(argb: 0xFFE4_E6EE),
    colorSurfaceRaised: Color(argb: 0xFFF4_F6FD),
    colorSurfaceOverlay: Color(argb: 0xFFF4_F6FD),
    colorSurfaceChrome: Color(argb: 0xFFED_EEF6),
    colorSurfaceColumn: Color(argb: 0xFFF0_F1F9),
    colorContent: Color(argb: 0xFF1E_1E2E),
    colorContentSecondary: Color(argb: 0xFF45_4553),
    colorContentNav: Color(argb: 0xFF5A_5A68),
    colorContentMuted: Color(argb: 0xFF6B_6C79),
    colorContentSubtle: Color(argb: 0xFF89_8A96),
    colorContentFaint: Color(argb: 0xFFA7_A8B2),
    colorBorder: Color(argb: 0x211E_1E2E),
    colorBorderStrong: Color(argb: 0x2B1E_1E2E),
    colorBorderMuted: Color(argb: 0xFFA3_A4AE),
    colorOnAccent: Color(argb: 0xFFF4_F6FD),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyCatppuccinLight[600],
    progressGradientTo: Colors.omarchyCatppuccinLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-catppuccin-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyCatppuccinDark = ThemeVariables(
    colorPrimary: Colors.omarchyCatppuccinDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1A_1A29),
    colorSurface: Color(argb: 0xFF1E_1E2E),
    colorSurfaceMuted: Color(argb: 0xFF24_2535),
    colorSurfaceSunken: Color(argb: 0xFF32_3445),
    colorSurfaceSubtle: Color(argb: 0x12CD_D6F4),
    colorSurfaceInset: Color(argb: 0xFF2A_2B3B),
    colorSurfaceRaised: Color(argb: 0xFF48_4A5E),
    colorSurfaceOverlay: Color(argb: 0xFF24_2535),
    colorSurfaceChrome: Color(argb: 0xFF24_2535),
    colorSurfaceColumn: Color(argb: 0xFF1B_1B2A),
    colorContent: Color(argb: 0xFFCD_D6F4),
    colorContentSecondary: Color(argb: 0xFFAE_B5D0),
    colorContentNav: Color(argb: 0xFF9C_A2BD),
    colorContentMuted: Color(argb: 0xFF8E_94AD),
    colorContentSubtle: Color(argb: 0xFF76_7A91),
    colorContentFaint: Color(argb: 0xFF5D_6075),
    colorBorder: Color(argb: 0x29CD_D6F4),
    colorBorderStrong: Color(argb: 0x33CD_D6F4),
    colorBorderMuted: Color(argb: 0xFF61_6479),
    colorOnAccent: Color(argb: 0xFF1E_1E2E),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyCatppuccinDark[500],
    progressGradientTo: Colors.omarchyCatppuccinDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-catppuccin-latte-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyCatppuccinLatteLight = ThemeVariables(
    colorPrimary: Colors.omarchyCatppuccinLatteLight,
    colorCanvas: Color(argb: 0xFFDD_DFE6),
    colorSurface: Color(argb: 0xFFEF_F1F5),
    colorSurfaceMuted: Color(argb: 0xFFE7_E9EF),
    colorSurfaceSunken: Color(argb: 0xFFD0_D3DB),
    colorSurfaceSubtle: Color(argb: 0x144C_4F69),
    colorSurfaceInset: Color(argb: 0xFFDF_E1E8),
    colorSurfaceRaised: Color(argb: 0xFFEF_F1F5),
    colorSurfaceOverlay: Color(argb: 0xFFEF_F1F5),
    colorSurfaceChrome: Color(argb: 0xFFE7_E9EF),
    colorSurfaceColumn: Color(argb: 0xFFEA_ECF1),
    colorContent: Color(argb: 0xFF4C_4F69),
    colorContentSecondary: Color(argb: 0xFF68_6B81),
    colorContentNav: Color(argb: 0xFF68_6B81),
    colorContentMuted: Color(argb: 0xFF68_6B81),
    colorContentSubtle: Color(argb: 0xFF9E_A0AF),
    colorContentFaint: Color(argb: 0xFFB4_B7C3),
    colorBorder: Color(argb: 0x214C_4F69),
    colorBorderStrong: Color(argb: 0x2B4C_4F69),
    colorBorderMuted: Color(argb: 0xFFB1_B3C0),
    colorOnAccent: Color(argb: 0xFFEF_F1F5),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyCatppuccinLatteLight[600],
    progressGradientTo: Colors.omarchyCatppuccinLatteLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-catppuccin-latte-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyCatppuccinLatteDark = ThemeVariables(
    colorPrimary: Colors.omarchyCatppuccinLatteDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1E_2029),
    colorSurface: Color(argb: 0xFF22_242F),
    colorSurfaceMuted: Color(argb: 0xFF29_2B35),
    colorSurfaceSunken: Color(argb: 0xFF38_3A44),
    colorSurfaceSubtle: Color(argb: 0x12EF_F1F5),
    colorSurfaceInset: Color(argb: 0xFF2F_313B),
    colorSurfaceRaised: Color(argb: 0xFF4F_515A),
    colorSurfaceOverlay: Color(argb: 0xFF29_2B35),
    colorSurfaceChrome: Color(argb: 0xFF29_2B35),
    colorSurfaceColumn: Color(argb: 0xFF1F_212A),
    colorContent: Color(argb: 0xFFEF_F1F5),
    colorContentSecondary: Color(argb: 0xFFCA_CCD1),
    colorContentNav: Color(argb: 0xFFB6_B8BE),
    colorContentMuted: Color(argb: 0xFFA5_A7AE),
    colorContentSubtle: Color(argb: 0xFF89_8B92),
    colorContentFaint: Color(argb: 0xFF6C_6E76),
    colorBorder: Color(argb: 0x29EF_F1F5),
    colorBorderStrong: Color(argb: 0x33EF_F1F5),
    colorBorderMuted: Color(argb: 0xFF70_727A),
    colorOnAccent: Color(argb: 0xFF22_242F),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyCatppuccinLatteDark[500],
    progressGradientTo: Colors.omarchyCatppuccinLatteDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-ethereal-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyEtherealLight = ThemeVariables(
    colorPrimary: Colors.omarchyEtherealLight,
    colorCanvas: Color(argb: 0xFFEC_E3DE),
    colorSurface: Color(argb: 0xFFFF_F4ED),
    colorSurfaceMuted: Color(argb: 0xFFF7_EDE6),
    colorSurfaceSunken: Color(argb: 0xFFDF_D6D3),
    colorSurfaceSubtle: Color(argb: 0x0F06_0B1E),
    colorSurfaceInset: Color(argb: 0xFFEE_E4DF),
    colorSurfaceRaised: Color(argb: 0xFFFF_F4ED),
    colorSurfaceOverlay: Color(argb: 0xFFFF_F4ED),
    colorSurfaceChrome: Color(argb: 0xFFF7_EDE6),
    colorSurfaceColumn: Color(argb: 0xFFFA_EFE9),
    colorContent: Color(argb: 0xFF06_0B1E),
    colorContentSecondary: Color(argb: 0xFF33_3543),
    colorContentNav: Color(argb: 0xFF4C_4C58),
    colorContentMuted: Color(argb: 0xFF60_5F69),
    colorContentSubtle: Color(argb: 0xFF83_8086),
    colorContentFaint: Color(argb: 0xFFA5_A0A2),
    colorBorder: Color(argb: 0x2106_0B1E),
    colorBorderStrong: Color(argb: 0x2B06_0B1E),
    colorBorderMuted: Color(argb: 0xFFA0_9B9E),
    colorOnAccent: Color(argb: 0xFFFF_F4ED),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyEtherealLight[600],
    progressGradientTo: Colors.omarchyEtherealLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-ethereal-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyEtherealDark = ThemeVariables(
    colorPrimary: Colors.omarchyEtherealDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF02_050C),
    colorSurface: Color(argb: 0xFF06_0B1E),
    colorSurfaceMuted: Color(argb: 0xFF11_1324),
    colorSurfaceSunken: Color(argb: 0xFF22_212E),
    colorSurfaceSubtle: Color(argb: 0x12FF_CEAD),
    colorSurfaceInset: Color(argb: 0xFF18_1929),
    colorSurfaceRaised: Color(argb: 0xFF3C_363D),
    colorSurfaceOverlay: Color(argb: 0xFF11_1324),
    colorSurfaceChrome: Color(argb: 0xFF11_1324),
    colorSurfaceColumn: Color(argb: 0xFF04_0612),
    colorContent: Color(argb: 0xFFFF_CEAD),
    colorContentSecondary: Color(argb: 0xFFD2_AB93),
    colorContentNav: Color(argb: 0xFFB9_9785),
    colorContentMuted: Color(argb: 0xFFA5_887A),
    colorContentSubtle: Color(argb: 0xFF83_6D66),
    colorContentFaint: Color(argb: 0xFF60_5151),
    colorBorder: Color(argb: 0x29FF_CEAD),
    colorBorderStrong: Color(argb: 0x33FF_CEAD),
    colorBorderMuted: Color(argb: 0xFF65_5554),
    colorOnAccent: Color(argb: 0xFF06_0B1E),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyEtherealDark[500],
    progressGradientTo: Colors.omarchyEtherealDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-everforest-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyEverforestLight = ThemeVariables(
    colorPrimary: Colors.omarchyEverforestLight,
    colorCanvas: Color(argb: 0xFFE3_E1DC),
    colorSurface: Color(argb: 0xFFF5_F2EC),
    colorSurfaceMuted: Color(argb: 0xFFED_EBE5),
    colorSurfaceSunken: Color(argb: 0xFFD6_D4D0),
    colorSurfaceSubtle: Color(argb: 0x112D_353B),
    colorSurfaceInset: Color(argb: 0xFFE5_E2DD),
    colorSurfaceRaised: Color(argb: 0xFFF5_F2EC),
    colorSurfaceOverlay: Color(argb: 0xFFF5_F2EC),
    colorSurfaceChrome: Color(argb: 0xFFED_EBE5),
    colorSurfaceColumn: Color(argb: 0xFFF0_EDE8),
    colorContent: Color(argb: 0xFF2D_353B),
    colorContentSecondary: Color(argb: 0xFF51_575B),
    colorContentNav: Color(argb: 0xFF65_6A6D),
    colorContentMuted: Color(argb: 0xFF69_6E70),
    colorContentSubtle: Color(argb: 0xFF91_9394),
    colorContentFaint: Color(argb: 0xFFAD_AEAC),
    colorBorder: Color(argb: 0x212D_353B),
    colorBorderStrong: Color(argb: 0x2B2D_353B),
    colorBorderMuted: Color(argb: 0xFFA9_AAA9),
    colorOnAccent: Color(argb: 0xFFF5_F2EC),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyEverforestLight[600],
    progressGradientTo: Colors.omarchyEverforestLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-everforest-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyEverforestDark = ThemeVariables(
    colorPrimary: Colors.omarchyEverforestDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF29_3036),
    colorSurface: Color(argb: 0xFF2D_353B),
    colorSurfaceMuted: Color(argb: 0xFF34_3C40),
    colorSurfaceSunken: Color(argb: 0xFF47_4C4C),
    colorSurfaceSubtle: Color(argb: 0x16D3_C6AA),
    colorSurfaceInset: Color(argb: 0xFF3C_4345),
    colorSurfaceRaised: Color(argb: 0xFF62_635E),
    colorSurfaceOverlay: Color(argb: 0xFF34_3C40),
    colorSurfaceChrome: Color(argb: 0xFF34_3C40),
    colorSurfaceColumn: Color(argb: 0xFF2A_3137),
    colorContent: Color(argb: 0xFFD3_C6AA),
    colorContentSecondary: Color(argb: 0xFFB5_AC96),
    colorContentNav: Color(argb: 0xFFA5_9D8B),
    colorContentMuted: Color(argb: 0xFFA5_9D8B),
    colorContentSubtle: Color(argb: 0xFF80_7E72),
    colorContentFaint: Color(argb: 0xFF69_6963),
    colorBorder: Color(argb: 0x29D3_C6AA),
    colorBorderStrong: Color(argb: 0x33D3_C6AA),
    colorBorderMuted: Color(argb: 0xFF6C_6C65),
    colorOnAccent: Color(argb: 0xFF2D_353B),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyEverforestDark[500],
    progressGradientTo: Colors.omarchyEverforestDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-flexoki-light-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyFlexokiLightLight = ThemeVariables(
    colorPrimary: Colors.omarchyFlexokiLightLight,
    colorCanvas: Color(argb: 0xFFED_EADF),
    colorSurface: Color(argb: 0xFFFF_FCF0),
    colorSurfaceMuted: Color(argb: 0xFFF7_F5E9),
    colorSurfaceSunken: Color(argb: 0xFFE0_DED3),
    colorSurfaceSubtle: Color(argb: 0x0F10_0F0F),
    colorSurfaceInset: Color(argb: 0xFFEF_ECE1),
    colorSurfaceRaised: Color(argb: 0xFFFF_FCF0),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FCF0),
    colorSurfaceChrome: Color(argb: 0xFFF7_F5E9),
    colorSurfaceColumn: Color(argb: 0xFFFA_F7EC),
    colorContent: Color(argb: 0xFF10_0F0F),
    colorContentSecondary: Color(argb: 0xFF3B_3A38),
    colorContentNav: Color(argb: 0xFF53_514E),
    colorContentMuted: Color(argb: 0xFF66_6460),
    colorContentSubtle: Color(argb: 0xFF88_8680),
    colorContentFaint: Color(argb: 0xFFA9_A79F),
    colorBorder: Color(argb: 0x2110_0F0F),
    colorBorderStrong: Color(argb: 0x2B10_0F0F),
    colorBorderMuted: Color(argb: 0xFFA4_A29B),
    colorOnAccent: Color(argb: 0xFFFF_FCF0),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyFlexokiLightLight[600],
    progressGradientTo: Colors.omarchyFlexokiLightLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-flexoki-light-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyFlexokiLightDark = ThemeVariables(
    colorPrimary: Colors.omarchyFlexokiLightDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0E_0E0D),
    colorSurface: Color(argb: 0xFF07_0707),
    colorSurfaceMuted: Color(argb: 0xFF11_1110),
    colorSurfaceSunken: Color(argb: 0xFF20_201E),
    colorSurfaceSubtle: Color(argb: 0x12FF_FCF0),
    colorSurfaceInset: Color(argb: 0xFF18_1817),
    colorSurfaceRaised: Color(argb: 0xFF35_3532),
    colorSurfaceOverlay: Color(argb: 0xFF11_1110),
    colorSurfaceChrome: Color(argb: 0xFF11_1110),
    colorSurfaceColumn: Color(argb: 0xFF01_0101),
    colorContent: Color(argb: 0xFFFF_FCF0),
    colorContentSecondary: Color(argb: 0xFFD2_D0C6),
    colorContentNav: Color(argb: 0xFFBA_B7AF),
    colorContentMuted: Color(argb: 0xFFA6_A49C),
    colorContentSubtle: Color(argb: 0xFF83_827C),
    colorContentFaint: Color(argb: 0xFF60_5F5B),
    colorBorder: Color(argb: 0x29FF_FCF0),
    colorBorderStrong: Color(argb: 0x33FF_FCF0),
    colorBorderMuted: Color(argb: 0xFF65_6460),
    colorOnAccent: Color(argb: 0xFF07_0707),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyFlexokiLightDark[500],
    progressGradientTo: Colors.omarchyFlexokiLightDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-gruvbox-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyGruvboxLight = ThemeVariables(
    colorPrimary: Colors.omarchyGruvboxLight,
    colorCanvas: Color(argb: 0xFFE4_E0D8),
    colorSurface: Color(argb: 0xFFF6_F1E8),
    colorSurfaceMuted: Color(argb: 0xFFEE_EAE1),
    colorSurfaceSunken: Color(argb: 0xFFD8_D3CC),
    colorSurfaceSubtle: Color(argb: 0x1028_2828),
    colorSurfaceInset: Color(argb: 0xFFE6_E1D9),
    colorSurfaceRaised: Color(argb: 0xFFF6_F1E8),
    colorSurfaceOverlay: Color(argb: 0xFFF6_F1E8),
    colorSurfaceChrome: Color(argb: 0xFFEE_EAE1),
    colorSurfaceColumn: Color(argb: 0xFFF1_ECE4),
    colorContent: Color(argb: 0xFF28_2828),
    colorContentSecondary: Color(argb: 0xFF4D_4C4B),
    colorContentNav: Color(argb: 0xFF62_605E),
    colorContentMuted: Color(argb: 0xFF6E_6C69),
    colorContentSubtle: Color(argb: 0xFF8F_8D88),
    colorContentFaint: Color(argb: 0xFFAC_A9A3),
    colorBorder: Color(argb: 0x2128_2828),
    colorBorderStrong: Color(argb: 0x2B28_2828),
    colorBorderMuted: Color(argb: 0xFFA8_A59F),
    colorOnAccent: Color(argb: 0xFFF6_F1E8),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyGruvboxLight[600],
    progressGradientTo: Colors.omarchyGruvboxLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-gruvbox-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyGruvboxDark = ThemeVariables(
    colorPrimary: Colors.omarchyGruvboxDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF23_2323),
    colorSurface: Color(argb: 0xFF28_2828),
    colorSurfaceMuted: Color(argb: 0xFF2F_2F2D),
    colorSurfaceSunken: Color(argb: 0xFF41_3E38),
    colorSurfaceSubtle: Color(argb: 0x14D4_BE98),
    colorSurfaceInset: Color(argb: 0xFF36_3531),
    colorSurfaceRaised: Color(argb: 0xFF5B_5549),
    colorSurfaceOverlay: Color(argb: 0xFF2F_2F2D),
    colorSurfaceChrome: Color(argb: 0xFF2F_2F2D),
    colorSurfaceColumn: Color(argb: 0xFF24_2424),
    colorContent: Color(argb: 0xFFD4_BE98),
    colorContentSecondary: Color(argb: 0xFFB5_A384),
    colorContentNav: Color(argb: 0xFFA4_9479),
    colorContentMuted: Color(argb: 0xFF9D_8E74),
    colorContentSubtle: Color(argb: 0xFF7E_7360),
    colorContentFaint: Color(argb: 0xFF66_5E50),
    colorBorder: Color(argb: 0x29D4_BE98),
    colorBorderStrong: Color(argb: 0x33D4_BE98),
    colorBorderMuted: Color(argb: 0xFF69_6153),
    colorOnAccent: Color(argb: 0xFF28_2828),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyGruvboxDark[500],
    progressGradientTo: Colors.omarchyGruvboxDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-hackerman-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyHackermanLight = ThemeVariables(
    colorPrimary: Colors.omarchyHackermanLight,
    colorCanvas: Color(argb: 0xFFE7_EBEE),
    colorSurface: Color(argb: 0xFFF8_FDFF),
    colorSurfaceMuted: Color(argb: 0xFFF1_F5F8),
    colorSurfaceSunken: Color(argb: 0xFFDA_DEE2),
    colorSurfaceSubtle: Color(argb: 0x0F0B_0C16),
    colorSurfaceInset: Color(argb: 0xFFE8_EDF0),
    colorSurfaceRaised: Color(argb: 0xFFF8_FDFF),
    colorSurfaceOverlay: Color(argb: 0xFFF8_FDFF),
    colorSurfaceChrome: Color(argb: 0xFFF1_F5F8),
    colorSurfaceColumn: Color(argb: 0xFFF4_F8FB),
    colorContent: Color(argb: 0xFF0B_0C16),
    colorContentSecondary: Color(argb: 0xFF36_3740),
    colorContentNav: Color(argb: 0xFF4D_4F57),
    colorContentMuted: Color(argb: 0xFF60_636A),
    colorContentSubtle: Color(argb: 0xFF82_858B),
    colorContentFaint: Color(argb: 0xFFA3_A6AB),
    colorBorder: Color(argb: 0x210B_0C16),
    colorBorderStrong: Color(argb: 0x2B0B_0C16),
    colorBorderMuted: Color(argb: 0xFF9E_A1A6),
    colorOnAccent: Color(argb: 0xFFF8_FDFF),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyHackermanLight[600],
    progressGradientTo: Colors.omarchyHackermanLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-hackerman-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyHackermanDark = ThemeVariables(
    colorPrimary: Colors.omarchyHackermanDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF05_050A),
    colorSurface: Color(argb: 0xFF0B_0C16),
    colorSurfaceMuted: Color(argb: 0xFF13_141E),
    colorSurfaceSunken: Color(argb: 0xFF1F_232C),
    colorSurfaceSubtle: Color(argb: 0x12DD_F7FF),
    colorSurfaceInset: Color(argb: 0xFF18_1B24),
    colorSurfaceRaised: Color(argb: 0xFF33_3942),
    colorSurfaceOverlay: Color(argb: 0xFF13_141E),
    colorSurfaceChrome: Color(argb: 0xFF13_141E),
    colorSurfaceColumn: Color(argb: 0xFF06_070D),
    colorContent: Color(argb: 0xFFDD_F7FF),
    colorContentSecondary: Color(argb: 0xFFB7_CDD5),
    colorContentNav: Color(argb: 0xFFA2_B5BE),
    colorContentMuted: Color(argb: 0xFF91_A2AB),
    colorContentSubtle: Color(argb: 0xFF74_828B),
    colorContentFaint: Color(argb: 0xFF57_616A),
    colorBorder: Color(argb: 0x29DD_F7FF),
    colorBorderStrong: Color(argb: 0x33DD_F7FF),
    colorBorderMuted: Color(argb: 0xFF5B_656F),
    colorOnAccent: Color(argb: 0xFF0B_0C16),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyHackermanDark[500],
    progressGradientTo: Colors.omarchyHackermanDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-kanagawa-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyKanagawaLight = ThemeVariables(
    colorPrimary: Colors.omarchyKanagawaLight,
    colorCanvas: Color(argb: 0xFFE5_E4E0),
    colorSurface: Color(argb: 0xFFF7_F6F0),
    colorSurfaceMuted: Color(argb: 0xFFEF_EFE9),
    colorSurfaceSunken: Color(argb: 0xFFD8_D8D4),
    colorSurfaceSubtle: Color(argb: 0x0F1F_1F28),
    colorSurfaceInset: Color(argb: 0xFFE7_E6E2),
    colorSurfaceRaised: Color(argb: 0xFFF7_F6F0),
    colorSurfaceOverlay: Color(argb: 0xFFF7_F6F0),
    colorSurfaceChrome: Color(argb: 0xFFEF_EFE9),
    colorSurfaceColumn: Color(argb: 0xFFF2_F1EC),
    colorContent: Color(argb: 0xFF1F_1F28),
    colorContentSecondary: Color(argb: 0xFF46_464C),
    colorContentNav: Color(argb: 0xFF5B_5B60),
    colorContentMuted: Color(argb: 0xFF6D_6C70),
    colorContentSubtle: Color(argb: 0xFF8B_8B8C),
    colorContentFaint: Color(argb: 0xFFA9_A9A8),
    colorBorder: Color(argb: 0x211F_1F28),
    colorBorderStrong: Color(argb: 0x2B1F_1F28),
    colorBorderMuted: Color(argb: 0xFFA5_A4A4),
    colorOnAccent: Color(argb: 0xFFF7_F6F0),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyKanagawaLight[600],
    progressGradientTo: Colors.omarchyKanagawaLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-kanagawa-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyKanagawaDark = ThemeVariables(
    colorPrimary: Colors.omarchyKanagawaDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1B_1B23),
    colorSurface: Color(argb: 0xFF1F_1F28),
    colorSurfaceMuted: Color(argb: 0xFF26_262D),
    colorSurfaceSunken: Color(argb: 0xFF35_3539),
    colorSurfaceSubtle: Color(argb: 0x12DC_D7BA),
    colorSurfaceInset: Color(argb: 0xFF2C_2C32),
    colorSurfaceRaised: Color(argb: 0xFF4D_4C4B),
    colorSurfaceOverlay: Color(argb: 0xFF26_262D),
    colorSurfaceChrome: Color(argb: 0xFF26_262D),
    colorSurfaceColumn: Color(argb: 0xFF1C_1C24),
    colorContent: Color(argb: 0xFFDC_D7BA),
    colorContentSecondary: Color(argb: 0xFFBA_B6A0),
    colorContentNav: Color(argb: 0xFFA7_A391),
    colorContentMuted: Color(argb: 0xFF98_9585),
    colorContentSubtle: Color(argb: 0xFF7E_7B71),
    colorContentFaint: Color(argb: 0xFF63_615D),
    colorBorder: Color(argb: 0x29DC_D7BA),
    colorBorderStrong: Color(argb: 0x33DC_D7BA),
    colorBorderMuted: Color(argb: 0xFF67_655F),
    colorOnAccent: Color(argb: 0xFF1F_1F28),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyKanagawaDark[500],
    progressGradientTo: Colors.omarchyKanagawaDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-last-horizon-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLastHorizonLight = ThemeVariables(
    colorPrimary: Colors.omarchyLastHorizonLight,
    colorCanvas: Color(argb: 0xFFED_ECED),
    colorSurface: Color(argb: 0xFFFE_FEFE),
    colorSurfaceMuted: Color(argb: 0xFFF7_F6F7),
    colorSurfaceSunken: Color(argb: 0xFFE0_DFE0),
    colorSurfaceSubtle: Color(argb: 0x0F0C_0B0C),
    colorSurfaceInset: Color(argb: 0xFFEE_EEEE),
    colorSurfaceRaised: Color(argb: 0xFFFE_FEFE),
    colorSurfaceOverlay: Color(argb: 0xFFFE_FEFE),
    colorSurfaceChrome: Color(argb: 0xFFF7_F6F7),
    colorSurfaceColumn: Color(argb: 0xFFFA_F9FA),
    colorContent: Color(argb: 0xFF0C_0B0C),
    colorContentSecondary: Color(argb: 0xFF38_3738),
    colorContentNav: Color(argb: 0xFF50_4F50),
    colorContentMuted: Color(argb: 0xFF63_6263),
    colorContentSubtle: Color(argb: 0xFF85_8485),
    colorContentFaint: Color(argb: 0xFFA7_A7A7),
    colorBorder: Color(argb: 0x210C_0B0C),
    colorBorderStrong: Color(argb: 0x2B0C_0B0C),
    colorBorderMuted: Color(argb: 0xFFA2_A2A2),
    colorOnAccent: Color(argb: 0xFFFE_FEFE),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyLastHorizonLight[600],
    progressGradientTo: Colors.omarchyLastHorizonLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-last-horizon-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLastHorizonDark = ThemeVariables(
    colorPrimary: Colors.omarchyLastHorizonDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF04_0404),
    colorSurface: Color(argb: 0xFF0C_0B0C),
    colorSurfaceMuted: Color(argb: 0xFF14_1414),
    colorSurfaceSunken: Color(argb: 0xFF22_2222),
    colorSurfaceSubtle: Color(argb: 0x12FA_FCFB),
    colorSurfaceInset: Color(argb: 0xFF1A_1A1A),
    colorSurfaceRaised: Color(argb: 0xFF38_3738),
    colorSurfaceOverlay: Color(argb: 0xFF14_1414),
    colorSurfaceChrome: Color(argb: 0xFF14_1414),
    colorSurfaceColumn: Color(argb: 0xFF06_0506),
    colorContent: Color(argb: 0xFFFA_FCFB),
    colorContentSecondary: Color(argb: 0xFFCF_D1D0),
    colorContentNav: Color(argb: 0xFFB7_B9B8),
    colorContentMuted: Color(argb: 0xFFA4_A5A5),
    colorContentSubtle: Color(argb: 0xFF83_8484),
    colorContentFaint: Color(argb: 0xFF62_6262),
    colorBorder: Color(argb: 0x29FA_FCFB),
    colorBorderStrong: Color(argb: 0x33FA_FCFB),
    colorBorderMuted: Color(argb: 0xFF66_6767),
    colorOnAccent: Color(argb: 0xFF0C_0B0C),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyLastHorizonDark[500],
    progressGradientTo: Colors.omarchyLastHorizonDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-lumon-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLumonLight = ThemeVariables(
    colorPrimary: Colors.omarchyLumonLight,
    colorCanvas: Color(argb: 0xFFE4_E8EA),
    colorSurface: Color(argb: 0xFFF6_F9FB),
    colorSurfaceMuted: Color(argb: 0xFFEE_F2F4),
    colorSurfaceSunken: Color(argb: 0xFFD7_DBDE),
    colorSurfaceSubtle: Color(argb: 0x0F16_242D),
    colorSurfaceInset: Color(argb: 0xFFE6_E9EC),
    colorSurfaceRaised: Color(argb: 0xFFF6_F9FB),
    colorSurfaceOverlay: Color(argb: 0xFFF6_F9FB),
    colorSurfaceChrome: Color(argb: 0xFFEE_F2F4),
    colorSurfaceColumn: Color(argb: 0xFFF1_F4F7),
    colorContent: Color(argb: 0xFF16_242D),
    colorContentSecondary: Color(argb: 0xFF3E_4A52),
    colorContentNav: Color(argb: 0xFF55_6067),
    colorContentMuted: Color(argb: 0xFF67_7177),
    colorContentSubtle: Color(argb: 0xFF86_8F94),
    colorContentFaint: Color(argb: 0xFFA5_ACB1),
    colorBorder: Color(argb: 0x2116_242D),
    colorBorderStrong: Color(argb: 0x2B16_242D),
    colorBorderMuted: Color(argb: 0xFFA1_A8AD),
    colorOnAccent: Color(argb: 0xFFF6_F9FB),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyLumonLight[600],
    progressGradientTo: Colors.omarchyLumonLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-lumon-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLumonDark = ThemeVariables(
    colorPrimary: Colors.omarchyLumonDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF13_1F27),
    colorSurface: Color(argb: 0xFF16_242D),
    colorSurfaceMuted: Color(argb: 0xFF1D_2B34),
    colorSurfaceSunken: Color(argb: 0xFF2C_3A43),
    colorSurfaceSubtle: Color(argb: 0x12D6_E2EE),
    colorSurfaceInset: Color(argb: 0xFF23_313A),
    colorSurfaceRaised: Color(argb: 0xFF43_515A),
    colorSurfaceOverlay: Color(argb: 0xFF1D_2B34),
    colorSurfaceChrome: Color(argb: 0xFF1D_2B34),
    colorSurfaceColumn: Color(argb: 0xFF14_2029),
    colorContent: Color(argb: 0xFFD6_E2EE),
    colorContentSecondary: Color(argb: 0xFFB3_C0CB),
    colorContentNav: Color(argb: 0xFFA0_ADB8),
    colorContentMuted: Color(argb: 0xFF91_9EA9),
    colorContentSubtle: Color(argb: 0xFF76_838E),
    colorContentFaint: Color(argb: 0xFF5B_6872),
    colorBorder: Color(argb: 0x29D6_E2EE),
    colorBorderStrong: Color(argb: 0x33D6_E2EE),
    colorBorderMuted: Color(argb: 0xFF5F_6C76),
    colorOnAccent: Color(argb: 0xFF16_242D),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyLumonDark[500],
    progressGradientTo: Colors.omarchyLumonDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-lupine-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLupineLight = ThemeVariables(
    colorPrimary: Colors.omarchyLupineLight,
    colorCanvas: Color(argb: 0xFFE8_E8E8),
    colorSurface: Color(argb: 0xFFFA_FAFA),
    colorSurfaceMuted: Color(argb: 0xFFF2_F2F2),
    colorSurfaceSunken: Color(argb: 0xFFDC_DCDC),
    colorSurfaceSubtle: Color(argb: 0x0F21_2121),
    colorSurfaceInset: Color(argb: 0xFFEA_EAEA),
    colorSurfaceRaised: Color(argb: 0xFFFA_FAFA),
    colorSurfaceOverlay: Color(argb: 0xFFFA_FAFA),
    colorSurfaceChrome: Color(argb: 0xFFF2_F2F2),
    colorSurfaceColumn: Color(argb: 0xFFF5_F5F5),
    colorContent: Color(argb: 0xFF21_2121),
    colorContentSecondary: Color(argb: 0xFF48_4848),
    colorContentNav: Color(argb: 0xFF5E_5E5E),
    colorContentMuted: Color(argb: 0xFF6F_6F6F),
    colorContentSubtle: Color(argb: 0xFF8E_8E8E),
    colorContentFaint: Color(argb: 0xFFAC_ACAC),
    colorBorder: Color(argb: 0x2121_2121),
    colorBorderStrong: Color(argb: 0x2B21_2121),
    colorBorderMuted: Color(argb: 0xFFA8_A8A8),
    colorOnAccent: Color(argb: 0xFFFA_FAFA),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyLupineLight[600],
    progressGradientTo: Colors.omarchyLupineLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-lupine-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyLupineDark = ThemeVariables(
    colorPrimary: Colors.omarchyLupineDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF08_0808),
    colorSurface: Color(argb: 0xFF0F_0F0F),
    colorSurfaceMuted: Color(argb: 0xFF17_1717),
    colorSurfaceSunken: Color(argb: 0xFF25_2525),
    colorSurfaceSubtle: Color(argb: 0x12FA_FAFA),
    colorSurfaceInset: Color(argb: 0xFF1D_1D1D),
    colorSurfaceRaised: Color(argb: 0xFF3A_3A3A),
    colorSurfaceOverlay: Color(argb: 0xFF17_1717),
    colorSurfaceChrome: Color(argb: 0xFF17_1717),
    colorSurfaceColumn: Color(argb: 0xFF0A_0A0A),
    colorContent: Color(argb: 0xFFFA_FAFA),
    colorContentSecondary: Color(argb: 0xFFD0_D0D0),
    colorContentNav: Color(argb: 0xFFB8_B8B8),
    colorContentMuted: Color(argb: 0xFFA5_A5A5),
    colorContentSubtle: Color(argb: 0xFF84_8484),
    colorContentFaint: Color(argb: 0xFF64_6464),
    colorBorder: Color(argb: 0x29FA_FAFA),
    colorBorderStrong: Color(argb: 0x33FA_FAFA),
    colorBorderMuted: Color(argb: 0xFF68_6868),
    colorOnAccent: Color(argb: 0xFF0F_0F0F),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyLupineDark[500],
    progressGradientTo: Colors.omarchyLupineDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-matte-black-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyMatteBlackLight = ThemeVariables(
    colorPrimary: Colors.omarchyMatteBlackLight,
    colorCanvas: Color(argb: 0xFFDF_DFDF),
    colorSurface: Color(argb: 0xFFF1_F1F1),
    colorSurfaceMuted: Color(argb: 0xFFE9_E9E9),
    colorSurfaceSunken: Color(argb: 0xFFD3_D3D3),
    colorSurfaceSubtle: Color(argb: 0x0F12_1212),
    colorSurfaceInset: Color(argb: 0xFFE1_E1E1),
    colorSurfaceRaised: Color(argb: 0xFFF1_F1F1),
    colorSurfaceOverlay: Color(argb: 0xFFF1_F1F1),
    colorSurfaceChrome: Color(argb: 0xFFE9_E9E9),
    colorSurfaceColumn: Color(argb: 0xFFEC_ECEC),
    colorContent: Color(argb: 0xFF12_1212),
    colorContentSecondary: Color(argb: 0xFF3A_3A3A),
    colorContentNav: Color(argb: 0xFF50_5050),
    colorContentMuted: Color(argb: 0xFF62_6262),
    colorContentSubtle: Color(argb: 0xFF82_8282),
    colorContentFaint: Color(argb: 0xFFA1_A1A1),
    colorBorder: Color(argb: 0x2112_1212),
    colorBorderStrong: Color(argb: 0x2B12_1212),
    colorBorderMuted: Color(argb: 0xFF9C_9C9C),
    colorOnAccent: Color(argb: 0xFFF1_F1F1),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyMatteBlackLight[600],
    progressGradientTo: Colors.omarchyMatteBlackLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-matte-black-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyMatteBlackDark = ThemeVariables(
    colorPrimary: Colors.omarchyMatteBlackDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0C_0C0C),
    colorSurface: Color(argb: 0xFF12_1212),
    colorSurfaceMuted: Color(argb: 0xFF19_1919),
    colorSurfaceSunken: Color(argb: 0xFF27_2727),
    colorSurfaceSubtle: Color(argb: 0x12BE_BEBE),
    colorSurfaceInset: Color(argb: 0xFF1F_1F1F),
    colorSurfaceRaised: Color(argb: 0xFF3D_3D3D),
    colorSurfaceOverlay: Color(argb: 0xFF19_1919),
    colorSurfaceChrome: Color(argb: 0xFF19_1919),
    colorSurfaceColumn: Color(argb: 0xFF0D_0D0D),
    colorContent: Color(argb: 0xFFBE_BEBE),
    colorContentSecondary: Color(argb: 0xFF9F_9F9F),
    colorContentNav: Color(argb: 0xFF8E_8E8E),
    colorContentMuted: Color(argb: 0xFF80_8080),
    colorContentSubtle: Color(argb: 0xFF68_6868),
    colorContentFaint: Color(argb: 0xFF50_5050),
    colorBorder: Color(argb: 0x29BE_BEBE),
    colorBorderStrong: Color(argb: 0x33BE_BEBE),
    colorBorderMuted: Color(argb: 0xFF53_5353),
    colorOnAccent: Color(argb: 0xFF12_1212),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyMatteBlackDark[500],
    progressGradientTo: Colors.omarchyMatteBlackDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-miasma-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyMiasmaLight = ThemeVariables(
    colorPrimary: Colors.omarchyMiasmaLight,
    colorCanvas: Color(argb: 0xFFE0_E0DD),
    colorSurface: Color(argb: 0xFFF2_F2EE),
    colorSurfaceMuted: Color(argb: 0xFFEA_EAE7),
    colorSurfaceSunken: Color(argb: 0xFFD4_D4D1),
    colorSurfaceSubtle: Color(argb: 0x1022_2222),
    colorSurfaceInset: Color(argb: 0xFFE2_E2DF),
    colorSurfaceRaised: Color(argb: 0xFFF2_F2EE),
    colorSurfaceOverlay: Color(argb: 0xFFF2_F2EE),
    colorSurfaceChrome: Color(argb: 0xFFEA_EAE7),
    colorSurfaceColumn: Color(argb: 0xFFED_EDEA),
    colorContent: Color(argb: 0xFF22_2222),
    colorContentSecondary: Color(argb: 0xFF47_4747),
    colorContentNav: Color(argb: 0xFF5C_5C5B),
    colorContentMuted: Color(argb: 0xFF6D_6D6B),
    colorContentSubtle: Color(argb: 0xFF8A_8A88),
    colorContentFaint: Color(argb: 0xFFA7_A7A5),
    colorBorder: Color(argb: 0x2122_2222),
    colorBorderStrong: Color(argb: 0x2B22_2222),
    colorBorderMuted: Color(argb: 0xFFA3_A3A0),
    colorOnAccent: Color(argb: 0xFFF2_F2EE),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyMiasmaLight[600],
    progressGradientTo: Colors.omarchyMiasmaLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-miasma-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyMiasmaDark = ThemeVariables(
    colorPrimary: Colors.omarchyMiasmaDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1D_1D1D),
    colorSurface: Color(argb: 0xFF22_2222),
    colorSurfaceMuted: Color(argb: 0xFF29_2928),
    colorSurfaceSunken: Color(argb: 0xFF38_3835),
    colorSurfaceSubtle: Color(argb: 0x13C2_C2B0),
    colorSurfaceInset: Color(argb: 0xFF2F_2F2D),
    colorSurfaceRaised: Color(argb: 0xFF4F_4F49),
    colorSurfaceOverlay: Color(argb: 0xFF29_2928),
    colorSurfaceChrome: Color(argb: 0xFF29_2928),
    colorSurfaceColumn: Color(argb: 0xFF1E_1E1E),
    colorContent: Color(argb: 0xFFC2_C2B0),
    colorContentSecondary: Color(argb: 0xFFA5_A596),
    colorContentNav: Color(argb: 0xFF95_9588),
    colorContentMuted: Color(argb: 0xFF8C_8C80),
    colorContentSubtle: Color(argb: 0xFF72_7269),
    colorContentFaint: Color(argb: 0xFF5C_5C55),
    colorBorder: Color(argb: 0x29C2_C2B0),
    colorBorderStrong: Color(argb: 0x33C2_C2B0),
    colorBorderMuted: Color(argb: 0xFF5F_5F58),
    colorOnAccent: Color(argb: 0xFF22_2222),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyMiasmaDark[500],
    progressGradientTo: Colors.omarchyMiasmaDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-nord-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyNordLight = ThemeVariables(
    colorPrimary: Colors.omarchyNordLight,
    colorCanvas: Color(argb: 0xFFE4_E7E9),
    colorSurface: Color(argb: 0xFFF6_F8FA),
    colorSurfaceMuted: Color(argb: 0xFFEE_F1F3),
    colorSurfaceSunken: Color(argb: 0xFFD7_DADE),
    colorSurfaceSubtle: Color(argb: 0x112E_3440),
    colorSurfaceInset: Color(argb: 0xFFE6_E8EB),
    colorSurfaceRaised: Color(argb: 0xFFF6_F8FA),
    colorSurfaceOverlay: Color(argb: 0xFFF6_F8FA),
    colorSurfaceChrome: Color(argb: 0xFFEE_F1F3),
    colorSurfaceColumn: Color(argb: 0xFFF1_F3F6),
    colorContent: Color(argb: 0xFF2E_3440),
    colorContentSecondary: Color(argb: 0xFF52_5761),
    colorContentNav: Color(argb: 0xFF66_6B74),
    colorContentMuted: Color(argb: 0xFF6C_717A),
    colorContentSubtle: Color(argb: 0xFF92_969D),
    colorContentFaint: Color(argb: 0xFFAE_B1B7),
    colorBorder: Color(argb: 0x212E_3440),
    colorBorderStrong: Color(argb: 0x2B2E_3440),
    colorBorderMuted: Color(argb: 0xFFAA_AEB3),
    colorOnAccent: Color(argb: 0xFFF6_F8FA),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyNordLight[600],
    progressGradientTo: Colors.omarchyNordLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-nord-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyNordDark = ThemeVariables(
    colorPrimary: Colors.omarchyNordDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF2A_2F3A),
    colorSurface: Color(argb: 0xFF2E_3440),
    colorSurfaceMuted: Color(argb: 0xFF35_3B46),
    colorSurfaceSunken: Color(argb: 0xFF45_4B56),
    colorSurfaceSubtle: Color(argb: 0x13D8_DEE9),
    colorSurfaceInset: Color(argb: 0xFF3C_424D),
    colorSurfaceRaised: Color(argb: 0xFF5D_636E),
    colorSurfaceOverlay: Color(argb: 0xFF35_3B46),
    colorSurfaceChrome: Color(argb: 0xFF35_3B46),
    colorSurfaceColumn: Color(argb: 0xFF2B_303C),
    colorContent: Color(argb: 0xFFD8_DEE9),
    colorContentSecondary: Color(argb: 0xFFB9_BFCB),
    colorContentNav: Color(argb: 0xFFA8_AEBA),
    colorContentMuted: Color(argb: 0xFF9B_A1AC),
    colorContentSubtle: Color(argb: 0xFF83_8994),
    colorContentFaint: Color(argb: 0xFF6B_717D),
    colorBorder: Color(argb: 0x29D8_DEE9),
    colorBorderStrong: Color(argb: 0x33D8_DEE9),
    colorBorderMuted: Color(argb: 0xFF6F_7580),
    colorOnAccent: Color(argb: 0xFF2E_3440),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyNordDark[500],
    progressGradientTo: Colors.omarchyNordDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-osaka-jade-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyOsakaJadeLight = ThemeVariables(
    colorPrimary: Colors.omarchyOsakaJadeLight,
    colorCanvas: Color(argb: 0xFFDF_E1D8),
    colorSurface: Color(argb: 0xFFF1_F2E8),
    colorSurfaceMuted: Color(argb: 0xFFE9_EBE1),
    colorSurfaceSunken: Color(argb: 0xFFD2_D4CB),
    colorSurfaceSubtle: Color(argb: 0x0F11_1C18),
    colorSurfaceInset: Color(argb: 0xFFE1_E2D9),
    colorSurfaceRaised: Color(argb: 0xFFF1_F2E8),
    colorSurfaceOverlay: Color(argb: 0xFFF1_F2E8),
    colorSurfaceChrome: Color(argb: 0xFFE9_EBE1),
    colorSurfaceColumn: Color(argb: 0xFFEC_EDE4),
    colorContent: Color(argb: 0xFF11_1C18),
    colorContentSecondary: Color(argb: 0xFF39_433D),
    colorContentNav: Color(argb: 0xFF50_5852),
    colorContentMuted: Color(argb: 0xFF62_6963),
    colorContentSubtle: Color(argb: 0xFF81_8780),
    colorContentFaint: Color(argb: 0xFFA0_A59D),
    colorBorder: Color(argb: 0x2111_1C18),
    colorBorderStrong: Color(argb: 0x2B11_1C18),
    colorBorderMuted: Color(argb: 0xFF9C_A199),
    colorOnAccent: Color(argb: 0xFFF1_F2E8),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyOsakaJadeLight[600],
    progressGradientTo: Colors.omarchyOsakaJadeLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-osaka-jade-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyOsakaJadeDark = ThemeVariables(
    colorPrimary: Colors.omarchyOsakaJadeDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0E_1714),
    colorSurface: Color(argb: 0xFF11_1C18),
    colorSurfaceMuted: Color(argb: 0xFF18_221D),
    colorSurfaceSunken: Color(argb: 0xFF27_3128),
    colorSurfaceSubtle: Color(argb: 0x12C1_C497),
    colorSurfaceInset: Color(argb: 0xFF1E_2921),
    colorSurfaceRaised: Color(argb: 0xFF3F_4839),
    colorSurfaceOverlay: Color(argb: 0xFF18_221D),
    colorSurfaceChrome: Color(argb: 0xFF18_221D),
    colorSurfaceColumn: Color(argb: 0xFF0F_1815),
    colorContent: Color(argb: 0xFFC1_C497),
    colorContentSecondary: Color(argb: 0xFFA1_A680),
    colorContentNav: Color(argb: 0xFF90_9573),
    colorContentMuted: Color(argb: 0xFF82_8869),
    colorContentSubtle: Color(argb: 0xFF69_7058),
    colorContentFaint: Color(argb: 0xFF50_5846),
    colorBorder: Color(argb: 0x29C1_C497),
    colorBorderStrong: Color(argb: 0x33C1_C497),
    colorBorderMuted: Color(argb: 0xFF54_5C48),
    colorOnAccent: Color(argb: 0xFF11_1C18),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyOsakaJadeDark[500],
    progressGradientTo: Colors.omarchyOsakaJadeDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-retro-82-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRetro82Light = ThemeVariables(
    colorPrimary: Colors.omarchyRetro82Light,
    colorCanvas: Color(argb: 0xFFEA_E6DF),
    colorSurface: Color(argb: 0xFFFD_F7ED),
    colorSurfaceMuted: Color(argb: 0xFFF5_EFE7),
    colorSurfaceSunken: Color(argb: 0xFFDC_D9D4),
    colorSurfaceSubtle: Color(argb: 0x0F05_182E),
    colorSurfaceInset: Color(argb: 0xFFEC_E7E0),
    colorSurfaceRaised: Color(argb: 0xFFFD_F7ED),
    colorSurfaceOverlay: Color(argb: 0xFFFD_F7ED),
    colorSurfaceChrome: Color(argb: 0xFFF5_EFE7),
    colorSurfaceColumn: Color(argb: 0xFFF8_F2E9),
    colorContent: Color(argb: 0xFF05_182E),
    colorContentSecondary: Color(argb: 0xFF32_4050),
    colorContentNav: Color(argb: 0xFF4A_5663),
    colorContentMuted: Color(argb: 0xFF5E_6873),
    colorContentSubtle: Color(argb: 0xFF81_888E),
    colorContentFaint: Color(argb: 0xFFA4_A7A8),
    colorBorder: Color(argb: 0x2105_182E),
    colorBorderStrong: Color(argb: 0x2B05_182E),
    colorBorderMuted: Color(argb: 0xFF9F_A2A4),
    colorOnAccent: Color(argb: 0xFFFD_F7ED),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyRetro82Light[600],
    progressGradientTo: Colors.omarchyRetro82Light[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-retro-82-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRetro82Dark = ThemeVariables(
    colorPrimary: Colors.omarchyRetro82Dark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF04_1425),
    colorSurface: Color(argb: 0xFF05_182E),
    colorSurfaceMuted: Color(argb: 0xFF0D_1F32),
    colorSurfaceSunken: Color(argb: 0xFF1F_2E3C),
    colorSurfaceSubtle: Color(argb: 0x12F6_DCAC),
    colorSurfaceInset: Color(argb: 0xFF14_2536),
    colorSurfaceRaised: Color(argb: 0xFF3A_444A),
    colorSurfaceOverlay: Color(argb: 0xFF0D_1F32),
    colorSurfaceChrome: Color(argb: 0xFF0D_1F32),
    colorSurfaceColumn: Color(argb: 0xFF04_1528),
    colorContent: Color(argb: 0xFFF6_DCAC),
    colorContentSecondary: Color(argb: 0xFFCB_B995),
    colorContentNav: Color(argb: 0xFFB3_A589),
    colorContentMuted: Color(argb: 0xFF9F_957F),
    colorContentSubtle: Color(argb: 0xFF7E_7A6D),
    colorContentFaint: Color(argb: 0xFF5C_5F5B),
    colorBorder: Color(argb: 0x29F6_DCAC),
    colorBorderStrong: Color(argb: 0x33F6_DCAC),
    colorBorderMuted: Color(argb: 0xFF61_625E),
    colorOnAccent: Color(argb: 0xFF05_182E),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyRetro82Dark[500],
    progressGradientTo: Colors.omarchyRetro82Dark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-ristretto-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRistrettoLight = ThemeVariables(
    colorPrimary: Colors.omarchyRistrettoLight,
    colorCanvas: Color(argb: 0xFFE9_E5E5),
    colorSurface: Color(argb: 0xFFFA_F7F7),
    colorSurfaceMuted: Color(argb: 0xFFF3_EFEF),
    colorSurfaceSunken: Color(argb: 0xFFDC_D9D9),
    colorSurfaceSubtle: Color(argb: 0x102C_2525),
    colorSurfaceInset: Color(argb: 0xFFEB_E7E7),
    colorSurfaceRaised: Color(argb: 0xFFFA_F7F7),
    colorSurfaceOverlay: Color(argb: 0xFFFA_F7F7),
    colorSurfaceChrome: Color(argb: 0xFFF3_EFEF),
    colorSurfaceColumn: Color(argb: 0xFFF6_F2F2),
    colorContent: Color(argb: 0xFF2C_2525),
    colorContentSecondary: Color(argb: 0xFF51_4B4B),
    colorContentNav: Color(argb: 0xFF66_6060),
    colorContentMuted: Color(argb: 0xFF74_6F6F),
    colorContentSubtle: Color(argb: 0xFF93_8E8E),
    colorContentFaint: Color(argb: 0xFFB0_ABAB),
    colorBorder: Color(argb: 0x212C_2525),
    colorBorderStrong: Color(argb: 0x2B2C_2525),
    colorBorderMuted: Color(argb: 0xFFAC_A7A7),
    colorOnAccent: Color(argb: 0xFFFA_F7F7),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyRistrettoLight[600],
    progressGradientTo: Colors.omarchyRistrettoLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-ristretto-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRistrettoDark = ThemeVariables(
    colorPrimary: Colors.omarchyRistrettoDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF27_2121),
    colorSurface: Color(argb: 0xFF2C_2525),
    colorSurfaceMuted: Color(argb: 0xFF33_2C2C),
    colorSurfaceSunken: Color(argb: 0xFF42_3B3B),
    colorSurfaceSubtle: Color(argb: 0x12E6_D9DB),
    colorSurfaceInset: Color(argb: 0xFF39_3232),
    colorSurfaceRaised: Color(argb: 0xFF5A_5252),
    colorSurfaceOverlay: Color(argb: 0xFF33_2C2C),
    colorSurfaceChrome: Color(argb: 0xFF33_2C2C),
    colorSurfaceColumn: Color(argb: 0xFF28_2222),
    colorContent: Color(argb: 0xFFE6_D9DB),
    colorContentSecondary: Color(argb: 0xFFC5_B9BA),
    colorContentNav: Color(argb: 0xFFB2_A7A8),
    colorContentMuted: Color(argb: 0xFFA3_9899),
    colorContentSubtle: Color(argb: 0xFF89_7F80),
    colorContentFaint: Color(argb: 0xFF6F_6667),
    colorBorder: Color(argb: 0x29E6_D9DB),
    colorBorderStrong: Color(argb: 0x33E6_D9DB),
    colorBorderMuted: Color(argb: 0xFF73_696A),
    colorOnAccent: Color(argb: 0xFF2C_2525),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyRistrettoDark[500],
    progressGradientTo: Colors.omarchyRistrettoDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-rose-pine-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRosePineLight = ThemeVariables(
    colorPrimary: Colors.omarchyRosePineLight,
    colorCanvas: Color(argb: 0xFFE8_E2E0),
    colorSurface: Color(argb: 0xFFFA_F4ED),
    colorSurfaceMuted: Color(argb: 0xFFF2_ECE8),
    colorSurfaceSunken: Color(argb: 0xFFDB_D5D7),
    colorSurfaceSubtle: Color(argb: 0x1657_5279),
    colorSurfaceInset: Color(argb: 0xFFE9_E4E1),
    colorSurfaceRaised: Color(argb: 0xFFFA_F4ED),
    colorSurfaceOverlay: Color(argb: 0xFFFA_F4ED),
    colorSurfaceChrome: Color(argb: 0xFFF2_ECE8),
    colorSurfaceColumn: Color(argb: 0xFFF5_EFEA),
    colorContent: Color(argb: 0xFF57_5279),
    colorContentSecondary: Color(argb: 0xFF6F_6A8A),
    colorContentNav: Color(argb: 0xFF6F_6A8A),
    colorContentMuted: Color(argb: 0xFF6F_6A8A),
    colorContentSubtle: Color(argb: 0xFFA9_A3B3),
    colorContentFaint: Color(argb: 0xFFBF_BAC3),
    colorBorder: Color(argb: 0x2157_5279),
    colorBorderStrong: Color(argb: 0x2B57_5279),
    colorBorderMuted: Color(argb: 0xFFBC_B6C1),
    colorOnAccent: Color(argb: 0xFFFA_F4ED),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyRosePineLight[600],
    progressGradientTo: Colors.omarchyRosePineLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-rose-pine-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyRosePineDark = ThemeVariables(
    colorPrimary: Colors.omarchyRosePineDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF23_2130),
    colorSurface: Color(argb: 0xFF27_2536),
    colorSurfaceMuted: Color(argb: 0xFF2E_2C3C),
    colorSurfaceSunken: Color(argb: 0xFF3D_3B4A),
    colorSurfaceSubtle: Color(argb: 0x12FA_F4ED),
    colorSurfaceInset: Color(argb: 0xFF34_3242),
    colorSurfaceRaised: Color(argb: 0xFF55_535E),
    colorSurfaceOverlay: Color(argb: 0xFF2E_2C3C),
    colorSurfaceChrome: Color(argb: 0xFF2E_2C3C),
    colorSurfaceColumn: Color(argb: 0xFF24_2231),
    colorContent: Color(argb: 0xFFFA_F4ED),
    colorContentSecondary: Color(argb: 0xFFD4_CFCC),
    colorContentNav: Color(argb: 0xFFBF_BABA),
    colorContentMuted: Color(argb: 0xFFAE_A9AB),
    colorContentSubtle: Color(argb: 0xFF91_8D92),
    colorContentFaint: Color(argb: 0xFF73_7078),
    colorBorder: Color(argb: 0x29FA_F4ED),
    colorBorderStrong: Color(argb: 0x33FA_F4ED),
    colorBorderMuted: Color(argb: 0xFF77_747C),
    colorOnAccent: Color(argb: 0xFF27_2536),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyRosePineDark[500],
    progressGradientTo: Colors.omarchyRosePineDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-solitude-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchySolitudeLight = ThemeVariables(
    colorPrimary: Colors.omarchySolitudeLight,
    colorCanvas: Color(argb: 0xFFE1_E3E3),
    colorSurface: Color(argb: 0xFFF3_F4F4),
    colorSurfaceMuted: Color(argb: 0xFFEB_EDED),
    colorSurfaceSunken: Color(argb: 0xFFD5_D6D7),
    colorSurfaceSubtle: Color(argb: 0x0F10_1315),
    colorSurfaceInset: Color(argb: 0xFFE3_E4E5),
    colorSurfaceRaised: Color(argb: 0xFFF3_F4F4),
    colorSurfaceOverlay: Color(argb: 0xFFF3_F4F4),
    colorSurfaceChrome: Color(argb: 0xFFEB_EDED),
    colorSurfaceColumn: Color(argb: 0xFFEE_EFF0),
    colorContent: Color(argb: 0xFF10_1315),
    colorContentSecondary: Color(argb: 0xFF39_3C3D),
    colorContentNav: Color(argb: 0xFF50_5253),
    colorContentMuted: Color(argb: 0xFF62_6465),
    colorContentSubtle: Color(argb: 0xFF82_8485),
    colorContentFaint: Color(argb: 0xFFA1_A3A4),
    colorBorder: Color(argb: 0x2110_1315),
    colorBorderStrong: Color(argb: 0x2B10_1315),
    colorBorderMuted: Color(argb: 0xFF9D_9F9F),
    colorOnAccent: Color(argb: 0xFFF3_F4F4),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchySolitudeLight[600],
    progressGradientTo: Colors.omarchySolitudeLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-solitude-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchySolitudeDark = ThemeVariables(
    colorPrimary: Colors.omarchySolitudeDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF0B_0D0F),
    colorSurface: Color(argb: 0xFF10_1315),
    colorSurfaceMuted: Color(argb: 0xFF17_1A1B),
    colorSurfaceSunken: Color(argb: 0xFF25_2829),
    colorSurfaceSubtle: Color(argb: 0x12CA_CCCC),
    colorSurfaceInset: Color(argb: 0xFF1D_2021),
    colorSurfaceRaised: Color(argb: 0xFF3B_3E3F),
    colorSurfaceOverlay: Color(argb: 0xFF17_1A1B),
    colorSurfaceChrome: Color(argb: 0xFF17_1A1B),
    colorSurfaceColumn: Color(argb: 0xFF0C_0F10),
    colorContent: Color(argb: 0xFFCA_CCCC),
    colorContentSecondary: Color(argb: 0xFFA9_ABAB),
    colorContentNav: Color(argb: 0xFF96_9899),
    colorContentMuted: Color(argb: 0xFF87_898A),
    colorContentSubtle: Color(argb: 0xFF6D_7071),
    colorContentFaint: Color(argb: 0xFF53_5657),
    colorBorder: Color(argb: 0x29CA_CCCC),
    colorBorderStrong: Color(argb: 0x33CA_CCCC),
    colorBorderMuted: Color(argb: 0xFF57_595B),
    colorOnAccent: Color(argb: 0xFF10_1315),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchySolitudeDark[500],
    progressGradientTo: Colors.omarchySolitudeDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-vantablack-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyVantablackLight = ThemeVariables(
    colorPrimary: Colors.omarchyVantablackLight,
    colorCanvas: Color(argb: 0xFFED_EDED),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF7_F7F7),
    colorSurfaceSunken: Color(argb: 0xFFE0_E0E0),
    colorSurfaceSubtle: Color(argb: 0x0F00_0000),
    colorSurfaceInset: Color(argb: 0xFFEF_EFEF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF7_F7F7),
    colorSurfaceColumn: Color(argb: 0xFFFA_FAFA),
    colorContent: Color(argb: 0xFF00_0000),
    colorContentSecondary: Color(argb: 0xFF2E_2E2E),
    colorContentNav: Color(argb: 0xFF47_4747),
    colorContentMuted: Color(argb: 0xFF5C_5C5C),
    colorContentSubtle: Color(argb: 0xFF80_8080),
    colorContentFaint: Color(argb: 0xFFA3_A3A3),
    colorBorder: Color(argb: 0x2100_0000),
    colorBorderStrong: Color(argb: 0x2B00_0000),
    colorBorderMuted: Color(argb: 0xFF9E_9E9E),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyVantablackLight[600],
    progressGradientTo: Colors.omarchyVantablackLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-vantablack-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyVantablackDark = ThemeVariables(
    colorPrimary: Colors.omarchyVantablackDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF08_0808),
    colorSurface: Color(argb: 0xFF00_0000),
    colorSurfaceMuted: Color(argb: 0xFF0B_0B0B),
    colorSurfaceSunken: Color(argb: 0xFF1C_1C1C),
    colorSurfaceSubtle: Color(argb: 0x13FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF14_1414),
    colorSurfaceRaised: Color(argb: 0xFF31_3131),
    colorSurfaceOverlay: Color(argb: 0xFF0B_0B0B),
    colorSurfaceChrome: Color(argb: 0xFF0B_0B0B),
    colorSurfaceColumn: Color(argb: 0xFF06_0606),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD1_D1D1),
    colorContentNav: Color(argb: 0xFFB8_B8B8),
    colorContentMuted: Color(argb: 0xFFA3_A3A3),
    colorContentSubtle: Color(argb: 0xFF80_8080),
    colorContentFaint: Color(argb: 0xFF5C_5C5C),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF61_6161),
    colorOnAccent: Color(argb: 0xFF00_0000),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyVantablackDark[500],
    progressGradientTo: Colors.omarchyVantablackDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `omarchy-white-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyWhiteLight = ThemeVariables(
    colorPrimary: Colors.omarchyWhiteLight,
    colorCanvas: Color(argb: 0xFFED_EDED),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF7_F7F7),
    colorSurfaceSunken: Color(argb: 0xFFE0_E0E0),
    colorSurfaceSubtle: Color(argb: 0x0F00_0000),
    colorSurfaceInset: Color(argb: 0xFFEF_EFEF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF7_F7F7),
    colorSurfaceColumn: Color(argb: 0xFFFA_FAFA),
    colorContent: Color(argb: 0xFF00_0000),
    colorContentSecondary: Color(argb: 0xFF2E_2E2E),
    colorContentNav: Color(argb: 0xFF47_4747),
    colorContentMuted: Color(argb: 0xFF5C_5C5C),
    colorContentSubtle: Color(argb: 0xFF80_8080),
    colorContentFaint: Color(argb: 0xFFA3_A3A3),
    colorBorder: Color(argb: 0x2100_0000),
    colorBorderStrong: Color(argb: 0x2B00_0000),
    colorBorderMuted: Color(argb: 0xFF9E_9E9E),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyWhiteLight[600],
    progressGradientTo: Colors.omarchyWhiteLight[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15
  )

  /// The `omarchy-white-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let omarchyWhiteDark = ThemeVariables(
    colorPrimary: Colors.omarchyWhiteDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF08_0808),
    colorSurface: Color(argb: 0xFF00_0000),
    colorSurfaceMuted: Color(argb: 0xFF0B_0B0B),
    colorSurfaceSunken: Color(argb: 0xFF1C_1C1C),
    colorSurfaceSubtle: Color(argb: 0x13FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF14_1414),
    colorSurfaceRaised: Color(argb: 0xFF31_3131),
    colorSurfaceOverlay: Color(argb: 0xFF0B_0B0B),
    colorSurfaceChrome: Color(argb: 0xFF0B_0B0B),
    colorSurfaceColumn: Color(argb: 0xFF06_0606),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD1_D1D1),
    colorContentNav: Color(argb: 0xFFB8_B8B8),
    colorContentMuted: Color(argb: 0xFFA3_A3A3),
    colorContentSubtle: Color(argb: 0xFF80_8080),
    colorContentFaint: Color(argb: 0xFF5C_5C5C),
    colorBorder: Color(argb: 0x29FF_FFFF),
    colorBorderStrong: Color(argb: 0x33FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF61_6161),
    colorOnAccent: Color(argb: 0xFF00_0000),
    controlFieldRadius: 0.0,
    controlContainerRadius: 0.0,
    controlMediumPaddingInline: 10.0,
    controlMediumSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 0.0,
    framePopoverRadius: 0.0,
    radiusTiny: 0.0,
    radiusSmall: 0.0,
    radiusMedium: 0.0,
    radiusLarge: 0.0,
    radiusBig: 0.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 17.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 19.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 17.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .medium, lineHeight: 12.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .bold, lineHeight: 12.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 0.0,
    progressGradientFrom: Colors.omarchyWhiteDark[500],
    progressGradientTo: Colors.omarchyWhiteDark[300],
    sliderMediumTrack: 4.0,
    sliderMediumThumb: 14.0,
    sliderThumbInk: 1,
    sliderThumbLift: 0,
    sliderThumbEdgeWidth: 2.0,
    sliderThumbHotScale: 1.15,
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

  /// The `macos27-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let macos27Light = ThemeVariables(
    colorPrimary: Colors.macos27Light,
    colorCanvas: Color(argb: 0xFFE3_E3E5),
    colorSurface: Color(argb: 0xFFF5_F5F7),
    colorSurfaceMuted: Color(argb: 0xFFED_EDEF),
    colorSurfaceSunken: Color(argb: 0xFFD7_D7D9),
    colorSurfaceSubtle: Color(argb: 0x1025_2525),
    colorSurfaceInset: Color(argb: 0xFFE5_E5E7),
    colorSurfaceRaised: Color(argb: 0xFFF5_F5F7),
    colorSurfaceOverlay: Color(argb: 0xFFF5_F5F7),
    colorSurfaceChrome: Color(argb: 0xFFED_EDEF),
    colorSurfaceColumn: Color(argb: 0xFFF0_F0F2),
    colorContent: Color(argb: 0xFF25_2525),
    colorContentSecondary: Color(argb: 0xFF4A_4A4B),
    colorContentNav: Color(argb: 0xFF5F_5F60),
    colorContentMuted: Color(argb: 0xFF6D_6D6E),
    colorContentSubtle: Color(argb: 0xFF8D_8D8E),
    colorContentFaint: Color(argb: 0xFFAA_AAAB),
    colorBorder: Color(argb: 0x1225_2525),
    colorBorderStrong: Color(argb: 0x1A25_2525),
    colorBorderMuted: Color(argb: 0xFFA6_A6A7),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 8.0,
    controlContainerRadius: 10.0,
    controlTinySize: 20.0,
    controlSmallSize: 24.0,
    controlMediumPaddingInline: 18.0,
    controlMediumSize: 28.0,
    controlLargeSize: 32.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 16.0,
    framePopoverRadius: 16.0,
    radiusTiny: 6.0,
    radiusSmall: 9999.0,
    radiusMedium: 9999.0,
    radiusLarge: 10.0,
    radiusBig: 16.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .semibold, lineHeight: 20.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 17.0, weight: .semibold, lineHeight: 22.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .regular, lineHeight: 20.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .medium, lineHeight: 13.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .bold, lineHeight: 13.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 4.0,
    progressGradientFrom: Colors.macos27Light[600],
    progressGradientTo: Colors.macos27Light[300]
  )

  /// The `macos27-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let macos27Dark = ThemeVariables(
    colorPrimary: Colors.macos27Dark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1E_1E21),
    colorSurface: Color(argb: 0xFF23_2326),
    colorSurfaceMuted: Color(argb: 0xFF2A_2A2C),
    colorSurfaceSunken: Color(argb: 0xFF39_393B),
    colorSurfaceSubtle: Color(argb: 0x12DE_DEDF),
    colorSurfaceInset: Color(argb: 0xFF30_3032),
    colorSurfaceRaised: Color(argb: 0xFF50_5052),
    colorSurfaceOverlay: Color(argb: 0xFF2A_2A2C),
    colorSurfaceChrome: Color(argb: 0xFF2A_2A2C),
    colorSurfaceColumn: Color(argb: 0xFF20_2022),
    colorContent: Color(argb: 0xFFDE_DEDF),
    colorContentSecondary: Color(argb: 0xFFBD_BDBD),
    colorContentNav: Color(argb: 0xFFAA_AAAB),
    colorContentMuted: Color(argb: 0xFF9B_9B9C),
    colorContentSubtle: Color(argb: 0xFF81_8182),
    colorContentFaint: Color(argb: 0xFF66_6668),
    colorBorder: Color(argb: 0x14DE_DEDF),
    colorBorderStrong: Color(argb: 0x1CDE_DEDF),
    colorBorderMuted: Color(argb: 0xFF6A_6A6C),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 8.0,
    controlContainerRadius: 10.0,
    controlTinySize: 20.0,
    controlSmallSize: 24.0,
    controlMediumPaddingInline: 18.0,
    controlMediumSize: 28.0,
    controlLargeSize: 32.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 16.0,
    framePopoverRadius: 16.0,
    radiusTiny: 6.0,
    radiusSmall: 9999.0,
    radiusMedium: 9999.0,
    radiusLarge: 10.0,
    radiusBig: 16.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .semibold, lineHeight: 20.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 17.0, weight: .semibold, lineHeight: 22.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .regular, lineHeight: 20.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .medium, lineHeight: 13.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .bold, lineHeight: 13.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 4.0,
    progressGradientFrom: Colors.macos27Dark[500],
    progressGradientTo: Colors.macos27Dark[300],
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

  /// The `macos15-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let macos15Light = ThemeVariables(
    colorPrimary: Colors.macos15Light,
    colorCanvas: Color(argb: 0xFFDA_DADA),
    colorSurface: Color(argb: 0xFFEC_ECEC),
    colorSurfaceMuted: Color(argb: 0xFFE4_E4E4),
    colorSurfaceSunken: Color(argb: 0xFFCE_CECE),
    colorSurfaceSubtle: Color(argb: 0x1023_2323),
    colorSurfaceInset: Color(argb: 0xFFDC_DCDC),
    colorSurfaceRaised: Color(argb: 0xFFEC_ECEC),
    colorSurfaceOverlay: Color(argb: 0xFFEC_ECEC),
    colorSurfaceChrome: Color(argb: 0xFFE4_E4E4),
    colorSurfaceColumn: Color(argb: 0xFFE7_E7E7),
    colorContent: Color(argb: 0xFF23_2323),
    colorContentSecondary: Color(argb: 0xFF47_4747),
    colorContentNav: Color(argb: 0xFF5B_5B5B),
    colorContentMuted: Color(argb: 0xFF69_6969),
    colorContentSubtle: Color(argb: 0xFF88_8888),
    colorContentFaint: Color(argb: 0xFFA4_A4A4),
    colorBorder: Color(argb: 0x1A23_2323),
    colorBorderStrong: Color(argb: 0x2123_2323),
    colorBorderMuted: Color(argb: 0xFFA0_A0A0),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 6.0,
    controlContainerRadius: 6.0,
    controlTinySize: 16.0,
    controlSmallSize: 19.0,
    controlMediumPaddingInline: 12.0,
    controlMediumSize: 22.0,
    controlLargeSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 10.0,
    framePopoverRadius: 10.0,
    radiusTiny: 4.0,
    radiusSmall: 6.0,
    radiusMedium: 6.0,
    radiusLarge: 6.0,
    radiusBig: 10.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .semibold, lineHeight: 20.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 17.0, weight: .semibold, lineHeight: 22.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .regular, lineHeight: 20.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .medium, lineHeight: 13.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .bold, lineHeight: 13.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 3.0,
    progressGradientFrom: Colors.macos15Light[600],
    progressGradientTo: Colors.macos15Light[300]
  )

  /// The `macos15-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let macos15Dark = ThemeVariables(
    colorPrimary: Colors.macos15Dark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF2D_2D2D),
    colorSurface: Color(argb: 0xFF32_3232),
    colorSurfaceMuted: Color(argb: 0xFF39_3939),
    colorSurfaceSunken: Color(argb: 0xFF49_4949),
    colorSurfaceSubtle: Color(argb: 0x13E0_E0E0),
    colorSurfaceInset: Color(argb: 0xFF40_4040),
    colorSurfaceRaised: Color(argb: 0xFF61_6161),
    colorSurfaceOverlay: Color(argb: 0xFF39_3939),
    colorSurfaceChrome: Color(argb: 0xFF39_3939),
    colorSurfaceColumn: Color(argb: 0xFF2E_2E2E),
    colorContent: Color(argb: 0xFFE0_E0E0),
    colorContentSecondary: Color(argb: 0xFFC1_C1C1),
    colorContentNav: Color(argb: 0xFFB0_B0B0),
    colorContentMuted: Color(argb: 0xFFA2_A2A2),
    colorContentSubtle: Color(argb: 0xFF89_8989),
    colorContentFaint: Color(argb: 0xFF71_7171),
    colorBorder: Color(argb: 0x1AE0_E0E0),
    colorBorderStrong: Color(argb: 0x21E0_E0E0),
    colorBorderMuted: Color(argb: 0xFF74_7474),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 6.0,
    controlContainerRadius: 6.0,
    controlTinySize: 16.0,
    controlSmallSize: 19.0,
    controlMediumPaddingInline: 12.0,
    controlMediumSize: 22.0,
    controlLargeSize: 28.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 10.0,
    framePopoverRadius: 10.0,
    radiusTiny: 4.0,
    radiusSmall: 6.0,
    radiusMedium: 6.0,
    radiusLarge: 6.0,
    radiusBig: 10.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 16.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .semibold, lineHeight: 20.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 17.0, weight: .semibold, lineHeight: 22.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 16.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 15.0, weight: .regular, lineHeight: 20.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .medium, lineHeight: 13.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .bold, lineHeight: 13.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .semibold, lineHeight: 10.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .semibold, lineHeight: 13.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.0, weight: .regular, lineHeight: 13.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 14.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.0, weight: .regular, lineHeight: 16.0),
    checkboxRadius: 3.0,
    progressGradientFrom: Colors.macos15Dark[500],
    progressGradientTo: Colors.macos15Dark[300],
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

  /// The `windows11-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let windows11Light = ThemeVariables(
    colorPrimary: Colors.windows11Light,
    colorCanvas: Color(argb: 0xFFE1_E1E1),
    colorSurface: Color(argb: 0xFFF3_F3F3),
    colorSurfaceMuted: Color(argb: 0xFFEB_EBEB),
    colorSurfaceSunken: Color(argb: 0xFFD5_D5D5),
    colorSurfaceSubtle: Color(argb: 0x0F18_1818),
    colorSurfaceInset: Color(argb: 0xFFE3_E3E3),
    colorSurfaceRaised: Color(argb: 0xFFF3_F3F3),
    colorSurfaceOverlay: Color(argb: 0xFFF3_F3F3),
    colorSurfaceChrome: Color(argb: 0xFFEB_EBEB),
    colorSurfaceColumn: Color(argb: 0xFFEE_EEEE),
    colorContent: Color(argb: 0xFF18_1818),
    colorContentSecondary: Color(argb: 0xFF3F_3F3F),
    colorContentNav: Color(argb: 0xFF55_5555),
    colorContentMuted: Color(argb: 0xFF67_6767),
    colorContentSubtle: Color(argb: 0xFF85_8585),
    colorContentFaint: Color(argb: 0xFFA4_A4A4),
    colorBorder: Color(argb: 0x1418_1818),
    colorBorderStrong: Color(argb: 0x1C18_1818),
    colorBorderMuted: Color(argb: 0xFFA0_A0A0),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 4.0,
    controlContainerRadius: 4.0,
    controlTinySize: 24.0,
    controlSmallSize: 28.0,
    controlMediumPaddingInline: 11.0,
    controlMediumSize: 32.0,
    controlLargeSize: 40.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 8.0,
    framePopoverRadius: 8.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 4.0,
    radiusSmall: 4.0,
    radiusMedium: 4.0,
    radiusLarge: 8.0,
    radiusBig: 8.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 18.0, weight: .semibold, lineHeight: 24.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 28.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 18.0, weight: .regular, lineHeight: 24.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .medium, lineHeight: 14.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .bold, lineHeight: 14.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 14.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 14.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 4.0,
    progressGradientFrom: Colors.windows11Light[600],
    progressGradientTo: Colors.windows11Light[300]
  )

  /// The `windows11-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let windows11Dark = ThemeVariables(
    colorPrimary: Colors.windows11Dark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1B_1B1B),
    colorSurface: Color(argb: 0xFF20_2020),
    colorSurfaceMuted: Color(argb: 0xFF27_2727),
    colorSurfaceSunken: Color(argb: 0xFF36_3636),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF2D_2D2D),
    colorSurfaceRaised: Color(argb: 0xFF4C_4C4C),
    colorSurfaceOverlay: Color(argb: 0xFF27_2727),
    colorSurfaceChrome: Color(argb: 0xFF27_2727),
    colorSurfaceColumn: Color(argb: 0xFF1C_1C1C),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD7_D7D7),
    colorContentNav: Color(argb: 0xFFC1_C1C1),
    colorContentMuted: Color(argb: 0xFFAF_AFAF),
    colorContentSubtle: Color(argb: 0xFF90_9090),
    colorContentFaint: Color(argb: 0xFF70_7070),
    colorBorder: Color(argb: 0x14FF_FFFF),
    colorBorderStrong: Color(argb: 0x1CFF_FFFF),
    colorBorderMuted: Color(argb: 0xFF75_7575),
    colorOnAccent: Color(argb: 0xFF00_0000),
    controlFieldRadius: 4.0,
    controlContainerRadius: 4.0,
    controlTinySize: 24.0,
    controlSmallSize: 28.0,
    controlMediumPaddingInline: 11.0,
    controlMediumSize: 32.0,
    controlLargeSize: 40.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 8.0,
    framePopoverRadius: 8.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 4.0,
    radiusSmall: 4.0,
    radiusMedium: 4.0,
    radiusLarge: 8.0,
    radiusBig: 8.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 18.0, weight: .semibold, lineHeight: 24.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 28.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 18.0, weight: .regular, lineHeight: 24.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .medium, lineHeight: 14.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .bold, lineHeight: 14.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 14.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 14.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 4.0,
    progressGradientFrom: Colors.windows11Dark[500],
    progressGradientTo: Colors.windows11Dark[300],
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

  /// The `ubuntu-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let ubuntuLight = ThemeVariables(
    colorPrimary: Colors.ubuntuLight,
    colorCanvas: Color(argb: 0xFFE8_E8E8),
    colorSurface: Color(argb: 0xFFFA_FAFA),
    colorSurfaceMuted: Color(argb: 0xFFF2_F2F2),
    colorSurfaceSunken: Color(argb: 0xFFDC_DCDC),
    colorSurfaceSubtle: Color(argb: 0x113D_3D3D),
    colorSurfaceInset: Color(argb: 0xFFEA_EAEA),
    colorSurfaceRaised: Color(argb: 0xFFFA_FAFA),
    colorSurfaceOverlay: Color(argb: 0xFFFA_FAFA),
    colorSurfaceChrome: Color(argb: 0xFFF2_F2F2),
    colorSurfaceColumn: Color(argb: 0xFFF5_F5F5),
    colorContent: Color(argb: 0xFF3D_3D3D),
    colorContentSecondary: Color(argb: 0xFF5F_5F5F),
    colorContentNav: Color(argb: 0xFF72_7272),
    colorContentMuted: Color(argb: 0xFF72_7272),
    colorContentSubtle: Color(argb: 0xFF9C_9C9C),
    colorContentFaint: Color(argb: 0xFFB6_B6B6),
    colorBorder: Color(argb: 0x1A3D_3D3D),
    colorBorderStrong: Color(argb: 0x213D_3D3D),
    colorBorderMuted: Color(argb: 0xFFB2_B2B2),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 9.0,
    controlContainerRadius: 12.0,
    controlTinySize: 24.0,
    controlSmallSize: 28.0,
    controlMediumPaddingInline: 12.0,
    controlMediumSize: 34.0,
    controlLargeSize: 40.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 15.0,
    framePopoverRadius: 12.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 6.0,
    radiusSmall: 9.0,
    radiusMedium: 9.0,
    radiusLarge: 12.0,
    radiusBig: 12.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .semibold, lineHeight: 22.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 26.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .regular, lineHeight: 22.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .medium, lineHeight: 14.5),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .bold, lineHeight: 14.5),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 15.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 6.0,
    progressGradientFrom: Colors.ubuntuLight[600],
    progressGradientTo: Colors.ubuntuLight[300]
  )

  /// The `ubuntu-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let ubuntuDark = ThemeVariables(
    colorPrimary: Colors.ubuntuDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF27_2727),
    colorSurface: Color(argb: 0xFF2C_2C2C),
    colorSurfaceMuted: Color(argb: 0xFF33_3333),
    colorSurfaceSunken: Color(argb: 0xFF43_4343),
    colorSurfaceSubtle: Color(argb: 0x12F7_F7F7),
    colorSurfaceInset: Color(argb: 0xFF3A_3A3A),
    colorSurfaceRaised: Color(argb: 0xFF5A_5A5A),
    colorSurfaceOverlay: Color(argb: 0xFF33_3333),
    colorSurfaceChrome: Color(argb: 0xFF33_3333),
    colorSurfaceColumn: Color(argb: 0xFF28_2828),
    colorContent: Color(argb: 0xFFF7_F7F7),
    colorContentSecondary: Color(argb: 0xFFD2_D2D2),
    colorContentNav: Color(argb: 0xFFBE_BEBE),
    colorContentMuted: Color(argb: 0xFFAE_AEAE),
    colorContentSubtle: Color(argb: 0xFF92_9292),
    colorContentFaint: Color(argb: 0xFF75_7575),
    colorBorder: Color(argb: 0x1AF7_F7F7),
    colorBorderStrong: Color(argb: 0x21F7_F7F7),
    colorBorderMuted: Color(argb: 0xFF79_7979),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 9.0,
    controlContainerRadius: 12.0,
    controlTinySize: 24.0,
    controlSmallSize: 28.0,
    controlMediumPaddingInline: 12.0,
    controlMediumSize: 34.0,
    controlLargeSize: 40.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 15.0,
    framePopoverRadius: 12.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 6.0,
    radiusSmall: 9.0,
    radiusMedium: 9.0,
    radiusLarge: 12.0,
    radiusBig: 12.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .semibold, lineHeight: 22.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 26.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .regular, lineHeight: 22.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .medium, lineHeight: 14.5),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .bold, lineHeight: 14.5),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 15.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 6.0,
    progressGradientFrom: Colors.ubuntuDark[500],
    progressGradientTo: Colors.ubuntuDark[300],
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

  /// The `debian-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let debianLight = ThemeVariables(
    colorPrimary: Colors.debianLight,
    colorCanvas: Color(argb: 0xFFE8_E8EA),
    colorSurface: Color(argb: 0xFFFA_FAFB),
    colorSurfaceMuted: Color(argb: 0xFFF2_F2F4),
    colorSurfaceSunken: Color(argb: 0xFFDC_DCDD),
    colorSurfaceSubtle: Color(argb: 0x1132_3237),
    colorSurfaceInset: Color(argb: 0xFFEA_EAEC),
    colorSurfaceRaised: Color(argb: 0xFFFA_FAFB),
    colorSurfaceOverlay: Color(argb: 0xFFFA_FAFB),
    colorSurfaceChrome: Color(argb: 0xFFF2_F2F4),
    colorSurfaceColumn: Color(argb: 0xFFF5_F5F7),
    colorContent: Color(argb: 0xFF32_3237),
    colorContentSecondary: Color(argb: 0xFF56_565A),
    colorContentNav: Color(argb: 0xFF6A_6A6E),
    colorContentMuted: Color(argb: 0xFF70_7074),
    colorContentSubtle: Color(argb: 0xFF96_9699),
    colorContentFaint: Color(argb: 0xFFB2_B2B4),
    colorBorder: Color(argb: 0x1A32_3237),
    colorBorderStrong: Color(argb: 0x2132_3237),
    colorBorderMuted: Color(argb: 0xFFAE_AEB1),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 9.0,
    controlContainerRadius: 12.0,
    controlTinySize: 24.0,
    controlSmallSize: 28.0,
    controlMediumPaddingInline: 12.0,
    controlMediumSize: 34.0,
    controlLargeSize: 40.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 15.0,
    framePopoverRadius: 12.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 6.0,
    radiusSmall: 9.0,
    radiusMedium: 9.0,
    radiusLarge: 12.0,
    radiusBig: 12.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .semibold, lineHeight: 22.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 26.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .regular, lineHeight: 22.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .medium, lineHeight: 14.5),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .bold, lineHeight: 14.5),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 15.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 6.0,
    progressGradientFrom: Colors.debianLight[600],
    progressGradientTo: Colors.debianLight[300]
  )

  /// The `debian-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let debianDark = ThemeVariables(
    colorPrimary: Colors.debianDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1E_1E21),
    colorSurface: Color(argb: 0xFF22_2226),
    colorSurfaceMuted: Color(argb: 0xFF29_292C),
    colorSurfaceSunken: Color(argb: 0xFF38_383B),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF2F_2F32),
    colorSurfaceRaised: Color(argb: 0xFF4F_4F52),
    colorSurfaceOverlay: Color(argb: 0xFF29_292C),
    colorSurfaceChrome: Color(argb: 0xFF29_292C),
    colorSurfaceColumn: Color(argb: 0xFF1F_1F22),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD7_D7D8),
    colorContentNav: Color(argb: 0xFFC1_C1C2),
    colorContentMuted: Color(argb: 0xFFAF_AFB1),
    colorContentSubtle: Color(argb: 0xFF91_9193),
    colorContentFaint: Color(argb: 0xFF72_7274),
    colorBorder: Color(argb: 0x1AFF_FFFF),
    colorBorderStrong: Color(argb: 0x21FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF76_7678),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 9.0,
    controlContainerRadius: 12.0,
    controlTinySize: 24.0,
    controlSmallSize: 28.0,
    controlMediumPaddingInline: 12.0,
    controlMediumSize: 34.0,
    controlLargeSize: 40.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 15.0,
    framePopoverRadius: 12.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 6.0,
    radiusSmall: 9.0,
    radiusMedium: 9.0,
    radiusLarge: 12.0,
    radiusBig: 12.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .semibold, lineHeight: 22.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 26.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .regular, lineHeight: 22.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .medium, lineHeight: 14.5),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .bold, lineHeight: 14.5),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 15.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 6.0,
    progressGradientFrom: Colors.debianDark[500],
    progressGradientTo: Colors.debianDark[300],
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

  /// The `fedora-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let fedoraLight = ThemeVariables(
    colorPrimary: Colors.fedoraLight,
    colorCanvas: Color(argb: 0xFFE8_E8EA),
    colorSurface: Color(argb: 0xFFFA_FAFB),
    colorSurfaceMuted: Color(argb: 0xFFF2_F2F4),
    colorSurfaceSunken: Color(argb: 0xFFDC_DCDD),
    colorSurfaceSubtle: Color(argb: 0x1132_3237),
    colorSurfaceInset: Color(argb: 0xFFEA_EAEC),
    colorSurfaceRaised: Color(argb: 0xFFFA_FAFB),
    colorSurfaceOverlay: Color(argb: 0xFFFA_FAFB),
    colorSurfaceChrome: Color(argb: 0xFFF2_F2F4),
    colorSurfaceColumn: Color(argb: 0xFFF5_F5F7),
    colorContent: Color(argb: 0xFF32_3237),
    colorContentSecondary: Color(argb: 0xFF56_565A),
    colorContentNav: Color(argb: 0xFF6A_6A6E),
    colorContentMuted: Color(argb: 0xFF70_7074),
    colorContentSubtle: Color(argb: 0xFF96_9699),
    colorContentFaint: Color(argb: 0xFFB2_B2B4),
    colorBorder: Color(argb: 0x1A32_3237),
    colorBorderStrong: Color(argb: 0x2132_3237),
    colorBorderMuted: Color(argb: 0xFFAE_AEB1),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 9.0,
    controlContainerRadius: 12.0,
    controlTinySize: 24.0,
    controlSmallSize: 28.0,
    controlMediumPaddingInline: 12.0,
    controlMediumSize: 34.0,
    controlLargeSize: 40.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 15.0,
    framePopoverRadius: 12.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 6.0,
    radiusSmall: 9.0,
    radiusMedium: 9.0,
    radiusLarge: 12.0,
    radiusBig: 12.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .semibold, lineHeight: 22.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 26.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .regular, lineHeight: 22.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .medium, lineHeight: 14.5),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .bold, lineHeight: 14.5),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 15.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 6.0,
    progressGradientFrom: Colors.fedoraLight[600],
    progressGradientTo: Colors.fedoraLight[300]
  )

  /// The `fedora-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let fedoraDark = ThemeVariables(
    colorPrimary: Colors.fedoraDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1E_1E21),
    colorSurface: Color(argb: 0xFF22_2226),
    colorSurfaceMuted: Color(argb: 0xFF29_292C),
    colorSurfaceSunken: Color(argb: 0xFF38_383B),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF2F_2F32),
    colorSurfaceRaised: Color(argb: 0xFF4F_4F52),
    colorSurfaceOverlay: Color(argb: 0xFF29_292C),
    colorSurfaceChrome: Color(argb: 0xFF29_292C),
    colorSurfaceColumn: Color(argb: 0xFF1F_1F22),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFD7_D7D8),
    colorContentNav: Color(argb: 0xFFC1_C1C2),
    colorContentMuted: Color(argb: 0xFFAF_AFB1),
    colorContentSubtle: Color(argb: 0xFF91_9193),
    colorContentFaint: Color(argb: 0xFF72_7274),
    colorBorder: Color(argb: 0x1AFF_FFFF),
    colorBorderStrong: Color(argb: 0x21FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF76_7678),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 9.0,
    controlContainerRadius: 12.0,
    controlTinySize: 24.0,
    controlSmallSize: 28.0,
    controlMediumPaddingInline: 12.0,
    controlMediumSize: 34.0,
    controlLargeSize: 40.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 15.0,
    framePopoverRadius: 12.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 6.0,
    radiusSmall: 9.0,
    radiusMedium: 9.0,
    radiusLarge: 12.0,
    radiusBig: 12.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .semibold, lineHeight: 22.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 20.0, weight: .semibold, lineHeight: 26.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.5, weight: .regular, lineHeight: 22.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .medium, lineHeight: 14.5),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .bold, lineHeight: 14.5),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .semibold, lineHeight: 12.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .semibold, lineHeight: 14.5),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 15.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.5, weight: .regular, lineHeight: 17.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.5, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 6.0,
    progressGradientFrom: Colors.fedoraDark[500],
    progressGradientTo: Colors.fedoraDark[300],
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

  /// The `kde-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let kdeLight = ThemeVariables(
    colorPrimary: Colors.kdeLight,
    colorCanvas: Color(argb: 0xFFDD_DFE0),
    colorSurface: Color(argb: 0xFFEF_F0F1),
    colorSurfaceMuted: Color(argb: 0xFFE7_E9EA),
    colorSurfaceSunken: Color(argb: 0xFFD1_D2D4),
    colorSurfaceSubtle: Color(argb: 0x1023_2629),
    colorSurfaceInset: Color(argb: 0xFFDF_E0E2),
    colorSurfaceRaised: Color(argb: 0xFFEF_F0F1),
    colorSurfaceOverlay: Color(argb: 0xFFEF_F0F1),
    colorSurfaceChrome: Color(argb: 0xFFE7_E9EA),
    colorSurfaceColumn: Color(argb: 0xFFEA_EBED),
    colorContent: Color(argb: 0xFF23_2629),
    colorContentSecondary: Color(argb: 0xFF48_4A4D),
    colorContentNav: Color(argb: 0xFF5C_5F61),
    colorContentMuted: Color(argb: 0xFF68_6B6D),
    colorContentSubtle: Color(argb: 0xFF89_8B8D),
    colorContentFaint: Color(argb: 0xFFA6_A7A9),
    colorBorder: Color(argb: 0x2123_2629),
    colorBorderStrong: Color(argb: 0x2B23_2629),
    colorBorderMuted: Color(argb: 0xFFA1_A3A5),
    colorOnAccent: Color(argb: 0xFF23_2629),
    controlFieldRadius: 5.0,
    controlContainerRadius: 5.0,
    controlTinySize: 22.0,
    controlSmallSize: 26.0,
    controlMediumPaddingInline: 6.0,
    controlMediumSize: 30.0,
    controlLargeSize: 36.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 5.0,
    framePopoverRadius: 5.0,
    radiusTiny: 3.0,
    radiusSmall: 5.0,
    radiusMedium: 5.0,
    radiusLarge: 5.0,
    radiusBig: 5.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .semibold, lineHeight: 18.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.0, weight: .semibold, lineHeight: 21.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 18.67, weight: .semibold, lineHeight: 24.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.5, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .regular, lineHeight: 18.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.0, weight: .regular, lineHeight: 21.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .medium, lineHeight: 13.33),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .bold, lineHeight: 13.33),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.67, weight: .semibold, lineHeight: 10.67),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .semibold, lineHeight: 13.33),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .semibold, lineHeight: 13.33),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.67, weight: .regular, lineHeight: 14.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.5, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .regular, lineHeight: 18.0),
    checkboxRadius: 3.0,
    progressGradientFrom: Colors.kdeLight[600],
    progressGradientTo: Colors.kdeLight[300]
  )

  /// The `kde-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let kdeDark = ThemeVariables(
    colorPrimary: Colors.kdeDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF1C_1E21),
    colorSurface: Color(argb: 0xFF20_2326),
    colorSurfaceMuted: Color(argb: 0xFF27_2A2C),
    colorSurfaceSunken: Color(argb: 0xFF36_393B),
    colorSurfaceSubtle: Color(argb: 0x12FC_FCFC),
    colorSurfaceInset: Color(argb: 0xFF2D_3032),
    colorSurfaceRaised: Color(argb: 0xFF4D_5052),
    colorSurfaceOverlay: Color(argb: 0xFF27_2A2C),
    colorSurfaceChrome: Color(argb: 0xFF27_2A2C),
    colorSurfaceColumn: Color(argb: 0xFF1D_2022),
    colorContent: Color(argb: 0xFFFC_FCFC),
    colorContentSecondary: Color(argb: 0xFFD4_D5D5),
    colorContentNav: Color(argb: 0xFFBE_BFC0),
    colorContentMuted: Color(argb: 0xFFAD_AEAF),
    colorContentSubtle: Color(argb: 0xFF8E_9091),
    colorContentFaint: Color(argb: 0xFF6F_7173),
    colorBorder: Color(argb: 0x29FC_FCFC),
    colorBorderStrong: Color(argb: 0x33FC_FCFC),
    colorBorderMuted: Color(argb: 0xFF74_7577),
    colorOnAccent: Color(argb: 0xFF20_2326),
    controlFieldRadius: 5.0,
    controlContainerRadius: 5.0,
    controlTinySize: 22.0,
    controlSmallSize: 26.0,
    controlMediumPaddingInline: 6.0,
    controlMediumSize: 30.0,
    controlLargeSize: 36.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 5.0,
    framePopoverRadius: 5.0,
    radiusTiny: 3.0,
    radiusSmall: 5.0,
    radiusMedium: 5.0,
    radiusLarge: 5.0,
    radiusBig: 5.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .semibold, lineHeight: 18.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.0, weight: .semibold, lineHeight: 21.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 18.67, weight: .semibold, lineHeight: 24.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.5, weight: .regular, lineHeight: 15.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .regular, lineHeight: 18.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.0, weight: .regular, lineHeight: 21.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .medium, lineHeight: 13.33),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .bold, lineHeight: 13.33),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.67, weight: .semibold, lineHeight: 10.67),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .semibold, lineHeight: 13.33),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .semibold, lineHeight: 13.33),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 10.67, weight: .regular, lineHeight: 14.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.5, weight: .regular, lineHeight: 15.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 13.33, weight: .regular, lineHeight: 18.0),
    checkboxRadius: 3.0,
    progressGradientFrom: Colors.kdeDark[500],
    progressGradientTo: Colors.kdeDark[300],
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

  /// The `spotify-light` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let spotifyLight = ThemeVariables(
    colorPrimary: Colors.spotifyLight,
    colorCanvas: Color(argb: 0xFFED_EDED),
    colorSurface: Color(argb: 0xFFFF_FFFF),
    colorSurfaceMuted: Color(argb: 0xFFF7_F7F7),
    colorSurfaceSunken: Color(argb: 0xFFE0_E0E0),
    colorSurfaceSubtle: Color(argb: 0x0F12_1212),
    colorSurfaceInset: Color(argb: 0xFFEF_EFEF),
    colorSurfaceRaised: Color(argb: 0xFFFF_FFFF),
    colorSurfaceOverlay: Color(argb: 0xFFFF_FFFF),
    colorSurfaceChrome: Color(argb: 0xFFF7_F7F7),
    colorSurfaceColumn: Color(argb: 0xFFFA_FAFA),
    colorContent: Color(argb: 0xFF12_1212),
    colorContentSecondary: Color(argb: 0xFF3D_3D3D),
    colorContentNav: Color(argb: 0xFF54_5454),
    colorContentMuted: Color(argb: 0xFF67_6767),
    colorContentSubtle: Color(argb: 0xFF89_8989),
    colorContentFaint: Color(argb: 0xFFAA_AAAA),
    colorBorder: Color(argb: 0x1A12_1212),
    colorBorderStrong: Color(argb: 0x2112_1212),
    colorBorderMuted: Color(argb: 0xFFA5_A5A5),
    colorOnAccent: Color(argb: 0xFFFF_FFFF),
    controlFieldRadius: 4.0,
    controlContainerRadius: 8.0,
    controlTinySize: 24.0,
    controlSmallSize: 32.0,
    controlMediumPaddingInline: 24.0,
    controlMediumSize: 40.0,
    controlLargeSize: 48.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x1A00_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x1F00_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 12.0, spread: 0.0, color: Color(argb: 0x2400_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color(argb: 0x3300_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 10.0, blur: 30.0, spread: 0.0, color: Color(argb: 0x3800_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 12.0, blur: 32.0, spread: 0.0, color: Color(argb: 0x3D00_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 56.0, spread: 0.0, color: Color(argb: 0x4700_0000))],
    frameWindowRadius: 8.0,
    framePopoverRadius: 8.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 4.0,
    radiusSmall: 9999.0,
    radiusMedium: 9999.0,
    radiusLarge: 8.0,
    radiusBig: 8.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.0, weight: .semibold, lineHeight: 24.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 24.0, weight: .semibold, lineHeight: 32.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.0, weight: .regular, lineHeight: 24.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .medium, lineHeight: 14.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .bold, lineHeight: 14.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .semibold, lineHeight: 11.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 14.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 14.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 16.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 4.0,
    progressGradientFrom: Colors.spotifyLight[600],
    progressGradientTo: Colors.spotifyLight[300]
  )

  /// The `spotify-dark` counterpart of ``brightLight``.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let spotifyDark = ThemeVariables(
    colorPrimary: Colors.spotifyDark,
    colorSuccess: Colors.greenDark,
    colorWarning: Colors.amberDark,
    colorDanger: Colors.redDark,
    colorCanvas: Color(argb: 0xFF00_0000),
    colorSurface: Color(argb: 0xFF12_1212),
    colorSurfaceMuted: Color(argb: 0xFF1F_1F1F),
    colorSurfaceSunken: Color(argb: 0xFF33_3333),
    colorSurfaceSubtle: Color(argb: 0x12FF_FFFF),
    colorSurfaceInset: Color(argb: 0xFF2A_2A2A),
    colorSurfaceRaised: Color(argb: 0xFF53_5353),
    colorSurfaceOverlay: Color(argb: 0xFF28_2828),
    colorSurfaceChrome: Color(argb: 0xFF1F_1F1F),
    colorSurfaceColumn: Color(argb: 0xFF0A_0A0A),
    colorContent: Color(argb: 0xFFFF_FFFF),
    colorContentSecondary: Color(argb: 0xFFB3_B3B3),
    colorContentNav: Color(argb: 0xFFB3_B3B3),
    colorContentMuted: Color(argb: 0xFFB3_B3B3),
    colorContentSubtle: Color(argb: 0xFF89_8989),
    colorContentFaint: Color(argb: 0xFF67_6767),
    colorBorder: Color(argb: 0x1AFF_FFFF),
    colorBorderStrong: Color(argb: 0x21FF_FFFF),
    colorBorderMuted: Color(argb: 0xFF6C_6C6C),
    colorOnAccent: Color(argb: 0xFF00_0000),
    controlFieldRadius: 4.0,
    controlContainerRadius: 8.0,
    controlTinySize: 24.0,
    controlSmallSize: 32.0,
    controlMediumPaddingInline: 24.0,
    controlMediumSize: 40.0,
    controlLargeSize: 48.0,
    shadow2xs: [TokenShadow(x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color(argb: 0x6600_0000))],
    shadowXs: [TokenShadow(x: 0.0, y: 1.0, blur: 3.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowSm: [TokenShadow(x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color(argb: 0x7300_0000))],
    shadowMd: [TokenShadow(x: 0.0, y: 10.0, blur: 28.0, spread: 0.0, color: Color(argb: 0x8C00_0000))],
    shadowLg: [TokenShadow(x: 0.0, y: 14.0, blur: 36.0, spread: 0.0, color: Color(argb: 0x9400_0000))],
    shadowXl: [TokenShadow(x: 0.0, y: 16.0, blur: 40.0, spread: 0.0, color: Color(argb: 0x9900_0000))],
    shadow2xl: [TokenShadow(x: 0.0, y: 24.0, blur: 64.0, spread: 0.0, color: Color(argb: 0xB300_0000))],
    focusRingAlpha: 0.5,
    frameWindowRadius: 8.0,
    framePopoverRadius: 8.0,
    iconSmall: 14.0,
    iconMedium: 20.0,
    iconLarge: 24.0,
    radiusTiny: 4.0,
    radiusSmall: 9999.0,
    radiusMedium: 9999.0,
    radiusLarge: 8.0,
    radiusBig: 8.0,
    titleSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 20.0),
    titleMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.0, weight: .semibold, lineHeight: 24.0),
    titleLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 24.0, weight: .semibold, lineHeight: 32.0),
    bodySmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    bodyMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .regular, lineHeight: 20.0),
    bodyLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 16.0, weight: .regular, lineHeight: 24.0),
    labelQuiet: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .medium, lineHeight: 14.0),
    labelStrong: TypeStyle(family: ["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .bold, lineHeight: 14.0),
    labelSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .semibold, lineHeight: 11.0),
    labelMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 14.0),
    labelLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .semibold, lineHeight: 14.0),
    captionSmall: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 11.0, weight: .regular, lineHeight: 16.0),
    captionMedium: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 12.0, weight: .regular, lineHeight: 16.0),
    captionLarge: TypeStyle(family: ["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "Noto Sans CJK SC", "sans-serif"], size: 14.0, weight: .regular, lineHeight: 20.0),
    checkboxRadius: 4.0,
    progressGradientFrom: Colors.spotifyDark[500],
    progressGradientTo: Colors.spotifyDark[300],
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

  /// The `bright-dark` counterpart of ``brightLight``.
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
    colorSurfaceMuted: Color(argb: 0xFF14_1D29),
    colorSurfaceSunken: Color(argb: 0xFF1C_2734),
    colorSurfaceSubtle: Color(argb: 0x12F2_F4EF),
    colorSurfaceInset: Color(argb: 0xFF16_202C),
    colorSurfaceRaised: Color(argb: 0xFF2A_3644),
    colorSurfaceOverlay: Color(argb: 0xFF11_1A26),
    colorSurfaceChrome: Color(argb: 0xFF11_1A26),
    colorSurfaceColumn: Color(argb: 0xFF0A_111A),
    colorContent: Color(argb: 0xFFF2_F4EF),
    colorContentSecondary: Color(argb: 0xC7F2_F4EF),
    colorContentNav: Color(argb: 0xB3F2_F4EF),
    colorContentMuted: Color(argb: 0xA8F2_F4EF),
    colorContentSubtle: Color(argb: 0x73F2_F4EF),
    colorContentFaint: Color(argb: 0x61F2_F4EF),
    colorBorder: Color(argb: 0x14F2_F4EF),
    colorBorderStrong: Color(argb: 0x17F2_F4EF),
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
}
