import { DazzTheme, DazzTokens, DazzWindowFrame, DazzDialog, DazzButton, DazzDialogBody, DazzDialogFooter, DazzDialogClose } from '@dazzlabs/dazzui'
import { DazzToastProvider, DazzToastViewport, DazzToastContext } from '@dazzlabs/dazzui'
import { DazzPopover, DazzPreviewCard } from '@dazzlabs/dazzui'
import { DazzMenubar, DazzMenubarEntry, DazzMenu, DazzMenuItem, DazzSelect, DazzSelectOption } from '@dazzlabs/dazzui'
import { DazzSidebarProvider, DazzSidebar, DazzSidebarInset, DazzSidebarHeader, DazzSidebarMenuItem, DazzSidebarMenuButton, DazzSidebarMenuBadge, DazzSizing } from '@dazzlabs/dazzui'
import { window, font } from '@kit.ArkUI'
import { common } from '@kit.AbilityKit'
import { BusinessError } from '@kit.BasicServicesKit'
import { DazzWindowPlatform, DazzWindowControls } from '@dazzlabs/dazzui'
import { DazzTooltip } from '@dazzlabs/dazzui'
import { DazzIconButton } from '@dazzlabs/dazzui'
import { DazzSlider } from '@dazzlabs/dazzui'
import { DazzVariant, DazzSize } from '@dazzlabs/dazzui'
import { DazzTabs, DazzTabItem } from '@dazzlabs/dazzui'
import { DazzCalendar, DazzCalendarValue, DazzDateRange } from '@dazzlabs/dazzui'
import { DazzDateRangePicker, DazzDateRangeValue } from '@dazzlabs/dazzui'

/** Isolate real surfaces from gallery borders and adjacent shadows. */
@Entry({ useSharedStorage: true })
@Component
struct PaintVerification {
  @LocalStorageProp('themeName') themeName: string = 'bright-light'
  @LocalStorageProp('paintInactive') inactive: boolean = false
  @LocalStorageProp('paintOmarchy') omarchy: boolean = false
  @LocalStorageProp('paintWindowState') windowState: string = 'enabled'
  @LocalStorageProp('paintDialog') dialog: boolean = false
  @LocalStorageProp('dialogOverflow') overflow: boolean = false
  @LocalStorageProp('paintToast') toast: boolean = false
  @LocalStorageProp('paintToastLong') longToast: boolean = false
  @LocalStorageProp('paintSidebar') sidebar: string = ''
  @LocalStorageProp('paintPopup') popup: string = ''
  @LocalStorageProp('sizing') sizing: DazzSizing = 'design'
  @LocalBuilder specimen() {
    if (this.dialog) { DialogShadowSpecimen({ overflow: this.overflow }) }
    else if (this.popup === 'control') { ControlShadowSpecimen() }
    else if (this.popup === 'menubar') { MenubarSpecimen() }
    else if (this.popup === 'slider') { SliderSpecimen() }
    else if (this.popup === 'tabs-live') { TabsSpecimen() }
    else if (this.popup === 'tabs-actions') { TabsActionsSpecimen() }
    else if (this.popup === 'keyboard-actions') { KeyboardActionsSpecimen() }
    else if (this.popup === 'button-surfaces') { ButtonSurfacesSpecimen() }
    else if (this.popup === 'font-metrics') { FontMetricsSpecimen() }
    else if (this.popup === 'calendar-live') { CalendarSpecimen() }
    else if (this.popup === 'date-range-live') { DateRangeSpecimen() }
    else if (this.popup === 'tooltip-nesting') { TooltipNestingSpecimen() }
    else if (this.popup.startsWith('tooltip-multiline-')) { TooltipSpecimen({ side: this.popup.slice(18), label: 'Helpful label\nSecond line' }) }
    else if (this.popup.startsWith('tooltip-')) { TooltipSpecimen({ side: this.popup.slice(8) }) }
    else if (this.popup.startsWith('window-controls-')) { WindowControlSpecimen({
      platform: this.popup === 'window-controls-macos15' ? 'macos15' : 'macos', state: this.windowState }) }
    else if (this.popup === 'menu' || this.popup === 'select' || this.popup === 'context' || this.popup === 'menu-long' || this.popup === 'select-long') { MenuShadowSpecimen({ kind: this.popup }) }
    else if (this.popup) { PopupShadowSpecimen({ side: this.popup }) }
    else if (this.toast) { ToastShadowSpecimen({ long: this.longToast }) }
    else if (this.sidebar === 'labels') { SidebarLabelSpecimen() }
    else if (this.sidebar) { SidebarShadowSpecimen({ kind: this.sidebar }) }
    else { WindowShadowSpecimen({ inactive: this.inactive, omarchy: this.omarchy }) }
  }
  build() {
    Column() { DazzTheme({ name: this.themeName, sizing: this.sizing, content: this.specimen }) }
      .width('100%').height('100%').id('document-column')
  }
}

@Component
struct TooltipNestingSpecimen {
  @State actions: number = 0
  @LocalBuilder icon() {
    DazzIconButton({ label: 'Settings', icon: 'Settings20Regular', focusControlID: 'tooltip-nested-icon',
      onAction: () => { this.actions++ } })
  }
  build() {
    Column({ space: 32 }) {
      DazzButton({ label: 'Focus icon', onAction: () => {
        this.getUIContext().getFocusController().activate(true)
        this.getUIContext().getFocusController().requestFocus('tooltip-nested-icon')
      } }).id('tooltip-nested-focus')
      DazzTooltip({ label: 'Custom settings', side: Placement.Bottom, content: this.icon })
      Text('Actions: ' + String(this.actions)).id('tooltip-nested-actions')
    }.alignItems(HorizontalAlign.Start).margin({ left: 128, top: 96 })
  }
}

@Component
struct DateRangeSpecimen {
  @State start: number = new Date(2026, 9, 16, 14, 30).getTime()
  @State end: number = new Date(2026, 9, 16, 15, 30).getTime()
  @State allDay: boolean = false
  @State refuse: boolean = false
  @State commits: number = 0
  @State proposedStart: number = 0
  @State proposedEnd: number = 0
  @State proposedAllDay: boolean = false
  @State outside: number = 0
  state(): string {
    return JSON.stringify({ start: this.start, end: this.end, allDay: this.allDay, refuse: this.refuse,
      commits: this.commits, proposedStart: this.proposedStart, proposedEnd: this.proposedEnd,
      proposedAllDay: this.proposedAllDay, outside: this.outside })
  }
  value(): DazzDateRangeValue { return { start: new Date(this.start), end: new Date(this.end), allDay: this.allDay } }
  build() {
    Column({ space: 16 }) {
      Row({ space: 8 }) {
        DazzButton({ label: 'Refuse value', onAction: () => { this.refuse = !this.refuse } }).id('picker-live-refuse')
        DazzButton({ label: 'Replace value', onAction: () => {
          this.start = new Date(2026, 9, 20, 8).getTime(); this.end = new Date(2026, 9, 20, 9).getTime(); this.allDay = false
        } }).id('picker-live-replace')
      }
      Text(this.state()).id('picker-live-state')
      DazzButton({ label: 'Before', focusControlID: 'picker-live-before',
        onAction: () => { this.getUIContext().getFocusController().activate(true) } })
      Row({ space: 16 }) {
        DazzDateRangePicker({ value: this.value(), today: new Date(2026, 9, 16), locale: 'en-US', accessibilityLabel: 'When',
          onValueChange: (value: DazzDateRangeValue) => {
            this.commits++; this.proposedStart = value.start.getTime(); this.proposedEnd = value.end.getTime(); this.proposedAllDay = value.allDay
            if (!this.refuse) { this.start = value.start.getTime(); this.end = value.end.getTime(); this.allDay = value.allDay }
          } }).id('picker-live-trigger')
        DazzButton({ label: 'Outside action', focusControlID: 'picker-live-outside', onAction: () => { this.outside++ } })
      }
    }.alignItems(HorizontalAlign.Start).margin({ left: 64, top: 64 })
  }
}

@Component
struct CalendarSpecimen {
  @State month: number = new Date(2026, 9, 1).getTime()
  @State selected: number = new Date(2026, 9, 20).getTime()
  @State rangeStart: number = new Date(2026, 9, 20).getTime()
  @State rangeEnd: number = 0
  @State rangeMode: boolean = false
  @State twoMonths: boolean = false
  @State refuseMonth: boolean = false
  @State monthProposal: number = 0
  @State monthRequests: number = 0
  state(): string {
    return JSON.stringify({ month: this.month, selected: this.selected, start: this.rangeStart, end: this.rangeEnd,
      range: this.rangeMode, two: this.twoMonths, refuse: this.refuseMonth, proposal: this.monthProposal, requests: this.monthRequests })
  }
  value(): DazzCalendarValue {
    return this.rangeMode ? { start: new Date(this.rangeStart), end: this.rangeEnd ? new Date(this.rangeEnd) : null } : new Date(this.selected)
  }
  pick(value: DazzCalendarValue): void {
    if (this.rangeMode) {
      const range = value as DazzDateRange
      this.rangeStart = range.start?.getTime() ?? 0
      this.rangeEnd = range.end?.getTime() ?? 0
    } else this.selected = (value as Date).getTime()
  }
  build() {
    Column({ space: 16 }) {
      Row({ space: 8 }) {
        DazzButton({ label: 'Show June', onAction: () => { this.month = new Date(2027, 5, 1).getTime() } }).id('calendar-june')
        DazzButton({ label: 'Show October', onAction: () => { this.month = new Date(2026, 9, 1).getTime() } }).id('calendar-october')
        DazzButton({ label: 'Refuse month', onAction: () => { this.refuseMonth = !this.refuseMonth } }).id('calendar-refuse')
        DazzButton({ label: 'Two months', onAction: () => { this.twoMonths = !this.twoMonths } }).id('calendar-two')
        DazzButton({ label: 'Range', onAction: () => { this.rangeMode = !this.rangeMode } }).id('calendar-range')
      }
      Text(this.state()).id('calendar-live-state')
      DazzButton({ label: 'Before', focusControlID: 'calendar-before',
        onAction: () => { this.getUIContext().getFocusController().activate(true) } })
      DazzCalendar({ month: new Date(this.month), mode: this.rangeMode ? 'range' : 'single', months: this.twoMonths ? 2 : 1,
        value: this.value(), today: new Date(2026, 9, 16), min: new Date(2026, 9, 12), max: new Date(2026, 9, 23), locale: 'en-US',
        onValueChange: (value: DazzCalendarValue) => this.pick(value),
        onMonthChange: (month: Date) => { this.monthRequests++; this.monthProposal = month.getTime(); if (!this.refuseMonth) this.month = month.getTime() }
      }).id('calendar-live-control')
      DazzButton({ label: 'After', focusControlID: 'calendar-after',
        onAction: () => { this.getUIContext().getFocusController().activate(true) } })
    }.alignItems(HorizontalAlign.Start).margin({ left: 64, top: 64 })
  }
}

@Component
struct TabsSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State headerWidth: number = 560
  @State selected: string = 'alpha'
  @State busy: boolean = true
  @State renamed: boolean = false
  @State wideCount: boolean = false
  @State wideTail: boolean = false
  @State showGamma: boolean = true
  tabs(): DazzTabItem[] {
    const result: DazzTabItem[] = [
      { value: 'alpha', label: this.renamed ? 'Alpha with a longer name' : 'Alpha', icon: 'BranchRegular', count: 2, title: 'Alpha details' },
      { value: 'beta', label: 'Beta', icon: 'CircleRegular', count: this.wideCount ? 400000 : 4 },
      { value: 'gamma', label: 'Gamma', icon: 'PlayRegular', status: 'primary', busy: this.busy },
      { value: 'delta', label: 'Delta', icon: 'TagRegular', detail: 'Preview', count: 12, title: '' },
      { value: 'epsilon', label: 'Epsilon', icon: 'HistoryRegular', disabled: true, title: 'Unavailable tab' },
    ]
    return this.showGamma ? result : result.filter((item: DazzTabItem): boolean => item.value !== 'gamma')
  }
  @LocalBuilder trailing() {
    Button(this.wideTail ? 'Extra trailing action' : 'New', { type: ButtonType.Normal })
      .width(this.wideTail ? 180 : 24).height(24).padding(0).id('tabs-live-trailing')
  }
  build() {
    Column({ space: 16 }) {
      Row({ space: 8 }) {
        DazzButton({ label: 'Width 560', onAction: () => { this.headerWidth = 560 } }).id('tabs-width-560')
        DazzButton({ label: 'Width 340', onAction: () => { this.headerWidth = 340 } }).id('tabs-width-340')
        DazzButton({ label: 'Width 200', onAction: () => { this.headerWidth = 200 } }).id('tabs-width-200')
        DazzButton({ label: 'Rename', onAction: () => { this.renamed = !this.renamed } }).id('tabs-rename')
        DazzButton({ label: 'Count', onAction: () => { this.wideCount = !this.wideCount } }).id('tabs-count')
      }
      Row({ space: 8 }) {
        DazzButton({ label: 'Toggle busy', onAction: () => { this.busy = !this.busy } }).id('tabs-busy')
        DazzButton({ label: 'Toggle trailing', onAction: () => { this.wideTail = !this.wideTail } }).id('tabs-tail')
        DazzButton({ label: 'Toggle Gamma', onAction: () => { this.showGamma = !this.showGamma } }).id('tabs-gamma')
      }
      Text('Selected: ' + this.selected + '; Busy: ' + String(this.busy)).id('tabs-live-state')
      Row() {
        DazzTabs({ items: this.tabs(), value: this.selected, contentHeight: 36, trailing: this.trailing,
          onValueChange: (value: string) => { this.selected = value } }).id('tabs-live-control')
      }.width(this.headerWidth).height(36).backgroundColor(this.tokens.color('surface-chrome')).id('tabs-live-header')
    }.alignItems(HorizontalAlign.Start).margin({ left: 64, top: 64 })
  }
}

@Component
struct TabsActionsSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State values: string[] = ['a', 'b', 'c', 'd']
  @State selected: string = 'a'
  @State headerWidth: number = 700
  @State closed: number = 0
  @State created: number = 0
  @State refuse: boolean = false
  @State closable: boolean = true
  items(): DazzTabItem[] {
    return this.values.map((value: string): DazzTabItem => ({ value,
      label: 'Document ' + String(value.charCodeAt(0) - 96), icon: 'Document20Regular', closable: this.closable }))
  }
  @LocalBuilder trailing() {
    DazzButton({ label: 'New', controlSize: 'tiny', onAction: () => { this.created++ } }).id('tabs-actions-new')
  }
  build() {
    Column({ space: 24 }) {
      Row({ space: 12 }) {
        DazzButton({ label: 'Reset', onAction: () => {
          this.values = ['a', 'b', 'c', 'd']; this.selected = 'a'; this.headerWidth = 700; this.closed = 0; this.created = 0; this.refuse = false; this.closable = true
        } }).id('tabs-actions-reset')
        DazzButton({ label: 'Narrow', onAction: () => { this.headerWidth = 260 } }).id('tabs-actions-narrow')
        DazzButton({ label: 'Refuse close', onAction: () => { this.refuse = !this.refuse } }).id('tabs-actions-refuse')
        DazzButton({ label: 'Toggle close', onAction: () => { this.closable = !this.closable; this.headerWidth = 200 } }).id('tabs-actions-toggle-close')
      }
      Row() {
        DazzTabs({ items: this.items(), value: this.selected, contentHeight: 36, trailing: this.trailing,
          onValueChange: (value: string) => { this.selected = value },
          onClose: (value: string) => { if (!this.refuse) this.values = this.values.filter((item: string): boolean => item !== value); this.closed++ } })
          .id('tabs-actions-control')
      }.width(this.headerWidth).height(36).backgroundColor(this.tokens.color('surface-chrome')).id('tabs-actions-header')
      Text(JSON.stringify({ values: this.values, selected: this.selected, closed: this.closed, created: this.created, width: this.headerWidth, refuse: this.refuse, closable: this.closable }))
        .id('tabs-actions-state')
    }.alignItems(HorizontalAlign.Start).margin({ left: 64, top: 64 })
  }
}

@Component
struct KeyboardActionsSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State button: number = 0
  @State icon: number = 0
  @State closed: number = 0
  build() {
    Column({ space: 24 }) {
      Row({ space: 12 }) {
        DazzButton({ label: 'Before', focusControlID: 'keyboard-before' })
        DazzButton({ label: 'Action', focusControlID: 'keyboard-button', onAction: () => { this.button++ } })
        DazzIconButton({ label: 'Settings', icon: 'Settings20Regular', focusControlID: 'keyboard-icon', onAction: () => { this.icon++ } })
        DazzButton({ label: 'After', focusControlID: 'keyboard-after' })
      }
      Row() {
        DazzTabs({ items: [
          { value: 'a', label: 'Document 1', icon: 'Document20Regular', closable: true },
          { value: 'b', label: 'Document 2', icon: 'Document20Regular', closable: true },
          { value: 'c', label: 'Document 3', icon: 'Document20Regular', closable: true },
          { value: 'd', label: 'Document 4', icon: 'Document20Regular', closable: true },
        ], value: 'a', onClose: () => { this.closed++ } }).id('keyboard-tabs')
      }.width(200).height(36).backgroundColor(this.tokens.color('surface-chrome'))
      Text(JSON.stringify({ button: this.button, icon: this.icon, closed: this.closed })).id('keyboard-actions-state')
    }.alignItems(HorizontalAlign.Start).margin({ left: 64, top: 64 })
  }
}

@Component
struct ButtonSurfacesSpecimen {
  @State disabled: boolean = false
  private variants: DazzVariant[] = ['normal', 'recessed', 'filled', 'tinted', 'outlined', 'plain']
  private sizes: DazzSize[] = ['tiny', 'small', 'medium', 'large']
  build() {
    Column({ space: 24 }) {
      DazzButton({ label: 'Disable controls', focusControlID: 'surface-disable', onAction: () => { this.disabled = !this.disabled } })
      ForEach(this.sizes, (controlSize: DazzSize) => {
        Row({ space: 16 }) {
          ForEach(this.variants, (variant: DazzVariant) => {
            DazzButton({ label: variant, variant, controlSize, disabled: this.disabled,
              focusControlID: 'surface-' + controlSize + '-' + variant })
          })
        }
      })
    }.alignItems(HorizontalAlign.Start).margin({ left: 64, top: 64 })
  }
}

@Component
struct FontMetricsSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State report: string = ''
  private timer: number = -1
  samples: FontMetricSample[] = [
    { text: 'Gamma', role: 'labelMedium', weight: 500, tabular: false },
    { text: 'Gamma', role: 'labelMedium', weight: 700, tabular: false },
    { text: 'Alpha', role: 'labelMedium', weight: 500, tabular: false },
    { text: 'Alpha', role: 'labelMedium', weight: 700, tabular: false },
    { text: '2', role: 'captionSmall', weight: 500, tabular: false },
    { text: '2', role: 'captionSmall', weight: 500, tabular: true },
    { text: '12', role: 'captionSmall', weight: 500, tabular: false },
    { text: '12', role: 'captionSmall', weight: 500, tabular: true },
    { text: '400000', role: 'captionSmall', weight: 500, tabular: false },
    { text: '400000', role: 'captionSmall', weight: 500, tabular: true },
    { text: '国', role: 'labelMedium', weight: 500, tabular: false },
  ]
  aboutToAppear(): void { this.timer = setTimeout(() => this.capture(), 500) }
  aboutToDisappear(): void { clearTimeout(this.timer) }
  capture(): void {
    const ui = this.getUIContext()
    ui.runScopedTask(() => {
      const config = font.getUIFontConfig()
      const uiFont = ui.getFont()
      const names = uiFont.getSystemFontList().filter((name: string): boolean => name.toLowerCase().indexOf('harmony') >= 0).slice(0, 12)
      const faces = names.map((name: string) => uiFont.getFontByName(name))
      const measurements = this.samples.map((sample: FontMetricSample, index: number): FontMetricResult => {
        const size = this.tokens.type(sample.role).size
        const measured = ui.getMeasureUtils().measureTextSize({ textContent: sample.text, fontSize: this.tokens.fontSize(size), fontWeight: sample.weight })
        const actual = ui.getComponentUtils().getRectangleById('font-metric-' + String(index))
        return { text: sample.text, size, weight: sample.weight, tabular: sample.tabular,
          measured: ui.px2vp(Number(measured.width)), actual: ui.px2vp(actual.size.width) }
      })
      const context = ui.getHostContext() as common.UIAbilityContext
      const systemFamily = context.resourceManager.getStringSync($r('sys.string.ohos_id_text_font_family_regular').id)
      this.report = JSON.stringify({ density: ui.vp2px(1), systemFamily, fontId: context.config.fontId ?? '',
        fontScale: context.config.fontSizeScale ?? 1, fontWeightScale: context.config.fontWeightScale ?? 1,
        config, names, faces, measurements })
    })
  }
  build() {
    Column({ space: 12 }) {
      ForEach(this.samples, (sample: FontMetricSample, index: number) => {
        Row({ space: 24 }) {
          Text(sample.role + ' / ' + String(sample.weight) + (sample.tabular ? ' / tnum' : ' / normal')).width(240)
          Text(sample.text).id('font-metric-' + String(index))
            .fontSize(this.tokens.fontSize(this.tokens.type(sample.role).size)).fontWeight(sample.weight)
            .fontFeature(sample.tabular ? '"tnum" on' : 'normal').maxLines(1)
        }
      }, (_sample: FontMetricSample, index: number): string => String(index))
      Text(this.report).id('font-metrics-report').fontSize(10).maxLines(5)
    }.alignItems(HorizontalAlign.Start).margin(64)
  }
}

interface FontMetricSample { text: string; role: string; weight: number; tabular: boolean }
interface FontMetricResult { text: string; size: number; weight: number; tabular: boolean; measured: number; actual: number }

@Component
struct SliderSpecimen {
  build() {
    Column({ space: 36 }) {
      DazzSlider({ defaultValue: 40, controlSize: 'small', accessibilityLabel: 'Small' }).id('slider-paint-small')
      DazzSlider({ defaultValue: 40, accessibilityLabel: 'Medium' }).id('slider-paint-medium')
      DazzSlider({ defaultValue: 40, controlSize: 'large', accessibilityLabel: 'Large' }).id('slider-paint-large')
      DazzSlider({ defaultValue: 0, accessibilityLabel: 'Minimum' }).id('slider-paint-min')
      DazzSlider({ defaultValue: 100, accessibilityLabel: 'Maximum' }).id('slider-paint-max')
      DazzSlider({ defaultValue: [20, 80], accessibilityLabel: 'Range' }).id('slider-paint-range')
      DazzSlider({ defaultValue: 40, disabled: true, accessibilityLabel: 'Disabled' }).id('slider-paint-disabled')
    }.width(320).margin({ left: 64, top: 64 }).id('slider-paint-stage')
  }
}

@Component
struct TooltipSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop side: string = 'top'
  @Prop label: string = 'Helpful label'
  @State actions: number = 0
  @State underActions: number = 0
  @State disabled: boolean = false
  placement(): Placement { return this.side === 'bottom' ? Placement.Bottom : this.side === 'left' ? Placement.Left : this.side === 'right' ? Placement.Right : Placement.Top }
  @LocalBuilder trigger() {
    Button({ type: ButtonType.Normal }) {}.width(28).height(28).padding(0).borderRadius(0)
      .backgroundColor(this.tokens.color('surface')).hoverEffect(HoverEffect.None).stateEffect(false)
      .focusOnTouch(false).id('tooltip-native-target').accessibilityText('Tooltip trigger')
      .onClick(() => { this.actions++ })
  }
  build() {
    Column() {
      Row({ space: 12 }) {
        Text('Trigger: ' + String(this.actions) + '; Under: ' + String(this.underActions)).id('tooltip-actions').fontColor(this.tokens.color('content'))
        DazzButton({ label: 'Focus target', onAction: () => {
          this.getUIContext().getFocusController().activate(true)
          this.getUIContext().getFocusController().requestFocus('tooltip-native-target')
        } }).id('tooltip-focus')
        DazzButton({ label: 'Disable tooltip', onAction: () => { this.disabled = !this.disabled } }).id('tooltip-disable')
      }
      Stack() {
        Button({ type: ButtonType.Normal }) {}.width('100%').height('100%').padding(0).borderRadius(0)
          .backgroundColor(this.tokens.color('surface')).hoverEffect(HoverEffect.None).stateEffect(false)
          .focusable(false).focusOnTouch(false).accessibilityText('Under the tooltip').id('tooltip-under')
          .onClick(() => { this.underActions++ })
        DazzTooltip({ label: this.label, side: this.placement(), disabled: this.disabled, content: this.trigger })
      }.width(640).height(480).id('tooltip-paint-stage')
    }.width('100%').height('100%').justifyContent(FlexAlign.Center).backgroundColor(this.tokens.color('surface'))
  }
}

@Component
struct WindowControlSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop platform: DazzWindowPlatform = 'macos'
  @Prop state: string = 'enabled'
  @State closeCount: number = 0
  @State minimizeCount: number = 0
  @State maximizeCount: number = 0
  controls(): DazzWindowControls {
    const value = this.state === 'hidden' ? 'hidden' : this.state === 'disabled' ? 'disabled' : 'enabled'
    return { close: value, minimize: value, maximize: value }
  }
  @LocalBuilder body() {
    Text('Window controls').fontColor(this.tokens.color('content')).padding(20)
  }
  build() {
    Column() {
      Text('Close: ' + String(this.closeCount) + '; Minimize: ' + String(this.minimizeCount) + '; Maximize: ' + String(this.maximizeCount))
        .fontColor(this.tokens.color('content')).id('window-control-count')
      Stack() {
        DazzWindowFrame({ platform: this.platform, title: 'Window chrome', contentWidth: 300, contentHeight: 120,
          controls: this.controls(), inactive: this.state === 'inactive', content: this.body,
          onClose: () => { this.closeCount++ }, onMinimize: () => { this.minimizeCount++ }, onMaximize: () => { this.maximizeCount++ } })
          .id('paint-window-controls')
      }.width(640).height(480).id('window-control-stage')
    }.width('100%').height('100%').justifyContent(FlexAlign.Center)
      .backgroundColor(this.tokens.color('surface'))
  }
}

@Component
struct MenuShadowSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop kind: string = 'menu'
  @State actions: number = 0
  @State value: string = 'first'
  @State resizeStatus: string = 'idle'
  private resizeTimer: number = -1
  private alive: boolean = false
  aboutToAppear(): void { this.alive = true }
  aboutToDisappear(): void { this.alive = false; clearTimeout(this.resizeTimer) }
  armResize(height: number): void {
    clearTimeout(this.resizeTimer)
    this.resizeStatus = 'pending'
    const ui = this.getUIContext()
    // Capture the owning window before opening the menu. A delayed public
    // resize changes its real constraints without an outside dismissal press.
    window.getLastWindow(ui.getHostContext() as common.UIAbilityContext).then((host: window.Window) => {
      if (!this.alive) return
      this.resizeTimer = setTimeout(() => {
        host.resize(Math.round(ui.vp2px(1100)), Math.round(ui.vp2px(height))).then(() => {
          if (this.alive) this.resizeStatus = String(height)
        }).catch((error: BusinessError) => {
          if (this.alive) this.resizeStatus = 'error-' + String(error.code)
        })
      }, 6000)
    }).catch((error: BusinessError) => {
      if (this.alive) this.resizeStatus = 'error-' + String(error.code)
    })
  }
  options(): DazzSelectOption[] {
    if (this.kind.endsWith('-long')) {
      const choices: DazzSelectOption[] = []
      for (let index: number = 1; index <= 60; index++) choices.push({ value: 'choice-' + String(index), label: 'Documentation choice ' + String(index) })
      return choices
    }
    return [ { value: 'first', label: 'Documentation and release notes' },
      { value: 'second', label: 'Keyboard shortcuts' } ]
  }
  items(): DazzMenuItem[] {
    if (this.kind.endsWith('-long')) return this.options().map((option: DazzSelectOption): DazzMenuItem =>
      ({ label: option.label, onSelect: () => { this.actions++ } }))
    return [ { label: 'Rename', onSelect: () => { this.actions++ } },
      { label: 'Duplicate', onSelect: () => { this.actions++ } },
      { label: 'Delete', disabled: true } ]
  }
  @LocalBuilder trigger() { PopupPaintTrigger() }
  build() {
    Column() {
      Row() {
        Text('Menu actions: ' + String(this.actions)).fontColor(this.tokens.color('content'))
          .id('paint-menu-count')
        if (this.kind.endsWith('-long')) {
          Button('Resize smaller').id('menu-resize-small').onClick(() => this.armResize(600))
          Button('Resize larger').id('menu-resize-large').onClick(() => this.armResize(820))
          Text(this.resizeStatus).fontColor(this.tokens.color('content')).id('menu-resize-status')
        }
      }.width('100%').justifyContent(FlexAlign.Start)
      Stack() {
        if (this.kind.startsWith('select')) {
          // A narrow real anchor leaves the popup's central top shadow clear
          // of the open field and its focus outline.
          DazzSelect({ options: this.options(), value: this.value,
            onValueChange: (value: string) => { this.value = value; this.actions++ } })
            .width(64).id('paint-menu-trigger')
        } else {
          DazzMenu({ contextMenu: this.kind === 'context', trigger: this.trigger, items: this.items() }).id('paint-menu-trigger')
        }
      }.width(640).height(640).id('menu-paint-stage')
    }.width('100%').height('100%').alignItems(HorizontalAlign.Center)
      .backgroundColor(this.tokens.color('surface'))
  }
}

@Component
struct MenubarSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State actions: number = 0
  @State outside: number = 0
  @State modal: boolean = true
  menus(): DazzMenubarEntry[] {
    return [
      { label: 'File', items: [
        { label: 'Rename', onSelect: () => { this.actions++ } },
        { label: 'Duplicate', onSelect: () => { this.actions++ } },
        { label: 'Delete', disabled: true },
      ] },
      { label: 'Disabled', disabled: true, items: [{ label: 'Unavailable' }] },
      { label: 'View', items: [
        { label: 'Sidebar', checked: true }, { label: 'Theme panel', checked: false }, { label: 'Full screen' },
      ] },
      { label: 'Help', items: [{ label: 'Documentation', onSelect: () => { this.actions++ } }] },
    ]
  }
  build() {
    Column({ space: this.tokens.metric('spacing.4') }) {
      Button('Before bar').id('bar-before').focusOnTouch(true)
        .onClick(() => this.getUIContext().getFocusController().activate(true))
      DazzMenubar({ menus: this.menus(), modal: this.modal }).id('bar-primary')
      Button('After bar').id('bar-after').onClick(() => { this.outside++ })
      DazzMenubar({ menus: [
        { label: 'Window', items: [{ label: 'Arrange', onSelect: () => { this.actions++ } }] },
        { label: 'Tools', items: [{ label: 'Inspect', onSelect: () => { this.actions++ } }] },
      ], modal: this.modal }).id('bar-secondary')
      Button('Toggle modality').id('bar-toggle-modal').onClick(() => { this.modal = !this.modal })
      Text('Actions: ' + String(this.actions) + '; outside: ' + String(this.outside))
        .fontColor(this.tokens.color('content')).id('bar-count')
      Text(this.modal ? 'Modal' : 'Nonmodal').fontColor(this.tokens.color('content')).id('bar-modal')
    }.width('100%').height('100%').padding(this.tokens.metric('spacing.12'))
      .alignItems(HorizontalAlign.Start).backgroundColor(this.tokens.color('surface'))
  }
}

@Component
struct ControlShadowSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State actions: number = 0
  @State underlay: number = 0
  aboutToAppear(): void { this.getUIContext().getFocusController().activate(false) }
  build() {
    Column() {
      Row() {
        Text('Button actions: ' + String(this.actions) + '; underlay: ' + String(this.underlay))
          .fontColor(this.tokens.color('content')).id('paint-control-count')
      }.width('100%').justifyContent(FlexAlign.Start)
      Stack() {
        Button({ type: ButtonType.Normal }) {}.width('100%').height('100%')
          .padding(0).borderRadius(0).backgroundColor(this.tokens.color('surface'))
          .stateEffect(false).hoverEffect(HoverEffect.None).focusOnTouch(false)
          .onClick(() => { this.underlay++ }).id('paint-control-underlay')
        DazzButton({ label: 'Filled control', variant: 'filled', controlSize: 'small',
          onAction: () => { this.actions++ } })
          .id('paint-direct-control')
      }.width(640).height(640).id('control-paint-stage')
    }.width('100%').height('100%').justifyContent(FlexAlign.Center)
      .alignItems(HorizontalAlign.Center).backgroundColor(this.tokens.color('surface'))
  }
}

interface PopupPaintActivation { activate: () => void }

@Component
struct PopupPaintTrigger {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.activation') activation: PopupPaintActivation = { activate: () => {} }
  build() {
    // A real, flat native trigger leaves a uniform backdrop under the blur.
    Button({ type: ButtonType.Normal }) {}.width(28).height(28)
      .padding(0).borderRadius(0).backgroundColor(this.tokens.color('surface'))
      .hoverEffect(HoverEffect.None).stateEffect(false).focusOnTouch(false)
      .accessibilityText('Open paint popup').id('paint-popup-trigger')
      .onClick(() => this.activation.activate())
  }
}

@Component
struct PopupShadowSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop side: string = 'bottom'
  @State actions: number = 0
  @State previewOpen: boolean = false
  placement(): Placement {
    return this.side === 'top' ? Placement.Top : this.side === 'left' ? Placement.Left
      : this.side === 'right' ? Placement.Right : Placement.Bottom
  }
  @LocalBuilder trigger() { PopupPaintTrigger() }
  @LocalBuilder body() {
    Column() {
      Blank().layoutWeight(1)
      DazzButton({ label: 'Popup action', onAction: () => { this.actions++ } }).id('paint-popup-action')
      Blank().layoutWeight(1)
    }.width('100%').height(200)
  }
  build() {
    Column() {
      Row() {
        Text('Popup actions: ' + String(this.actions)).fontColor(this.tokens.color('content'))
          .id('paint-popup-count')
        if (this.side === 'preview') {
          DazzButton({ label: 'Open paint preview', onAction: () => { this.previewOpen = true } })
            .id('paint-preview-open')
        }
      }.width('100%').justifyContent(FlexAlign.Start)
      Stack() {
        if (this.side === 'preview') {
          DazzPreviewCard({ open: this.previewOpen, contentWidth: 300, trigger: this.trigger,
            content: this.body, onOpenChange: (open: boolean) => { this.previewOpen = open } })
        } else {
          DazzPopover({ side: this.placement(), contentWidth: 300, trigger: this.trigger, content: this.body })
        }
      }.width(640).height(640).id('popup-paint-stage')
    }.width('100%').height('100%').alignItems(HorizontalAlign.Center)
      .backgroundColor(this.tokens.color('surface'))
  }
}

@Observed
class SidebarLabelModel {
  longLabel: boolean = true
  actions: number = 0
}

@Component
struct SidebarLabelParts {
  @Consume('dazzui.sidebar.labels') model: SidebarLabelModel
  @Prop controlSize: string = 'medium'
  build() {
    Stack({ alignContent: Alignment.Start }) {
      DazzSidebarMenuButton({ label: this.model.longLabel ? 'Documents and shared workspace resources' : 'Docs',
        icon: 'Document20Regular', controlSize: this.controlSize,
        onAction: () => { this.model.actions++ } }).id('sidebar-label-button-' + this.controlSize)
      DazzSidebarMenuBadge({ label: '12', badgeID: 'sidebar-label-badge-' + this.controlSize })
    }.width('100%').height(LayoutPolicy.wrapContent)
  }
}

@Component
struct SidebarLabelRow {
  @Prop controlSize: string = 'medium'
  @LocalBuilder parts() { SidebarLabelParts({ controlSize: this.controlSize }) }
  build() { DazzSidebarMenuItem({ active: true, hasBadge: true, content: this.parts }).width('100%') }
}

@Component
struct SidebarLabelSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide('dazzui.sidebar.labels') model: SidebarLabelModel = new SidebarLabelModel()
  @State open: boolean = true
  @LocalBuilder rows() {
    Column({ space: this.tokens.metric('spacing.2') }) {
      SidebarLabelRow({ controlSize: 'small' })
      SidebarLabelRow({ controlSize: 'medium' })
      SidebarLabelRow({ controlSize: 'large' })
    }.width('100%')
  }
  @LocalBuilder sidebarBody() { DazzSidebarHeader({ content: this.rows }) }
  @LocalBuilder empty() {}
  @LocalBuilder columns() {
    DazzSidebar({ variant: 'floating', collapsible: 'icon', content: this.sidebarBody })
    DazzSidebarInset({ content: this.empty })
  }
  build() {
    Column({ space: this.tokens.metric('spacing.3') }) {
      DazzButton({ label: 'Toggle label length', onAction: () => { this.model.longLabel = !this.model.longLabel } })
        .id('sidebar-label-length')
      DazzButton({ label: 'Toggle label sidebar', onAction: () => { this.open = !this.open } })
        .id('sidebar-label-toggle')
      Text('Label actions: ' + String(this.model.actions)).id('sidebar-label-actions')
      Text(this.model.longLabel ? 'Long labels' : 'Short labels').id('sidebar-label-mode')
      DazzSidebarProvider({ open: this.open, content: this.columns,
        onOpenChange: (open: boolean) => { this.open = open } })
        .width(this.tokens.metric('frame.sidebar.width') + 300).height(300)
        .id('sidebar-label-provider')
    }.width('100%').height('100%').alignItems(HorizontalAlign.Start)
      .backgroundColor(this.tokens.color('surface'))
  }
}

@Component
struct SidebarShadowSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop kind: string = 'floating'
  @State open: boolean = true
  @State actions: number = 0
  @State underlay: number = 0
  aboutToAppear(): void { this.open = this.kind === 'floating' }
  ground(): string { return this.kind === 'inset' ? 'surface-column' : 'surface' }
  @LocalBuilder menuButton() {
    DazzSidebarMenuButton({ label: 'Paint navigation', icon: 'Document20Regular',
      onAction: () => { this.actions++ } }).id('sidebar-paint-action')
  }
  @LocalBuilder menuItem() { DazzSidebarMenuItem({ content: this.menuButton }) }
  @LocalBuilder sideContent() { DazzSidebarHeader({ content: this.menuItem }) }
  @LocalBuilder empty() {}
  @LocalBuilder pane() {
    Column() {
      Blank().layoutWeight(1)
      DazzButton({ label: 'Paint pane action', onAction: () => { this.actions++ } }).id('sidebar-paint-pane-action')
      Blank().layoutWeight(1)
    }.width('100%').height('100%')
  }
  @LocalBuilder columns() {
    if (this.kind === 'floating') {
      DazzSidebar({ variant: 'floating', collapsible: 'icon', content: this.sideContent })
      DazzSidebarInset({ content: this.empty })
    } else {
      DazzSidebar({ variant: 'inset', collapsible: 'offcanvas', content: this.empty })
      DazzSidebarInset({ content: this.pane })
    }
  }
  build() {
    Column() {
      if (this.kind === 'floating') {
        DazzButton({ label: 'Toggle paint sidebar', onAction: () => { this.open = !this.open } }).id('sidebar-paint-toggle')
      }
      Text('Actions: ' + String(this.actions) + '; underlay: ' + String(this.underlay))
        .fontColor(this.tokens.color('content')).id('sidebar-paint-counts')
      Stack() {
        Button({ type: ButtonType.Normal }) {}.width('100%').height('100%')
          .padding(0).borderRadius(0).backgroundColor(this.tokens.color(this.ground()))
          .stateEffect(false).hoverEffect(HoverEffect.None).focusOnTouch(false)
          .id('sidebar-paint-underlay').accessibilityText('Underlying canvas button')
          .onClick(() => { this.underlay++ })
        DazzSidebarProvider({ open: this.open, content: this.columns,
          onOpenChange: (open: boolean) => { this.open = open } })
          .width(this.kind === 'floating' ? this.tokens.metric('frame.sidebar.width') + 300 : 300).height(300)
          .id('sidebar-paint-provider')
      }.width(640).height(640).id('sidebar-paint-stage')
    }.width('100%').height('100%').justifyContent(FlexAlign.Center)
      .backgroundColor(this.tokens.color(this.ground()))
  }
}

@Component
struct ToastPaintControls {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.toast') toast: DazzToastContext
  @Prop long: boolean = false
  @State actions: number = 0
  show(): void {
    this.toast.add({ id: 'paint-toast', title: this.long
      ? 'Your changes were saved successfully and synchronized across all of your connected devices. This receipt must truncate before its actions.' : 'Saved',
      timeout: 0, icon: this.long ? 'Document20Regular' : undefined,
      action: this.long ? { label: 'Undo', onAction: () => { this.actions++ } } : undefined })
  }
  build() {
    Column() {
      DazzButton({ label: 'Show paint toast', onAction: () => this.show() }).id('paint-toast-trigger')
      Text('Actions: ' + String(this.actions)).fontColor(this.tokens.color('content')).id('paint-toast-actions')
      Row() {}.width(640).height(640).id('toast-paint-stage')
      DazzToastViewport({ viewportId: 'paint-toast-viewport' })
    }.width('100%').height('100%').alignItems(HorizontalAlign.Start)
      .backgroundColor(this.tokens.color('surface'))
  }
}

@Component
struct ToastShadowSpecimen {
  @Prop long: boolean = false
  @LocalBuilder controls() { ToastPaintControls({ long: this.long }) }
  build() { DazzToastProvider({ content: this.controls }).width('100%').height('100%') }
}

@Component
struct DialogShadowSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop overflow: boolean = false
  @LocalBuilder trigger() { DazzButton({ label: 'Open paint dialog' }).id('paint-dialog-trigger') }
  @LocalBuilder rows() {
    ForEach(Array.from({ length: 40 }, (_value: Object, index: number): number => index), (index: number) => {
      Text('Dialog row ' + String(index + 1)).height(32).fontColor(this.tokens.color('content')).id('dialog-row-' + String(index))
    })
    DazzButton({ label: 'End of dialog' }).id('dialog-end')
  }
  @LocalBuilder body() {
    if (this.overflow) { DazzDialogBody({ content: this.rows }).id('dialog-scroll-body') }
    else { Column() {}.width('100%').height(300) }
  }
  @LocalBuilder close() { DazzDialogClose({ label: 'Close overflow dialog' }) }
  @LocalBuilder footer() {
    if (this.overflow) { DazzDialogFooter({ content: this.close }).id('dialog-pinned-footer').flexShrink(0) }
  }
  build() {
    Column() {
      DazzDialog({ title: this.overflow ? 'Overflow verification' : '',
        accessibilityLabel: 'Isolated paint dialog', contentWidth: this.overflow ? 440 : 300,
        trigger: this.trigger, content: this.body, footer: this.footer })
      Row() {}.width(640).height(640).id('dialog-paint-stage')
    }.width('100%').height('100%').justifyContent(FlexAlign.Start)
      .backgroundColor(this.tokens.color('surface'))
  }
}

@Component
struct WindowShadowSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop inactive: boolean = false
  @Prop omarchy: boolean = false
  @LocalBuilder empty() { Column() {} }
  build() {
    Column() {
      Text('Window shadow paint verification').fontColor(this.tokens.color('content'))
      Stack() {
        DazzWindowFrame({ platform: this.omarchy ? 'omarchy' : 'macos', title: 'Isolated window',
          showTitlebar: false, controls: false, inactive: this.inactive,
          contentWidth: 300, contentHeight: 300, content: this.empty })
          .id('paint-window')
      }.width(640).height(640).id('paint-stage')
        .backgroundColor(this.tokens.color('surface'))
    }.width('100%').height('100%').justifyContent(FlexAlign.Center)
      .backgroundColor(this.tokens.color('surface'))
  }
}
