export const TRIM_CHARACTERS =
  '\t\n\v\f\r \u00a0\u1680\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u3000\ufeff'
/** Side-effect-free expressions shared by all story runtimes. */
export function valueType(value, state, depth = 0) {
  if (depth > 64) throw Error('Expression nesting exceeds 64 levels')
  if (typeof value === 'string' || typeof value === 'boolean')
    return typeof value
  if (typeof value === 'number' && Number.isFinite(value)) return 'number'
  if (!value || typeof value !== 'object' || Array.isArray(value))
    throw Error('Invalid scene expression')
  const keys = Object.keys(value)
  if (
    keys.length === 1 &&
    keys[0] === 'state' &&
    typeof value.state === 'string' &&
    Object.hasOwn(state, value.state)
  )
    return typeof state[value.state]
  if (
    keys.length === 3 &&
    ['if', 'then', 'else'].every(k => Object.hasOwn(value, k))
  ) {
    if (valueType(value.if, state, depth + 1) !== 'boolean')
      throw Error('Condition must be boolean')
    const result = valueType(value.then, state, depth + 1)
    if (valueType(value.else, state, depth + 1) !== result)
      throw Error('Conditional branches must have the same type')
    return result
  }
  if (keys.length === 1 && keys[0] === 'trim') {
    if (valueType(value.trim, state, depth + 1) !== 'string')
      throw Error('trim requires text')
    return 'string'
  }
  if (keys.length === 1 && keys[0] === 'concat') {
    if (
      !Array.isArray(value.concat) ||
      value.concat.length === 0 ||
      value.concat.some(v => valueType(v, state, depth + 1) !== 'string')
    )
      throw Error('concat requires text operands')
    return 'string'
  }
  if (keys.length === 1 && ['add', 'multiply', 'gte'].includes(keys[0])) {
    const operands = value[keys[0]]
    if (
      !Array.isArray(operands) ||
      operands.length !== 2 ||
      operands.some(v => valueType(v, state, depth + 1) !== 'number')
    )
      throw Error(`${keys[0]} requires two numeric operands`)
    return keys[0] === 'gte' ? 'boolean' : 'number'
  }
  if (keys.length === 1 && keys[0] === 'eq') {
    if (
      !Array.isArray(value.eq) ||
      value.eq.length !== 2 ||
      valueType(value.eq[0], state, depth + 1) !==
        valueType(value.eq[1], state, depth + 1)
    )
      throw Error('eq requires two operands of the same type')
    return 'boolean'
  }
  throw Error('Unknown scene expression or state reference')
}
export function evaluateValue(value, state) {
  if (typeof value !== 'object') return value
  if (Object.hasOwn(value, 'state')) return state[value.state]
  if (Object.hasOwn(value, 'add'))
    return (
      evaluateValue(value.add[0], state) + evaluateValue(value.add[1], state)
    )
  if (Object.hasOwn(value, 'multiply'))
    return (
      evaluateValue(value.multiply[0], state) *
      evaluateValue(value.multiply[1], state)
    )
  if (Object.hasOwn(value, 'gte'))
    return (
      evaluateValue(value.gte[0], state) >= evaluateValue(value.gte[1], state)
    )
  if (Object.hasOwn(value, 'trim'))
    return evaluateValue(value.trim, state).trim()
  if (Object.hasOwn(value, 'concat'))
    return value.concat.map(v => evaluateValue(v, state)).join('')
  if (Object.hasOwn(value, 'eq'))
    return (
      evaluateValue(value.eq[0], state) === evaluateValue(value.eq[1], state)
    )
  return evaluateValue(
    evaluateValue(value.if, state) ? value.then : value.else,
    state
  )
}
