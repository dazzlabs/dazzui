import { useCallback, useSyncExternalStore } from 'react'

/**
 * The whole address is the state: the query for the theme and the
 * platforms, the hash for the story. A link is a complete description of
 * what it shows.
 */
const CHANGE = 'example:location'

function subscribe(notify: () => void) {
  window.addEventListener('popstate', notify)
  window.addEventListener('hashchange', notify)
  window.addEventListener(CHANGE, notify)
  return () => {
    window.removeEventListener('popstate', notify)
    window.removeEventListener('hashchange', notify)
    window.removeEventListener(CHANGE, notify)
  }
}

const read = () => location.search + location.hash

export function useQuery(name: string, fallback: string) {
  const address = useSyncExternalStore(subscribe, read)
  const value = new URLSearchParams(address.split('#')[0]).get(name) ?? fallback
  const set = useCallback(
    (next: string) => {
      const url = new URL(location.href)
      if (next === fallback) url.searchParams.delete(name)
      else url.searchParams.set(name, next)
      history.replaceState(null, '', url)
      window.dispatchEvent(new Event(CHANGE))
    },
    [name, fallback]
  )
  return [value, set] as const
}

/** The story: an id, or undefined for the first one. */
export function useStory() {
  const address = useSyncExternalStore(subscribe, read)
  const story = address.split('#/')[1] || undefined
  const open = useCallback((next: string) => {
    location.hash = `/${next}`
  }, [])
  return [story, open] as const
}
