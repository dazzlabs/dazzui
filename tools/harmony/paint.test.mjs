import assert from 'node:assert/strict'
import test from 'node:test'
import { readPNG, gaussianRoundRectCoverage } from './paint.mjs'

test('rounded shadow integration agrees with the independent closed-form Gaussian mass of a disk', () => {
  for (const radius of [10, 25, 50]) {
    for (const sigma of [5, 12, 30]) {
      const mass = gaussianRoundRectCoverage(
        [-radius, -radius, radius, radius],
        radius,
        0,
        0,
        sigma
      )
      const expected = 1 - Math.exp((-radius * radius) / (2 * sigma * sigma))
      assert.ok(
        Math.abs(mass - expected) < 0.00002,
        `${radius}/${sigma}: ${mass} vs ${expected}`
      )
    }
  }
})

test('rounded shadow integration preserves translations and has less corner coverage than a square', () => {
  const bounds = [0, 0, 200, 40]
  const rounded = gaussianRoundRectCoverage(bounds, 16, -5, -5, 12)
  const square = gaussianRoundRectCoverage(bounds, 0, -5, -5, 12)
  assert.ok(rounded < square * 0.8)
  assert.ok(
    Math.abs(
      rounded - gaussianRoundRectCoverage([100, 200, 300, 240], 16, 95, 195, 12)
    ) < 1e-12
  )
})

// Valid 2x2 RGBA PNGs with identical pixels and each PNG row filter. Fixed
// compressed inputs keep the decoder test independent of its implementation.
const fixtures = [
  'iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAGklEQVR4nGPgEpH7r2Fk08DAL6n8S9fcsQIAJtIEtPA3d1YAAAAASUVORK5CYII=',
  'iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAGUlEQVR4nGPkEpH7Lycn18jIL6n8C8ioAwAjdAQ2V0DT/gAAAABJRU5ErkJggg==',
  'iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAGUlEQVR4nGPiEpH7r2Fk08DEysr6G4h/AAAkTwRnhbdVcAAAAABJRU5ErkJggg==',
  'iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAGklEQVR4nGPmEpH7r6yhy8jMxS9SLSQktBsAHJYDVA8IkekAAAAASUVORK5CYII=',
  'iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAGUlEQVR4nGPhEpH7Lycn18jCysr6G4h/AAAh0wQwwHhfPAAAAABJRU5ErkJggg==',
]

test('the capture decoder reverses all five PNG row filters without losing alpha', () => {
  for (const value of fixtures) {
    const image = readPNG(Buffer.from(value, 'base64'))
    assert.equal(image.width, 2)
    assert.equal(image.height, 2)
    assert.deepEqual(image.pixel(0, 0), [10, 20, 30, 255])
    assert.deepEqual(image.pixel(1, 0), [40, 50, 60, 128])
    assert.deepEqual(image.pixel(0, 1), [15, 25, 35, 250])
    assert.deepEqual(image.pixel(1, 1), [45, 55, 65, 120])
  }
})

test('RGB captures have opaque pixels', () => {
  const image = readPNG(
    Buffer.from(
      'iVBORw0KGgoAAAANSUhEUgAAAAIAAAABCAIAAAB7QOjdAAAAD0lEQVR4nGPgEpHTMLIBAAI3ANNbVlHYAAAAAElFTkSuQmCC',
      'base64'
    )
  )
  assert.deepEqual(image.pixel(0, 0), [10, 20, 30, 255])
  assert.deepEqual(image.pixel(1, 0), [40, 50, 60, 255])
})

test('unsupported or truncated captures fail instead of yielding guessed pixels', () => {
  const valid = Buffer.from(fixtures[0], 'base64')
  assert.throws(() => readPNG(valid.subarray(0, 42)), /Truncated/)
  const depth = Buffer.from(valid)
  depth[24] = 16
  assert.throws(() => readPNG(depth), /8-bit/)
  const interlace = Buffer.from(valid)
  interlace[28] = 1
  assert.throws(() => readPNG(interlace), /Non-interlaced/)
  assert.throws(() => readPNG(valid).pixel(2, 0), /outside/)
})
