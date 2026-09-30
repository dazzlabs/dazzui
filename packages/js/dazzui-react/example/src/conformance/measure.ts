/*
 * A specimen's readings, in the conformance vocabulary rather than CSS's:
 * `border-radius` is `corner`, `background-color` is `fill`, and every
 * colour is sRGB hex. See specs/conformance.md.
 */

export type Values = Record<string, number | string | null>

export interface Reading {
  box: { width: number; height: number }
  values: Values
}

const round = (n: number) => Math.round(n * 100) / 100

const px = (value: string) => {
  const n = parseFloat(value)
  return Number.isFinite(n) ? round(n) : null
}

/* A canvas pixel is the one colour format every browser agrees on: whatever
   the computed style says — oklch, color-mix, display-p3 — is painted and
   read back as sRGB bytes. */
const canvas = document.createElement('canvas')
canvas.width = canvas.height = 1
const context = canvas.getContext('2d', { willReadFrequently: true })!

export function hex(colour: string): string {
  context.clearRect(0, 0, 1, 1)
  context.fillStyle = '#000'
  context.fillStyle = colour
  context.fillRect(0, 0, 1, 1)
  const [r, g, b, a] = context.getImageData(0, 0, 1, 1).data
  const byte = (n: number) => n.toString(16).padStart(2, '0')
  if (a === 0) return '#00000000'
  return `#${byte(r!)}${byte(g!)}${byte(b!)}${a === 255 ? '' : byte(a!)}`
}

/* A shadow is named by the token it resolves to, so a platform that draws
   `shadow-sm` its own way still reads `sm`. Resolved once per theme. */
let shadows: Map<string, string> | undefined
let shadowsTheme: string | null | undefined

function shadowNames(): Map<string, string> {
  const theme = document.documentElement.getAttribute('data-theme')
  if (shadows && shadowsTheme === theme) return shadows
  const names = new Set<string>()
  for (const sheet of document.styleSheets)
    for (const rule of sheet.cssRules)
      if (rule instanceof CSSStyleRule)
        for (const property of rule.style)
          if (property.startsWith('--shadow-')) names.add(property)
  const probe = document.createElement('div')
  document.body.append(probe)
  shadows = new Map()
  for (const name of names) {
    probe.style.boxShadow = `var(${name})`
    const value = getComputedStyle(probe).boxShadow
    if (value !== 'none' && !shadows.has(value))
      shadows.set(value, name.slice('--shadow-'.length))
  }
  probe.remove()
  shadowsTheme = theme
  return shadows
}

function shadow(value: string) {
  if (value === 'none') return 'none'
  return shadowNames().get(value) ?? value
}

/**
 * The first run of text a specimen draws itself — not one inside a
 * specimen nested in it, which is read on its own — and failing that, a
 * glyph it draws, which has an ink but no type.
 */
function firstText(node: HTMLElement, nested: ReadonlySet<Element>) {
  const walker = document.createTreeWalker(
    node,
    NodeFilter.SHOW_ELEMENT | NodeFilter.SHOW_TEXT,
    {
      acceptNode: current =>
        current !== node && nested.has(current as Element)
          ? NodeFilter.FILTER_REJECT
          : NodeFilter.FILTER_ACCEPT,
    }
  )
  let glyph: Element | undefined
  for (let current = walker.nextNode(); current; current = walker.nextNode()) {
    if (current.nodeType === Node.TEXT_NODE) {
      if (current.textContent?.trim() && current.parentElement)
        return { text: getComputedStyle(current.parentElement), glyph }
    } else if (!glyph && current instanceof SVGSVGElement) glyph = current
  }
  return { text: undefined, glyph }
}

export function measure(
  node: HTMLElement,
  nested: ReadonlySet<Element>
): Reading {
  const rect = node.getBoundingClientRect()
  const style = getComputedStyle(node)
  const { text, glyph } = firstText(node, nested)
  const ink = text?.color ?? (glyph && getComputedStyle(glyph).color)
  const stroke =
    style.borderTopStyle === 'none' ? 0 : (px(style.borderTopWidth) ?? 0)
  // A radius past half the short side draws a capsule either way.
  const radius = style.borderTopLeftRadius.endsWith('%')
    ? (parseFloat(style.borderTopLeftRadius) / 100) * rect.width
    : parseFloat(style.borderTopLeftRadius) || 0
  const corner = round(Math.min(radius, Math.min(rect.width, rect.height) / 2))
  const flex = /flex|grid/.test(style.display)
  return {
    box: { width: round(rect.width), height: round(rect.height) },
    values: {
      corner,
      stroke,
      strokeColor: stroke ? hex(style.borderTopColor) : null,
      fill: hex(style.backgroundColor),
      ink: ink ? hex(ink) : null,
      fontSize: text ? px(text.fontSize) : null,
      fontWeight: text ? Number(text.fontWeight) : null,
      lineHeight:
        !text || text.lineHeight === 'normal' ? null : px(text.lineHeight),
      paddingX: px(style.paddingLeft),
      paddingY: px(style.paddingTop),
      gap: flex ? (px(style.columnGap) ?? 0) : null,
      shadow: shadow(style.boxShadow),
    },
  }
}
