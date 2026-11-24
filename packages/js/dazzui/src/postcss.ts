/**
 * Minimal structural PostCSS interfaces so `pruneUnusedThemes` ships with
 * zero runtime or type dependencies on `postcss` and works in any PostCSS 8
 * config (`vite.config.ts`, `postcss.config.mjs`, etc.).
 */
export interface PostcssRuleLike {
  selector: string
  selectors: string[]
  remove(): unknown
}

export interface PostcssAtRuleLike {
  nodes?: unknown[]
  remove(): unknown
}

export interface PostcssRootLike {
  walkRules(callback: (rule: PostcssRuleLike) => void): void
  walkAtRules(callback: (atRule: PostcssAtRuleLike) => void): void
}

export interface PostcssPluginLike {
  postcssPlugin: string
  OnceExit(root: PostcssRootLike): void
}

const THEME_ATTRIBUTE =
  /\[data-theme\s*([~|^$*]?=)\s*["']?([^"'\]\s]+)["']?\s*\]/g

/**
 * PostCSS plugin that strips `[data-theme=...]` rules not listed in `keep`
 * while preserving DazzUI's shared `:root` foundations.
 *
 * Pass the `data-theme` names your application uses (for example
 * `['bright-light', 'bright-dark']`). Bright Light lives on `:root` and is
 * always retained.
 */
export function pruneUnusedThemes(keep: readonly string[]): PostcssPluginLike {
  function canMatch(selector: string) {
    // Boolean selector functions need more than a conjunction of attributes.
    // Retain them conservatively instead of deleting a possibly valid branch.
    if (/:(?:is|where|not|has)\(/.test(selector)) return true

    return [...selector.matchAll(THEME_ATTRIBUTE)].every(
      ([, operator, value]) => {
        if (value === undefined) return true
        return keep.some(theme => {
          switch (operator) {
            case '=':
              return theme === value
            case '^=':
              return theme.startsWith(value)
            case '$=':
              return theme.endsWith(value)
            case '*=':
              return theme.includes(value)
            case '|=':
              return theme === value || theme.startsWith(`${value}-`)
            case '~=':
              return theme.split(/\s+/).includes(value)
            default:
              return true
          }
        })
      }
    )
  }

  return {
    postcssPlugin: 'dazzui-prune-unused-themes',
    OnceExit(root) {
      root.walkRules(rule => {
        if (!rule.selector.includes('data-theme')) return
        const selectors = rule.selectors.filter(canMatch)
        if (selectors.length) rule.selectors = selectors
        else rule.remove()
      })

      let removed = true
      while (removed) {
        removed = false
        root.walkAtRules(atRule => {
          if (atRule.nodes?.length === 0) {
            atRule.remove()
            removed = true
          }
        })
      }
    },
  }
}
