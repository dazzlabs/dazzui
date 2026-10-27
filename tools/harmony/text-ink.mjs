import assert from 'node:assert/strict'

const argb = value => {
  assert.match(value, /^#[0-9a-f]{8}$/i)
  return [1, 3, 5, 7].map(index => parseInt(value.slice(index, index + 2), 16))
}

/** Check actual text ink without assuming that native and browser glyph AA match. */
export function checkTextInk(image, bounds, ink, background) {
  const [alpha, ...foreground] = argb(ink)
  const [backgroundAlpha, ...ground] = argb(background)
  assert.equal(
    backgroundAlpha,
    255,
    'The isolated text uses an opaque frame surface'
  )
  const target = foreground.map(
    (channel, i) => ground[i] + ((channel - ground[i]) * alpha) / 255
  )
  const delta = target.map((channel, i) => channel - ground[i])
  const norm = delta.reduce((sum, n) => sum + n * n, 0)
  assert.ok(norm > 4, 'The source text has visible contrast')
  let maxRangeError = 0,
    strongestCoverage = 0,
    strongPixels = 0,
    samples = 0,
    worst
  for (
    let y = Math.max(0, Math.ceil(bounds[1]));
    y < Math.min(image.height, Math.floor(bounds[3]));
    y++
  )
    for (
      let x = Math.max(0, Math.ceil(bounds[0]));
      x < Math.min(image.width, Math.floor(bounds[2]));
      x++
    ) {
      const pixel = image.pixel(x, y)
      const coverage =
        delta.reduce((sum, d, i) => sum + (pixel[i] - ground[i]) * d, 0) / norm
      strongestCoverage = Math.max(strongestCoverage, coverage)
      if (coverage >= 0.6) strongPixels++
      for (let i = 0; i < 3; i++) {
        const error = Math.max(
          0,
          Math.min(ground[i], target[i]) - pixel[i],
          pixel[i] - Math.max(ground[i], target[i])
        )
        if (error > maxRangeError) {
          maxRangeError = error
          worst = { x, y, channel: i, actual: pixel[i] }
        }
      }
      samples++
    }
  assert.ok(samples > 10, 'Measure the visible native text rectangle')
  return {
    ink,
    background,
    samples,
    maxRangeError,
    strongestCoverage,
    strongPixels,
    worst,
    passed: maxRangeError <= 2 && strongPixels >= 3,
  }
}
