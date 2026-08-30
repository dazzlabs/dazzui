/**
 * Terrazzo plugin that resolves the interactive color recipes into CSS.
 *
 * `control.color.*` stores an interactive color as a recipe — a shade to pick
 * off a ramp plus an alpha — because a control does not know which hue it
 * carries until it is tinted. CSS cannot index a ramp by a variable, so the
 * recipe cannot be resolved in a hand-written stylesheet without copying shade
 * numbers into it, where they would silently drift from the tokens. Resolving
 * them here keeps the tokens the only place a shade is ever written down.
 *
 * The output has two halves:
 *
 *   1. Ramp bindings — one block per `color.<ramp>.<shade>` family found in
 *      the graph, aliasing that ramp onto `--tint-*`.
 *   2. Resolved recipes — `--control-color-<variant>-<part>-<state>`, each a
 *      finished color built from its own shade and alpha.
 *
 * Both halves land on `[data-tint]` rather than `:root`, and that is load
 * bearing. A custom property holding `var()` is substituted where it is
 * *declared*, not where it is used: `:root { --x: var(--tint-600) }` resolves
 * against `:root`, where no ramp is bound, and every descendant then inherits
 * that dead value no matter what it binds later. Declaring both halves on the
 * same element makes the ramp available exactly where the recipe reads it.
 *
 * The rule this puts on components: anything reading `--control-color-*` has
 * to carry `data-tint`.
 *
 * On top of those, the state machine every interactive control shares. A
 * control that opts in with `data-control` and names a `data-variant` gets
 * `--control-surface` / `--control-content` / `--control-border` tracking its
 * own hover, press, and disabled states, so the component stylesheet reads
 * three variables instead of restating five variants by four states. The set
 * of variants comes from the tokens, so adding one to `color.yaml` reaches
 * every control without a stylesheet being touched.
 *
 * Only genuinely interactive controls opt in: `:hover` would fire on a badge
 * or a card just as readily, and those want the resting state alone.
 *
 * A third, smaller part: any other token that pairs a `color` with an `alpha`
 * — a scrim, a wash — is resolved the same way and emitted on `:root`, since
 * nothing about it depends on a ramp.
 */

const RAMP = /^color\.([^.]+)\.(\d+)$/
const RECIPE =
  /^control\.color\.([^.]+)\.([^.]+)\.(normal|hovered|pressed|disabled)\.(shade|alpha|color)$/
const WASH = /^(?!control\.color\.)(.+)\.(alpha|color)$/
const PROFILE = /^control\.(?!color\.)([^.]+)\.size$/

class ControlPlugin {
  constructor(options = {}) {
    this.modeSelectors = options.modeSelectors ?? []
  }

  get name() {
    return '@dazzlabs/dazzui-ds-plugin-control'
  }

  async build({ tokens, outputFile }) {
    const ramps = collectRamps(tokens)
    const recipes = collectRecipes(tokens)
    const washes = collectWashes(tokens)
    const sizes = collectSizes(tokens)

    if (ramps.size === 0) {
      throw new Error(
        'No color ramps found. Expected tokens shaped like `color.<ramp>.<shade>`.'
      )
    }

    outputFile(
      'control.css',
      render(ramps, recipes, washes, sizes, this.modeSelectors)
    )
  }
}

/**
 * `modeSelectors` mirrors the CSS plugin's option and should be given the
 * same array, so a theme selects the same set of tokens in both files. A
 * recipe only appears under a mode it actually re-picks a shade for.
 */
export default function controlPlugin(options) {
  return new ControlPlugin(options)
}

/** `color.primary.600` … -> Map<'primary', ['50', '100', …]>, in graph order. */
function collectRamps(tokens) {
  const ramps = new Map()

  for (const id of Object.keys(tokens)) {
    const match = RAMP.exec(id)
    if (!match) continue

    const [, ramp, shade] = match
    if (!ramps.has(ramp)) ramps.set(ramp, [])
    ramps.get(ramp).push(shade)
  }

  for (const shades of ramps.values()) {
    shades.sort((a, b) => Number(a) - Number(b))
  }
  return ramps
}

/**
 * Gather the `shade`/`alpha`/`color` leaves back into one entry per
 * variant/part/state.
 */
function collectRecipes(tokens) {
  const recipes = new Map()

  for (const [id, token] of Object.entries(tokens)) {
    const match = RECIPE.exec(id)
    if (!match) continue

    const [, variant, part, state, key] = match
    const name = `${variant}-${part}-${state}`

    let recipe = recipes.get(name)
    if (!recipe) {
      recipe = { variant, part, state }
      recipes.set(name, recipe)
    }
    recipe[key] = token.$value

    // A recipe that picks a different step of the ramp under some theme
    // carries that step as a mode on its own leaf. It has to be resolved
    // per mode rather than left to the cascade: the shade is baked into the
    // emitted value here, so nothing downstream could re-pick it.
    for (const [mode, value] of Object.entries(token.mode ?? {})) {
      if (mode === '.') continue
      if (!recipe.modes) recipe.modes = {}
      recipe.modes[mode] ??= {}
      recipe.modes[mode][key] = value.$value
    }
  }

  return recipes
}

/** `control.<size>.size` -> ['tiny', 'small', …], in graph order. */
function collectSizes(tokens) {
  const sizes = []
  for (const id of Object.keys(tokens)) {
    const match = PROFILE.exec(id)
    if (match) sizes.push(match[1])
  }
  return sizes
}

/**
 * Any other `color` + `alpha` pair in the graph, keyed by the path they share.
 * A lone `color` is left alone: the CSS plugin already emits it verbatim, and
 * re-emitting it without an alpha would only add a synonym.
 */
function collectWashes(tokens) {
  const washes = new Map()

  for (const [id, token] of Object.entries(tokens)) {
    const match = WASH.exec(id)
    if (!match) continue

    const [, path, key] = match
    let wash = washes.get(path)
    if (!wash) {
      wash = {}
      washes.set(path, wash)
    }
    wash[key] = token.$value
  }

  for (const [path, wash] of washes) {
    if (wash.color === undefined || wash.alpha === undefined)
      washes.delete(path)
  }
  return washes
}

/**
 * The finished color for one recipe.
 *
 * A recipe names its base either as a shade off the tinted ramp or as a
 * literal color, and the literal case is read back through the variable the
 * CSS plugin already emitted for it, so a change to that token still lands
 * here without this plugin knowing how colors are serialised.
 */
function resolve(recipe, mode = '.') {
  const override = mode === '.' ? null : recipe.modes?.[mode]
  const source = override ? { ...recipe, ...override } : recipe
  const { variant, part, state, shade, color } = source
  const alpha = source.alpha ?? 1

  let base
  if (shade !== undefined) {
    base = `var(--tint-${shade})`
  } else if (color !== undefined) {
    base = `var(--control-color-${variant}-${part}-${state}-color)`
  } else {
    return null
  }

  // `color-mix` against transparent is how a plain alpha is applied to a color
  // that arrives as a variable — `rgb(from …)` would need the base parsed.
  return applyAlpha(base, alpha)
}

/** `dialog.scrim` -> `--dialog-scrim`. */
function cssName(path) {
  return `--${path.split('.').join('-')}`
}

/** The same alpha application as a recipe, over a color that needs no ramp. */
function applyAlpha(base, alpha) {
  if (alpha === 0) return 'transparent'
  if (alpha === 1) return base
  return `color-mix(in srgb, ${base} ${alpha * 100}%, transparent)`
}

/**
 * Disabled is matched on both the attribute and the property: `:disabled` only
 * applies to form elements, and a control drawn as something else says so with
 * `aria-disabled`. Hover and press then have to exclude both, or a disabled
 * control would still light up under the pointer.
 */
const ENABLED = ':not(:disabled):not([aria-disabled="true"])'

const STATE_SELECTOR = {
  normal: base => [base],
  hovered: base => [`${base}:hover${ENABLED}`],
  pressed: base => [`${base}:active${ENABLED}`],
  disabled: base => [`${base}:disabled`, `${base}[aria-disabled="true"]`],
}

/** The shared surface/content/border state machine, one block per state. */
function renderStateMachine(recipes) {
  const variants = new Map()
  for (const recipe of recipes.values()) {
    if (!variants.has(recipe.variant)) variants.set(recipe.variant, new Map())
    variants.get(recipe.variant).set(`${recipe.part}-${recipe.state}`, recipe)
  }

  const lines = []
  for (const [variant, parts] of variants) {
    const base = `[data-control][data-variant='${variant}']`

    for (const state of Object.keys(STATE_SELECTOR)) {
      const declarations = []
      for (const part of ['surface', 'content', 'border']) {
        const recipe = parts.get(`${part}-${state}`)
        if (!recipe) continue
        declarations.push(
          `  --control-${part}: var(--control-color-${variant}-${part}-${state});`
        )
      }
      if (declarations.length === 0) continue

      lines.push(
        `${STATE_SELECTOR[state](base).join(',\n')} {`,
        ...declarations,
        '}',
        ''
      )
    }
  }
  return lines
}

/**
 * The size half of the shared vocabulary: `[data-size]` binds the matching
 * `control.<size>` profile onto `--control-size` … `--control-line-height`.
 * Unlike the tint bindings these reference `:root` variables, so they resolve
 * wherever they are declared; they live here rather than in a hand-written
 * stylesheet so the set of sizes stays defined by the tokens alone.
 */
function renderSizeProfiles(sizes) {
  const BOUND = [
    ['size', 'size'],
    ['gap', 'gap'],
    ['padding-block', 'padding-block'],
    ['padding-inline', 'padding-inline'],
    ['radius', 'radius'],
    ['font-family', 'content-font-family'],
    ['font-size', 'content-font-size'],
    ['font-weight', 'content-font-weight'],
    ['line-height', 'content-line-height'],
  ]

  const lines = []
  for (const size of sizes) {
    lines.push(`[data-size='${size}'] {`)
    for (const [bound, source] of BOUND) {
      lines.push(`  --control-${bound}: var(--control-${size}-${source});`)
    }
    lines.push('}', '')
  }
  return lines
}

function render(ramps, recipes, washes, sizes, modeSelectors) {
  const lines = [
    '/* ------------------------------------------- *',
    ' *  Autogenerated by ⛋ Terrazzo. DO NOT EDIT!  *',
    ' * ------------------------------------------- */',
    '',
  ]

  for (const [ramp, shades] of ramps) {
    lines.push(`[data-tint='${ramp}'] {`)
    for (const shade of shades) {
      lines.push(`  --tint-${shade}: var(--color-${ramp}-${shade});`)
    }
    lines.push('}', '')
  }

  const resolved = []
  for (const [name, recipe] of recipes) {
    const value = resolve(recipe)
    if (value === null) continue
    resolved.push(`  --control-color-${name}: ${value};`)
  }

  if (resolved.length > 0) {
    lines.push('[data-tint] {', ...resolved.sort(), '}', '')
  }

  // One block per theme that re-picks a shade. Only those recipes appear;
  // one that names a literal colour is already reading a variable that
  // follows the theme on its own, and re-stating it here would freeze it.
  //
  // Each theme selector is emitted twice — as an ancestor and on the element
  // itself — because the tinted element is usually a descendant of the
  // themed root but may be that root. Blocks come out in the order the
  // selectors were configured, which is what lets a theme selected by two of
  // them take the later one's shades.
  for (const { mode, selectors = [] } of modeSelectors) {
    const resolvedForMode = []
    for (const [name, recipe] of recipes) {
      if (!recipe.modes?.[mode]) continue
      const value = resolve(recipe, mode)
      if (value === null) continue
      resolvedForMode.push(`  --control-color-${name}: ${value};`)
    }
    if (resolvedForMode.length === 0) continue

    const scoped = selectors.flatMap(selector => [
      `${selector} [data-tint]`,
      `${selector}[data-tint]`,
    ])
    lines.push(`${scoped.join(',\n')} {`, ...resolvedForMode.sort(), '}', '')
  }

  lines.push(...renderSizeProfiles(sizes))
  lines.push(...renderStateMachine(recipes))

  const washed = []
  for (const [path, wash] of washes) {
    const base = `var(${cssName(path)}-color)`
    washed.push(`  ${cssName(path)}: ${applyAlpha(base, wash.alpha)};`)
  }

  if (washed.length > 0) {
    lines.push(':root {', ...washed.sort(), '}', '')
  }

  return lines.join('\n')
}
