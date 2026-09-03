/**
 * Fill in the interactive colour recipes a variant leaves unwritten.
 *
 * `control.color` in `tokens/global/color.yaml` is a table of six variants by
 * three parts by four states, and most of its cells are one rule applied:
 * every variant goes dead the same way, a part that is not drawn is
 * transparent in every state, and a press is the hover dimmed. Writing all
 * seventy-two cells out said each of those rules a dozen times, so the table
 * now writes its decisions and this fills in the rest — once, before the
 * graph is parsed, so the four platform plugins read the full table they
 * always have and none of them carries a rule of its own.
 *
 * The rules never vary; what they fill in with is data, under `$defaults`:
 *
 *   hovered   unwritten → the same as `normal`
 *   pressed   unwritten → `hovered` with `$defaults.pressed` laid over it
 *   disabled  unwritten → `$defaults.disabled` for that part
 *   a part    unwritten → `$defaults.undrawn` in every state
 *   a shade with no alpha is opaque
 *
 * The expanded document is what Terrazzo parses, so `$defaults` itself never
 * reaches the graph. A key that is not a part or a state is an error rather
 * than a cell that quietly falls through to a default — with defaults in
 * play, a misspelt `hovered` would otherwise build, and build wrong.
 */

const PARTS = ['surface', 'content', 'border']
const STATES = ['normal', 'hovered', 'pressed', 'disabled']

/**
 * The token document with `control.color` expanded, or the same document
 * back when it carries no `$defaults` and so nothing to expand.
 */
export function expandRecipes(document) {
  const recipes = document?.control?.color
  if (!recipes?.$defaults) return document

  const { $defaults: defaults, ...written } = recipes
  for (const key of ['pressed', 'disabled', 'undrawn']) {
    if (!defaults[key]) {
      throw new Error(`control.color.$defaults is missing \`${key}\`.`)
    }
  }
  for (const part of PARTS) {
    if (!defaults.disabled[part]) {
      throw new Error(
        `control.color.$defaults.disabled is missing \`${part}\`.`
      )
    }
  }

  const expanded = {}
  for (const [variant, parts] of Object.entries(written)) {
    expanded[variant] = variant.startsWith('$')
      ? parts
      : expandVariant(parts, defaults, `control.color.${variant}`)
  }

  return {
    ...document,
    control: { ...document.control, color: expanded },
  }
}

function expandVariant(parts, defaults, path) {
  for (const key of Object.keys(parts)) {
    if (!PARTS.includes(key)) {
      throw new Error(
        `${path}.${key}: a recipe has ${list(PARTS)}, nothing else.`
      )
    }
  }

  const variant = {}
  for (const part of PARTS) {
    variant[part] = parts[part]
      ? expandPart(
          parts[part],
          defaults,
          defaults.disabled[part],
          `${path}.${part}`
        )
      : undrawn(defaults.undrawn)
  }
  return variant
}

function expandPart(states, defaults, disabled, path) {
  for (const key of Object.keys(states)) {
    if (!STATES.includes(key)) {
      throw new Error(
        `${path}.${key}: a part has ${list(STATES)}, nothing else.`
      )
    }
  }
  if (!states.normal) {
    throw new Error(
      `${path}: a written part must at least say what it is at rest.`
    )
  }

  const normal = opaque(states.normal)
  const hovered = states.hovered ? opaque(states.hovered) : clone(normal)
  const pressed = states.pressed
    ? opaque(states.pressed)
    : { ...clone(hovered), ...clone(defaults.pressed) }
  return {
    normal,
    hovered,
    pressed,
    disabled: states.disabled ? opaque(states.disabled) : clone(disabled),
  }
}

/** A part the variant does not draw: transparent in every state, no alpha. */
function undrawn(cell) {
  return Object.fromEntries(STATES.map(state => [state, clone(cell)]))
}

/** A shade written without an alpha is the ramp step itself. */
function opaque(cell) {
  const copy = clone(cell)
  if ('shade' in copy && !('alpha' in copy)) copy.alpha = { $value: 1 }
  return copy
}

function clone(value) {
  return structuredClone(value)
}

function list(items) {
  return `\`${items.join('`, `')}\``
}
