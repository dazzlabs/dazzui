import { CustomTheme } from '@kit.ArkUI'
import { dazzTheme, DazzThemeFamily } from './generated/DazzThemes'

/** Keep native controls and onWillApplyTheme on the same effective palette. */
export function resolvedTheme(family: DazzThemeFamily, dark: boolean): CustomTheme {
  const paired = dazzTheme(family)
  const selected = dark ? paired.darkColors : paired.colors
  return { colors: selected, darkColors: selected }
}
