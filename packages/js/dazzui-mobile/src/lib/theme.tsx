import {
  type ComponentProps,
  createContext,
  type RefObject,
  useContext,
  useEffect,
  useRef,
  useState,
} from 'react'

import type { Family } from '../generated/families'
import { cx } from './cx'

/** Which OS the controls are drawn as. */
export type Platform = 'ios' | 'android'

/** `system` follows `prefers-color-scheme`, as the native packages follow the OS. */
export type Appearance = 'light' | 'dark' | 'system'

interface ThemeContextValue {
  family: Family
  appearance: Appearance
  platform: Platform
  container: RefObject<HTMLDivElement | null>
}

const ThemeContext = createContext<ThemeContextValue | null>(null)

/** The OS the page is running on, read off the user agent; iOS otherwise. */
export function detectPlatform(): Platform {
  if (typeof navigator === 'undefined') return 'ios'
  return /android/i.test(navigator.userAgent) ? 'android' : 'ios'
}

export interface MobileThemeProps extends Omit<ComponentProps<'div'>, 'ref'> {
  /** A ds family; the OS picks its light or dark half. */
  family?: Family
  appearance?: Appearance
  /** `auto` reads the user agent once, on mount. */
  platform?: Platform | 'auto'
  /**
   * Keep sheets, alerts, menus and toasts inside this element rather than
   * over the viewport — what a device frame in a gallery wants. A theme that
   * is the whole page leaves it off.
   */
  contained?: boolean
}

/**
 * Installs a ds family and an OS on everything under it, the way
 * `mobileTheme` and `DazzTheme` do in the native packages. The element
 * carries `data-family`, `data-appearance` and `data-platform`, which is
 * all the stylesheets read, and it is the container every floating part
 * portals into, so a sheet inherits the colours of the screen it opens
 * over. Nested, it inherits whatever it is not given.
 */
export function MobileTheme({
  family,
  appearance,
  platform = 'auto',
  contained = false,
  className,
  children,
  ...props
}: MobileThemeProps) {
  const parent = useContext(ThemeContext)
  const container = useRef<HTMLDivElement>(null)
  const [detected, setDetected] = useState<Platform>(
    () => parent?.platform ?? 'ios'
  )
  useEffect(() => {
    if (platform === 'auto' && !parent) setDetected(detectPlatform())
  }, [platform, parent])

  const value: ThemeContextValue = {
    family: family ?? parent?.family ?? 'bright',
    appearance: appearance ?? parent?.appearance ?? 'system',
    platform: platform === 'auto' ? (parent?.platform ?? detected) : platform,
    container,
  }

  useRipple(container, value.platform === 'android')

  return (
    <ThemeContext.Provider value={value}>
      <div
        ref={container}
        data-family={value.family}
        data-appearance={
          value.appearance === 'system' ? undefined : value.appearance
        }
        data-platform={value.platform}
        data-contained={contained ? '' : undefined}
        className={cx('dzm-theme', className)}
        {...props}
      >
        {children}
      </div>
    </ThemeContext.Provider>
  )
}

/** The OS the enclosing theme draws for; iOS outside any theme. */
export function usePlatform(): Platform {
  return useContext(ThemeContext)?.platform ?? 'ios'
}

/** Where a floating part portals to: the enclosing theme's element. */
export function useThemeContainer():
  | RefObject<HTMLDivElement | null>
  | undefined {
  return useContext(ThemeContext)?.container
}

/**
 * Material's ripple, for every part under the theme that wears the state
 * layer. One listener on the theme's element starts it where the pointer
 * lands and lets it go on release; the drawing is the part's `::after`,
 * placed by three custom properties, so nothing is added to a tree React
 * owns. A nested theme draws its own, and a disabled part none.
 */
function useRipple(
  container: RefObject<HTMLDivElement | null>,
  enabled: boolean
) {
  useEffect(() => {
    const root = container.current
    if (!root || !enabled) return
    const release = (target: HTMLElement) => {
      if (target.dataset.ripple !== 'down') return
      target.dataset.ripple = 'up'
      // Once it has faded, the part is back to its keyboard press.
      window.setTimeout(() => {
        if (target.dataset.ripple === 'up') delete target.dataset.ripple
      }, 600)
    }
    const onDown = (event: PointerEvent) => {
      if (event.button !== 0) return
      const target = (event.target as Element | null)?.closest<HTMLElement>(
        '.dzm-m3-layer'
      )
      if (!target || target.closest('.dzm-theme') !== root) return
      if (
        target.matches(':disabled, [data-disabled], [aria-disabled="true"]')
      ) {
        return
      }
      const box = target.getBoundingClientRect()
      const x = event.clientX - box.left
      const y = event.clientY - box.top
      // The radius reaches the farthest corner, so the wave fills the part.
      const radius = Math.hypot(
        Math.max(x, box.width - x),
        Math.max(y, box.height - y)
      )
      target.style.setProperty('--dzm-ripple-x', `${x}px`)
      target.style.setProperty('--dzm-ripple-y', `${y}px`)
      target.style.setProperty('--dzm-ripple-size', `${radius * 2}px`)
      delete target.dataset.ripple
      // A second press restarts the wave rather than joining the first.
      void target.offsetWidth
      target.dataset.ripple = 'down'
      const up = () => {
        release(target)
        window.removeEventListener('pointerup', up)
        window.removeEventListener('pointercancel', up)
      }
      window.addEventListener('pointerup', up)
      window.addEventListener('pointercancel', up)
    }
    root.addEventListener('pointerdown', onDown)
    return () => root.removeEventListener('pointerdown', onDown)
  }, [container, enabled])
}
