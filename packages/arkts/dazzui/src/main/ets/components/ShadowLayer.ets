import { RenderNode, FrameNode, NodeController, DrawContext, UIContext, LengthMetricsUnit } from '@kit.ArkUI'
import { drawing } from '@kit.ArkGraphics2D'
import { DazzShadow } from '../foundation/TokenTypes'

function shadowOutset(layers: DazzShadow[], density: number): number {
  const logical = layers.reduce((maximum: number, layer: DazzShadow): number =>
    Math.max(maximum, 2 * layer.radius + Math.abs(layer.x), 2 * layer.radius + Math.abs(layer.y)), 0)
  // Padding is a canvas allocation, not part of the design geometry. Keep it
  // on whole physical pixels so relocating the canvas preserves pixel phase.
  return Math.ceil(logical * density) / density
}

/** CSS uses a Gaussian image blur, not the native shadow mask's falloff. */
class ShadowRenderNode extends RenderNode {
  layers: DazzShadow[] = []
  paintWidth: number = 0
  paintHeight: number = 0
  corner: number = 0
  corners: number[] = []
  density: number = 1
  outset: number = 0
  constructor() {
    super()
    this.lengthMetricsUnit = LengthMetricsUnit.PX
    this.clipToFrame = false
  }
  roundRect(left: number, top: number, width: number, height: number): drawing.RoundRect {
    const corner = this.corner * this.density
    const box = new drawing.RoundRect({ left, top, right: left + width, bottom: top + height }, corner, corner)
    if (this.corners.length === 4) {
      const radii = this.corners.map((value: number): number => Math.max(0, value * this.density))
      // Apply CSS's common scale before individual native setters can
      // normalize a partially populated rectangle (notably pill groups).
      let scale: number = 1
      const sums = [radii[0] + radii[1], radii[1] + radii[2],
        radii[2] + radii[3], radii[3] + radii[0]]
      sums.forEach((sum: number, index: number) => {
        if (sum > 0) scale = Math.min(scale, (index % 2 === 0 ? width : height) / sum)
      })
      const positions: drawing.CornerPos[] = [drawing.CornerPos.TOP_LEFT_POS, drawing.CornerPos.TOP_RIGHT_POS,
        drawing.CornerPos.BOTTOM_RIGHT_POS, drawing.CornerPos.BOTTOM_LEFT_POS]
      positions.forEach((position: drawing.CornerPos, index: number) => {
        const radius = radii[index] * scale
        box.setCorner(position, radius, radius)
      })
    }
    return box
  }
  draw(context: DrawContext): void {
    if (this.paintWidth <= 0 || this.paintHeight <= 0) return
    const canvas = context.canvas
    const width = this.paintWidth * this.density
    const height = this.paintHeight * this.density
    const origin = this.outset * this.density
    // CSS lists its nearest shadow first; native source-over draws it last.
    for (let index = this.layers.length - 1; index >= 0; index--) {
      const value = this.layers[index]
      // Blur an opaque silhouette, then apply the token's alpha on restore.
      // Low-alpha tinted pixels otherwise lose precision in the blur's
      // intermediate premultiplied surface before reaching the backdrop.
      const composite = new drawing.Brush()
      composite.setAlpha(parseInt(value.color.slice(1, 3), 16))
      canvas.saveLayer({ left: 0, top: 0, right: width + 2 * origin, bottom: height + 2 * origin }, composite)
      const brush = new drawing.Brush()
      brush.setAntiAlias(true)
      brush.setColor4f({ alpha: 1,
        red: parseInt(value.color.slice(3, 5), 16) / 255, green: parseInt(value.color.slice(5, 7), 16) / 255, blue: parseInt(value.color.slice(7, 9), 16) / 255 }, null)
      const sigma = value.radius * this.density / 2
      if (sigma > 0) brush.setImageFilter(drawing.ImageFilter.createBlurImageFilter(sigma, sigma, drawing.TileMode.DECAL))
      const x = origin + value.x * this.density
      const y = origin + value.y * this.density
      canvas.attachBrush(brush)
      canvas.drawRoundRect(this.roundRect(x, y, width, height))
      canvas.detachBrush()
      // Knock out after filtering. Clipping before an image blur can clip its
      // source and erase the shadow, rather than just its final interior.
      const clear = new drawing.Brush()
      clear.setAntiAlias(true)
      clear.setBlendMode(drawing.BlendMode.CLEAR)
      canvas.attachBrush(clear)
      canvas.drawRoundRect(this.roundRect(origin, origin, width, height))
      canvas.detachBrush()
      canvas.restore()
    }
  }
}

class ShadowController extends NodeController {
  private root?: FrameNode
  private paint: ShadowRenderNode = new ShadowRenderNode()
  update(width: number, height: number, corner: number, corners: number[], layers: DazzShadow[], density: number): void {
    this.paint.paintWidth = width; this.paint.paintHeight = height
    this.paint.corner = corner; this.paint.layers = layers; this.paint.density = density
    this.paint.corners = corners.slice()
    this.paint.outset = shadowOutset(layers, density)
    const fullWidth = width + 2 * this.paint.outset
    const fullHeight = height + 2 * this.paint.outset
    this.paint.frame = { x: 0, y: 0, width: fullWidth * density, height: fullHeight * density }
    this.root?.commonAttribute.width(fullWidth).height(fullHeight)
    this.paint.invalidate()
  }
  makeNode(ui: UIContext): FrameNode {
    if (!this.root) {
      this.root = new FrameNode(ui)
      this.root.commonAttribute.width(this.paint.paintWidth + 2 * this.paint.outset).height(this.paint.paintHeight + 2 * this.paint.outset)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
      const render = this.root.getRenderNode()
      if (render) { render.clipToFrame = false; render.appendChild(this.paint) }
    }
    return this.root
  }
  dispose(): void { this.paint.dispose(); this.root?.dispose(); this.root = undefined }
}

/** A pointer-transparent drawing layer; the native content owns layout/input. */
@Component
export struct DazzShadowLayer {
  @Prop @Watch('sync') paintWidth: number = 0
  @Prop @Watch('sync') paintHeight: number = 0
  @Prop @Watch('sync') corner: number = 0
  @Prop @Watch('sync') corners: number[] = []
  @Prop @Watch('sync') shadowLayers: DazzShadow[] = []
  private controller: ShadowController = new ShadowController()
  sync(): void { this.controller.update(this.paintWidth, this.paintHeight, this.corner, this.corners, this.shadowLayers, this.getUIContext().vp2px(1)) }
  aboutToAppear(): void { this.sync() }
  aboutToDisappear(): void { this.controller.dispose() }
  outset(): number { return shadowOutset(this.shadowLayers, this.getUIContext().vp2px(1)) }
  build() {
    Stack() {
      NodeContainer(this.controller)
        .width(this.paintWidth + 2 * this.outset()).height(this.paintHeight + 2 * this.outset())
        .position({ x: -this.outset(), y: -this.outset() })
        // This is the expanded canvas, not the surface-sized wrapper. Clipping
        // here preserves the full blur and bounds the non-touchable subtree.
        .clip(true).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no')
    // None alone still contributes to ancestors' overflow-region hit tests.
    }.width(this.paintWidth).height(this.paintHeight).clip(false).hitTestBehavior(HitTestMode.None).touchable(false).accessibilityLevel('no')
  }
}
