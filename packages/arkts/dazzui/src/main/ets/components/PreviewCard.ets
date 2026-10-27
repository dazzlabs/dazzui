import { FrameCallback, UIObserver } from '@kit.ArkUI'
import { DazzTokens } from '../foundation/Tokens'
import { DazzEditingContext } from '../foundation/Editing'
import { DazzPointerContext } from '../foundation/Pointer'
import { DazzPreviewAnchor, DazzPreviewInteraction, DazzPreviewPoint, DazzPreviewRect } from '../foundation/PreviewData'
import { DazzPreviewPolygon } from '../foundation/PreviewPolygon'
import { DazzPopupSurface, popupShadow } from './PopupSurface'

let previewSequence: number = 0
class PreviewFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  onIdle(_timeLeft: number): void { this.action() }
}

@Component
export struct DazzPreviewCard {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.editing') editing: DazzEditingContext = new DazzEditingContext()
  @Consume('dazzui.pointer') pointer: DazzPointerContext = new DazzPointerContext()
  @Prop @Watch('sync') open: boolean | undefined = undefined
  @Prop defaultOpen: boolean = false
  @Prop delay: number | undefined = undefined
  @Prop closeDelay: number | undefined = undefined
  @Prop side: string = 'bottom'
  @Prop contentAlign: string = 'center'
  @Prop contentWidth: number | undefined = undefined
  @State localOpen: boolean = false
  @State presented: boolean = false
  anchor: DazzPreviewAnchor = new DazzPreviewAnchor()
  onOpenChange: (open: boolean) => void = () => {}
  @BuilderParam trigger: () => void = this.empty
  @BuilderParam content: () => void = this.empty
  private interaction: DazzPreviewInteraction = new DazzPreviewInteraction()
  private polygon: DazzPreviewPolygon = new DazzPreviewPolygon()
  private pendingLeave: string = ''
  private closingRequested: boolean = false
  private previewID: string = 'dazzui-preview-' + String(++previewSequence)
  private mounted: boolean = false
  private watching: boolean = false
  private observer?: UIObserver
  private pressHandled: boolean = false
  private pressEpoch: number = 0
  private press: (x: number, y: number) => void = (x: number, y: number) => {
    this.pressHandled = true
    this.pressEpoch++
    this.outside(x, y)
  }
  private release: () => void = () => {
    const epoch = this.pressEpoch
    // willClick follows touch-up in the same input frame. Do not propose the
    // same outside dismissal once on press and again on its synthesized click.
    this.getUIContext().postFrameCallback(new PreviewFrame(() => {
      if (this.mounted && epoch === this.pressEpoch) this.pressHandled = false
    }))
  }
  private click: (event: ClickEvent) => void = (event: ClickEvent) => {
    if (!this.pressHandled && (event.source === SourceType.Mouse || event.source === SourceType.TouchScreen))
      this.outside(event.windowX, event.windowY)
  }
  private move: (x: number, y: number) => void = (x: number, y: number) => {
    if (!this.current()) return
    // Touch cancels ArkUI hover without moving the physical mouse. Release
    // its retained interaction only on a genuine mouse move, not that cancel.
    if (Number.isFinite(x) && Number.isFinite(y)) this.interaction.touchInside(false)
    const trigger = this.rectangle(this.previewID)
    const popup = this.rectangle(this.previewID + '-surface')
    if (!trigger || !popup) return
    const side = this.placedSide(trigger, popup)
    const positioner = this.positioner(popup, side)
    const point: DazzPreviewPoint = { x, y }
    const inside = (rect: DazzPreviewRect): boolean => x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom
    if (this.pendingLeave) {
      const target = this.pendingLeave
      this.pendingLeave = ''
      this.polygon.begin(point, side)
      this.interaction.transit(true)
      this.polygon.move(point, trigger, positioner, target, true, inside(positioner))
    }
    this.polygon.move(point, trigger, positioner,
      inside(positioner) ? 'popup' : inside(trigger) ? 'reference' : 'outside')
  }
  placedSide(trigger: DazzPreviewRect, popup: DazzPreviewRect): string {
    if (popup.top >= trigger.bottom) return 'bottom'
    if (popup.bottom <= trigger.top) return 'top'
    if (popup.left >= trigger.right) return 'right'
    if (popup.right <= trigger.left) return 'left'
    return this.side
  }
  positioner(popup: DazzPreviewRect, side: string): DazzPreviewRect {
    // React's positioner includes the gap as padding in its hover target.
    const gap = this.tokens.metric('spacing.1')
    return { left: popup.left - (side === 'right' ? gap : 0),
      right: popup.right + (side === 'left' ? gap : 0),
      top: popup.top - (side === 'bottom' ? gap : 0),
      bottom: popup.bottom + (side === 'top' ? gap : 0) }
  }
  hover(popup: boolean, entered: boolean): void {
    this.pendingLeave = !entered && this.current() ? (popup ? 'popup' : 'reference') : ''
    this.polygon.dispose()
    this.interaction.hover(popup, entered)
  }
  @LocalBuilder empty() {}
  current(): boolean { return this.open ?? this.localOpen }
  outside(x: number, y: number): void {
    if (!this.current()) return
    const trigger = this.rectangle(this.previewID)
    const surface = this.rectangle(this.previewID + '-surface')
    const popup = trigger && surface ? this.positioner(surface, this.placedSide(trigger, surface)) : surface
    const inside = (rect: DazzPreviewRect | undefined): boolean => !!rect &&
      x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom
    if (!inside(trigger) && !inside(popup)) this.interaction.dismiss()
  }
  watch(open: boolean): void {
    if (open === this.watching) return
    this.watching = open
    this.pressHandled = false
    this.pressEpoch++
    if (open) {
      this.observer = this.getUIContext().getUIObserver()
      this.observer.on('willClick', this.click)
      this.pointer.on(this.press)
      this.pointer.onMove(this.move)
      this.pointer.onRelease(this.release)
    } else {
      this.observer?.off('willClick', this.click)
      this.pointer.off(this.press)
      this.pointer.offMove(this.move)
      this.pointer.offRelease(this.release)
    }
  }
  rectangle(id: string): DazzPreviewRect | undefined {
    const ui = this.getUIContext()
    const node = ui.getFrameNodeById(id)
    if (!node) return undefined
    const position = node.getPositionToWindow()
    const size = node.getMeasuredSize()
    return { left: position.x, top: position.y,
      right: position.x + ui.px2vp(size.width), bottom: position.y + ui.px2vp(size.height) }
  }
  aboutToAppear(): void {
    this.mounted = true
    this.localOpen = this.defaultOpen
    this.interaction.current = () => this.current()
    this.interaction.openDelay = () => this.delay ?? this.tokens.metric('preview-card.open-delay')
    this.interaction.closeDelay = () => this.closeDelay ?? this.tokens.metric('preview-card.close-delay')
    this.interaction.change = (open: boolean) => this.propose(open)
    this.polygon.onClose = () => this.interaction.finishTransit()
    this.anchor.hover = (entered: boolean) => this.hover(false, entered)
    this.anchor.focus = (focused: boolean) => this.interaction.focus(false, focused)
    this.anchor.dismiss = () => this.interaction.dismiss()
    this.getUIContext().postFrameCallback(new PreviewFrame(() => this.sync()))
  }
  aboutToDisappear(): void {
    this.mounted = false
    this.interaction.dispose()
    this.polygon.dispose()
    this.watch(false)
    this.anchor.hover = () => {}
    this.anchor.focus = () => {}
    this.anchor.dismiss = () => {}
  }
  propose(open: boolean): void {
    this.closingRequested = !open
    if (this.open === undefined) this.localOpen = open
    this.sync()
    this.onOpenChange(open)
  }
  sync(): void {
    if (!this.mounted) return
    this.presented = this.current()
    if (this.presented && !this.watching) this.closingRequested = false
    this.watch(this.presented)
    if (!this.presented) {
      this.pendingLeave = ''
      this.polygon.dispose()
      this.interaction.dispose()
    }
  }
  placement(): Placement {
    if (this.side === 'top') return this.contentAlign === 'start' ? Placement.TopLeft
      : this.contentAlign === 'end' ? Placement.TopRight : Placement.Top
    if (this.side === 'left') return this.contentAlign === 'start' ? Placement.LeftTop
      : this.contentAlign === 'end' ? Placement.LeftBottom : Placement.Left
    if (this.side === 'right') return this.contentAlign === 'start' ? Placement.RightTop
      : this.contentAlign === 'end' ? Placement.RightBottom : Placement.Right
    return this.contentAlign === 'start' ? Placement.BottomLeft
      : this.contentAlign === 'end' ? Placement.BottomRight : Placement.Bottom
  }
  @LocalBuilder forwarded() { this.content() }
  @LocalBuilder surface() {
    Column() {
      DazzPopupSurface({ contentWidth: this.contentWidth,
        maximumWidth: this.tokens.metric('preview-card.width'), content: this.forwarded })
    }.width(LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent)
      .id(this.previewID + '-surface')
      .onHover((hovered: boolean) => this.hover(true, hovered))
      .onTouchIntercept((event: TouchEvent): HitTestMode => {
        if (event.source === SourceType.TouchScreen && event.type === TouchType.Down) this.interaction.touchInside(true)
        return HitTestMode.Default
      })
      .onFocus(() => this.interaction.focus(true, true))
      .onBlur(() => this.interaction.focus(true, false))
      .onMouse((event: MouseEvent) => {
        if (event.action === MouseAction.Move) this.pointer.move(event.windowX, event.windowY, event.timestamp)
      })
      .onKeyEvent((event: KeyEvent): boolean => this.handleKey(event))
      .onKeyPreIme((event: KeyEvent): boolean => this.handleKey(event))
  }
  handleKey(event: KeyEvent): boolean {
    if (event.type !== KeyType.Down || event.keyCode !== 2070 || !this.current() || this.editing.composing()) return false
    this.interaction.dismiss()
    event.stopPropagation()
    return true
  }
  build() {
    Row() { this.trigger() }.id(this.previewID)
      .onHover((hovered: boolean) => this.hover(false, hovered))
      .onFocus(() => this.interaction.focus(false, this.getUIContext().getFocusController().isActive()))
      .onBlur(() => this.interaction.focus(false, false))
      .onKeyEvent((event: KeyEvent): boolean => this.handleKey(event))
      .bindPopup(this.presented, {
        builder: this.surface, placement: this.placement(), mask: false,
        focusable: false, enableArrow: false, targetSpace: this.tokens.metric('spacing.1'),
        popupColor: Color.Transparent, backgroundBlurStyle: BlurStyle.NONE,
        radius: this.tokens.metric('radius.large'), shadow: popupShadow(this.tokens, this.getUIContext().vp2px(1)),
        autoCancel: false,
        onWillDismiss: (action: DismissPopupAction) => {
          // Explicit press observation distinguishes the trigger from the
          // actual outside target, including touch while a mouse is parked.
          if (action.reason !== DismissReason.TOUCH_OUTSIDE) this.interaction.dismiss()
        },
        onStateChange: (event: PopupStateChangeParam) => {
          if (!event.isVisible && this.presented) {
            this.presented = false
            this.watch(false)
            this.interaction.dispose()
            // A native hide callback can precede propagation of a controlled
            // host's accepted value. Its earlier proposal already notified it.
            if (this.current() && !this.closingRequested) {
              this.closingRequested = true
              if (this.open === undefined) this.localOpen = false
              this.onOpenChange(false)
            }
          }
        },
      })
  }
}
