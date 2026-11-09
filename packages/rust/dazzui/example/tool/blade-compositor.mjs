// Compare Metal, Blade or DirectX GPU probe PNGs with CSS in Chromium.
// Selected interior and edge probes gate semantics (one UNORM rounding unit).
// Full-image differences, including rounded-edge AA, remain diagnostic.
import assert from 'node:assert/strict'
import { mkdir, readFile, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { parseArgs } from 'node:util'

const { values } = parseArgs({
  options: {
    native: { type: 'string' },
    gpu: { type: 'boolean', default: false },
    cases: { type: 'string' },
    out: {
      type: 'string',
      default: join(tmpdir(), `dazzui-renderer-css-${process.pid}`),
    },
  },
})
assert(values.native, 'pass --native <DAZZUI_GPU_PROBE_OUT directory>')
const native = resolve(values.native),
  out = resolve(values.out),
  example = fileURLToPath(
    new URL('../../../../js/dazzui/example/', import.meta.url)
  )
const { chromium } = await import(
  pathToFileURL(join(example, 'node_modules/playwright/index.mjs'))
)
const rect = (x, y, width, height, color) =>
  `<div style="left:${x}px;top:${y}px;width:${width}px;height:${height}px;background:${color}"></div>`
const cases = [
  {
    id: 'alpha-overlap',
    html: `<div style="width:64px;height:64px;opacity:.5">${rect(4, 4, 28, 28, 'rgba(255,0,0,.5)')}${rect(20, 4, 28, 28, 'rgba(255,0,0,.5)')}</div>`,
    probes: [
      [10, 10],
      [24, 10],
      [40, 10],
      [55, 10],
    ],
  },
  {
    id: 'opacity',
    html: `<div style="width:64px;height:64px;opacity:.5">${rect(4, 8, 32, 16, 'red')}${rect(20, 8, 32, 16, 'blue')}</div>`,
    probes: [
      [10, 16],
      [25, 16],
      [60, 16],
    ],
  },
  {
    id: 'rounded',
    html: `<div style="width:64px;height:64px;opacity:.5"><div style="left:8px;top:8px;width:32px;height:32px;border-radius:10px;overflow:hidden;opacity:.5">${rect(-8, -8, 64, 64, '#00ff00')}</div></div>`,
    probes: [
      [8, 8],
      [24, 24],
      [45, 24],
    ],
  },
  {
    id: 'padding-edge',
    html: `<div style="box-sizing:border-box;left:8px;top:8px;width:48px;height:48px;border-style:solid;border-color:transparent;border-width:4px 10px 8px 2px;border-radius:16px;overflow:hidden">${rect(-2, -4, 48, 48, '#00ff00')}</div>`,
    probes: [
      [9, 30],
      [47, 30],
      [28, 10],
      [28, 49],
      [10, 14],
      [11, 47],
      [28, 28],
      [15, 20],
      [41, 20],
      [16, 43],
      [41, 43],
    ],
  },
  {
    id: 'padding-edge-wide-border',
    html: `<div style="box-sizing:border-box;left:8px;top:8px;width:48px;height:48px;border-style:solid;border-color:transparent;border-width:4px 0px 8px 38px;border-radius:16px;overflow:hidden">${rect(-38, -4, 48, 48, '#00ff00')}</div>`,
    probes: [
      [44, 24],
      [48, 12],
      [54, 16],
      [52, 46],
      [48, 24],
      [54, 24],
      [48, 36],
    ],
  },
  {
    id: 'axis-clip',
    html: `<div style="box-sizing:border-box;left:16px;top:16px;width:32px;height:32px;border:4px solid transparent;border-radius:16px;overflow-x:clip;overflow-y:visible">${rect(-20, -20, 64, 64, 'blue')}</div>`,
    probes: [
      [20, 4],
      [22, 17],
      [32, 60],
      [19, 32],
      [44, 32],
    ],
  },
  {
    id: 'scale',
    html: `<div style="left:28px;width:36px;height:64px;overflow:hidden"><div style="left:-28px;width:64px;height:64px;transform:scale(.5);transform-origin:32px 32px">${rect(16, 16, 32, 32, 'red')}</div></div>`,
    probes: [
      [25, 32],
      [32, 32],
      [41, 32],
    ],
  },
  {
    id: 'offscreen-scale',
    html: `<div style="left:-32px;top:-32px;width:128px;height:128px;opacity:.5;transform:scale(.5);transform-origin:64px 64px">${rect(16, 56, 16, 16, 'red')}${rect(96, 56, 16, 16, 'blue')}${rect(56, 16, 16, 16, '#00ff00')}${rect(56, 96, 16, 16, 'yellow')}</div>`,
    probes: [
      [12, 32],
      [52, 32],
      [32, 12],
      [32, 52],
      [32, 32],
    ],
  },
  {
    id: 'primitives',
    html: `<div style="width:64px;height:64px;opacity:.5">${rect(4, 4, 12, 12, 'red')}${rect(20, 4, 12, 12, '#00ff0080')}${rect(36, 4, 12, 12, '#0000ff80')}</div>${rect(36, 32, 12, 12, '#0000ff80')}`,
    probes: [
      [10, 10],
      [26, 10],
      [42, 10],
      [42, 38],
      [60, 16],
      [4, 4],
      [15, 15],
      [20, 4],
      [31, 15],
    ],
  },
  {
    id: 'alpha-image',
    html: '<img style="left:4px;top:4px;width:16px;height:8px;opacity:.5"><div style="width:64px;height:64px;opacity:.5"><img style="left:4px;top:20px;width:16px;height:8px"></div>',
    imagePixels: [255, 0, 0, 255, 0, 0, 255, 0],
    probes: [
      [4, 7],
      [11, 7],
      [12, 7],
      [19, 7],
      [4, 23],
      [11, 23],
      [12, 23],
      [19, 23],
    ],
  },
  {
    id: 'color-font',
    html: '<div style="width:64px;height:64px;opacity:.5"><div style="left:4px;top:4px;font-family:DazzUIProbe;font-size:24px;line-height:24px">A</div></div>',
    fontProbe: true,
    probes: [
      [16, 16],
      [40, 40],
    ],
  },
  {
    id: 'decorations',
    html: '<div style="left:10px;width:30px;height:64px;overflow:hidden"><div style="left:-10px;width:64px;height:64px"><div style="left:4px;top:4px;width:48px;height:0;border-bottom:2px solid red"></div><div style="width:64px;height:64px;opacity:.5"><div style="left:4px;top:24px;width:48px;height:0;border-bottom:2px solid rgba(255,0,0,.5)"></div></div></div></div>',
    probes: [
      [16, 4],
      [16, 24],
      [9, 4],
      [40, 24],
    ],
  },
]
const requested = values.cases?.split(',')
assert(
  !requested || requested.every(id => cases.some(c => c.id === id)),
  'unknown case'
)
await mkdir(out, { recursive: true })
const browser = await chromium.launch({
  headless: true,
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  args: values.gpu
    ? [
        '--enable-gpu',
        '--enable-gpu-rasterization',
        ...(process.platform === 'linux' ? ['--use-angle=vulkan'] : []),
      ]
    : [],
})
let browserRenderer
const results = []
try {
  const browserSession = await browser.newBrowserCDPSession()
  const { gpu } = await browserSession.send('SystemInfo.getInfo')
  browserRenderer = {
    requestedGpu: values.gpu,
    version: browser.version(),
    devices: gpu.devices,
    features: gpu.featureStatus,
    backend: gpu.auxAttributes?.skiaBackendType,
    renderer: gpu.auxAttributes?.glRenderer,
  }
  await writeFile(
    join(out, 'browser.json'),
    JSON.stringify(browserRenderer, null, 2) + '\n'
  )
  if (values.gpu) {
    const software =
      /swiftshader|llvmpipe|lavapipe|software|Microsoft Basic Render Driver|\bWARP\b/i
    assert(
      gpu.featureStatus.gpu_compositing?.startsWith('enabled') &&
        gpu.featureStatus.rasterization?.startsWith('enabled') &&
        gpu.devices.length > 0 &&
        typeof browserRenderer.renderer === 'string' &&
        browserRenderer.renderer.length > 0 &&
        !software.test(browserRenderer.renderer) &&
        gpu.devices.some(d => !software.test(d.deviceString)),
      `browser did not enable hardware rasterization: ${JSON.stringify(browserRenderer)}`
    )
  }

  const page = await browser.newPage({
    viewport: { width: 64, height: 64 },
    deviceScaleFactor: 1,
  })
  for (const { id, html, probes, imagePixels, fontProbe } of cases) {
    if (requested && !requested.includes(id)) continue
    await page.setContent(
      `<style>html,body{margin:0;width:64px;height:64px}div,img{position:absolute}</style>${html}`
    )
    if (fontProbe) {
      const data = await readFile(
        new URL('../../compat/gpui/fixtures/colr-alpha.ttf', import.meta.url)
      )
      await page.evaluate(async base64 => {
        const font = new FontFace(
          'DazzUIProbe',
          `url(data:font/ttf;base64,${base64})`
        )
        document.fonts.add(await font.load())
        await document.fonts.ready
      }, data.toString('base64'))
    }
    if (imagePixels) {
      await page.evaluate(async pixels => {
        const canvas = document.createElement('canvas')
        canvas.width = 2
        canvas.height = 1
        canvas
          .getContext('2d')
          .putImageData(
            new ImageData(new Uint8ClampedArray(pixels), 2, 1),
            0,
            0
          )
        const src = canvas.toDataURL()
        await Promise.all(
          Array.from(document.images, async image => {
            image.src = src
            await image.decode()
          })
        )
      }, imagePixels)
    }
    const css = await page.screenshot({
      path: join(out, `${id}-css.png`),
      omitBackground: true,
    })
    const gpu = await readFile(join(native, `${id}.png`))
    await writeFile(join(out, `${id}-native.png`), gpu)
    const result = await page.evaluate(
      async ({ css, gpu, probes }) => {
        const load = async base64 => {
          const image = new Image()
          image.src = `data:image/png;base64,${base64}`
          await image.decode()
          if (image.width !== 64 || image.height !== 64)
            throw new Error('expected a 64×64 probe PNG')
          const canvas = document.createElement('canvas')
          canvas.width = canvas.height = 64
          const ctx = canvas.getContext('2d', { willReadFrequently: true })
          ctx.drawImage(image, 0, 0)
          return ctx.getImageData(0, 0, 64, 64).data
        }
        const [expected, actual] = await Promise.all([load(css), load(gpu)])
        let differentPixels = 0,
          maxChannelDifference = 0,
          maxPremultipliedDifference = 0,
          maxAlphaDifference = 0
        const differences = []
        for (let i = 0; i < expected.length; i += 4) {
          const delta = Math.max(
            ...Array.from({ length: 4 }, (_, c) =>
              Math.abs(actual[i + c] - expected[i + c])
            )
          )
          if (delta) {
            differentPixels++
            if (differences.length < 8)
              differences.push({
                x: (i / 4) % 64,
                y: Math.floor(i / 256),
                css: Array.from(expected.slice(i, i + 4)),
                renderer: Array.from(actual.slice(i, i + 4)),
              })
          }
          maxChannelDifference = Math.max(maxChannelDifference, delta)
          maxAlphaDifference = Math.max(
            maxAlphaDifference,
            Math.abs(actual[i + 3] - expected[i + 3])
          )
          for (let c = 0; c < 3; c++)
            maxPremultipliedDifference = Math.max(
              maxPremultipliedDifference,
              Math.abs(
                (actual[i + c] * actual[i + 3]) / 255 -
                  (expected[i + c] * expected[i + 3]) / 255
              )
            )
        }
        return {
          differentPixels,
          totalPixels: 4096,
          maxChannelDifference,
          maxPremultipliedDifference,
          maxAlphaDifference,
          differences,
          probes: probes.map(([x, y]) => {
            const i = (y * 64 + x) * 4
            return {
              x,
              y,
              css: Array.from(expected.slice(i, i + 4)),
              renderer: Array.from(actual.slice(i, i + 4)),
              pass: Array.from(
                { length: 4 },
                (_, c) => Math.abs(actual[i + c] - expected[i + c]) <= 1
              ).every(Boolean),
            }
          }),
        }
      },
      { css: css.toString('base64'), gpu: gpu.toString('base64'), probes }
    )
    results.push({ id, ...result })
  }
} finally {
  await browser.close()
}
await writeFile(
  join(out, 'report.json'),
  JSON.stringify(
    {
      note: 'Selected probes gate semantics; full-image differences are diagnostic.',
      browser: browserRenderer,
      results,
    },
    null,
    2
  ) + '\n'
)
for (const result of results) {
  console.log(
    `${result.id}: ${result.probes.filter(p => p.pass).length}/${result.probes.length} probes; ${result.differentPixels}/4096 pixels differ, max channel delta ${result.maxChannelDifference}`
  )
}
assert(
  results.every(r => r.probes.every(p => p.pass)),
  `probe mismatch: ${out}/report.json`
)
