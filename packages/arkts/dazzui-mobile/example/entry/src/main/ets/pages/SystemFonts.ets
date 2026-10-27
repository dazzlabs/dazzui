import { common } from '@kit.AbilityKit'
import { hilog } from '@kit.PerformanceAnalysisKit'
import { DazzMobileTheme, readSystemFontSettings, DazzThemeFamily } from '@dazzlabs/dazzui-mobile'

@Component
struct NativeFontSpecimen {
  @LocalStorageProp('runId') runId: string = ''
  private brand: ResourceColor = ''
  private app?: common.ApplicationContext
  private environmentId: number = -1
  private sequence: number = 0
  private timer: number = -1
  aboutToAppear(): void {
    this.app = this.getUIContext().getHostContext()?.getApplicationContext()
    this.environmentId = this.app?.on('environment', {
      onConfigurationUpdated: () => {
        clearTimeout(this.timer)
        this.timer = setTimeout(() => this.snapshot(), 400)
      },
      onMemoryLevel: () => {}
    }) ?? -1
    this.timer = setTimeout(() => this.snapshot(), 400)
  }
  aboutToDisappear(): void {
    clearTimeout(this.timer)
    if (this.environmentId !== -1) this.app?.off('environment', this.environmentId, () => {})
  }
  onWillApplyTheme(theme: Theme): void { this.brand = theme.colors.brand }
  snapshot(): void {
    clearTimeout(this.timer)
    const ui = this.getUIContext()
    const settings = readSystemFontSettings(ui)
    const bounds = ui.getComponentUtils().getRectangleById('native-font-text')
    const button = ui.getComponentUtils().getRectangleById('native-font-button')
    const field = ui.getComponentUtils().getRectangleById('native-font-field')
    hilog.info(0x0000, 'DazzSystemFontTest', 'FONT_SNAPSHOT %{public}s', JSON.stringify({
      runId: this.runId, sequence: ++this.sequence, brand: this.brand, settings,
      height: ui.px2vp(bounds.size.height), font16: ui.px2vp(ui.fp2px(16)),
      buttonHeight: ui.px2vp(button.size.height), fieldHeight: ui.px2vp(field.size.height)
    }))
    this.timer = setTimeout(() => this.snapshot(), 500)
  }
  build() {
    Column({ space: 16 }) {
      Text('System text Ag国').fontSize(16).id('native-font-text')
      Button('System button').id('native-font-button')
      TextInput({ placeholder: 'Name' }).id('native-font-field')
      Button('Scale 1').id('font-scale-1').onClick(() => { this.app?.setFontSizeScale(1) })
      Button('Scale 2').id('font-scale-2').onClick(() => { this.app?.setFontSizeScale(2) })
    }.padding(24).width('100%')
  }
}

@Entry({ useSharedStorage: true })
@Component
struct SystemFonts {
  @State family: DazzThemeFamily = DazzThemeFamily.Studio
  @LocalBuilder specimen() { NativeFontSpecimen() }
  build() {
    Column({ space: 16 }) {
      Button('Change theme').id('font-theme').onClick(() => {
        this.family = this.family === DazzThemeFamily.Studio ? DazzThemeFamily.Ember : DazzThemeFamily.Studio
      })
      DazzMobileTheme({ family: this.family, content: this.specimen })
    }
  }
}
