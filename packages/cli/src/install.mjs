import { posix, relative, sep } from 'node:path'

import { sha256, writePristine, writeProjectFile } from './context.mjs'
import { readRegistryFile, requireItem } from './registry.mjs'

/*
 * `index.css` is the one file the CLI authors rather than copies: the single
 * import a consumer's entry point names, so adding foundation files upstream
 * never touches user code. It is tracked and updated like the copied three.
 */
const FOUNDATION_INDEX = `/*
 * DazzUI foundation — vendored by dzui, updated by \`dzui update\`.
 * Import this file once from your app entry; do not edit these copies, they
 * are overwritten wholesale on update (theming is done with \`data-theme\`
 * on the root element, not by editing the token sheet).
 */

@import './style.css';
@import './control.css';
@import './base.css';
`

/*
 * Where component code finds the lib once both are copied out. An alias wins
 * when the project has one configured; otherwise a relative path, which needs
 * no tsconfig cooperation.
 */
function libImportPrefix(config) {
  if (config.aliases.lib) return config.aliases.lib
  const rel = relative(config.paths.components, config.paths.lib)
    .split(sep)
    .join('/')
  return rel.startsWith('.') ? rel : `./${rel}`
}

function rewriteComponentSource(name, source, config) {
  const prefix = libImportPrefix(config)
  const rewritten = source.replace(/from '\.\.\/lib\//g, `from '${prefix}/`)
  // The package bundles component CSS through a registry file; the copies
  // carry their own sheet instead, so `add`/`remove` never edit user code.
  return `import './${name}.css'\n\n${rewritten}`
}

function targetDir(config, item) {
  if (item.type === 'foundation') return config.paths.styles
  if (item.type === 'lib') return config.paths.lib
  return config.paths.components
}

/*
 * The plan for one item: every file it lands in the project, with the
 * content exactly as it would be written (rewrites applied). `add`, `status`
 * and `update` all reason over this — the lockfile hashes are hashes of
 * planned content, so "modified" and "update available" are both one
 * comparison.
 */
export async function planItem(registry, config, name) {
  const item = requireItem(registry, name)
  const dir = targetDir(config, item)
  const files = []
  for (const file of item.files) {
    let content = await readRegistryFile(name, file.name)
    if (item.type === 'component' && file.name.endsWith('.tsx')) {
      content = rewriteComponentSource(name, content, config)
    }
    files.push({
      name: file.name,
      path: posix.join(dir.split(sep).join('/'), file.name),
      content,
    })
  }
  if (item.type === 'foundation') {
    files.push({
      name: 'index.css',
      path: posix.join(dir.split(sep).join('/'), 'index.css'),
      content: FOUNDATION_INDEX,
    })
  }
  return { name, type: item.type, files }
}

/*
 * Write a plan into the project: the working copy, the pristine snapshot the
 * next merge will use as its base, and the lockfile entry. The lock hash is
 * of the planned (rewritten) content, so a byte-identical working file reads
 * as unmodified.
 */
export async function applyPlan(ctx, plan) {
  const files = {}
  for (const file of plan.files) {
    await writeProjectFile(ctx, file.path, file.content)
    await writePristine(ctx, plan.name, file.name, file.content)
    files[file.path] = sha256(file.content)
  }
  ctx.lock.items[plan.name] = { files }
}
