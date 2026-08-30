import { resolve } from 'node:path'

import { defineConfig } from 'vite'
import dts from 'vite-plugin-dts'

export default defineConfig({
  plugins: [dts({ include: ['src'] })],
  build: {
    lib: {
      entry: {
        index: resolve(import.meta.dirname, 'src/index.ts'),
        // Separate entry so the stylesheet is emitted without the component
        // bundle carrying a CSS side effect.
        styles: resolve(import.meta.dirname, 'src/styles.ts'),
      },
      formats: ['es'],
      fileName: (_format, name) => `${name}.js`,
      cssFileName: 'components',
    },
    cssCodeSplit: false,
    // React stays external so the host app owns a single copy.
    rollupOptions: {
      external: ['react', 'react/jsx-runtime'],
    },
    sourcemap: true,
  },
})
