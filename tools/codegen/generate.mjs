import fs from 'node:fs/promises'
import { createHash } from 'node:crypto'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import {
  glyphsDart,
  storiesIndexDart,
  storyDart,
  storyFile,
  themesDart,
  unsupported,
} from './flutter.mjs'
import { formatOutputs } from './format.mjs'
import * as gpui from './gpui.mjs'
import * as nsh from './nsh.mjs'
import * as gtk4 from './gtk4.mjs'
import * as harmony from './harmony.mjs'
import { readHarmonyReactStories } from './harmony-react.mjs'
import * as swiftui from './swiftui.mjs'
import * as winui from './winui.mjs'
import { readStories, readThemes } from './stories.mjs'
import { scenesJson } from './scenes.mjs'

const root = fileURLToPath(new URL('../../', import.meta.url))
const check = process.argv.includes('--check')

const stories = await readStories(root)
const [flutterStories, swiftStories, winuiStories] = await Promise.all(
  ['flutter', 'swiftui', 'winui'].map(platform =>
    readStories(root, { platform })
  )
)
const gaps = [
  ...nsh
    .unsupported(stories)
    .map(
      gap =>
        `${gap} — map it in packages/rust/dazzui-nsh/example/src/lib/scene/ and list it in tools/codegen/nsh.mjs`
    ),
  ...unsupported(flutterStories).map(
    gap =>
      `${gap} — map it in packages/dart/dazzui/example/lib/lib/mapping.dart and list it in tools/codegen/flutter.mjs`
  ),
  ...swiftui
    .unsupported(swiftStories)
    .map(
      gap =>
        `${gap} — map it in packages/swift/DazzUI/Example/Sources/Example/Lib/Mapping.swift and list it in tools/codegen/swiftui.mjs`
    ),
  ...winui
    .unsupported(winuiStories)
    .map(
      gap =>
        `${gap} — map it in packages/csharp/DazzUI/example/Lib/Mapping.cs and list it in tools/codegen/winui.mjs`
    ),
  ...gpui
    .unsupported(stories)
    .map(
      gap =>
        `${gap} — map it in packages/rust/dazzui/example/src/lib/mapping/ and list it in tools/codegen/gpui.mjs`
    ),
]
if (gaps.length)
  throw new Error(
    `A platform cannot draw what these stories ask for:\n${gaps.join('\n')}`
  )
/*
 * The conformance contract: every scene and the addresses in it. First,
 * because it writes each specimen's address into the scenes the platforms
 * are emitted from.
 */
const scenes = scenesJson(stories)
// Native extensions keep stable specimen addresses without entering the
// shared conformance contract or claiming mappings on other platforms.
for (const platformStories of [flutterStories, swiftStories, winuiStories])
  scenesJson(platformStories)
const themes = await readThemes(root)
const variables = await fs.readFile(
  path.join(
    root,
    'packages/dart/dazzui/lib/src/generated/theme_variables.dart'
  ),
  'utf8'
)

/*
 * The Flutter example is laid out as the React one is: `stories/` a file
 * per component, `lib/` the story type and the themes.
 */
const flutter = 'packages/dart/dazzui/example/lib'
const outputs = new Map([
  ...flutterStories.map(story => [
    `${flutter}/stories/${storyFile(story.id)}`,
    storyDart(story),
  ]),
  [`${flutter}/stories/index.dart`, storiesIndexDart(flutterStories)],
  [`${flutter}/lib/glyphs.dart`, glyphsDart(flutterStories)],
  [`${flutter}/lib/themes.dart`, themesDart(themes, variables)],
])

/* The SwiftUI example, laid out the same way. */
const swift = 'packages/swift/DazzUI/Example/Sources/Example'
const themeNames = await fs.readFile(
  path.join(
    root,
    'packages/swift/DazzUI/Sources/DazzUI/Generated/ThemeName.swift'
  ),
  'utf8'
)
for (const story of swiftStories)
  outputs.set(
    `${swift}/Stories/${swiftui.storyFile(story.id)}`,
    swiftui.storySwift(story)
  )
outputs.set(
  `${swift}/Stories/Index.swift`,
  swiftui.storiesIndexSwift(swiftStories)
)
outputs.set(`${swift}/Lib/Glyphs.swift`, swiftui.glyphsSwift(swiftStories))
outputs.set(
  `${swift}/Lib/Themes.swift`,
  swiftui.themesSwift(themes, themeNames)
)

/* The WinUI 3 example, laid out the same way. */
const csharp = 'packages/csharp/DazzUI/example'
const tokens = Object.keys(
  JSON.parse(
    await fs.readFile(
      path.join(
        root,
        'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'
      ),
      'utf8'
    )
  )
)
for (const story of winuiStories)
  outputs.set(
    `${csharp}/Stories/${winui.storyFile(story.id)}`,
    winui.storyCs(story)
  )
outputs.set(`${csharp}/Stories/Index.cs`, winui.storiesIndexCs(winuiStories))
outputs.set(`${csharp}/Lib/Glyphs.cs`, winui.glyphsCs(winuiStories))
outputs.set(`${csharp}/Lib/Themes.cs`, winui.themesCs(themes, tokens))

/*
 * The GPUI example, laid out the same way, and the one file codegen writes
 * into the kit itself: the glyphs its components import.
 */
const rust = 'packages/rust/dazzui/example/src'
for (const story of stories)
  outputs.set(
    `${rust}/stories/${gpui.storyFile(story.id)}`,
    gpui.storyRs(story)
  )
outputs.set(`${rust}/stories/mod.rs`, gpui.storiesIndexRs(stories))
outputs.set(`${rust}/lib/glyphs.rs`, gpui.glyphsRs(root, stories))
outputs.set(
  `${rust}/lib/themes.rs`,
  gpui.themesRs(
    themes,
    await fs.readFile(
      path.join(root, 'packages/rust/dazzui/src/generated/theme_tokens.rs'),
      'utf8'
    )
  )
)
outputs.set('packages/rust/dazzui/src/glyphs.rs', gpui.kitGlyphsRs(root))

outputs.set('tools/conformance/scenes.json', scenes)
const gtk = 'packages/rust/dazzui-gtk4/example/src'
for (const story of stories)
  outputs.set(
    `${gtk}/stories/${gtk4.storyFile(story.id)}`,
    gtk4.storyRust(story)
  )
outputs.set(`${gtk}/stories/mod.rs`, gtk4.storiesIndexRust(stories))
outputs.set(`${gtk}/lib/themes.rs`, gtk4.themesRust(themes))
const gtkIcons = 'packages/rust/dazzui-gtk4/src/generated/icons'
const glyphs = gpui.glyphSvgs(root, stories)
for (const name of gpui.kitGlyphNames(root))
  glyphs.set(name, gpui.glyphSvg(root, name))
const ark = 'packages/arkts/dazzui'
const arkExample = `${ark}/example/entry/src/main/ets/generated`
const reactOnly = await readHarmonyReactStories(root)
// Give native-only scenes stable addresses without adding other platforms to
// their support gate or changing the shared conformance contract.
outputs.set(`${ark}/example/react-scenes.json`, scenesJson(reactOnly.stories))
/*
 * The native reader compiles a page that is portable too — Calendar and
 * DateRangePicker — because its wire format carries dates as integer parts,
 * not the portable data's local civil time. The portable list drops those
 * pages so the gallery carries each one once, and coverage counts it once.
 */
const nativePages = new Set(reactOnly.stories.map(story => story.id))
const portableStories = stories.filter(story => !nativePages.has(story.id))
const harmonyStories = [...portableStories, ...reactOnly.stories].sort((a, b) =>
  a.name.localeCompare(b.name)
)
outputs.set(`${arkExample}/Stories.ets`, harmony.storiesETS(harmonyStories))
outputs.set(`${arkExample}/Themes.ets`, harmony.themesETS(themes))
outputs.set(
  `${ark}/example/coverage.json`,
  harmony.coverageJSON(portableStories)
)
outputs.set(
  `${ark}/example/react-coverage.json`,
  JSON.stringify(
    {
      pages: reactOnly.pages,
      scenes: JSON.parse(harmony.coverageJSON(reactOnly.stories)),
    },
    null,
    2
  ) + '\n'
)
const harmonyGlyphs = new Map([
  ...glyphs,
  ...gpui.glyphSvgs(root, reactOnly.stories),
])
for (const [name, svg] of harmonyGlyphs)
  outputs.set(
    `${ark}/src/main/resources/rawfile/dazzui/icons/${name}.svg`,
    svg.replaceAll('currentColor', '#000000') + '\n'
  )
const iconFiles = [...glyphs]
  .sort(([a], [b]) => a.localeCompare(b))
  .map(([name, svg]) => {
    const file = `scalable/actions/dazzui-${name}-symbolic.svg`
    outputs.set(`${gtkIcons}/${file}`, svg + '\n')
    return file
  })
outputs.set(
  `${gtkIcons}/icons.gresource.xml`,
  `<?xml version="1.0" encoding="UTF-8"?>
<!-- Generated by tools/codegen from the React glyphs. Do not edit. -->
<gresources><gresource prefix="/dev/dazzlabs/dazzui/icons">
${iconFiles.map(file => `  <file>${file}</file>`).join('\n')}
</gresource></gresources>
`
)
for (const [file, content] of nsh.outputs(root, stories, themes))
  outputs.set(file, content)
formatOutputs(outputs)

/*
 * `outputs.json` records what this owns and the hash it last wrote. A file it
 * owned and no longer writes is deleted — but only while it still matches
 * that hash, so a hand edit is never thrown away.
 */
const ownershipPath = 'tools/codegen/outputs.json'
const previous = JSON.parse(
  await fs.readFile(path.join(root, ownershipPath), 'utf8').catch(error => {
    if (error.code !== 'ENOENT') throw error
    return '{}'
  })
)
const hash = content => createHash('sha256').update(content).digest('hex')
const stale = []
for (const [relative, expectedHash] of Object.entries(previous)) {
  if (outputs.has(relative)) continue
  if (
    !relative.startsWith('packages/') &&
    !relative.startsWith('tools/codegen/')
  )
    throw new Error(`Invalid owned output: ${relative}`)
  const file = path.resolve(root, relative)
  if (!file.startsWith(root))
    throw new Error(`Output escapes the repository: ${relative}`)
  const content = await fs.readFile(file, 'utf8').catch(error => {
    if (error.code !== 'ENOENT') throw error
    return null
  })
  if (content === null) continue
  if (hash(content) !== expectedHash)
    throw new Error(`Refusing to delete modified generated file: ${relative}`)
  if (check) stale.push(relative)
  else await fs.unlink(file)
}
outputs.set(
  ownershipPath,
  JSON.stringify(
    Object.fromEntries(
      [...outputs].map(([file, content]) => [file, hash(content)])
    ),
    null,
    2
  ) + '\n'
)
for (const [relative, content] of outputs) {
  const file = path.join(root, relative)
  const current = await fs.readFile(file, 'utf8').catch(error => {
    if (error.code !== 'ENOENT') throw error
    return null
  })
  if (current === content) continue
  if (check) stale.push(relative)
  else {
    await fs.mkdir(path.dirname(file), { recursive: true })
    await fs.writeFile(file, content)
  }
}
if (stale.length)
  throw new Error(`Run pnpm codegen; stale outputs:\n${stale.join('\n')}`)
console.log(
  `${stories.length} stories, ${stories.reduce((n, s) => n + s.examples.length, 0)} examples; ${outputs.size} generated files checked`
)
console.log(
  `HarmonyOS also reads ${reactOnly.stories.length} native page(s); ${reactOnly.pages.filter(page => !page.mapped).length} remain unmapped`
)
