import { resolveThemes } from './resolve.mjs'

/** Resolved theme snapshots embedded in the standalone WinUI assembly. */
export default function winUI3Plugin({ modeSelectors = [] } = {}) {
  return {
    name: '@dazzlabs/dazzui-ds-plugin-winui3',
    build({ tokens, outputFile }) {
      const result = resolveThemes(tokens, modeSelectors)
      outputFile('winui3/tokens.json', `${JSON.stringify(result, null, 2)}\n`)
    },
  }
}
