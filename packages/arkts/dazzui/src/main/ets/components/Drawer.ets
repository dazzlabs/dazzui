import { curves } from '@kit.ArkUI'
import { DazzActivationBoundary, DazzActivationContext } from '../foundation/Activation'
import { DazzTokens } from '../foundation/Tokens'
import { DazzDialogBody, DazzDialogFooter, DazzDialogHeader } from './Dialog'
import { DazzButton } from './Button'

export type DazzDrawerSide = 'left' | 'right' | 'top' | 'bottom'
let drawerID: number = 0

export class DazzDrawerContext {
  controller?: CustomDialogController
  close(): void { this.controller?.close() }
}

/** Native modal lifecycle and gesture recognition, with the shared sheet rows. */
@CustomDialog
struct DazzDrawerSurface {
  controller?: CustomDialogController
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide({ allowOverride: 'dazzui.activation' }) activation: DazzActivationContext = new DazzActivationContext()
  @Provide({ allowOverride: 'dazzui.drawer' }) drawer: DazzDrawerContext = new DazzDrawerContext()
  @Prop side: DazzDrawerSide = 'right'
  @Prop title: string = ''
  @Prop description: string = ''
  @Prop extent: Length | undefined = undefined
  @BuilderParam content: () => void = this.empty
  @BuilderParam footer: () => void = this.empty
  @Prop footerVisible: boolean = false
  private scroller: Scroller = new Scroller()
  @State bodyHeight: number = 0
  @State viewportHeight: number = 0
  @LocalBuilder empty() {}
  aboutToAppear(): void { this.drawer.controller = this.controller }
  horizontal(): boolean { return this.side === 'top' || this.side === 'bottom' }
  corner(): BorderRadiuses {
    const radius = this.tokens.metric('frame.window.radius')
    return {
      topLeft: this.side === 'right' || this.side === 'bottom' ? radius : 0,
      topRight: this.side === 'left' || this.side === 'bottom' ? radius : 0,
      bottomLeft: this.side === 'right' || this.side === 'top' ? radius : 0,
      bottomRight: this.side === 'left' || this.side === 'top' ? radius : 0,
    }
  }
  edge(): TransitionEdge {
    return this.side === 'left' ? TransitionEdge.START : this.side === 'right' ? TransitionEdge.END :
      this.side === 'top' ? TransitionEdge.TOP : TransitionEdge.BOTTOM
  }
  sheetTransition(): TransitionEffect {
    return TransitionEffect.move(this.edge()).animation({
      duration: this.tokens.metric('motion.duration'),
      curve: curves.cubicBezierCurve(this.tokens.metric('motion.easing.x1'), this.tokens.metric('motion.easing.y1'),
        this.tokens.metric('motion.easing.x2'), this.tokens.metric('motion.easing.y2')),
    })
  }
  dismissSwipe(event: GestureEvent): void {
    const x = Math.cos(event.angle * Math.PI / 180)
    const y = Math.sin(event.angle * Math.PI / 180)
    if ((this.side === 'left' && x < 0) || (this.side === 'right' && x > 0) ||
      (this.side === 'top' && y < 0) || (this.side === 'bottom' && y > 0)) this.drawer.close()
  }
  scrollGesture(event: BaseGestureEvent): GestureJudgeResult {
    if (this.horizontal()) {
      const movement = (event as PanGestureEvent).offsetY
      // Hand an outward edge gesture to the sheet; all other movement is
      // the native scroll view's, including scrolling a long panel's body.
      if ((this.side === 'top' && movement < 0 && this.scroller.isAtEnd()) ||
        (this.side === 'bottom' && movement > 0 && this.scroller.currentOffset().yOffset <= 0)) return GestureJudgeResult.REJECT
    }
    return GestureJudgeResult.CONTINUE
  }
  build() {
    Column() {
      if (this.horizontal()) {
        Row().width(this.tokens.metric('spacing.9')).height(this.tokens.metric('spacing.1'))
          .borderRadius(this.tokens.metric('radius.full')).backgroundColor(this.tokens.color('border-strong'))
          .margin({ top: this.tokens.metric('spacing.2'), bottom: this.tokens.metric('spacing.2') })
          .accessibilityLevel('no')
      }
      if (this.title || this.description) { DazzDialogHeader({ title: this.title, subtitle: this.description }) }
      Scroll(this.scroller) {
        DazzDialogBody({ content: this.forwardContent })
          .onAreaChange((_old: Area, area: Area) => { this.bodyHeight = Number(area.height) })
      }
        .width('100%').layoutWeight(1).scrollBar(BarState.Auto)
        .align(Alignment.TopStart).enableScrollInteraction(this.bodyHeight > this.viewportHeight)
        .onAreaChange((_old: Area, area: Area) => { this.viewportHeight = Number(area.height) })
        .onGestureRecognizerJudgeBegin((event: BaseGestureEvent, current: GestureRecognizer, _others: GestureRecognizer[]) =>
          current.getType() === GestureControl.GestureType.PAN_GESTURE ? this.scrollGesture(event) : GestureJudgeResult.CONTINUE)
      if (this.footerVisible) { DazzDialogFooter({ contentAlign: FlexAlign.Start, content: this.forwardFooter }) }
    }.width(this.horizontal() ? '100%' : (this.extent ?? this.tokens.metric('drawer.size')))
      .height(this.horizontal() ? (this.extent ?? this.tokens.metric('drawer.size')) : '100%')
      .constraintSize({ maxWidth: '100%', maxHeight: '100%' })
      .backgroundColor(this.tokens.color('surface-overlay'))
      .border({ width: {
        left: this.side === 'right' ? this.tokens.metric('stroke.hairline') : 0,
        right: this.side === 'left' ? this.tokens.metric('stroke.hairline') : 0,
        top: this.side === 'bottom' ? this.tokens.metric('stroke.hairline') : 0,
        bottom: this.side === 'top' ? this.tokens.metric('stroke.hairline') : 0,
      }, color: this.tokens.color('border-strong'), radius: this.corner() })
      .clip(true).accessibilityRole(AccessibilityRoleType.DIALOG)
      .accessibilityText(this.title).accessibilityDescription(this.description)
      .id('dazz-drawer-surface').transition(this.sheetTransition())
      .gesture(SwipeGesture({ direction: this.horizontal() ? SwipeDirection.Vertical : SwipeDirection.Horizontal })
        .onAction((event: GestureEvent) => this.dismissSwipe(event)))
  }
  @LocalBuilder forwardContent() { this.content() }
  @LocalBuilder forwardFooter() { if (this.footer) { this.footer() } }
}

@Component
export struct DazzDrawer {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop side: DazzDrawerSide = 'right'
  @Prop title: string = ''
  @Prop description: string = ''
  @Prop extent: Length | undefined = undefined
  @BuilderParam trigger: () => void = this.defaultTrigger
  @BuilderParam content: () => void = this.empty
  @BuilderParam footer?: () => void
  private controller?: CustomDialogController
  private opened: boolean = false
  private triggerKey: string = 'dazz-drawer-trigger-' + String(++drawerID)
  @LocalBuilder empty() {}
  @LocalBuilder defaultTrigger() { DazzButton({ label: 'Open drawer' }) }
  open(): void {
    if (this.opened) return
    // Give the native modal a concrete focus target to restore on dismissal.
    // A caller may supply a trigger without a focusable native descendant.
    try { this.getUIContext().getFocusController().requestFocus(this.triggerKey) } catch (_error) {}
    this.opened = true
    this.controller = new CustomDialogController({
      builder: DazzDrawerSurface({ side: this.side, title: this.title, description: this.description,
        extent: this.extent, content: this.forwardContent, footer: this.forwardFooter, footerVisible: this.footer !== undefined }),
      customStyle: true, autoCancel: true, maskColor: this.tokens.scrim(),
      alignment: this.side === 'left' ? DialogAlignment.CenterStart : this.side === 'right' ? DialogAlignment.CenterEnd :
        this.side === 'top' ? DialogAlignment.Top : DialogAlignment.Bottom,
      offset: { dx: 0, dy: 0 },
      openAnimation: { duration: 0 }, closeAnimation: { duration: 0 },
      onDidDisappear: () => { this.opened = false },
    })
    this.controller.open()
  }
  aboutToDisappear(): void { this.controller?.close() }
  build() { DazzActivationBoundary({ focusKey: this.triggerKey, content: this.forwardTrigger, onAction: () => this.open() }) }
  @LocalBuilder forwardTrigger() { this.trigger() }
  @LocalBuilder forwardContent() { this.content() }
  @LocalBuilder forwardFooter() { if (this.footer) { this.footer() } }
}

@Component
export struct DazzDrawerClose {
  @Prop label: string = 'Close'
  @Consume('dazzui.drawer') drawer: DazzDrawerContext = new DazzDrawerContext()
  @BuilderParam content: () => void = this.defaultContent
  @LocalBuilder defaultContent() { DazzButton({ label: this.label }) }
  @LocalBuilder forwardContent() { this.content() }
  build() { DazzActivationBoundary({ content: this.forwardContent, onAction: () => this.drawer.close() }) }
}
