import {
  DEFAULT_THEME,
  DEFAULT_THEME_FAMILY,
  DRAWN_THEME_FAMILIES,
  isThemeFamily,
  isThemeName,
  resolveThemeName,
  THEME_FAMILIES,
  THEME_FAMILY_META,
  THEME_GROUPS,
  THEME_META,
  THEMES,
  type Appearance,
  type DrawnThemeFamily,
  type ThemeFamily,
  type ThemeFamilyMeta,
  type ThemeGroup,
  type ThemeGroupInfo,
  type ThemeMeta,
  type ThemeName,
} from '../generated/themes'

export {
  DEFAULT_THEME,
  DEFAULT_THEME_FAMILY,
  DRAWN_THEME_FAMILIES,
  isThemeFamily,
  isThemeName,
  resolveThemeName,
  THEME_FAMILIES,
  THEME_FAMILY_META,
  THEME_GROUPS,
  THEME_META,
  THEMES,
  type Appearance,
  type DrawnThemeFamily,
  type ThemeFamily,
  type ThemeFamilyMeta,
  type ThemeGroup,
  type ThemeGroupInfo,
  type ThemeMeta,
  type ThemeName,
}

export const APPEARANCE_MODES = ['light', 'dark', 'system'] as const

export type AppearanceMode = (typeof APPEARANCE_MODES)[number]

export function isAppearanceMode(value: unknown): value is AppearanceMode {
  return value === 'light' || value === 'dark' || value === 'system'
}

/** Resolves an `AppearanceMode` into a concrete `'light'` or `'dark'` appearance. */
export function resolveAppearance(
  mode: AppearanceMode,
  systemDark?: boolean
): Appearance {
  if (mode === 'light') return 'light'
  if (mode === 'dark') return 'dark'
  if (systemDark !== undefined) return systemDark ? 'dark' : 'light'
  if (
    typeof window !== 'undefined' &&
    typeof window.matchMedia === 'function'
  ) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light'
  }
  return 'light'
}

/**
 * Applies a DazzUI theme to the target element (defaulting to
 * `document.documentElement`). The default theme lives on `:root`, so applying
 * it removes `data-theme` instead of writing a redundant attribute.
 */
export function applyTheme(
  theme: ThemeName,
  target: Element | null | undefined = typeof document !== 'undefined'
    ? document.documentElement
    : null
): void {
  if (!target) return
  if (theme === DEFAULT_THEME) target.removeAttribute('data-theme')
  else target.setAttribute('data-theme', theme)
}

export interface ThemeInitScriptOptions {
  /** Fallback family when no stored family is present. Defaults to `DEFAULT_THEME_FAMILY`. */
  defaultFamily?: ThemeFamily
  /** Fallback appearance mode when no stored mode is present. Defaults to `'system'`. */
  defaultMode?: AppearanceMode
  /**
   * `localStorage` key for persisting the theme family. Omit for single-brand
   * sites that only switch light / dark / system within `defaultFamily`.
   */
  familyStorageKey?: string
  /** `localStorage` key for persisting the appearance mode. Defaults to `'dazzui-theme-mode'`. */
  modeStorageKey?: string
  /** Families accepted from storage when `familyStorageKey` is set. Defaults to `THEME_FAMILIES`. */
  allowedFamilies?: readonly ThemeFamily[]
}

/**
 * Produces an inline, zero-dependency script string to place in `<head>` for
 * FOUC-free SSR theme initialization. Reads persisted preferences (handling
 * blocked storage safely), applies the resolved `data-theme` attribute to
 * `document.documentElement`, and keeps `'system'` mode synced with
 * `prefers-color-scheme`.
 */
export function themeInitScript({
  defaultFamily = DEFAULT_THEME_FAMILY,
  defaultMode = 'system',
  familyStorageKey,
  modeStorageKey = 'dazzui-theme-mode',
  allowedFamilies = THEME_FAMILIES,
}: ThemeInitScriptOptions = {}): string {
  const familyKeyLiteral =
    familyStorageKey === undefined ? 'null' : JSON.stringify(familyStorageKey)
  const familiesLiteral =
    familyStorageKey === undefined ? 'null' : JSON.stringify(allowedFamilies)

  return `(function(){var modes=${JSON.stringify(APPEARANCE_MODES)};var defaultFamily=${JSON.stringify(defaultFamily)};var defaultMode=${JSON.stringify(defaultMode)};var defaultTheme=${JSON.stringify(DEFAULT_THEME)};var modeKey=${JSON.stringify(modeStorageKey)};var familyKey=${familyKeyLiteral};var families=${familiesLiteral};var media=window.matchMedia('(prefers-color-scheme: dark)');var apply=function(){var mode=defaultMode;var family=defaultFamily;try{var sm=window.localStorage.getItem(modeKey);if(modes.indexOf(sm)!==-1)mode=sm;if(familyKey){var sf=window.localStorage.getItem(familyKey);if(families&&families.indexOf(sf)!==-1)family=sf}}catch(e){}var dark=mode==='dark'||(mode==='system'&&media.matches);var theme=family+'-'+(dark?'dark':'light');var root=document.documentElement;if(theme===defaultTheme)root.removeAttribute('data-theme');else root.setAttribute('data-theme',theme)};apply();if(media.addEventListener)media.addEventListener('change',apply)})()`
}
