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
    // React, the icon set and Base UI stay external so the host app owns a
    // single copy of each. All three are declared — React as a peer, the
    // other two as dependencies — so a consumer has them either way, and
    // bundling any of them here would ship a second copy to anyone already
    // using it. Base UI in particular holds the popup and toast state in
    // React context, and two copies would be two contexts.
    rollupOptions: {
      external: [
        'react',
        'react/jsx-runtime',
        '@fluentui/react-icons',
        /^@base-ui\//,
      ],
    },
    sourcemap: true,
  },
})
