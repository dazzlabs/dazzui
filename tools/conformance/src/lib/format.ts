import type { Values } from './model'

/** A reading, as a cell prints it. */
export function formatValue(value: Values[string] | undefined): string {
  if (value === null || value === undefined) return '—'
  if (typeof value === 'number') {
    return Number.isInteger(value)
      ? String(value)
      : value.toFixed(2).replace(/0$/, '')
  }
  return value
}

export const percent = (score: number | null) =>
  score === null ? '—' : `${Math.round(score * 100)}%`

/** `size=small variant=filled`, the way a specimen is named. */
export const axesOf = (axes: Record<string, string>) =>
  Object.entries(axes)
    .map(([k, v]) => `${k}=${v}`)
    .join(' ')

export const plural = (n: number, one: string, many = `${one}s`) =>
  `${n} ${n === 1 ? one : many}`
