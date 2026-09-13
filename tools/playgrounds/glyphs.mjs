import { createRequire } from 'node:module'
import { GLYPHS } from './schema/glyphs.mjs'

export { GLYPHS }
/** `ArrowDownload20Regular` → `arrow_download_20_regular`. */
export const snake = name =>
  name
    .replace(/([a-z])([A-Z0-9])/g, '$1_$2')
    .replace(/([0-9])([A-Z])/g, '$1_$2')
    .toLowerCase()
/** `ArrowDownload20Regular` → `arrowDownload20Regular`. */
export const camel = name => name[0].toLowerCase() + name.slice(1)

let renderer
/** The glyph's grid size and path data, exactly as React renders it. */
export function glyphSvg(name) {
  if (!Object.hasOwn(GLYPHS, name)) throw Error(`Unknown glyph ${name}`)
  renderer ??= (() => {
    const require = createRequire(
      new URL('../../packages/ui_react/example/package.json', import.meta.url)
    )
    const React = require('react')
    const { renderToStaticMarkup } = require('react-dom/server')
    const icons = require('@fluentui/react-icons')
    return glyph => renderToStaticMarkup(React.createElement(icons[glyph]))
  })()
  const markup = renderer(name)
  const grid = Number(/viewBox="0 0 (\d+) \1"/.exec(markup)?.[1])
  const paths = [...markup.matchAll(/ d="([^"]+)"/g)].map(match => match[1])
  if (!grid || !paths.length) throw Error(`Unreadable glyph ${name}`)
  return { grid, data: paths.join(' ') }
}
