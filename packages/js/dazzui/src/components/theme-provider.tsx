import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  useSyncExternalStore,
} from 'react'

import {
  type Appearance,
  type AppearanceMode,
  applyTheme,
  DEFAULT_THEME_FAMILY,
  isAppearanceMode,
  isThemeFamily,
  resolveAppearance,
  resolveThemeName,
  type ThemeFamily,
  type ThemeName,
} from '../lib/theme'

export interface ThemeContextValue {
  family: ThemeFamily
  mode: AppearanceMode
  /** Concrete resolved appearance (`'light'` or `'dark'`). */
  appearance: Appearance
  /** Concrete resolved DazzUI theme name (e.g. `'bright-dark'`). */
  theme: ThemeName
  setFamily: (family: ThemeFamily) => void
  setMode: (mode: AppearanceMode) => void
}

const ThemeContext = createContext<ThemeContextValue | null>(null)

const DARK_MEDIA_QUERY = '(prefers-color-scheme: dark)'

function subscribeSystemDark(callback: () => void) {
  if (
    typeof window === 'undefined' ||
    typeof window.matchMedia !== 'function'
  ) {
    return () => {}
  }
  const media = window.matchMedia(DARK_MEDIA_QUERY)
  media.addEventListener('change', callback)
  return () => media.removeEventListener('change', callback)
}

function getSystemDarkSnapshot() {
  if (
    typeof window === 'undefined' ||
    typeof window.matchMedia !== 'function'
  ) {
    return false
  }
  return window.matchMedia(DARK_MEDIA_QUERY).matches
}

function getSystemDarkServerSnapshot() {
  return false
}

function readStoredValue(key: string | undefined): string | null {
  if (!key || typeof window === 'undefined') return null
  try {
    return window.localStorage.getItem(key)
  } catch {
    return null
  }
}

function writeStoredValue(key: string | undefined, value: string): void {
  if (!key || typeof window === 'undefined') return
  try {
    window.localStorage.setItem(key, value)
  } catch {
    // Blocked storage still allows the theme to apply for the current session.
  }
}

export interface ThemeProviderProps {
  /** Initial family when uncontrolled. Defaults to `DEFAULT_THEME_FAMILY`. */
  defaultFamily?: ThemeFamily
  /** Initial appearance mode when uncontrolled. Defaults to `'system'`. */
  defaultMode?: AppearanceMode
  /** Controlled theme family. */
  family?: ThemeFamily
  /** Controlled appearance mode. */
  mode?: AppearanceMode
  onFamilyChange?: (family: ThemeFamily) => void
  onModeChange?: (mode: AppearanceMode) => void
  /** Optional `localStorage` key for persisting `family`. */
  familyStorageKey?: string
  /** Optional `localStorage` key for persisting `mode`. */
  modeStorageKey?: string
  /**
   * Element to write `data-theme` onto. Defaults to `document.documentElement`;
   * pass `null` to skip DOM attribute updates.
   */
  target?: Element | null
  children?: ReactNode
}

/**
 * Manages a DazzUI theme family and appearance mode (`light` / `dark` /
 * `system`), tracks `prefers-color-scheme`, optionally persists to
 * `localStorage`, and applies `data-theme` to `document.documentElement`.
 */
export function ThemeProvider({
  defaultFamily = DEFAULT_THEME_FAMILY,
  defaultMode = 'system',
  family: familyProp,
  mode: modeProp,
  onFamilyChange,
  onModeChange,
  familyStorageKey,
  modeStorageKey,
  target,
  children,
}: ThemeProviderProps) {
  const [ownFamily, setOwnFamily] = useState<ThemeFamily>(defaultFamily)
  const [ownMode, setOwnMode] = useState<AppearanceMode>(defaultMode)
  const [storageReady, setStorageReady] = useState(
    () => !familyStorageKey && !modeStorageKey
  )

  const systemDark = useSyncExternalStore(
    subscribeSystemDark,
    getSystemDarkSnapshot,
    getSystemDarkServerSnapshot
  )

  useEffect(() => {
    if (familyProp === undefined && familyStorageKey) {
      const stored = readStoredValue(familyStorageKey)
      if (stored && isThemeFamily(stored)) setOwnFamily(stored)
    }
    if (modeProp === undefined && modeStorageKey) {
      const stored = readStoredValue(modeStorageKey)
      if (isAppearanceMode(stored)) setOwnMode(stored)
    }
    setStorageReady(true)
  }, [familyProp, modeProp, familyStorageKey, modeStorageKey])

  useEffect(() => {
    if (
      typeof window === 'undefined' ||
      (!familyStorageKey && !modeStorageKey)
    ) {
      return
    }
    const onStorage = (event: StorageEvent) => {
      if (
        familyProp === undefined &&
        familyStorageKey &&
        event.key === familyStorageKey &&
        event.newValue &&
        isThemeFamily(event.newValue)
      ) {
        setOwnFamily(event.newValue)
      }
      if (
        modeProp === undefined &&
        modeStorageKey &&
        event.key === modeStorageKey &&
        isAppearanceMode(event.newValue)
      ) {
        setOwnMode(event.newValue)
      }
    }
    window.addEventListener('storage', onStorage)
    return () => window.removeEventListener('storage', onStorage)
  }, [familyProp, modeProp, familyStorageKey, modeStorageKey])

  const family = familyProp ?? ownFamily
  const mode = modeProp ?? ownMode
  const appearance = resolveAppearance(mode, systemDark)
  const theme = resolveThemeName(family, appearance)

  useEffect(() => {
    if (!storageReady) return
    if (target === null) return
    applyTheme(theme, target)
  }, [theme, target, storageReady])

  const setFamily = useCallback(
    (next: ThemeFamily) => {
      if (familyProp === undefined) setOwnFamily(next)
      writeStoredValue(familyStorageKey, next)
      onFamilyChange?.(next)
    },
    [familyProp, familyStorageKey, onFamilyChange]
  )

  const setMode = useCallback(
    (next: AppearanceMode) => {
      if (modeProp === undefined) setOwnMode(next)
      writeStoredValue(modeStorageKey, next)
      onModeChange?.(next)
    },
    [modeProp, modeStorageKey, onModeChange]
  )

  const value = useMemo<ThemeContextValue>(
    () => ({
      family,
      mode,
      appearance,
      theme,
      setFamily,
      setMode,
    }),
    [family, mode, appearance, theme, setFamily, setMode]
  )

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
}

export function useTheme(): ThemeContextValue {
  const context = useContext(ThemeContext)
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider.')
  }
  return context
}
