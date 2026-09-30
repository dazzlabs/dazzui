import assert from 'node:assert/strict'
import { test } from 'node:test'

import {
  baseline,
  format,
  hold,
  measure,
  parse,
  prune,
} from '../src/lib/expectations.ts'
import type { Readings, Tolerances } from '../src/lib/model.ts'

const tolerances: Tolerances = {
  length: 0.5,
  colour: 0.02,
  image: 0.98,
  kinds: { width: 'length', height: 'length', corner: 'length' },
}

const reading = (corner: number) => ({
  box: { width: 40, height: 24 },
  values: { corner },
})

/** Readings over two themes: `a/0/0` and `a/0/1` in each. */
function readings(
  role: Readings['role'],
  themes: Record<string, Record<string, ReturnType<typeof reading>>>
): Readings {
  return {
    platform: role === 'reference' ? 'react' : 'probe',
    title: '',
    role,
    collected: '',
    commit: '',
    scale: 2,
    font: null,
    themes,
    scenes: {},
  }
}

const reference = readings('reference', {
  light: { 'a/0/0': reading(6), 'a/0/1': reading(6) },
  dark: { 'a/0/0': reading(6), 'a/0/1': reading(6) },
})

test('a check failing in every theme is one line, a missing specimen one more', () => {
  const measured = measure(
    reference,
    readings('implementation', {
      light: { 'a/0/0': reading(8) },
      dark: { 'a/0/0': reading(8) },
    }),
    tolerances
  )
  assert.deepEqual(baseline(measured), [
    { theme: '*', address: 'a/0/0', property: 'corner' },
    { theme: '*', address: 'a/0/1', property: '*' },
  ])
  assert.deepEqual(hold(baseline(measured), measured), {
    unexpected: [],
    stale: [],
  })
})

test('an unlisted failure is unexpected, and a listed pass is stale', () => {
  const measured = measure(
    reference,
    readings('implementation', {
      light: { 'a/0/0': reading(8), 'a/0/1': reading(6) },
      dark: { 'a/0/0': reading(6), 'a/0/1': reading(6) },
    }),
    tolerances
  )
  const held = hold(parse('# a comment\n* a/0/1 corner\n'), measured)
  assert.deepEqual(held.unexpected, ['light a/0/0 corner'])
  assert.deepEqual(held.stale, [
    {
      line: { theme: '*', address: 'a/0/1', property: 'corner' },
      passes: ['light', 'dark'],
    },
  ])
})

test('pruning keeps the themes a * line still fails in', () => {
  const measured = measure(
    reference,
    readings('implementation', {
      light: { 'a/0/0': reading(8), 'a/0/1': reading(6) },
      dark: { 'a/0/0': reading(6), 'a/0/1': reading(6) },
    }),
    tolerances
  )
  assert.deepEqual(prune(parse('* a/0/0 corner\n'), measured), [
    { theme: 'light', address: 'a/0/0', property: 'corner' },
  ])
})

test('a line about a theme the run did not measure is left alone', () => {
  const measured = measure(
    reference,
    readings('implementation', {
      light: { 'a/0/0': reading(6), 'a/0/1': reading(6) },
    }),
    tolerances
  )
  assert.deepEqual(hold(parse('dark a/0/0 corner\n'), measured).stale, [])
})

test('the file reads back as it was written', () => {
  const lines = parse('*  a/0/10  corner\nlight a/0/9 *\n')
  assert.deepEqual(parse(format(['# header'], lines)), [
    { theme: 'light', address: 'a/0/9', property: '*' },
    { theme: '*', address: 'a/0/10', property: 'corner' },
  ])
})

test('a specimen that stops being missing gives way to what it is off in', () => {
  const measured = measure(
    reference,
    readings('implementation', {
      light: { 'a/0/0': reading(6), 'a/0/1': reading(8) },
      dark: { 'a/0/0': reading(6), 'a/0/1': reading(8) },
    }),
    tolerances
  )
  assert.deepEqual(prune(parse('* a/0/1 *\n'), measured), [
    { theme: '*', address: 'a/0/1', property: 'corner' },
  ])
  assert.deepEqual(hold(prune(parse('* a/0/1 *\n'), measured), measured), {
    unexpected: [],
    stale: [],
  })
})
