import { UIContext } from '@kit.ArkUI'
import { image } from '@kit.ImageKit'
import { Scene, SceneValue } from './SceneTypes'

export interface CapturedBox {
  address: string
  width: number
  height: number
  left: number
  top: number
}
export interface SceneCapture {
  scene: string
  theme: string
  scale: number
  gaps: string[]
  boxes: CapturedBox[]
  unmounted: string[]
}

function addresses(node: SceneValue, result: number[]): void {
  if (node.kind === 'element' && (node.address ?? -1) >= 0)
    result.push(node.address ?? -1)
  for (const child of node.items ?? []) addresses(child, result)
  const fields = node.fields ?? {}
  for (const key of Object.keys(fields)) addresses(fields[key], result)
}

// Call after layout. Read actual ArkUI geometry, never the token's expected
// box. Hidden popup content is kept separate from an unsupported scene.
export function captureBoxes(
  ui: UIContext,
  scene: Scene,
  theme: string
): SceneCapture {
  const result: SceneCapture = {
    scene: scene.id,
    theme: theme,
    scale: ui.vp2px(1),
    gaps: scene.gaps,
    boxes: [],
    unmounted: [],
  }
  if (scene.gaps.length) return result
  const indices: number[] = []
  addresses(scene.content, indices)
  const components = ui.getComponentUtils()
  const root = components.getRectangleById('dazz-scene-root')
  for (const index of indices) {
    const address = scene.id + '/' + index
    try {
      const box = components.getRectangleById(address)
      if (box.size.width <= 0 || box.size.height <= 0) {
        result.unmounted.push(address)
        continue
      }
      result.boxes.push({
        address: address,
        width: ui.px2vp(box.size.width),
        height: ui.px2vp(box.size.height),
        left: ui.px2vp(box.windowOffset.x - root.windowOffset.x),
        top: ui.px2vp(box.windowOffset.y - root.windowOffset.y),
      })
    } catch (_) {
      result.unmounted.push(address)
    }
  }
  return result
}

export function capturePixels(ui: UIContext): Promise<image.PixelMap> {
  return ui
    .getComponentSnapshot()
    .get('dazz-scene-root', { scale: 2, waitUntilRenderFinished: true })
}
