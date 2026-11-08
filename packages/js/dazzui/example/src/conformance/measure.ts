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

/* Convert colours through the browser, but paint opaque RGB. An 8-bit canvas
   stores premultiplied channels: painting a 12% wash and unpremultiplying its
   pixel can change a channel by several bytes. Alpha is serialized separately. */
const canvas = document.createElement('canvas')
canvas.width = canvas.height = 1
const context = canvas.getContext('2d', { willReadFrequently: true })!

export function hex(colour: string): string {
  context.clearRect(0, 0, 1, 1)
  context.fillStyle = '#000'
  context.fillStyle = colour
  const normalized = context.fillStyle
  let alpha = 1
  const slash = /\/\s*([\d.eE+-]+%?)\s*\)$/.exec(normalized)
  const rgba = /^rgba\((.*),\s*([\d.eE+-]+%?)\)$/.exec(normalized)
  const channel = (value: string) =>
    parseFloat(value) / (value.endsWith('%') ? 100 : 1)
  if (slash) {
    alpha = channel(slash[1]!)
    context.fillStyle = normalized.replace(slash[0], '/ 1)')
  } else if (rgba) {
    alpha = channel(rgba[2]!)
    context.fillStyle = `rgb(${rgba[1]})`
  } else if (/^#[\da-f]{8}$/i.test(normalized)) {
    alpha = parseInt(normalized.slice(7), 16) / 255
    context.fillStyle = normalized.slice(0, 7)
  }
  context.fillRect(0, 0, 1, 1)
  const [r, g, b] = context.getImageData(0, 0, 1, 1).data
  const a = Math.round(Math.max(0, Math.min(1, alpha)) * 255)
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

/* Read a visible text run's baseline from its actual first line rectangle
   and the browser's metrics for that font. Inputs expose no text rectangle;
   rotated/skewed runs and incompatible fallback metrics remain unread. */
function textBaseline(node: Node, style: CSSStyleDeclaration): number | null {
  const range = document.createRange()
  range.selectNodeContents(node)
  const rect = [...range.getClientRects()].find(
    rect => rect.width > 0 && rect.height > 0
  )
  if (!rect) return null
  let scale = 1
  let zoom = 1
  for (
    let element = node.parentElement;
    element;
    element = element.parentElement
  ) {
    const inherited = getComputedStyle(element)
    if (inherited.transform !== 'none') {
      const matrix = new DOMMatrixReadOnly(inherited.transform)
      if (
        !matrix.is2D ||
        Math.abs(matrix.b) > 1e-6 ||
        Math.abs(matrix.c) > 1e-6 ||
        matrix.a <= 0 ||
        Math.abs(matrix.a - matrix.d) > 1e-6
      )
        return null
      scale *= matrix.d
    }
    if (inherited.scale !== 'none' && inherited.scale !== '1') return null
    const localZoom = Number(inherited.zoom)
    if (Number.isFinite(localZoom) && localZoom > 0) zoom *= localZoom
  }
  // Layout zoom resolves font metrics at the effective font size, including
  // their rounding. A transform only scales those already resolved metrics.
  const size = parseFloat(style.fontSize) * zoom
  context.font = `${style.fontStyle} ${style.fontWeight} ${size}px ${style.fontFamily}`
  const metrics = context.measureText(node.textContent ?? '')
  const height = metrics.fontBoundingBoxAscent + metrics.fontBoundingBoxDescent
  if (!height || Math.abs(rect.height - height * scale) > 0.1) return null
  return round(rect.top + metrics.fontBoundingBoxAscent * scale)
}

/**
 * The first run of text a specimen draws itself — not one inside a
 * specimen nested in it, which is read on its own — and failing that, a
 * glyph it draws, which has an ink but no type.
 */
function firstText(node: HTMLElement, nested: ReadonlySet<Element>) {
  const inputStyle = (element: Node) => {
    if (
      element instanceof HTMLInputElement ||
      element instanceof HTMLTextAreaElement
    ) {
      if (
        (!element.value && !element.placeholder) ||
        element.getAttribute('aria-hidden') === 'true' ||
        (element instanceof HTMLInputElement &&
          ![
            'text',
            'email',
            'password',
            'search',
            'tel',
            'url',
            'number',
          ].includes(element.type))
      )
        return undefined
      const style = getComputedStyle(element)
      return !element.value && element.placeholder
        ? {
            color: getComputedStyle(element, '::placeholder').color,
            fontSize: style.fontSize,
            fontWeight: style.fontWeight,
            lineHeight: style.lineHeight,
          }
        : style
    }
    return undefined
  }
  const input = inputStyle(node)
  if (input) return { text: input, glyph: undefined }
  const walker = document.createTreeWalker(
    node,
    NodeFilter.SHOW_ELEMENT | NodeFilter.SHOW_TEXT,
    {
      acceptNode: current => {
        if (nested.has(current as Element)) return NodeFilter.FILTER_REJECT
        if (current instanceof Element) {
          const style = getComputedStyle(current)
          if (style.display === 'none') return NodeFilter.FILTER_REJECT
          // A descendant can restore visibility, so still walk this subtree.
          if (style.visibility === 'hidden') return NodeFilter.FILTER_SKIP
        } else if (
          current.parentElement &&
          getComputedStyle(current.parentElement).visibility === 'hidden'
        ) {
          return NodeFilter.FILTER_REJECT
        }
        // Base UI's clipped announcement sentinel contains text for assistive
        // technology, but submits no visible glyphs. It is not a painted run.
        if (
          current instanceof Element &&
          getComputedStyle(current).clipPath === 'inset(50%)'
        )
          return NodeFilter.FILTER_REJECT
        return NodeFilter.FILTER_ACCEPT
      },
    }
  )
  // A standalone Icon is itself the SVG; TreeWalker starts at its children.
  let glyph: Element | undefined =
    node instanceof SVGSVGElement ? node : undefined
  for (let current = walker.nextNode(); current; current = walker.nextNode()) {
    const input = inputStyle(current)
    if (input) return { text: input, glyph }
    if (current.nodeType === Node.TEXT_NODE) {
      if (current.textContent?.trim() && current.parentElement) {
        const text = getComputedStyle(current.parentElement)
        return { text, glyph, baseline: textBaseline(current, text) }
      }
    } else if (!glyph && current instanceof SVGSVGElement) glyph = current
  }
  return { text: undefined, glyph }
}

export function measure(
  node: HTMLElement,
  nested: ReadonlySet<Element>
): Reading {
  const rect = node.getBoundingClientRect()
  // Some compound controls put the painted box inside a behaviour root.
  // Read that box rather than reporting its unstyled wrapper as square.
  const surface = node.matches('.dz-number-field')
    ? (node.querySelector<HTMLElement>('.dz-number-field__group') ?? node)
    : node.matches('.dz-shortcut-recorder')
      ? (node.querySelector<HTMLElement>('.dz-shortcut-recorder__button') ??
        node)
      : node
  const style = getComputedStyle(surface)
  const { text, glyph, baseline } = firstText(node, nested)
  // Stroked platform chrome can use its own ink rather than currentColor.
  const shape =
    glyph?.querySelector('path,rect,circle,ellipse,line,polyline,polygon') ??
    glyph
  const glyphStyle = shape && getComputedStyle(shape)
  const glyphInk =
    glyphStyle &&
    (glyphStyle.fill !== 'none'
      ? glyphStyle.fill
      : glyphStyle.stroke !== 'none'
        ? glyphStyle.stroke
        : undefined)
  const ink = text?.color ?? glyphInk
  const border =
    style.borderTopStyle === 'none' ? 0 : (px(style.borderTopWidth) ?? 0)
  // A heavier stroke laid over the border on a layer of its own — the way a
  // chosen OptionCard draws it, so its content does not move — is the stroke
  // the reader sees, and the one the other platforms draw.
  const over = getComputedStyle(surface, '::after')
  const laid =
    over.content !== 'none' &&
    over.position === 'absolute' &&
    over.borderTopStyle !== 'none'
      ? (px(over.borderTopWidth) ?? 0)
      : 0
  const stroke = Math.max(border, laid)
  const strokeStyle = laid > border ? over : style
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
      strokeColor: stroke ? hex(strokeStyle.borderTopColor) : null,
      fill: hex(style.backgroundColor),
      ink: ink ? hex(ink) : null,
      baseline: baseline ?? null,
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
