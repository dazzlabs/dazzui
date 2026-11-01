import { CustomTheme } from '@kit.ArkUI'
import { common, Configuration, ConfigurationConstant } from '@kit.AbilityKit'
import { resourceManager } from '@kit.LocalizationKit'
import { DazzThemeFamily } from './generated/DazzThemes'
import { resolvedTheme } from './ResolvedTheme'

@Builder
function emptyContent() {
  Column() {}
}

/** Color-only scope for stock ArkUI controls. System appearance by default. */
@Component
export struct DazzMobileTheme {
  @Prop family: DazzThemeFamily = DazzThemeFamily.Bright
  @Prop appearance: ThemeColorMode = ThemeColorMode.SYSTEM
  @BuilderParam content: () => void = emptyContent
  @State private systemDark: boolean = false
  private app?: common.ApplicationContext
  private environmentId: number = -1

  aboutToAppear(): void {
    const context = this.getUIContext().getHostContext()
    this.systemDark = context?.resourceManager.getConfigurationSync().colorMode ===
      resourceManager.ColorMode.DARK
    this.app = context?.getApplicationContext()
    this.environmentId = this.app?.on('environment', {
      onConfigurationUpdated: (configuration: Configuration) => {
        if (configuration.colorMode === ConfigurationConstant.ColorMode.COLOR_MODE_DARK ||
          configuration.colorMode === ConfigurationConstant.ColorMode.COLOR_MODE_LIGHT) {
          this.systemDark = configuration.colorMode === ConfigurationConstant.ColorMode.COLOR_MODE_DARK
        }
      },
      onMemoryLevel: () => {}
    }) ?? -1
  }

  aboutToDisappear(): void {
    if (this.environmentId !== -1) {
      this.app?.off('environment', this.environmentId, () => {})
      this.environmentId = -1
    }
  }

  effectiveTheme(): CustomTheme {
    const dark = this.appearance === ThemeColorMode.DARK ||
      (this.appearance === ThemeColorMode.SYSTEM && this.systemDark)
    return resolvedTheme(this.family, dark)
  }

  build() {
    WithTheme({ theme: this.effectiveTheme(), colorMode: this.appearance }) {
      this.content()
    }
  }
}
