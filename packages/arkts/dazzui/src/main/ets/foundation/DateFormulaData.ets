import { addDays, startOfMonth } from './DateData'
import { DazzDateRangeSpan } from './DateRangeData'

export interface DazzDateFormula { op: string; number?: number; name?: string; args?: DazzDateFormula[] }
export interface DazzDateRangeFormula { start: DazzDateFormula; end: DazzDateFormula }

/** Evaluate only the closed date operations emitted by the story reader. */
export function evaluateDateFormula(formula: DazzDateFormula, today: Date): Date | number {
  const input = (index: number): Date | number => {
    const child = formula.args?.[index]
    if (!child) throw new Error('Missing date formula input: ' + formula.op)
    return evaluateDateFormula(child, today)
  }
  const number = (index: number): number => {
    const value = input(index)
    if (typeof value !== 'number' || !Number.isFinite(value)) throw new Error('Expected finite date formula number')
    return value
  }
  const date = (index: number): Date => {
    const value = input(index)
    if (!(value instanceof Date)) throw new Error('Expected date formula date')
    return value
  }
  if (formula.op === 'number') {
    if (formula.number === undefined || !Number.isFinite(formula.number)) throw new Error('Invalid date formula constant')
    return formula.number ?? 0
  }
  if (formula.op === 'today') return new Date(today.getTime())
  if (formula.op === 'timestamp') return new Date(number(0))
  if (formula.op === 'local') {
    const count = formula.args?.length ?? 0
    return new Date(number(0), number(1), number(2), count > 3 ? number(3) : 0, count > 4 ? number(4) : 0, count > 5 ? number(5) : 0, count > 6 ? number(6) : 0)
  }
  if (formula.op === 'addDays') return addDays(date(0), number(1))
  if (formula.op === 'startOfMonth') return startOfMonth(date(0))
  if (formula.op === 'get') {
    const target = date(0)
    if (formula.name === 'getTime') return target.getTime()
    if (formula.name === 'getDay') return target.getDay()
    if (formula.name === 'getFullYear') return target.getFullYear()
    if (formula.name === 'getMonth') return target.getMonth()
    if (formula.name === 'getDate') return target.getDate()
    if (formula.name === 'getHours') return target.getHours()
    if (formula.name === 'getMinutes') return target.getMinutes()
    throw new Error('Unsupported date formula getter: ' + formula.name)
  }
  const left = number(0)
  if (formula.op === 'or') return left || number(1)
  const right = number(1)
  if (formula.op === 'add') return left + right
  if (formula.op === 'subtract') return left - right
  if (formula.op === 'multiply') return left * right
  if (formula.op === 'divide') return left / right
  if (formula.op === 'modulo') return left % right
  throw new Error('Unsupported date formula operation: ' + formula.op)
}
export function evaluateDateRangeFormula(formula: DazzDateRangeFormula, today: Date): DazzDateRangeSpan {
  const start = evaluateDateFormula(formula.start, today)
  const end = evaluateDateFormula(formula.end, today)
  if (!(start instanceof Date) || !(end instanceof Date) || !Number.isFinite(start.getTime()) || !Number.isFinite(end.getTime())) throw new Error('A date preset must return valid dates')
  return { start, end }
}
