import fs from 'node:fs/promises'
import os from 'node:os'
import path from 'node:path'
import { execSync } from 'node:child_process'
import { loadTokens, writeFiles } from '@terrazzo/cli'
import { Logger, build, parse } from '@terrazzo/parser'
import YAML from 'yaml'
import yamlToMomoa from 'yaml-to-momoa'
import config from '../terrazzo.config.mjs'
import { expandRecipes } from './recipes.mjs'

/**
 * Where each platform package's generated sources land.
 *
 * This package is a generator, not a dependency: a platform package holds the
 * output rather than importing it, so it builds and ships without the design
 * system being installed alongside it. `dist/` is only the staging area the
 * plugins write to on the way here.
 *
 * The distributed files are committed: what a token change does to a
 * platform is then visible in the diff rather than only after a rebuild.
 */
const TARGETS = [
  {
    package: 'DazzUI (WinUI 3)',
    dir: path.join('..', 'ui_winui3', 'src', 'DazzUI', 'Generated'),
    files: ['winui3/tokens.json'],
  },
  {
    package: 'mobile_ui_android',
    dir: path.join(
      '..',
      'mobile_ui_android',
      'src',
      'main',
      'kotlin',
      'dev',
      'dazzlabs',
      'dazzui',
      'mobile',
      'generated'
    ),
    files: ['compose/DazzColorSchemes.kt'],
  },
  {
    package: '@dazzlabs/dazzui-react',
    dir: path.join('..', 'ui_react', 'src', 'generated'),
    files: ['style.css', 'control.css'],
  },
  {
    package: 'dazzui',
    dir: path.join('..', 'ui_flutter', 'lib', 'src', 'generated'),
    files: ['flutter/colors.dart', 'flutter/theme_variables.dart'],
  },
  {
    package: 'DazzUI',
    dir: path.join('..', 'ui_swiftui', 'Sources', 'DazzUI', 'Generated'),
    files: ['swiftui/Colors.swift', 'swiftui/ThemeVariables.swift'],
  },
  {
    package: 'DazzMobileUI',
    dir: path.join(
      '..',
      'mobile_ui_ios',
      'Sources',
      'DazzMobileUI',
      'Generated'
    ),
    files: [
      'mobile-swiftui/Colors.swift',
      'mobile-swiftui/Palette.swift',
      'mobile-swiftui/ThemeFamily.swift',
    ],
  },
  {
    package: 'dazzui (gpui)',
    dir: path.join('..', 'ui_gpui', 'src', 'generated'),
    files: ['gpui/colors.rs', 'gpui/theme_variables.rs'],
  },
]

/**
 * What `tz build` does, done here so one step can come first.
 *
 * The token files are read exactly as the CLI reads them and handed to the
 * same `parse` and `build` it calls. The step in between is the recipe
 * table: `control.color` writes only its decisions, and `expandRecipes`
 * fills in the derived cells before the graph exists, so every plugin reads
 * the full table and none of them carries a rule of its own. Nothing is
 * written back — the expanded document lives only in this process.
 *
 * A file that is expanded is parsed from the expanded object rather than
 * from its text, so a validation error in it is reported against Terrazzo's
 * own print of that object: the same keys in the same order, just without
 * the comments. Every other file keeps its YAML, and its line numbers.
 */
async function generate() {
  const logger = new Logger()

  const sources = await loadTokens(config.tokens, { logger })
  if (!sources) {
    throw new Error('Could not load the token files terrazzo.config.mjs lists.')
  }
  for (const source of sources) {
    const document = YAML.parse(source.src)
    const expanded = expandRecipes(document)
    if (expanded !== document) source.src = expanded
  }

  const parsed = await parse(sources, { config, logger, yamlToMomoa })
  const result = await build(parsed.tokens, {
    sources: parsed.sources,
    config,
    logger,
  })
  writeFiles(result, { config, logger })
  console.log(`✔  ${Object.keys(parsed.tokens).length} tokens built`)
}

/**
 * The declarations the CSS plugin re-emits despite being excluded.
 *
 * `exclude` is honoured in the `:root` block but not in the theme blocks,
 * where anything aliasing a themed token is written out again — so every
 * excluded recipe leaf leaks back in there. Nothing reads them: the control
 * plugin resolves each one into a finished value beside them, and a raw
 * shade or alpha in the contract is a lever that does nothing.
 */
const LEAKED =
  /^\s*--(?:control-color-[a-z0-9-]+|focus-(?:ring|glow)-(?:shade|alpha)|control-pressed-alpha|dialog-scrim-alpha):.*\n/gm

/** Strip them, and rewrite the staged copy so the gallery reads what ships. */
async function stripLeaks(source) {
  const css = await fs.readFile(source, 'utf8')
  const cleaned = css.replace(LEAKED, '')
  if (cleaned !== css) await fs.writeFile(source, cleaned)
  return cleaned
}

async function distribute() {
  for (const target of TARGETS) {
    const targetDir = path.join(process.cwd(), target.dir)
    await fs.mkdir(targetDir, { recursive: true })

    for (const file of target.files) {
      const source = path.join(process.cwd(), 'dist', file)
      const dest = path.join(targetDir, path.basename(file))

      if (file === 'style.css') {
        await fs.writeFile(dest, await stripLeaks(source))
        continue
      }

      await fs.copyFile(source, dest)
    }
    console.log(
      `→ ${target.files.length} files to ${target.package} (${target.dir})`
    )
  }
}

/**
 * The Dart files are committed formatted, so the generator has to leave them
 * the way `dart format` would — otherwise every rebuild dirties the Flutter
 * package with a formatting-only diff. The formatter is looked for on PATH
 * and then in fvm's default install; a machine with neither still builds,
 * it just leaves the formatting to whoever runs Flutter next.
 */
function formatDart(files) {
  const candidates = [
    'dart',
    path.join(os.homedir(), 'fvm/versions/stable/bin/dart'),
  ]
  for (const dart of candidates) {
    try {
      execSync(`${dart} format ${files.map(f => `"${f}"`).join(' ')}`, {
        stdio: 'pipe',
      })
      return
    } catch {
      // Not this candidate — try the next.
    }
  }
  console.warn('dart not found; the generated Dart files are unformatted.')
}

async function main() {
  await generate()
  await distribute()

  const dartFiles = TARGETS.flatMap(target =>
    target.files
      .filter(file => file.endsWith('.dart'))
      .map(file => path.join(process.cwd(), target.dir, path.basename(file)))
  )
  if (dartFiles.length > 0) formatDart(dartFiles)
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})
