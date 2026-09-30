import { existsSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'

/**
 * The viewer reads a run as static files: `data/` when a collection has
 * been made on this machine, the made-up `fixtures/` otherwise. Nothing in
 * the viewer knows which; it asks for `/run.json` and goes from there.
 */
const here = fileURLToPath(new URL('.', import.meta.url))
const data = `${here}data`
const fixtures = `${here}fixtures`

export default defineConfig({
  publicDir: existsSync(`${data}/run.json`) ? data : fixtures,
})
