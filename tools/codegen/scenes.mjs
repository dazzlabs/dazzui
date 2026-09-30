/*
 * The conformance contract: every scene the stories compile to and every kit
 * element in it, at its address. See specs/conformance.md.
 *
 * A scene is walked as it is first drawn — every slot at its initial value,
 * every `$if` taken the way that value takes it — because that is the scene
 * each platform's collector draws. The walk is pre-order: an element, then
 * the elements among its props in the order they were written (into lists
 * and objects, as deep as they go), then its children. A collector finds its
 * own specimens by the same walk, so the order here is the contract.
 */

/** The stage arranges; it is never a specimen. */
const STAGE = new Set(['Row', 'Col', 'Frame', 'Text'])

/** The props that pick a recipe, as the viewer names a specimen by them. */
const AXES = [
  'size',
  'variant',
  'tint',
  'tone',
  'shape',
  'state',
  'status',
  'orientation',
  'placement',
]
const FLAGS = ['disabled', 'checked', 'defaultChecked', 'invalid']

/** `SegmentedControl` → `segmented-control`, the kit's file name. */
export const kebab = name =>
  name.replace(/([a-z0-9])([A-Z])/g, '$1-$2').toLowerCase()

/** A live value, at the scene's initial state. */
function resolve(value, state) {
  if (Array.isArray(value)) return value.map(item => resolve(item, state))
  if (!value || typeof value !== 'object') return value
  if ('$state' in value) return resolve(state[value.$state], state)
  if ('$arg' in value) return null
  if ('$not' in value) return !resolve(value.$not, state)
  if ('$eq' in value) {
    const [a, b] = value.$eq.map(item => resolve(item, state))
    return a === b
  }
  if ('$if' in value) {
    const [test, yes, no] = value.$if
    return resolve(resolve(test, state) ? yes : no, state)
  }
  if ('$concat' in value)
    return value.$concat.map(item => String(resolve(item, state))).join('')
  return value
}

/** The nodes a value holds, flattened the way JSX flattens children. */
function nodes(value, state) {
  const resolved = resolve(value, state)
  if (Array.isArray(resolved))
    return resolved.flatMap(item => nodes(item, state))
  if (resolved === null || resolved === false || resolved === true) return []
  return [resolved]
}

function scene(storyId, id, title, compiled) {
  const state = compiled.state ?? {}
  const specimens = []
  const address = `${storyId}/${id}`

  /* Every element a prop value holds, in order. */
  const inProps = value => {
    const resolved = resolve(value, state)
    if (Array.isArray(resolved)) return resolved.forEach(inProps)
    if (!resolved || typeof resolved !== 'object') return
    if ('$el' in resolved) return visit(resolved)
    if ('$do' in resolved || '$icon' in resolved) return
    Object.values(resolved).forEach(inProps)
  }

  /* Walks one node; answers whether it held a specimen. */
  const visit = node => {
    if (!node || typeof node !== 'object' || !('$el' in node)) return false
    const before = specimens.length
    let specimen
    if (!STAGE.has(node.$el)) {
      specimen = {
        address: `${address}/${specimens.length}`,
        component: kebab(node.$el),
        axes: {},
      }
      specimens.push(specimen)
    }
    const props = node.props ?? {}
    for (const value of Object.values(props)) inProps(value)
    const children = nodes(node.children ?? [], state)
    for (const child of children) visit(child)
    if (!specimen) return specimens.length > before
    for (const axis of AXES) {
      const value = resolve(props[axis], state)
      if (typeof value === 'string') specimen.axes[axis] = value
    }
    for (const flag of FLAGS)
      if (resolve(props[flag], state) === true) specimen.axes[flag] = 'true'
    const text = children.filter(child => typeof child === 'string')
    if (text.length) specimen.text = text.join('').trim()
    if (specimens.length > before + 1) specimen.container = true
    return true
  }

  for (const node of nodes(compiled.content, state)) visit(node)
  return { id: address, title, specimens }
}

/** `tools/conformance/scenes.json`. */
export function scenesJson(stories) {
  return (
    JSON.stringify(
      {
        stories: stories.map(story => ({
          id: story.id,
          name: story.name,
          summary: story.summary,
          scenes: [
            scene(story.id, 'preview', 'Preview', story.preview),
            ...story.examples.map((example, index) =>
              scene(story.id, String(index), example.title, example.scene)
            ),
          ],
        })),
      },
      null,
      2
    ) + '\n'
  )
}
