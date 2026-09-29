import { GLYPHS } from './schema/glyphs.mjs'

export { GLYPHS }
/** `ArrowDownload20Regular` → `arrow_download_20_regular`. */
export const snake = name =>
  name
    .replace(/([a-z])([A-Z0-9])/g, '$1_$2')
    .replace(/([0-9])([A-Z])/g, '$1_$2')
    .toLowerCase()
