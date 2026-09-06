/**
 * The React reference, made machine-readable.
 *
 * `packages/ui_react` is the standard every other platform is held to, but
 * the standard is written in stylesheets, and a port reads a stylesheet with
 * its eyes. This script reads it with a browser instead: it opens every story
 * the Storybook build lists, in every theme, and records what actually
 * rendered — the box, the border, the corner, the colours, the type and the
 * shadow of every component root on the page — plus a screenshot. That is the
 * basis the platform tests compare themselves against, and it is what a
 * screenshot is made of, minus the parts a screen is needed for.
 *
 * Three outputs, staged under `parity/` and then distributed into the
 * platform packages. None of it is committed — it is a measurement, not a
 * contract, and this script regenerates it:
 *
 *   parity/stories.json          the story list — the cross-platform key
 *   parity/reference/<page>.json what every story rendered, per theme
 *   parity/screenshots/<theme>/  one PNG per story, for the report
 *
 * The story list is the contract's index: a platform's own registry is held
 * to it, so a story added on the React side is a story the ports have to
 * draw. The reference is the contract's body: the numbers a platform test
 * measures itself against. It leaves out each component's `Overview` — that
 * is the documented page, prose and specimens around the component rather
 * than the component, and the bare stories beside it are where a state is
 * drawn on its own. The screenshots are not distributed — they are large and
 * only reproducible on one machine, so they stay staging and feed the
 * side-by-side report.
 *
 * Run `pnpm parity` from the root, which builds the Storybook first; or
 * `node scripts/parity.mjs` here against an existing `dist/`.
 *
 *   --only <text>        only stories whose id contains the text
 *   --theme <name>       one theme rather than all four
 *   --no-screenshots     numbers only
 *   --no-distribute      stage, but leave the platform packages alone
 *   --distribute-only    copy what is already staged; extract nothing
 */

import fs from 'node:fs/promises'
import http from 'node:http'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { chromium } from 'playwright'
import { THEMES } from '../src/lib/themes.ts'

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const DIST = path.join(ROOT, 'dist')
const STAGE = path.join(ROOT, 'parity')

/**
 * The window every story is measured in. Bare stories are centred in it and
 * fit; a documented page is taller and is captured whole. 2x because that is
 * the display the design is drawn on and where a hairline halves.
 */
const VIEWPORT = { width: 960, height: 640, scale: 2 }

/**
 * Where each platform's copy lands. The same shape as the token build's
 * `TARGETS`: a platform holds the data rather than reaching across the
 * workspace for it, so its tests run from inside the package. The copies
 * are gitignored there. Adding a platform is one entry here.
 */
const TARGETS = [
  {
    package: 'dazzui_example (flutter)',
    dir: path.join(
      '..',
      '..',
      'packages',
      'ui_flutter',
      'example',
      'test',
      'parity'
    ),
  },
  {
    package: 'Playground (swiftui)',
    // Beside the sources rather than inside them: Xcode's synchronised
    // folder takes everything under `Tests/` as a resource to copy into the
    // bundle, and four themes' worth of drawings share their file names.
    dir: path.join('..', '..', 'packages', 'ui_swiftui', 'Example', 'Parity'),
  },
  {
    package: 'dazzui (gpui)',
    dir: path.join('..', '..', 'packages', 'ui_gpui', 'tests', 'parity'),
  },
]

/**
 * The computed properties a probe records. Lengths are parsed to numbers,
 * colours to hex; everything else is kept as the browser reports it.
 */
const LENGTHS = [
  'paddingTop',
  'paddingRight',
  'paddingBottom',
  'paddingLeft',
  'borderTopWidth',
  'borderRightWidth',
  'borderBottomWidth',
  'borderLeftWidth',
  'borderTopLeftRadius',
  'borderTopRightRadius',
  'borderBottomRightRadius',
  'borderBottomLeftRadius',
  'fontSize',
  'lineHeight',
  'letterSpacing',
  'columnGap',
  'rowGap',
]
const COLORS = [
  'backgroundColor',
  'color',
  'borderTopColor',
  'borderRightColor',
  'borderBottomColor',
  'borderLeftColor',
]
const RAW = [
  'fontFamily',
  'fontWeight',
  'fontStyle',
  'textTransform',
  'boxShadow',
  'opacity',
  'borderTopStyle',
]

function args() {
  const argv = process.argv.slice(2)
  const value = flag => {
    const at = argv.indexOf(flag)
    return at === -1 ? undefined : argv[at + 1]
  }
  return {
    only: value('--only'),
    theme: value('--theme'),
    screenshots: !argv.includes('--no-screenshots'),
    distribute: !argv.includes('--no-distribute'),
    distributeOnly: argv.includes('--distribute-only'),
  }
}

/**
 * A specimen's key: the class and the axes, the way every platform spells
 * it. Text is left out — a badge reading Beta and one reading Failed are the
 * same specimen.
 */
export function specimenKey(probe) {
  const axes = Object.keys(probe.data)
    .sort()
    .map(axis => `${axis}=${probe.data[axis]}`)
  return axes.length === 0 ? probe.class : `${probe.class} ${axes.join(' ')}`
}

/**
 * That key as a file name — the same derivation on every platform, so the
 * viewer can find one port's picture beside another's.
 */
export function renderName(key) {
  return key
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
}

/** Storybook's own id sanitiser, so a platform can derive the same key. */
export function sanitize(text) {
  return text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
}

/** Serve the build: the iframe loads module scripts, which `file:` refuses. */
function serve(dir) {
  const types = {
    '.html': 'text/html',
    '.js': 'text/javascript',
    '.mjs': 'text/javascript',
    '.css': 'text/css',
    '.json': 'application/json',
    '.svg': 'image/svg+xml',
    '.png': 'image/png',
    '.woff2': 'font/woff2',
    '.woff': 'font/woff',
  }
  const server = http.createServer(async (req, res) => {
    const url = new URL(req.url, 'http://localhost')
    const file = path.join(dir, decodeURIComponent(url.pathname))
    try {
      const data = await fs.readFile(file)
      res.writeHead(200, {
        'content-type': types[path.extname(file)] ?? 'application/octet-stream',
      })
      res.end(data)
    } catch {
      res.writeHead(404)
      res.end()
    }
  })
  return new Promise(resolve => {
    server.listen(0, '127.0.0.1', () => {
      const { port } = server.address()
      resolve({
        origin: `http://127.0.0.1:${port}`,
        close: () => server.close(),
      })
    })
  })
}

/**
 * Runs in the page. Every element carrying a bare `ff-*` class is a
 * component root (a part is `dz-x__part`, and is left out), and each is
 * recorded with the attributes that name its axes, so a platform test can
 * find the one instance it is measuring by `data-variant` and `data-size`
 * rather than by position.
 */
function collect({ lengths, colors, raw }) {
  const isRoot = name => /^ff-[a-z0-9]+(-[a-z0-9]+)*$/.test(name)
  const STATE_ATTRIBUTES = new Set([
    'disabled',
    'aria-disabled',
    'aria-checked',
    'aria-selected',
    'aria-expanded',
    'aria-invalid',
  ])

  // The CSS properties behind each field a platform test compares, so the
  // probe can say which of them the component's own stylesheet decides.
  // The shorthands are listed beside the longhands because a shorthand
  // holding a `var()` — `border: var(--stroke-hairline) solid …`, which is
  // how most of this system draws an edge — cannot be read back as its
  // longhands at all: CSSOM leaves them pending until the variable
  // resolves, and answers a longhand query with nothing.
  const DECLARES = {
    background: ['background-color', 'background'],
    'border-color': [
      'border-top-color',
      'border-color',
      'border-top',
      'border',
    ],
    'border-width': [
      'border-top-width',
      'border-width',
      'border-top',
      'border',
    ],
    radius: ['border-top-left-radius', 'border-radius'],
    color: ['color'],
    'font-size': ['font-size', 'font'],
    'font-weight': ['font-weight', 'font'],
  }

  // Every style rule in the page, flattened once. A rule inside a media
  // query counts only while that query matches, which is how the hairline's
  // `min-resolution` block is either in force or not.
  const rules = (window.__parityRules ??= (() => {
    const flat = []
    const walk = list => {
      for (const rule of list) {
        if (rule.media || rule.conditionText) {
          const condition = rule.conditionText ?? rule.media?.mediaText
          if (condition && !window.matchMedia(condition).matches) continue
          if (rule.cssRules) walk(rule.cssRules)
          continue
        }
        if (rule.selectorText && rule.style) flat.push(rule)
      }
    }
    for (const sheet of document.styleSheets) {
      try {
        walk(sheet.cssRules)
      } catch {
        // A sheet from another origin cannot be read; none of ours are.
      }
    }
    return flat
  })())

  /**
   * The fields the element's own stylesheet decides, rather than inherits.
   *
   * A computed style always answers — a callout root reports the page's
   * 13px because it inherits it, having no type of its own — so comparing a
   * port's callout title against it is comparing two different things. Only
   * a field some rule matching this element declares is the component's own
   * decision, and only those are worth holding a port to.
   */
  const declaredCache = (window.__parityDeclared ??= new Map())
  const declaredFields = (el, key) => {
    const cached = declaredCache.get(key)
    if (cached) return cached
    const found = new Set()
    const scan = style => {
      for (const [field, properties] of Object.entries(DECLARES)) {
        if (properties.some(property => style.getPropertyValue(property))) {
          found.add(field)
        }
      }
    }
    scan(el.style)
    for (const rule of rules) {
      let matches = false
      try {
        matches = el.matches(rule.selectorText)
      } catch {
        continue
      }
      if (matches) scan(rule.style)
    }
    const fields = [...found].sort()
    declaredCache.set(key, fields)
    return fields
  }
  const px = value => {
    const number = Number.parseFloat(value)
    return Number.isNaN(number) ? value : Math.round(number * 100) / 100
  }
  // Chromium reports an opaque colour as `rgb()`, a translucent one as
  // `rgba()`, and one that came through `color-mix()` as `color(srgb r g b
  // / a)` with unit channels. All three are the same sRGB value.
  const hex = value => {
    const channel = n => Math.round(n).toString(16).padStart(2, '0')
    const pack = (r, g, b, a) =>
      a >= 1
        ? `#${channel(r)}${channel(g)}${channel(b)}`
        : `#${channel(r)}${channel(g)}${channel(b)}${channel(a * 255)}`
    const unit = raw => {
      if (raw === undefined) return 1
      const number = Number.parseFloat(raw)
      return raw.endsWith('%') ? number / 100 : number
    }
    const rgb = value.match(
      /^rgba?\(\s*(\d+)[\s,]+(\d+)[\s,]+(\d+)(?:[\s,/]+([\d.]+%?))?\s*\)$/
    )
    if (rgb)
      return pack(Number(rgb[1]), Number(rgb[2]), Number(rgb[3]), unit(rgb[4]))
    const srgb = value.match(
      /^color\(srgb\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)(?:\s*\/\s*([\d.]+%?))?\s*\)$/
    )
    if (srgb) {
      return pack(
        Number(srgb[1]) * 255,
        Number(srgb[2]) * 255,
        Number(srgb[3]) * 255,
        unit(srgb[4])
      )
    }
    return value
  }
  const rect = el => {
    const r = el.getBoundingClientRect()
    return {
      x: px(r.x),
      y: px(r.y),
      width: px(r.width),
      height: px(r.height),
    }
  }

  const probes = []
  for (const el of document.body.querySelectorAll('[class]')) {
    const names = [...el.classList].filter(isRoot)
    if (names.length === 0) continue
    const computed = getComputedStyle(el)
    const style = {}
    for (const key of lengths) style[key] = px(computed[key])
    for (const key of colors) style[key] = hex(computed[key])
    for (const key of raw) style[key] = computed[key]
    // The axes a component names, plus the two ways it says it is off.
    // Base UI's own bookkeeping — which trigger owns an open popup, which
    // composite item is active — is left out: it is state of the page,
    // not of the component.
    const internal = new Set([
      'control',
      'popup-open',
      'composite-item-active',
      'activation-direction',
      // This one is the extractor's own: it marks an element so a
      // screenshot can be taken of it, and it outlives the story it was set
      // in because a theme switch re-renders rather than remounts.
      'parity-probe',
    ])
    const data = {}
    for (const { name, value } of el.attributes) {
      if (name.startsWith('data-')) {
        const axis = name.slice(5)
        if (axis.startsWith('base-ui') || internal.has(axis)) continue
        data[axis] = value
      } else if (STATE_ATTRIBUTES.has(name)) {
        // A state a component states in ARIA rather than in a `data-`
        // attribute is a state all the same: an option card says which of a
        // group is chosen with `aria-checked`, and a specimen that folded
        // the chosen one together with the rest would be two drawings under
        // one name.
        data[name] = value || 'true'
      }
    }
    // Marked so a screenshot can be taken of this element and no other.
    el.setAttribute('data-parity-probe', String(probes.length))
    const text = (el.textContent ?? '').trim().replace(/\s+/g, ' ')
    const axes = Object.keys(data)
      .sort()
      .map(axis => `${axis}=${data[axis]}`)
      .join(' ')
    probes.push({
      class: names[0],
      // Where in the page this probe sits, so a screenshot can be taken of
      // that element and no other: two badges in one story are two elements
      // with one class between them.
      index: probes.length,
      // Whether what it holds is built out of elements rather than being one
      // run of text. A port's adapter is given a specimen's axes and its
      // text and nothing else, so it can reproduce a badge's label exactly
      // and a card's heading-over-paragraph not at all — and a picture of
      // the second compares the story's content rather than the component.
      composed: el.children.length > 0,
      declares: declaredFields(el, `${names[0]} ${axes}`),
      tag: el.tagName.toLowerCase(),
      data,
      text: text.length > 60 ? `${text.slice(0, 57)}…` : text,
      rect: rect(el),
      style,
    })
  }

  const root = document.getElementById('storybook-root')
  const render = window.__STORYBOOK_PREVIEW__?.currentRender
  return {
    layout: render?.story?.parameters?.layout ?? null,
    root: root ? rect(root) : null,
    probes,
  }
}

async function main() {
  const options = args()
  if (options.distributeOnly) {
    await distribute()
    return
  }

  let index
  try {
    index = JSON.parse(await fs.readFile(path.join(DIST, 'index.json'), 'utf8'))
  } catch {
    throw new Error(
      'No Storybook build in dist/. Run `pnpm parity` from the root, which builds it first.'
    )
  }

  const stories = Object.values(index.entries)
    .filter(entry => entry.type === 'story')
    .filter(entry => !options.only || entry.id.includes(options.only))
  const themes = THEMES.map(theme => theme.value).filter(
    theme => !options.theme || theme === options.theme
  )
  if (stories.length === 0) throw new Error('No stories matched.')
  if (themes.length === 0) throw new Error(`No theme called ${options.theme}.`)

  const server = await serve(DIST)
  const browser = await chromium.launch()
  const context = await browser.newContext({
    viewport: { width: VIEWPORT.width, height: VIEWPORT.height },
    deviceScaleFactor: VIEWPORT.scale,
    reducedMotion: 'reduce',
    colorScheme: 'light',
  })
  const page = await context.newPage()
  const errors = []
  page.on('pageerror', error => errors.push(error.message))

  // One document for the whole run: the preview switches story and theme
  // over its own channel, which is what the manager does, and saves a page
  // load per story.
  await page.goto(
    `${server.origin}/iframe.html?viewMode=story&id=${stories[0].id}`
  )
  await page.waitForFunction(
    () => window.__STORYBOOK_PREVIEW__?.currentRender?.phase === 'finished'
  )

  const pages = new Map()
  const manifest = []
  // One picture per specimen per theme, from the first story it is in view
  // in — the rest are the same drawing in another arrangement.
  const drawn = new Set()
  let count = 0

  for (const story of stories) {
    const pageId = sanitize(story.title)
    const record = {
      id: story.id,
      title: story.title,
      name: story.name,
      layout: null,
    }
    const themed = {}

    for (const theme of themes) {
      await page.evaluate(
        ([storyId, theme]) => {
          const preview = window.__STORYBOOK_PREVIEW__
          preview.channel.emit('updateGlobals', { globals: { theme } })
          preview.channel.emit('setCurrentStory', {
            storyId,
            viewMode: 'story',
          })
        },
        [story.id, theme]
      )
      await page.waitForFunction(
        ([storyId, theme]) => {
          const preview = window.__STORYBOOK_PREVIEW__
          const render = preview?.currentRender
          const applied = document.documentElement.getAttribute('data-theme')
          const wanted = theme === 'studio-light' ? null : theme
          return (
            render?.id === storyId &&
            render.phase === 'finished' &&
            applied === wanted
          )
        },
        [story.id, theme]
      )
      await page.evaluate(() =>
        document.fonts.ready.then(
          () =>
            new Promise(resolve =>
              requestAnimationFrame(() => requestAnimationFrame(resolve))
            )
        )
      )
      // Hold every animation at its first frame. A spinner is the reason:
      // its box is measured through `getBoundingClientRect`, which reports
      // the *transformed* rect, so a ring caught mid-turn measures its
      // rotated bounding box — 28% wider than the ring — and measures a
      // different width on every run. Paused at zero it is its layout box
      // again, and the screenshot is the same picture every time.
      await page.evaluate(() => {
        for (const animation of document.getAnimations()) {
          animation.currentTime = 0
          animation.pause()
        }
      })

      const result = await page.evaluate(collect, {
        lengths: LENGTHS,
        colors: COLORS,
        raw: RAW,
      })
      record.layout ??= result.layout
      themed[theme] = { root: result.root, probes: result.probes }

      if (options.screenshots) {
        const dir = path.join(STAGE, 'screenshots', theme)
        await fs.mkdir(dir, { recursive: true })
        await page.screenshot({
          path: path.join(dir, `${story.id}.png`),
          fullPage: result.layout !== 'centered',
          animations: 'disabled',
          caret: 'hide',
        })

        // And one picture per specimen, of that element and nothing else.
        //
        // This is the half of the comparison a value table cannot reach. A
        // port measured by its own framework can only report what that
        // framework will tell it, and two of the three will not say what
        // colour a view painted or what face it set a label in. The picture
        // says all of it, and it is the same picture on every platform
        // because every platform draws the same specimen at the same scale.
        const renders = path.join(STAGE, 'renders', theme)
        await fs.mkdir(renders, { recursive: true })
        for (const probe of result.probes) {
          const key = specimenKey(probe)
          const file = path.join(renders, `${renderName(key)}.png`)
          if (drawn.has(`${theme}/${key}`)) continue
          drawn.add(`${theme}/${key}`)
          const element = page.locator(`[data-parity-probe='${probe.index}']`)
          try {
            await element.screenshot({
              path: file,
              animations: 'disabled',
              caret: 'hide',
            })
          } catch {
            // An element clipped out of the viewport cannot be shot; the
            // next story it appears in will be in view.
            drawn.delete(`${theme}/${key}`)
          }
        }
      }
    }

    manifest.push(record)
    if (story.name === 'Overview') continue
    if (!pages.has(pageId))
      pages.set(pageId, { title: story.title, stories: {} })
    pages.get(pageId).stories[story.id] = {
      name: story.name,
      layout: record.layout,
      themes: themed,
    }

    count += 1
    if (count % 25 === 0 || count === stories.length) {
      console.log(`  ${count}/${stories.length} stories`)
    }
  }

  await browser.close()
  server.close()

  if (errors.length > 0) {
    throw new Error(`The preview threw while rendering:\n${errors.join('\n')}`)
  }

  // A full run replaces the staged reference, so a story that left React
  // leaves the staging too; a partial run only rewrites the pages it drew.
  const partial = Boolean(options.only || options.theme)
  if (!partial) {
    await fs.rm(path.join(STAGE, 'reference'), { recursive: true, force: true })
  }
  await fs.mkdir(path.join(STAGE, 'reference'), { recursive: true })
  await fs.writeFile(
    path.join(STAGE, 'stories.json'),
    `${JSON.stringify(
      {
        source: 'apps/storybook',
        viewport: VIEWPORT,
        themes,
        stories: manifest,
      },
      null,
      2
    )}\n`
  )
  for (const [pageId, data] of pages) {
    await fs.writeFile(
      path.join(STAGE, 'reference', `${pageId}.json`),
      serialize(data)
    )
  }
  console.log(
    `✔  ${manifest.length} stories × ${themes.length} themes staged in parity/`
  )

  if (options.distribute && !partial) await distribute()
  else if (options.distribute) {
    console.log(
      '   (a partial run is not distributed — the platform copies stay whole)'
    )
  }
}

/**
 * A reference file, laid out for a diff: the structure indented, and each
 * probe on one line, so a changed padding shows as one changed line under
 * the story and theme it belongs to rather than as a screenful of keys.
 */
function serialize(page) {
  const last = (index, list) => (index < list.length - 1 ? ',' : '')
  const lines = [
    '{',
    `  "title": ${JSON.stringify(page.title)},`,
    '  "stories": {',
  ]
  const stories = Object.entries(page.stories)
  stories.forEach(([id, story], i) => {
    lines.push(`    ${JSON.stringify(id)}: {`)
    lines.push(`      "name": ${JSON.stringify(story.name)},`)
    lines.push(`      "layout": ${JSON.stringify(story.layout)},`)
    lines.push('      "themes": {')
    const themes = Object.entries(story.themes)
    themes.forEach(([theme, data], j) => {
      lines.push(`        ${JSON.stringify(theme)}: {`)
      lines.push(`          "root": ${JSON.stringify(data.root)},`)
      lines.push('          "probes": [')
      data.probes.forEach((probe, k) => {
        lines.push(
          `            ${JSON.stringify(probe)}${last(k, data.probes)}`
        )
      })
      lines.push('          ]')
      lines.push(`        }${last(j, themes)}`)
    })
    lines.push('      }')
    lines.push(`    }${last(i, stories)}`)
  })
  lines.push('  }', '}', '')
  return lines.join('\n')
}

/**
 * Copy the staged contract into each platform package. The reference
 * directory is replaced rather than merged, so a story that left the React
 * side leaves the platforms too.
 */
async function distribute() {
  for (const target of TARGETS) {
    const dir = path.resolve(ROOT, target.dir)
    await fs.rm(path.join(dir, 'reference'), { recursive: true, force: true })
    await fs.mkdir(path.join(dir, 'reference'), { recursive: true })
    await fs.copyFile(
      path.join(STAGE, 'stories.json'),
      path.join(dir, 'stories.json')
    )
    const files = await fs.readdir(path.join(STAGE, 'reference'))
    for (const file of files) {
      await fs.copyFile(
        path.join(STAGE, 'reference', file),
        path.join(dir, 'reference', file)
      )
    }
    console.log(
      `→ stories.json + ${files.length} reference files to ${target.package}`
    )
  }
}

main().catch(error => {
  console.error(error.message)
  process.exit(1)
})
