import { DazzTheme, DazzTokens, DazzDrawer, DazzDrawerClose, DazzButton, DazzDialog, DazzDialogBody, DazzShortcutRecorder } from '@dazzlabs/dazzui'
import { DazzOtpField, DazzCodeBlock, DazzForm } from '@dazzlabs/dazzui'
import { DazzTree, DazzTreeNode } from '@dazzlabs/dazzui'
import { DazzSidebarProvider, DazzSidebar, DazzSidebarInset, DazzSidebarHeader, DazzSidebarTrigger } from '@dazzlabs/dazzui'
import { DazzPreviewCard, DazzTextField, DazzEditingContext, DazzLink } from '@dazzlabs/dazzui'
import { DazzSlider, DazzSliderValue } from '@dazzlabs/dazzui'
import { DazzToastProvider, DazzToastViewport, DazzToastContext, DazzSearchField } from '@dazzlabs/dazzui'

@Component
struct ToastControls {
  @Consume('dazzui.toast') toast: DazzToastContext
  @State sequence: number = 0
  @State value: string = ''
  @State actions: number = 0
  build() {
    Column({ space: 16 }) {
      DazzButton({ label: 'Add persistent notice', onAction: () => {
        this.sequence++
        this.toast.add({ title: 'Persistent ' + String(this.sequence), timeout: 0,
          action: { label: 'Count action', onAction: () => { this.actions++ } } })
      } }).id('toast-probe-add')
      DazzTextField({ value: this.value, accessibilityLabel: 'Toast source editor',
        onValueChange: (value: string) => { this.value = value } }).id('toast-probe-editor')
      DazzSearchField({ placeholder: 'Toast source search' }).id('toast-probe-search')
      Button('Host native button').id('toast-probe-native').focusOnTouch(true)
      TextInput({ placeholder: 'Host native editor' }).id('toast-probe-native-editor')
      Text('Toast actions: ' + String(this.actions)).id('toast-probe-actions')
      DazzToastViewport({ viewportId: 'toast-probe-viewport' })
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
}

@Component
struct ToastSpecimens {
  @LocalBuilder controls() { ToastControls() }
  build() { DazzToastProvider({ content: this.controls }) }
}

@Component
struct ToastUnderlayControls {
  @Consume('dazzui.toast') toast: DazzToastContext
  @State count: number = 0
  build() {
    Column({ space: 16 }) {
      DazzButton({ label: 'Show notification over control', onAction: () => {
        this.toast.add({ title: 'Keep the covered control untouched', timeout: 0 })
      } }).id('toast-underlay-show')
      Text('Underlay actions: ' + String(this.count)).id('toast-underlay-count')
      Blank().layoutWeight(1)
      Button('Underlay action').width('100%').height(120).id('toast-underlay-action')
        .onClick(() => { this.count++ })
      DazzToastViewport({ viewportId: 'toast-underlay-viewport' })
    }.width('100%').height('100%').alignItems(HorizontalAlign.Start)
  }
}

@Component
struct ToastUnderlaySpecimens {
  @LocalBuilder controls() { ToastUnderlayControls() }
  build() { DazzToastProvider({ content: this.controls }) }
}

@Component
struct SliderSpecimens {
  @State value: number[] = [20, 80]
  @State accepted: number = 0
  @State refused: number = 0
  @State lastRefused: string = ''
  build() {
    Column({ space: 24 }) {
      DazzSlider({ value: this.value, showValue: true, accessibilityLabel: 'Accepted range', onValueChange: (value: DazzSliderValue) => {
        if (Array.isArray(value)) this.value = value
        this.accepted++
      } }).id('slider-accepted')
      Text('Range: ' + this.value.join(',')).id('slider-accepted-values')
      Text('Range changes: ' + String(this.accepted)).id('slider-accepted-count')
      DazzSlider({ value: [20, 80], showValue: true, accessibilityLabel: 'Refused range', onValueChange: (value: DazzSliderValue) => { this.refused++; this.lastRefused = Array.isArray(value) ? value.join(',') : String(value) } }).id('slider-refused')
      Text('Range refused: ' + String(this.refused)).id('slider-refused-count')
      Text('Refused proposal: ' + this.lastRefused).id('slider-refused-proposal')
      DazzSlider({ defaultValue: [20, 80], disabled: true, showValue: true, accessibilityLabel: 'Disabled range', onValueChange: () => { this.refused++ } }).id('slider-disabled')
      DazzButton({ label: 'Reset range', onAction: () => { this.value = [20, 80]; this.accepted = 0; this.refused = 0 } })
      DazzButton({ label: 'After range' })
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
}

@Component
struct PreviewEditableBody {
  @Consume('dazzui.editing') editing: DazzEditingContext
  @State count: number = 0
  @State value: string = ''
  @State composing: boolean = false
  @State visits: number = 0
  build() {
    Column({ space: 12 }) {
      Text('Interactive preview body')
      DazzButton({ label: 'Increment preview', onAction: () => { this.count++ } })
      Text('Preview count: ' + String(this.count)).id('preview-count')
      DazzLink({ label: 'Preview destination', href: '#/probe', onNavigate: () => { this.visits++ } })
      Text('Preview visits: ' + String(this.visits)).id('preview-visits')
      DazzTextField({ value: this.value, accessibilityLabel: 'Preview name',
        onValueChange: (value: string) => { this.value = value; this.composing = this.editing.composing() } })
      Text('Preview value: ' + this.value).id('preview-value')
      Text('Preview composing: ' + String(this.composing)).id('preview-composition')
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
}

@Component
struct PreviewSpecimens {
  @State open: boolean = false
  @State accepted: number = 0
  @State refused: number = 0
  @LocalBuilder defaultTrigger() { DazzButton({ label: 'Default preview' }) }
  @LocalBuilder acceptedTrigger() { DazzButton({ label: 'Accepted preview' }) }
  @LocalBuilder refusedTrigger() { DazzButton({ label: 'Refused preview' }) }
  @LocalBuilder defaultBody() { Text('Default preview body').id('preview-default-body') }
  @LocalBuilder acceptedBody() { PreviewEditableBody() }
  @LocalBuilder refusedBody() { Text('Refused preview body').id('preview-refused-body') }
  build() {
    Column({ space: 24 }) {
      DazzPreviewCard({ defaultOpen: true, delay: 0, closeDelay: 0,
        trigger: this.defaultTrigger, content: this.defaultBody })
        .id('preview-default-trigger')
      DazzPreviewCard({ open: this.open, delay: 0, closeDelay: 0,
        contentWidth: 288, trigger: this.acceptedTrigger, content: this.acceptedBody,
        onOpenChange: (open: boolean) => { this.open = open; this.accepted++ } })
        .id('preview-accepted-trigger')
      DazzPreviewCard({ open: false, delay: 0, closeDelay: 0,
        trigger: this.refusedTrigger, content: this.refusedBody,
        onOpenChange: () => { this.refused++ } }).id('preview-refused-trigger')
      Text('Preview changes: ' + String(this.accepted)).id('preview-accepted')
      Text('Preview refused: ' + String(this.refused)).id('preview-refused')
      DazzButton({ label: 'Outside preview action' }).alignSelf(ItemAlign.End)
    }.alignItems(HorizontalAlign.Start).width('100%')
  }
}

@Component
struct SidebarSpecimens {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State open: boolean = false
  @State accepted: number = 0
  @State refused: number = 0
  @State shortcut: string = ''
  @State report: string = ''
  @LocalBuilder heading() { Text('Column').fontColor(this.tokens.color('content')) }
  @LocalBuilder sideBody() { DazzSidebarHeader({ content: this.heading }) }
  @LocalBuilder trigger() { DazzSidebarTrigger() }
  @LocalBuilder paneBody() { DazzSidebarHeader({ content: this.trigger }) }
  @LocalBuilder acceptedLayout() {
    DazzSidebar({ variant: 'inset', collapsible: 'icon', content: this.sideBody }).id('sidebar-probe-accepted-side')
    DazzSidebarInset({ content: this.paneBody })
  }
  @LocalBuilder uncontrolledLayout() {
    DazzSidebarInset({ content: this.paneBody })
    DazzSidebar({ side: 'right', variant: 'floating', collapsible: 'icon', content: this.sideBody }).id('sidebar-probe-local-side')
  }
  @LocalBuilder refusedLayout() {
    DazzSidebar({ content: this.sideBody }).id('sidebar-probe-refused-side')
    DazzSidebarInset({ content: this.paneBody })
  }
  measure(): void {
    const ui = this.getUIContext()
    const frame = ui.getComponentUtils().getRectangleById('sidebar-probe-frame')
    this.report = 'Scale: ' + String(ui.vp2px(1)) + '; frame: ' +
      String(ui.px2vp(frame.size.width)) + ' x ' + String(ui.px2vp(frame.size.height))
  }
  build() {
    Column({ space: this.tokens.metric('spacing.3') }) {
      Row() {
        DazzSidebarProvider({ open: this.open, content: this.acceptedLayout,
          onOpenChange: (open: boolean) => { this.open = open; this.accepted++ } })
      }.width(576).height(120).id('sidebar-probe-frame')
      Row() { DazzSidebarProvider({ defaultOpen: false, content: this.uncontrolledLayout }) }
        .width(576).height(120).id('sidebar-probe-local')
      Row() {
        DazzSidebarProvider({ open: true, content: this.refusedLayout,
          onOpenChange: () => { this.refused++ } })
      }.width(576).height(120).id('sidebar-probe-refused')
      Text('Accepted: ' + String(this.accepted)).id('sidebar-probe-accepted')
      Text('Refused: ' + String(this.refused)).id('sidebar-probe-attempts')
      DazzShortcutRecorder({ value: this.shortcut, accessibilityLabel: 'Sidebar binding',
        onValueChange: (value: string) => { this.shortcut = value } })
      DazzButton({ label: 'Measure sidebars', onAction: () => this.measure() })
      Text(this.report).id('sidebar-probe-report')
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
}

@Component
struct TreeSpecimens {
  @State expanded: string[] = []
  @State selected: string = 'leaf'
  @State attempts: number = 0
  @State changed: boolean = false
  items(): DazzTreeNode[] {
    return [{ id: 'root', label: this.changed ? 'Renamed folder' : 'Folder', children: [
      { id: 'empty', label: 'Empty folder', children: [] },
      { id: 'leaf', label: this.changed ? 'Renamed file' : 'A file', text: 'needle' },
    ] }, { id: 'locked', label: 'Unavailable', disabled: true }]
  }
  build() {
    Column({ space: 12 }) {
      DazzButton({ label: 'Before tree' })
      DazzTree({ items: this.items(), expanded: this.expanded, selected: this.selected,
        accessibilityLabel: 'Controlled files',
        onExpandedChange: (ids: string[]) => { this.expanded = ids },
        onSelectedChange: (id: string) => { this.selected = id } }).id('controlled-tree')
      DazzButton({ label: 'After tree' })
      DazzButton({ label: 'Rename tree', onAction: () => { this.changed = !this.changed } })
      Text('Expanded: ' + this.expanded.join(',')).id('tree-expanded')
      Text('Selected: ' + this.selected).id('tree-selected')
      DazzTree({ items: [{ id: 'refused', label: 'Refused folder', children: [] }],
        expanded: [], selected: undefined, accessibilityLabel: 'Refused files',
        onExpandedChange: () => { this.attempts++ },
        onSelectedChange: () => { this.attempts++ } }).id('refused-tree')
      Text('Attempts: ' + String(this.attempts)).id('tree-attempts')
    }.width(320).alignItems(HorizontalAlign.Start)
  }
}

@Component
struct OtpEditableSpecimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State code: string = ''
  @State completed: number = 0
  @State rejected: number = 0
  build() {
    Column({ space: this.tokens.metric('spacing.3') }) {
      DazzOtpField({ value: this.code, length: 6, required: true,
        autoSubmit: true, accessibilityLabel: 'Editable code',
        onValueChange: (value: string) => { this.code = value },
        onValueComplete: () => { this.completed++ },
        onValueInvalid: () => { this.rejected++ } })
      Row({ space: this.tokens.metric('spacing.2') }) {
        DazzButton({ label: 'Submit code', buttonType: 'submit' })
        DazzButton({ label: 'Reset code', buttonType: 'reset' })
      }
      Text('Code: ' + this.code).id('otp-stored')
      Text('Completed: ' + String(this.completed)).id('otp-completed')
      Text('Rejected: ' + String(this.rejected)).id('otp-rejected')
    }.alignItems(HorizontalAlign.Start)
  }
}

@Component
struct OtpSpecimens {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State submitted: number = 0
  @State attempted: number = 0
  @LocalBuilder editable() { OtpEditableSpecimen() }
  build() {
    Column({ space: this.tokens.metric('spacing.3') }) {
      DazzCodeBlock({ code: '12a 3456', copyable: true })
      DazzForm({ content: this.editable,
        onSubmit: () => { this.submitted++ } })
      Text('Submitted: ' + String(this.submitted)).id('otp-submitted')
      Text('Attempted: ' + String(this.attempted)).id('otp-attempted')
      DazzOtpField({ length: 4, defaultValue: '1234', readOnly: true,
        accessibilityLabel: 'Read only code' })
      DazzOtpField({ length: 4, defaultValue: '8765', disabled: true,
        accessibilityLabel: 'Disabled code' })
      DazzOtpField({ length: 4, value: '11',
        accessibilityLabel: 'Unchanged code',
        onValueChange: () => { this.attempted++ } })
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
}

// Keep modal state in its content component. In the API 26 SDK run, a builder
// owned by the background page refreshed Text but left a child's @Prop stale.
// The gallery's SceneNode similarly owns and interprets its own scene state.
@Component
struct ShortcutSpecimens {
  @Consume('dazzui.theme') tokens: DazzTokens
  @State shortcut: string = '⌃T'
  @State outsideClicks: number = 0
  build() {
    Column({ space: this.tokens.metric('spacing.4') }) {
      DazzShortcutRecorder({ value: this.shortcut, accessibilityLabel: 'Modal shortcut',
        placeholder: 'Choose keys', recordingLabel: 'Listening…', clearLabel: 'Remove binding',
        onValueChange: (value: string) => { this.shortcut = value } })
      DazzButton({ label: 'Other modal control', onAction: () => { this.outsideClicks++ } })
      Text('Stored: ' + this.shortcut).id('modal-shortcut-value')
      Text('Outside actions: ' + String(this.outsideClicks)).id('modal-shortcut-clicks')
    }.alignItems(HorizontalAlign.Start)
  }
}

// Test-only modal probes; the gallery uses React's stories.
@Component
struct ModalSpecimens {
  @Consume('dazzui.theme') tokens: DazzTokens
  private rows: number[] = []
  aboutToAppear(): void { for (let index: number = 0; index < 40; index++) this.rows.push(index) }
  @LocalBuilder topTrigger() { DazzButton({ label: 'Scrollable top' }) }
  @LocalBuilder bottomTrigger() { DazzButton({ label: 'Scrollable bottom' }) }
  @LocalBuilder bareTrigger() { DazzButton({ label: 'Without footer' }) }
  @LocalBuilder shortcutTrigger() { DazzButton({ label: 'Edit shortcut' }) }
  @LocalBuilder shortcutFields() { ShortcutSpecimens() }
  @LocalBuilder shortcutBody() { DazzDialogBody({ content: this.shortcutFields }) }
  @LocalBuilder otpTrigger() { DazzButton({ label: 'Edit code' }) }
  @LocalBuilder otpFields() { OtpSpecimens() }
  @LocalBuilder otpBody() { DazzDialogBody({ content: this.otpFields }) }
  @LocalBuilder treeTrigger() { DazzButton({ label: 'Edit tree' }) }
  @LocalBuilder treeFields() { TreeSpecimens() }
  @LocalBuilder treeBody() { DazzDialogBody({ content: this.treeFields }) }
  @LocalBuilder sidebarTrigger() { DazzButton({ label: 'Edit sidebar' }) }
  @LocalBuilder sidebarFields() { SidebarSpecimens() }
  @LocalBuilder sidebarBody() { DazzDialogBody({ content: this.sidebarFields }) }
  @LocalBuilder previewTrigger() { DazzButton({ label: 'Edit preview' }) }
  @LocalBuilder previewFields() { PreviewSpecimens() }
  @LocalBuilder previewBody() { DazzDialogBody({ content: this.previewFields }) }
  @LocalBuilder sliderTrigger() { DazzButton({ label: 'Edit range' }) }
  @LocalBuilder sliderFields() { SliderSpecimens() }
  @LocalBuilder sliderBody() { DazzDialogBody({ content: this.sliderFields }) }
  @LocalBuilder toastTrigger() { DazzButton({ label: 'Edit notifications' }) }
  @LocalBuilder toastFields() { ToastSpecimens() }
  @LocalBuilder toastBody() { DazzDialogBody({ content: this.toastFields }) }
  @LocalBuilder footer() { DazzDrawerClose() }
  @LocalBuilder body() {
    Column({ space: this.tokens.metric('spacing.4') }) {
      ForEach(this.rows, (index: number) => {
        Text('Content row ' + String(index))
          .fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size))
          .lineHeight(this.tokens.fontSize(this.tokens.type('bodySmall').leading))
          .fontColor(this.tokens.color('content')).id('modal-row-' + String(index))
      }, (index: number) => String(index))
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
  @LocalBuilder shortBody() {
    Text('A panel without an optional footer.')
      .fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size)).fontColor(this.tokens.color('content'))
  }
  build() {
    Column({ space: 16 }) {
      Row({ space: this.tokens.metric('spacing.4') }) {
        DazzDrawer({ side: 'top', extent: 256, title: 'Long top panel',
          trigger: this.topTrigger, content: this.body, footer: this.footer })
        DazzDrawer({ side: 'bottom', extent: 256, title: 'Long bottom panel',
          trigger: this.bottomTrigger, content: this.body, footer: this.footer })
        DazzDrawer({ side: 'right', extent: '50%', title: 'No footer',
          trigger: this.bareTrigger, content: this.shortBody })
        DazzDialog({ title: 'Shortcut modal', trigger: this.shortcutTrigger, content: this.shortcutBody })
        DazzDialog({ title: 'Code verification', trigger: this.otpTrigger, content: this.otpBody })
        DazzDialog({ title: 'Tree verification', trigger: this.treeTrigger, content: this.treeBody })
        DazzDialog({ title: 'Sidebar verification', contentWidth: 720, trigger: this.sidebarTrigger, content: this.sidebarBody })
        DazzDialog({ title: 'Preview verification', contentWidth: 720, trigger: this.previewTrigger, content: this.previewBody })
        DazzDialog({ title: 'Range verification', contentWidth: 520, trigger: this.sliderTrigger, content: this.sliderBody })
      }.width('100%')
      DazzDialog({ title: 'Notification verification', contentWidth: 520, trigger: this.toastTrigger, content: this.toastBody })
      ToastUnderlaySpecimens().width('100%').layoutWeight(1)
    }.width('100%').height('100%').alignItems(HorizontalAlign.Start).id('document-column')
  }
}

@Entry
@Component
struct ModalVerification {
  @LocalBuilder specimens() { ModalSpecimens() }
  build() {
    Column() { DazzTheme({ content: this.specimens }) }.width('100%').height('100%')
  }
}
