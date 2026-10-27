import { DazzTokens } from '../foundation/Tokens'
import { DazzShadowLayer } from './ShadowLayer'
import { FrameNode } from '@kit.ArkUI'

export class DazzMenuViewport {
  owner: number = -1
  scrollOffset: number = 0
  restoreOffset: number | undefined = undefined
  reveal: (node: FrameNode | null) => void = () => {}
}

/** Native Menu owns measurement and input; its sibling draws the ds shadow. */
@Component
export struct DazzMenuShadow {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop anchorID: string = ''
  @State paintWidth: number = 0
  @State paintHeight: number = 0
  @State viewportLimit: number | undefined = undefined
  @BuilderParam content: () => void = this.empty
  viewport: DazzMenuViewport = new DazzMenuViewport()
  onMenuKey: (event: KeyEvent) => boolean = () => false
  private scroller: Scroller = new Scroller()
  private observing: boolean = false
  private geometryKey: string = ''
  private pendingOffset: number | undefined = undefined
  private layoutChanged: () => void = () => this.updateViewport()
  @LocalBuilder empty() {}
  observe(): void {
    if (this.observing) return
    this.observing = true
    this.pendingOffset = this.viewport.restoreOffset
    this.viewport.restoreOffset = undefined
    this.viewport.owner = this.getUniqueId()
    this.viewport.reveal = (node: FrameNode | null) => this.reveal(node)
    this.getUIContext().getUIObserver().on('didLayout', this.layoutChanged)
  }
  unobserve(): void {
    if (!this.observing) return
    this.observing = false
    if (this.viewport.owner === this.getUniqueId()) {
      this.viewport.owner = -1
      this.viewport.reveal = () => {}
    }
    this.getUIContext().getUIObserver().off('didLayout', this.layoutChanged)
  }
  aboutToDisappear(): void { this.unobserve() }
  reveal(node: FrameNode | null): void {
    if (!node || !this.observing) return
    const ui = this.getUIContext()
    const viewport = ui.getFrameNodeByUniqueId(this.getUniqueId())?.getChild(0)
    if (!viewport || viewport.getNodeType() !== 'Scroll') return
    const item = node.getPositionToScreen()
    const origin = viewport.getPositionToScreen()
    const height = ui.px2vp(node.getMeasuredSize().height)
    const visibleHeight = ui.px2vp(viewport.getMeasuredSize().height)
    const inset = this.tokens.metric('spacing.1') + this.tokens.metric('stroke.hairline')
    const delta = item.y < origin.y + inset ? item.y - origin.y - inset :
      item.y + height > origin.y + visibleHeight - inset ? item.y + height - origin.y - visibleHeight + inset : 0
    if (Math.abs(delta) > 0.5) this.scroller.scrollTo({ xOffset: 0,
      yOffset: Math.max(0, this.scroller.currentOffset().yOffset + delta), animation: false })
  }
  measureSurface(area: Area): void {
    const density = this.getUIContext().vp2px(1)
    // Measure the foreground, never the Stack that also contains this shadow:
    // a previous canvas size must not hold an overflowing viewport open.
    this.paintWidth = Math.round(Number(area.width) * density) / density
    this.paintHeight = Math.round(Number(area.height) * density) / density
    this.updateViewport()
  }
  updateViewport(): void {
    const ui = this.getUIContext()
    // Unique IDs work for both the owning window and a native menu subwindow.
    const node = ui.getFrameNodeByUniqueId(this.getUniqueId())
    if (!node) return
    let parent = node.getParent()
    let viewportHeight: number = 0
    let windowSize = node.getMeasuredSize()
    while (parent) {
      if (!viewportHeight && parent.getNodeType() === 'Scroll') viewportHeight = parent.getMeasuredSize().height
      windowSize = parent.getMeasuredSize()
      parent = parent.getParent()
    }
    const anchor = ui.getAttachedFrameNodeById(this.anchorID)
    const position = anchor?.getPositionToScreen()
    const size = anchor?.getMeasuredSize()
    const key = [windowSize.width, windowSize.height, position?.x, position?.y, size?.width, size?.height].join(':')
    if (key !== this.geometryKey) { this.geometryKey = key; this.viewportLimit = undefined }
    if (viewportHeight > 0 && node.getMeasuredSize().height > viewportHeight + 0.5) {
      const density = ui.vp2px(1)
      const next = viewportHeight / density
      if (this.viewportLimit === undefined || Math.abs(next - this.viewportLimit) > 0.5 / density) this.viewportLimit = next
    } else if (viewportHeight > 0 && this.pendingOffset !== undefined) {
      const offset = this.pendingOffset
      this.pendingOffset = undefined
      this.scroller.scrollTo({ xOffset: 0, yOffset: offset, animation: false })
    }
  }
  build() {
    Stack({ alignContent: Alignment.TopStart }) {
      Scroll(this.scroller) {
        Stack({ alignContent: Alignment.TopStart }) {
          // The actual Menu and its background are real siblings. The native
          // desktop host leaves this composite's outer shadow unclipped.
          this.content()
          Column() {}.width(LayoutPolicy.matchParent).height(LayoutPolicy.matchParent)
            .backgroundColor(this.tokens.color('surface-overlay')).position({ x: 0, y: 0 }).zIndex(0)
            .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
        }.width(LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent)
      }.width(LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent)
        .constraintSize({ maxHeight: this.viewportLimit }).scrollBar(BarState.Auto)
        .borderRadius(this.tokens.metric('radius.large')).clip(true)
        .onAreaChange((_old: Area, area: Area) => this.measureSurface(area))
        .onDidScroll(() => { this.viewport.scrollOffset = this.scroller.currentOffset().yOffset })
      DazzShadowLayer({ paintWidth: this.paintWidth, paintHeight: this.paintHeight,
        corner: this.tokens.metric('radius.large'), shadowLayers: this.tokens.shadow('shadow.lg') })
        .position({ x: 0, y: 0 }).zIndex(0)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
      // Paint the viewport border independently of the scrolling content.
      Column() {}.width(LayoutPolicy.matchParent).height(LayoutPolicy.matchParent)
        .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color('border-strong'),
          radius: this.tokens.metric('radius.large') }).position({ x: 0, y: 0 }).zIndex(2)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
    }.width(LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent).clip(false)
      .onAppear(() => this.observe()).onDisAppear(() => this.unobserve())
      .onKeyEvent((event: KeyEvent): boolean => this.onMenuKey(event))
      .onAreaChange(() => this.updateViewport())
  }
}
