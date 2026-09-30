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
    // The glass reads `light-dark()` off the `color-scheme` the colour
    // sheet sets. Lowered, it would wait on variables only a `color-scheme`
    // in this same build defines — and the colour sheet ships beside it,
    // uncompiled — so the stylesheet targets the browsers that have it.
    cssTarget: ['chrome123', 'edge123', 'firefox120', 'safari17.5'],
    // React and Base UI stay external for the reason the desktop package
    // keeps them so: the host owns one copy of each, and Base UI holds the
    // open popup and the toast queue in React context.
    rollupOptions: {
      external: ['react', 'react/jsx-runtime', /^@base-ui\//],
    },
    sourcemap: true,
  },
})
