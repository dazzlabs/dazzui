import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { createHash } from 'node:crypto'
import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'

const repo = fileURLToPath(new URL('../../', import.meta.url))
const require = createRequire(
  path.join(repo, 'packages/js/dazzui/example/package.json')
)
const { chromium } = require('playwright')
const directory = path.resolve(
  process.argv[2] ??
    'packages/arkts/dazzui/example/entry/build/verification/font-metrics'
)
const native = JSON.parse(
  fs.readFileSync(path.join(directory, 'native-font-metrics.json'), 'utf8')
)
const fontPath = path.resolve(
  process.argv[3] ?? path.join(directory, 'HarmonyOS_Sans.ttf')
)
const font = fs.readFileSync(fontPath)
assert.ok(
  native.density > 0,
  'Capture the native pixel density with the measurements'
)
const browser = await chromium.launch({
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
})
let reports
try {
  const page = await browser.newPage()
  reports = await page.evaluate(
    async ({ samples, source, density }) => {
      const face = new FontFace(
        'DazzNativeMetricsReference',
        'url(data:font/ttf;base64,' + source + ')',
        { weight: '1 1000' }
      )
      document.fonts.add(await face.load())
      return samples.map(sample => {
        const node = document.createElement('span')
        node.style.cssText = `display:inline-block;font-family:DazzNativeMetricsReference;font-size:${sample.size}px;font-weight:${sample.weight};font-feature-settings:${sample.tabular ? '"tnum" 1' : 'normal'}`
        node.textContent = sample.text
        document.body.append(node)
        const width = node.getBoundingClientRect().width
        return {
          ...sample,
          browserWidth: width,
          pixelCeiling: Math.ceil(width * density) / density,
        }
      })
    },
    {
      samples: native.measurements.filter(sample =>
        /^[\x20-\x7e]+$/.test(sample.text)
      ),
      source: font.toString('base64'),
      density: native.density,
    }
  )
} finally {
  await browser.close()
}
const measured = native.measurements.map(sample => ({
  ...sample,
  passed: Math.abs(sample.actual - sample.measured) < 0.0001,
}))
const comparisons = reports.map(sample => ({
  ...sample,
  passed: Math.abs(sample.actual - sample.pixelCeiling) < 0.0001,
}))
const result = {
  referenceFont: {
    file: path.basename(fontPath),
    sha256: createHash('sha256').update(font).digest('hex'),
  },
  density: native.density,
  systemFamily: native.systemFamily,
  fontId: native.fontId,
  fontScale: native.fontScale,
  fontWeightScale: native.fontWeightScale,
  nativeLayout: measured,
  sameFontLatin: comparisons,
  // The Latin font file does not establish which CJK fallback Chromium chose.
  fallbackNotCompared: native.measurements.filter(
    sample => !/^[\x20-\x7e]+$/.test(sample.text)
  ),
}
fs.writeFileSync(
  path.join(directory, 'font-reference-results.json'),
  JSON.stringify(result, null, 2)
)
console.log(
  JSON.stringify({
    nativeLayout: measured.filter(sample => sample.passed).length,
    nativeTotal: measured.length,
    sameFont: comparisons.filter(sample => sample.passed).length,
    sameFontTotal: comparisons.length,
  })
)
assert.ok(
  measured.every(sample => sample.passed),
  'Native measurement agrees with actual Text layout'
)
assert.ok(
  comparisons.every(sample => sample.passed),
  'Same-font browser widths agree after native physical-pixel ceiling'
)
