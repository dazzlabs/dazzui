// Full alpha-mask comparisons of CSS overflow clipping against native GPU output.
// A one-byte tolerance covers UNORM rounding; every larger difference fails.
import assert from 'node:assert/strict'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { parseArgs } from 'node:util'
import { mkdir, readFile, writeFile } from 'node:fs/promises'
import { join } from 'node:path'
const loadPixels = (page, png) =>
  page.evaluate(async s => {
    const i = new Image()
    i.src = 'data:image/png;base64,' + s
    await i.decode()
    const c = document.createElement('canvas')
    c.width = i.width
    c.height = i.height
    const x = c.getContext('2d')
    x.drawImage(i, 0, 0)
    const data = x.getImageData(0, 0, c.width, c.height).data
    return {
      width: c.width,
      height: c.height,
      rgba: Array.from(data),
      alpha: Array.from({ length: data.length / 4 }, (_, i) => data[i * 4 + 3]),
    }
  }, png.toString('base64'))

const comparePixels = (pixels, actual) => {
  assert(
    actual.width === pixels.width && actual.height === pixels.height,
    'native image size differs'
  )
  let maxDelta = 0,
    failedPixels = 0
  const failures = []
  pixels.alpha.forEach((v, i) => {
    const delta = Math.abs(v - actual.alpha[i])
    maxDelta = Math.max(maxDelta, delta)
    if (delta > 1) {
      failedPixels++
      if (failures.length < 12)
        failures.push({
          x: i % pixels.width,
          y: Math.floor(i / pixels.width),
          css: v,
          native: actual.alpha[i],
        })
    }
  })
  // Keep color evidence beside the alpha gate. Premultiplication avoids
  // inflating harmless straight-color errors in nearly transparent pixels.
  let maxPremultipliedColorDelta = 0,
    colorPixelsOverOne = 0
  for (let i = 0; i < pixels.rgba.length; i += 4) {
    let delta = 0
    for (let channel = 0; channel < 3; channel++) {
      delta = Math.max(
        delta,
        Math.abs(
          pixels.rgba[i + channel] * pixels.rgba[i + 3] -
            actual.rgba[i + channel] * actual.rgba[i + 3]
        )
      )
    }
    maxPremultipliedColorDelta = Math.max(
      maxPremultipliedColorDelta,
      delta / 255
    )
    if (delta > 255) colorPixelsOverOne++
  }
  return {
    maxDelta,
    failedPixels,
    failures,
    maxPremultipliedColorDelta,
    colorPixelsOverOne,
  }
}

const example = fileURLToPath(
  new URL('../../../../js/dazzui/example/', import.meta.url)
)
const { chromium } = await import(
  pathToFileURL(join(example, 'node_modules/playwright/index.mjs'))
)
const { values } = parseArgs({
  options: {
    gpu: { type: 'boolean' },
    native: { type: 'string' },
    out: { type: 'string' },
    cases: { type: 'string' },
  },
})
assert(
  values.out && values.native,
  'pass --native <DAZZUI_GPU_PROBE_OUT> --out <results>'
)
const out = values.out
await mkdir(out, { recursive: true })
const requested = values.cases?.split(',')
const cases = JSON.parse(
  await readFile(
    new URL('../../compat/gpui/fixtures/overflow-raster.json', import.meta.url),
    'utf8'
  )
)
if (requested)
  assert(
    requested.every(id => cases.some(c => c.id === id)),
    'unknown raster case'
  )
const browser = await chromium.launch({
  headless: true,
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE,
  args: values.gpu
    ? [
        '--enable-gpu',
        '--enable-gpu-rasterization',
        ...(process.platform === 'linux' ? ['--use-angle=vulkan'] : []),
      ]
    : [],
})
try {
  const session = await browser.newBrowserCDPSession()
  const { gpu } = await session.send('SystemInfo.getInfo')
  await writeFile(
    join(out, 'browser.json'),
    JSON.stringify({ version: browser.version(), gpu }, null, 2) + '\n'
  )
  const renderer = gpu.auxAttributes?.glRenderer
  const software =
    /swiftshader|llvmpipe|lavapipe|software|Microsoft Basic Render Driver|\bWARP\b/i
  if (values.gpu)
    assert(
      gpu.featureStatus.gpu_compositing?.startsWith('enabled') &&
        gpu.featureStatus.rasterization?.startsWith('enabled') &&
        renderer &&
        !software.test(renderer),
      'browser did not enable hardware rasterization: ' + renderer
    )
  const results = []
  const styledResults = []
  for (const dpr of [1, 2]) {
    const page = await browser.newPage({
      viewport: { width: 64, height: 64 },
      deviceScaleFactor: dpr,
    })
    const cdp = await page.context().newCDPSession(page)
    let layers = []
    cdp.on('LayerTree.layerTreeDidChange', e => {
      layers = e.layers ?? []
    })
    await cdp.send('LayerTree.enable')
    for (const c of cases) {
      if (requested && !requested.includes(c.id)) continue
      const [t, r, b, l] = c.b,
        w = c.w ?? 48,
        h = c.h ?? 48,
        x = c.x ?? 8,
        y = c.y ?? 8
      const base = `left:${-l}px;top:${-t}px;width:${w}px;height:${h}px;`
      let child = `<div style="${base}background:#00ff00"></div>`
      if (c.paint === 'gradient')
        child = `<div style="${base}background:linear-gradient(red,blue)"></div>`
      if (['image', 'image-overlay'].includes(c.paint))
        child = `<img style="${base}"/>`
      if (c.paint === 'split')
        child = `<div style="left:0;top:0;width:${(w - l - r) / 2}px;height:${h - t - b}px;background:red"></div><div style="left:${(w - l - r) / 2}px;top:0;width:${(w - l - r) / 2}px;height:${h - t - b}px;background:blue"></div>`
      if (['overlap', 'nested-overlap'].includes(c.paint)) {
        child = `<div style="left:0;top:0;width:${(w - l - r) * 0.7}px;height:${h - t - b}px;background:rgba(255,0,0,.5)"></div><div style="left:${(w - l - r) * 0.3}px;top:0;width:${(w - l - r) * 0.7}px;height:${h - t - b}px;background:rgba(0,0,255,.5)"></div>`
        if (c.paint === 'nested-overlap')
          child = `<div style="left:0;top:0;width:${w - l - r}px;height:${h - t - b}px;opacity:.5">${child}</div>`
      }
      if (c.paint === 'interleaved') {
        child = `<div style="left:0;top:0;width:${(w - l - r) * 0.7}px;height:${h - t - b}px;background:rgba(255,0,0,.5)"></div><div style="left:${(w - l - r) * 0.3}px;top:0;width:${(w - l - r) * 0.7}px;height:${h - t - b}px;opacity:.5"><div style="inset:0;background:blue"></div></div><div style="left:0;top:${(h - t - b) * 0.5}px;width:${(w - l - r) * 0.7}px;height:${(h - t - b) * 0.5}px;background:rgba(0,255,0,.25)"></div>`
      }
      if (c.paint === 'image-overlay')
        child += `<div style="left:${(w - l - r) * 0.3}px;top:0;width:${(w - l - r) * 0.7}px;height:${h - t - b}px;background:rgba(0,0,255,.5)"></div>`
      await page.setContent(
        `<style>html,body{margin:0;width:64px;height:64px}div,img{position:absolute}</style><div style="box-sizing:border-box;left:${x}px;top:${y}px;width:${w}px;height:${h}px;border-style:solid;border-color:transparent;border-width:${t}px ${r}px ${b}px ${l}px;border-radius:${c.r.map(v => v + 'px').join(' ')};overflow:hidden;${c.scale === undefined ? '' : `transform:scale(${c.scale});transform-origin:0 0`}">${child}</div>`
      )
      if (['image', 'image-overlay'].includes(c.paint))
        await page.evaluate(async () => {
          const c = document.createElement('canvas')
          c.width = c.height = 1
          const x = c.getContext('2d')
          x.fillStyle = 'lime'
          x.fillRect(0, 0, 1, 1)
          document.images[0].src = c.toDataURL()
          await document.images[0].decode()
        })
      const png = await page.screenshot({
        path: join(out, `${c.id}-${dpr}.png`),
        omitBackground: true,
        timeout: 30000,
      })
      const pixels = await loadPixels(page, png)
      const nativePng = await readFile(
        join(values.native, `${c.id}-${dpr}.png`)
      )
      await writeFile(join(out, `${c.id}-${dpr}-native.png`), nativePng)
      const actual = await loadPixels(page, nativePng)
      const comparison = comparePixels(pixels, actual)
      const { maxDelta, failedPixels } = comparison
      const clips = []
      for (const layer of layers.filter(
        l => l.drawsContent && l.width && l.height
      )) {
        try {
          const { snapshotId } = await cdp.send('LayerTree.makeSnapshot', {
            layerId: layer.layerId,
          })
          const { commandLog } = await cdp.send(
            'LayerTree.snapshotCommandLog',
            { snapshotId }
          )
          clips.push(
            ...commandLog
              .filter(c => c.method === 'clipRRect')
              .map(c => c.params.rrect)
          )
          await cdp.send('LayerTree.releaseSnapshot', { snapshotId })
        } catch {}
      }
      results.push({
        fixture: c,
        dpr,
        clips,
        width: pixels.width,
        height: pixels.height,
        ...comparison,
      })
      if (c.styled && dpr === 2) {
        const styled = await readFile(
          join(values.native, `styled-${c.id}-${dpr}.png`)
        )
        await writeFile(join(out, `styled-${c.id}-${dpr}-native.png`), styled)
        const comparison = comparePixels(pixels, await loadPixels(page, styled))
        styledResults.push({ fixture: c, dpr, ...comparison })
        console.log(
          `styled ${c.id} @${dpr}x: max alpha delta ${comparison.maxDelta}, ${comparison.failedPixels} failed pixels`
        )
      }
      console.log(
        `${c.id} @${dpr}x: max alpha delta ${maxDelta}, ${failedPixels} failed pixels`
      )
    }
    await page.close()
  }
  await writeFile(
    join(out, 'model.json'),
    JSON.stringify({ version: browser.version(), gpu, results, styledResults })
  )
  if (
    [...results, ...styledResults].some(
      r => r.failedPixels || (r.fixture.rgba && r.colorPixelsOverOne)
    )
  )
    process.exitCode = 1
  console.log(
    `${results.filter(r => !r.failedPixels).length}/${results.length} full alpha masks passed`
  )
  console.log(
    `${styledResults.filter(r => !r.failedPixels).length}/${styledResults.length} styled Div alpha masks passed`
  )
  const colorResults = results.filter(r => r.fixture.rgba)
  console.log(
    `${colorResults.filter(r => !r.failedPixels && !r.colorPixelsOverOne).length}/${colorResults.length} premultiplied RGBA masks passed`
  )
} finally {
  await browser.close()
}
