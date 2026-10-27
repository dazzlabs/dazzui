import { common } from '@kit.AbilityKit'
import { FrameCallback, ColorMetrics, LengthMetrics } from '@kit.ArkUI'
import { text } from '@kit.ArkGraphics2D'
import { DazzTokens, DazzPreviewCard, DazzPreviewAnchor, DazzTint, DazzWindowTitleStyle } from '@dazzlabs/dazzui'
import { SceneValue } from './SceneTypes'
import { SceneState, booleanValue } from './SceneState'
import { InlineRun, inlineRuns } from './SceneInlineData'

interface InlinePosition { run: number; x: number; y: number; width: number; height: number }
class InlineFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  onIdle(_timeLeft: number): void { this.action() }
}

/** Native spans retain wrapping, selection and copy; overlays only anchor popups. */
@Component
export struct SceneInline {
  @Consume('dazzui.theme') @Watch('queueGeometry') tokens: DazzTokens
  @Consume('dazzui.prose') @Watch('queueGeometry') prose: string = 'bodyMedium'
  @Prop @Watch('queueGeometry') node: SceneValue
  @Prop sceneId: string = ''
  @Consume('dazzui.window-title') @Watch('queueGeometry') titleStyle: DazzWindowTitleStyle = new DazzWindowTitleStyle()
  @Prop inWindowTitle: boolean = false
  state: SceneState = new SceneState()
  @Link @Watch('queueGeometry') revision: number
  @BuilderParam previewContent: (node: SceneValue) => void = this.empty
  onNavigate: (href: string) => void = () => {}
  @State positions: InlinePosition[] = []
  @State hovered: number = -1
  @State focused: number = -1
  private controller: TextController = new TextController()
  private mounted: boolean = false
  private epoch: number = 0
  private anchors: Map<number, DazzPreviewAnchor> = new Map()
  private cachedRevision: number = -1
  private cachedNode: SceneValue | undefined = undefined
  private cachedState: SceneState | undefined = undefined
  private cachedRuns: InlineRun[] = []
  @LocalBuilder empty(_node: SceneValue) {}
  title(): DazzWindowTitleStyle | undefined { return this.inWindowTitle && this.titleStyle.active ? this.titleStyle : undefined }
  ink(): string {
    return this.node.fields?.['tint']?.text ? this.tokens.color(this.node.fields['tint'].text + '.500') :
      this.node.fields?.['tone']?.text === 'secondary' ? this.tokens.color('content-secondary') : this.title()?.color ?? this.tokens.color('content')
  }
  aboutToAppear(): void { this.mounted = true; this.queueGeometry() }
  aboutToDisappear(): void { this.mounted = false; this.epoch++ }
  runs(): InlineRun[] {
    const revision = this.revision
    const node = this.node
    if (revision !== this.cachedRevision || node !== this.cachedNode || this.state !== this.cachedState) {
      this.cachedRuns = inlineRuns(node, this.state)
      this.cachedRevision = revision
      this.cachedNode = node
      this.cachedState = this.state
    }
    return this.cachedRuns
  }
  runAt(index: number): InlineRun {
    return this.runs()[index] ?? { label: '', code: false, href: '', start: 0, end: 0 }
  }
  anchor(index: number): DazzPreviewAnchor {
    const key = this.runAt(index).preview?.address ?? index
    let anchor = this.anchors.get(key)
    if (!anchor) { anchor = new DazzPreviewAnchor(); this.anchors.set(key, anchor) }
    return anchor
  }
  p(index: number, key: string): SceneValue {
    if (this.revision < 0) return { kind: 'null' }
    return this.state.evaluate(this.runAt(index).preview?.fields?.[key] ?? { kind: 'null' })
  }
  previewWidth(index: number): number | undefined {
    const value = this.p(index, 'width')
    if (value.kind === 'number') return value.number
    if (value.text?.endsWith('rem')) return parseFloat(value.text) * 16
    if (value.text?.endsWith('px')) return parseFloat(value.text)
    return undefined
  }
  change(index: number, open: boolean): void {
    const action = this.runAt(index).preview?.fields?.['onOpenChange']
    if (action) { this.state.run(action, booleanValue(open)); this.revision++ }
  }
  linkValue(index: number, key: string): SceneValue {
    if (this.revision < 0) return { kind: 'null' }
    return this.state.evaluate(this.runAt(index).link?.fields?.[key] ?? { kind: 'null' })
  }
  underline(index: number): boolean {
    const value = this.linkValue(index, 'underline').text ?? 'hover'
    return !!this.runAt(index).link && (value === 'always' || (value === 'hover' && this.hovered === index))
  }
  queueGeometry(): void {
    if (!this.mounted || !this.runs().some((run: InlineRun): boolean => !!run.preview)) return
    const epoch = ++this.epoch
    this.getUIContext().postFrameCallback(new InlineFrame(() => {
      if (this.mounted && epoch === this.epoch) this.measureAnchors()
    }))
  }
  measureAnchors(): void {
    const layout = this.controller.getLayoutManager()
    const next: InlinePosition[] = []
    this.runs().forEach((run: InlineRun, index: number) => {
      if (!run.preview || run.end === run.start) return
      const boxes = layout.getRectsForRange({ start: run.start, end: run.end }, text.RectWidthStyle.TIGHT, text.RectHeightStyle.TIGHT)
      if (!boxes.length) return
      const left = Math.min(...boxes.map((box: TextBox): number => box.rect.left))
      const top = Math.min(...boxes.map((box: TextBox): number => box.rect.top))
      const right = Math.max(...boxes.map((box: TextBox): number => box.rect.right))
      const bottom = Math.max(...boxes.map((box: TextBox): number => box.rect.bottom))
      const ui = this.getUIContext()
      next.push({ run: index, x: ui.px2vp(left), y: ui.px2vp(top),
        width: ui.px2vp(right - left), height: ui.px2vp(bottom - top) })
    })
    this.positions = next
  }
  inlinePosition(index: number): InlinePosition {
    return this.positions[index] ?? { run: 0, x: 0, y: 0, width: 0, height: 0 }
  }
  async navigate(href: string): Promise<void> {
    if (!href) return
    if (href.startsWith('#/')) { this.onNavigate(href); return }
    const context = this.getUIContext().getHostContext() as common.UIAbilityContext
    try { await context.startAbility({ action: 'ohos.want.action.viewData', uri: href }) } catch (_) {}
  }
  @LocalBuilder focusAnchor(index: number) {
    // Layout containers are focus scopes in ArkUI. An empty Text supplies a
    // native focus node while the paragraph's Span still owns pointer input.
    Text('').width('100%').height('100%').focusable(true)
      .id(this.sceneId + '/' + String(this.runAt(index).preview?.fields?.['trigger']?.address))
      .accessibilityRole(AccessibilityRoleType.HYPERLINK).accessibilityText(this.runAt(index).label)
      .focusBox({ margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
        strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')),
        strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
      .onFocus(() => {
        this.focused = this.getUIContext().getFocusController().isActive() ? index : -1
        this.anchor(index).focus(this.focused === index)
      })
      .onBlur(() => { this.focused = -1; this.anchor(index).focus(false) })
      .onKeyEvent((event: KeyEvent): boolean => {
        if (event.type === KeyType.Down && event.keyCode === 2054) {
          this.navigate(this.runAt(index).href)
          event.stopPropagation(); return true
        }
        return false
      })
  }
  @LocalBuilder paragraph() {
    Text(undefined, { controller: this.controller }) {
      ForEach(this.runs().map((_run: InlineRun, index: number) => index), (index: number) => {
        Span(this.runAt(index).label).fontFamily(this.runAt(index).code ? 'monospace' : undefined)
          .fontSize(this.tokens.fontSize(this.runAt(index).code ? this.tokens.type('captionSmall').size : this.title()?.size ?? this.tokens.type(this.prose).size))
          .fontColor(this.runAt(index).code ? this.tokens.color('content') : this.runAt(index).link ? this.tokens.paint('plain', 'content', this.hovered === index ? 'hovered' : 'normal', (this.linkValue(index, 'tint').text ?? 'primary') as DazzTint) : this.ink())
          .textBackgroundStyle({ color: this.runAt(index).code ? this.tokens.color('surface-sunken') : Color.Transparent, radius: this.tokens.metric('radius.tiny') })
          .decoration({ type: this.underline(index) ? TextDecorationType.Underline : TextDecorationType.None })
          .onHover((entered: boolean) => {
            this.hovered = entered ? index : -1
            if (this.runAt(index).preview) this.anchor(index).hover(entered)
          })
          .onClick(() => {
            this.navigate(this.runAt(index).href)
          })
      }, (index: number) => String(index))
    }.fontSize(this.tokens.fontSize(this.title()?.size ?? this.tokens.type(this.prose).size))
      .lineHeight(this.tokens.fontSize(this.title()?.leading ?? this.tokens.type(this.prose).leading))
      .fontWeight(this.node.fields?.['strong']?.boolean ? this.tokens.type('labelStrong').weight : this.title()?.weight ?? this.tokens.type(this.prose).weight)
      .fontColor(this.ink())
      .maxLines(this.title() ? 1 : undefined).textOverflow({ overflow: this.title() ? TextOverflow.Ellipsis : TextOverflow.None })
      .copyOption(this.title() ? CopyOptions.None : CopyOptions.LocalDevice)
      .id(this.title() ? 'dazz-window-title-text-' + String(this.getUniqueId()) : this.sceneId + '/' + String(this.node.address))
      .onAreaChange(() => this.queueGeometry())
  }
  build() {
    if (this.runs().some((run: InlineRun): boolean => !!run.preview)) {
      Stack({ alignContent: Alignment.TopStart }) {
        this.paragraph()
        ForEach(this.positions.map((_position: InlinePosition, index: number) => index), (index: number) => {
          DazzPreviewCard({ anchor: this.anchor(this.inlinePosition(index).run),
            open: this.p(this.inlinePosition(index).run, 'open').boolean,
            defaultOpen: this.p(this.inlinePosition(index).run, 'defaultOpen').boolean ?? false,
            delay: this.p(this.inlinePosition(index).run, 'delay').number,
            closeDelay: this.p(this.inlinePosition(index).run, 'closeDelay').number,
            side: this.p(this.inlinePosition(index).run, 'side').text ?? 'bottom',
            contentAlign: this.p(this.inlinePosition(index).run, 'align').text ?? 'center',
            contentWidth: this.previewWidth(this.inlinePosition(index).run),
            onOpenChange: (open: boolean) => this.change(this.inlinePosition(index).run, open),
            trigger: () => { this.focusAnchor(this.inlinePosition(index).run) },
            content: () => { this.previewContent(this.runAt(this.inlinePosition(index).run).preview ?? { kind: 'null' }) },
          }).width(this.inlinePosition(index).width).height(this.inlinePosition(index).height)
            .position({ x: this.inlinePosition(index).x, y: this.inlinePosition(index).y })
            .id(this.sceneId + '/' + String(this.runAt(this.inlinePosition(index).run).preview?.address))
            .hitTestBehavior(HitTestMode.None)
        }, (index: number) => String(index))
      }.width(LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent)
    } else { this.paragraph() }
  }
}
