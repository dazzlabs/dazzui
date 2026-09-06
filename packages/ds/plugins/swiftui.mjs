import YAML from 'yaml'

/**
 * Terrazzo plugin to generate the SwiftUI token sources.
 *
 * The Flutter plugin beside it is the model, and the two solve the same
 * problem the same way: neither platform has a cascade, so a theme cannot be
 * a block of overrides layered onto a base one at render time. Each theme is
 * instead resolved whole at build time into its own `ThemeVariables` value,
 * and a token that never moves is simply absent from it.
 *
 * Two files come out:
 *
 * - `Colors.swift` — the `base.color.*` ramps as `ColorSwatch` constants
 * - `ThemeVariables.swift` — the semantic and component layers as one struct,
 *   plus the instance per theme
 *
 * The split between a stored property and a computed one is forced rather
 * than chosen: a token whose value names another token has to read that
 * property, and a default argument in an initialiser cannot reach `self`. So
 * anything aliasing a non-`base` token becomes a computed property, which
 * follows whichever instance it is read from — that is what lets a theme
 * re-point `colorSurface` and have every recipe built on it follow without
 * the generator emitting a single line about it.
 */
class SwiftUIPlugin {
  constructor(options = {}) {
    this.modeSelectors = options.modeSelectors ?? []
  }

  get name() {
    return '@dazzlabs/dazzui-ds-plugin-swiftui'
  }

  async build(context) {
    const { tokens, sources, outputFile } = context

    outputFile('swiftui/Colors.swift', generateColorsFile(tokens, sources))
    outputFile(
      'swiftui/ThemeVariables.swift',
      generateThemeVariablesFile(tokens, sources, this.modeSelectors)
    )
  }
}

export default function swiftUIPlugin(options) {
  return new SwiftUIPlugin(options)
}

const HEADER = `// Design tokens generated from Terrazzo
// DO NOT EDIT - This file is auto-generated`

const STATES = ['normal', 'hovered', 'pressed', 'disabled']
const PALETTE_SHADE = /^color\.([^.]+)\.(\d+)$/
const BASE_RAMP_STEP = /^base\.color\.([^.]+)\.(\d+)$/
const RECIPE_LEAF =
  /^(control\.color\.[^.]+\.[^.]+)\.(normal|hovered|pressed|disabled)\.(shade|alpha|color)$/

// MARK: - Colors.swift

/** Every `base.color.<ramp>.<shade>` in the graph becomes a `ColorSwatch`. */
function generateColorsFile(tokens, sources) {
  const ramps = new Map()

  for (const [id, token] of Object.entries(tokens)) {
    const step = BASE_RAMP_STEP.exec(id)
    if (!step || token.$type !== 'color') continue
    const [, ramp, shade] = step
    if (!ramps.has(ramp)) ramps.set(ramp, new Map())
    ramps.get(ramp).set(Number(shade), token)
  }

  const lines = [
    'import SwiftUI',
    '',
    `/// Primitive color tokens.`,
    `///`,
    `/// ${HEADER.split('\n')[1].replace('// ', '')}`,
    'public enum Colors {',
    '  /// Completely invisible.',
    '  public static let transparent = Color(argb: 0x0000_0000)',
    '',
    '  /// Completely opaque black.',
    '  public static let black = Color(argb: 0xFF00_0000)',
    '',
    '  /// Completely opaque white.',
    '  public static let white = Color(argb: 0xFFFF_FFFF)',
    '',
  ]

  for (const [ramp, shades] of [...ramps].sort()) {
    if (ramp === 'black' || ramp === 'white') continue

    lines.push(`  public static let ${swiftName(ramp)} = ColorSwatch([`)
    for (const shade of [...shades.keys()].sort((a, b) => a - b)) {
      lines.push(`    ${shade}: ${colorLiteral(shades.get(shade).$value)},`)
    }
    lines.push('  ])', '')
  }

  lines.push('}')
  return `${lines.join('\n')}\n`
}

// MARK: - ThemeVariables.swift

function generateThemeVariablesFile(tokens, sources, modeSelectors) {
  const categories = categorizeTokensBySource(tokens, sources)
  const { stored, computed } = buildMembers(categories, tokens)

  const params = stored.map(
    member => `    ${member.name}: ${member.type} = ${member.value}`
  )
  const assignments = stored.map(
    member => `    self.${member.name} = ${member.name}`
  )

  return `import SwiftUI

/// The complete token set for one theme.
///
/// ${HEADER.split('\n')[1].replace('// ', '')}
public struct ThemeVariables: Sendable {
  public init(
${params.join(',\n')}
  ) {
${assignments.join('\n')}
  }

${stored
  .map(member => `${member.doc}  public let ${member.name}: ${member.type}`)
  .join('\n')}

${computed
  .map(
    member =>
      `${member.doc}  public var ${member.name}: ${member.type} { ${member.value} }`
  )
  .join('\n')}
}
${generateThemeInstances(tokens, modeSelectors, stored)}`
}

/**
 * Split every token into a stored property or a computed one.
 *
 * The test is what the token's value *names*, not what it resolves to: a
 * value naming another semantic token has to follow that token per theme, and
 * only a computed property can. Anything else is stored, so a theme can pass
 * it to the initialiser.
 */
function buildMembers(categories, tokens) {
  const stored = []
  const computed = []
  const seen = new Set()

  const recipes = new Map()
  const ramps = collectSemanticRamps(tokens)

  for (const category of orderedCategories(categories)) {
    for (const subcategory of Object.keys(category.subcategories).sort()) {
      for (const token of category.subcategories[subcategory].tokens) {
        const { id } = token
        if (id.startsWith('base.') || seen.has(id)) continue
        seen.add(id)

        const palette = PALETTE_SHADE.exec(id)
        if (palette) {
          const ramp = ramps.get(palette[1])
          if (!ramp || ramp.emitted) continue
          ramp.emitted = true
          stored.push({
            doc: '',
            name: `color${capitalize(palette[1])}`,
            type: 'ColorSwatch',
            value: `Colors.${swiftName(ramp.base)}`,
          })
          continue
        }

        const leaf = RECIPE_LEAF.exec(id)
        if (leaf) {
          const [, base, state, part] = leaf
          if (!recipes.has(base)) recipes.set(base, { states: {}, doc: '' })
          const recipe = recipes.get(base)
          recipe.states[state] ??= {}
          recipe.states[state][part] = token
          if (!recipe.doc) recipe.doc = docComment(token)
          continue
        }

        const value = convertTokenValue(token, tokens)
        const member = {
          doc: docComment(token),
          name: tokenIdToVarName(id),
          type: value.type,
          value: value.value,
        }
        ;(value.references ? computed : stored).push(member)
      }
    }
  }

  for (const [base, recipe] of recipes) {
    // A recipe naming a semantic role — `{color.surface}` for the neutral
    // control, `{color.on-accent}` for what a filled one prints — reads a
    // property of this same value, which an initialiser default cannot do.
    // Such a recipe is computed; but a *shade* on it that a theme re-picks
    // (`tinted` reads 700 on paper and 300 on a dark canvas) is stored on
    // its own, so the theme has a parameter to re-point and the computed
    // descriptor reads the step this instance carries.
    if (!descriptorReferences(recipe.states)) {
      stored.push({
        doc: recipe.doc,
        name: tokenIdToVarName(base),
        type: 'ColorDescriptor',
        value: descriptorLiteral(recipe.states, tokens),
      })
      continue
    }
    const shadeNames = {}
    for (const state of STATES) {
      const shade = recipe.states[state]?.shade
      if (!shade || !movesPerTheme(shade)) continue
      const name = tokenIdToVarName(`${base}.${state}.shade`)
      shadeNames[state] = name
      stored.push({
        doc: `  /// The ramp step \`${tokenIdToVarName(base)}\` picks when ${state}; a theme re-points it.\n`,
        name,
        type: 'Int',
        value: String(shade.$value),
      })
    }
    computed.push({
      doc: recipe.doc,
      name: tokenIdToVarName(base),
      type: 'ColorDescriptor',
      value: descriptorLiteral(recipe.states, tokens, '  ', shadeNames),
    })
  }

  return { stored, computed }
}

/** Does a token take a different value under any mode? */
function movesPerTheme(token) {
  return Object.keys(token.mode ?? {}).some(mode => mode !== '.')
}

/**
 * The semantic ramps (`color.primary.*` and friends), each with the base ramp
 * it aliases. A ramp is one `ColorSwatch` property rather than eleven colors,
 * so it can only be emitted when every step points at the same base ramp.
 */
function collectSemanticRamps(tokens) {
  const ramps = new Map()

  for (const [id, token] of Object.entries(tokens)) {
    const palette = PALETTE_SHADE.exec(id)
    if (!palette) continue

    const base = BASE_RAMP_STEP.exec(refId(originalOf(token)) ?? '')
    if (!ramps.has(palette[1])) {
      ramps.set(palette[1], { base: base?.[1] ?? null, bases: new Set() })
    }
    ramps.get(palette[1]).bases.add(base?.[1] ?? null)
  }

  for (const [group, ramp] of ramps) {
    if (ramp.bases.size !== 1 || ramp.base === null) {
      console.warn(
        `Ramp color.${group} does not alias one base ramp; ` +
          'a ramp is one ColorSwatch property, so it has to move whole. ' +
          'Skipping it in the generated Swift.'
      )
      ramps.delete(group)
    }
  }

  return ramps
}

/** One `ThemeVariables` per theme, beside the Studio Light default. */
function generateThemeInstances(tokens, modeSelectors, stored) {
  const themes = collectThemes(modeSelectors)
  // Swift matches an initialiser's arguments in declaration order, so an
  // override list built in token order would not compile. Each theme's
  // overrides are collected by property name and sorted back into the order
  // the initialiser declares them.
  const declarationOrder = new Map(stored.map((member, i) => [member.name, i]))
  const ramps = collectSemanticRamps(tokens)
  const rampSizes = new Map()
  for (const id of Object.keys(tokens)) {
    const palette = PALETTE_SHADE.exec(id)
    if (palette) {
      rampSizes.set(palette[1], (rampSizes.get(palette[1]) ?? 0) + 1)
    }
  }

  const blocks = [
    `
public extension ThemeVariables {
  /// Studio Light — the baseline every other theme is written against.
  static let studioLight = ThemeVariables()`,
  ]

  for (const [theme, modes] of themes) {
    const overrides = []
    const movedRamps = new Map()
    const movedRecipes = new Map()

    for (const [id, token] of Object.entries(tokens)) {
      if (id.startsWith('base.')) continue

      const palette = PALETTE_SHADE.exec(id)
      if (palette) {
        const themed = valueForTheme(token, modes)
        if (!themed) continue
        const base = BASE_RAMP_STEP.exec(refId(originalOf(themed)) ?? '')
        if (!movedRamps.has(palette[1])) {
          movedRamps.set(palette[1], { bases: new Set(), moved: 0 })
        }
        const moved = movedRamps.get(palette[1])
        moved.bases.add(base?.[1] ?? null)
        moved.moved += 1
        continue
      }

      const leaf = RECIPE_LEAF.exec(id)
      if (leaf) {
        if (valueForTheme(token, modes)) movedRecipes.set(leaf[1], true)
        continue
      }

      const themed = valueForTheme(token, modes)
      if (!themed) continue

      const value = convertTokenValue(
        { ...token, $value: themed.$value, $originalValue: originalOf(themed) },
        tokens
      )
      if (value.references) continue
      overrides.push({ name: tokenIdToVarName(id), value: value.value })
    }

    for (const [group, moved] of [...movedRamps].sort()) {
      const [base] = [...moved.bases]
      if (
        moved.bases.size !== 1 ||
        base === null ||
        moved.moved !== rampSizes.get(group) ||
        !ramps.has(group)
      ) {
        console.warn(
          `Ramp color.${group} moves only part-way under "${theme}"; ` +
            'a ramp is one ColorSwatch property, so it has to move whole. ' +
            'Skipping it in the generated Swift.'
        )
        continue
      }
      overrides.push({
        name: `color${capitalize(group)}`,
        value: `Colors.${swiftName(base)}`,
      })
    }

    for (const base of [...movedRecipes.keys()].sort()) {
      const states = {}
      for (const state of STATES) {
        for (const part of ['shade', 'alpha', 'color']) {
          const token = tokens[`${base}.${state}.${part}`]
          if (!token) continue
          states[state] ??= {}
          const themed = valueForTheme(token, modes)
          states[state][part] = themed
            ? {
                ...token,
                $value: themed.$value,
                $originalValue: originalOf(themed),
              }
            : token
        }
      }
      // A recipe emitted as a computed property has no initialiser parameter
      // of its own: it reads the roles this instance already re-points. The
      // shades a theme re-picks on it are stored members, and those are
      // what move here.
      if (descriptorReferences(states)) {
        for (const state of STATES) {
          const shade = tokens[`${base}.${state}.shade`]
          const themed = shade && valueForTheme(shade, modes)
          if (!themed) continue
          overrides.push({
            name: tokenIdToVarName(`${base}.${state}.shade`),
            value: String(themed.$value),
          })
        }
        continue
      }
      overrides.push({
        name: tokenIdToVarName(base),
        value: descriptorLiteral(states, tokens, '    '),
      })
    }

    if (overrides.length === 0) continue

    overrides.sort(
      (a, b) =>
        (declarationOrder.get(a.name) ?? Infinity) -
        (declarationOrder.get(b.name) ?? Infinity)
    )

    blocks.push(`
  /// The \`${theme}\` counterpart of \`\`studioLight\`\`.
  ///
  /// Every token that moves under this theme is named here; the rest fall
  /// through to the initialiser's defaults, and the computed properties
  /// follow this instance.
  static let ${swiftName(theme)} = ThemeVariables(
${overrides.map(o => `    ${o.name}: ${o.value}`).join(',\n')}
  )`)
  }

  return `${blocks.join('\n')}\n}\n`
}

/**
 * Which modes apply to each theme, in the order they would cascade in CSS.
 *
 * Read back out of the same `modeSelectors` the stylesheets use, so the
 * platforms cannot drift: a theme selected by two modes gets both, later
 * winning, exactly as the equal-specificity CSS blocks would.
 */
function collectThemes(modeSelectors) {
  const themes = new Map()

  for (const { mode, selectors = [] } of modeSelectors) {
    for (const selector of selectors) {
      const match = /\[data-theme='([^']+)'\]/.exec(selector)
      if (!match) continue
      if (!themes.has(match[1])) themes.set(match[1], [])
      themes.get(match[1]).push(mode)
    }
  }

  return themes
}

/** The value a token takes under one theme, or undefined if it never moves. */
function valueForTheme(token, modes) {
  let picked
  for (const mode of modes) {
    if (token.mode?.[mode]) picked = token.mode[mode]
  }
  return picked
}

// MARK: - Color descriptors

function descriptorLiteral(states, tokens, indent = '  ', shadeNames = {}) {
  const params = []

  for (const state of STATES) {
    const parts = states[state]
    if (!parts) continue

    if (parts.color) {
      const value = convertTokenValue(parts.color, tokens)
      params.push(`${indent}  ${state}Color: ${value.value}`)
    }
    if (parts.shade) {
      params.push(
        `${indent}  ${state}Shade: ${shadeNames[state] ?? parts.shade.$value}`
      )
    }
    if (parts.alpha) {
      params.push(`${indent}  ${state}Opacity: ${double(parts.alpha.$value)}`)
    }
  }

  return `ColorDescriptor(\n${params.join(',\n')}\n${indent})`
}

/** Does any state of this recipe name a semantic role rather than a ramp? */
function descriptorReferences(states) {
  return STATES.some(state => {
    const color = states[state]?.color
    if (!color) return false
    const ref = refId(originalOf(color))
    return ref !== null && !ref.startsWith('base.')
  })
}

// MARK: - Value conversion

function convertTokenValue(token, tokens) {
  const original = originalOf(token)

  switch (token.$type) {
    case 'color': {
      const ref = refId(original)
      if (ref) {
        const step = BASE_RAMP_STEP.exec(ref)
        if (step) {
          return {
            type: 'Color',
            value: `Colors.${swiftName(step[1])}[${step[2]}]`,
          }
        }
        if (/^base\.color\.[^.]+$/.test(ref)) {
          return {
            type: 'Color',
            value: `Colors.${swiftName(ref.split('.')[2])}`,
          }
        }
        const palette = PALETTE_SHADE.exec(ref)
        if (palette) {
          return {
            type: 'Color',
            value: `color${capitalize(palette[1])}[${palette[2]}]`,
            references: true,
          }
        }
        return {
          type: 'Color',
          value: tokenIdToVarName(ref),
          references: true,
        }
      }
      return { type: 'Color', value: colorLiteral(token.$value) }
    }

    case 'dimension':
    case 'spacing':
    case 'fontSize':
    case 'lineHeight': {
      const ref = refId(original)
      if (ref && !ref.startsWith('base.')) {
        return {
          type: 'CGFloat',
          value: tokenIdToVarName(ref),
          references: true,
        }
      }
      return { type: 'CGFloat', value: double(dimension(token.$value)) }
    }

    case 'number': {
      const ref = refId(original)
      if (ref && !ref.startsWith('base.')) {
        const referenced = tokens[ref]
        return {
          type: isIntToken(ref, referenced?.$value) ? 'Int' : 'CGFloat',
          value: tokenIdToVarName(ref),
          references: true,
        }
      }
      return isIntToken(token.id, token.$value)
        ? { type: 'Int', value: String(token.$value) }
        : { type: 'CGFloat', value: double(token.$value) }
    }

    case 'fontWeight':
      return { type: 'Font.Weight', value: fontWeight(token.$value) }

    case 'fontFamily':
      return { type: '[String]', value: fontFamily(token.$value) }

    case 'duration':
      return { type: 'Double', value: double(seconds(token.$value)) }

    case 'cubicBezier': {
      const [x1, y1, x2, y2] = token.$value
      return {
        type: 'BezierCurve',
        value: `BezierCurve(${double(x1)}, ${double(y1)}, ${double(x2)}, ${double(y2)})`,
      }
    }

    case 'shadow': {
      const layers = Array.isArray(token.$value) ? token.$value : [token.$value]
      const shadows = layers.map(
        layer =>
          `TokenShadow(x: ${double(dimension(layer.offsetX))}, ` +
          `y: ${double(dimension(layer.offsetY))}, ` +
          `blur: ${double(dimension(layer.blur))}, ` +
          `spread: ${double(dimension(layer.spread))}, ` +
          `color: ${colorLiteral(layer.color)})`
      )
      return { type: '[TokenShadow]', value: `[${shadows.join(', ')}]` }
    }

    case 'typography': {
      const ref = refId(original)
      if (ref) {
        return {
          type: 'TypeStyle',
          value: tokenIdToVarName(ref),
          references: true,
        }
      }
      return {
        type: 'TypeStyle',
        value: typeStyle(original ?? token.$value, tokens),
      }
    }

    default:
      return { type: 'CGFloat', value: '0' }
  }
}

/**
 * A `TypeStyle` from a typography token.
 *
 * Every face in the graph writes its size and leading as base references, so
 * they inline to literals here; the family stack is carried whole rather than
 * collapsed to its first entry, because on Apple platforms `-apple-system`
 * is a request for the system face rather than a family that can be looked up
 * by name.
 */
function typeStyle(value, tokens) {
  const resolve = (field, fallback) => {
    const raw = value?.[field]
    const ref = refId(raw)
    const token = ref ? tokens[ref] : null
    return token?.$value ?? (ref ? fallback : (raw ?? fallback))
  }

  const size = dimension(resolve('fontSize', 13))
  const leading = dimension(resolve('lineHeight', size))
  const family = fontFamily(resolve('fontFamily', ['-apple-system']))
  const weight = fontWeight(resolve('fontWeight', 400))

  return (
    `TypeStyle(family: ${family}, size: ${double(size)}, ` +
    `weight: ${weight}, lineHeight: ${double(leading)})`
  )
}

// MARK: - Scalars

/** A `$value` that may be a number, a `12px` string, or `{value, unit}`. */
function dimension(value) {
  if (typeof value === 'number') return value
  if (typeof value === 'string') {
    const match = value.match(/^([\d.]+)(rem|em|px)?$/)
    if (match) {
      const num = parseFloat(match[1])
      return match[2] === 'rem' || match[2] === 'em' ? num * 16 : num
    }
    return 0
  }
  if (value && typeof value === 'object' && value.value !== undefined) {
    return value.unit === 'rem' || value.unit === 'em'
      ? value.value * 16
      : value.value
  }
  return 0
}

function seconds(value) {
  if (typeof value === 'object' && value?.value !== undefined) {
    return value.unit === 's' ? value.value : value.value / 1000
  }
  if (typeof value === 'string') {
    const match = value.match(/^([\d.]+)(ms|s)$/)
    if (match) {
      return match[2] === 's'
        ? parseFloat(match[1])
        : parseFloat(match[1]) / 1000
    }
  }
  return 0
}

/** Swift infers `Int` from `1`, so a `CGFloat` literal keeps its point. */
function double(value) {
  const num = Number(value)
  return Number.isInteger(num) ? `${num}.0` : String(num)
}

/** A shade is an index into a ramp; an alpha named `alpha` never is. */
function isIntToken(id, value) {
  if (typeof id === 'string' && id.toLowerCase().includes('alpha')) return false
  return Number.isInteger(value)
}

function fontWeight(value) {
  const weights = {
    100: '.ultraLight',
    200: '.thin',
    300: '.light',
    400: '.regular',
    500: '.medium',
    600: '.semibold',
    700: '.bold',
    800: '.heavy',
    900: '.black',
  }
  return weights[value] ?? '.regular'
}

function fontFamily(value) {
  const families = Array.isArray(value) ? value : [value]
  return `[${families.map(family => `"${family}"`).join(', ')}]`
}

/**
 * `Color(argb: 0xAARRGGBB)`.
 *
 * CSS writes the alpha last and Swift's literal puts it first, so the two
 * halves are swapped rather than passed through — reading the red channel as
 * the alpha turns a 7%-black hairline into a near-opaque slab.
 */
function colorLiteral(value) {
  if (typeof value === 'string' && value.startsWith('#')) {
    const hex = value.slice(1)
    if (hex.length === 3) {
      const [r, g, b] = hex
      return argb(`FF${r}${r}${g}${g}${b}${b}`)
    }
    if (hex.length === 6) return argb(`FF${hex}`)
    if (hex.length === 8) return argb(`${hex.slice(6, 8)}${hex.slice(0, 6)}`)
  }

  if (value && typeof value === 'object' && value.colorSpace) {
    // Terrazzo normalises a colour into whichever space it was written in, so
    // the space is read rather than assumed: a hex literal arrives as sRGB
    // and its components are already the channels, where running those
    // through the OKLCH conversion would read lightness off the red channel.
    const rgb =
      value.colorSpace === 'srgb'
        ? {
            r: value.components[0],
            g: value.components[1],
            b: value.components[2],
          }
        : oklchToRgb(value)

    const bytes = [
      Math.round((value.alpha ?? 1) * 255),
      Math.round(rgb.r * 255),
      Math.round(rgb.g * 255),
      Math.round(rgb.b * 255),
    ]
    return argb(
      bytes
        .map(byte => byte.toString(16).padStart(2, '0'))
        .join('')
        .toUpperCase()
    )
  }

  return argb('FF000000')
}

/** `0xAARRGGBB`, grouped the way Swift reads long hex literals. */
function argb(hex) {
  const upper = hex.toUpperCase()
  return `Color(argb: 0x${upper.slice(0, 4)}_${upper.slice(4)})`
}

function oklchToRgb(oklch) {
  const [L, C, H] = oklch.components
  const hRad = (H * Math.PI) / 180
  const a = C * Math.cos(hRad)
  const b = C * Math.sin(hRad)

  const l_ = L + 0.3963377774 * a + 0.2158037573 * b
  const m_ = L - 0.1055613458 * a - 0.0638541728 * b
  const s_ = L - 0.0894841775 * a - 1.291485548 * b

  const l = l_ * l_ * l_
  const m = m_ * m_ * m_
  const s = s_ * s_ * s_

  return {
    r: clamp(srgbGamma(4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s)),
    g: clamp(
      srgbGamma(-1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s)
    ),
    b: clamp(srgbGamma(-0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s)),
  }
}

function srgbGamma(value) {
  return value <= 0.0031308
    ? 12.92 * value
    : 1.055 * Math.pow(value, 1 / 2.4) - 0.055
}

function clamp(value) {
  return Math.max(0, Math.min(1, value))
}

// MARK: - Names

/** `"{color.surface}"` -> `color.surface`, or null when it is not a reference. */
function refId(value) {
  if (typeof value !== 'string') return null
  const match = value.match(/^\{(.+)\}$/)
  return match ? match[1] : null
}

/**
 * What a token was *written* as, before the graph resolved it.
 *
 * Terrazzo hands this back two ways: the parsed node (`{$value, $extensions}`)
 * on a token, and the bare value on one of its modes. Both are unwrapped
 * here, because every caller wants the reference the author typed rather than
 * the colour it resolved to.
 */
function originalOf(token) {
  const original = token?.$originalValue ?? token?.originalValue
  if (original && typeof original === 'object' && '$value' in original) {
    return original.$value
  }
  return original ?? token?.$value
}

/** `control.color.filled.surface` -> `controlColorFilledSurface`. */
function tokenIdToVarName(id) {
  const words = id.split(/[.\-_\s:]+/).filter(Boolean)
  const name = words
    .map((word, index) => {
      const clean = word.replace(/[^a-zA-Z0-9]/g, '')
      if (!clean) return ''
      return index === 0
        ? clean.charAt(0).toLowerCase() + clean.slice(1)
        : capitalize(clean)
    })
    .join('')

  if (!name) return 'token'
  return /^\d/.test(name) ? `n${name}` : name
}

/** `bright-dark` -> `brightDark`, for a ramp or theme constant. */
function swiftName(name) {
  const words = name.split(/[-_\s]+/).filter(Boolean)
  return words
    .map((word, index) => (index === 0 ? word : capitalize(word)))
    .join('')
}

function capitalize(value) {
  return value.charAt(0).toUpperCase() + value.slice(1)
}

function docComment(token) {
  return token.$description ? `  /// ${token.$description}\n` : ''
}

// MARK: - Source ordering

/**
 * Group tokens by the file they were written in, so the generated struct
 * reads in the same order as `tokens/` does rather than alphabetically.
 */
function categorizeTokensBySource(tokens, sources) {
  const categories = {}
  const origin = {}

  for (const source of sources ?? []) {
    const path = String(source.filename ?? '')
      .replace(/\\/g, '/')
      .replace('file://', '')
    const index = path.lastIndexOf('/tokens/')
    if (index === -1) continue

    const parts = path.slice(index + '/tokens/'.length).split('/')
    const subcategory = parts[parts.length - 1].replace(/\.ya?ml$/, '')
    const categoryPath = parts.slice(0, -1).join('/')
    if (!categoryPath) continue

    try {
      collect(YAML.parse(source.src), '', id => {
        origin[id] = { categoryPath, subcategory }
      })
    } catch (error) {
      console.warn(`Failed to parse ${path}`, error.message)
    }
  }

  for (const [id, token] of Object.entries(tokens)) {
    const { categoryPath, subcategory } = origin[id] ?? {
      categoryPath: 'other',
      subcategory: 'uncategorized',
    }

    categories[categoryPath] ??= { subcategories: {} }
    categories[categoryPath].subcategories[subcategory] ??= { tokens: [] }
    categories[categoryPath].subcategories[subcategory].tokens.push({
      id,
      ...token,
      $originalValue: token.$originalValue ?? token.originalValue,
    })
  }

  return categories
}

function collect(node, prefix, visit) {
  for (const [key, value] of Object.entries(node ?? {})) {
    if (key.startsWith('$')) continue
    if (!value || typeof value !== 'object') continue

    const id = prefix ? `${prefix}.${key}` : key
    if (value.$value !== undefined) visit(id)
    else collect(value, id, visit)
  }
}

function orderedCategories(categories) {
  const order = ['global', 'component']
  const keys = [
    ...order.filter(key => categories[key]),
    ...Object.keys(categories).filter(
      key => !order.includes(key) && key !== 'other'
    ),
  ]
  if (categories.other) keys.push('other')
  return keys.map(key => categories[key])
}
