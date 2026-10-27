import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import vm from 'node:vm'
import test from 'node:test'
import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'
import {
  HarmonyReactCompiler,
  readHarmonyReactStories,
} from '../../../../tools/codegen/harmony-react.mjs'
import { dateRangeKitSource } from '../../../../tools/codegen/harmony-dates.mjs'
import { value } from '../../../../tools/codegen/harmony.mjs'

const root = fileURLToPath(new URL('../', import.meta.url))
const repo = path.resolve(root, '../../..')
const require = createRequire(path.resolve(repo, 'tools/codegen/package.json'))
const reactRequire = createRequire(
  path.resolve(repo, 'packages/js/dazzui/package.json')
)
const ts = require('typescript')
const loaded = new Map()
function load(file) {
  const absolute = path.resolve(root, file)
  if (loaded.has(absolute)) return loaded.get(absolute)
  const output = ts.transpileModule(fs.readFileSync(absolute, 'utf8'), {
    compilerOptions: {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.CommonJS,
    },
    reportDiagnostics: true,
  })
  assert.deepEqual(output.diagnostics, [], absolute)
  const exports = {}
  loaded.set(absolute, exports)
  vm.runInThisContext(`(function(exports, require) { ${output.outputText} })`, {
    filename: absolute,
  })(exports, relative => {
    assert.ok(
      relative.startsWith('.'),
      'Only actual non-UI modules in this harness: ' + relative
    )
    return load(path.resolve(path.dirname(absolute), relative + '.ets'))
  })
  return exports
}
const native = load('src/main/ets/foundation/DateRangeData.ets')
const formulas = load('src/main/ets/foundation/DateFormulaData.ets')
const glue = load('example/entry/src/main/ets/lib/SceneDateRangeData.ets')
const state = load('example/entry/src/main/ets/lib/SceneState.ets')
const referenceDates = {}
vm.runInThisContext(
  `(function(exports) { ${ts.transpileModule(fs.readFileSync(path.resolve(repo, 'packages/js/dazzui/src/lib/date.ts'), 'utf8'), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText} })`
)(referenceDates)
const sourceText = fs.readFileSync(
  path.resolve(repo, 'packages/js/dazzui/src/components/date-range-picker.tsx'),
  'utf8'
)
const source = ts.createSourceFile(
  'date-range-picker.tsx',
  sourceText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TSX
)
const compiled = new Map()
function declaration(name) {
  if (compiled.has(name)) return compiled.get(name)
  let found
  function visit(node) {
    if (ts.isVariableDeclaration(node) && node.name.getText(source) === name)
      found = `const ${name} = ${node.initializer.getText(source)}`
    if (ts.isFunctionDeclaration(node) && node.name?.text === name)
      found = node.getText(source)
    ts.forEachChild(node, visit)
  }
  visit(source)
  assert.ok(found, 'Locate actual React date-range declaration: ' + name)
  const output = ts.transpileModule(found, {
    compilerOptions: {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.CommonJS,
      jsx: ts.JsxEmit.ReactJSX,
    },
  }).outputText
  compiled.set(name, output)
  return output
}
const deps = ['pad', 'parseClock', 'clock', 'weekend', 'nextMonday']
  .map(declaration)
  .join('\n')
const plain = value => JSON.parse(JSON.stringify(value))
function reference(names, expression, extra = {}) {
  return vm.runInNewContext(
    `${deps}\n${names.map(declaration).join('\n')}\n${expression}`,
    {
      ...referenceDates,
      Date,
      HOUR: 3600000,
      require: reactRequire,
      exports: {},
      ...extra,
    }
  )
}
const at = (day, hour = 0, minute = 0, month = 9) =>
  new Date(2026, month, day, hour, minute)
const cases = [
  { start: at(16, 14, 30), end: at(16, 15, 30), allDay: false },
  { start: at(31, 22, 7), end: at(1, 2, 59, 10), allDay: false },
  { start: at(16, 23), end: at(18, 1), allDay: true },
]

test('date range day/time/duration edits execute the actual React handlers as references', () => {
  for (const draft of cases) {
    const original = plain(draft)
    for (const [handler, method, args] of [
      [
        'setDays',
        native.dateRangeDays,
        [
          { start: at(20), end: null },
          { start: at(20), end: at(23) },
          { start: null, end: null },
        ],
      ],
      [
        'setStartTime',
        native.dateRangeStartTime,
        ['00:00', '09:07', '14:30', '23:59'],
      ],
      [
        'setEndTime',
        native.dateRangeEndTime,
        ['00:00', '09:07', '14:30', '23:59'],
      ],
      ['setDuration', native.dateRangeDuration, [-30, 0, 30, 60, 480, 3000]],
    ])
      for (const argument of args) {
        let expected
        reference([handler], `${handler}(argument)`, {
          draft,
          argument,
          setDaysState: () => {},
          setDraft: updater => {
            expected = updater(draft)
          },
        })
        assert.deepEqual(
          plain(method(draft, argument)),
          plain(expected),
          handler
        )
        assert.deepEqual(
          plain(draft),
          original,
          'The caller’s dates remain unchanged'
        )
      }
    for (const minutes of [0, 30, 60, 480, 3000]) {
      const expected = reference(['durationOn'], 'durationOn({minutes})', {
        draft,
        length: draft.end.getTime() - draft.start.getTime(),
        minutes,
      })
      assert.equal(native.dateRangeDurationOn(draft, minutes), expected)
    }
    assert.equal(
      native.dateRangeOneDay(draft),
      referenceDates.compareDay(draft.start, draft.end) === 0
    )
  }
})

test('date range time options keep off-step values exactly as the React list does', () => {
  for (const timeStep of [1, 15, 30, 45, 60, 90])
    for (const date of [at(16), at(16, 14, 7), at(16, 23, 59)]) {
      const expected = reference(['times', 'timesWith'], 'timesWith(date)', {
        timeStep,
        date,
        useMemo: fn => fn(),
      })
      assert.deepEqual(native.dateRangeTimes(timeStep, date), plain(expected))
      assert.equal(
        native.dateRangeClock(date),
        reference([], 'clock(date)', { date })
      )
    }
  for (const step of [0, -1, Infinity, NaN])
    assert.throws(
      () => native.dateRangeTimes(step, at(16)),
      /positive and finite/
    )
})

test('date range blank values follow React’s clock and session cancellation cannot publish a draft', () => {
  const today = at(16)
  for (const now of [at(16, 10, 37), at(16, 23, 59), at(17, 3)]) {
    class ClockDate extends Date {
      constructor(...args) {
        super(...(args.length ? args : [now.getTime()]))
      }
    }
    const expected = reference(['blank'], 'blank(today)', {
      Date: ClockDate,
      today,
    })
    assert.deepEqual(plain(native.dateRangeBlank(today, now)), plain(expected))
  }
  const session = new native.DazzDateRangeSession()
  let revisions = 0
  let commits = 0
  const observer = () => {
    revisions++
  }
  session.observe(observer)
  session.start(cases[0], today)
  session.setStartTime('16:30')
  session.setDuration(120)
  session.setAllDay(true)
  assert.equal(commits, 0)
  session.close()
  session.confirm(() => {
    commits++
  })
  assert.equal(commits, 0)
  session.start(cases[0], today)
  assert.deepEqual(
    plain(session.draft),
    plain(cases[0]),
    'Reopening discards uncommitted edits'
  )
  session.setDuration(30)
  session.confirm(value => {
    commits++
    assert.equal(session.editing, true)
    value.start.setHours(1)
  })
  assert.equal(commits, 1)
  assert.equal(session.editing, false)
  assert.equal(
    session.draft.start.getHours(),
    14,
    'Published dates do not mutate the retained draft'
  )
  session.confirm(() => {
    commits++
  })
  assert.equal(commits, 1)
  session.start(cases[0], today)
  assert.throws(
    () =>
      session.confirm(() => {
        throw new Error('consumer failure')
      }),
    /consumer failure/
  )
  assert.equal(
    session.editing,
    true,
    'Like React, a failing consumer does not reach the subsequent close'
  )
  const before = revisions
  session.unobserve(observer)
  session.close()
  assert.equal(revisions, before)
  session.observe(observer)
  session.start(cases[0], today)
  const mounted = revisions
  session.dispose()
  session.invalidate()
  session.confirm(() => {
    commits++
  })
  assert.equal(
    revisions,
    mounted,
    'Disposal removes every retained popup observer'
  )
  assert.equal(commits, 1)
})

test('date range defaults and generated preset formulas follow the actual React module in the device timezone', () => {
  const kit = dateRangeKitSource(
    sourceText,
    'components/date-range-picker.tsx',
    HarmonyReactCompiler
  )
  const compiler = new HarmonyReactCompiler('unused.tsx', '', { kitValues: {} })
  const plans = compiler.json(kit.presets)
  const actualPresets = reference(['DATE_RANGE_PRESETS'], 'DATE_RANGE_PRESETS')
  assert.deepEqual(
    native.DATE_RANGE_DURATIONS,
    plain(reference(['DATE_RANGE_DURATIONS'], 'DATE_RANGE_DURATIONS'))
  )
  assert.deepEqual(
    native.DATE_RANGE_LABELS,
    plain(reference(['LABELS'], 'LABELS'))
  )
  assert.deepEqual(
    native.DATE_RANGE_PRESETS.map(p => p.label),
    plain(actualPresets.map(p => p.label))
  )
  for (const today of [
    at(12),
    at(16),
    at(17),
    at(18),
    at(31),
    at(1, 0, 0, 10),
    new Date(2024, 1, 29),
  ])
    actualPresets.forEach((preset, index) => {
      const expected = plain(preset.range(today))
      assert.deepEqual(
        plain(native.DATE_RANGE_PRESETS[index].range(today)),
        expected
      )
      assert.deepEqual(
        plain(
          formulas.evaluateDateRangeFormula(
            plans[index].range.$dateRange,
            today
          )
        ),
        expected
      )
      assert.deepEqual(
        plain(
          glue
            .sceneDateRangePreset(
              value(plans[index]),
              formulas.evaluateDateRangeFormula
            )
            .range(today)
        ),
        expected
      )
    })
})

test('date range summaries preserve React’s actual JSX text and separator positions', () => {
  const format = new Intl.DateTimeFormat('en-US', {
    month: 'short',
    day: 'numeric',
    weekday: 'short',
  })
  function parts(node) {
    if (node == null || typeof node === 'boolean') return []
    if (Array.isArray(node)) return node.flatMap(parts)
    if (typeof node === 'string' || typeof node === 'number')
      return [{ text: String(node) }]
    if (node.props.className === 'dz-date-range-picker__sep')
      return [{ text: node.props.children, separator: true }]
    return parts(node.props.children)
  }
  function merge(parts) {
    return parts.reduce((out, part) => {
      if (!part.separator && out.length && !out.at(-1).separator)
        out.at(-1).text += part.text
      else out.push({ ...part })
      return out
    }, [])
  }
  for (const range of [...cases, { ...cases[0], allDay: true }]) {
    const tree = reference(['Summary'], 'Summary({range,format})', {
      range,
      format,
    })
    assert.deepEqual(
      native.dateRangeSummary(range, date => format.format(date)),
      merge(parts(tree))
    )
  }
})

test('date range custom sprint presets retain timestamp arithmetic and generated callbacks update the live caption', async () => {
  const story = (await readHarmonyReactStories(repo)).stories.find(
    story => story.id === 'date-range-picker'
  )
  assert.ok(story)
  const custom = story.examples[3].scene.content.props
  assert.deepEqual(custom.durations, [{ label: 'Working day', minutes: 480 }])
  assert.equal(custom.timeStep, 30)
  const presets = custom.presets.map(plan =>
    glue.sceneDateRangePreset(value(plan), formulas.evaluateDateRangeFormula)
  )
  const today = at(16)
  const next = presets[2].range(today)
  assert.equal(next.start.getTime(), today.getTime() + 14 * 864e5)
  assert.equal(next.end.getTime(), today.getTime() + 27 * 864e5)
  const data = load(
    'example/entry/src/main/ets/generated/Stories.ets'
  ).STORIES.find(story => story.id === 'date-range-picker')
  const first = data.examples[0]
  const model = new state.SceneState(first.initial)
  const sibling = new state.SceneState(first.initial)
  const [control, caption] = first.content.items
  const initial = glue.sceneDateRange(model.evaluate(control.fields.value))
  const changed = { start: at(18, 9, 7), end: at(19, 14, 37), allDay: true }
  model.run(control.fields.onValueChange, glue.sceneDateRangeValue(changed))
  assert.deepEqual(
    glue.sceneDateRange(model.evaluate(control.fields.value)),
    changed
  )
  assert.deepEqual(
    glue.sceneDateRange(sibling.evaluate(control.fields.value)),
    initial
  )
  assert.equal(
    caption.items.map(item => state.text(model.evaluate(item))).join(''),
    `${changed.start.toLocaleString()} → ${changed.end.toLocaleString()} (all day)`
  )
})
