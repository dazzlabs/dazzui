import { UIContext, LengthMetrics, ColorMetrics, FrameNode } from '@kit.ArkUI'
import { deviceInfo } from '@kit.BasicServicesKit'
import { DazzTokens } from '../foundation/Tokens'
import { DazzIcon } from './Icon'

/** A Menu and a Select share popup.css's columns and intrinsic row width. */
export interface DazzMenuRowData {
  label: string
  icon?: string
  shortcut?: string
  checked?: boolean
  disabled?: boolean
}

/** API 26 exposes the target gap; older runtimes keep native placement. */
export function menuAnchorOptions(tokens: DazzTokens, options: MenuOptions): MenuOptions {
  if (deviceInfo.sdkApiVersion >= 26) options.targetSpace = LengthMetrics.vp(tokens.metric('spacing.1'))
  return options
}

export function menuRowHeight(tokens: DazzTokens, item: DazzMenuRowData): number {
  return Math.max(tokens.type('labelQuiet').size,
    item.checked === undefined ? 0 : tokens.metric('icon.small'),
    item.icon ? tokens.metric('icon.medium') : 0,
    item.shortcut ? tokens.type('labelSmall').size : 0) +
    2 * tokens.metric('menu.item-padding')
}

export function menuWidth(ui: UIContext, tokens: DazzTokens,
  items: DazzMenuRowData[], minimum: number | undefined = undefined): number {
  const gap = tokens.metric('spacing.2')
  // Select's anchor minimum replaces the generic popup minimum in CSS.
  let width = minimum ?? tokens.metric('menu.min-width')
  for (const item of items) {
    const label = tokens.type('labelQuiet')
    const hint = tokens.type('labelSmall')
    const measure = (text: string, size: number, weight: number): number => {
      const result = ui.getMeasureUtils().measureTextSize({ textContent: text,
        fontSize: tokens.fontSize(size), fontWeight: weight })
      return typeof result.width === 'number' ? ui.px2vp(result.width) : 0
    }
    const columns = measure(item.label, label.size, label.weight) +
      (item.checked === undefined ? 0 : tokens.metric('icon.small') + gap) +
      (item.icon ? tokens.metric('icon.medium') + gap : 0) +
      (item.shortcut ? measure(item.shortcut, hint.size, hint.weight) + gap : 0)
    width = Math.max(width, columns + 2 * gap +
      2 * (tokens.metric('spacing.1') + tokens.metric('stroke.hairline')))
  }
  return width
}

let menuRowSequence: number = 0

/** A custom drawing inside the native MenuItem, retaining its focus/activation. */
@Component
export struct DazzMenuRow {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop icon: string = ''
  @Prop shortcut: string = ''
  @Prop checked: boolean | undefined = undefined
  @Prop disabled: boolean = false
  @Prop itemID: string = ''
  @State focused: boolean = false
  onSelect: () => void = () => {}
  onHighlight: (node: FrameNode | null) => void = () => {}
  onMenuKey: (event: KeyEvent) => boolean = () => false
  private ownID: string = 'dazzui-menu-row-' + String(++menuRowSequence)
  rowID(): string { return this.itemID || this.ownID }
  color(): string {
    return this.disabled ? this.tokens.paint('normal', 'content', 'disabled', 'neutral') : this.tokens.color('content')
  }
  @LocalBuilder content() {
    Row({ space: this.tokens.metric('spacing.2') }) {
      if (this.checked !== undefined) {
        DazzIcon({ name: 'Checkmark12Filled', iconSize: this.tokens.metric('icon.small'), color: this.color() })
          .visibility(this.checked ? Visibility.Visible : Visibility.Hidden)
          .flexShrink(0)
          .id(this.rowID() + '-check')
      }
      if (this.icon) {
        DazzIcon({ name: this.icon, iconSize: this.tokens.metric('icon.medium'),
          color: this.tokens.color('content-muted') }).flexShrink(0)
      }
      Text(this.label).fontSize(this.tokens.fontSize(this.tokens.type('labelQuiet').size))
        .fontWeight(this.tokens.type('labelQuiet').weight)
        .lineHeight(this.tokens.fontSize(this.tokens.type('labelQuiet').size))
        .fontColor(this.color()).maxLines(1)
        .textOverflow({ overflow: TextOverflow.Ellipsis }).layoutWeight(1)
        .id(this.rowID() + '-label')
      if (this.shortcut) {
        Text(this.shortcut).fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
          .fontWeight(this.tokens.type('labelSmall').weight)
          .lineHeight(this.tokens.fontSize(this.tokens.type('labelSmall').size))
          .fontColor(this.tokens.color('content-faint')).maxLines(1).flexShrink(0)
          .id(this.rowID() + '-shortcut')
      }
    }.width('100%').height('100%')
      .padding({ left: this.tokens.metric('spacing.2'), right: this.tokens.metric('spacing.2'),
        top: this.tokens.metric('menu.item-padding'), bottom: this.tokens.metric('menu.item-padding') })
      .hitTestBehavior(HitTestMode.None)
  }
  build() {
    MenuItem(this.content).id(this.rowID())
      .selected(this.checked ?? false).selectIcon(false)
      .height(menuRowHeight(this.tokens, { label: this.label, icon: this.icon,
        shortcut: this.shortcut, checked: this.checked }))
      .width('100%').padding(0).margin(0)
      .borderRadius(Math.max(0, this.tokens.metric('radius.large') - this.tokens.metric('spacing.1')))
      .backgroundColor(!this.disabled && this.focused
        ? this.tokens.paint('normal', 'surface', 'hovered', 'neutral') : Color.Transparent)
      .shadow({ radius: 0, color: Color.Transparent })
      .enabled(!this.disabled)
      .focusBox({ strokeWidth: LengthMetrics.vp(0), strokeColor: ColorMetrics.resourceColor(Color.Transparent) })
      .accessibilityText(this.label).accessibilityDescription(this.shortcut)
      .onMouse((event: MouseEvent) => {
        // Scrolling can move a different row under a stationary cursor. Only
        // a real move transfers keyboard focus to the pointer's destination.
        if (!this.disabled && event.action === MouseAction.Move)
          this.getUIContext().getFocusController().requestFocus(this.rowID())
      })
      .onFocus(() => {
        this.focused = true
        this.onHighlight(this.getUIContext().getFrameNodeByUniqueId(this.getUniqueId()))
      }).onBlur(() => { this.focused = false })
      .onKeyEvent((event: KeyEvent): boolean => this.onMenuKey(event))
      .onClick(() => { if (!this.disabled) this.onSelect() })
  }
}
