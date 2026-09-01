import fs from 'node:fs/promises'
import path from 'node:path'
import { execSync } from 'node:child_process'

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
]

async function distribute() {
  for (const target of TARGETS) {
    const targetDir = path.join(process.cwd(), target.dir)
    await fs.mkdir(targetDir, { recursive: true })

    for (const file of target.files) {
      const source = path.join(process.cwd(), 'dist', file)
      const dest = path.join(targetDir, path.basename(file))

      // The CSS plugin's `exclude` is honoured in the `:root` block but not
      // in the theme blocks, where anything aliasing a themed token is
      // re-emitted — so the excluded `control.color` leaves leak back in
      // there. Nothing reads them (the control plugin resolves recipes to
      // the roles directly), so they are dropped on the way through.
      if (file === 'style.css') {
        const css = await fs.readFile(source, 'utf8')
        await fs.writeFile(
          dest,
          css.replace(/^\s*--control-color-[a-z0-9-]+:.*\n/gm, '')
        )
        continue
      }

      await fs.copyFile(source, dest)
    }
    console.log(
      `→ ${target.files.length} files to ${target.package} (${target.dir})`
    )
  }
}

async function main() {
  execSync('tz build', { stdio: 'inherit' })
  await distribute()
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})
