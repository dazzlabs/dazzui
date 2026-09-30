import { useCallback, useSyncExternalStore } from 'react'

/**
 * The whole address is the state: `?theme=` for the theme, the hash for
 * the page. Nothing else remembers anything, so a link is a
 * complete description of what it shows.
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

export function useQuery(name: string) {
  const address = useSyncExternalStore(subscribe, read)
  const value = new URLSearchParams(address.split('#')[0]).get(name)
  const set = useCallback(
    (next: string) => {
      const url = new URL(location.href)
      url.searchParams.set(name, next)
      history.replaceState(null, '', url)
      window.dispatchEvent(new Event(CHANGE))
    },
    [name]
  )
  return [value, set] as const
}

/** The page: a component id, or undefined for the overview. */
export function usePage() {
  const address = useSyncExternalStore(subscribe, read)
  const page = address.split('#/')[1] || undefined
  const open = useCallback((next?: string) => {
    location.hash = next ? `/${next}` : '/'
    window.scrollTo(0, 0)
  }, [])
  return [page, open] as const
}
