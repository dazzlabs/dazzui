import { RenderNode, FrameNode, NodeController, DrawContext, UIContext, LengthMetricsUnit } from '@kit.ArkUI'
import { drawing } from '@kit.ArkGraphics2D'
import { image } from '@kit.ImageKit'
import { DazzTokens } from '../foundation/Tokens'

interface ButtonPaintData {
  width: number
  height: number
  x: number
  y: number
  density: number
  corners: number[]
  thickness: number
  offset: number
  color: string
  surface: string
  edge: string
  edgeWidth: number
  hasFocus: boolean
}

function padding(data: ButtonPaintData): number {
  // Whole physical pixels preserve the anchor's phase when the canvas grows.
  return Math.ceil((Math.max(0, data.offset) + data.thickness + 2) * data.density) / data.density
}

/** CSS solid outlines fill two nested, CSS-pixel-snapped rounded rectangles.
 * See Blink's paint/contoured_border_geometry.cc and box_border_painter.cc:
 * https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/core/paint/contoured_border_geometry.cc
 */
class ButtonPaintRender extends RenderNode {
  data?: ButtonPaintData
  private bitmap?: image.PixelMap
  private raster?: drawing.Canvas
  clearRaster(): void { this.raster = undefined; this.bitmap?.release(() => {}); this.bitmap = undefined }
  constructor() { super(); this.lengthMetricsUnit = LengthMetricsUnit.PX; this.clipToFrame = false }
  draw(context: DrawContext): void {
    const data = this.data
    if (!data || data.width <= 0 || data.height <= 0) return
    if (!data.hasFocus && data.surface.startsWith('#00') && data.edge.startsWith('#00')) return
    if (!this.bitmap) {
      const width = Math.ceil((data.width + 2 * padding(data)) * data.density)
      const height = Math.ceil((data.height + 2 * padding(data)) * data.density)
      this.bitmap = image.createPixelMapSync(new ArrayBuffer(width * height * 4), {
        size: { width, height }, pixelFormat: image.PixelMapFormat.RGBA_8888, alphaType: image.AlphaType.PREMUL, editable: true })
      this.raster = new drawing.Canvas(this.bitmap)
      this.paint(this.raster, data)
    }
    context.canvas.drawImage(this.bitmap, 0, 0, new drawing.SamplingOptions())
  }
  private paint(canvas: drawing.Canvas, data: ButtonPaintData): void {
    const d = data.density, origin = padding(data) * d
    // Match the reference's LayoutUnit grid before its integer CSS-pixel snap.
    const layout = (value: number): number => Math.trunc(value * 64) / 64
    const x = layout(data.x), y = layout(data.y), width = layout(data.width), height = layout(data.height)
    const baseWidth = Math.max(0, Math.round(x + width) - Math.round(x))
    const baseHeight = Math.max(0, Math.round(y + height) - Math.round(y))
    let scale: number = 1
    const corners = data.corners.slice(), sums = [corners[0] + corners[1], corners[1] + corners[2], corners[2] + corners[3], corners[3] + corners[0]]
    sums.forEach((sum: number, index: number) => { if (sum > 0) scale = Math.min(scale, (index % 2 === 0 ? baseWidth : baseHeight) / sum) })
    const positions: drawing.CornerPos[] = [drawing.CornerPos.TOP_LEFT_POS, drawing.CornerPos.TOP_RIGHT_POS,
      drawing.CornerPos.BOTTOM_RIGHT_POS, drawing.CornerPos.BOTTOM_LEFT_POS]
    const rect = (outset: number): drawing.RoundRect => {
      const left = origin + (Math.round(x - outset) - data.x) * d
      const top = origin + (Math.round(y - outset) - data.y) * d
      const right = origin + (Math.round(x + width + outset) - data.x) * d
      const bottom = origin + (Math.round(y + height + outset) - data.y) * d
      if (corners.every((value: number): boolean => value === corners[0])) {
        const radius = corners[0] > 0 ? Math.max(0, corners[0] * scale + outset) * d : 0
        return new drawing.RoundRect({ left, top, right, bottom }, radius, radius)
      }
      const box = new drawing.RoundRect({ left, top, right, bottom }, 0, 0)
      positions.forEach((position: drawing.CornerPos, index: number) => {
        const radius = corners[index] > 0 ? Math.max(0, corners[index] * scale + outset) * d : 0
        box.setCorner(position, radius, radius)
      })
      return box
    }
    const brushFor = (ink: string): drawing.Brush => {
      const brush = new drawing.Brush()
      brush.setAntiAlias(true)
      brush.setColor({ alpha: parseInt(ink.slice(1, 3), 16), red: parseInt(ink.slice(3, 5), 16),
        green: parseInt(ink.slice(5, 7), 16), blue: parseInt(ink.slice(7, 9), 16) })
      return brush
    }
    const base = rect(0)
    if (!data.surface.startsWith('#00')) {
      canvas.save()
      // Rounded CSS background fill retains its rounded border-box clip.
      // Square boxes do not need that additional antialiased clip.
      if (corners.some((value: number): boolean => value > 0)) canvas.clipRoundRect(base, drawing.ClipOp.INTERSECT, true)
      canvas.attachBrush(brushFor(data.surface)); canvas.drawRoundRect(base); canvas.detachBrush()
      canvas.restore()
    }
    if (data.edgeWidth > 0 && !data.edge.startsWith('#00')) {
      canvas.attachBrush(brushFor(data.edge)); canvas.drawNestedRoundRect(base, rect(-data.edgeWidth)); canvas.detachBrush()
    }
    if (data.hasFocus && data.thickness > 0) {
      canvas.attachBrush(brushFor(data.color))
      canvas.drawNestedRoundRect(rect(data.offset + data.thickness), rect(data.offset))
      canvas.detachBrush()
    }
  }
}

class ButtonPaintController extends NodeController {
  private root?: FrameNode
  private render: ButtonPaintRender = new ButtonPaintRender()
  update(data: ButtonPaintData): void {
    this.render.clearRaster()
    this.render.data = data
    const width = data.width + 2 * padding(data), height = data.height + 2 * padding(data)
    this.render.frame = { x: 0, y: 0, width: width * data.density, height: height * data.density }
    this.root?.commonAttribute.width(width).height(height)
    this.render.invalidate()
  }
  makeNode(ui: UIContext): FrameNode {
    if (!this.root) {
      this.root = new FrameNode(ui)
      const data = this.render.data
      if (data) this.root.commonAttribute.width(data.width + 2 * padding(data)).height(data.height + 2 * padding(data))
      this.root.commonAttribute.hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
      const node = this.root.getRenderNode()
      if (node) { node.clipToFrame = false; node.appendChild(this.render) }
    }
    return this.root
  }
  dispose(): void { this.render.dispose(); this.render.clearRaster(); this.root?.dispose(); this.root = undefined }
}

/** Passive control paint; layout, input and accessibility stay on the native Button. */
@Component
export struct DazzButtonPaint {
  @Consume('dazzui.theme') @Watch('sync') tokens: DazzTokens
  @Prop @Watch('sync') paintWidth: number = 0
  @Prop @Watch('sync') paintHeight: number = 0
  @Prop @Watch('sync') corners: number[] = [0, 0, 0, 0]
  @Prop anchorID: string = ''
  @Prop @Watch('sync') surfaceColor: string = '#00000000'
  @Prop @Watch('sync') edgeColor: string = '#00000000'
  @Prop @Watch('sync') edgeWidth: number = 0
  @Prop @Watch('sync') hasFocus: boolean = false
  private controller: ButtonPaintController = new ButtonPaintController()
  private modelKey: string = ''
  private dead: boolean = false
  private layoutChanged: () => void = () => { if (!this.dead) this.sync() }
  data(): ButtonPaintData {
    const position = this.getUIContext().getAttachedFrameNodeById(this.anchorID)?.getPositionToWindow()
    return { width: this.paintWidth, height: this.paintHeight, x: position?.x ?? 0, y: position?.y ?? 0,
      density: this.getUIContext().vp2px(1), corners: this.corners.slice(), thickness: this.tokens.metric('focus.width'),
      offset: this.tokens.metric('focus.offset'), color: this.tokens.focus(),
      surface: this.surfaceColor, edge: this.edgeColor, edgeWidth: this.edgeWidth, hasFocus: this.hasFocus }
  }
  sync(): void { if (this.dead) return; const data = this.data(), key = JSON.stringify(data); if (key === this.modelKey) return; this.modelKey = key; this.controller.update(data) }
  aboutToAppear(): void { this.sync(); this.getUIContext().getUIObserver().on('didLayout', this.layoutChanged) }
  aboutToDisappear(): void { this.dead = true; this.getUIContext().getUIObserver().off('didLayout', this.layoutChanged); this.controller.dispose() }
  build() {
    Stack() {
      NodeContainer(this.controller).width(this.paintWidth + 2 * padding(this.data())).height(this.paintHeight + 2 * padding(this.data()))
        .position({ x: -padding(this.data()), y: -padding(this.data()) }).clip(false)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
    }.width(this.paintWidth).height(this.paintHeight).clip(false).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
  }
}
