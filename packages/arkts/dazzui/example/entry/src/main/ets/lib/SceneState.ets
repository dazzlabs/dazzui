import { SceneValue } from './SceneTypes'

export const EMPTY_VALUE: SceneValue = { kind: 'null' }
export function stringValue(value: string): SceneValue {
  return { kind: 'string', text: value }
}
export function numberValue(value: number): SceneValue {
  return { kind: 'number', number: value }
}
export function booleanValue(value: boolean): SceneValue {
  return { kind: 'boolean', boolean: value }
}

export function text(value: SceneValue): string {
  if (value.kind === 'string' || value.kind === 'icon') return value.text ?? ''
  if (value.kind === 'number') return String(value.number)
  if (value.kind === 'boolean') return String(value.boolean)
  return (value.items ?? []).map((item: SceneValue) => text(item)).join('')
}

export class SceneState {
  private values: Record<string, SceneValue> = {}
  constructor(initial: Record<string, SceneValue> = {}) {
    for (const key of Object.keys(initial)) this.values[key] = initial[key]
  }
  evaluate(value: SceneValue, argument: SceneValue = EMPTY_VALUE): SceneValue {
    if (value.kind === 'state')
      return this.values[value.text ?? ''] ?? EMPTY_VALUE
    if (value.kind === 'argument')
      return value.number === 0 ? argument : EMPTY_VALUE
    if (value.kind === 'eq') {
      const items = value.items ?? []
      const left = this.evaluate(items[0] ?? EMPTY_VALUE, argument)
      const right = this.evaluate(items[1] ?? EMPTY_VALUE, argument)
      return booleanValue(
        left.kind === right.kind && text(left) === text(right)
      )
    }
    return value
  }
  run(action: SceneValue, argument: SceneValue = EMPTY_VALUE): void {
    for (const entry of action.items ?? []) {
      const fields = entry.fields ?? {}
      const key = text(fields['set'] ?? EMPTY_VALUE)
      if (!key || this.values[key] === undefined)
        throw new Error('Unknown scene state: ' + key)
      this.values[key] = this.evaluate(fields['to'] ?? EMPTY_VALUE, argument)
    }
  }
}
