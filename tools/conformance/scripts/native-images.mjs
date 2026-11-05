import sharp from 'sharp'
import { mkdirSync } from 'node:fs'
import { dirname } from 'node:path'

/** Crop native-window pixels using measured logical bounds, without rescaling. */
export async function cropNativeImage(input, output, rect, scale) {
  const image = sharp(input)
  const { width, height } = await image.metadata()
  const left = Math.max(0, Math.round(rect.x * scale))
  const top = Math.max(0, Math.round(rect.y * scale))
  const right = Math.min(width, Math.round((rect.x + rect.width) * scale))
  const bottom = Math.min(height, Math.round((rect.y + rect.height) * scale))
  if (right <= left || bottom <= top) return false
  mkdirSync(dirname(output), { recursive: true })
  await image
    .extract({ left, top, width: right - left, height: bottom - top })
    .toColourspace('srgb')
    .png()
    .toFile(output)
  return true
}

export async function nativeImageScale(input, viewportWidth) {
  const { width } = await sharp(input).metadata()
  const scale = width / viewportWidth
  if (!Number.isFinite(scale) || scale <= 0)
    throw new Error('Invalid native image scale')
  return scale
}
