/** Compare browser caret pixels with quads submitted by a real GPUI window.
 * Build dazzui-example first, then run with --out <dir> [--themes drawn|all|ids].
 * This deliberately does not use TestAppContext's NoopTextSystem.
 */
import assert from 'node:assert/strict'
import { execFile as execFileCallback } from 'node:child_process'
import { mkdir, readFile, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { promisify, parseArgs } from 'node:util'

const repo = fileURLToPath(new URL('../../../../../', import.meta.url))
const { values: args } = parseArgs({
  options: {
    out: {
      type: 'string',
      default: join(tmpdir(), `dazzui-field-caret-${process.pid}`),
    },
    themes: { type: 'string', default: 'drawn' },
    cases: { type: 'string' },
    binary: {
      type: 'string',
      default: join(repo, 'packages/rust/dazzui/target/debug/dazzui-example'),
    },
  },
})
const out = resolve(args.out)
const example = join(repo, 'packages/js/dazzui/example')
const { createServer } = await import(
  pathToFileURL(join(example, 'node_modules/vite/dist/node/index.js'))
)
const { chromium } = await import(
  pathToFileURL(join(example, 'node_modules/playwright/index.mjs'))
)
const execFile = promisify(execFileCallback)
const field = (id, scene, at, input, root, index = 0) => ({
  id,
  scene,
  address: `${scene}/${at}`,
  input,
  root,
  index,
})
const mac = process.platform === 'darwin'
const wordLeft = mac ? 'Alt+ArrowLeft' : 'Control+ArrowLeft'
const wordRight = mac ? 'Alt+ArrowRight' : 'Control+ArrowRight'
const wordDelete = mac ? 'Alt+Delete' : 'Control+Delete'
const home = mac ? 'Meta+ArrowLeft' : 'Home'
const rowKeys = [
  {
    native: mac ? 'cmd-up' : 'ctrl-home',
    browser: mac ? 'Meta+ArrowUp' : 'Control+Home',
  },
  { native: 'down', browser: 'ArrowDown' },
  {
    native: mac ? 'cmd-right' : 'end',
    browser: mac ? 'Meta+ArrowRight' : 'End',
  },
]
const batches = [
  ...[
    ['embedding', 'abc \u202bאבג\u202c xyz'],
    ['isolate', 'abc \u2067אבג\u2069 xyz'],
    ['lre', 'abc \u202aאבג\u202c xyz'],
    ['lro', 'abc \u202dאבג\u202c xyz'],
    ['rlo', 'abc \u202eאבג\u202c xyz'],
    ['kerning-av', 'abc \u202eAV\u202c xyz'],
    ['kerning-to', 'abc \u202eTo\u202c xyz'],
    ['fsi', 'abc \u2068אבג\u2069 xyz'],
    ['alm', 'abc \u061cאבג xyz'],
    ['lrm', 'abc \u200eאבג xyz'],
    ['rlm', 'abc \u200fאבג xyz'],
    ['ligature', 'abc \u2067لا\u2069 xyz'],
  ].flatMap(([kind, inputText]) =>
    (['embedding', 'isolate'].includes(kind)
      ? Array.from({ length: inputText.length + 1 }, (_, count) => count)
      : kind.startsWith('kerning-')
        ? [4, 5, 6, 7, 8, 9]
        : ['alm', 'lrm', 'rlm'].includes(kind)
          ? [7, 8]
          : kind === 'ligature'
            ? [5, 6, 7, 8]
            : [4, 5, 8, 9]
    ).map(count => ({
      focus: 0,
      inputText,
      batchKeys: true,
      keys: Array.from({ length: count }, () => ({
        native: 'left',
        browser: 'ArrowLeft',
      })),
      cases: [
        field(
          `text-field/bidi-${kind}-left-${count}`,
          'text-field/preview',
          0,
          '.dz-text-field',
          '.dz-text-field'
        ),
      ],
    }))
  ),
  ...[
    [
      'pure-rtl-start',
      'אבג דהו',
      false,
      [
        {
          native: mac ? 'cmd-up' : 'ctrl-home',
          browser: mac ? 'Meta+ArrowUp' : 'Control+Home',
        },
      ],
    ],
    ['pure-rtl-end', 'אבג דהו', false, undefined],
    [
      'wrapped-pure-rtl-end',
      'בראשית ברא אלוהים את השמים ואת הארץ והארץ היתה תוהו ובוהו '.repeat(2),
      true,
      undefined,
    ],
  ].map(([id, inputText, multiline, keys]) => ({
    focus: 0,
    inputText,
    keys,
    cases: [
      field(
        `text-field/${id}`,
        multiline ? 'text-field/2' : 'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
  {
    focus: 0,
    inputText: 'אבג דהו זחט יכל abc מנע פצק רשת more אבג דהו end '.repeat(2),
    clickX: 14,
    clickY: 36,
    cases: [
      field(
        'text-field/bidi-wrapped-click-start',
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    cases: [
      field(
        'text-field/default',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        'search-field/default',
        'search-field/preview',
        0,
        '.dz-search-field__input',
        '.dz-search-field'
      ),
      field(
        'number-field/default',
        'number-field/preview',
        0,
        '.dz-number-field__input',
        '.dz-number-field__group'
      ),
      field(
        'combobox/default',
        'combobox/preview',
        0,
        '.dz-combobox__input',
        '.dz-combobox__group'
      ),
      field(
        'autocomplete/default',
        'combobox/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        'otp-field/default',
        'otp-field/preview',
        0,
        '.dz-otp-field__cell',
        '.dz-otp-field__cell'
      ),
      field(
        'text-field/small',
        'text-field/0',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        'text-field/multiline-empty',
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        'otp-field/small',
        'otp-field/2',
        0,
        '.dz-otp-field__cell',
        '.dz-otp-field__cell'
      ),
    ],
  },
  {
    focus: 1,
    cases: [
      field(
        'text-field/medium',
        'text-field/0',
        1,
        '.dz-text-field',
        '.dz-text-field',
        1
      ),
      field(
        'text-field/multiline-filled',
        'text-field/2',
        1,
        '.dz-text-field',
        '.dz-text-field',
        1
      ),
    ],
  },
  {
    focus: 2,
    cases: [
      field(
        'text-field/large',
        'text-field/0',
        2,
        '.dz-text-field',
        '.dz-text-field',
        2
      ),
      // The disabled input before the mono field is absent from native Tab order.
      field(
        'text-field/mono',
        'text-field/1',
        3,
        '.dz-text-field',
        '.dz-text-field',
        3
      ),
    ],
  },
  {
    focus: 4,
    cases: [
      field(
        'otp-field/large',
        'otp-field/2',
        1,
        '.dz-otp-field__cell',
        '.dz-otp-field__cell',
        4
      ),
    ],
  },
  {
    focus: 8,
    cases: [
      field(
        'otp-field/error',
        'otp-field/2',
        2,
        '.dz-otp-field__cell',
        '.dz-otp-field__cell',
        8
      ),
    ],
  },
  {
    focus: 0,
    inputText: Array.from({ length: 20 }, (_, i) => `Line ${i + 1}`).join('\n'),
    cases: [
      field(
        'text-field/multiline-overflow',
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'Line 20',
    cases: [
      field(
        'text-field/filled-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'DazzUI 世界 end',
    cases: [
      field(
        'text-field/cjk-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'क्षि कर्मि',
    cases: [
      field(
        'text-field/indic-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'DazzUI 世界 end '.repeat(12),
    keys: rowKeys,
    cases: [
      field(
        'text-field/cjk-wrapped-end',
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'Hello 👨‍👩‍👧‍👦 end',
    cases: [
      field(
        'text-field/emoji-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 2,
    inputText: 'Hello 👨‍👩‍👧‍👦 end',
    cases: [
      field(
        'text-field/mono-emoji-end',
        'text-field/1',
        3,
        '.dz-text-field',
        '.dz-text-field',
        3
      ),
    ],
  },
  ...[false, true].map(home => ({
    focus: 0,
    inputText: 'abc def ghi jkl mno pqr '.repeat(8),
    keys: [
      ...rowKeys,
      ...(home
        ? [
            {
              native: mac ? 'cmd-left' : 'home',
              browser: mac ? 'Meta+ArrowLeft' : 'Home',
            },
          ]
        : []),
    ],
    cases: [
      field(
        `text-field/wrapped-${home ? 'home' : 'end'}`,
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
  ...[
    ['grapheme-family-left', 'X👨‍👩‍👧‍👦Y', ['ArrowLeft', 'ArrowLeft']],
    ['grapheme-family-backspace', 'X👨‍👩‍👧‍👦Y', ['ArrowLeft', 'Backspace']],
    ['grapheme-flags-left', 'X🇦🇧🇨Y', ['ArrowLeft', 'ArrowLeft']],
    ['grapheme-flags-backspace', 'X🇦🇧🇨Y', ['ArrowLeft', 'Backspace']],
    ['grapheme-marks-left', 'Xa\u0301\u0323Y', ['ArrowLeft', 'ArrowLeft']],
    ['grapheme-marks-backspace', 'Xa\u0301\u0323Y', ['ArrowLeft', 'Backspace']],
    ['grapheme-indic-joined-left', 'Xक्‍षिY', ['ArrowLeft', 'ArrowLeft']],
    ['grapheme-prepend-backspace', 'X\u0600AY', ['ArrowLeft', 'Backspace']],
    ['grapheme-ligature-left', 'office', Array(3).fill('ArrowLeft')],
    ['word-marks-left', 'e\u0301 foo', [wordLeft, wordLeft]],
    ['word-marks-right', 'e\u0301 foo', [home, wordRight]],
    ['word-indic-right', 'नमस्ते दुनिया', [home, wordRight]],
    ['word-cjk-right', '你好世界，中文测试', [home, wordRight]],
    ['word-thai-right', 'ภาษาไทยทดสอบ สวัสดี', [home, wordRight]],
    ['word-punctuation-right', 'word...next', [home, wordRight, wordRight]],
    ['word-punctuation-delete', 'word...next', [home, wordRight, wordDelete]],
    ['bidi-hebrew-end', 'שלום עולם', []],
    ['bidi-hebrew-home', 'שלום עולם', [mac ? 'Meta+ArrowLeft' : 'Home']],
    ['bidi-hebrew-left', 'שלום עולם', ['ArrowLeft']],
    ['bidi-hebrew-right', 'שלום עולם', ['ArrowRight']],
    ['bidi-arabic-end', 'مرحبا بالعالم', []],
    ['bidi-arabic-vowels', 'مَرْحَبًا', []],
    ['bidi-reordered-marks', 'אבג a\u0301\u0323', []],
    ['bidi-rtl-first', 'אבג abc', Array(5).fill('ArrowLeft')],
    ['bidi-numbers', 'אבג 123 דה', Array(4).fill('ArrowLeft')],
    ...[4, 5, 6, 7].map(count => [
      `bidi-mixed-left-${count}`,
      'abc אבג def',
      Array(count).fill('ArrowLeft'),
    ]),
  ].map(([id, inputText, keys]) => ({
    focus: 0,
    inputText,
    keys: keys.map(browser => ({
      browser,
      native: {
        ArrowLeft: 'left',
        ArrowRight: 'right',
        Backspace: 'backspace',
        'Meta+ArrowLeft': 'cmd-left',
        Home: 'home',
        'Alt+ArrowLeft': 'alt-left',
        'Alt+ArrowRight': 'alt-right',
        'Alt+Delete': 'alt-delete',
        'Control+ArrowLeft': 'ctrl-left',
        'Control+ArrowRight': 'ctrl-right',
        'Control+Delete': 'ctrl-delete',
      }[browser],
    })),
    cases: [
      field(
        `text-field/${id}`,
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
  {
    focus: 0,
    inputText: 'a\u0323\u0301 a\u0301\u0323',
    cases: [
      field(
        'text-field/combining-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  ...[38, 42, 48, 54, 58].map(clickX => ({
    focus: 0,
    inputText: 'abc אבג def',
    clickX,
    cases: [
      field(
        `text-field/bidi-click-${clickX}`,
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
  ...[
    ['double-click-word', 'one two three', 39],
    ['double-click-gap', 'one   two', 33],
    ['double-click-punctuation', 'one...two', 36],
    ['double-click-marks', 'e\u0301 a\u0301\u0323.foo', 14],
    ['double-click-cjk', '你好世界，中文测试', 25],
    ['double-click-thai', 'ภาษาไทยทดสอบ สวัสดี', 25],
    ['double-click-emoji', 'a 👨‍👩‍👧‍👦 😀😀 b', 32],
  ].map(([id, inputText, clickX]) => ({
    focus: 0,
    inputText,
    clickX,
    clickCount: 2,
    replacement: '#',
    cases: [
      field(
        `text-field/${id}`,
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
  ...[
    ['triple-click-single', 'one two three', false, undefined, []],
    ['triple-click-first', 'one\ntwo\nthree', true, 16, []],
    ['triple-click-middle', 'one\ntwo\nthree', true, 36, []],
    ['triple-click-empty-start', '\none\n', true, 16, []],
    ['triple-click-empty-end', 'one\n', true, 56, []],
    ['triple-click-unicode', '世界 😀\nnext', true, 16, []],
    [
      'triple-click-wrapped',
      'one two three four five six seven eight '.repeat(6) + '\nend',
      true,
      16,
      [],
    ],
    ['paragraph-drag-forward', 'one\ntwo\nthree', true, 16, [[16, 56]]],
    ['paragraph-drag-backward', 'one\ntwo\nthree', true, 56, [[16, 16]]],
    [
      'paragraph-drag-return',
      'one\ntwo\nthree',
      true,
      36,
      [
        [16, 16],
        [16, 36],
      ],
    ],
  ].map(([id, inputText, multiline, clickY, drag]) => ({
    focus: 0,
    inputText,
    clickX: 16,
    clickY,
    clickCount: 3,
    drag,
    replacement: '#',
    cases: [
      field(
        `text-field/${id}`,
        multiline ? 'text-field/2' : 'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
  ...[
    [
      'undo',
      [
        {
          native: mac ? 'cmd-z' : 'ctrl-z',
          browser: mac ? 'Meta+z' : 'Control+z',
        },
      ],
    ],
    [
      'redo',
      [
        {
          native: mac ? 'cmd-z' : 'ctrl-z',
          browser: mac ? 'Meta+z' : 'Control+z',
        },
        {
          native: mac ? 'cmd-shift-z' : 'ctrl-shift-z',
          browser: mac ? 'Meta+Shift+z' : 'Control+Shift+z',
        },
      ],
    ],
    [
      'typing-undo',
      [
        ...['x', 'y', 'z'].map(key => ({ native: key, browser: key })),
        {
          native: mac ? 'cmd-z' : 'ctrl-z',
          browser: mac ? 'Meta+z' : 'Control+z',
        },
      ],
    ],
    [
      'delete-undo-replace',
      [
        { native: 'backspace', browser: 'Backspace' },
        { native: 'backspace', browser: 'Backspace' },
        {
          native: mac ? 'cmd-z' : 'ctrl-z',
          browser: mac ? 'Meta+z' : 'Control+z',
        },
      ],
    ],
  ].map(([id, keys]) => ({
    focus: 0,
    inputText: 'one two three',
    keys,
    ...(id === 'delete-undo-replace' ? { replacement: '#' } : {}),
    cases: [
      field(
        `text-field/${id}`,
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        `text-field/multiline-${id}`,
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        `search-field/${id}`,
        'search-field/preview',
        0,
        '.dz-search-field__input',
        '.dz-search-field'
      ),
    ],
  })),
  ...[
    ['line-join', 'e\n\u0301😀\n世界', ['start', 'right', 'delete']],
    [
      'line-join-redo',
      'e\n\u0301😀\n世界',
      ['start', 'right', 'delete', 'undo', 'redo'],
    ],
    [
      'empty-line-undo-replace',
      'a\n\nb',
      ['start', 'right', 'delete', 'delete', 'undo'],
    ],
  ].map(([id, inputText, steps]) => ({
    focus: 0,
    inputText,
    ...(id === 'empty-line-undo-replace' ? { replacement: '#' } : {}),
    keys: steps.map(key => ({
      native:
        {
          start: mac ? 'cmd-up' : 'ctrl-home',
          undo: mac ? 'cmd-z' : 'ctrl-z',
          redo: mac ? 'cmd-shift-z' : 'ctrl-shift-z',
        }[key] || key,
      browser: {
        start: mac ? 'Meta+ArrowUp' : 'Control+Home',
        right: 'ArrowRight',
        delete: 'Delete',
        undo: mac ? 'Meta+z' : 'Control+z',
        redo: mac ? 'Meta+Shift+z' : 'Control+Shift+z',
      }[key],
    })),
    cases: [
      field(
        `text-field/${id}`,
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
  ...[
    ['history-step', '20', ['up', 'undo', 'redo']],
    ['history-typing', '', ['3', '4', '5', 'undo']],
    ['history-redo', '', ['3', '4', '5', 'undo', 'redo']],
    ['history-partial', '-', ['undo', 'redo', 'up']],
  ].map(([id, inputText, steps]) => ({
    focus: 0,
    inputText,
    keys: steps.map(key => ({
      native:
        {
          undo: mac ? 'cmd-z' : 'ctrl-z',
          redo: mac ? 'cmd-shift-z' : 'ctrl-shift-z',
        }[key] || key,
      browser:
        {
          up: 'ArrowUp',
          undo: mac ? 'Meta+z' : 'Control+z',
          redo: mac ? 'Meta+Shift+z' : 'Control+Shift+z',
        }[key] || key,
    })),
    cases: [
      field(
        `number-field/${id}`,
        'number-field/preview',
        0,
        '.dz-number-field__input',
        '.dz-number-field__group'
      ),
    ],
  })),
  ...[
    ['word-shift-right', false, 2, 39, undefined, [[110, 16]]],
    ['word-shift-left', false, 2, 39, undefined, [[10, 16]]],
    [
      'word-shift-return',
      false,
      2,
      39,
      undefined,
      [
        [110, 16],
        [10, 16],
        [76, 16],
        [39, 16],
      ],
    ],
    ['character-shift-left', false, 1, 76, undefined, [[10, 16]]],
    ['paragraph-shift-right', true, 3, 16, 36, [[16, 56]]],
    [
      'paragraph-shift-return',
      true,
      3,
      16,
      36,
      [
        [16, 56],
        [16, 16],
        [16, 36],
      ],
    ],
  ].map(([id, multiline, clickCount, clickX, clickY, shiftClicks]) => ({
    focus: 0,
    inputText: multiline ? 'one\ntwo\nthree' : 'one two three four',
    clickCount,
    clickX,
    clickY,
    shiftClicks,
    replacement: '#',
    cases: [
      field(
        `text-field/${id}`,
        multiline ? 'text-field/2' : 'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
  ...[38, 54].map(clickX => ({
    focus: 0,
    inputText: 'abc אבג def',
    clickX,
    keys: [{ native: 'backspace', browser: 'Backspace' }],
    cases: [
      field(
        `text-field/bidi-click-${clickX}-delete`,
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
]
  .map(batch => ({
    ...batch,
    cases: batch.cases.filter(
      item => !args.cases || args.cases.split(',').includes(item.id)
    ),
  }))
  .filter(batch => batch.cases.length)
assert.ok(batches.length, 'no matching caret cases')
await mkdir(out, { recursive: true })
const server = await createServer({
  root: example,
  cacheDir: 'node_modules/.vite-native-field-caret',
  logLevel: 'silent',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
const browser = await chromium.launch({
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
})
let page, scale
const readings = [],
  failures = []
try {
  for (const [batchIndex, batch] of batches.entries()) {
    const nativeDir = join(out, `native-${batchIndex}`)
    console.log(
      `native caret batch ${batchIndex + 1}/${batches.length}: focus ${batch.focus}`
    )
    const result = await execFile(
      resolve(args.binary),
      [
        '--conformance',
        '--out',
        nativeDir,
        '--contract',
        join(repo, 'tools/conformance/scenes.json'),
        '--themes',
        args.themes,
        '--scenes',
        [...new Set(batch.cases.map(item => item.scene))].join(','),
        '--no-images',
        '--caret-focus',
        String(batch.focus),
        ...(batch.inputText !== undefined
          ? ['--caret-input', batch.inputText]
          : []),
        ...(batch.clickX !== undefined
          ? ['--caret-click', String(batch.clickX)]
          : []),
        ...(batch.clickCount
          ? ['--caret-click-count', String(batch.clickCount)]
          : []),
        ...(batch.clickY !== undefined
          ? ['--caret-click-y', String(batch.clickY)]
          : []),
        ...(batch.drag?.length
          ? ['--caret-drag', batch.drag.map(point => point.join(',')).join(';')]
          : []),
        ...(batch.shiftClicks?.length
          ? [
              '--caret-shift-clicks',
              batch.shiftClicks.map(point => point.join(',')).join(';'),
            ]
          : []),
        ...(batch.replacement !== undefined
          ? ['--caret-replace-selection', batch.replacement]
          : []),
        ...(batch.keys
          ? ['--caret-keys', batch.keys.map(key => key.native).join(',')]
          : []),
        ...(batch.batchKeys ? ['--caret-batch-keys'] : []),
      ],
      { maxBuffer: 1024 * 1024 }
    )
    await writeFile(
      join(out, `native-${batchIndex}.log`),
      result.stdout + result.stderr
    )
    const native = JSON.parse(
      await readFile(join(nativeDir, 'carets.json'), 'utf8')
    )
    if (page)
      assert.equal(
        native.scaleFactor,
        scale,
        'native display scale changed between batches'
      )
    else {
      scale = native.scaleFactor
      page = await browser.newPage({ deviceScaleFactor: scale })
      await page.goto(
        new URL('conformance.html', server.resolvedUrls.local[0]).href
      )
      await page.waitForFunction(() => window.conformance)
    }
    for (const [theme, traces] of Object.entries(native.themes))
      for (const item of batch.cases) {
        await page.evaluate(
          ({ scene, theme }) => window.conformance.show(scene, theme),
          { scene: item.scene, theme }
        )
        const trace = traces[item.address]
        const input = page.locator(item.input).nth(item.index)
        await input.focus()
        const font = await input.evaluate(
          (node, { root, frame }) => {
            document
              .querySelectorAll('[data-caret-crop]')
              .forEach(node => node.removeAttribute('data-caret-crop'))
            const crop = node.closest(root)
            crop.setAttribute('data-caret-crop', '')
            // Compare the caret in the same outer box. The ordinary collector
            // owns box conformance; fractional layout differences can otherwise
            // cross a pixel boundary and change the caret's raster position.
            if (frame) {
              Object.assign(crop.style, {
                position: 'fixed',
                left: `${frame.left}px`,
                top: `${frame.top}px`,
                width: `${frame.width}px`,
                height: `${frame.height}px`,
                margin: '0',
              })
            }
            node.setSelectionRange(0, 0)
            node.style.setProperty('caret-color', '#ff00ff', 'important')
            const style = getComputedStyle(node)
            return {
              family: style.fontFamily,
              size: style.fontSize,
              weight: style.fontWeight,
              lineHeight: style.lineHeight,
              frame: crop.getBoundingClientRect().toJSON(),
            }
          },
          { root: item.root, frame: trace?.surface }
        )
        if (batch.inputText !== undefined) await input.fill(batch.inputText)
        if (batch.clickX !== undefined) {
          const x = font.frame.left + batch.clickX
          const y = font.frame.top + (batch.clickY ?? font.frame.height / 2)
          const clickCount = batch.clickCount || 1
          if (batch.drag?.length) {
            await page.mouse.move(x, y)
            await page.mouse.down({ clickCount })
            for (const [x, y] of batch.drag)
              await page.mouse.move(font.frame.left + x, font.frame.top + y)
            await page.mouse.up({ clickCount })
          } else {
            await page.mouse.click(x, y, { clickCount })
          }
        }
        for (const [x, y] of batch.shiftClicks || []) {
          await page.keyboard.down('Shift')
          await page.mouse.click(font.frame.left + x, font.frame.top + y)
          await page.keyboard.up('Shift')
        }
        for (const key of batch.keys || []) await input.press(key.browser)
        if (batch.replacement !== undefined)
          await page.keyboard.insertText(batch.replacement)
        font.value = await input.inputValue()
        font.selection = await input.evaluate(node => [
          node.selectionStart,
          node.selectionEnd,
        ])
        font.selectionReversed = await input.evaluate(
          node => node.selectionDirection === 'backward'
        )
        const png = await page
          .locator('[data-caret-crop]')
          .screenshot({ caret: 'initial' })
        const imageDir = join(out, 'browser', theme)
        await mkdir(imageDir, { recursive: true })
        await writeFile(
          join(imageDir, item.id.replaceAll('/', '-') + '.png'),
          png
        )
        const reference = await page.evaluate(
          async ({ data, scale }) => {
            const image = new Image()
            image.src = data
            await image.decode()
            const canvas = document.createElement('canvas')
            canvas.width = image.width
            canvas.height = image.height
            const context = canvas.getContext('2d')
            context.drawImage(image, 0, 0)
            const pixels = context.getImageData(
              0,
              0,
              canvas.width,
              canvas.height
            ).data
            let left = Infinity,
              top = Infinity,
              right = -1,
              bottom = -1
            for (let y = 0; y < canvas.height; y++)
              for (let x = 0; x < canvas.width; x++) {
                const at = (y * canvas.width + x) * 4
                if (
                  pixels[at] > 240 &&
                  pixels[at + 1] < 20 &&
                  pixels[at + 2] > 240
                ) {
                  left = Math.min(left, x)
                  top = Math.min(top, y)
                  right = Math.max(right, x)
                  bottom = Math.max(bottom, y)
                }
              }
            return right < 0
              ? null
              : {
                  left: left / scale,
                  top: top / scale,
                  width: (right - left + 1) / scale,
                  height: (bottom - top + 1) / scale,
                }
          },
          { data: 'data:image/png;base64,' + png.toString('base64'), scale }
        )
        let actual = null
        const errors = []
        if (!trace?.input)
          errors.push('native input did not report its value and selection')
        if (
          trace?.input &&
          (trace.input.value !== font.value ||
            JSON.stringify(trace.input.selection) !==
              JSON.stringify(font.selection) ||
            (font.selection[0] !== font.selection[1] &&
              trace.input.reversed !== font.selectionReversed))
        )
          errors.push('native input value or selection differs')
        if (!reference) errors.push('browser did not paint a caret')
        if (trace?.quads.length !== 1)
          errors.push('native must submit exactly one caret quad')
        else {
          const { bounds, clip } = trace.quads[0],
            surface = trace.surface
          actual = {
            // Playwright encloses the crop in whole CSS pixels.
            left: bounds.left - Math.floor(surface.left),
            top: bounds.top - Math.floor(surface.top),
            width: bounds.width,
            height: bounds.height,
          }
          if (
            ['left', 'top', 'width', 'height'].some(
              key => Math.abs(font.frame[key] - surface[key]) > 0.01
            )
          )
            errors.push('browser did not adopt the native outer box')
          if (
            bounds.left < clip.left ||
            bounds.top < clip.top ||
            bounds.left + bounds.width > clip.left + clip.width ||
            bounds.top + bounds.height > clip.top + clip.height
          )
            errors.push('native caret is clipped')
          if (
            reference &&
            Object.keys(reference).some(
              key => Math.abs(reference[key] - actual[key]) > 0.01
            )
          )
            errors.push('caret geometry differs')
        }
        const reading = {
          theme,
          case: item.id,
          scene: item.scene,
          address: item.address,
          font,
          input: trace?.input,
          reference,
          actual,
          errors,
        }
        readings.push(reading)
        if (errors.length) failures.push(reading)
      }
  }
} finally {
  await browser.close()
  await server.close()
}
await writeFile(
  join(out, 'report.json'),
  JSON.stringify(
    { scale, cases: readings.length, failures, readings },
    null,
    2
  ) + '\n'
)
console.log(
  `${readings.length} native/browser caret comparisons; ${failures.length} failures — ${out}`
)
if (failures.length) {
  console.error(JSON.stringify(failures.slice(0, 8), null, 2))
  process.exitCode = 1
}
