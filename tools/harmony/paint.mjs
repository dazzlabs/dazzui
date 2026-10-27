import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { inflateSync } from 'node:zlib'
import { pathToFileURL } from 'node:url'

// The SDK screenshot command emits non-interlaced 8-bit RGB/RGBA PNGs. Keep
// their real pixels readable without adding a host image-library dependency.
export function readPNG(buffer) {
  assert.equal(buffer.subarray(0, 8).toString('hex'), '89504e470d0a1a0a')
  let width, height, channels
  const chunks = []
  let ended = false
  for (let offset = 8; offset < buffer.length; ) {
    const length = buffer.readUInt32BE(offset)
    assert.ok(offset + length + 12 <= buffer.length, 'Truncated PNG chunk')
    const type = buffer.toString('ascii', offset + 4, offset + 8)
    const value = buffer.subarray(offset + 8, offset + 8 + length)
    if (type === 'IHDR') {
      width = value.readUInt32BE(0)
      height = value.readUInt32BE(4)
      assert.ok(width > 0 && height > 0 && width * height <= 64 * 1024 * 1024)
      assert.equal(value[8], 8, '8-bit screenshot required')
      assert.ok(
        value[9] === 2 || value[9] === 6,
        'RGB/RGBA screenshot required'
      )
      assert.equal(value[12], 0, 'Non-interlaced screenshot required')
      channels = value[9] === 6 ? 4 : 3
    } else if (type === 'IDAT') chunks.push(value)
    else if (type === 'IEND') {
      ended = true
      break
    }
    offset += length + 12
  }
  assert.ok(ended && width && channels && chunks.length, 'Incomplete PNG')
  const stride = width * channels
  const raw = inflateSync(Buffer.concat(chunks))
  assert.equal(raw.length, (stride + 1) * height)
  const data = Buffer.alloc(stride * height)
  for (let row = 0; row < height; row++) {
    const kind = raw[row * (stride + 1)]
    assert.ok(kind <= 4, 'Unknown PNG row filter')
    for (let x = 0; x < stride; x++) {
      const index = row * stride + x
      const left = x >= channels ? data[index - channels] : 0
      const up = row ? data[index - stride] : 0
      const corner = row && x >= channels ? data[index - stride - channels] : 0
      let predictor = 0
      if (kind === 1) predictor = left
      else if (kind === 2) predictor = up
      else if (kind === 3) predictor = Math.floor((left + up) / 2)
      else if (kind === 4) {
        const value = left + up - corner
        const a = Math.abs(value - left),
          b = Math.abs(value - up),
          c = Math.abs(value - corner)
        predictor = a <= b && a <= c ? left : b <= c ? up : corner
      }
      data[index] = raw[row * (stride + 1) + x + 1] + predictor
    }
  }
  return {
    width,
    height,
    pixel(x, y) {
      assert.ok(
        x >= 0 && x < width && y >= 0 && y < height,
        'Sample outside screenshot'
      )
      const index = (y * width + x) * channels
      return [
        data[index],
        data[index + 1],
        data[index + 2],
        channels === 4 ? data[index + 3] : 255,
      ]
    },
  }
}

const length = value => {
  assert.ok(value.unit === 'rem' || value.unit === 'px')
  return value.value * (value.unit === 'rem' ? 16 : 1)
}
const cdf = value => {
  // Abramowitz-Stegun erf approximation, absolute error below 1.5e-7.
  const x = Math.abs(value) / Math.SQRT2
  const t = 1 / (1 + 0.3275911 * x)
  const erf =
    1 -
    ((((1.061405429 * t - 1.453152027) * t + 1.421413741) * t - 0.284496736) *
      t +
      0.254829592) *
      t *
      Math.exp(-x * x)
  return (1 + (value < 0 ? -erf : erf)) / 2
}

// Integrate the two rounded caps in one dimension; the middle strip has an
// analytic rectangular convolution. This also covers very short receipts,
// whose corner contribution cannot be ignored at the centre of an edge.
export function gaussianRoundRectCoverage(bounds, radius, x, y, sigma) {
  assert.ok(sigma > 0)
  const [left, top, right, bottom] = bounds
  const r = Math.min(
    Math.max(0, radius),
    (right - left) / 2,
    (bottom - top) / 2
  )
  const span = inset =>
    cdf((right - inset - x) / sigma) - cdf((left + inset - x) / sigma)
  if (!r) return span(0) * (cdf((bottom - y) / sigma) - cdf((top - y) / sigma))
  let coverage =
    span(0) * (cdf((bottom - r - y) / sigma) - cdf((top + r - y) / sigma))
  // Angle parameterization removes the square-root endpoint singularity.
  const steps = 256
  const step = Math.PI / (2 * steps)
  for (let index = 0; index < steps; index++) {
    const angle = (index + 0.5) * step
    const sine = Math.sin(angle)
    const offset = r * (1 - Math.cos(angle))
    const inset = r * (1 - sine)
    const upper = (top + offset - y) / sigma
    const lower = (bottom - offset - y) / sigma
    coverage +=
      (span(inset) *
        (Math.exp((-upper * upper) / 2) + Math.exp((-lower * lower) / 2)) *
        r *
        sine *
        step) /
      (sigma * Math.sqrt(2 * Math.PI))
  }
  return coverage
}

export function checkShadowPaint(file, themes) {
  const capture = JSON.parse(fs.readFileSync(file, 'utf8'))
  const tokens = themes[capture.theme]
  const layers = capture.role === 'none' ? [] : tokens[capture.role]
  assert.ok(layers.length <= 1, 'This edge probe covers one outer shadow')
  const layer = layers[0]
  if (layer) {
    assert.ok(!layer.inset)
    assert.equal(length(layer.spread), 0)
  }
  const density = capture.density
  const [left, top, right, bottom] = capture.bounds
  const radius = capture.radiusRole
    ? length(tokens[capture.radiusRole]) * density
    : 0
  const sigma = layer ? (length(layer.blur) * density) / 2 : 0
  const dx = layer ? length(layer.offsetX) * density : 0
  const dy = layer ? length(layer.offsetY) * density : 0
  const [alpha, ...color] = layer?.color ?? [0, 0, 0, 0]
  assert.ok(
    !capture.backdropRole || capture.backdropRole === 'color.surface-column'
  )
  const [backgroundAlpha, ...surface] =
    tokens[capture.backdropRole ?? 'color.surface']
  assert.equal(backgroundAlpha, 255, 'Opaque probe backdrop required')
  assert.ok(!capture.backdrop || capture.backdrop === 'dialog-scrim')
  const scrim = tokens['dialog.scrim.color']
  const scrimAlpha = Math.round(scrim[0] * tokens['dialog.scrim.alpha']) / 255
  const background =
    capture.backdrop === 'dialog-scrim'
      ? surface.map(
          (channel, index) =>
            channel * (1 - scrimAlpha) + scrim[index + 1] * scrimAlpha
        )
      : surface
  const image = readPNG(fs.readFileSync(file.replace(/\.json$/, '.png')))
  const viewport = capture.viewport ?? [0, 0, image.width, image.height]
  const midX = Math.round((left + right) / 2),
    midY = Math.round((top + bottom) / 2)
  assert.ok(capture.outline === undefined || capture.outline === 'none')
  assert.ok(capture.sampling === undefined || capture.sampling === 'pixels')
  // An unfocused control has no exterior outline; sample its thin shadow
  // immediately outside the border box, rather than beyond the whole blur.
  const start = capture.outline === 'none' ? 2 : Math.ceil(2 * density) + 1
  const stop = layer
    ? Math.ceil(3.5 * sigma + Math.max(Math.abs(dx), Math.abs(dy)))
    : start + 64
  const errors = []
  const profiles = {}
  let peak = 0
  let worstSample
  for (let distance = start; distance < stop; distance++) {
    const points = [
      [Math.floor(left) - distance, midY, true, 'left'],
      [Math.ceil(right) + distance, midY, true, 'right'],
      [midX, Math.floor(top) - distance, false, 'top'],
      [midX, Math.ceil(bottom) + distance, false, 'bottom'],
    ]
    if (capture.radiusRole)
      points.push(
        [
          Math.floor(left) - distance,
          Math.floor(top) - distance,
          false,
          'top-left',
        ],
        [
          Math.ceil(right) + distance,
          Math.floor(top) - distance,
          false,
          'top-right',
        ],
        [
          Math.floor(left) - distance,
          Math.ceil(bottom) + distance,
          false,
          'bottom-left',
        ],
        [
          Math.ceil(right) + distance,
          Math.ceil(bottom) + distance,
          false,
          'bottom-right',
        ]
      )
    for (const [x, y, horizontal, profile] of points) {
      const locations = [-2, -1, 0, 1, 2].map(delta => [
        horizontal ? x : x + delta,
        horizontal ? y + delta : y,
      ])
      // Receipts at the viewport edge have the same clipped outer tail as
      // CSS. Never compare their clipped pixels against the desktop/taskbar.
      if (
        locations.some(
          ([px, py]) =>
            px < viewport[0] ||
            py < viewport[1] ||
            px >= viewport[2] ||
            py >= viewport[3]
        )
      )
        continue
      const groups =
        capture.sampling === 'pixels'
          ? locations.map(point => [point])
          : [locations]
      for (const group of groups) {
        profiles[profile] = (profiles[profile] ?? 0) + 1
        const coverage = layer
          ? group.reduce(
              (sum, [px, py]) =>
                sum +
                gaussianRoundRectCoverage(
                  [left + dx, top + dy, right + dx, bottom + dy],
                  radius,
                  px + 0.5,
                  py + 0.5,
                  sigma
                ),
              0
            ) / group.length
          : 0
        const opacity = (coverage * alpha) / 255
        const pixels = group.map(([px, py]) => image.pixel(px, py))
        assert.ok(
          pixels.every(pixel => pixel[3] === 255),
          'Opaque display capture required'
        )
        for (let channel = 0; channel < 3; channel++) {
          const expected =
            background[channel] * (1 - opacity) + color[channel] * opacity
          const actual =
            pixels.reduce((sum, pixel) => sum + pixel[channel], 0) /
            pixels.length
          const error = Math.abs(actual - expected)
          errors.push(error)
          if (!worstSample || error > worstSample.error)
            worstSample = {
              x: group.length === 1 ? group[0][0] : x,
              y: group.length === 1 ? group[0][1] : y,
              channel,
              actual,
              expected,
              error,
            }
          peak = Math.max(peak, Math.abs(expected - background[channel]))
        }
      }
    }
  }
  assert.ok(
    capture.profileClipping === undefined ||
      capture.profileClipping === 'viewport'
  )
  const clippedProfiles = []
  for (const profile of capture.radiusRole
    ? [
        'left',
        'right',
        'top',
        'bottom',
        'top-left',
        'top-right',
        'bottom-left',
        'bottom-right',
      ]
    : ['left', 'right', 'top', 'bottom']) {
    if ((profiles[profile] ?? 0) >= 8) continue
    // Long native sheets can end at the window edge. Every visible pixel is
    // still compared; record profiles whose remaining samples are outside
    // the independently measured viewport. Ordinary captures require all.
    assert.equal(
      capture.profileClipping,
      'viewport',
      'Insufficient visible samples on ' + profile
    )
    clippedProfiles.push(profile)
  }
  assert.ok(
    Object.values(profiles).some(count => count >= 8),
    'At least one shadow falloff remains visible'
  )
  if (layer)
    assert.ok(
      peak > 2,
      'Insufficient shadow contrast for this byte-level probe'
    )
  const maximum = Math.max(...errors)
  const rms = Math.sqrt(
    errors.reduce((sum, error) => sum + error * error, 0) / errors.length
  )
  return {
    file,
    role: capture.role,
    theme: capture.theme,
    sigmaPx: sigma,
    samples: errors.length,
    profiles,
    clippedProfiles,
    maxChannelError: maximum,
    rmsChannelError: rms,
    worstSample,
    passed: maximum <= 2 && rms <= 1,
  }
}

if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href
) {
  const directory = process.argv[2]
  assert.ok(directory, 'Pass a measured paint-capture directory')
  const themes = JSON.parse(
    fs.readFileSync(
      new URL(
        '../../packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
        import.meta.url
      ),
      'utf8'
    )
  )
  const files = fs
    .readdirSync(directory)
    .filter(file => /^native-(?:shadow-|none).*\.json$/.test(file))
  assert.ok(files.length, 'No measured paint captures found')
  const reports = files.map(file =>
    checkShadowPaint(path.join(directory, file), themes)
  )
  console.log(JSON.stringify(reports, null, 2))
  if (reports.some(report => !report.passed)) process.exitCode = 1
}
