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
export function dateValue(value: Date | null): SceneValue {
  return value ? { kind: 'date', items: [value.getFullYear(), value.getMonth(), value.getDate(), value.getHours(), value.getMinutes(), value.getSeconds(), value.getMilliseconds()].map((part: number): SceneValue => numberValue(part)) } : EMPTY_VALUE
}
export function dateFromValue(value: SceneValue): Date | null {
  if (value.kind !== 'date') return null
  const parts = (value.items ?? []).map((part: SceneValue): number => part.number ?? 0)
  return new Date(parts[0], parts[1], parts[2], parts[3] ?? 0, parts[4] ?? 0, parts[5] ?? 0, parts[6] ?? 0)
}
export function truth(value: SceneValue): boolean {
  if (value.kind === 'null') return false
  if (value.kind === 'boolean') return value.boolean ?? false
  if (value.kind === 'string') return !!value.text
  if (value.kind === 'number') return !!value.number
  return true
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
    if (value.kind === 'if') {
      const items = value.items ?? []
      return this.evaluate(items[truth(this.evaluate(items[0] ?? EMPTY_VALUE, argument)) ? 1 : 2] ?? EMPTY_VALUE, argument)
    }
    if (value.kind === 'not') return booleanValue(!truth(this.evaluate(value.items?.[0] ?? EMPTY_VALUE, argument)))
    if (value.kind === 'concat') return stringValue((value.items ?? []).map((item: SceneValue): string => text(this.evaluate(item, argument))).join(''))
    if (value.kind === 'get') {
      const target = this.evaluate(value.items?.[0] ?? EMPTY_VALUE, argument)
      const key = text(this.evaluate(value.items?.[1] ?? EMPTY_VALUE, argument))
      return this.evaluate(target.fields?.[key] ?? EMPTY_VALUE, argument)
    }
    if (value.kind === 'dateText') {
      const date = dateFromValue(this.evaluate(value.items?.[0] ?? EMPTY_VALUE, argument))
      if (!date) throw new Error('Date formatting requires a date value')
      return stringValue(text(value.items?.[1] ?? EMPTY_VALUE) === 'date' ? date.toLocaleDateString() : date.toLocaleString())
    }
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
  run(action: SceneValue, argument: SceneValue = EMPTY_VALUE, toast?: (options: SceneValue) => void): void {
    for (const entry of action.items ?? []) {
      const fields = entry.fields ?? {}
      if (fields['remove'] !== undefined) {
        const key = text(fields['remove'])
        const current = this.values[key]
        if (!current || current.kind !== 'array') throw new Error('Array removal requires array state: ' + key)
        const property = text(fields['key'] ?? EMPTY_VALUE)
        const match = this.evaluate(fields['match'] ?? EMPTY_VALUE, argument)
        this.values[key] = { kind: 'array', items: (current.items ?? []).filter((item: SceneValue): boolean => {
          const value = item.fields?.[property] ?? EMPTY_VALUE
          return value.kind !== match.kind || text(value) !== text(match)
        }) }
        continue
      }
      if (fields['toast'] !== undefined) {
        if (!toast) throw new Error('Toast actions require a native provider')
        toast(this.evaluate(fields['toast'], argument))
        continue
      }
      const key = text(fields['set'] ?? EMPTY_VALUE)
      if (!key || this.values[key] === undefined)
        throw new Error('Unknown scene state: ' + key)
      this.values[key] = this.evaluate(fields['to'] ?? EMPTY_VALUE, argument)
    }
  }
}
