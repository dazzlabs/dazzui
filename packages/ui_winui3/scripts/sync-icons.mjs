import fs from 'node:fs/promises'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

// Copy only the Fluent System Icons the native kit uses, from the same package
// as React. These are vendored assets, so the .NET library has no Node dependency.
const root = fileURLToPath(new URL('../', import.meta.url))
const source = path.join(root, '../ui_react/node_modules/@fluentui/react-icons')
const names = [
  'Add',
  'Search',
  'Dismiss',
  'Settings',
  'Folder',
  'Document',
  'Checkmark',
  'ChevronRight',
  'ChevronDown',
  'ArrowDownload',
  'MoreHorizontal',
  'Info',
  'Warning',
  'ErrorCircle',
  'Person',
  'ArrowClockwise',
  'Copy',
  'Delete',
  'ArrowLeft',
  'ArrowRight',
  'Eye',
  'EyeOff',
  'TextBold',
  'TextItalic',
  'Star',
]
const paths = {}
const directory = path.join(source, 'lib/sizedIcons')
for (const file of await fs.readdir(directory)) {
  if (!file.endsWith('.js')) continue
  const text = await fs.readFile(path.join(directory, file), 'utf8')
  for (const name of names) {
    const match = new RegExp(
      `createFluentIcon\\('${name}16Regular', "16", (\\[.*?\\])\\)`
    ).exec(text)
    if (match) paths[name] = JSON.parse(match[1]).join(' ')
  }
}
for (const name of names)
  if (!paths[name]) throw new Error(`Missing Fluent icon: ${name}`)
const output = path.join(root, 'src/DazzUI/Assets')
await fs.mkdir(output, { recursive: true })
await fs.writeFile(
  path.join(output, 'icon-paths.json'),
  `${JSON.stringify(paths, null, 2)}\n`
)
// FluentIcons.LICENSE is checked in beside the paths and retained on rebuild.
console.log(`Vendored ${names.length} Fluent System Icons`)
