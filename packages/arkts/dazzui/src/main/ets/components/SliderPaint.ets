import { RenderNode, FrameNode, NodeController, DrawContext, UIContext, LengthMetricsUnit, FrameCallback, curves } from '@kit.ArkUI'
import { drawing, common2D } from '@kit.ArkGraphics2D'
import { DazzTokens } from '../foundation/Tokens'
import { DazzTint, DazzShadow } from '../foundation/TokenTypes'
import { DazzSliderSize } from '../foundation/SliderData'

interface SliderPaintData {
  width: number
  height: number
  originX: number
  originY: number
  diameter: number
  track: number
  fractions: number[]
  fill: string
  groove: string
  indicator: string
  edges: string[]
  scales: number[]
  hotScale: number
  edgeWidth: number
  focusIndex: number
  focusWidth: number
  focusOffset: number
  focusColor: string
  shadow?: DazzShadow
  lift: number
  density: number
  duration: number
  easing: number[]
}

function color(value: string): common2D.Color {
  return { alpha: parseInt(value.slice(1, 3), 16), red: parseInt(value.slice(3, 5), 16), green: parseInt(value.slice(5, 7), 16), blue: parseInt(value.slice(7, 9), 16) }
}
function interpolateColor(from: string, to: string, progress: number): string {
  const a = color(from), b = color(to)
  const alpha = a.alpha + (b.alpha - a.alpha) * progress
  const channel = (left: number, right: number): number => alpha > 0 ? Math.round((left * a.alpha * (1 - progress) + right * b.alpha * progress) / alpha) : 0
  return '#' + [Math.round(alpha), channel(a.red, b.red), channel(a.green, b.green), channel(a.blue, b.blue)].map((value: number): string => value.toString(16).padStart(2, '0')).join('').toUpperCase()
}
function outset(data: SliderPaintData): number {
  const shadow = data.shadow
  return Math.ceil((data.diameter * Math.max(1, data.hotScale) / 2 + Math.max(data.focusWidth + data.focusOffset,
    shadow ? 2 * shadow.radius + Math.max(Math.abs(shadow.x), Math.abs(shadow.y)) : 0)) * data.density) / data.density
}

/** Full-track coordinates avoid native frame rounding at each circle/edge. */
class SliderRender extends RenderNode {
  data?: SliderPaintData
  scales: number[] = []
  edges: string[] = []
  constructor() { super(); this.lengthMetricsUnit = LengthMetricsUnit.PX; this.clipToFrame = false }
  brush(value: string): drawing.Brush { const brush = new drawing.Brush(); brush.setAntiAlias(true); brush.setColor(color(value)); return brush }
  roundRect(canvas: drawing.Canvas, left: number, top: number, right: number, bottom: number, radius: number, ink: string): void {
    canvas.attachBrush(this.brush(ink)); canvas.drawRoundRect(new drawing.RoundRect({ left, top, right, bottom }, radius, radius)); canvas.detachBrush()
  }
  circle(canvas: drawing.Canvas, x: number, y: number, radius: number, ink: string): void {
    this.roundRect(canvas, x - radius, y - radius, x + radius, y + radius, radius, ink)
  }
  ring(canvas: drawing.Canvas, x: number, y: number, radius: number, width: number, ink: string): void {
    if (width <= 0) return
    const inner = Math.max(0, radius - width)
    canvas.attachBrush(this.brush(ink))
    canvas.drawNestedRoundRect(new drawing.RoundRect({ left: x - radius, top: y - radius, right: x + radius, bottom: y + radius }, radius, radius),
      new drawing.RoundRect({ left: x - inner, top: y - inner, right: x + inner, bottom: y + inner }, inner, inner))
    canvas.detachBrush()
  }
  shadow(canvas: drawing.Canvas, data: SliderPaintData, x: number, y: number, radius: number): void {
    const shadow = data.shadow
    if (!shadow || data.lift <= 0) return
    const shade = color(shadow.color), layer = new drawing.Brush()
    layer.setAlpha(Math.round(shade.alpha * data.lift))
    canvas.saveLayer({ left: 0, top: 0, right: (data.width + 2 * outset(data)) * data.density, bottom: (data.height + 2 * outset(data)) * data.density }, layer)
    const brush = this.brush('#FF' + shadow.color.slice(3))
    const sigma = shadow.radius * data.density / 2
    if (sigma > 0) brush.setImageFilter(drawing.ImageFilter.createBlurImageFilter(sigma, sigma, drawing.TileMode.DECAL))
    canvas.attachBrush(brush); canvas.drawCircle(x + shadow.x * data.density, y + shadow.y * data.density, radius); canvas.detachBrush()
    const clear = new drawing.Brush(); clear.setAntiAlias(true); clear.setBlendMode(drawing.BlendMode.CLEAR)
    canvas.attachBrush(clear); canvas.drawCircle(x, y, radius); canvas.detachBrush(); canvas.restore()
  }
  draw(context: DrawContext): void {
    const data = this.data
    if (!data || data.width <= 0) return
    const canvas = context.canvas, d = data.density, origin = outset(data) * d
    // The reference snaps CSS backgrounds before applying the thumb's -50%
    // translate. Preserve that window-relative phase in one drawing scope.
    const layout = (value: number): number => Math.floor(value * 64) / 64
    const rootX = layout(data.originX), rootY = layout(data.originY)
    const px = (value: number): number => origin + (Math.round(value) - data.originX) * d
    const py = (value: number): number => origin + (Math.round(value) - data.originY) * d
    const cy = py(rootY + data.height / 2), track = data.track * d
    const top = py(rootY + (data.height - data.track) / 2), bottom = py(rootY + (data.height + data.track) / 2)
    this.roundRect(canvas, px(rootX), top, px(rootX + data.width), bottom, track / 2, data.groove)
    const start = data.fractions.length > 1 ? data.fractions[0] : 0
    const end = data.fractions[data.fractions.length - 1] ?? 0
    if (end > start) this.roundRect(canvas, px(rootX + layout(start * data.width)), top, px(rootX + layout(end * data.width)), bottom, track / 2, data.indicator)
    data.fractions.forEach((fraction: number, index: number) => {
      const scale = this.scales[index] ?? 1, x = px(rootX + layout(fraction * data.width))
      // CSS scales the complete thumb, including its shadow and outline.
      canvas.save(); canvas.translate(x, cy); canvas.scale(scale, scale); canvas.translate(-x, -cy)
      const baseRadius = data.diameter * d / 2
      this.shadow(canvas, data, x, cy, baseRadius)
      this.circle(canvas, x, cy, baseRadius, data.fill)
      this.ring(canvas, x, cy, baseRadius, data.edgeWidth * d, this.edges[index] ?? data.edges[index])
      if (index === data.focusIndex) this.ring(canvas, x, cy, baseRadius + (data.focusOffset + data.focusWidth) * d, data.focusWidth * d, data.focusColor)
      canvas.restore()
    })
  }
}

class SliderFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  onIdle(_timeLeft: number): void { this.action() }
}

class SliderPaintController extends NodeController {
  private root?: FrameNode
  private render: SliderRender = new SliderRender()
  private ui?: UIContext
  private dead: boolean = false
  private pending: boolean = false
  private started: number = 0
  private fromScales: number[] = []
  private fromEdges: string[] = []
  private targetKey: string = ''
  update(data: SliderPaintData, ui: UIContext): void {
    this.ui = ui
    this.tick()
    const key = data.scales.join(',') + ':' + data.edges.join(',')
    if (!this.render.data || data.fractions.length !== this.render.scales.length) {
      this.render.scales = data.scales.slice(); this.render.edges = data.edges.slice(); this.started = 0
    } else if (key !== this.targetKey) {
      this.fromScales = this.render.scales.slice(); this.fromEdges = this.render.edges.slice(); this.started = Date.now()
    }
    this.targetKey = key; this.render.data = data
    const padding = 2 * outset(data), width = data.width + padding, height = data.height + padding
    this.render.frame = { x: 0, y: 0, width: width * data.density, height: height * data.density }
    this.root?.commonAttribute.width(width).height(height)
    this.render.invalidate(); this.schedule()
  }
  tick(): void {
    const data = this.render.data
    if (!data || !this.started) return
    const fraction = Math.min(1, Math.max(0, (Date.now() - this.started) / Math.max(1, data.duration)))
    const easing = data.easing
    const progress = curves.cubicBezierCurve(easing[0], easing[1], easing[2], easing[3]).interpolate(fraction)
    this.render.scales = data.scales.map((value: number, index: number): number => this.fromScales[index] + (value - this.fromScales[index]) * progress)
    this.render.edges = data.edges.map((value: string, index: number): string => interpolateColor(this.fromEdges[index], value, progress))
    if (fraction === 1) this.started = 0
    this.render.invalidate()
  }
  schedule(): void {
    if (!this.ui || !this.started || this.pending || this.dead) return
    this.pending = true
    this.ui.postFrameCallback(new SliderFrame(() => { this.pending = false; if (this.dead) return; this.tick(); this.schedule() }))
  }
  makeNode(ui: UIContext): FrameNode {
    if (!this.root) {
      this.root = new FrameNode(ui)
      const data = this.render.data
      if (data) this.root.commonAttribute.width(data.width + 2 * outset(data)).height(data.height + 2 * outset(data))
      this.root.commonAttribute.hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
      const node = this.root.getRenderNode()
      if (node) { node.clipToFrame = false; node.appendChild(this.render) }
    }
    return this.root
  }
  dispose(): void { this.dead = true; this.render.dispose(); this.root?.dispose(); this.root = undefined }
}

@Component
export struct DazzSliderPaint {
  @Consume('dazzui.theme') @Watch('sync') tokens: DazzTokens
  @Prop @Watch('sync') contentWidth: number = 0
  @Prop @Watch('sync') contentHeight: number = 0
  @Prop anchorID: string = ''
  @Prop @Watch('sync') fractions: number[] = []
  @Prop @Watch('sync') controlSize: DazzSliderSize = 'medium'
  @Prop @Watch('sync') tint: DazzTint = 'primary'
  @Prop @Watch('sync') disabled: boolean = false
  @Prop @Watch('sync') hotIndex: number = -1
  @Prop @Watch('sync') focusIndex: number = -1
  private controller: SliderPaintController = new SliderPaintController()
  private modelKey: string = ''
  private layoutChanged: () => void = () => this.sync()
  data(): SliderPaintData {
    const ink = this.tokens.metric('slider.thumb.ink') === 1
    const position = this.getUIContext().getAttachedFrameNodeById(this.anchorID)?.getPositionToWindow()
    return { width: this.contentWidth, height: this.contentHeight, originX: position?.x ?? 0, originY: position?.y ?? 0, fractions: this.fractions.slice(),
      diameter: this.tokens.metric('slider.' + this.controlSize + '.thumb'), track: this.tokens.metric('slider.' + this.controlSize + '.track'),
      fill: this.disabled ? this.tokens.paint('normal', 'surface', 'disabled') : this.tokens.color(ink ? 'content' : 'surface'),
      groove: this.tokens.color('surface-sunken'), indicator: this.disabled ? this.tokens.paint('normal', 'content', 'disabled') : this.tokens.color(this.tint + '.600'),
      edges: this.fractions.map((_value: number, index: number): string => this.disabled ? this.tokens.color('border') : ink ? this.tokens.color('surface') : this.tokens.color(this.hotIndex === index ? this.tint + '.600' : 'border-strong')),
      scales: this.fractions.map((_value: number, index: number): number => !this.disabled && this.hotIndex === index ? this.tokens.metric('slider.thumb.hot-scale') : 1),
      hotScale: this.tokens.metric('slider.thumb.hot-scale'),
      edgeWidth: this.tokens.metric('slider.thumb.edge-width'), focusIndex: this.disabled ? -1 : this.focusIndex,
      focusWidth: this.tokens.metric('focus.width'), focusOffset: this.tokens.metric('focus.offset'), focusColor: this.tokens.focus(),
      shadow: this.tokens.shadow('shadow.xs')[0], lift: this.disabled ? 0 : this.tokens.metric('slider.thumb.lift'), density: this.getUIContext().vp2px(1),
      duration: this.tokens.metric('motion.duration'), easing: ['x1', 'y1', 'x2', 'y2'].map((axis: string): number => this.tokens.metric('motion.easing.' + axis)) }
  }
  sync(): void { const data = this.data(), key = JSON.stringify(data); if (key === this.modelKey) return; this.modelKey = key; this.controller.update(data, this.getUIContext()) }
  aboutToAppear(): void { this.sync(); this.getUIContext().getUIObserver().on('didLayout', this.layoutChanged) }
  aboutToDisappear(): void { this.getUIContext().getUIObserver().off('didLayout', this.layoutChanged); this.controller.dispose() }
  build() {
    Stack() {
      NodeContainer(this.controller).width(this.contentWidth + 2 * outset(this.data())).height(this.contentHeight + 2 * outset(this.data()))
        .position({ x: -outset(this.data()), y: -outset(this.data()) }).clip(false)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
    }.width(this.contentWidth).height(this.contentHeight).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
  }
}
