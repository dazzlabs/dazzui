import { UIAbility, Want, AbilityConstant } from '@kit.AbilityKit'
import { BusinessError } from '@kit.BasicServicesKit'
import { window } from '@kit.ArkUI'
import { DAZZ_THEME_FAMILIES, DazzThemeFamily } from '@dazzlabs/dazzui-mobile'

export default class EntryAbility extends UIAbility {
  private storage: LocalStorage = new LocalStorage()
  private stage?: window.WindowStage
  private verification: boolean = false

  configure(want: Want): void {
    const family = want.parameters?.['family']
    const selected = DAZZ_THEME_FAMILIES.find(value => value === family)
    const appearance = want.parameters?.['appearance']
    this.storage.setOrCreate('family', selected ?? DazzThemeFamily.Studio)
    this.storage.setOrCreate('appearance', appearance === 'dark' ? ThemeColorMode.DARK :
      appearance === 'light' ? ThemeColorMode.LIGHT : ThemeColorMode.SYSTEM)
    this.verification = want.parameters?.['verification'] === true
    const runId = want.parameters?.['runId']
    this.storage.setOrCreate('runId', typeof runId === 'string' ? runId : '')
  }
  onCreate(want: Want, _launch: AbilityConstant.LaunchParam): void {
    this.configure(want)
  }
  onNewWant(want: Want, _launch: AbilityConstant.LaunchParam): void {
    this.configure(want)
    if (this.stage) this.load(this.stage)
  }
  load(stage: window.WindowStage): void {
    stage.loadContent(this.verification ? 'pages/Verification' : 'pages/Index', this.storage,
      (error: BusinessError) => {
        if (error.code) console.error('Cannot load DazzUI Mobile: ' + error.message)
      })
  }
  onWindowStageCreate(stage: window.WindowStage): void {
    this.stage = stage
    this.load(stage)
  }
  onWindowStageDestroy(): void {
    this.stage = undefined
  }
}
