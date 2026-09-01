#!/usr/bin/env node

import { parseArgs } from 'node:util'

import { add } from '../src/commands/add.mjs'
import { init } from '../src/commands/init.mjs'
import { list } from '../src/commands/list.mjs'
import { remove } from '../src/commands/remove.mjs'
import { status } from '../src/commands/status.mjs'
import { update } from '../src/commands/update.mjs'

const HELP = `dzui — copy DazzUI component source into your project

Usage:
  dzui init [--force]            set up dzui.json and vendor the foundation
  dzui add <name...> [--force]   copy components in (dependencies included)
  dzui list                      registry inventory, with install state
  dzui status                    what is modified locally / changed upstream
  dzui update [name...] [--dry-run]
                                 pull upstream changes; merges local edits
  dzui remove <name...> [--force]

Copied files are yours to edit — except the foundation stylesheets, which
are vendored and overwritten on update. dzui.lock.json and .dzui/ record
what was installed so \`update\` can three-way merge; commit them.`

async function main() {
  const { values, positionals } = parseArgs({
    args: process.argv.slice(2),
    allowPositionals: true,
    options: {
      force: { type: 'boolean', default: false },
      'dry-run': { type: 'boolean', default: false },
      help: { type: 'boolean', short: 'h', default: false },
    },
  })

  const [command, ...names] = positionals
  const root = process.cwd()

  if (values.help || !command) {
    console.log(HELP)
    return
  }

  switch (command) {
    case 'init':
      return init(root, { force: values.force })
    case 'add':
      return add(root, names, { force: values.force })
    case 'list':
      return list(root)
    case 'status':
      return status(root)
    case 'update':
      return update(root, names, { dryRun: values['dry-run'] })
    case 'remove':
      return remove(root, names, { force: values.force })
    default:
      throw new Error(`unknown command '${command}' — see dzui --help`)
  }
}

// `dzui list | head` closes stdout early; that is not an error worth a trace
process.stdout.on('error', error => {
  if (error.code === 'EPIPE') process.exit(0)
  throw error
})

try {
  await main()
} catch (error) {
  console.error(`dzui: ${error.message}`)
  process.exitCode = 1
}
