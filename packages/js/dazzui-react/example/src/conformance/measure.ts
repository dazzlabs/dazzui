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

export function measure(node: HTMLElement): Reading {
  const rect = node.getBoundingClientRect()
  const style = getComputedStyle(node)
  const stroke =
    style.borderTopStyle === 'none' ? 0 : (px(style.borderTopWidth) ?? 0)
  const corner = style.borderTopLeftRadius.endsWith('%')
    ? round((parseFloat(style.borderTopLeftRadius) / 100) * rect.width)
    : px(style.borderTopLeftRadius)
  const flex = /flex|grid/.test(style.display)
  return {
    box: { width: round(rect.width), height: round(rect.height) },
    values: {
      corner,
      stroke,
      strokeColor: stroke ? hex(style.borderTopColor) : null,
      fill: hex(style.backgroundColor),
      ink: hex(style.color),
      fontSize: px(style.fontSize),
      fontWeight: Number(style.fontWeight),
      lineHeight: style.lineHeight === 'normal' ? null : px(style.lineHeight),
      paddingX: px(style.paddingLeft),
      paddingY: px(style.paddingTop),
      gap: flex ? (px(style.columnGap) ?? 0) : null,
      shadow: shadow(style.boxShadow),
    },
  }
}
