import { DazzTokens } from './Tokens'
import { common } from '@kit.AbilityKit'
import { readSystemStyle } from './SystemStyle'
import { DazzSizing, DazzSystemStyle } from './SystemStyleData'
import { DazzPointerContext } from './Pointer'
import { DazzEditingContext } from './Editing'
import { focusID, rememberFocusScope, forgetFocusScope } from './FocusData'
import { dispatchSidebarKey } from './SidebarKeys'
import { hasWindowSidebars, toggleWindowSidebars, observeSidebarShortcuts, unobserveSidebarShortcuts } from './SidebarData'

@Component
export struct DazzTheme {
  @Prop @Watch('themeChanged') name: string = 'bright-light'
  @Prop @Watch('themeChanged') sizing: DazzSizing = 'design'
  @Provide({ allowOverride: 'dazzui.theme' }) tokens: DazzTokens = new DazzTokens()
  @Provide({ allowOverride: 'dazzui.pointer' }) pointer: DazzPointerContext = new DazzPointerContext()
  @Provide({ allowOverride: 'dazzui.editing' }) editing: DazzEditingContext = new DazzEditingContext()
  @BuilderParam content: () => void = this.empty
  private app?: common.ApplicationContext
  private environmentId: number = -1
  private pending: number = -1
  private systemStyle?: DazzSystemStyle
  private scopeID: string = focusID('theme')
  @State hasSidebarShortcuts: boolean = false
  private sidebarChanged: () => void = () => {}

  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.app = this.getUIContext().getHostContext()?.getApplicationContext()
    this.environmentId = this.app?.on('environment', {
      onConfigurationUpdated: () => {
        clearTimeout(this.pending)
        this.pending = setTimeout(() => this.themeChanged(), 0)
      },
      onMemoryLevel: () => {}
    }) ?? -1
    this.themeChanged()
    this.sidebarChanged = () => {
      this.hasSidebarShortcuts = hasWindowSidebars(this.getUIContext().getWindowName() ?? '')
    }
    observeSidebarShortcuts(this.sidebarChanged)
    this.sidebarChanged()
  }
  aboutToDisappear(): void {
    forgetFocusScope(this.getUIContext().getWindowName() ?? '', this.scopeID)
    unobserveSidebarShortcuts(this.sidebarChanged)
    clearTimeout(this.pending)
    if (this.environmentId !== -1) this.app?.off('environment', this.environmentId, () => {})
    this.environmentId = -1
  }
  themeChanged(): void {
    this.systemStyle = this.sizing === 'system' ? readSystemStyle(this.getUIContext()) : undefined
    this.tokens = new DazzTokens(this.name, this.systemStyle)
  }

  build() {
    Column() {
      this.content()
    }
      .alignItems(HorizontalAlign.Start)
      .backgroundColor(this.tokens.color('surface'))
      .width('100%')
      .height('100%')
      .id(this.scopeID)
      .onFocus(() => rememberFocusScope(this.getUIContext().getWindowName() ?? '', this.scopeID))
      .onBlur(() => forgetFocusScope(this.getUIContext().getWindowName() ?? '', this.scopeID))
      // Native shortcuts run after a child's onKeyPreIme, so a recorder can
      // capture the chord, and before TextInput's own editing commands.
      .keyboardShortcut(this.hasSidebarShortcuts ? 'b' : '', [ModifierKey.CTRL],
        () => { toggleWindowSidebars(this.getUIContext().getWindowName() ?? '') })
      .onKeyEvent((event: KeyEvent): boolean =>
        dispatchSidebarKey(this.getUIContext().getWindowName() ?? '', event))
      .onTouch((event: TouchEvent) => {
        if (event.type === TouchType.Down && event.touches.length > 0) {
          this.pointer.press(event.touches[0].windowX, event.touches[0].windowY, event.timestamp)
        }
        if (event.type === TouchType.Up || event.type === TouchType.Cancel) this.pointer.release()
      })
      .onMouse((event: MouseEvent) => {
        if (event.action === MouseAction.Move) this.pointer.move(event.windowX, event.windowY, event.timestamp)
        if (event.action === MouseAction.Press) {
          this.pointer.press(event.windowX, event.windowY, event.timestamp)
        }
        if (event.action === MouseAction.Release) this.pointer.release()
      })
      .onHover((hovered: boolean) => {
        if (!hovered) this.pointer.move(Number.NEGATIVE_INFINITY, Number.NEGATIVE_INFINITY)
      })
  }
}
