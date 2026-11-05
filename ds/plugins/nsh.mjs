import { generate } from './gpui.mjs'

/** The Rust token representation is shared; each toolkit owns its adapters. */
export default function nshPlugin({ modeSelectors = [] } = {}) {
  return {
    name: '@dazzlabs/dazzui-ds-plugin-nsh',
    build({ tokens, outputFile }) {
      outputFile(
        'nsh/theme_tokens.rs',
        generate(tokens, modeSelectors, { generator: 'nsh' })
      )
    },
  }
}
