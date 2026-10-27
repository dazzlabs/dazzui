import { ComponentContent, FrameCallback, LengthMetrics, LevelOrder, curves, window } from '@kit.ArkUI'
import { common } from '@kit.AbilityKit'
import { BusinessError } from '@kit.BasicServicesKit'
import { hilog } from '@kit.PerformanceAnalysisKit'
import { accessibility } from '@kit.AccessibilityKit'
import { DazzTokens } from '../foundation/Tokens'
import { DazzToastContext, DazzToastQueue, DazzToastItem } from '../foundation/ToastData'
import { DazzActivationContext } from '../foundation/Activation'
import { DazzPointerContext } from '../foundation/Pointer'
import { DazzToastSwipe } from '../foundation/ToastSwipe'
import { DazzShadowLayer } from './ShadowLayer'
import { DazzButton } from './Button'
import { DazzIcon } from './Icon'
import { focusID, rememberFocus, forgetFocus, rememberFocusScope, forgetFocusScope, focusTarget, captureFocus } from '../foundation/FocusData'
import { observeToastShortcuts, unobserveToastShortcuts, setToastShortcut, toastShortcutProvider, enterWindowToasts, toastFocusStops, toastRootFocus, toastActionFocus, toastDismissFocus } from '../foundation/ToastFocus'

let viewportSequence: number = 0

class ToastFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  onIdle(_timeLeft: number): void { this.action() }
}

@Component
export struct DazzToastProvider {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop @Watch('syncLimit') limit: number = 3
  @Provide({ allowOverride: 'dazzui.toast' }) toast: DazzToastContext = new DazzToastContext()
  @BuilderParam content: () => void = this.empty
  @State shortcut: boolean = false
  private shortcutChanged: () => void = () => {}
  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.toast.providerID = focusID('toast-provider')
    this.shortcutChanged = () => { this.shortcut = toastShortcutProvider(this.getUIContext().getWindowName() ?? '') === this.toast.providerID }
    observeToastShortcuts(this.shortcutChanged)
    this.shortcutChanged()
    this.toast.queue = new DazzToastQueue({
      notice: this.tokens.metric('toast.notice-duration'),
      action: this.tokens.metric('toast.action-duration'),
      transition: this.tokens.metric('toast.transition.duration'),
    }, this.limit)
  }
  syncLimit(): void { this.toast.queue?.setLimit(this.limit) }
  aboutToDisappear(): void { unobserveToastShortcuts(this.shortcutChanged); this.toast.queue?.dispose(); this.toast.queue = undefined }
  build() {
    Row() { this.content() }
      .width(LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent)
      .keyboardShortcut(this.shortcut ? FunctionKey.F6 : '', [], () => enterWindowToasts(this.getUIContext().getWindowName() ?? ''))
  }
}

@Observed
class ToastLayerData {
  tokens: DazzTokens = new DazzTokens()
  items: DazzToastItem[] = []
  placement: string = 'bottom'
  viewportID: string = ''
  width: number = 0
  windowName: string = ''
  previousFocus: string = ''
  focusReady: boolean = false
  moveFocus: (id: string) => boolean = () => false
  context: DazzToastContext = new DazzToastContext()
  pointer: DazzPointerContext = new DazzPointerContext()
  outsidePress: (x: number, y: number) => void = () => {}
  hovered: Set<string> = new Set()
  focused: Set<string> = new Set()
  touching: Set<string> = new Set()
  hover(id: string, active: boolean): void {
    if (active) this.hovered.add(id)
    else this.hovered.delete(id)
    this.context.queue?.pause('hover', this.hovered.size > 0)
  }
  focus(id: string, active: boolean): void {
    if (active) this.focused.add(id)
    else this.focused.delete(id)
    this.context.queue?.pause('focus', this.focused.size > 0)
  }
  touch(id: string, active: boolean): void {
    if (active) this.touching.add(id)
    else this.touching.delete(id)
    this.context.queue?.pause('touch', this.touching.size > 0)
  }
  outside(): void {
    this.hovered.clear()
    this.focused.clear()
    this.touching.clear()
    this.context.queue?.pause('hover', false)
    this.context.queue?.pause('focus', false)
    this.context.queue?.pause('touch', false)
  }
  anchorID(): string { return this.viewportID + '-focus' }
  enter(): boolean {
    if (focusTarget(this.windowName) === this.anchorID()) return true
    this.previousFocus = captureFocus(this.windowName)
    this.focus('viewport', true)
    return this.moveFocus(this.anchorID())
  }
  restore(): boolean { return !!this.previousFocus && this.moveFocus(this.previousFocus) }
  tab(reverse: boolean): boolean {
    const current = focusTarget(this.windowName)
    const stops = toastFocusStops(this.viewportID, this.items)
    if (current === this.anchorID()) {
      if (reverse || !stops.length) return this.restore()
      return this.moveFocus(stops[0])
    }
    const index = stops.indexOf(current)
    if (index < 0) return false
    const next = stops[index + (reverse ? -1 : 1)]
    return next ? this.moveFocus(next) : this.restore()
  }
}

@Component
struct ToastDismiss {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = 'Dismiss'
  @Prop controlID: string = ''
  @State hovered: boolean = false
  @State focused: boolean = false
  onAction: () => void = () => {}
  aboutToDisappear(): void { forgetFocus(this.getUIContext().getWindowName() ?? '', this.controlID) }
  build() {
    Button({ type: ButtonType.Normal }) {
      DazzIcon({ name: 'Dismiss12Regular', iconSize: this.tokens.metric('icon.small'),
        color: this.tokens.color(this.hovered ? 'content' : 'content-subtle') })
    }
      .id(this.controlID).width(this.tokens.size('tiny')).height(this.tokens.size('tiny')).padding(0)
      .backgroundColor(this.hovered ? this.tokens.paint('normal', 'surface', 'hovered', 'neutral') : Color.Transparent)
      .border({ width: 0, radius: this.tokens.metric('radius.small') })
      .outline({ width: this.focused ? this.tokens.metric('focus.width') : 0,
        color: this.tokens.focus(), radius: this.tokens.metric('radius.small') })
      .stateEffect(false).accessibilityText(this.label)
      .onHover((hovered: boolean) => { this.hovered = hovered })
      .onFocus(() => { this.focused = this.getUIContext().getFocusController().isActive(); rememberFocus(this.getUIContext().getWindowName() ?? '', this.controlID) })
      .onBlur(() => { this.focused = false; forgetFocus(this.getUIContext().getWindowName() ?? '', this.controlID) })
      .onClick(() => this.onAction())
  }
}

@Component
struct ToastReceipt {
  @Consume('dazzui.theme') tokens: DazzTokens
  @ObjectLink data: ToastLayerData
  @Prop @Watch('syncPhase') item: DazzToastItem
  @State hidden: boolean = true
  @State actionWidth: number = 0
  @State receiptWidth: number = 0
  @State receiptHeight: number = 0
  @State dragging: boolean = false
  @State dragX: number = 0
  @State dragY: number = 0
  @State dragScale: number = 1
  private swipe: DazzToastSwipe = new DazzToastSwipe()
  private dragPointer: number = -1
  private lastX: number = 0
  private lastY: number = 0
  private originX: number = 0
  private originY: number = 0
  private lastMoveTime: number = -1
  private pointerHovered: boolean = false
  private alive: boolean = false
  aboutToAppear(): void {
    this.alive = true
    this.getUIContext().postFrameCallback(new ToastFrame(() => { if (this.alive) this.syncPhase() }))
  }
  aboutToDisappear(): void {
    this.alive = false
    this.swipe.end(this.dragPointer, true)
    this.data.hover(this.item.id, false)
    this.data.focus(this.item.id, false)
    this.data.touch(this.item.id, false)
    forgetFocus(this.data.windowName, toastRootFocus(this.data.viewportID, this.item.id))
  }
  syncPhase(): void {
    if (!this.alive) return
    this.getUIContext().animateTo({ duration: this.tokens.metric('toast.transition.duration'),
      curve: curves.cubicBezierCurve(this.tokens.metric('toast.transition.x1'), this.tokens.metric('toast.transition.y1'),
        this.tokens.metric('toast.transition.x2'), this.tokens.metric('toast.transition.y2')),
    }, () => { this.hidden = this.item.ending; if (this.item.ending) this.dragging = false })
  }
  interactive(x: number, y: number): boolean {
    const ui = this.getUIContext()
    for (const id of [toastActionFocus(this.data.viewportID, this.item.id), toastDismissFocus(this.data.viewportID, this.item.id)]) {
      const node = ui.getAttachedFrameNodeById(id)
      if (!node) continue
      const position = node.getPositionToWindow()
      const size = node.getMeasuredSize()
      if (x >= position.x && x <= position.x + ui.px2vp(size.width) &&
        y >= position.y && y <= position.y + ui.px2vp(size.height)) return true
    }
    return false
  }
  startDrag(pointer: number, x: number, y: number): void {
    if (this.swipe.active && this.dragPointer === pointer) return
    if (!this.alive || this.item.ending || this.item.limited || this.interactive(x, y)) return
    const rect = this.getUIContext().getComponentUtils().getRectangleById(this.data.viewportID + '/' + this.item.id)
    if (!this.swipe.begin(pointer, x, y, { x: rect.translate.x, y: rect.translate.y, scale: rect.scale.x })) return
    this.dragPointer = pointer
    this.originX = x; this.originY = y
    this.lastX = x; this.lastY = y; this.lastMoveTime = -1
    this.dragX = this.swipe.transform.x; this.dragY = this.swipe.transform.y; this.dragScale = this.swipe.transform.scale
    this.dragging = true
    this.data.hover(this.item.id, true)
    // Like the web root's tabIndex=0, a body press selects the receipt as the
    // native focus target without activating keyboard focus or its ring.
    try { this.getUIContext().getFocusController().requestFocus(toastRootFocus(this.data.viewportID, this.item.id)) }
    catch (error) {
      const code = (error as BusinessError).code
      if (code !== 150001 && code !== 150002 && code !== 150003) throw error as Error
    }
  }
  moveDrag(pointer: number, x: number, y: number, timestamp: number): void {
    if (!this.alive || !this.swipe.active || pointer !== this.dragPointer ||
      (timestamp === this.lastMoveTime && x === this.lastX && y === this.lastY)) return
    this.swipe.move(pointer, x, y, x - this.lastX, y - this.lastY)
    this.lastX = x; this.lastY = y; this.lastMoveTime = timestamp
    this.dragX = this.swipe.transform.x; this.dragY = this.swipe.transform.y
  }
  finishDrag(pointer: number, cancel: boolean = false): void {
    if (!this.swipe.active || pointer !== this.dragPointer) return
    const dismiss = this.swipe.end(pointer, cancel)
    this.data.touch(this.item.id, false)
    this.data.hover(this.item.id, this.pointerHovered)
    if (dismiss) { this.data.context.close(this.item.id); return }
    this.getUIContext().animateTo({ duration: this.tokens.metric('toast.transition.duration'),
      curve: curves.cubicBezierCurve(this.tokens.metric('toast.transition.x1'), this.tokens.metric('toast.transition.y1'),
        this.tokens.metric('toast.transition.x2'), this.tokens.metric('toast.transition.y2')),
    }, () => { this.dragging = false })
  }
  touch(event: TouchEvent): void {
    if (event.source !== SourceType.TouchScreen) return
    if (event.type === TouchType.Cancel) {
      this.data.touch(this.item.id, false)
      this.finishDrag(this.dragPointer, true)
      this.data.pointer.release()
      return
    }
    const point = event.changedTouches[0]
    if (!point) return
    if (event.type === TouchType.Down) {
      this.data.pointer.press(point.windowX, point.windowY, event.timestamp, this.data.outsidePress)
      if (!this.alive) return
      this.data.touch(this.item.id, true)
      this.startDrag(point.id, point.windowX, point.windowY)
    } else if (event.type === TouchType.Move) {
      const active = event.touches.find((touch: TouchObject): boolean => touch.id === this.dragPointer)
      if (active) this.moveDrag(active.id, active.windowX, active.windowY, event.timestamp)
    } else {
      this.data.touch(this.item.id, false)
      this.finishDrag(point.id)
      this.data.pointer.release()
    }
  }
  mouse(event: MouseEvent): void {
    if (event.action === MouseAction.Press) this.data.pointer.press(event.windowX, event.windowY, event.timestamp, this.data.outsidePress)
    if (event.action === MouseAction.Move) this.data.pointer.move(event.windowX, event.windowY, event.timestamp)
    if (event.action === MouseAction.Release) this.data.pointer.release()
    if (event.action === MouseAction.Press && event.button === MouseButton.Left) this.startDrag(-2, event.windowX, event.windowY)
    else if (event.action === MouseAction.Move) this.moveDrag(-2, event.windowX, event.windowY, event.timestamp)
    else if (event.action === MouseAction.Release) this.finishDrag(-2)
  }
  pan(event: GestureEvent): void {
    this.moveDrag(this.dragPointer, this.originX + event.offsetX, this.originY + event.offsetY, event.timestamp)
  }
  panAllowed(event: BaseGestureEvent): boolean {
    if (!this.alive || this.item.ending || this.item.limited || event.axisHorizontal || event.axisVertical) return false
    if (this.swipe.active) return true
    const points = event.fingerInfos?.length ? event.fingerInfos : event.fingerList
    const point = points.find((finger: FingerInfo): boolean => !!finger)
    const pan = event as PanGestureEvent
    return !!point && !this.interactive(point.globalX - pan.offsetX, point.globalY - pan.offsetY)
  }
  beginPan(event: GestureEvent): void {
    if (!this.swipe.active) {
      const points = event.fingerInfos?.length ? event.fingerInfos : event.fingerList
      const point = points.find((finger: FingerInfo): boolean => !!finger)
      if (!point) return
      this.startDrag(event.source === SourceType.Mouse ? -2 : point.id,
        point.globalX - event.offsetX, point.globalY - event.offsetY)
    }
    // A zero-distance recognizer can begin on pointerdown. That is not the
    // first pointermove which Base UI deliberately discards.
    if (event.offsetX || event.offsetY) this.pan(event)
  }
  maximum(): number {
    return Math.min(this.tokens.metric('toast.max-width'), this.data.width > 0 ? Math.max(0, this.data.width - 2 * this.tokens.metric('spacing.4')) : this.tokens.metric('toast.max-width'))
  }
  textWidth(): number {
    const gaps = Number(!!this.item.icon) + Number(!!this.item.action) + Number(this.item.dismissable)
    return Math.max(0, this.maximum() - this.tokens.metric('spacing.3') - this.tokens.metric('spacing.1.5') -
      2 * this.tokens.metric('stroke.hairline') - gaps * this.tokens.metric('spacing.2.5') -
      (this.item.icon ? this.tokens.metric('icon.medium') : 0) - (this.item.action ? this.actionWidth : 0) -
      (this.item.dismissable ? this.tokens.size('tiny') : 0))
  }
  build() {
    Stack({ alignContent: Alignment.TopStart }) {
      if (this.receiptWidth > 0 && this.receiptHeight > 0) {
        DazzShadowLayer({ paintWidth: this.receiptWidth, paintHeight: this.receiptHeight,
          corner: this.tokens.metric('radius.big'), shadowLayers: this.tokens.shadow('shadow.md') })
          .position({ x: 0, y: 0 }).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      }
      Row({ space: this.tokens.metric('spacing.2.5') }) {
        if (this.item.icon) { DazzIcon({ name: this.item.icon, iconSize: this.tokens.metric('icon.medium'), color: this.tokens.color(this.item.tint + '.600') }) }
        Text(this.item.title)
          .fontSize(this.tokens.fontSize(this.tokens.type('labelMedium').size))
          .fontWeight(this.tokens.type('labelMedium').weight)
          .lineHeight(this.tokens.fontSize(this.tokens.type('labelMedium').size * 1.4))
          .fontColor(this.tokens.color('content')).maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis })
          .padding({ top: this.tokens.metric('spacing.0.5'), bottom: this.tokens.metric('spacing.0.5') })
          .constraintSize({ maxWidth: this.textWidth() })
        if (this.item.action) {
          DazzButton({ label: this.item.action.label, controlSize: 'small', tint: this.item.tint, variant: 'plain',
            focusControlID: toastActionFocus(this.data.viewportID, this.item.id),
            onAction: () => this.data.context.queue?.activate(this.item.id) })
            .onAreaChange((_old: Area, area: Area) => { this.actionWidth = Number(area.width) })
        }
        if (this.item.dismissable) { ToastDismiss({ label: this.item.dismissLabel, controlID: toastDismissFocus(this.data.viewportID, this.item.id), onAction: () => this.data.context.close(this.item.id) }) }
      }
        .width(LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent)
        .constraintSize({ minHeight: this.tokens.metric('spacing.9'), maxWidth: this.maximum() })
        .padding({ left: this.tokens.metric('spacing.3') + this.tokens.metric('stroke.hairline'), right: this.tokens.metric('spacing.1.5') + this.tokens.metric('stroke.hairline'),
          top: this.tokens.metric('spacing.1.5') + this.tokens.metric('stroke.hairline'), bottom: this.tokens.metric('spacing.1.5') + this.tokens.metric('stroke.hairline') })
        .backgroundColor(this.tokens.color('surface-overlay'))
        .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color('border-strong'), radius: this.tokens.metric('radius.big') })
        .id('dazzui-toast-surface/' + this.data.viewportID + '/' + this.item.id)
        .hitTestBehavior(HitTestMode.Default)
        .onAreaChange((_old: Area, area: Area) => { this.receiptWidth = Number(area.width); this.receiptHeight = Number(area.height) })
      // The toast itself is a Tab stop before its buttons. An absolute native
      // focus node covers the complete receipt without becoming a pointer target
      // or changing the row's intrinsic dimensions.
      Text('').width(this.receiptWidth).height(this.receiptHeight)
        .position({ x: 0, y: 0 }).hitTestBehavior(HitTestMode.None)
        .id(toastRootFocus(this.data.viewportID, this.item.id)).focusable(true).tabIndex(0)
        .borderRadius(this.tokens.metric('radius.big'))
        .focusBox({ margin: LengthMetrics.vp(0), strokeWidth: LengthMetrics.vp(this.tokens.metric('spacing.px')) })
        .accessibilityText(this.item.title)
        .onFocus(() => rememberFocus(this.data.windowName, toastRootFocus(this.data.viewportID, this.item.id)))
        .onBlur(() => forgetFocus(this.data.windowName, toastRootFocus(this.data.viewportID, this.item.id)))
    }
      .id(this.data.viewportID + '/' + this.item.id)
      .width(LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent)
      .clip(false)
      .responseRegion({ x: 0, y: 0, width: this.receiptWidth, height: this.receiptHeight })
      .hitTestBehavior(HitTestMode.BLOCK_HIERARCHY)
      .opacity(this.hidden ? 0 : 1)
      .translate({ x: this.dragging ? this.dragX : 0, y: this.dragging ? this.dragY : this.hidden ? this.tokens.metric('toast.transition.offset') : 0 })
      .scale({ x: this.dragging ? this.dragScale : this.hidden ? this.tokens.metric('toast.transition.scale') : 1, y: this.dragging ? this.dragScale : this.hidden ? this.tokens.metric('toast.transition.scale') : 1 })
      .enabled(!this.item.limited && !this.item.ending).focusable(!this.item.limited && !this.item.ending)
      .accessibilityRole(this.item.priority === 'high' ? AccessibilityRoleType.ALERT_DIALOG : AccessibilityRoleType.DIALOG)
      .accessibilityText(this.item.title)
      .accessibilityLevel(this.item.limited || this.item.ending ? 'no-hide-descendants' : 'auto')
      .onHover((hovered: boolean) => { this.pointerHovered = hovered; this.data.hover(this.item.id, (hovered || this.swipe.active) && !this.item.limited) })
      .onTouch((event: TouchEvent) => this.touch(event))
      .onMouse((event: MouseEvent) => this.mouse(event))
      .onGestureRecognizerJudgeBegin((event: BaseGestureEvent, current: GestureRecognizer, _others: GestureRecognizer[]): GestureJudgeResult =>
        current.getType() !== GestureControl.GestureType.PAN_GESTURE ||
        this.panAllowed(event) ? GestureJudgeResult.CONTINUE : GestureJudgeResult.REJECT)
      .gesture(PanGesture({ fingers: 1, direction: PanDirection.All, distance: 0 })
        .onActionStart((event: GestureEvent) => this.beginPan(event))
        .onActionUpdate((event: GestureEvent) => this.pan(event))
        .onActionEnd(() => this.finishDrag(this.dragPointer))
        .onActionCancel(() => this.finishDrag(this.dragPointer, true)))
      .onFocus(() => this.data.focus(this.item.id, this.getUIContext().getFocusController().isActive()))
      .onBlur(() => this.data.focus(this.item.id, false))
      .onKeyEvent((event: KeyEvent): boolean => {
        if (event.keyCode !== 2070 || event.type !== KeyType.Down) return false
        this.data.context.close(this.item.id)
        event.stopPropagation()
        return true
      })
  }
}

@Component
struct ToastLayer {
  @ObjectLink @Watch('syncTheme') data: ToastLayerData
  @Provide({ allowOverride: 'dazzui.theme' }) tokens: DazzTokens = new DazzTokens()
  @Provide({ allowOverride: 'dazzui.activation' }) activation: DazzActivationContext = new DazzActivationContext()
  aboutToAppear(): void { this.syncTheme() }
  aboutToDisappear(): void {
    forgetFocus(this.data.windowName, this.data.anchorID())
    forgetFocusScope(this.data.windowName, this.data.viewportID)
  }
  syncTheme(): void { this.tokens = this.data.tokens }
  ordered(): DazzToastItem[] { return this.data.placement === 'top' ? this.data.items.slice().reverse() : this.data.items }
  build() {
    Column() {
      Column({ space: this.tokens.metric('spacing.2') }) {
        ForEach(this.ordered(), (item: DazzToastItem) => { ToastReceipt({ data: this.data, item }) }, (item: DazzToastItem): string => item.id)
        // A layout container is a focus scope in ArkUI. This native node is
        // the programmatic region stop, without changing the viewport's box.
        Text('').width(1).height(1).position({ x: 0, y: 0 })
          .id(this.data.anchorID()).focusable(true).tabIndex(-1)
          .focusBox({ strokeWidth: LengthMetrics.vp(0) }).accessibilityText('Notifications')
          .onFocus(() => { rememberFocus(this.data.windowName, this.data.anchorID()); this.data.focus('viewport', true) })
          .onBlur(() => forgetFocus(this.data.windowName, this.data.anchorID()))
      }
        .id(this.data.viewportID).width('100%').height(LayoutPolicy.wrapContent)
        .padding(this.tokens.metric('spacing.4')).hitTestBehavior(HitTestMode.None)
        .accessibilityText('Notifications')
        .onAreaChange((_old: Area, area: Area) => { this.data.width = Number(area.width) })
    }.width('100%').height('100%')
        .justifyContent(this.data.placement === 'top' ? FlexAlign.Start : FlexAlign.End)
        .hitTestBehavior(HitTestMode.None).focusable(this.data.focusReady)
        .onFocus(() => {
          rememberFocusScope(this.data.windowName, this.data.viewportID)
          if (this.getUIContext().getFocusController().isActive()) this.data.focus('viewport', true)
        })
        .onBlur(() => { forgetFocusScope(this.data.windowName, this.data.viewportID); this.data.focus('viewport', false) })
        .onKeyPreIme((event: KeyEvent): boolean => {
          if (event.type !== KeyType.Down || event.keyCode !== 2049) return false
          if (!this.data.tab(event.getModifierKeyState?.(['Shift']) ?? false)) return false
          event.stopPropagation(); return true
        })
  }
}

// Transparency must cross the custom-component boundary as well as its Column.
// Otherwise its full-window wrapper intercepts controls below the receipts.
@Builder
function toastLayer(data: ToastLayerData) { ToastLayer({ data }).hitTestBehavior(HitTestMode.None) }

@Component
export struct DazzToastViewport {
  @Consume('dazzui.theme') @Watch('refresh') tokens: DazzTokens
  @Consume('dazzui.toast') toast: DazzToastContext = new DazzToastContext()
  @Consume('dazzui.pointer') pointer: DazzPointerContext = new DazzPointerContext()
  @Prop @Watch('refresh') placement: string = 'bottom'
  @Prop viewportId: string = ''
  private data: ToastLayerData = new ToastLayerData()
  private content?: ComponentContent<ToastLayerData>
  private unsubscribe: () => void = () => {}
  private observedWindow?: window.Window
  private announced: Map<string, number> = new Map()
  private mounted: boolean = false
  private overlayOrder: number = ++viewportSequence
  private outsidePress: () => void = () => {}
  private windowChanged: (event: window.WindowEventType) => void = () => {}
  private windowResized: () => void = () => {}
  aboutToAppear(): void {
    this.mounted = true
    if (!this.toast.queue) throw new Error('ToastViewport requires a ToastProvider')
    this.data.context = this.toast
    this.data.viewportID = this.viewportId || 'dazzui-toast-viewport-' + String(this.overlayOrder)
    this.data.windowName = this.getUIContext().getWindowName() ?? ''
    this.data.moveFocus = (id: string): boolean => this.moveFocus(id)
    this.unsubscribe = this.toast.queue.subscribe(() => this.refresh())
    this.outsidePress = () => this.data.outside()
    this.data.pointer = this.pointer
    this.data.outsidePress = this.outsidePress
    this.pointer.on(this.outsidePress)
    this.refresh()
    this.windowChanged = (event: window.WindowEventType) => {
      if (event === window.WindowEventType.WINDOW_INACTIVE || event === window.WindowEventType.WINDOW_HIDDEN) this.toast.queue?.pause('window', true)
      if (event === window.WindowEventType.WINDOW_ACTIVE) this.toast.queue?.pause('window', false)
    }
    this.windowResized = () => {
      this.getUIContext().postFrameCallback(new ToastFrame(() => { if (this.mounted) this.refresh() }))
    }
    try {
      const name = this.getUIContext().getWindowName()
      if (!name) throw new Error('ToastViewport requires a window')
      const observed = window.findWindow(name)
      this.observedWindow = observed
      observed.on('windowEvent', this.windowChanged)
      observed.on('windowSizeChange', this.windowResized)
      this.toast.queue.pause('window', !observed.isFocused())
    } catch (_error) {
      hilog.warn(0x0000, 'DazzToast', 'Window observation unavailable')
    }
  }
  moveFocus(id: string): boolean {
    const ui = this.getUIContext()
    if (!this.mounted || !ui.getAttachedFrameNodeById(id)) return false
    try {
      ui.getFocusController().activate(true)
      ui.getFocusController().requestFocus(id)
      return true
    } catch (error) {
      const code = (error as BusinessError).code
      // A consumer can remove or disable the previous control during a toast.
      if (code === 150001 || code === 150002 || code === 150003) return false
      throw error as Error
    }
  }
  refresh(): void {
    if (!this.mounted) return
    this.data.tokens = this.tokens
    this.data.placement = this.placement
    const previousItems = this.data.items
    const focused = focusTarget(this.data.windowName)
    const currentIndex = previousItems.findIndex((item: DazzToastItem): boolean =>
      toastFocusStops(this.data.viewportID, [item]).includes(focused))
    const nextItems = this.toast.queue?.snapshot() ?? []
    this.data.items = nextItems
    if (currentIndex >= 0 && this.getUIContext().getFocusController().isActive() &&
      !toastFocusStops(this.data.viewportID, nextItems).includes(focused)) {
      const closingIndex = nextItems.findIndex((item: DazzToastItem): boolean => item.id === previousItems[currentIndex].id)
      const available = (item: DazzToastItem): boolean => !item.ending && !item.limited
      const next = nextItems.slice(closingIndex + 1).find(available) ?? nextItems.slice(0, closingIndex).reverse().find(available)
      if (next) this.moveFocus(toastRootFocus(this.data.viewportID, next.id))
      else this.data.restore()
    }
    const ui = this.getUIContext()
    if (this.data.items.length > 0 && !this.content) {
      this.data.focusReady = false
      this.content = new ComponentContent(ui, wrapBuilder<[ToastLayerData]>(toastLayer), this.data)
      // Keep provider mount order even when an older viewport opens later.
      ui.getOverlayManager().addComponentContentWithOrder(this.content, LevelOrder.clamp(this.overlayOrder))
      // Adding a top ordered overlay requests focus. Keep its first mount outside
      // keyboard traversal; enabling an already mounted scope does not take
      // focus away from the control that raised this non-modal notification.
      ui.postFrameCallback(new ToastFrame(() => {
        if (!this.mounted || !this.content) return
        this.data.focusReady = true
        this.content.update(this.data)
      }))
    } else if (this.data.items.length === 0 && this.content) {
      ui.getOverlayManager().removeComponentContent(this.content)
      this.content.dispose()
      this.content = undefined
    } else this.content?.update(this.data)
    setToastShortcut(this.data.windowName, this.data.viewportID, this.toast.providerID, this.data.items.length > 0, () => {
      if (this.data.enter()) return
      const previous = focusTarget(this.data.windowName)
      // Newly added detached content may not have mounted until this frame.
      this.getUIContext().postFrameCallback(new ToastFrame(() => {
        if (this.mounted && this.data.items.length && focusTarget(this.data.windowName) === previous) this.moveFocus(this.data.anchorID())
      }))
    })
    const live = new Set(this.data.items.map((item: DazzToastItem): string => item.id))
    for (const id of this.announced.keys()) if (!live.has(id)) this.announced.delete(id)
    for (const item of this.data.items) {
      if (item.ending || item.limited || this.announced.get(item.id) === item.revision) continue
      this.announced.set(item.id, item.revision)
      const context = ui.getHostContext() as common.UIAbilityContext
      accessibility.sendAccessibilityEvent({ type: item.priority === 'high' ? 'announceForAccessibility' : 'announceForAccessibilityNotInterrupt',
        bundleName: context.abilityInfo.bundleName, triggerAction: 'common', textAnnouncedForAccessibility: item.title }).catch(() => {})
    }
  }
  aboutToDisappear(): void {
    this.mounted = false
    setToastShortcut(this.data.windowName, this.data.viewportID, this.toast.providerID, false, () => {})
    this.unsubscribe()
    this.pointer.off(this.outsidePress)
    this.observedWindow?.off('windowEvent', this.windowChanged)
    this.observedWindow?.off('windowSizeChange', this.windowResized)
    this.observedWindow = undefined
    this.announced.clear()
    this.data.outside()
    if (this.content) {
      this.getUIContext().getOverlayManager().removeComponentContent(this.content)
      this.content.dispose()
      this.content = undefined
    }
  }
  build() { Row() {}.id(this.data.viewportID + '-host').width(0).height(0).accessibilityLevel('no') }
}
