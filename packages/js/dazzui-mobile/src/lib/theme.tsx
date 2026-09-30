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
    family: family ?? parent?.family ?? 'studio',
    appearance: appearance ?? parent?.appearance ?? 'system',
    platform: platform === 'auto' ? (parent?.platform ?? detected) : platform,
    container,
  }

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
