import {
  DazzThemeFamily,
  DazzThemeGroup,
  DazzThemeName,
  DRAWN_THEME_FAMILIES,
  THEME_FAMILIES,
  THEME_NAMES,
  resolveThemeName,
} from '../generated/ThemeData'

export {
  DazzThemeFamily,
  DazzThemeGroup,
  DazzThemeName,
  DRAWN_THEME_FAMILIES,
  THEME_FAMILIES,
  resolveThemeName,
}

export type AppearanceMode = 'light' | 'dark' | 'system'
export type DazzAppearanceMode = AppearanceMode

export function isThemeName(value: string): boolean {
  return THEME_NAMES.indexOf(value) >= 0
}

export function isThemeFamily(value: string): boolean {
  return THEME_FAMILIES.indexOf(value as DazzThemeFamily) >= 0
}

export function isAppearanceMode(value: string): boolean {
  return value === 'light' || value === 'dark' || value === 'system'
}

export function isDarkTheme(theme: string): boolean {
  return theme.endsWith('-dark')
}

export function resolveAppearanceDark(
  mode: AppearanceMode,
  systemDark: boolean
): boolean {
  if (mode === 'dark') return true
  if (mode === 'light') return false
  return systemDark
}

export function resolveThemeForMode(
  family: DazzThemeFamily,
  mode: AppearanceMode,
  systemDark: boolean
): DazzThemeName {
  return resolveThemeName(family, resolveAppearanceDark(mode, systemDark))
}

export function parseThemeFamily(
  theme: string,
  fallback: DazzThemeFamily = 'bright'
): DazzThemeFamily {
  if (isThemeFamily(theme)) return theme as DazzThemeFamily
  if (theme.endsWith('-light')) {
    const stripped = theme.slice(0, theme.length - '-light'.length)
    if (isThemeFamily(stripped)) return stripped as DazzThemeFamily
  } else if (theme.endsWith('-dark')) {
    const stripped = theme.slice(0, theme.length - '-dark'.length)
    if (isThemeFamily(stripped)) return stripped as DazzThemeFamily
  }
  return fallback
}

export function parseAppearanceMode(
  value: string,
  fallback: AppearanceMode = 'system'
): AppearanceMode {
  return isAppearanceMode(value) ? (value as AppearanceMode) : fallback
}
