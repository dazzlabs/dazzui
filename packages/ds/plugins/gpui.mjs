import YAML from 'yaml'

/**
 * Terrazzo plugin to generate the GPUI token sources.
 *
 * The SwiftUI plugin beside it is the model, and the three non-web platforms
 * solve the same problem the same way: none of them has a cascade, so a theme
 * cannot be a block of overrides layered onto a base one at render time. Each
 * theme is instead resolved whole at build time, and a token that never moves
 * is simply absent from it.
 *
 * Two files come out:
 *
 * - `colors.rs` — the `base.color.*` ramps as `ColorSwatch` constants
 * - `theme_variables.rs` — the semantic and component layers as one struct,
 *   plus the instance per theme
 *
 * Where Swift splits a stored property from a computed one, Rust splits the
 * struct in two: `ThemeSeed` carries what a theme may re-point and `resolve`
 * derives everything written in terms of it. That is forced by the language
 * rather than chosen — Rust has no default arguments and no computed
 * properties, so `..STUDIO_LIGHT_SEED` is how a theme names only what moves,
 * and a `const fn` is how the rest follows without the generator emitting a
 * line about it.
 *
 * Everything is `const`, which is what lets every theme be a `static`: a
 * component reads `&'static ThemeVariables` and nothing is allocated or
 * recomputed per frame.
 */
class GPUIPlugin {
  constructor(options = {}) {
    this.modeSelectors = options.modeSelectors ?? []
  }

  get name() {
    return '@dazzlabs/dazzui-ds-plugin-gpui'
  }

  async build(context) {
    const { tokens, sources, outputFile } = context

    outputFile('gpui/colors.rs', generateColorsFile(tokens))
    outputFile(
      'gpui/theme_variables.rs',
      generateThemeVariablesFile(tokens, sources, this.modeSelectors)
    )
  }
}

export default function gpuiPlugin(options) {
  return new GPUIPlugin(options)
}

const HEADER = `// Design tokens generated from Terrazzo
// DO NOT EDIT - This file is auto-generated`

const STATES = ['normal', 'hovered', 'pressed', 'disabled']
const PALETTE_SHADE = /^color\.([^.]+)\.(\d+)$/
const BASE_RAMP_STEP = /^base\.color\.([^.]+)\.(\d+)$/
const RECIPE_LEAF =
  /^(control\.color\.[^.]+\.[^.]+)\.(normal|hovered|pressed|disabled)\.(shade|alpha|color)$/

// MARK: - colors.rs

/** Every `base.color.<ramp>.<shade>` in the graph becomes a `ColorSwatch`. */
function generateColorsFile(tokens) {
  const ramps = new Map()

  for (const [id, token] of Object.entries(tokens)) {
    const step = BASE_RAMP_STEP.exec(id)
    if (!step || token.$type !== 'color') continue
    const [, ramp, shade] = step
    if (!ramps.has(ramp)) ramps.set(ramp, new Map())
    ramps.get(ramp).set(Number(shade), token)
  }

  const lines = [
    // Generated code is not hand-edited, and a ramp reads as a table rather
    // than as eleven five-line structs — so `cargo fmt` is asked to leave it
    // alone rather than fight the generator over it.
    '#![cfg_attr(rustfmt, rustfmt_skip)]',
    '//! Primitive color tokens.',
    '//!',
    `//! ${HEADER.split('\n')[1].replace('// ', '')}`,
    '',
    'use gpui::Hsla;',
    '',
    'use crate::foundation::ColorSwatch;',
    '',
    '/// Completely invisible.',
    `pub const TRANSPARENT: Hsla = ${colorLiteral('#00000000')};`,
    '',
    '/// Completely opaque black.',
    `pub const BLACK: Hsla = ${colorLiteral('#000000')};`,
    '',
    '/// Completely opaque white.',
    `pub const WHITE: Hsla = ${colorLiteral('#ffffff')};`,
    '',
  ]

  for (const [ramp, shades] of [...ramps].sort()) {
    if (ramp === 'black' || ramp === 'white') continue

    lines.push(`pub const ${constName(ramp)}: ColorSwatch = ColorSwatch(&[`)
    for (const shade of [...shades.keys()].sort((a, b) => a - b)) {
      lines.push(`    (${shade}, ${colorLiteral(shades.get(shade).$value)}),`)
    }
    lines.push(']);', '')
  }

  return `${lines.join('\n')}`
}

// MARK: - theme_variables.rs

function generateThemeVariablesFile(tokens, sources, modeSelectors) {
  const categories = categorizeTokensBySource(tokens, sources)
  const { stored, derived } = buildMembers(categories, tokens)

  const seedFields = stored.map(
    member => `${member.doc}    pub ${member.name}: ${member.type},`
  )
  const seedDefaults = stored.map(
    member => `    ${member.name}: ${member.value},`
  )
  const varsFields = [...stored, ...derived].map(
    member => `${member.doc}    pub ${member.name}: ${member.type},`
  )
  // Every seed field is bound as a local first, so a derived value reads the
  // same name whether what it names is seeded or derived itself; and the
  // derived ones are bound in dependency order, since one may be written in
  // terms of another.
  const bindings = [
    ...stored.map(
      member => `        let ${member.name} = seed.${member.name};`
    ),
    '',
    ...orderDerived(derived).map(
      member => `        let ${member.name} = ${member.body ?? member.value};`
    ),
  ]
  const assembly = [...stored, ...derived].map(
    member => `            ${member.name},`
  )

  return `#![cfg_attr(rustfmt, rustfmt_skip)]
//! The complete token set for one theme.
//!
//! ${HEADER.split('\n')[1].replace('// ', '')}

use gpui::{FontWeight, Hsla, Pixels, px};

use super::colors;
use crate::foundation::{
    BezierCurve, ColorDescriptor, ColorRecipe, ColorSwatch, TokenShadow,
    TypeStyle,
};

/// The half of the token set a theme may re-point.
///
/// Rust has no default arguments, so this is how a theme names only what
/// moves: \`..STUDIO_LIGHT_SEED\` fills in the rest. Everything written in
/// terms of these — the control profiles, the component parts, the
/// interactive recipes — is derived by [\`ThemeVariables::resolve\`] and so
/// follows a re-pointed role on its own.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeSeed {
${seedFields.join('\n')}
}

/// Studio Light — the baseline every other theme is written against.
pub const STUDIO_LIGHT_SEED: ThemeSeed = ThemeSeed {
${seedDefaults.join('\n')}
};

/// Every token, for one theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeVariables {
${varsFields.join('\n')}
}

impl ThemeVariables {
    /// Derive the whole token set from the half a theme re-points.
    ///
    /// This is the Rust half of what a cascade does for CSS and a computed
    /// property does for Swift: a role moves once in the seed and everything
    /// written in terms of it follows here.
    pub const fn resolve(seed: ThemeSeed) -> Self {
${bindings.join('\n')}

        Self {
${assembly.join('\n')}
        }
    }
}
${generateThemeInstances(tokens, modeSelectors)}`
}

/**
 * Split every token into a seed field or a derived one.
 *
 * The test is what the token's value *names*, not what it resolves to: a
 * value naming another semantic token has to follow that token per theme, so
 * it is derived. Anything else is a seed field, which is what a theme can
 * re-point.
 */
function buildMembers(categories, tokens) {
  const stored = []
  const derived = []
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
            name: `color_${snakeName(palette[1])}`,
            type: 'ColorSwatch',
            value: `colors::${constName(ramp.base)}`,
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
          references: value.references ?? [],
        }
        ;(value.references ? derived : stored).push(member)
      }
    }
  }

  for (const [base, recipe] of recipes) {
    const references = descriptorReferences(recipe.states)
    // A recipe naming a semantic role — `{color.surface}` for the neutral
    // control, `{color.on-accent}` for what a filled one prints — is written
    // in terms of a value a theme may move, so it is derived rather than
    // seeded. A *shade* on it that a theme re-picks (`tinted` reads 700 on
    // paper and 300 on a dark canvas) is seeded on its own, so the theme has
    // a field to re-point and the derived descriptor reads the step this
    // seed carries.
    if (!references) {
      stored.push({
        doc: recipe.doc,
        name: tokenIdToVarName(base),
        type: 'ColorDescriptor',
        value: descriptorLiteral(recipe.states, tokens),
        references,
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
        doc: `    /// The ramp step \`${tokenIdToVarName(base)}\` picks when ${state}; a theme re-points it.\n`,
        name,
        type: 'u16',
        value: String(shade.$value),
        references: [],
      })
    }
    derived.push({
      doc: recipe.doc,
      name: tokenIdToVarName(base),
      type: 'ColorDescriptor',
      value: descriptorLiteral(recipe.states, tokens, '    ', shadeNames),
      // Bound inside `resolve`, which is two levels in.
      body: descriptorLiteral(recipe.states, tokens, '        ', shadeNames),
      references,
    })
  }

  return { stored, derived }
}

/** Does a token take a different value under any mode? */
function movesPerTheme(token) {
  return Object.keys(token.mode ?? {}).some(mode => mode !== '.')
}

/**
 * Bind the derived values in dependency order.
 *
 * `resolve` is a run of `let` bindings rather than one struct literal, so a
 * value written in terms of another derived one has to come after it. The
 * graph is shallow — nothing in `tokens/` chains more than a step or two —
 * but the order is computed rather than assumed, so a new token that does
 * chain is emitted correctly instead of failing to compile.
 */
function orderDerived(derived) {
  const byName = new Map(derived.map(member => [member.name, member]))
  const ordered = []
  const state = new Map()

  const visit = member => {
    if (state.get(member.name) === 'done') return
    // A cycle cannot be ordered; emitting it in source order at least points
    // the compiler at the token that made one.
    if (state.get(member.name) === 'visiting') return
    state.set(member.name, 'visiting')
    for (const reference of member.references ?? []) {
      const dependency = byName.get(reference)
      if (dependency) visit(dependency)
    }
    state.set(member.name, 'done')
    ordered.push(member)
  }

  for (const member of derived) visit(member)
  return ordered
}

/**
 * The semantic ramps (`color.primary.*` and friends), each with the base ramp
 * it aliases. A ramp is one `ColorSwatch` field rather than eleven colors, so
 * it can only be emitted when every step points at the same base ramp.
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
          'a ramp is one ColorSwatch field, so it has to move whole. ' +
          'Skipping it in the generated Rust.'
      )
      ramps.delete(group)
    }
  }

  return ramps
}

/** One `ThemeVariables` per theme, beside the Studio Light default. */
function generateThemeInstances(tokens, modeSelectors) {
  const themes = collectThemes(modeSelectors)
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
/// Studio Light — white card, light grey canvas, violet accent.
pub static STUDIO_LIGHT: ThemeVariables = ThemeVariables::resolve(STUDIO_LIGHT_SEED);`,
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
            'a ramp is one ColorSwatch field, so it has to move whole. ' +
            'Skipping it in the generated Rust.'
        )
        continue
      }
      overrides.push({
        name: `color_${snakeName(group)}`,
        value: `colors::${constName(base)}`,
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
      // A derived recipe has no seed field of its own: it is written in
      // terms of the roles this theme already re-points. The shades a theme
      // re-picks on it are seed fields, and those are what move here.
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
        value: descriptorLiteral(states, tokens, '        '),
      })
    }

    if (overrides.length === 0) continue

    blocks.push(`
/// The \`${theme}\` counterpart of [\`STUDIO_LIGHT\`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [\`STUDIO_LIGHT_SEED\`], and everything derived from them
/// follows on its own.
pub static ${constName(theme)}: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
${overrides.map(o => `    ${o.name}: ${o.value},`).join('\n')}
    ..STUDIO_LIGHT_SEED
});`)
  }

  return `${blocks.join('\n')}\n`
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

function descriptorLiteral(states, tokens, indent = '    ', shadeNames = {}) {
  const fields = []

  for (const state of STATES) {
    const parts = states[state]

    const color = parts?.color
      ? `Some(${convertTokenValue(parts.color, tokens).value})`
      : 'None'
    const shade = parts?.shade
      ? `Some(${shadeNames[state] ?? parts.shade.$value})`
      : 'None'
    const alpha = parts?.alpha ? `Some(${float(parts.alpha.$value)})` : 'None'

    fields.push(
      `${indent}    ${state}: ColorRecipe ` +
        `{ color: ${color}, shade: ${shade}, alpha: ${alpha} },`
    )
  }

  return `ColorDescriptor {\n${fields.join('\n')}\n${indent}}`
}

/**
 * The names of the semantic roles this recipe reads, or `null` when every
 * state names a ramp step or a literal.
 */
function descriptorReferences(states) {
  const references = []

  for (const state of STATES) {
    const color = states[state]?.color
    if (!color) continue
    const ref = refId(originalOf(color))
    if (ref !== null && !ref.startsWith('base.')) {
      references.push(tokenIdToVarName(ref))
    }
  }

  return references.length > 0 ? references : null
}

// MARK: - Value conversion

/**
 * One token as Rust.
 *
 * `references` carries the names this value reads when it names other
 * semantic tokens, which is both what marks it derived and what orders the
 * bindings in `resolve`.
 */
function convertTokenValue(token, tokens) {
  const original = originalOf(token)

  switch (token.$type) {
    case 'color': {
      const ref = refId(original)
      if (ref) {
        const step = BASE_RAMP_STEP.exec(ref)
        if (step) {
          return {
            type: 'Hsla',
            value: `colors::${constName(step[1])}.shade(${step[2]})`,
          }
        }
        if (/^base\.color\.[^.]+$/.test(ref)) {
          return {
            type: 'Hsla',
            value: `colors::${constName(ref.split('.')[2])}`,
          }
        }
        const palette = PALETTE_SHADE.exec(ref)
        if (palette) {
          const name = `color_${snakeName(palette[1])}`
          return {
            type: 'Hsla',
            value: `${name}.shade(${palette[2]})`,
            references: [name],
          }
        }
        return {
          type: 'Hsla',
          value: tokenIdToVarName(ref),
          references: [tokenIdToVarName(ref)],
        }
      }
      return { type: 'Hsla', value: colorLiteral(token.$value) }
    }

    case 'dimension':
    case 'spacing':
    case 'fontSize':
    case 'lineHeight': {
      const ref = refId(original)
      if (ref && !ref.startsWith('base.')) {
        return {
          type: 'Pixels',
          value: tokenIdToVarName(ref),
          references: [tokenIdToVarName(ref)],
        }
      }
      return { type: 'Pixels', value: `px(${float(dimension(token.$value))})` }
    }

    case 'number': {
      const ref = refId(original)
      if (ref && !ref.startsWith('base.')) {
        const referenced = tokens[ref]
        return {
          type: isShadeToken(ref, referenced?.$value) ? 'u16' : 'f32',
          value: tokenIdToVarName(ref),
          references: [tokenIdToVarName(ref)],
        }
      }
      return isShadeToken(token.id, token.$value)
        ? { type: 'u16', value: String(token.$value) }
        : { type: 'f32', value: float(token.$value) }
    }

    case 'fontWeight':
      return { type: 'FontWeight', value: fontWeight(token.$value) }

    case 'fontFamily':
      return {
        type: "&'static [&'static str]",
        value: fontFamily(token.$value),
      }

    case 'duration':
      return { type: 'f32', value: float(seconds(token.$value)) }

    case 'cubicBezier': {
      const [x1, y1, x2, y2] = token.$value
      return {
        type: 'BezierCurve',
        value:
          `BezierCurve { x1: ${float(x1)}, y1: ${float(y1)}, ` +
          `x2: ${float(x2)}, y2: ${float(y2)} }`,
      }
    }

    case 'shadow': {
      const layers = Array.isArray(token.$value) ? token.$value : [token.$value]
      const shadows = layers.map(
        layer =>
          `TokenShadow { x: px(${float(dimension(layer.offsetX))}), ` +
          `y: px(${float(dimension(layer.offsetY))}), ` +
          `blur: px(${float(dimension(layer.blur))}), ` +
          `spread: px(${float(dimension(layer.spread))}), ` +
          `color: ${colorLiteral(layer.color)} }`
      )
      return {
        type: "&'static [TokenShadow]",
        value: `&[${shadows.join(', ')}]`,
      }
    }

    case 'typography': {
      const ref = refId(original)
      if (ref) {
        return {
          type: 'TypeStyle',
          value: tokenIdToVarName(ref),
          references: [tokenIdToVarName(ref)],
        }
      }
      return {
        type: 'TypeStyle',
        value: typeStyle(original ?? token.$value, tokens),
      }
    }

    default:
      return { type: 'f32', value: '0.0' }
  }
}

/**
 * A `TypeStyle` from a typography token.
 *
 * Every face in the graph writes its size and leading as base references, so
 * they inline to literals here; the family stack is carried whole rather than
 * collapsed to its first entry, because the platform resolves the stack
 * itself — a face whose stack leads with `-apple-system` is a request for the
 * system face rather than a family to look up by name.
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
    `TypeStyle { family: ${family}, size: px(${float(size)}), ` +
    `weight: ${weight}, line_height: px(${float(leading)}) }`
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

/** Rust infers `{integer}` from `1`, so an `f32` literal keeps its point. */
function float(value) {
  const num = Number(value)
  return Number.isInteger(num) ? `${num}.0` : String(num)
}

/** A shade is an index into a ramp; an alpha named `alpha` never is. */
function isShadeToken(id, value) {
  if (typeof id === 'string' && id.toLowerCase().includes('alpha')) return false
  return Number.isInteger(value)
}

function fontWeight(value) {
  const weights = {
    100: 'FontWeight::THIN',
    200: 'FontWeight::EXTRA_LIGHT',
    300: 'FontWeight::LIGHT',
    400: 'FontWeight::NORMAL',
    500: 'FontWeight::MEDIUM',
    600: 'FontWeight::SEMIBOLD',
    700: 'FontWeight::BOLD',
    800: 'FontWeight::EXTRA_BOLD',
    900: 'FontWeight::BLACK',
  }
  return weights[value] ?? 'FontWeight::NORMAL'
}

function fontFamily(value) {
  const families = Array.isArray(value) ? value : [value]
  return `&[${families.map(family => `"${family}"`).join(', ')}]`
}

/**
 * `Hsla { .. }`.
 *
 * GPUI holds a colour as HSL rather than as bytes, so the conversion happens
 * here instead of at read time — the literal is `const`, which is what lets a
 * whole theme be a `static`. It runs through the same 8-bit rounding the
 * other platforms ship at first, so a swatch is the same colour in Rust as it
 * is in CSS, Dart and Swift rather than a hair off it.
 */
function colorLiteral(value) {
  if (typeof value === 'string' && value.startsWith('#')) {
    const hex = value.slice(1)
    if (hex.length === 3) {
      const [r, g, b] = hex
      return hsla(bytes(`${r}${r}${g}${g}${b}${b}FF`))
    }
    if (hex.length === 6) return hsla(bytes(`${hex}FF`))
    if (hex.length === 8) return hsla(bytes(hex))
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

    return hsla([
      Math.round(rgb.r * 255),
      Math.round(rgb.g * 255),
      Math.round(rgb.b * 255),
      Math.round((value.alpha ?? 1) * 255),
    ])
  }

  return hsla([0, 0, 0, 255])
}

/** `RRGGBBAA` -> the four 0–255 channels. */
function bytes(hex) {
  return [0, 2, 4, 6].map(index => parseInt(hex.slice(index, index + 2), 16))
}

/** The four 0–255 channels as GPUI's `Hsla`, hue and all in 0–1. */
function hsla([red, green, blue, alpha]) {
  const r = red / 255
  const g = green / 255
  const b = blue / 255

  const max = Math.max(r, g, b)
  const min = Math.min(r, g, b)
  const delta = max - min
  const l = (max + min) / 2

  let h = 0
  let s = 0

  if (delta !== 0) {
    s = l > 0.5 ? delta / (2 - max - min) : delta / (max + min)
    if (max === r) h = ((g - b) / delta + (g < b ? 6 : 0)) / 6
    else if (max === g) h = ((b - r) / delta + 2) / 6
    else h = ((r - g) / delta + 4) / 6
  }

  const round = value => float(Number(value.toFixed(6)))
  return (
    `Hsla { h: ${round(h)}, s: ${round(s)}, ` +
    `l: ${round(l)}, a: ${round(alpha / 255)} }`
  )
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

/**
 * `control.color.filled.surface` -> `control_color_filled_surface`.
 *
 * A token written in one camelCase word is one Rust word per hump —
 * `labelSmall` is `label_small` — so a face reads the same here as it does in
 * `--label-small-font-size` on the CSS side.
 */
function tokenIdToVarName(id) {
  const name = id
    .split(/[.\-_\s:]+/)
    .map(word => word.replace(/[^a-zA-Z0-9]/g, ''))
    .filter(Boolean)
    .map(word => word.replace(/([a-z0-9])([A-Z])/g, '$1_$2').toLowerCase())
    .join('_')

  if (!name) return 'token'
  return /^\d/.test(name) ? `n${name}` : name
}

/** `bright-dark` -> `bright_dark`. */
function snakeName(name) {
  return name
    .split(/[-_\s]+/)
    .filter(Boolean)
    .map(word => word.toLowerCase())
    .join('_')
}

/** `bright-dark` -> `BRIGHT_DARK`, for a ramp or theme constant. */
function constName(name) {
  return snakeName(name).toUpperCase()
}

function docComment(token) {
  return token.$description ? `    /// ${token.$description}\n` : ''
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
