/** Resolved theme snapshots embedded in the standalone WinUI assembly. */
export default function winUI3Plugin({ modeSelectors = [] } = {}) {
  return {
    name: '@dazzlabs/dazzui-ds-plugin-winui3',
    build({ tokens, outputFile }) {
      const themes = new Map([['studio-light', []]])
      for (const { mode, selectors } of modeSelectors) {
        for (const selector of selectors) {
          const name = /\[data-theme='([^']+)'\]/.exec(selector)?.[1]
          if (!name) continue
          if (!themes.has(name)) themes.set(name, [])
          themes.get(name).push(mode)
        }
      }
      const result = {}
      for (const [name, modes] of themes) {
        const resolve = (id, seen = new Set()) => {
          if (seen.has(id)) throw new Error(`Circular WinUI token: ${id}`)
          const token = tokens[id]
          if (!token) throw new Error(`Missing WinUI token: ${id}`)
          let picked = token
          for (const mode of modes) {
            if (token.mode?.[mode]) picked = token.mode[mode]
          }
          const next = new Set([...seen, id])
          if (picked.aliasOf) return resolve(picked.aliasOf, next)
          const expand = value => {
            if (typeof value === 'string') {
              const ref = /^\{(.+)\}$/.exec(value)
              return ref ? resolve(ref[1], next) : value
            }
            if (Array.isArray(value)) return value.map(expand)
            if (value && typeof value === 'object') {
              if (value.colorSpace) return colorBytes(value)
              return Object.fromEntries(
                Object.entries(value).map(([key, entry]) => [
                  key,
                  expand(entry),
                ])
              )
            }
            return value
          }
          return expand(picked.$value)
        }
        result[name] = Object.fromEntries(
          Object.keys(tokens)
            .filter(id => !id.startsWith('base.'))
            .sort()
            .map(id => [id, resolve(id)])
        )
      }
      outputFile('winui3/tokens.json', `${JSON.stringify(result, null, 2)}\n`)
    },
  }
}

/** Same clipped sRGB conversion used by the native platform generators. */
function colorBytes(value) {
  let rgb = value.components
  if (value.colorSpace === 'oklch') {
    const [lightness, chroma, hue] = rgb
    const a = chroma * Math.cos((hue * Math.PI) / 180)
    const b = chroma * Math.sin((hue * Math.PI) / 180)
    const l = (lightness + 0.3963377774 * a + 0.2158037573 * b) ** 3
    const m = (lightness - 0.1055613458 * a - 0.0638541728 * b) ** 3
    const s = (lightness - 0.0894841775 * a - 1.291485548 * b) ** 3
    rgb = [
      4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
      -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
      -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
    ].map(v => (v <= 0.0031308 ? 12.92 * v : 1.055 * v ** (1 / 2.4) - 0.055))
  } else if (value.colorSpace !== 'srgb') {
    throw new Error(`Unsupported WinUI color space: ${value.colorSpace}`)
  }
  return [value.alpha ?? 1, ...rgb].map(v =>
    Math.round(Math.max(0, Math.min(1, v)) * 255)
  )
}
