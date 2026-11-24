import { DazzTokens, THEME_NAMES } from './Tokens'

export const SWATCH_CORNER_RADIUS: number = 8

export interface DazzPreferenceThemeOption {
  value: string
  label: string
  theme?: string
}

export interface DazzSwatchPalette {
  themeName: string
  surface: string
  column: string
  border: string
  ink: string
  accent: string
  swatchWidth: number
  swatchHeight: number
  radius: number
  hairline: number
  paperPadding: number
  inkHeight: number
  accentHeight: number
  barRadius: number
  ringGap: number
  ringGapColor: string
  ringStroke: number
  ringColor: string
}

/**
 * Resolve the palette name a swatch draws in.
 * Matches React/Flutter: uses `option.theme` when provided (with `""` denoting
 * the `:root` baseline `'bright-light'`), falling back to `option.value`, then
 * `fallback` or `'bright-light'`.
 */
export function resolveSwatchThemeName(
  option: DazzPreferenceThemeOption,
  fallback: string = 'bright-light'
): string {
  const candidate = option.theme !== undefined ? option.theme : option.value
  if (candidate === '') return 'bright-light'
  if (THEME_NAMES.indexOf(candidate) >= 0) return candidate
  if (THEME_NAMES.indexOf(fallback) >= 0) return fallback
  return 'bright-light'
}

export function resolveSwatchPalette(
  option: DazzPreferenceThemeOption,
  outerTokens: DazzTokens
): DazzSwatchPalette {
  const themeName = resolveSwatchThemeName(option, outerTokens.name)
  const swatchTokens =
    themeName === outerTokens.name
      ? outerTokens
      : new DazzTokens(themeName, outerTokens.systemStyle)
  const swatchHeight = swatchTokens.size('small')
  return {
    themeName,
    surface: swatchTokens.color('surface'),
    column: swatchTokens.color('surface-column'),
    border: swatchTokens.color('border-strong'),
    ink: swatchTokens.color('content'),
    accent: swatchTokens.color('primary.500'),
    swatchWidth: swatchHeight * 1.5,
    swatchHeight,
    radius: SWATCH_CORNER_RADIUS,
    hairline: swatchTokens.metric('stroke.hairline'),
    paperPadding: swatchTokens.metric('spacing.1'),
    inkHeight: swatchTokens.metric('spacing.0.5'),
    accentHeight: swatchTokens.metric('spacing.1.5'),
    barRadius: swatchTokens.metric('spacing.px'),
    ringGap: outerTokens.metric('stroke.hairline'),
    ringGapColor: outerTokens.color('surface'),
    ringStroke: outerTokens.metric('stroke.control'),
    ringColor: outerTokens.paint('outlined', 'border', 'normal', 'primary'),
  }
}
