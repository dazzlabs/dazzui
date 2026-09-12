import { rustSources } from './rust.mjs'

/** Emit the platform's committed Rust vocabulary from the shared token graph. */
export default function insetPlugin(options = {}) {
  return {
    name: '@dazzlabs/dazzui-ds-plugin-inset',
    async build({ tokens, sources, outputFile }) {
      const result = rustSources(
        tokens,
        sources,
        options.modeSelectors ?? [],
        'inset'
      )
      outputFile('inset/colors.rs', result.colors)
      outputFile('inset/theme_variables.rs', result.themeVariables)
    },
  }
}
