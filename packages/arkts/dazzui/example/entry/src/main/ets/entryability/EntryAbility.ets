import { UIAbility, Want, AbilityConstant } from '@kit.AbilityKit'
import { BusinessError } from '@kit.BasicServicesKit'
import { window } from '@kit.ArkUI'
import { hilog } from '@kit.PerformanceAnalysisKit'

interface InspectorChunk { requestId: string; id: string; index: number; total: number; data: string }
interface InspectorFrame { x: number; y: number; width: number; height: number }
interface InspectorGeometry { density: number; frame: InspectorFrame; reference: InspectorFrame }
interface InspectorCellAttributes { accessibilityText: string; tabIndex: number }
interface InspectorCellNode { $attrs: InspectorCellAttributes }
interface InspectorCell { id: string; label: string; tabIndex: number }

export default class EntryAbility extends UIAbility {
  private conformance: boolean = false
  private systemStyleTest: boolean = false
  private modalTest: boolean = false
  private paintTest: boolean = false
  private sceneId: string = 'button/preview'
  private themeName: string = 'bright-light'
  private stage?: window.WindowStage
  private storage: LocalStorage = new LocalStorage()

  configure(want: Want): void {
    this.conformance = want.parameters?.['conformance'] === true
    this.systemStyleTest = want.parameters?.['systemStyleTest'] === true
    this.modalTest = want.parameters?.['modalTest'] === true
    this.paintTest = want.parameters?.['paintTest'] === true
    this.storage.setOrCreate('paintInactive', want.parameters?.['paintInactive'] === true)
    this.storage.setOrCreate('paintOmarchy', want.parameters?.['paintOmarchy'] === true)
    const paintWindowState = want.parameters?.['paintWindowState']
    this.storage.setOrCreate('paintWindowState', typeof paintWindowState === 'string' ? paintWindowState : 'enabled')
    this.storage.setOrCreate('paintDialog', want.parameters?.['paintDialog'] === true)
    this.storage.setOrCreate('dialogOverflow', want.parameters?.['dialogOverflow'] === true)
    this.storage.setOrCreate('paintToast', want.parameters?.['paintToast'] === true)
    this.storage.setOrCreate('paintToastLong', want.parameters?.['paintToastLong'] === true)
    const paintSidebar = want.parameters?.['paintSidebar']
    this.storage.setOrCreate('paintSidebar', typeof paintSidebar === 'string' ? paintSidebar : '')
    const paintPopup = want.parameters?.['paintPopup']
    this.storage.setOrCreate('paintPopup', typeof paintPopup === 'string' ? paintPopup : '')
    const runId = want.parameters?.['runId']
    this.storage.setOrCreate('runId', typeof runId === 'string' ? runId : '')
    this.storage.setOrCreate('sizing', want.parameters?.['sizing'] === 'system' ? 'system' : 'design')
    const page = want.parameters?.['page']
    this.storage.setOrCreate('page', typeof page === 'string' ? page : '')
    const scene = want.parameters?.['sceneId']
    const theme = want.parameters?.['themeName']
    this.sceneId = typeof scene === 'string' ? scene : 'button/preview'
    this.themeName = typeof theme === 'string' ? theme : 'bright-light'
    this.storage.setOrCreate('sceneId', this.sceneId)
    this.storage.setOrCreate('themeName', this.themeName)
  }
  onCreate(want: Want, _launch: AbilityConstant.LaunchParam): void {
    this.configure(want)
  }
  onNewWant(want: Want, _launch: AbilityConstant.LaunchParam): void {
    // Read existing attributes without rebuilding the gallery's live scenes.
    if (want.parameters?.['inspectOnly'] === true) { this.inspect(want); return }
    this.configure(want)
    if (this.stage) this.load(this.stage)
  }
  inspect(want: Want): void {
    const id = want.parameters?.['inspectorId']
    const requestId = want.parameters?.['requestId']
    const keys = want.parameters?.['inspectorKeys']
    if (typeof id !== 'string' || typeof requestId !== 'string') return
    let result: string = ''
    try {
      if (!this.stage) throw new Error('Inspector requires a loaded gallery window')
      const ui = this.stage.getMainWindowSync().getUIContext()
      ui.runScopedTask(() => {
        if (typeof keys === 'string') {
          const cells: InspectorCell[] = keys.split(',').map((key: string): InspectorCell => {
            const node = JSON.parse(getInspectorByKey(key)) as InspectorCellNode
            return { id: key, label: node.$attrs.accessibilityText, tabIndex: node.$attrs.tabIndex }
          })
          result = JSON.stringify(cells)
          return
        }
        result = getInspectorByKey(id)
        const node = ui.getAttachedFrameNodeById(id)
        const reference = ui.getAttachedFrameNodeById('document-column')
        if (node && reference && result.endsWith('}')) {
          const position = node.getPositionToWindow()
          const size = node.getMeasuredSize()
          const referencePosition = reference.getPositionToWindow()
          const referenceSize = reference.getMeasuredSize()
          const geometry: InspectorGeometry = { density: ui.vp2px(1),
            frame: { x: ui.vp2px(position.x), y: ui.vp2px(position.y), width: size.width, height: size.height },
            reference: { x: ui.vp2px(referencePosition.x), y: ui.vp2px(referencePosition.y), width: referenceSize.width, height: referenceSize.height } }
          const paint = node.getCustomProperty('dazzui.paint')
          result = result.slice(0, -1) + ',"$geometry":' + JSON.stringify(geometry) +
            (typeof paint === 'string' ? ',"$paint":' + paint : '') + '}'
        }
      })
    } catch (error) { result = JSON.stringify({ 'error': String(error) } as Record<string, string>) }
    // Test-only inspector output may exceed one log message. Reassemble
    // independently identified chunks without dropping any attributes.
    const total = Math.max(1, Math.ceil(result.length / 512))
    for (let index = 0; index < total; index++) {
      const chunk: InspectorChunk = { requestId, id, index, total, data: result.slice(index * 512, (index + 1) * 512) }
      hilog.info(0x0000, 'DazzUIInspector', '%{public}s', JSON.stringify(chunk))
    }
  }
  load(stage: window.WindowStage): void {
    stage.loadContent(
      this.paintTest ? 'pages/PaintVerification' : this.modalTest ? 'pages/ModalVerification' : this.systemStyleTest ? 'pages/SystemStyle' : this.conformance ? 'pages/Conformance' : 'pages/Index',
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
