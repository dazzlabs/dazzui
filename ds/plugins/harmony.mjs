import { resolveThemes } from './resolve.mjs'

const groups = ['colors', 'metrics', 'typography', 'recipes', 'shadows']
export const dimension = value => {
  if (value.unit === 'rem') return value.value * 16
  if (value.unit === 'px' || value.unit === 'ms') return value.value
  throw new Error(`Unsupported ArkUI dimension: ${JSON.stringify(value)}`)
}

// ArkUI's eight-digit ResourceColor strings are ARGB, not CSS's RGBA.
export function color(bytes, alpha = 1) {
  if (!Array.isArray(bytes) || bytes.length !== 4)
    throw new Error('An ArkUI color requires four ARGB bytes')
  return (
    '#' +
    [Math.round(bytes[0] * alpha), ...bytes.slice(1)]
      .map(byte =>
        Math.max(0, Math.min(255, byte)).toString(16).padStart(2, '0')
      )
      .join('')
      .toUpperCase()
  )
}

export function snapshot(values) {
  const result = Object.fromEntries(groups.map(group => [group, {}]))
  for (const [id, value] of Object.entries(values)) {
    if (
      id.startsWith('control.color.') ||
      id.startsWith('font.') ||
      /^(focus\.(ring|glow)|dialog\.scrim)\./.test(id)
    )
      continue
    if (value?.unit) result.metrics[id] = dimension(value)
    else if (typeof value === 'number') result.metrics[id] = value
    else if (value?.fontFamily)
      result.typography[id] = {
        size: dimension(value.fontSize),
        weight: value.fontWeight,
        leading: dimension(value.lineHeight),
        // A CSS font stack cannot be installed as an ArkUI font. The system UI
        // face supplies HarmonyOS's Latin and CJK glyphs at ds's size and weight.
      }
    else if (Array.isArray(value) && value[0]?.offsetX)
      result.shadows[id] = value
        .filter(shadow => !shadow.inset)
        .map(shadow => ({
          x: dimension(shadow.offsetX),
          y: dimension(shadow.offsetY),
          radius: dimension(shadow.blur),
          color: color(shadow.color),
        }))
    else if (id.startsWith('motion.') && id.endsWith('easing')) {
      for (const [index, coordinate] of ['x1', 'y1', 'x2', 'y2'].entries())
        result.metrics[`${id}.${coordinate}`] = value[index]
    } else if (
      Array.isArray(value) &&
      value.length === 4 &&
      value.every(v => typeof v === 'number')
    )
      result.colors[id] = color(value)
    else throw new Error(`Unsupported ArkUI token: ${id}`)
  }
  for (const id of ['focus.ring', 'focus.glow', 'dialog.scrim']) {
    const shade = values[`${id}.shade`]
    const bytes =
      shade === undefined
        ? values[`${id}.color`]
        : values[`color.primary.${shade}`]
    result.colors[id] = color(bytes, values[`${id}.alpha`] ?? 1)
    if (id === 'focus.glow') {
      for (const tint of [
        'primary',
        'neutral',
        'info',
        'success',
        'warning',
        'danger',
      ]) {
        result.colors[`${id}.${tint}`] = color(
          shade === undefined ? bytes : values[`color.${tint}.${shade}`],
          values[`${id}.alpha`] ?? 1
        )
      }
    }
  }
  for (const variant of [
    'filled',
    'tinted',
    'outlined',
    'normal',
    'recessed',
    'plain',
  ])
    for (const part of ['surface', 'content', 'border'])
      for (const state of ['normal', 'hovered', 'pressed', 'disabled']) {
        const id = `control.color.${variant}.${part}.${state}`
        const shade = values[`${id}.shade`]
        result.recipes[`${variant}.${part}.${state}`] = {
          shade: shade ?? 0,
          color: shade === undefined ? color(values[`${id}.color`]) : '',
          alpha: values[`${id}.alpha`] ?? 1,
        }
      }
  return result
}

export function themeData(themes) {
  const base = snapshot(themes['bright-light'])
  const overrides = Object.fromEntries(
    Object.entries(themes).map(([name, values]) => {
      const resolved = snapshot(values)
      return [
        name,
        Object.fromEntries(
          groups.map(group => [
            group,
            Object.fromEntries(
              Object.entries(resolved[group]).filter(
                ([id, value]) =>
                  JSON.stringify(value) !== JSON.stringify(base[group][id])
              )
            ),
          ])
        ),
      ]
    })
  )
  return { base, overrides }
}

const DRAWN_FAMILIES = [
  'bright',
  'studio',
  'frost',
  'graphite',
  'ember',
  'nocturne',
]

export function generate(themes) {
  const data = themeData(themes)
  const names = Object.keys(themes)
  const nameSet = new Set(names)
  const pairedFamilies = []
  const seenFamilies = new Set()
  const addFamily = family => {
    if (seenFamilies.has(family)) return
    if (!nameSet.has(`${family}-light`) || !nameSet.has(`${family}-dark`))
      return
    seenFamilies.add(family)
    pairedFamilies.push(family)
  }
  for (const family of DRAWN_FAMILIES) addFamily(family)
  for (const name of names) {
    const match = /^(.+)-(light|dark)$/.exec(name)
    if (match) addFamily(match[1])
  }
  const drawnFamilies = DRAWN_FAMILIES.filter(family =>
    seenFamilies.has(family)
  )
  const nameUnion = names.map(name => `'${name}'`).join(' | ')
  const familyUnion = pairedFamilies.map(family => `'${family}'`).join(' | ')

  return `// Generated by ds/plugins/harmony.mjs. Do not edit.
import { ThemeData } from '../foundation/TokenTypes'

export type DazzThemeName = ${nameUnion}
export type DazzThemeFamily = ${familyUnion}
export type DazzThemeGroup = 'drawn' | 'desktops' | 'applications' | 'omarchy'

export const BASE: ThemeData = ${JSON.stringify(data.base, null, 2)}
export const THEMES: Record<string, ThemeData> = ${JSON.stringify(data.overrides, null, 2)}
export const THEME_NAMES: string[] = ${JSON.stringify(names)}
export const THEME_FAMILIES: DazzThemeFamily[] = ${JSON.stringify(pairedFamilies)}
export const DRAWN_THEME_FAMILIES: DazzThemeFamily[] = ${JSON.stringify(drawnFamilies)}

export function resolveThemeName(
  family: DazzThemeFamily,
  dark: boolean
): DazzThemeName {
  return (family + (dark ? '-dark' : '-light')) as DazzThemeName
}
`
}

export default function harmonyPlugin({ modeSelectors = [] } = {}) {
  return {
    name: '@dazzlabs/dazzui-ds-plugin-harmony',
    build({ tokens, outputFile }) {
      outputFile(
        'harmony/ThemeData.ets',
        generate(resolveThemes(tokens, modeSelectors))
      )
    },
  }
}
