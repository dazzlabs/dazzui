import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import vm from 'node:vm'
import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'
import test from 'node:test'

const require = createRequire(
  new URL('../../../../tools/codegen/package.json', import.meta.url)
)
const ts = require('typescript')
const root = fileURLToPath(new URL('../', import.meta.url))
const loaded = new Map()

// Execute the actual non-UI ArkTS modules. This tests their logic; it is not
// an ArkUI compiler, SDK validation or a substitute for a device test.
function load(file) {
  const absolute = path.resolve(root, file)
  if (loaded.has(absolute)) return loaded.get(absolute)
  const source = fs.readFileSync(absolute, 'utf8')
  const result = ts.transpileModule(source, {
    compilerOptions: {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.CommonJS,
    },
    reportDiagnostics: true,
  })
  assert.deepEqual(result.diagnostics, [], absolute)
  const exports = {}
  loaded.set(absolute, exports)
  const wrapper = vm.runInThisContext(
    `(function(exports, require) { ${result.outputText}\n})`,
    { filename: absolute }
  )
  wrapper(exports, relative =>
    load(path.resolve(path.dirname(absolute), relative + '.ets'))
  )
  return exports
}
const { DazzTokens, THEME_NAMES, withAlpha } = load(
  'src/main/ets/foundation/Tokens.ets'
)
const { DazzMenuTypeahead, menuCharacter, menubarDestination } = load(
  'src/main/ets/foundation/MenuData.ets'
)

test('menubar focus wraps, skips disabled labels, supports endpoints and preserves vertical arrow intent', () => {
  const items = [
    { label: 'File' },
    { label: 'Disabled', disabled: true },
    { label: 'View' },
    { label: 'Help' },
  ]
  assert.equal(menubarDestination(items, 0, 2015), 2)
  assert.equal(menubarDestination(items, 2, 2014), 0)
  assert.equal(menubarDestination(items, 0, 2014), 3)
  assert.equal(menubarDestination(items, 3, 2015), 0)
  assert.equal(menubarDestination(items, 2, 2081), 0)
  assert.equal(menubarDestination(items, 0, 2082), 3)
  assert.equal(menubarDestination(items, 0, 2013), -1)
  assert.equal(menubarDestination(items, 0, 2013, true), 2)
  assert.equal(menubarDestination(items, 0, 2012, true), 3)
  assert.equal(menubarDestination(items, 0, 2015, true), -1)
  assert.equal(menubarDestination([], -1, 2015), -1)
  assert.equal(
    menubarDestination([{ label: 'Unavailable', disabled: true }], 0, 2081),
    -1
  )
})

test('menu prefix search wraps, skips disabled options, cycles repeated initials and resets after inactivity', () => {
  const choices = [
    { id: 'german-disabled', label: 'German', disabled: true },
    { id: 'english', label: 'English' },
    { id: 'german', label: 'German' },
    { id: 'greek', label: 'Greek' },
  ]
  const input = new DazzMenuTypeahead()
  assert.equal(input.match(choices, 'G', 'english', 0), 'german')
  assert.equal(input.match(choices, 'g', 'german', 100), 'greek')
  assert.equal(input.match(choices, 'g', 'greek', 200), 'german')
  assert.equal(input.match(choices, 'r', 'german', 300), 'greek')
  assert.equal(input.match(choices, 'e', 'greek', 1050), 'english')
  assert.equal(input.match(choices, 'z', 'english', 1100), undefined)
  assert.equal(input.match(choices, 'g', 'english', 1150), 'german')
  input.reset()
  assert.equal(input.typing(1200), false)
  assert.equal(input.match(choices, ' ', 'german', 1200), undefined)
})

test('menu search supports spaces within prefixes and double-letter names without turning them into repeated-key cycles', () => {
  const input = new DazzMenuTypeahead(500)
  const choices = [
    { id: 'aa', label: 'Aaron' },
    { id: 'ad', label: 'Ada' },
    { id: 'new', label: 'New file' },
  ]
  assert.equal(input.match(choices, 'a', '', 0), 'aa')
  assert.equal(input.match(choices, 'a', 'aa', 100), 'aa')
  assert.equal(input.typing(599), true)
  assert.equal(input.typing(600), false)
  assert.equal(input.match(choices, 'n', 'aa', 700), 'new')
  assert.equal(input.match(choices, 'e', 'new', 750), 'new')
  assert.equal(input.match(choices, 'w', 'new', 800), 'new')
  assert.equal(input.match(choices, ' ', 'new', 850), 'new')
  assert.equal(input.match(choices, 'f', 'new', 900), 'new')
  assert.equal(input.match(choices, 'a', 'new', 1400), 'aa')
})

test('menu search accepts Unicode characters and leaves command chords to the host', () => {
  const event = (unicode, modifier = '') => ({
    unicode,
    metaKey: modifier === 'Meta' ? 1 : 0,
    getModifierKeyState: modifiers => modifiers.includes(modifier),
  })
  assert.equal(menuCharacter(event(0x4e2d)), '中')
  assert.equal(menuCharacter(event(0x1f600)), '😀')
  assert.equal(menuCharacter(event(0x1f)), '')
  for (const modifier of ['Ctrl', 'Alt', 'Meta'])
    assert.equal(menuCharacter(event(103, modifier)), '')
  assert.equal(menuCharacter(event(71, 'Shift')), 'G')
  assert.equal(menuCharacter({ ...event(0), keyCode: 2023 }), 'g')
  assert.equal(menuCharacter({ ...event(0), keyCode: 2003 }), '3')
  assert.equal(menuCharacter({ ...event(0), keyCode: 2050 }), ' ')
  assert.equal(menuCharacter({ ...event(0), keyCode: 2054 }), '')
  assert.equal(
    menuCharacter({ ...event(0), keyCode: 2023, keyText: '中' }),
    '中'
  )
})
const {
  SceneState,
  EMPTY_VALUE,
  text,
  booleanValue,
  stringValue,
  numberValue,
  dateValue,
  dateFromValue,
  truth,
} = load('example/entry/src/main/ets/lib/SceneState.ets')
const { normalizeOtp, replaceOtp, removeOtp } = load(
  'src/main/ets/foundation/Otp.ets'
)
const { flattenTree, treeNavigation, treeTypeahead } = load(
  'src/main/ets/foundation/TreeData.ets'
)
const {
  registerSidebarShortcut,
  unregisterSidebarShortcut,
  toggleWindowSidebars,
  isSidebarShortcut,
  hasWindowSidebars,
  observeSidebarShortcuts,
  unobserveSidebarShortcuts,
} = load('src/main/ets/foundation/SidebarData.ets')
const { DazzPreviewInteraction } = load(
  'src/main/ets/foundation/PreviewData.ets'
)
const { DazzPreviewPolygon } = load(
  'src/main/ets/foundation/PreviewPolygon.ets'
)
const { inlineRuns } = load(
  'example/entry/src/main/ets/lib/SceneInlineData.ets'
)
const { DazzPointerContext } = load('src/main/ets/foundation/Pointer.ets')
const { rememberFocus, forgetFocus, captureFocus, focusTarget, observeFocus } =
  load('src/main/ets/foundation/FocusData.ets')
const { setToastShortcut, enterWindowToasts, toastShortcutProvider } = load(
  'src/main/ets/foundation/ToastFocus.ets'
)
test('focus observation isolates windows, ignores stale blur and releases a disposed popup subscription', () => {
  const first = [],
    second = []
  const stopFirst = observeFocus('popup-observer-first', id => first.push(id))
  const stopSecond = observeFocus('popup-observer-second', id =>
    second.push(id)
  )
  rememberFocus('popup-observer-first', 'popup')
  rememberFocus('popup-observer-first', 'popup')
  rememberFocus('popup-observer-second', 'editor')
  rememberFocus('popup-observer-first', 'calendar-day')
  forgetFocus('popup-observer-first', 'popup')
  assert.equal(focusTarget('popup-observer-first'), 'calendar-day')
  forgetFocus('popup-observer-first', 'calendar-day')
  stopFirst()
  const reopened = []
  const stopReopened = observeFocus('popup-observer-first', id =>
    reopened.push(id)
  )
  stopFirst()
  rememberFocus('popup-observer-first', 'next-control')
  assert.deepEqual(first, ['popup', 'calendar-day', ''])
  assert.deepEqual(second, ['editor'])
  assert.deepEqual(reopened, ['next-control'])
  stopReopened()
  stopSecond()
})
const { DazzToastQueue, DazzToastContext } = load(
  'src/main/ets/foundation/ToastData.ets'
)
const { DazzToastSwipe } = load('src/main/ets/foundation/ToastSwipe.ets')
const { shadowOptions } = load('src/main/ets/foundation/Shadow.ets')
const { sceneToastOptions } = load(
  'example/entry/src/main/ets/lib/SceneToastData.ets'
)
const { sliderValues, sliderSnap, sliderChange, sliderKey, sliderNearest } =
  load('src/main/ets/foundation/SliderData.ets')
const {
  windowControls,
  windowControlItems,
  windowChrome,
  DazzWindowContext,
  windowTitlebarControls,
  windowHandoff,
} = load('src/main/ets/foundation/WindowData.ets')

test('shared shadow options map CSS sigma and offsets across native display densities', () => {
  const snapshots = JSON.parse(
    fs.readFileSync(
      path.resolve(
        root,
        '../../csharp/DazzUI/src/DazzUI/Generated/tokens.json'
      ),
      'utf8'
    )
  )
  const logical = value => value.value * (value.unit === 'rem' ? 16 : 1)
  for (const name of THEME_NAMES) {
    const tokens = new DazzTokens(name)
    for (const [id, source] of Object.entries(snapshots[name]).filter(([id]) =>
      id.startsWith('shadow.')
    )) {
      // These are the currently drawn roles. A later multi-layer or spread
      // recipe requires a renderer change rather than silently dropping it.
      assert.equal(
        source.length,
        1,
        `${name}/${id}: native outer-shadow coverage`
      )
      assert.equal(source[0].inset ?? false, false)
      assert.equal(logical(source[0].spread), 0)
      const value = tokens.shadow(id)[0]
      for (const density of [1, 1.5, 1.9, 2, 3]) {
        const result = shadowOptions(value, density)
        // Evaluate the native renderer's radius-to-sigma rule, then compare
        // with the CSS box-shadow definition, not the helper's radius.
        const sigma = result.radius > 0 ? result.radius * 0.57735 + 0.5 : 0
        assert.ok(
          Math.abs(sigma - (logical(source[0].blur) * density) / 2) < 1e-9,
          `${name}/${id}/${density}`
        )
        assert.equal(result.offsetX, logical(source[0].offsetX) * density)
        assert.equal(result.offsetY, logical(source[0].offsetY) * density)
        assert.equal(
          result.color,
          '#' +
            source[0].color
              .map(byte => byte.toString(16).padStart(2, '0'))
              .join('')
              .toUpperCase()
        )
      }
    }
  }
  assert.deepEqual(shadowOptions(undefined, 2), {
    radius: 0,
    offsetX: 0,
    offsetY: 0,
    color: '#00000000',
  })
})

test('window controls match the actual React rendering for every state combination', () => {
  const file = path.resolve(
    root,
    '../../js/dazzui/src/components/window-frame.tsx'
  )
  const source = fs.readFileSync(file, 'utf8')
  const ast = ts.createSourceFile(
    file,
    source,
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TSX
  )
  const names = [
    'stateOf',
    'controlsOf',
    'Controls',
    'Control',
    'TrafficLights',
    'CaptionButtons',
    'HeaderbarButtons',
    'WindowTitlebar',
  ]
  const functions = ast.statements
    .filter(
      node => ts.isFunctionDeclaration(node) && names.includes(node.name?.text)
    )
    .map(node => node.getText(ast))
    .join('\n')
  assert.equal(
    ast.statements.filter(
      node => ts.isFunctionDeclaration(node) && names.includes(node.name?.text)
    ).length,
    names.length
  )
  const labels = ast.statements
    .filter(
      node =>
        ts.isVariableStatement(node) &&
        node.declarationList.declarations.some(declaration =>
          ['LABELS', 'ALL'].includes(declaration.name.getText(ast))
        )
    )
    .map(node => node.getText(ast))
    .join('\n')
  const compiled = ts.transpileModule(
    functions +
      '\n' +
      labels +
      '\nconst FrameContext = {}; let currentFrame; const useContext = () => currentFrame; const cx = (...args) => args.join(" "); exports.reference = { controlsOf, Controls, WindowTitlebar, setFrame: value => { currentFrame = value } };',
    {
      compilerOptions: {
        target: ts.ScriptTarget.ES2022,
        module: ts.ModuleKind.CommonJS,
        jsx: ts.JsxEmit.ReactJSX,
      },
    }
  ).outputText
  const exports = {}
  vm.runInThisContext(`(function(exports, require) { ${compiled} })`, {
    filename: file,
  })(exports, createRequire(file))
  const buttons = element => {
    if (!element) return []
    if (Array.isArray(element)) return element.flatMap(buttons)
    if (typeof element.type === 'function')
      return buttons(element.type(element.props))
    if (element.type === 'button')
      return [
        {
          kind: element.props['data-control-kind'],
          state: element.props['data-state'],
        },
      ]
    return buttons(element.props?.children)
  }
  const states = [undefined, false, true, 'disabled', 'enabled', 'hidden']
  for (const platform of [
    'macos',
    'macos15',
    'windows',
    'gnome',
    'ubuntu',
    'kde',
    'omarchy',
  ]) {
    for (const rootBand of [false, true])
      for (const own of [undefined, false, {}, { close: true }])
        for (const pane of [undefined, false, {}, { close: true }]) {
          const context = new DazzWindowContext()
          context.owned = true
          context.platform = platform
          context.rootBand = rootBand
          context.controls =
            platform === 'omarchy' ? false : windowControls(platform, own)
          const handoff =
            !rootBand &&
            !['macos', 'macos15', 'omarchy'].includes(platform) &&
            own !== false
              ? context.controls
              : false
          const handlers = { onClose() {}, onMinimize() {}, onMaximize() {} }
          exports.reference.setFrame({
            platform,
            controls: context.controls !== false,
            handoff,
            ...handlers,
          })
          const tree = exports.reference.WindowTitlebar({
            title: 'Pane',
            controls: pane,
          })
          const control = tree.props.children.find(
            child => child?.type === exports.reference.Controls
          )
          assert.deepEqual(
            JSON.parse(JSON.stringify(windowTitlebarControls(context, pane))),
            JSON.parse(JSON.stringify(control?.props.controls ?? false)),
            JSON.stringify([platform, rootBand, own, pane])
          )
          assert.equal(
            windowHandoff(context, pane),
            !!handoff && pane === undefined
          )
          if (control)
            assert.equal(
              !!control.props.onClose,
              !!handoff && pane === undefined
            )
        }
    for (const controls of [
      undefined,
      false,
      ...states.flatMap(close =>
        states.flatMap(minimize =>
          states.map(maximize => ({ close, minimize, maximize }))
        )
      ),
    ]) {
      const resolved =
        controls === false
          ? false
          : exports.reference.controlsOf(
              platform,
              controls ?? { close: true, minimize: true, maximize: true }
            )
      const actual =
        controls === false || platform === 'omarchy'
          ? []
          : buttons(
              exports.reference.Controls({ platform, controls: resolved })
            )
      assert.deepEqual(
        windowControlItems(platform, windowControls(platform, controls)),
        actual,
        JSON.stringify([platform, controls])
      )
    }
  }
})

test('generated window slots and workspace callbacks retain their actual independent scene state', () => {
  const story = load(
    'example/entry/src/main/ets/generated/Stories.ets'
  ).STORIES.find(story => story.id === 'window-frame')
  assert.ok(story)
  const slots = story.examples[4]
  const state = new SceneState(slots.initial)
  const sibling = new SceneState(slots.initial)
  const pins = slots.content.items.map(
    frame => frame.fields.titlebar.fields.toolbar.items[0]
  )
  state.run(pins[0].fields.onPressedChange, booleanValue(true))
  assert.deepEqual(
    pins.map(pin => state.evaluate(pin.fields.pressed).boolean),
    [true, false, false]
  )
  assert.equal(sibling.evaluate(pins[0].fields.pressed).boolean, false)
  const accessory = story.examples[5]
  const accessoryState = new SceneState(accessory.initial)
  const segmented = accessory.content.fields.titlebar.fields.accessory
  accessoryState.run(segmented.fields.onValueChange, stringValue('board'))
  assert.equal(text(accessoryState.evaluate(segmented.fields.value)), 'board')
  const workspace = story.examples[6]
  const workState = new SceneState(workspace.initial)
  const descendants = node => [node, ...(node.items ?? []).flatMap(descendants)]
  const panes = workspace.content.items.map(frame =>
    descendants(frame).find(node => node.name === 'WindowTitlebar')
  )
  const first = descendants(workspace.content.items[0])
  const glossaries = first.find(
    node =>
      node.name === 'SidebarMenuButton' && node.items[0].text === 'Glossaries'
  )
  workState.run(glossaries.fields.onClick)
  assert.deepEqual(
    panes.map(pane => text(workState.evaluate(pane.fields.title))),
    ['Glossaries', 'Documents', 'Documents', 'Documents', 'Documents']
  )
  const searches = workspace.content.items.map(
    frame =>
      descendants(frame).find(node => node.name === 'WindowTitlebar').fields
        .toolbar
  )
  workState.run(searches[0].fields.onValueChange, stringValue('query'))
  assert.deepEqual(
    searches.map(search => text(workState.evaluate(search.fields.value))),
    ['query', '', '', '', '']
  )
})

test('window platform metrics remain the chrome values in the actual React stylesheet', () => {
  const source = fs.readFileSync(
    path.resolve(root, '../../js/dazzui/src/components/window-frame.css'),
    'utf8'
  )
  const block = selector => {
    const start = source.indexOf(selector)
    assert.ok(start >= 0, selector)
    return source.slice(
      source.indexOf('{', start) + 1,
      source.indexOf('}', start)
    )
  }
  const metric = (body, property) =>
    Number(new RegExp(`--${property}: ([0-9.]+)px;`).exec(body)?.[1])
  const sharedMac = block(
    ".dz-window-frame:is([data-platform='macos'], [data-platform='macos15'])"
  )
  const adwaita = block(
    ".dz-window-frame:is([data-platform='gnome'], [data-platform='ubuntu'])"
  )
  for (const platform of [
    'macos',
    'macos15',
    'windows',
    'gnome',
    'ubuntu',
    'kde',
  ]) {
    const own =
      platform === 'macos15'
        ? sharedMac
        : ['gnome', 'ubuntu'].includes(platform)
          ? adwaita
          : block(`.dz-window-frame[data-platform='${platform}']`)
    const chrome = windowChrome(platform)
    assert.equal(chrome.radius, metric(own, 'dz-window-radius'), platform)
    assert.equal(
      chrome.height,
      metric(own, 'dz-window-titlebar-height'),
      platform
    )
    if (platform.startsWith('macos')) {
      assert.equal(chrome.button, metric(own, 'dz-traffic-size'))
      assert.equal(chrome.gap, metric(own, 'dz-traffic-gap'))
      assert.equal(chrome.titleInset, metric(sharedMac, 'dz-traffic-room'))
    } else if (platform === 'windows') {
      assert.equal(
        chrome.button,
        Number(
          /width: ([0-9.]+)px/.exec(
            block('.dz-window-frame__caption > button')
          )[1]
        )
      )
    } else {
      for (const [key, property] of [
        ['button', 'dz-headerbar-button-size'],
        ['gap', 'dz-headerbar-gap'],
        ['inset', 'dz-headerbar-inset'],
        ['titleInset', 'dz-window-title-inset'],
        ['trailingInset', 'dz-window-controls-inset'],
      ])
        assert.equal(chrome[key], metric(own, property), `${platform} ${key}`)
    }
  }
})

const sliderReference = file =>
  require(
    path.resolve(
      root,
      '../../js/dazzui/node_modules/@base-ui/react/slider/',
      file
    )
  )
test('range pointer collisions and step rounding match the installed Base UI implementation', () => {
  const { resolveThumbCollision } = sliderReference(
    'utils/resolveThumbCollision.js'
  )
  const { roundValueToStep } = sliderReference('utils/roundValueToStep.js')
  for (const [min, max, step] of [
    [0, 100, 1],
    [-2.5, 3.5, 0.1],
    [0.001, 0.009, 0.0001],
    [0, 0.3, 0.2],
  ]) {
    for (const initial of [
      [min],
      [min, max],
      [min, min, max],
      [max, min, (min + max) / 2],
    ]) {
      const before = initial.slice()
      let current = sliderValues(initial, min, max)
      for (let index = 0; index < current.length; index++) {
        for (const fraction of [-0.1, 0, 0.1, 0.5, 0.9, 1, 1.1, 0.6, 0.2]) {
          const raw = min + fraction * (max - min)
          const rounded = Math.max(
            min,
            Math.min(max, roundValueToStep(raw, step, min))
          )
          assert.equal(sliderSnap(raw, min, max, step), rounded)
          const resolved = resolveThumbCollision(
            'push',
            current,
            current,
            current,
            index,
            rounded,
            min,
            max,
            step,
            0
          ).value
          const expected = Array.isArray(resolved) ? resolved : [resolved]
          current = sliderChange(current, index, rounded, min, max, true)
          assert.deepEqual(current, expected)
        }
      }
      assert.deepEqual(initial, before, 'Caller arrays remain unchanged')
    }
  }
  assert.equal(sliderNearest([20, 80], 50, 100), 1)
  assert.equal(sliderNearest([50, 50], 50, 100), 1)
  assert.equal(sliderNearest([100, 100], 100, 100), 0)
})

test('native slider keyboard proposals match the actual React thumb handler, including neighbour limits', () => {
  const filename = path.resolve(
    root,
    '../../js/dazzui/node_modules/@base-ui/react/slider/thumb/SliderThumb.js'
  )
  const source = ts.createSourceFile(
    filename,
    fs.readFileSync(filename, 'utf8'),
    ts.ScriptTarget.Latest,
    true
  )
  let handler
  let increment
  function visit(node) {
    if (
      ts.isMethodDeclaration(node) &&
      node.name.getText(source) === 'onKeyDown' &&
      node.getText(source).includes('let increment')
    )
      handler = node.getText(source)
    if (ts.isFunctionDeclaration(node) && node.name?.text === 'getNewValue')
      increment = node.getText(source)
    ts.forEachChild(node, visit)
  }
  visit(source)
  assert.ok(
    handler && increment,
    'Locate actual Base UI keyboard implementation'
  )
  const rounding = sliderReference('utils/roundValueToStep.js')
  const { getSliderValue } = sliderReference('utils/getSliderValue.js')
  const keys = [
    'ArrowUp',
    'ArrowDown',
    'ArrowLeft',
    'ArrowRight',
    'PageUp',
    'PageDown',
    'Home',
    'End',
  ]
  const composite = {
    ARROW_UP: 'ArrowUp',
    ARROW_DOWN: 'ArrowDown',
    ARROW_LEFT: 'ArrowLeft',
    ARROW_RIGHT: 'ArrowRight',
    PAGE_UP: 'PageUp',
    PAGE_DOWN: 'PageDown',
    HOME: 'Home',
    END: 'End',
    COMPOSITE_KEYS: new Set(keys),
  }
  for (const [min, max, step, values] of [
    [0, 100, 1, [20, 80]],
    [-2.5, 3.5, 0.1, [-1.2, 2.3]],
    [0, 0.3, 0.2, [0.1, 0.3]],
    [0, 100, 1, [50, 50]],
    [0, 100, 1, [100]],
  ]) {
    for (let index = 0; index < values.length; index++)
      for (const key of keys)
        for (const shift of [true, false]) {
          let expected
          const context = {
            _composite: composite,
            ALL_KEYS: new Set(keys),
            _roundValueToStep: rounding,
            _clamp: {
              clamp: (n, low, high) => Math.max(low, Math.min(high, n)),
            },
            _element: { matchesFocusVisible: () => true },
            thumbValue: values[index],
            sliderValues: values,
            index,
            range: values.length > 1,
            rtl: false,
            min,
            max,
            step,
            largeStep: 10,
            minStepsBetweenValues: 0,
            handleInputChange: value => {
              expected = getSliderValue(value, index, min, max, true, values)
            },
          }
          const reference = vm.runInNewContext(
            `(function() { ${increment}; return function ${handler} })()`,
            context
          )
          reference({
            key,
            shiftKey: shift,
            defaultPrevented: false,
            currentTarget: {},
            nativeEvent: {},
            stopPropagation() {},
            preventDefault() {},
          })
          assert.deepEqual(
            sliderKey(values, index, key, shift, min, max, step, 10),
            expected,
            JSON.stringify({ values, key, shift })
          )
        }
  }
  assert.equal(sliderKey([20, 80], 0, 'Tab', false, 0, 100, 1, 10), undefined)
})

test('scoped pointer observation deduplicates native samples and removes press, motion and release listeners', () => {
  const pointer = new DazzPointerContext()
  const events = []
  const press = (x, y) => events.push(['press', x, y])
  const move = (x, y) => events.push(['move', x, y])
  const release = () => events.push(['release'])
  pointer.on(press)
  pointer.on(press)
  pointer.onMove(move)
  pointer.onRelease(release)
  pointer.press(10, 20, 100)
  pointer.press(10, 20, 100)
  pointer.move(30, 40, 100)
  pointer.move(30, 40, 100)
  pointer.release()
  pointer.press(10, 20, 101)
  assert.deepEqual(events, [
    ['press', 10, 20],
    ['move', 30, 40],
    ['release'],
    ['press', 10, 20],
  ])
  pointer.off(press)
  pointer.offMove(move)
  pointer.offRelease(release)
  pointer.press(50, 60)
  pointer.move(70, 80)
  pointer.release()
  assert.equal(events.length, 4)
})

test('detached overlay presses retain other outside observers without clearing their own viewport', () => {
  const pointer = new DazzPointerContext()
  const events = []
  const first = () => events.push('first outside')
  const second = () => events.push('second outside')
  const preview = () => events.push('preview outside')
  pointer.on(first)
  pointer.on(second)
  pointer.on(preview)
  pointer.press(100, 200, 1, first)
  pointer.press(100, 200, 1)
  assert.deepEqual(events, ['second outside', 'preview outside'])
  pointer.press(150, 200, 2, second)
  assert.deepEqual(events.slice(2), ['first outside', 'preview outside'])
  pointer.off(first)
  pointer.press(250, 300, 3, preview)
  assert.deepEqual(events.slice(4), ['second outside'])
})

test('detached menu hover reset does not fabricate an outside press or retain removed controls', () => {
  const pointer = new DazzPointerContext()
  const events = []
  pointer.on(() => events.push('outside press'))
  const reset = () => {
    events.push('hover reset')
    pointer.offTouchHoverReset(reset)
  }
  pointer.onTouchHoverReset(reset)
  pointer.resetTouchHover()
  pointer.resetTouchHover()
  assert.deepEqual(events, ['hover reset'])
  pointer.press(10, 20)
  assert.deepEqual(events, ['hover reset', 'outside press'])
})

function previewFixture(initial = false) {
  let time = 0
  let nextId = 0
  const jobs = new Map()
  const state = { open: initial, accept: true, proposals: [] }
  const clock = {
    schedule(action, delay) {
      const id = ++nextId
      jobs.set(id, { action, due: time + delay })
      return id
    },
    cancel(id) {
      jobs.delete(id)
    },
  }
  const interaction = new DazzPreviewInteraction(clock)
  interaction.current = () => state.open
  interaction.change = value => {
    state.proposals.push(value)
    if (state.accept) state.open = value
  }
  const tokens = new DazzTokens()
  interaction.openDelay = () => tokens.metric('preview-card.open-delay')
  interaction.closeDelay = () => tokens.metric('preview-card.close-delay')
  const tick = elapsed => {
    const end = time + elapsed
    for (;;) {
      const ready = [...jobs]
        .sort((a, b) => a[1].due - b[1].due)
        .find(([, job]) => job.due <= end)
      if (!ready) break
      const [id, job] = ready
      time = job.due
      jobs.delete(id)
      job.action()
    }
    time = end
  }
  return { state, interaction, tick, jobs, clock, now: () => time }
}

const { DazzTooltipInteraction } = load(
  'src/main/ets/foundation/TooltipData.ets'
)
function tooltipFixture() {
  const { clock, tick, jobs } = previewFixture()
  const interaction = new DazzTooltipInteraction(clock)
  const state = { open: false, disabled: false }
  const tokens = new DazzTokens()
  interaction.current = () => state.open
  interaction.change = value => {
    state.open = value
  }
  interaction.disabled = () => state.disabled
  interaction.openDelay = () => tokens.metric('tooltip.open-delay')
  interaction.closeDelay = () => tokens.metric('tooltip.close-delay')
  return { interaction, state, tick, jobs }
}

test('tooltip tokens include the installed Base UI rest delay in every palette', () => {
  const reference = require(
    path.resolve(
      root,
      '../../js/dazzui/node_modules/@base-ui/react/tooltip/utils/constants.js'
    )
  )
  const trigger = fs.readFileSync(
    path.resolve(
      root,
      '../../js/dazzui/node_modules/@base-ui/react/tooltip/trigger/TooltipTrigger.js'
    ),
    'utf8'
  )
  const close = /closeDelayWithDefault\s*=\s*closeDelay\s*\?\?\s*(\d+)/.exec(
    trigger
  )
  assert.ok(close, 'Locate the installed trigger close-delay default')
  for (const name of THEME_NAMES) {
    const tokens = new DazzTokens(name)
    assert.equal(tokens.metric('tooltip.open-delay'), reference.OPEN_DELAY)
    assert.equal(tokens.metric('tooltip.close-delay'), Number(close[1]))
  }
})

test('tooltip hover waits for pointer rest, ignores jitter and cancels on exit or press', () => {
  const { interaction: tip, state, tick, jobs } = tooltipFixture()
  tip.hover(true)
  tip.move(0, 0)
  tick(500)
  tip.move(0.5, 0.5)
  tick(99)
  assert.equal(state.open, false)
  tick(1)
  assert.equal(state.open, true)
  tip.hover(false)
  assert.equal(state.open, false)
  tip.hover(true)
  tip.move(0, 0)
  tick(500)
  tip.move(10, 10)
  tick(599)
  assert.equal(state.open, false)
  tick(1)
  assert.equal(state.open, true)
  tip.hover(false)
  tip.hover(true)
  tick(599)
  tip.hover(false)
  tick(1000)
  assert.equal(state.open, false)
  tip.hover(true)
  tip.dismiss()
  tip.move(100, 100)
  tick(1000)
  assert.equal(state.open, false)
  assert.equal(jobs.size, 0)
  tip.hover(false)
  tip.hover(true)
  tick(600)
  assert.equal(state.open, true)
})

test('tooltip focus is immediate, Escape waits for refocus and disabled hints preserve cancellation', () => {
  const { interaction: tip, state, tick, jobs } = tooltipFixture()
  tip.focus(true)
  assert.equal(state.open, true)
  tip.dismiss()
  tip.focus(true)
  tick(1000)
  assert.equal(state.open, false)
  tip.focus(false)
  tip.focus(true)
  assert.equal(state.open, true)
  state.disabled = true
  tip.configure()
  assert.equal(state.open, false)
  tip.focus(false)
  tip.focus(true)
  tip.hover(true)
  tick(1000)
  assert.equal(state.open, false)
  tip.dispose()
  assert.equal(jobs.size, 0)
})

test('tooltip custom close delay is cancelled by reentry and pending opens die with their owner', () => {
  const { interaction: tip, state, tick, jobs } = tooltipFixture()
  tip.openDelay = () => 0
  tip.closeDelay = () => 200
  tip.hover(true)
  assert.equal(state.open, true)
  tip.hover(false)
  tick(199)
  assert.equal(state.open, true)
  tip.hover(true)
  tick(200)
  assert.equal(state.open, true)
  tip.hover(false)
  tick(200)
  assert.equal(state.open, false)
  tip.openDelay = () => 600
  tip.hover(true)
  tip.dispose()
  tick(1000)
  assert.equal(state.open, false)
  assert.equal(jobs.size, 0)
})

test('preview cursor-origin polygons match the installed Base UI handler, intent timer and velocity', () => {
  const source = fs.readFileSync(
    path.resolve(
      root,
      '../../js/dazzui/node_modules/@base-ui/react/floating-ui-react/safePolygon.js'
    ),
    'utf8'
  )
  const rectangle = rect => ({
    ...rect,
    x: rect.left,
    y: rect.top,
    width: rect.right - rect.left,
    height: rect.bottom - rect.top,
  })
  const reference = { left: 100, top: 100, right: 140, bottom: 120 }
  const fixtures = [
    [
      'bottom',
      { x: 138, y: 121 },
      { left: 40, top: 160, right: 200, bottom: 240 },
    ],
    ['top', { x: 138, y: 99 }, { left: 40, top: -20, right: 200, bottom: 60 }],
    ['left', { x: 99, y: 118 }, { left: -20, top: 60, right: 60, bottom: 160 }],
    [
      'right',
      { x: 141, y: 118 },
      { left: 180, top: 60, right: 260, bottom: 160 },
    ],
    [
      'bottom',
      { x: 138, y: 121 },
      { left: 110, top: 160, right: 130, bottom: 180 },
    ],
    ['top', { x: 102, y: 99 }, { left: 110, top: 60, right: 130, bottom: 80 }],
    ['left', { x: 99, y: 102 }, { left: 60, top: 105, right: 80, bottom: 115 }],
    [
      'right',
      { x: 141, y: 118 },
      { left: 180, top: 105, right: 200, bottom: 115 },
    ],
  ]
  let compared = 0
  for (const [side, origin, popup] of fixtures) {
    for (const elapsed of [0, 4, 25, 41]) {
      for (let x = 60; x <= 200; x += 14)
        for (let y = 60; y <= 180; y += 12) {
          const { clock, tick, now } = previewFixture()
          let actualClosed = 0
          let referenceClosed = 0
          const actual = new DazzPreviewPolygon(clock, now)
          actual.onClose = () => {
            actualClosed++
          }
          const exports = {}
          class Timeout {
            id = -1
            clear() {
              clock.cancel(this.id)
            }
            start(delay, action) {
              this.clear()
              this.id = clock.schedule(action, delay)
            }
          }
          vm.runInNewContext(source, {
            exports,
            performance: { now },
            require: name => {
              if (name === '@base-ui/utils/useTimeout') return { Timeout }
              if (name === '@floating-ui/utils/dom')
                return { isElement: target => !!target }
              if (name === './utils/element')
                return {
                  contains: (parent, target) => parent === target,
                  getTarget: event => event.target,
                }
              if (name === './utils/nodes') return { getNodeChildren: () => [] }
              throw new Error(name)
            },
          })
          const domReference = {
            getBoundingClientRect: () => rectangle(reference),
          }
          const floating = { getBoundingClientRect: () => rectangle(popup) }
          const expected = exports.safePolygon()({
            ...origin,
            placement: side,
            elements: { domReference, floating },
            onClose: () => {
              referenceClosed++
            },
          })
          const sample = (point, leave = false) => {
            const inside = rect =>
              point.x >= rect.left &&
              point.x <= rect.right &&
              point.y >= rect.top &&
              point.y <= rect.bottom
            const target =
              leave || inside(reference)
                ? 'reference'
                : inside(popup)
                  ? 'popup'
                  : 'outside'
            expected({
              clientX: point.x,
              clientY: point.y,
              type: leave ? 'mouseleave' : 'mousemove',
              target:
                target === 'reference'
                  ? domReference
                  : target === 'popup'
                    ? floating
                    : null,
            })
            actual.move(point, reference, popup, target, leave)
            assert.equal(
              actualClosed,
              referenceClosed,
              JSON.stringify({ side, origin, point, elapsed, leave })
            )
          }
          actual.begin(origin, side)
          sample(origin, true)
          if (!actualClosed) {
            tick(elapsed)
            assert.equal(actualClosed, referenceClosed)
            if (!actualClosed) {
              sample({ x, y })
              if (!actualClosed) {
                tick(20)
                sample({ x: x + 0.01, y: y + 0.01 })
                tick(40)
                assert.equal(actualClosed, referenceClosed)
              }
            }
          }
          compared++
        }
    }
  }
  assert.equal(compared, 3872)
})

test('preview timing follows the actual Base UI defaults in every palette', () => {
  const reference = require(
    path.resolve(
      root,
      '../../js/dazzui/node_modules/@base-ui/react/preview-card/utils/constants.js'
    )
  )
  for (const name of THEME_NAMES) {
    const tokens = new DazzTokens(name)
    assert.equal(tokens.metric('preview-card.open-delay'), reference.OPEN_DELAY)
    assert.equal(
      tokens.metric('preview-card.close-delay'),
      reference.CLOSE_DELAY
    )
  }
})

test('preview hover intent, card traversal, keyboard focus and disposal retain their independent lifetimes', () => {
  const { state, interaction: card, tick, jobs } = previewFixture()
  card.hover(false, true)
  tick(599)
  assert.equal(state.open, false)
  card.hover(false, false)
  tick(1000)
  assert.deepEqual(
    state.proposals,
    [],
    'Leaving before the delay does not open a card'
  )
  card.hover(false, true)
  tick(600)
  assert.equal(state.open, true)
  card.hover(false, false)
  card.transit(true)
  tick(3000)
  assert.equal(
    state.open,
    true,
    'The intent handler can retain a card while the pointer travels'
  )
  card.hover(true, true)
  card.transit(false)
  tick(1000)
  assert.equal(state.open, true, 'The popup itself is interactive')
  card.hover(true, false)
  tick(299)
  assert.equal(state.open, true)
  tick(1)
  assert.equal(state.open, false)
  card.focus(false, true)
  tick(600)
  assert.equal(state.open, true)
  card.focus(false, false)
  card.focus(true, true)
  tick(0)
  assert.equal(state.open, true, 'Focus can move from trigger to card')
  card.focus(true, false)
  tick(0)
  assert.equal(state.open, false)
  card.hover(false, true)
  card.dispose()
  assert.equal(jobs.size, 0)
  tick(1000)
  assert.equal(state.open, false)
})

test('preview controlled refusal and initial open state are not replaced by a local draft', () => {
  const { state, interaction: card, tick } = previewFixture(true)
  card.transit(false)
  tick(1000)
  assert.equal(
    state.open,
    true,
    'An initially open card has no pending pointer traversal'
  )
  state.accept = false
  card.dismiss()
  assert.equal(state.open, true)
  assert.deepEqual(state.proposals, [false])
  card.focus(false, true)
  tick(1000)
  assert.deepEqual(
    state.proposals,
    [false],
    'Escape cannot reopen the same focused trigger'
  )
  state.open = false
  card.dispose()
  card.focus(false, false)
  card.hover(false, true)
  tick(600)
  assert.equal(state.open, false)
  assert.deepEqual(
    state.proposals,
    [false, true],
    'Only the host accepts a controlled proposal'
  )
})

test('ending polygon traversal starts one close delay that outside movement cannot cancel', () => {
  const { state, interaction: card, tick } = previewFixture()
  card.hover(false, true)
  tick(600)
  card.hover(false, false)
  card.transit(true)
  tick(300)
  assert.equal(state.open, true)
  card.finishTransit()
  card.transit(true)
  tick(299)
  assert.equal(state.open, true)
  tick(1)
  assert.equal(state.open, false)
  assert.deepEqual(state.proposals, [true, false])
})

test('touch inside a preview survives native hover cancellation and trigger-to-popup focus handoff', () => {
  const { state, interaction: card, tick } = previewFixture()
  card.hover(false, true)
  tick(600)
  card.touchInside(true)
  card.hover(false, false)
  card.transit(false)
  card.focus(false, false)
  tick(1000)
  assert.equal(
    state.open,
    true,
    'The touch target survives until its focus is established'
  )
  card.focus(true, true)
  card.touchInside(false)
  tick(1000)
  assert.equal(
    state.open,
    true,
    'Native popup focus retains an interactive editor'
  )
  card.focus(true, false)
  tick(0)
  assert.equal(
    state.open,
    false,
    'Leaving the editor releases the touch retention'
  )
})

test('one outside dismissal is not proposed again by its ensuing native blur while controlled props are pending', () => {
  const { state, interaction: card, tick } = previewFixture(true)
  state.accept = false
  card.focus(true, true)
  card.dismiss()
  card.focus(true, false)
  card.hover(false, false)
  tick(1000)
  assert.deepEqual(state.proposals, [false])
  card.dismiss()
  assert.deepEqual(
    state.proposals,
    [false, false],
    'A separate explicit dismissal remains observable'
  )
  card.hover(false, true)
  card.hover(false, false)
  tick(300)
  assert.deepEqual(
    state.proposals,
    [false, false, false],
    'A fresh pointer traversal can close again'
  )
})

test('inline previews preserve sentence text, UTF-16 ranges and live state while excluding popup prose', () => {
  const state = new SceneState({ name: stringValue('@ada') })
  const prefix = 'Reviewed 😀 by '
  const node = {
    kind: 'element',
    name: 'Text',
    items: [
      stringValue(prefix),
      {
        kind: 'element',
        name: 'PreviewCard',
        address: 2,
        fields: {
          trigger: {
            kind: 'element',
            name: 'Link',
            fields: { href: stringValue('#/preview-card') },
            items: [{ kind: 'state', text: 'name' }],
          },
        },
        items: [
          {
            kind: 'element',
            name: 'Text',
            items: [stringValue('Private preview body')],
          },
        ],
      },
      stringValue(' three days ago.'),
      { kind: 'element', name: 'Code', items: [stringValue('code')] },
    ],
  }
  const first = inlineRuns(node, state)
  assert.equal(
    first.map(run => run.label).join(''),
    prefix + '@ada three days ago.code'
  )
  assert.equal(first[1].start, prefix.length)
  assert.equal(first[1].end, prefix.length + 4)
  assert.equal(first[1].href, '#/preview-card')
  assert.equal(first[1].preview.address, 2)
  assert.equal(first.at(-1).code, true)
  state.run({
    items: [
      { fields: { set: stringValue('name'), to: stringValue('@grace') } },
    ],
  })
  const second = inlineRuns(node, state)
  assert.equal(second[1].label, '@grace')
  assert.equal(second[2].start, prefix.length + 6)
  assert.equal(first[1].label, '@ada')
})

test('sidebar window shortcuts toggle each live provider and clean up independently', () => {
  const calls = []
  let changes = 0
  const observe = () => {
    changes++
  }
  observeSidebarShortcuts(observe)
  registerSidebarShortcut('window-a', 'first', () => calls.push('first'))
  registerSidebarShortcut('window-a', 'second', () => calls.push('second'))
  registerSidebarShortcut('window-b', 'third', () => calls.push('third'))
  assert.equal(hasWindowSidebars('window-a'), true)
  assert.equal(hasWindowSidebars('missing-window'), false)
  assert.equal(toggleWindowSidebars('window-a'), true)
  assert.deepEqual(calls, ['first', 'second'])
  unregisterSidebarShortcut('first')
  registerSidebarShortcut('window-a', 'second', () => calls.push('replacement'))
  toggleWindowSidebars('window-a')
  assert.deepEqual(calls, ['first', 'second', 'replacement'])
  toggleWindowSidebars('window-b')
  assert.equal(calls.at(-1), 'third')
  assert.equal(toggleWindowSidebars('missing-window'), false)
  unregisterSidebarShortcut('second')
  unregisterSidebarShortcut('third')
  assert.equal(toggleWindowSidebars('window-a'), false)
  assert.equal(toggleWindowSidebars('window-b'), false)
  assert.equal(hasWindowSidebars('window-a'), false)
  assert.ok(changes >= 6)
  unobserveSidebarShortcuts(observe)
  const before = changes
  registerSidebarShortcut('window-c', 'later', () => {})
  unregisterSidebarShortcut('later')
  assert.equal(changes, before, 'unmounted theme observers receive no updates')
  assert.equal(isSidebarShortcut(2018, 98, true, false), true)
  assert.equal(isSidebarShortcut(2018, 98, false, true), true)
  assert.equal(isSidebarShortcut(2018, 98, false, false), false)
  assert.equal(isSidebarShortcut(2018, 66, true, false), false)
  assert.equal(isSidebarShortcut(2018, 0, true, false), true)
  assert.equal(isSidebarShortcut(2018, 0, true, false, true), false)
  assert.equal(isSidebarShortcut(2018, 0, true, false, false, true), false)
})

test('tree visible hierarchy matches the actual React flattener', () => {
  const filename = path.resolve(root, '../../js/dazzui/src/components/tree.tsx')
  const source = ts.createSourceFile(
    filename,
    fs.readFileSync(filename, 'utf8'),
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TSX
  )
  const flatten = source.statements.find(
    node => ts.isFunctionDeclaration(node) && node.name?.text === 'flatten'
  )
  assert.ok(flatten)
  const compiled = ts.transpileModule(flatten.getText(source), {
    compilerOptions: { target: ts.ScriptTarget.ES2022 },
  }).outputText
  const reference = vm.runInThisContext(
    `(function() { ${compiled}; return flatten })()`
  )
  const items = [
    {
      id: 'src',
      label: 'src',
      children: [
        { id: 'empty', label: 'Empty', children: [] },
        {
          id: 'components',
          label: 'Components',
          children: [
            { id: 'button', label: 'Button.tsx' },
            { id: 'tree', label: 'Tree.tsx', text: ' file tree ' },
          ],
        },
      ],
    },
    { id: 'locked', label: 'Locked', disabled: true },
    { id: 'readme', label: 'README.md' },
  ]
  for (let mask = 0; mask < 8; mask++) {
    const expanded = ['src', 'empty', 'components'].filter(
      (_, i) => mask & (1 << i)
    )
    assert.deepEqual(
      flattenTree(items, expanded),
      reference(items, new Set(expanded))
    )
  }
  const rows = flattenTree(items, ['src', 'components'])
  assert.deepEqual(treeNavigation(rows, 'src', 2014), {
    handled: true,
    expand: false,
  })
  assert.deepEqual(treeNavigation(rows, 'src', 2015), {
    handled: true,
    focus: 'empty',
  })
  assert.deepEqual(treeNavigation(rows, 'empty', 2015), {
    handled: true,
    expand: true,
  })
  const emptyOpen = flattenTree(items, ['src', 'components', 'empty'])
  assert.deepEqual(treeNavigation(emptyOpen, 'empty', 2015), {
    handled: true,
    focus: 'components',
  })
  assert.deepEqual(treeNavigation(rows, 'button', 2014), {
    handled: true,
    focus: 'components',
  })
  assert.deepEqual(treeNavigation(rows, 'tree', 2013), {
    handled: true,
    focus: undefined,
  })
  assert.deepEqual(treeNavigation(rows, 'readme', 2012), {
    handled: true,
    focus: undefined,
  })
  assert.deepEqual(treeNavigation(rows, 'locked', 2054), {
    handled: true,
    select: undefined,
    expand: undefined,
  })
  assert.deepEqual(treeNavigation(rows, 'components', 2050), {
    handled: true,
    select: 'components',
    expand: false,
  })
  assert.deepEqual(treeNavigation(rows, 'tree', 2081), {
    handled: true,
    focus: 'src',
  })
  assert.deepEqual(treeNavigation(rows, 'tree', 2082), {
    handled: true,
    focus: 'readme',
  })
  for (const key of [2049, 2070, 2017])
    assert.deepEqual(treeNavigation(rows, 'src', key), { handled: false })
  assert.equal(treeTypeahead(rows, 'readme', 'comp'), 'components')
  assert.equal(treeTypeahead(rows, 'src', 'file t'), 'tree')
  assert.equal(treeTypeahead(rows, 'src', 'locked'), undefined)
  assert.equal(treeTypeahead(flattenTree(items, []), 'src', 'comp'), undefined)
})

test('numeric OTP normalization, overwrite and deletion match actual Base UI', () => {
  const reference = require(
    path.resolve(
      root,
      '../../js/dazzui/node_modules/@base-ui/react/otp-field/utils/otp.js'
    )
  )
  const samples = ['', '1', '1234', '987654321', '1 a2-3\n4', '１２3 4', 'x💡']
  for (const length of [1, 4, 6, 8]) {
    for (const value of samples) {
      assert.equal(
        normalizeOtp(value, length),
        reference.normalizeOTPValue(value, length, 'numeric')
      )
      for (let index = 0; index <= length; index++) {
        assert.equal(
          removeOtp(value, index),
          reference.removeOTPCharacter(value, index)
        )
        for (const inserted of samples) {
          assert.equal(
            replaceOtp(value, index, inserted, length),
            reference.replaceOTPValue(value, index, inserted, length, 'numeric')
          )
        }
      }
    }
  }
})
test('toast drag traces match the actual Base UI pointer handlers, thresholds, damping and cancellation', () => {
  const filename = path.resolve(
    root,
    '../../js/dazzui/node_modules/@base-ui/react/toast/root/ToastRoot.js'
  )
  const source = ts.createSourceFile(
    filename,
    fs.readFileSync(filename, 'utf8'),
    ts.ScriptTarget.Latest,
    true
  )
  const names = [
    'applyDirectionalDamping',
    'handlePointerDown',
    'handlePointerMove',
    'setResolvedDragOffset',
  ]
  const declarations = new Map()
  let end
  function visit(node) {
    if (ts.isFunctionDeclaration(node) && names.includes(node.name?.text))
      declarations.set(node.name.text, node.getText(source))
    if (
      ts.isVariableDeclaration(node) &&
      [
        'SWIPE_THRESHOLD',
        'REVERSE_CANCEL_THRESHOLD',
        'OPPOSITE_DIRECTION_DAMPING_FACTOR',
        'MIN_DRAG_THRESHOLD',
        'TOAST_SWIPE_IGNORE_SELECTOR',
      ].includes(node.name.getText(source))
    )
      declarations.set(
        node.name.getText(source),
        'const ' + node.getText(source)
      )
    if (
      ts.isVariableDeclaration(node) &&
      node.name.getText(source) === 'handleSwipeEnd'
    )
      end = node.initializer.arguments[0].getText(source)
    ts.forEachChild(node, visit)
  }
  visit(source)
  assert.equal(declarations.size, 9)
  assert.ok(end)
  const reference = initial =>
    vm.runInNewContext(
      `(() => {
    const swipeDirections = ['down', 'right'];
    let currentSwipeDirection, isSwiping = false, isRealSwipe = false, lockedDirection = null;
    let initialTransform = initial, dragOffset = initial, closed = 0;
    const dragStartPosRef = { current: {x: 0, y: 0} };
    const initialTransformRef = { current: initial };
    const intendedSwipeDirectionRef = { current: undefined };
    const maxSwipeDisplacementRef = { current: 0 };
    const cancelledSwipeRef = { current: false };
    const swipeCancelBaselineRef = { current: {x: 0, y: 0} };
    const isFirstPointerMoveRef = { current: false };
    const dragOffsetRef = { current: initial };
    const activePointerIdRef = { current: null };
    const dragAbortControllerRef = { current: null };
    const store = { pauseTimers() {}, set() {}, closeToast() { closed++; } };
    const toast = { id: 'reference' };
    const element = { setPointerCapture() {} };
    const setCurrentSwipeDirection = value => { currentSwipeDirection = value; };
    const setIsSwiping = value => { isSwiping = value; };
    const setIsRealSwipe = value => { isRealSwipe = value; };
    const setLockedDirection = value => { lockedDirection = value; };
    const setInitialTransform = value => { initialTransform = value; };
    const setDragOffset = value => { dragOffset = value; };
    ${Array.from(declarations.values()).join('\n')}
    const handleSwipeEnd = ${end};
    return {
      begin(id, x, y, button = 0, interactive = false) {
        handlePointerDown({button, pointerType: 'touch', pointerId: id, clientX: x, clientY: y,
          nativeEvent: {target: {closest() { return interactive; }}}, currentTarget: element});
      },
      move(id, x, y, mx, my) { handlePointerMove({pointerId: id, clientX: x, clientY: y, movementX: mx, movementY: my, preventDefault() {}}); },
      end(id, cancel) { const previous = closed; handleSwipeEnd({pointerId: id, type: cancel ? 'pointercancel' : 'pointerup'}); return closed > previous; },
      snapshot() { return {active: isSwiping, real: isRealSwipe, locked: lockedDirection || '', cancelled: cancelledSwipeRef.current, transform: {...dragOffset, scale: initialTransform.scale}}; }
    };
  })()`,
      {
        initial,
        AbortController: class {
          abort() {}
        },
        _owner: { ownerDocument: () => ({ addEventListener() {} }) },
        _utils: { getTarget: event => event.target },
        _constants: require(
          path.resolve(
            root,
            '../../js/dazzui/node_modules/@base-ui/react/internals/constants.js'
          )
        ),
        _useSwipeDismiss: {
          ...require(
            path.resolve(
              root,
              '../../js/dazzui/node_modules/@base-ui/react/utils/useSwipeDismiss.js'
            )
          ),
          getElementTransform: () => ({ ...initial }),
        },
      }
    )
  const checkTrace = (
    points,
    cancel = false,
    initial = { x: 0, y: 0, scale: 1 }
  ) => {
    const actual = new DazzToastSwipe()
    const expected = reference(initial)
    const snapshot = () => ({
      active: actual.active,
      real: actual.real,
      locked: actual.locked,
      cancelled: actual.cancelled,
      transform: actual.transform,
    })
    const compare = () =>
      assert.deepEqual(
        snapshot(),
        JSON.parse(JSON.stringify(expected.snapshot())),
        JSON.stringify({ points, cancel, initial })
      )
    actual.begin(2, 0, 0, initial)
    expected.begin(2, 0, 0)
    compare()
    actual.move(99, 1000, 1000, 1000, 1000)
    expected.move(99, 1000, 1000, 1000, 1000)
    compare()
    let previous = [0, 0]
    for (const point of points) {
      const args = [2, ...point, point[0] - previous[0], point[1] - previous[1]]
      actual.move(...args)
      expected.move(...args)
      compare()
      previous = point
    }
    assert.equal(
      actual.end(99),
      expected.end(99, false),
      'Another pointer cannot release the toast'
    )
    compare()
    assert.equal(
      actual.end(2, cancel),
      expected.end(2, cancel),
      'Actual threshold/cancellation decision'
    )
    compare()
  }
  for (const initial of [
    { x: 0, y: 0, scale: 1 },
    { x: 2, y: 7.5, scale: 0.99 },
  ]) {
    for (const distance of [0, 0.5, 1, 9, 10, 39, 40, 40.001, 41, 80]) {
      for (const [x, y] of [
        [1, 0],
        [0, 1],
        [-1, 0],
        [0, -1],
        [1, 1],
        [1, -1],
        [-1, 1],
      ]) {
        const points = [
          [x, y],
          [x * (1 + distance), y * (1 + distance)],
        ]
        checkTrace(points, false, initial)
        checkTrace(points, true, initial)
      }
    }
  }
  for (const points of [
    [
      [1, 0],
      [10, 0],
      [90, 0],
      [79, 0],
      [85, 0],
    ],
    [
      [1, 0],
      [90, 0],
      [55, 0],
      [80, 0],
      [130, 0],
    ],
    [
      [0, 1],
      [0, 20],
      [0, 120],
      [0, 91],
      [0, 100],
    ],
    [
      [0, 1],
      [0, -90],
      [0, 100],
    ],
    [
      [1, 0],
      [20, 1],
      [20, 160],
    ],
  ])
    checkTrace(points)
  let seed = 9137
  const random = () => {
    seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0
    return seed / 2 ** 32
  }
  for (let trace = 0; trace < 256; trace++) {
    const points = Array.from({ length: 8 }, () => [
      Math.round(random() * 240 - 120),
      Math.round(random() * 240 - 120),
    ])
    checkTrace(points, trace % 3 === 0)
  }
  for (const [button, interactive] of [
    [2, false],
    [0, true],
  ]) {
    const actual = new DazzToastSwipe()
    const expected = reference({ x: 0, y: 0, scale: 1 })
    assert.equal(
      actual.begin(1, 0, 0, { x: 0, y: 0, scale: 1 }, button, interactive),
      false
    )
    expected.begin(1, 0, 0, button, interactive)
    assert.equal(actual.active, expected.snapshot().active)
  }
})

test('toast F6 dispatch matches the actual Base UI window listeners across viewports and windows', () => {
  const filename = path.resolve(
    root,
    '../../js/dazzui/node_modules/@base-ui/react/toast/viewport/ToastViewport.js'
  )
  const source = ts.createSourceFile(
    filename,
    fs.readFileSync(filename, 'utf8'),
    ts.ScriptTarget.Latest,
    true
  )
  let handler
  function visit(node) {
    if (
      ts.isFunctionDeclaration(node) &&
      node.name?.text === 'handleGlobalKeyDown'
    )
      handler = node.getText(source)
    ts.forEachChild(node, visit)
  }
  visit(source)
  assert.ok(handler)
  const windows = new Map()
  const elements = new Map()
  const native = []
  const expected = []
  const listeners = []
  for (const [window, viewport] of [
    ['toast-a', 'one'],
    ['toast-b', 'other'],
    ['toast-a', 'two'],
  ]) {
    if (!windows.has(window))
      windows.set(window, { active: { id: window + '-button' } })
    const doc = windows.get(window)
    const element = {
      id: viewport + '-focus',
      focus() {
        doc.active = element
      },
    }
    elements.set(element.id, element)
    const store = {
      set(key, value) {
        if (key === 'prevFocusElement') expected.push([viewport, value.id])
      },
      pauseTimers() {},
    }
    const listener = vm.runInNewContext('(' + handler + ')', {
      store,
      doc,
      viewport: element,
      _utils: {
        getTarget: event => event.target,
        activeElement: doc => doc.active,
      },
    })
    listeners.push({ window, listener })
    setToastShortcut(window, viewport, viewport + '-provider', true, () => {
      native.push([viewport, captureFocus(window)])
      rememberFocus(window, viewport + '-focus')
    })
  }
  const dispatch = (window, id) => {
    const doc = windows.get(window)
    doc.active = elements.get(id) ?? { id }
    rememberFocus(window, id)
    const event = { key: 'F6', target: doc.active, preventDefault() {} }
    listeners
      .filter(item => item.window === window)
      .forEach(item => item.listener(event))
    enterWindowToasts(window)
    assert.deepEqual(native, expected)
    assert.equal(focusTarget(window), doc.active.id)
  }
  for (const id of ['toast-a-button', 'one-focus', 'two-focus'])
    dispatch('toast-a', id)
  dispatch('toast-b', 'toast-b-button')
  assert.equal(toastShortcutProvider('toast-a'), 'two-provider')
  for (const [window, viewport] of [
    ['toast-a', 'one'],
    ['toast-a', 'two'],
    ['toast-b', 'other'],
  ])
    setToastShortcut(window, viewport, '', false, () => {})
  assert.equal(toastShortcutProvider('toast-a'), '')
  forgetFocus('toast-a', focusTarget('toast-a'))
  forgetFocus('toast-b', focusTarget('toast-b'))
})

class ToastClock {
  time = 0
  sequence = 0
  jobs = new Map()
  now = () => this.time
  schedule = (action, delay) => {
    const id = ++this.sequence
    this.jobs.set(id, { action, due: this.time + delay })
    return id
  }
  cancel = id => {
    this.jobs.delete(id)
  }
  advance(ms) {
    const end = this.time + ms
    for (let count = 0; count < 10000; count++) {
      const next = [...this.jobs]
        .filter(([, job]) => job.due <= end)
        .sort((a, b) => a[1].due - b[1].due || a[0] - b[0])[0]
      if (!next) {
        this.time = end
        return
      }
      this.time = next[1].due
      this.jobs.delete(next[0])
      next[1].action()
    }
    throw new Error('Runaway toast timer')
  }
}

function toastReference(clock) {
  const file = path.resolve(
    root,
    '../../js/dazzui/node_modules/@base-ui/react/toast/store.js'
  )
  const nativeRequire = createRequire(fs.realpathSync(file))
  const source = fs.readFileSync(file, 'utf8')
  class Timeout {
    static create() {
      return new Timeout()
    }
    start(delay, action) {
      this.clear()
      this.id = clock.schedule(action, delay)
    }
    clear() {
      if (this.id !== undefined) clock.cancel(this.id)
      this.id = undefined
    }
  }
  const exports = {}
  const wrapper = vm.runInNewContext(
    `(function(exports, require) { ${source}\n})`,
    { Date: { now: clock.now } }
  )
  wrapper(exports, name => {
    if (name === '@base-ui/utils/useTimeout') return { Timeout }
    if (name === '@base-ui/utils/owner')
      return { ownerDocument: () => ({ activeElement: null }) }
    if (name === '../floating-ui-react/utils')
      return { activeElement: () => null, contains: () => false }
    return nativeRequire(name)
  })
  const store = new exports.ToastStore({
    timeout: 5000,
    limit: 3,
    viewport: null,
    toasts: [],
    hovering: false,
    focused: false,
    isWindowFocused: true,
    prevFocusElement: null,
  })
  const fileName = path.resolve(
    root,
    '../../js/dazzui/src/components/toast.tsx'
  )
  const parsed = ts.createSourceFile(
    fileName,
    fs.readFileSync(fileName, 'utf8'),
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TSX
  )
  const toBase = parsed.statements.find(
    node => ts.isFunctionDeclaration(node) && node.name?.text === 'toBase'
  )
  assert.ok(toBase, 'Use the actual DazzUI timeout/priority adapter')
  const constants = load('../../js/dazzui/src/generated/runtime.ts')
  const adapter = vm.runInNewContext(
    `(function() { ${ts.transpileModule(toBase.getText(parsed), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}; return toBase })()`,
    constants
  )
  return { store, adapter }
}

function toastPair() {
  const timings = {
    notice: new DazzTokens().metric('toast.notice-duration'),
    action: new DazzTokens().metric('toast.action-duration'),
    transition: new DazzTokens().metric('toast.transition.duration'),
  }
  const clock = new ToastClock()
  const referenceClock = new ToastClock()
  const queue = new DazzToastQueue(timings, 3, clock)
  const { store, adapter } = toastReference(referenceClock)
  const closed = []
  const referenceClosed = []
  const compare = () => {
    const actual = queue
      .snapshot()
      .filter(item => !item.ending)
      .map(item => [
        item.id,
        item.title,
        item.tint,
        item.timeout,
        item.priority,
        item.dismissable,
        item.dismissLabel,
        item.limited,
        item.action?.label,
      ])
    const expected = Array.from(store.state.toasts)
      .filter(item => item.transitionStatus !== 'ending')
      .map(item => [
        item.id,
        item.title,
        item.type,
        item.timeout,
        item.priority,
        item.data.dismissable,
        item.data.dismissLabel,
        item.limited,
        item.data.action?.label,
      ])
    assert.deepEqual(actual, expected)
    assert.deepEqual(closed, referenceClosed)
  }
  return {
    queue,
    clock,
    store,
    compare,
    closed,
    referenceClosed,
    add(options) {
      queue.add({
        ...options,
        action: options.action
          ? { label: options.action.label, onAction: options.action.onClick }
          : undefined,
        onClose: () => closed.push(options.id),
      })
      store.addToast(
        adapter({ ...options, onClose: () => referenceClosed.push(options.id) })
      )
      compare()
    },
    close(id) {
      queue.close(id)
      store.closeToast(id)
      compare()
    },
    advance(ms) {
      clock.advance(ms)
      referenceClock.advance(ms)
      compare()
    },
    pause(reason, paused) {
      queue.pause(reason, paused)
      const key = {
        hover: 'hovering',
        focus: 'focused',
        window: 'isWindowFocused',
      }[reason]
      store.update({ [key]: reason === 'window' ? !paused : paused })
      if (
        store.state.hovering ||
        store.state.focused ||
        !store.state.isWindowFocused
      )
        store.pauseTimers()
      else store.resumeTimers()
      compare()
    },
    dispose() {
      queue.dispose()
      store.disposeEffect()()
    },
  }
}

test('toast queue matches actual React defaults, ID replacement, ordering and limited flags', () => {
  const pair = toastPair()
  try {
    pair.add({ id: 'notice', title: 'Saved' })
    pair.add({ id: 'failure', title: 'Failed', tint: 'danger' })
    pair.add({
      id: 'undo',
      title: 'Archived',
      action: { label: 'Undo', onClick() {} },
    })
    pair.add({ id: 'latest', title: 'Newest', timeout: 100 })
    assert.equal(
      pair.queue.snapshot().find(item => item.id === 'notice').limited,
      true
    )
    pair.add({
      id: 'notice',
      title: 'Replaced in place',
      tint: 'info',
      timeout: 200,
    })
    pair.advance(100)
    pair.close('latest')
    pair.advance(100)
    pair.close('notice')
    pair.add({ id: 'notice', title: 'Reopened', timeout: 500 })
    pair.advance(260)
    assert.ok(pair.queue.snapshot().some(item => item.id === 'notice'))
    pair.queue.setLimit(1)
    pair.store.syncProviderProps(5000, 1)
    pair.compare()
    pair.close()
    pair.close()
    assert.equal(pair.closed.length, 5, 'Each lifetime closes once')
  } finally {
    pair.dispose()
  }
})

test('toast timers retain elapsed time across overlapping pause reasons and replacements', () => {
  const pair = toastPair()
  try {
    pair.add({ id: 'first', title: 'First', timeout: 1000 })
    pair.advance(200)
    pair.pause('hover', true)
    pair.advance(300)
    pair.pause('focus', true)
    pair.add({ id: 'second', title: 'Added while paused', timeout: 400 })
    pair.pause('hover', false)
    pair.advance(1000)
    pair.pause('window', true)
    pair.pause('focus', false)
    pair.advance(1000)
    pair.pause('window', false)
    pair.advance(399)
    pair.advance(1)
    pair.pause('hover', true)
    pair.add({ id: 'first', title: 'Reset while paused', timeout: 600 })
    pair.advance(1000)
    pair.pause('hover', false)
    pair.advance(599)
    pair.advance(1)
    pair.add({ id: 'danger', title: 'Persistent failure', tint: 'danger' })
    pair.advance(100000)
    assert.deepEqual(
      pair.queue
        .snapshot()
        .filter(item => !item.ending)
        .map(item => item.id),
      ['danger']
    )
  } finally {
    pair.dispose()
  }
})

test('toast action, snapshot isolation, exit lifetime and disposal do not leak callbacks', () => {
  const clock = new ToastClock()
  const queue = new DazzToastQueue(
    { notice: 5000, action: 8000, transition: 260 },
    3,
    clock
  )
  const events = []
  const unsubscribe = queue.subscribe(() => events.push('change'))
  queue.add({
    id: 'undo',
    title: 'Archived',
    action: { label: 'Undo', onAction: () => events.push('action') },
    onClose: () => events.push('close'),
  })
  const snapshot = queue.snapshot()
  snapshot[0].action.label = 'Mutated'
  snapshot[0].title = 'Mutated'
  assert.equal(queue.snapshot()[0].title, 'Archived')
  assert.equal(queue.snapshot()[0].action.label, 'Undo')
  queue.activate('undo')
  queue.activate('undo')
  queue.close('undo')
  assert.deepEqual(
    events.filter(event => event !== 'change'),
    ['action', 'close']
  )
  assert.equal(queue.snapshot()[0].ending, true)
  clock.advance(259)
  assert.equal(queue.snapshot().length, 1)
  clock.advance(1)
  assert.equal(queue.snapshot().length, 0)
  unsubscribe()
  const notifications = events.length
  queue.add({
    id: 'pending',
    title: 'Pending',
    onClose: () => events.push('close'),
  })
  queue.dispose()
  clock.advance(100000)
  assert.equal(events.length, notifications)
  assert.equal(clock.jobs.size, 0)
  assert.throws(() => queue.add({ title: 'After disposal' }), /disposed/)
})

test('portable toast actions use the consumed provider and keep callback state scoped', () => {
  const clock = new ToastClock()
  const context = new DazzToastContext()
  assert.throws(
    () => context.add({ title: 'Missing provider' }),
    /ToastProvider/
  )
  context.queue = new DazzToastQueue(
    { notice: 5000, action: 8000, transition: 260 },
    3,
    clock
  )
  const state = new SceneState({
    title: stringValue('Archived'),
    result: stringValue('pending'),
  })
  const sibling = new SceneState({
    title: stringValue('Other'),
    result: stringValue('pending'),
  })
  const setter = result => ({
    kind: 'actions',
    items: [
      {
        kind: 'object',
        fields: { set: stringValue('result'), to: stringValue(result) },
      },
    ],
  })
  const options = {
    kind: 'object',
    fields: {
      id: stringValue('receipt'),
      title: { kind: 'state', text: 'title' },
      action: {
        kind: 'object',
        fields: { label: stringValue('Undo'), onClick: setter('undone') },
      },
      onClose: setter('closed'),
    },
  }
  const raised = []
  const history = []
  const run = action => {
    state.run(action, EMPTY_VALUE, value => {
      raised.push(context.add(sceneToastOptions(value, state, run)))
    })
    history.push(text(state.evaluate({ kind: 'state', text: 'result' })))
  }
  run({
    kind: 'actions',
    items: [{ kind: 'object', fields: { toast: options } }],
  })
  assert.deepEqual(raised, ['receipt'])
  assert.equal(context.queue.snapshot()[0].title, 'Archived')
  assert.equal(context.queue.snapshot()[0].timeout, 8000)
  context.queue.activate('receipt')
  assert.deepEqual(history, ['pending', 'undone', 'closed'])
  assert.equal(
    text(state.evaluate({ kind: 'state', text: 'result' })),
    'closed'
  )
  assert.equal(
    text(sibling.evaluate({ kind: 'state', text: 'result' })),
    'pending'
  )
  assert.throws(
    () =>
      sibling.run({
        kind: 'actions',
        items: [{ kind: 'object', fields: { toast: options } }],
      }),
    /native provider/
  )
  context.queue.dispose()
})

const { fitTabs, tabsOrder, tabsNavigate, DazzTabsPulse } = load(
  'src/main/ets/foundation/TabsData.ets'
)

test('tab status pulses schedule forward halves and invalidate old completions on stop, restart and disposal', () => {
  const pulse = new DazzTabsPulse()
  const halves = [],
    completions = []
  let resets = 0
  pulse.animator = {
    reset: () => {
      resets++
    },
    half: (dimmed, finished) => {
      halves.push(dimmed)
      completions.push(finished)
    },
  }
  pulse.start(true)
  pulse.start(true)
  assert.deepEqual(
    halves,
    [true],
    'Repeated busy rendering does not create extra loops'
  )
  completions[0]()
  completions[1]()
  assert.deepEqual(
    halves,
    [true, false, true],
    'Both directions use a new forward interval'
  )
  pulse.start(false)
  completions[2]()
  assert.equal(halves.length, 3, 'Stopping prevents another half from starting')
  pulse.start(true)
  completions[2]()
  assert.deepEqual(
    halves,
    [true, false, true, true],
    'Old completions cannot affect a new busy lifetime'
  )
  pulse.dispose()
  completions[3]()
  pulse.start(true)
  assert.equal(halves.length, 4)
  assert.equal(resets, 3)
})

const calendar = load('src/main/ets/foundation/CalendarData.ets')
const dates = load('src/main/ets/foundation/DateData.ets')
const referenceDates = {}
vm.runInThisContext(
  `(function(exports) { ${
    ts.transpileModule(
      fs.readFileSync(
        path.resolve(root, '../../js/dazzui/src/lib/date.ts'),
        'utf8'
      ),
      {
        compilerOptions: {
          target: ts.ScriptTarget.ES2022,
          module: ts.ModuleKind.CommonJS,
        },
      }
    ).outputText
  } })`
)(referenceDates)
const calendarFile = path.resolve(
  root,
  '../../js/dazzui/src/components/calendar.tsx'
)
const calendarSource = ts.createSourceFile(
  calendarFile,
  fs.readFileSync(calendarFile, 'utf8'),
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TSX
)
function calendarFunction(name) {
  let found
  function visit(node) {
    if (
      ts.isVariableDeclaration(node) &&
      node.name.getText(calendarSource) === name
    )
      found = node.initializer.getText(calendarSource)
    if (ts.isFunctionDeclaration(node) && node.name?.text === name)
      found = node.getText(calendarSource)
    ts.forEachChild(node, visit)
  }
  visit(calendarSource)
  assert.ok(found, `Locate React Calendar ${name}`)
  return ts.transpileModule(
    found.startsWith('function') ? found : `const ${name} = ${found}`,
    { compilerOptions: { target: ts.ScriptTarget.ES2022 } }
  ).outputText
}
const plain = value => JSON.parse(JSON.stringify(value))

test('calendar date arithmetic matches React across month ends, leap years and every week start', () => {
  for (const year of [2019, 2020, 2024, 2026])
    for (let month = 0; month < 12; month++) {
      const day = new Date(year, month, 28, 14, 37)
      for (const weekStartsOn of [0, 1, 2, 3, 4, 5, 6]) {
        const grid = dates.monthGrid(day, weekStartsOn)
        assert.deepEqual(grid, referenceDates.monthGrid(day, weekStartsOn))
        assert.equal(grid.length, 42)
        assert.equal(grid[0].getDay(), weekStartsOn)
        assert.ok(grid.every(date => date.getHours() === 0))
      }
      for (const delta of [-366, -31, -7, -1, 0, 1, 7, 31, 366]) {
        assert.deepEqual(
          dates.addDays(day, delta),
          referenceDates.addDays(day, delta)
        )
        assert.deepEqual(
          dates.addMonths(day, delta),
          referenceDates.addMonths(day, delta)
        )
      }
      assert.deepEqual(dates.startOfDay(day), referenceDates.startOfDay(day))
      assert.deepEqual(
        dates.startOfMonth(day),
        referenceDates.startOfMonth(day)
      )
      const time = new Date(2026, 9, 16, 9, 12)
      assert.deepEqual(
        dates.withTime(day, time),
        referenceDates.withTime(day, time)
      )
      assert.deepEqual(
        dates.setTime(day, 26, -15),
        referenceDates.setTime(day, 26, -15)
      )
      assert.equal(dates.minutesOfDay(day), referenceDates.minutesOfDay(day))
    }
})

test('calendar retains one in-month tab stop after navigation for every week start and duplicate outside day', () => {
  const visible = calendarFunction('visible')
  const tabStop = calendarFunction('tabStop')
  for (let month = 0; month < 12; month++)
    for (const months of [1, 2])
      for (let weekStartsOn = 0; weekStartsOn < 7; weekStartsOn++) {
        const firstMonth = new Date(2026, month, 1)
        for (const focused of [
          new Date(2020, 0, 5),
          new Date(2026, month, 29),
          new Date(2026, month + 1, 1),
        ])
          for (const today of [
            new Date(2020, 0, 1),
            new Date(2026, month, 16),
          ]) {
            const context = {
              ...referenceDates,
              firstMonth,
              months,
              focused,
              today,
              result: undefined,
            }
            vm.runInNewContext(
              `${visible}; ${tabStop}; result = tabStop`,
              context
            )
            const actual = calendar.calendarTabStop(
              focused,
              today,
              firstMonth,
              months
            )
            assert.equal(actual.getTime(), context.result.getTime())
            const stops = []
            for (let index = 0; index < months; index++) {
              const current = dates.addMonths(firstMonth, index)
              for (const day of dates.monthGrid(current, weekStartsOn))
                if (dates.sameDay(day, actual) && dates.sameMonth(day, current))
                  stops.push(day)
            }
            assert.equal(
              stops.length,
              1,
              'Exactly one real month cell accepts Tab'
            )
          }
      }
})

test('calendar keyboard movement and month following execute the actual React handlers as references', () => {
  const keyboard = calendarFunction('onKeyDown')
  const daysIn = calendarFunction('daysIn')
  const moveFocus = calendarFunction('moveFocus')
  for (const day of [
    new Date(2024, 1, 29),
    new Date(2026, 0, 31),
    new Date(2026, 2, 8),
    new Date(2026, 10, 1),
    new Date(2026, 11, 31),
  ])
    for (const weekStartsOn of [0, 1, 6])
      for (const shift of [false, true])
        for (const key of [
          'ArrowLeft',
          'ArrowRight',
          'ArrowUp',
          'ArrowDown',
          'PageUp',
          'PageDown',
          'Home',
          'End',
          'Enter',
          ' ',
          'Tab',
          'Escape',
        ]) {
          const expected = { handled: false }
          vm.runInNewContext(`${daysIn}; ${keyboard}; onKeyDown(event)`, {
            ...referenceDates,
            Date,
            weekStartsOn,
            event: {
              target: { dataset: { day: String(day.getTime()) } },
              key,
              shiftKey: shift,
              preventDefault: () => {
                expected.handled = true
              },
            },
            moveFocus: day => {
              expected.focus = day
            },
            pick: day => {
              expected.pick = day
            },
          })
          assert.deepEqual(
            calendar.calendarKey(day, key, shift, weekStartsOn),
            expected
          )
        }
  for (const months of [1, 2])
    for (const offset of [-3, -1, 0, 1, 2, 3]) {
      const firstMonth = new Date(2026, 9, 1)
      const day = new Date(2026, 9 + offset, 16)
      let expected = firstMonth
      let focused
      const focusPending = { current: false }
      vm.runInNewContext(`${moveFocus}; moveFocus(day)`, {
        ...referenceDates,
        day,
        months,
        firstMonth,
        focusPending,
        visible: day =>
          Array.from({ length: months }, (_, i) =>
            referenceDates.addMonths(firstMonth, i)
          ).some(month => referenceDates.sameMonth(month, day)),
        setFocused: day => {
          focused = day
        },
        setMonth: day => {
          expected = referenceDates.startOfMonth(day)
        },
      })
      assert.deepEqual(
        calendar.calendarFocusMonth(day, firstMonth, months),
        expected
      )
      assert.deepEqual(focused, day)
      assert.equal(focusPending.current, true)
    }
})

test('calendar range picking and hover/focus preview match React, including reversed and bounded spans', () => {
  const pick = calendarFunction('pick')
  const dayState = calendarFunction('dayState')
  const ordered = calendarFunction('ordered')
  const rangeCases = [
    { start: null, end: null },
    { start: new Date(2026, 9, 20), end: null },
    { start: new Date(2026, 9, 20), end: new Date(2026, 9, 24) },
    { start: new Date(2026, 9, 24), end: new Date(2026, 9, 20) },
    { start: null, end: new Date(2026, 9, 20) },
  ]
  const focused = new Date(2026, 9, 22)
  for (const range of rangeCases)
    for (const mode of ['single', 'range'])
      for (const hovered of [
        null,
        new Date(2026, 9, 18),
        new Date(2026, 9, 24),
      ])
        for (const dayNumber of [9, 18, 20, 22, 24, 28]) {
          const day = new Date(2026, 9, dayNumber)
          const min = new Date(2026, 9, 12)
          const max = new Date(2026, 9, 25)
          for (const controlled of [false, true]) {
            let expected
            let changedInner = false
            vm.runInNewContext(`${pick}; pick(day)`, {
              ...referenceDates,
              range,
              day,
              disabled: day =>
                referenceDates.compareDay(day, min) < 0 ||
                referenceDates.compareDay(day, max) > 0,
              props: {
                mode,
                value: controlled ? range : undefined,
                onValueChange: value => {
                  expected = value
                },
              },
              setInnerRange: () => {
                changedInner = true
              },
              setInnerSingle: () => {
                changedInner = true
              },
            })
            const actual = calendar.calendarPick(mode, range, day, min, max)
            assert.equal(actual === undefined, expected === undefined)
            if (expected !== undefined)
              assert.deepEqual(plain(actual), plain(expected))
            assert.equal(changedInner, expected !== undefined && !controlled)
          }
          const pending = mode === 'range' && !!range.start && !range.end
          const preview = pending ? (hovered ?? focused) : null
          const [lo, hi] =
            mode !== 'range'
              ? [null, null]
              : vm.runInNewContext(`${ordered}; ordered(a, b)`, {
                  ...referenceDates,
                  a: range.start,
                  b: pending ? preview : range.end,
                })
          const expectedState = vm.runInNewContext(
            `${dayState}; dayState(day)`,
            { ...referenceDates, mode, range, pending, preview, lo, hi, day }
          )
          assert.deepEqual(
            calendar.calendarDayState(mode, range, day, focused, hovered),
            plain(expectedState)
          )
        }
})

test('generated Calendar callbacks preserve local dates and update the actual source captions in isolated scene state', () => {
  const story = load(
    'example/entry/src/main/ets/generated/Stories.ets'
  ).STORIES.find(story => story.id === 'calendar')
  assert.ok(story)
  const state = new SceneState(story.preview.initial)
  const sibling = new SceneState(story.preview.initial)
  const [control, caption] = story.preview.content.items
  const readCaption = () =>
    caption.items.map(item => text(state.evaluate(item))).join('')
  assert.equal(readCaption(), new Date(2026, 9, 20).toLocaleDateString())
  const next = new Date(2026, 10, 23, 14, 37, 42, 123)
  state.run(control.fields.onValueChange, dateValue(next))
  assert.deepEqual(dateFromValue(state.evaluate(control.fields.value)), next)
  assert.equal(readCaption(), next.toLocaleDateString())
  assert.deepEqual(
    dateFromValue(sibling.evaluate(control.fields.value)),
    new Date(2026, 9, 20)
  )
  state.run(control.fields.onValueChange, EMPTY_VALUE)
  assert.equal(
    readCaption(),
    'Nothing picked',
    'Null must choose the lazy fallback without formatting a null date'
  )

  const rangeScene = story.examples[1]
  const rangeState = new SceneState(rangeScene.initial)
  const [rangeControl, rangeCaption] = rangeScene.content.items
  const describe = () =>
    rangeCaption.items.map(item => text(rangeState.evaluate(item))).join('')
  rangeState.run(rangeControl.fields.onValueChange, {
    kind: 'object',
    fields: { start: dateValue(next), end: EMPTY_VALUE },
  })
  assert.equal(describe(), next.toLocaleDateString() + ' → …')
  rangeState.run(rangeControl.fields.onValueChange, {
    kind: 'object',
    fields: { start: EMPTY_VALUE, end: EMPTY_VALUE },
  })
  assert.equal(describe(), 'Pick a start')
  for (const [value, expected] of [
    [EMPTY_VALUE, false],
    [booleanValue(false), false],
    [stringValue(''), false],
    [numberValue(0), false],
    [numberValue(-1), true],
    [dateValue(next), true],
    [{ kind: 'object', fields: {} }, true],
    [{ kind: 'array', items: [] }, true],
  ])
    assert.equal(truth(value), expected)
})

test('native tab fit follows the actual React ruler solver, including active overflow and trailing width', () => {
  const file = path.resolve(root, '../../js/dazzui/src/components/tabs.tsx')
  const source = ts.createSourceFile(
    file,
    fs.readFileSync(file, 'utf8'),
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TSX
  )
  let solve
  function visit(node) {
    if (ts.isVariableDeclaration(node) && node.name.getText(source) === 'solve')
      solve = node.initializer.getText(source)
    ts.forEachChild(node, visit)
  }
  visit(source)
  assert.ok(solve, 'Locate the actual React layout solver')
  const js = ts.transpileModule(`const solve = ${solve}`, {
    compilerOptions: { target: ts.ScriptTarget.ES2022 },
  }).outputText
  for (const whole of [[], [90], [140, 90, 105, 110, 100], [31, 37, 45, 58]]) {
    const small = whole.map((width, index) => (index % 2 ? width : 34))
    for (let active = -1; active < whole.length; active++)
      for (const overflowWidth of [28, 44, 64])
        for (const trailing of [0, 28, 65])
          for (const width of [0, 80, 176, 200, 340, 520, 560, 1000]) {
            let expected
            const context = {
              element: { clientWidth: width },
              measure: {
                querySelector: () => ({ offsetWidth: overflowWidth }),
                querySelectorAll: selector =>
                  (selector.includes('whole') ? whole : small).map(
                    offsetWidth => ({ offsetWidth })
                  ),
              },
              tail: { current: { offsetWidth: trailing } },
              getComputedStyle: () => ({ marginInlineStart: '4px' }),
              items: whole.map((_, index) => ({ value: String(index) })),
              value: String(active),
              setFit: updater => {
                expected = updater({})
              },
            }
            vm.runInNewContext(`${js}; solve()`, context)
            assert.deepEqual(
              fitTabs(whole, small, active, width, trailing + 4, overflowWidth),
              JSON.parse(JSON.stringify(expected)),
              JSON.stringify({ whole, active, width, trailing })
            )
            const fit = fitTabs(
              whole,
              small,
              active,
              width,
              trailing,
              overflowWidth
            )
            const order = tabsOrder(whole.length, fit.shown, active)
            assert.deepEqual(
              [...order].sort((a, b) => a - b),
              whole.map((_, index) => index)
            )
            if (active >= 0)
              assert.ok(
                order.slice(0, fit.shown).includes(active),
                'Active tab remains visible'
              )
          }
  }
})

test('promoting a hidden active tab reserves its full width beside overflow and trailing controls', () => {
  const whole = [135, 91, 85, 113, 84]
  const small = [47, 47, 34, 55, 34]
  for (let active = -1; active < whole.length; active++)
    for (const width of [0, 80, 176, 200, 316, 520, 560])
      for (const trailing of [0, 28, 65]) {
        const fit = fitTabs(whole, small, active, width, trailing, 28)
        const order = tabsOrder(whole.length, fit.shown, active)
        const rendered =
          order
            .slice(0, fit.shown)
            .reduce(
              (sum, index) =>
                sum +
                (fit.compact && index !== active ? small[index] : whole[index]),
              0
            ) +
          trailing +
          (fit.shown < whole.length ? 28 : 0)
        assert.ok(
          rendered <= Math.max(width, fit.floor),
          JSON.stringify({ active, width, trailing, fit, rendered })
        )
      }
})

test('tab keyboard navigation matches the actual React handler for disabled, missing and removed selections', () => {
  const file = path.resolve(root, '../../js/dazzui/src/components/tabs.tsx')
  const source = ts.createSourceFile(
    file,
    fs.readFileSync(file, 'utf8'),
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TSX
  )
  let handler
  function visit(node) {
    if (
      ts.isVariableDeclaration(node) &&
      node.name.getText(source) === 'onKeyDown'
    )
      handler = node.initializer.getText(source)
    ts.forEachChild(node, visit)
  }
  visit(source)
  assert.ok(handler, 'Locate the actual React keyboard handler')
  const js = ts.transpileModule(`const onKeyDown = ${handler}`, {
    compilerOptions: { target: ts.ScriptTarget.ES2022 },
  }).outputText
  const items = [
    { value: 'a', label: 'A' },
    { value: 'b', label: 'B', disabled: true },
    { value: 'c', label: 'C' },
    { value: 'd', label: 'D' },
  ]
  for (let mask = 0; mask < 1 << items.length; mask++) {
    const shown = items.map((item, index) => ({
      ...item,
      disabled: !!(mask & (1 << index)),
    }))
    for (const value of [null, 'removed', ...items.map(item => item.value)])
      for (const key of [
        'ArrowLeft',
        'ArrowRight',
        'Home',
        'End',
        'ArrowDown',
        'Tab',
        'Escape',
      ]) {
        let expected
        let prevented = false
        let focused = false
        vm.runInNewContext(`${js}; onKeyDown(event)`, {
          shown,
          value,
          event: {
            key,
            preventDefault: () => {
              prevented = true
            },
          },
          onValueChange: value => {
            expected = value
          },
          root: {
            current: {
              querySelector: () => ({
                focus: () => {
                  focused = true
                },
              }),
            },
          },
          CSS: { escape: value => value },
        })
        assert.equal(tabsNavigate(shown, value, key), expected)
        assert.equal(prevented, expected !== undefined)
        assert.equal(focused, expected !== undefined)
      }
  }
  assert.equal(tabsNavigate(items, 'a', 'ArrowRight'), 'c')
  assert.equal(tabsNavigate(items, 'c', 'ArrowLeft'), 'a')
  assert.equal(tabsNavigate(items, 'd', 'ArrowRight'), 'a')
  assert.equal(tabsNavigate(items, 'a', 'ArrowLeft'), 'd')
  assert.equal(tabsNavigate(items, 'c', 'Home'), 'a')
  assert.equal(tabsNavigate(items, 'c', 'End'), 'd')
  assert.equal(tabsNavigate(items, null, 'ArrowRight'), 'a')
  assert.equal(tabsNavigate(items, null, 'ArrowLeft'), 'c')
  assert.equal(tabsNavigate(items, 'removed', 'ArrowRight'), 'a')
  assert.equal(tabsNavigate(items, 'a', 'ArrowDown'), undefined)
  assert.equal(
    tabsNavigate(
      items.map(item => ({ ...item, disabled: true })),
      'a',
      'End'
    ),
    undefined
  )
  assert.deepEqual(tabsOrder(5, 2, 4), [0, 4, 1, 2, 3])
  assert.deepEqual(tabsOrder(0, 0, -1), [])
})

const snapshots = JSON.parse(
  fs.readFileSync(
    new URL(
      '../../../../packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      import.meta.url
    )
  )
)

test('native runtime resolves all theme/tint/variant/state recipes to the shared bytes', () => {
  for (const name of THEME_NAMES) {
    const tokens = new DazzTokens(name)
    const source = snapshots[name]
    for (const tint of [
      'primary',
      'neutral',
      'info',
      'success',
      'warning',
      'danger',
    ])
      for (const variant of [
        'filled',
        'tinted',
        'outlined',
        'normal',
        'recessed',
        'plain',
      ])
        for (const part of ['surface', 'content', 'border'])
          for (const state of ['normal', 'hovered', 'pressed', 'disabled']) {
            const id = `control.color.${variant}.${part}.${state}`
            const shade = source[`${id}.shade`]
            const bytes =
              shade === undefined
                ? source[`${id}.color`]
                : source[`color.${tint}.${shade}`]
            const alpha = source[`${id}.alpha`] ?? 1
            const expected =
              '#' +
              [Math.round(bytes[0] * alpha), ...bytes.slice(1)]
                .map(n => n.toString(16).padStart(2, '0'))
                .join('')
                .toUpperCase()
            assert.equal(
              tokens.paint(variant, part, state, tint),
              expected,
              `${name}/${tint}/${id}`
            )
          }
  }
})

test('theme instances are isolated, unknown names and roles fail early', () => {
  const light = new DazzTokens()
  const dark = new DazzTokens('studio-dark')
  assert.notEqual(light.color('surface'), dark.color('surface'))
  assert.equal(light.metric('control.small.size'), 26)
  assert.equal(light.type('labelSmall').size, 11)
  const type = light.type('labelSmall')
  type.size = 999
  assert.equal(light.type('labelSmall').size, 11)
  assert.equal(new DazzTokens().type('labelSmall').size, 11)
  const shadows = light.shadow('shadow.lg')
  shadows[0].radius = 999
  assert.notEqual(light.shadow('shadow.lg')[0].radius, 999)
  assert.throws(() => new DazzTokens('missing'), /Unknown DazzUI theme/)
  assert.throws(() => light.color('missing'), /Unknown DazzUI color/)
  assert.throws(() => light.metric('missing'), /Unknown DazzUI metric/)
  assert.equal(withAlpha('#80010203', 0.5), '#40010203')
})

test('scene setters, arguments and comparisons use the real interpreter and do not leak across scenes', () => {
  const initial = { s0: stringValue('free') }
  const one = new SceneState(initial)
  const two = new SceneState(initial)
  const reference = { kind: 'state', text: 's0' }
  const eq = { kind: 'eq', items: [reference, stringValue('pro')] }
  const action = {
    kind: 'actions',
    items: [
      {
        kind: 'object',
        fields: {
          set: stringValue('s0'),
          to: { kind: 'argument', number: 0 },
        },
      },
    ],
  }
  assert.equal(one.evaluate(eq).boolean, false)
  one.run(action, stringValue('pro'))
  assert.equal(one.evaluate(eq).boolean, true)
  assert.equal(text(two.evaluate(reference)), 'free')
  assert.equal(text(initial.s0), 'free')
  assert.equal(text(booleanValue(false)), 'false')
  assert.equal(text(numberValue(0)), '0')
  assert.throws(
    () =>
      one.run({
        kind: 'actions',
        items: [{ kind: 'object', fields: { set: stringValue('unknown') } }],
      }),
    /Unknown scene state/
  )
})

test('controlled range arrays pass through scene actions without flattening their endpoints', () => {
  const initial = { kind: 'array', items: [numberValue(20), numberValue(80)] }
  const state = new SceneState({ s0: initial })
  const sibling = new SceneState({ s0: initial })
  const slot = { kind: 'state', text: 's0' }
  state.run(
    {
      kind: 'actions',
      items: [
        {
          kind: 'object',
          fields: {
            set: stringValue('s0'),
            to: { kind: 'argument', number: 0 },
          },
        },
      ],
    },
    { kind: 'array', items: [numberValue(30), numberValue(70)] }
  )
  assert.deepEqual(
    state.evaluate(slot).items.map(item => item.number),
    [30, 70]
  )
  assert.deepEqual(
    sibling.evaluate(slot).items.map(item => item.number),
    [20, 80]
  )
})

test('functional document removal preserves the selected value and sibling state like the React example', () => {
  const items = {
    kind: 'array',
    items: ['a', 'b', 'c'].map(value => ({
      kind: 'object',
      fields: {
        value: stringValue(value),
        label: stringValue(value.toUpperCase()),
      },
    })),
  }
  const initial = { items, selected: stringValue('a') }
  const state = new SceneState(initial)
  const sibling = new SceneState(initial)
  const action = {
    kind: 'actions',
    items: [
      {
        kind: 'object',
        fields: {
          remove: stringValue('items'),
          key: stringValue('value'),
          match: { kind: 'argument', number: 0 },
        },
      },
    ],
  }
  state.run(action, stringValue('a'))
  assert.deepEqual(
    state
      .evaluate({ kind: 'state', text: 'items' })
      .items.map(item => item.fields.value.text),
    ['b', 'c']
  )
  assert.equal(
    text(state.evaluate({ kind: 'state', text: 'selected' })),
    'a',
    'No invented adjacent-tab selection'
  )
  assert.equal(
    sibling.evaluate({ kind: 'state', text: 'items' }).items.length,
    3
  )
  state.run(action, stringValue('missing'))
  assert.equal(state.evaluate({ kind: 'state', text: 'items' }).items.length, 2)
  assert.throws(
    () =>
      state.run({
        kind: 'actions',
        items: [
          {
            kind: 'object',
            fields: {
              remove: stringValue('selected'),
              key: stringValue('value'),
            },
          },
        ],
      }),
    /requires array state/
  )
})

test('native package is standalone and declares the PC device type', () => {
  const module = JSON.parse(
    fs.readFileSync(path.join(root, 'src/main/module.json5'))
  )
  assert.equal(module.module.type, 'har')
  assert.deepEqual(module.module.deviceTypes, ['2in1'])
  assert.equal(
    JSON.parse(fs.readFileSync(path.join(root, 'oh-package.json5'))).main,
    'Index.ets'
  )
})

const { DazzSystemStyle } = load('src/main/ets/foundation/SystemStyleData.ets')
test('system sizing is independent of every palette and leaves design sizing stable', () => {
  const style = new DazzSystemStyle()
  style.fontScale = 2
  style.lengthScale = 2
  style.smallButtonFontSize = 28
  style.smallButtonPadding = 12
  style.buttonFontSize = 32
  style.buttonFontWeight = 500
  style.fieldFontSize = 32
  style.fieldFontWeight = 400
  style.buttonHeight = 56
  style.smallButtonHeight = 48
  style.buttonPadding = 16
  style.fieldHeight = 64
  style.fieldPadding = 20
  style.typography = { bodySmall: { size: 28, leading: 34, weight: 400 } }
  for (const name of THEME_NAMES) {
    const design = new DazzTokens(name)
    const adapted = new DazzTokens(name, style)
    assert.deepEqual(adapted.type('bodySmall'), style.typography.bodySmall)
    assert.equal(adapted.size('medium'), 56)
    assert.equal(adapted.size('small'), 48)
    for (const id of [
      'date-range-picker.side-width',
      'date-range-picker.trigger-min-width',
      'date-range-picker.time-min-width',
      'search-field.input-width',
    ])
      assert.equal(adapted.metric(id), design.metric(id) * 2)
    assert.equal(
      adapted.metric('date-range-picker.key-tracking'),
      design.metric('date-range-picker.key-tracking')
    )
    assert.equal(
      adapted.metric('calendar.medium.cell-size'),
      adapted.size('medium')
    )
    assert.equal(
      adapted.metric('calendar.small.cell-size'),
      adapted.size('small')
    )
    assert.equal(
      design.metric('calendar.medium.cell-size'),
      design.size('medium')
    )
    assert.equal(
      design.metric('calendar.small.cell-size'),
      design.size('small')
    )
    assert.equal(adapted.fieldSize('medium'), 64)
    assert.equal(adapted.fieldPadding(), 20)
    assert.equal(adapted.label('medium').size, 32)
    assert.equal(adapted.fontSize(adapted.label('medium').size), '32vp')
    assert.equal(adapted.metric('spacing.4'), design.metric('spacing.4') * 2)
    assert.equal(
      adapted.metric('checkbox.medium.box'),
      design.metric('checkbox.medium.box') * 2
    )
    assert.equal(
      adapted.metric('frame.sidebar.width'),
      design.metric('frame.sidebar.width') * 2
    )
    for (const id of [
      'radius.medium',
      'control.field.radius',
      'frame.window.radius',
      'checkbox.radius',
      'radio.thickness',
      'slider.thumb.edge-width',
      'motion.duration',
      'stroke.hairline',
    ]) {
      if (id !== 'motion.fast')
        assert.equal(adapted.metric(id), design.metric(id), id)
    }
    for (const role of ['surface', 'content', 'on-accent', 'border'])
      assert.equal(adapted.color(role), design.color(role))
    for (const tint of [
      'primary',
      'neutral',
      'info',
      'success',
      'warning',
      'danger',
    ])
      for (const variant of [
        'filled',
        'plain',
        'normal',
        'outlined',
        'tinted',
        'recessed',
      ])
        assert.equal(
          adapted.paint(variant, 'content', 'normal', tint),
          design.paint(variant, 'content', 'normal', tint)
        )
    assert.notEqual(design.label('medium').size, 32)
    assert.equal(
      design.fontSize(design.label('medium').size),
      design.label('medium').size + 'vp'
    )
  }
})

test('system snapshots cannot mutate an existing scope or sibling', () => {
  const style = new DazzSystemStyle()
  style.buttonFontSize = 16
  style.fieldFontSize = 16
  style.fieldHeight = 40
  style.typography = { bodySmall: { size: 14, leading: 20, weight: 400 } }
  const first = new DazzTokens('studio-light', style)
  const second = new DazzTokens('studio-dark', style)
  style.fieldHeight = 999
  style.typography.bodySmall.size = 999
  first.systemStyle.fieldHeight = 999
  first.systemStyle.typography.bodySmall.size = 999
  for (const scope of [first, second]) {
    assert.equal(scope.fieldSize('medium'), 40)
    assert.equal(scope.type('bodySmall').size, 14)
  }
})

const { toggleSelection, DazzAccordionContext } = load(
  'src/main/ets/foundation/Selection.ets'
)
test('disclosure and toggle selection preserve single/multiple semantics and scope isolation', () => {
  const original = ['bold']
  assert.deepEqual(toggleSelection(original, 'italic', false), ['italic'])
  assert.deepEqual(toggleSelection(original, 'italic', true), [
    'bold',
    'italic',
  ])
  assert.deepEqual(toggleSelection(original, 'bold', true), [])
  assert.deepEqual(original, ['bold'])
  const group = new DazzAccordionContext()
  const sibling = new DazzAccordionContext()
  for (const id of ['one', 'two', 'three', 'one']) group.register(id)
  assert.deepEqual(group.headers, ['one', 'two', 'three'])
  assert.equal(group.next('one', -1), 'three')
  assert.equal(group.next('three', 1), 'one')
  group.unregister('two')
  assert.equal(group.next('one', 1), 'three')
  assert.deepEqual(sibling.headers, [])
})

test('controlled multi-selection travels through the real scene action interpreter', () => {
  const state = new SceneState({
    s0: { kind: 'array', items: [stringValue('bold')] },
  })
  state.run(
    {
      kind: 'actions',
      items: [
        {
          kind: 'object',
          fields: {
            set: stringValue('s0'),
            to: { kind: 'argument', number: 0 },
          },
        },
      ],
    },
    { kind: 'array', items: [stringValue('italic'), stringValue('underline')] }
  )
  assert.deepEqual(
    state.evaluate({ kind: 'state', text: 's0' }).items.map(text),
    ['italic', 'underline']
  )
})

const { DazzFormContext, fieldError } = load(
  'src/main/ets/foundation/FormData.ets'
)
test('forms validate all editors, focus only the first failure, and clean up registrations', () => {
  const form = new DazzFormContext()
  form.active = true
  let submitted = 0
  let reset = 0
  let firstValid = false
  let secondValid = false
  const focused = []
  form.onSubmit = () => {
    submitted++
  }
  form.register('first', {
    validate: focus => {
      focused.push(['first', focus])
      return firstValid
    },
    reset: () => {
      reset++
    },
  })
  form.register('second', {
    validate: focus => {
      focused.push(['second', focus])
      return secondValid
    },
    reset: () => {
      reset++
    },
  })
  assert.equal(form.submit(), false)
  assert.equal(submitted, 0)
  assert.deepEqual(focused, [
    ['first', true],
    ['second', false],
  ])
  firstValid = true
  secondValid = true
  assert.equal(form.submit(), true)
  assert.equal(submitted, 1)
  form.unregister('second')
  form.reset()
  assert.equal(reset, 1)
  assert.equal(form.attempted, false)
  assert.equal(new DazzFormContext().submit(), false)
  assert.ok(fieldError('', true, false))
  assert.ok(fieldError('ada@', false, true))
  assert.equal(fieldError('', false, true), '')
  assert.equal(fieldError('ada@example.com', true, true), '')
})

test('query filtering preserves groups and skips disabled choices in keyboard order', () => {
  const { flattenOptions, filterOptions, optionLabel, nextOption } = load(
    'src/main/ets/foundation/Options.ets'
  )
  const source = [
    { value: 'none', label: 'Unassigned', disabled: true },
    {
      label: 'People',
      options: [
        { value: 'ada', label: 'Ada' },
        { value: 'grace', label: 'Grace' },
      ],
    },
    { label: 'Agents', options: [{ value: 'codex', label: 'Codex' }] },
  ]
  assert.equal(optionLabel(source, 'grace'), 'Grace')
  assert.equal(optionLabel(source, 'unknown'), '')
  assert.deepEqual(filterOptions(source, ' A '), [source[0], source[1]])
  assert.deepEqual(filterOptions(source, 'GRACE'), [
    { label: 'People', options: [source[1].options[1]] },
  ])
  assert.deepEqual(filterOptions(source, 'absent'), [])
  assert.equal(flattenOptions(source).length, 4)
  assert.equal(nextOption(source, -1, 1), 1)
  assert.equal(nextOption(source, -1, -1), 3)
  assert.equal(nextOption(source, 3, 1), 1)
  assert.equal(nextOption(source, 1, -1), 3)
  assert.equal(
    nextOption([{ value: 'x', label: 'X', disabled: true }], 0, 1),
    -1
  )
  assert.equal(nextOption([], -1, 1), -1)
  assert.equal(source[1].options.length, 2)
})

test('tab focus wraps, skips disabled choices and stays separate from selection', () => {
  const { nextTabFocus } = load('src/main/ets/foundation/Selection.ets')
  const items = [
    { value: 'all', label: 'All' },
    { value: 'disabled', label: 'Unavailable', disabled: true },
    { value: 'unread', label: 'Unread' },
  ]
  assert.equal(nextTabFocus(items, -1, 2015), 0)
  assert.equal(nextTabFocus(items, -1, 2014), 2)
  assert.equal(nextTabFocus(items, 0, 2015), 2)
  assert.equal(nextTabFocus(items, 2, 2015), 0)
  assert.equal(nextTabFocus(items, 0, 2014), 2)
  assert.equal(nextTabFocus(items, 2, 2081), 0)
  assert.equal(nextTabFocus(items, 0, 2082), 2)
  assert.equal(nextTabFocus(items, 0, 2054), -1)
  assert.equal(nextTabFocus(items, 0, 2049), -1)
  assert.equal(nextTabFocus([], 0, 2015), -1)
  assert.equal(nextTabFocus([{ ...items[0], disabled: true }], 0, 2015), -1)
  assert.equal(items[1].disabled, true)
})

test('physical native shortcuts match the React formatter across layouts and modifier combinations', () => {
  const {
    formatShortcut,
    shortcutModifiers,
    isShortcutModifier,
    heldShortcutModifiers,
  } = load('src/main/ets/foundation/Shortcut.ets')
  // Execute the actual React formatter too. Its component imports are unused
  // here; neither a DOM nor React rendering is involved in this comparison.
  const reactFile = path.resolve(
    root,
    '../../js/dazzui/src/components/shortcut-recorder.tsx'
  )
  const compiled = ts.transpileModule(fs.readFileSync(reactFile, 'utf8'), {
    compilerOptions: {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.CommonJS,
      jsx: ts.JsxEmit.ReactJSX,
    },
  })
  const react = {}
  vm.runInThisContext(
    `(function(exports, require) { ${compiled.outputText}\n})`,
    { filename: reactFile }
  )(react, () => ({}))
  const keys = [
    ...Array.from({ length: 26 }, (_, i) => [
      2017 + i,
      'Key' + String.fromCharCode(65 + i),
    ]),
    ...Array.from({ length: 10 }, (_, i) => [2000 + i, 'Digit' + i]),
    ...Array.from({ length: 10 }, (_, i) => [2103 + i, 'Numpad' + i]),
    ...Array.from({ length: 12 }, (_, i) => [2090 + i, 'F' + (i + 1)]),
    ...Array.from({ length: 8 }, (_, i) => [2816 + i, 'F' + (i + 13)]),
    [2050, 'Space'],
    [2054, 'Enter'],
    [2119, 'NumpadEnter'],
    [2049, 'Tab'],
    [2055, 'Backspace'],
    [2071, 'Delete'],
    [2012, 'ArrowUp'],
    [2013, 'ArrowDown'],
    [2014, 'ArrowLeft'],
    [2015, 'ArrowRight'],
    [2081, 'Home'],
    [2082, 'End'],
    [2068, 'PageUp'],
    [2069, 'PageDown'],
    [2057, 'Minus'],
    [2058, 'Equal'],
    [2059, 'BracketLeft'],
    [2060, 'BracketRight'],
    [2061, 'Backslash'],
    [2062, 'Semicolon'],
    [2063, 'Quote'],
    [2043, 'Comma'],
    [2044, 'Period'],
    [2064, 'Slash'],
    [2056, 'Backquote'],
    [2072, 'ControlLeft'],
    [2045, 'AltLeft'],
    [2074, 'CapsLock'],
    [2070, 'Escape'],
    [2824, 'F21'],
    [-1, 'Unidentified'],
  ]
  for (let mask = 0; mask < 16; mask++) {
    const modifiers = {
      ctrl: !!(mask & 1),
      alt: !!(mask & 2),
      shift: !!(mask & 4),
      meta: !!(mask & 8),
    }
    for (const [keyCode, code] of keys) {
      const actual = formatShortcut({ keyCode, ...modifiers })
      const expected = react.formatShortcut({
        code,
        key: '†',
        ctrlKey: modifiers.ctrl,
        altKey: modifiers.alt,
        shiftKey: modifiers.shift,
        metaKey: modifiers.meta,
      })
      assert.equal(actual, expected, code + '/' + mask)
    }
  }
  assert.equal(
    shortcutModifiers({
      keyCode: 0,
      ctrl: true,
      alt: true,
      shift: true,
      meta: true,
    }),
    '⌃⌥⇧⌘'
  )
  for (const code of [2045, 2046, 2047, 2048, 2072, 2073, 2076, 2077])
    assert.ok(isShortcutModifier(code))
  assert.equal(isShortcutModifier(2036), false)
  const sparse = {
    keyCode: 2047,
    ctrl: false,
    alt: false,
    shift: true,
    meta: false,
  }
  assert.equal(heldShortcutModifiers(sparse, [2045, 2047], false), '⌥⇧')
  assert.equal(heldShortcutModifiers(sparse, [2045], true), '⌥')
  assert.equal(
    heldShortcutModifiers(sparse, [2048], true),
    '⇧',
    'The other Shift key remains held'
  )
  assert.equal(
    heldShortcutModifiers(sparse, [], true),
    '',
    'A released key overrides a stale modifier flag'
  )
})
