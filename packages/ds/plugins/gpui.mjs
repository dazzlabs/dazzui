import { rustSources } from './rust.mjs'

/** Emit the platform's committed Rust vocabulary from the shared token graph. */
export default function gpuiPlugin(options = {}) {
  return {
    name: '@dazzlabs/dazzui-ds-plugin-gpui',
    async build({ tokens, sources, outputFile }) {
      const result = rustSources(
        tokens,
        sources,
        options.modeSelectors ?? [],
        'gpui'
      )
      outputFile('gpui/colors.rs', result.colors)
      outputFile('gpui/theme_variables.rs', result.themeVariables)
    },
  }
}
