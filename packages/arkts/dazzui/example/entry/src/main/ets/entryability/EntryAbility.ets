import { UIAbility, Want, AbilityConstant } from '@kit.AbilityKit'
import { BusinessError } from '@kit.BasicServicesKit'
import { window } from '@kit.ArkUI'

export default class EntryAbility extends UIAbility {
  private conformance: boolean = false
  private sceneId: string = 'button/preview'
  private themeName: string = 'studio-light'
  private stage?: window.WindowStage
  private storage: LocalStorage = new LocalStorage()

  configure(want: Want): void {
    this.conformance = want.parameters?.['conformance'] === true
    const scene = want.parameters?.['sceneId']
    const theme = want.parameters?.['themeName']
    this.sceneId = typeof scene === 'string' ? scene : 'button/preview'
    this.themeName = typeof theme === 'string' ? theme : 'studio-light'
    this.storage.setOrCreate('sceneId', this.sceneId)
    this.storage.setOrCreate('themeName', this.themeName)
  }
  onCreate(want: Want, _launch: AbilityConstant.LaunchParam): void {
    this.configure(want)
  }
  onNewWant(want: Want, _launch: AbilityConstant.LaunchParam): void {
    this.configure(want)
    if (this.stage) this.load(this.stage)
  }
  load(stage: window.WindowStage): void {
    stage.loadContent(
      this.conformance ? 'pages/Conformance' : 'pages/Index',
      this.storage,
      (error: BusinessError) => {
        if (error.code)
          console.error('Cannot load DazzUI gallery: ' + error.message)
      }
    )
  }
  onWindowStageCreate(stage: window.WindowStage): void {
    // Keep the real PC titlebar, resize handles and window lifecycle native.
    this.stage = stage
    this.load(stage)
  }
  onWindowStageDestroy(): void {
    this.stage = undefined
  }
}
