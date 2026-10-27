import { DazzDateRangeValue, DazzDateRangePreset, DazzDateRangeSpan, DazzDateFormula, DazzDateRangeFormula } from '@dazzlabs/dazzui'
import { SceneValue } from './SceneTypes'
import { EMPTY_VALUE, booleanValue, dateFromValue, dateValue, text } from './SceneState'

export function sceneDateRange(value: SceneValue): DazzDateRangeValue | null {
  if (value.kind === 'null') return null
  const start = dateFromValue(value.fields?.['start'] ?? EMPTY_VALUE)
  const end = dateFromValue(value.fields?.['end'] ?? EMPTY_VALUE)
  if (!start || !end) throw new Error('A picker value requires both dates')
  return { start, end, allDay: value.fields?.['allDay']?.boolean ?? false }
}
export function sceneDateRangeValue(value: DazzDateRangeValue): SceneValue {
  return { kind: 'object', fields: { 'start': dateValue(value.start), 'end': dateValue(value.end), 'allDay': booleanValue(value.allDay) } }
}
function formula(value: SceneValue): DazzDateFormula {
  const fields = value.fields ?? {}
  return { op: text(fields['op'] ?? EMPTY_VALUE), number: fields['number']?.number,
    name: fields['name'] ? text(fields['name']) : undefined,
    args: fields['args'] ? (fields['args'].items ?? []).map((item: SceneValue): DazzDateFormula => formula(item)) : undefined }
}
export function sceneDateRangePreset(value: SceneValue, evaluate: (formula: DazzDateRangeFormula, today: Date) => DazzDateRangeSpan): DazzDateRangePreset {
  const range = value.fields?.['range']
  if (range?.kind !== 'dateRange') throw new Error('A preset requires a typed date formula')
  const plan: DazzDateRangeFormula = { start: formula(range.fields?.['start'] ?? EMPTY_VALUE), end: formula(range.fields?.['end'] ?? EMPTY_VALUE) }
  return { label: text(value.fields?.['label'] ?? EMPTY_VALUE), range: (today: Date): DazzDateRangeSpan => evaluate(plan, today) }
}
