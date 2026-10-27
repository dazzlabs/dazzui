import { DazzActivationContext } from '../foundation/Activation'
import { focusID, rememberFocus, forgetFocus } from '../foundation/FocusData'
import { common } from '@kit.AbilityKit'
import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'
import { DazzIcon } from './Icon'

@Component
export struct DazzLink {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.prose') prose: string = 'bodyMedium'
  @Consume('dazzui.activation') activation: DazzActivationContext = new DazzActivationContext()
  @Prop label: string = ''
  @Prop href: string = ''
  @Prop external: boolean = false
  @Prop underline: string = 'hover'
  @Prop tint: DazzTint = 'primary'
  @Prop disabled: boolean = false
  @State hovered: boolean = false
  @State focused: boolean = false
  private nativeFocusID: string = focusID('link')
  aboutToDisappear(): void { forgetFocus(this.getUIContext().getWindowName() ?? '', this.nativeFocusID) }
  onNavigate: ((href: string) => void) | undefined = undefined
  handleKey(event: KeyEvent): boolean {
    if (event.type === KeyType.Down) {
      this.getUIContext().getFocusController().activate(true)
      this.focused = true
    }
    if (event.keyCode !== 2054 && event.keyCode !== 2050) return false
    // ArkUI's generic Text activation also synthesizes clicks for Space.
    // Own both phases before that default so Enter follows the link once.
    if (event.keyCode === 2054 && event.type === KeyType.Down) this.activate()
    event.stopPropagation()
    return true
  }
  async activate(): Promise<void> {
    if (this.disabled) return
    this.activation.activate()
    if (this.onNavigate) { this.onNavigate(this.href); return }
    if (!this.href || this.href.startsWith('#')) return
    const context = this.getUIContext().getHostContext() as common.UIAbilityContext
    try { await context.startAbility({ action: 'ohos.want.action.viewData', uri: this.href }) } catch (_) {}
  }
  build() {
    Row({ space: this.tokens.metric('spacing.0.5') }) {
      Text(this.label).fontSize(this.tokens.fontSize(this.tokens.type(this.prose).size))
        .focusable(!this.disabled).focusOnTouch(true)
        .id(this.nativeFocusID)
        .onFocus(() => rememberFocus(this.getUIContext().getWindowName() ?? '', this.nativeFocusID))
        .onBlur(() => forgetFocus(this.getUIContext().getWindowName() ?? '', this.nativeFocusID))
        .onKeyPreIme((event: KeyEvent): boolean => this.handleKey(event))
        .lineHeight(this.tokens.fontSize(this.tokens.type(this.prose).leading))
        .fontColor(this.tokens.paint('plain', 'content', this.disabled ? 'disabled' : this.hovered ? 'hovered' : 'normal', this.tint))
        .decoration({ type: this.underline === 'always' || (this.underline === 'hover' && this.hovered) ? TextDecorationType.Underline : TextDecorationType.None })
      if (this.external) {
        DazzIcon({ name: 'ArrowUpRight12Regular', iconSize: this.tokens.metric('icon.small'), color: this.tokens.paint('plain', 'content', 'normal', this.tint) })
      }
    }.focusable(!this.disabled).enabled(!this.disabled)
      .accessibilityRole(AccessibilityRoleType.HYPERLINK).accessibilityText(this.label)
      .outline({ width: this.focused ? this.tokens.metric('focus.width') : 0, color: this.tokens.focus(), radius: this.tokens.metric('radius.tiny') })
      .onFocus(() => { this.focused = this.getUIContext().getFocusController().isActive() }).onBlur(() => { this.focused = false })
      .onHover((value: boolean) => { this.hovered = value }).onClick(() => this.activate())
  }
}
