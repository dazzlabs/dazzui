import { createReadStream, existsSync, statSync } from 'node:fs'
import { extname, join, normalize, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { defineConfig, type Plugin } from 'vite'

/**
 * The viewer reads a run as static files: `data/` when a collection has
 * been made on this machine, the made-up `fixtures/` otherwise. Nothing in
 * the viewer knows which; it asks for `/run.json` and goes from there.
 *
 * The choice is made on every request rather than once at start, so a
 * viewer left open picks up `pnpm conformance` as soon as it has run — and
 * shows the fixtures only while no run exists.
 */
const here = fileURLToPath(new URL('.', import.meta.url))
const data = resolve(process.env.DAZZUI_CONFORMANCE_DATA ?? join(here, 'data'))
const fixtures = join(here, 'fixtures')

const TYPES: Record<string, string> = {
  '.json': 'application/json',
  '.png': 'image/png',
  '.svg': 'image/svg+xml',
}

function run(): Plugin {
  return {
    name: 'conformance-run',
    configureServer(server) {
      server.middlewares.use((request, response, next) => {
        const path = decodeURIComponent((request.url ?? '/').split('?')[0]!)
        const type = TYPES[extname(path)]
        if (!type) return next()
        const root = existsSync(join(data, 'run.json')) ? data : fixtures
        const file = normalize(join(root, path))
        if (!file.startsWith(root) || !existsSync(file)) return next()
        if (!statSync(file).isFile()) return next()
        response.setHeader('Content-Type', type)
        response.setHeader('Cache-Control', 'no-store')
        createReadStream(file).pipe(response)
      })
    },
  }
}

export default defineConfig({
  publicDir: false,
  plugins: [run()],
})
