export type DazzSliderValue = number | number[]
export type DazzSliderSize = 'small' | 'medium' | 'large'

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value))
}

export function sliderValues(value: DazzSliderValue, min: number, max: number): number[] {
  const values = Array.isArray(value) ? value : [value]
  return values.map((item: number): number => clamp(item, min, max)).sort((a: number, b: number): number => a - b)
}

function precision(value: number): number {
  if (value === 0) return 0
  const parts = value.toExponential().split('e')
  return Math.max(0, (parts[0].split('.')[1]?.length ?? 0) - Number(parts[1]))
}

function roundToStep(value: number, min: number, step: number): number {
  const snapped = Math.round((value - min) / step) * step + min
  return Number(snapped.toFixed(Math.max(precision(step), precision(min))))
}

export function sliderSnap(value: number, min: number, max: number, step: number): number {
  return clamp(roundToStep(value, min, step), min, max)
}

// Base UI's default pointer collision behavior pushes neighbours. Keyboard and
// accessibility input instead stop at the adjacent thumb; they do not push it.
export function sliderChange(values: number[], index: number, value: number, min: number, max: number, push: boolean): number[] {
  const next = values.slice()
  if (index < 0 || index >= next.length || !Number.isFinite(value)) return next
  next[index] = clamp(value, push ? min : (values[index - 1] ?? min), push ? max : (values[index + 1] ?? max))
  if (push) {
    for (let i = index + 1; i < next.length; i++) next[i] = Math.max(next[i], next[i - 1])
    for (let i = index - 1; i >= 0; i--) next[i] = Math.min(next[i], next[i + 1])
    for (let i = 0; i < next.length; i++) next[i] = Number(next[i].toFixed(12))
  }
  return next
}

export function sliderKey(values: number[], index: number, key: string, shift: boolean, min: number, max: number, step: number, largeStep: number): number[] | undefined {
  let value = roundToStep(values[index], min, step)
  let increment = shift ? largeStep : step
  let direction = 0
  if (key === 'ArrowUp' || key === 'ArrowRight') direction = 1
  else if (key === 'ArrowDown' || key === 'ArrowLeft') direction = -1
  else if (key === 'PageUp' || key === 'PageDown') {
    direction = key === 'PageUp' ? 1 : -1
    increment = largeStep
  } else if (key === 'Home') value = values[index - 1] ?? min
  else if (key === 'End') value = values[index + 1] ?? max
  else return undefined
  if (direction !== 0) value = Number((value + direction * increment).toFixed(Math.max(precision(value), precision(increment), precision(min))))
  return sliderChange(values, index, clamp(value, min, max), min, max, false)
}

export function sliderNearest(values: number[], value: number, max: number): number {
  let index = -1
  let distance = Infinity
  for (let i = 0; i < values.length; i++) {
    const candidate = Math.abs(values[i] - value)
    if (candidate <= distance) {
      index = i
      distance = candidate
    }
  }
  // At the maximum, Base UI lets the first overlapping thumb travel back.
  if (values[index] === max) {
    while (index > 0 && values[index] === values[index - 1]) index--
  }
  return index
}
