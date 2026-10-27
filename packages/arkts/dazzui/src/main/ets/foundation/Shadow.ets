import { DazzShadow } from './TokenTypes'

/** Map CSS box-shadow lengths to ArkUI's physical-pixel shadow options. */
export function shadowOptions(value: DazzShadow | undefined, density: number): ShadowOptions {
  // CSS blur uses sigma = blur / 2. ArkUI's renderer converts its radius with
  // sigma = radius * 0.57735 + 0.5, so passing the CSS blur directly is wider.
  // See DrawingDecorationPainter::ConvertRadiusToSigma in arkui_ace_engine.
  const sigma = (value?.radius ?? 0) * density / 2
  return {
    radius: sigma > 0 ? Math.max(0, (sigma - 0.5) / 0.57735) : 0,
    offsetX: (value?.x ?? 0) * density,
    offsetY: (value?.y ?? 0) * density,
    color: value?.color ?? '#00000000',
  }
}
