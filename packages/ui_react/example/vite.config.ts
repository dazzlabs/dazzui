import { defineConfig } from 'vite'
import catalog from './src/generated/catalog.json' with { type: 'json' }

export default defineConfig({
  plugins: [
    {
      name: 'playground-catalog',
      generateBundle() {
        this.emitFile({
          type: 'asset',
          fileName: 'index.json',
          source: JSON.stringify({
            entries: Object.fromEntries(catalog.stories.map(s => [s.id, s])),
          }),
        })
      },
    },
  ],
})
