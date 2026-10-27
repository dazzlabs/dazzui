import { UIContext, font } from '@kit.ArkUI'
import { common } from '@kit.AbilityKit'

export interface DazzSystemFontSettings {
  fontId: string
  fontScale: number
  fontWeightScale: number
  followsSystem: boolean
  fontFamilies: string[]
}

/** Native controls resolve the selected face and scaling themselves. */
export function readSystemFontSettings(ui: UIContext): DazzSystemFontSettings {
  const context = ui.getHostContext() as common.UIAbilityContext
  if (!context)
    throw new Error('Reading system fonts requires a live UIAbility context')
  const configuration = context.config
  const settings: DazzSystemFontSettings = {
    fontId: configuration.fontId ?? '',
    fontScale: configuration.fontSizeScale ?? 1,
    fontWeightScale: configuration.fontWeightScale ?? 1,
    followsSystem: ui.isFollowingSystemFontScale(),
    fontFamilies: [],
  }
  ui.runScopedTask(() => {
    const config = font.getUIFontConfig()
    settings.fontFamilies =
      config?.generic.map(
        (entry: font.UIFontGenericInfo): string => entry.family
      ) ?? []
  })
  return settings
}
