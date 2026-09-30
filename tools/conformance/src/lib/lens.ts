import { useQuery } from './location'
import type { Loaded } from './model'
import { type Theme, parseTheme, themeId } from './themes'

/*
 * The lens: what the document is looked at through. All of it is in the
 * address, so a link carries the lens along with the page.
 */

export type Show = 'all' | 'off' | 'unread'

export interface Lens {
  theme: Theme
  setTheme: (theme: Theme) => void
  /** The implementations shown, in the run's order. */
  platforms: readonly string[]
  setPlatforms: (platforms: readonly string[]) => void
  show: Show
  setShow: (show: Show) => void
}

export function useLens(loaded: Loaded): Lens {
  const [themeValue, setThemeValue] = useQuery('theme')
  const [platformsValue, setPlatformsValue] = useQuery('platforms')
  const [showValue, setShowValue] = useQuery('show')

  const chosen = platformsValue?.split(',').filter(Boolean)
  const platforms = chosen
    ? loaded.implementations.filter(p => chosen.includes(p))
    : loaded.implementations
  const show: Show =
    showValue === 'off' || showValue === 'unread' ? showValue : 'all'

  return {
    theme: parseTheme(themeValue),
    setTheme: theme => setThemeValue(themeId(theme)),
    platforms,
    setPlatforms: next => {
      const all = next.length === loaded.implementations.length
      setPlatformsValue(all ? null : next.join(','))
    },
    show,
    setShow: next => setShowValue(next === 'all' ? null : next),
  }
}
