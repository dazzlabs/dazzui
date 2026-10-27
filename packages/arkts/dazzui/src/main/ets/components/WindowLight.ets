import { RenderNode, FrameNode, NodeController, DrawContext, UIContext, LengthMetricsUnit } from '@kit.ArkUI'
import { drawing } from '@kit.ArkGraphics2D'

interface LightBounds { left: number; top: number; right: number; bottom: number }

/** AppKit's chrome constants are transcribed from window-frame.css. */
class LightRenderNode extends RenderNode {
  diameter: number = 14
  density: number = 1
  surface: string = '#FFFF5F57'
  glazed: boolean = true
  padding: number = 2
  constructor() { super(); this.lengthMetricsUnit = LengthMetricsUnit.PX }
  draw(context: DrawContext): void {
    const canvas = context.canvas
    const size = this.diameter * this.density
    const origin = this.padding * this.density
    const center = origin + size / 2
    const radius = size / 2
    const brush = new drawing.Brush()
    brush.setAntiAlias(true)
    const rgb = this.surface.slice(-6)
    brush.setColor4f({ alpha: this.surface.length === 9 ? parseInt(this.surface.slice(1, 3), 16) / 255 : 1,
      red: parseInt(rgb.slice(0, 2), 16) / 255, green: parseInt(rgb.slice(2, 4), 16) / 255,
      blue: parseInt(rgb.slice(4, 6), 16) / 255 }, null)
    canvas.attachBrush(brush)
    canvas.drawCircle(center, center, radius)
    canvas.detachBrush()
    if (this.glazed) {
      const gloss = new drawing.Brush()
      gloss.setAntiAlias(true)
      gloss.setShaderEffect(drawing.ShaderEffect.createLinearGradient({ x: center, y: origin },
        { x: center, y: origin + size * 0.55 }, [0x38FFFFFF, 0x00FFFFFF], drawing.TileMode.CLAMP))
      canvas.attachBrush(gloss)
      canvas.drawCircle(center, center, radius)
      canvas.detachBrush()
    }
    // CSS's zero-blur inset spread is an inside ring, not a centred stroke.
    const edge = new drawing.Path()
    edge.addCircle(center, center, radius)
    edge.addCircle(center, center, radius - 0.5 * this.density)
    edge.setFillType(drawing.PathFillType.EVEN_ODD)
    const dark = new drawing.Brush()
    dark.setAntiAlias(true)
    dark.setColor4f({ alpha: 0.12, red: 0, green: 0, blue: 0 }, null)
    canvas.attachBrush(dark)
    canvas.drawPath(edge)
    canvas.detachBrush()
    if (this.glazed) {
      const bounds: LightBounds = { left: 0, top: 0, right: size + 2 * origin, bottom: size + 2 * origin }
      const composite = new drawing.Brush()
      composite.setAlpha(Math.round(0.55 * 255))
      canvas.saveLayer(bounds, composite)
      const inverse = new drawing.Path()
      inverse.addRect(bounds)
      inverse.addCircle(center, center + 0.5 * this.density, radius)
      inverse.setFillType(drawing.PathFillType.EVEN_ODD)
      const white = new drawing.Brush()
      white.setAntiAlias(true)
      white.setColor4f({ alpha: 1, red: 1, green: 1, blue: 1 }, null)
      white.setMaskFilter(drawing.MaskFilter.createBlurMaskFilter(drawing.BlurType.NORMAL, 0.25 * this.density))
      canvas.attachBrush(white)
      canvas.drawPath(inverse)
      canvas.detachBrush()
      // Keep the inverse source outside the light until after filtering.
      // Clipping it first would erase the top inset's Gaussian source.
      const outside = new drawing.Path()
      // The knockout rectangle must cover the allocation fully. Sharing its
      // antialiased edge with the source leaves a pale rectangular seam.
      const knockout: LightBounds = { left: -origin, top: -origin, right: bounds.right + origin, bottom: bounds.bottom + origin }
      outside.addRect(knockout)
      outside.addCircle(center, center, radius)
      outside.setFillType(drawing.PathFillType.EVEN_ODD)
      const clear = new drawing.Brush()
      clear.setAntiAlias(true)
      clear.setBlendMode(drawing.BlendMode.CLEAR)
      canvas.attachBrush(clear)
      canvas.drawPath(outside)
      canvas.detachBrush()
      canvas.restore()
    }
  }
}

class LightController extends NodeController {
  private root?: FrameNode
  private paint: LightRenderNode = new LightRenderNode()
  private paintID: string = ''
  update(diameter: number, surface: string, glazed: boolean, density: number, paintID: string): void {
    this.paintID = paintID
    this.paint.diameter = diameter; this.paint.surface = surface
    this.paint.glazed = glazed; this.paint.density = density
    this.paint.padding = Math.ceil(2 * density) / density
    const full = diameter + 2 * this.paint.padding
    this.paint.frame = { x: 0, y: 0, width: full * density, height: full * density }
    this.root?.commonAttribute.width(full).height(full)
    this.paint.invalidate()
  }
  makeNode(ui: UIContext): FrameNode {
    if (!this.root) {
      this.root = new FrameNode(ui)
      this.root.commonAttribute.width(this.paint.diameter + 2 * this.paint.padding).height(this.paint.diameter + 2 * this.paint.padding)
        .id(this.paintID)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
      this.root.getRenderNode()?.appendChild(this.paint)
    }
    return this.root
  }
  dispose(): void { this.paint.dispose(); this.root?.dispose(); this.root = undefined }
}

/** Paint only; the separate native Button retains activation and semantics. */
@Component
export struct DazzWindowLight {
  @Prop @Watch('sync') diameter: number = 14
  @Prop @Watch('sync') surface: string = '#FFFF5F57'
  @Prop @Watch('sync') glazed: boolean = true
  private controller: LightController = new LightController()
  sync(): void { this.controller.update(this.diameter, this.surface, this.glazed, this.getUIContext().vp2px(1), 'dazz-window-light-paint-' + String(this.getUniqueId())) }
  aboutToAppear(): void { this.sync() }
  aboutToDisappear(): void { this.controller.dispose() }
  outset(): number { const density = this.getUIContext().vp2px(1); return Math.ceil(2 * density) / density }
  build() {
    Stack() {
      NodeContainer(this.controller).width(this.diameter + 2 * this.outset()).height(this.diameter + 2 * this.outset())
        .id('dazz-window-light-canvas-' + String(this.getUniqueId()))
        .position({ x: -this.outset(), y: -this.outset() }).clip(true)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
    }.width(this.diameter).height(this.diameter).clip(false)
      .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
  }
}
