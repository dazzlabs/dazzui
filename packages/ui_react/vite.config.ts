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
    // React and the icon set stay external so the host app owns a single copy
    // of each. Both are declared — React as a peer, the icons as a dependency
    // — so a consumer has them either way, and bundling either here would
    // ship a second copy to anyone already using it.
    rollupOptions: {
      external: ['react', 'react/jsx-runtime', '@fluentui/react-icons'],
    },
    sourcemap: true,
  },
})
