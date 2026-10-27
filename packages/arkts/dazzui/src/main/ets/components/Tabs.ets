import { ColorMetrics, LengthMetrics, curves, FrameCallback } from '@kit.ArkUI'
import { DazzTokens } from '../foundation/Tokens'
import { shadowOptions } from '../foundation/Shadow'
import { DazzTint, DazzSize, DazzItem } from '../foundation/TokenTypes'
import { DazzIcon } from './Icon'
import { nextTabFocus } from '../foundation/Selection'
import { DazzTabItem, DazzTabsFit, DazzTabsPulse, fitTabs, tabsOrder, tabsNavigate } from '../foundation/TabsData'
import { DazzState } from '../foundation/TokenTypes'
import { DazzActivationContext } from '../foundation/Activation'
import { DazzMenu, DazzMenuItem } from './Menu'
import { DazzTooltip } from './Tooltip'
import { rememberFocus, forgetFocus, focusTarget } from '../foundation/FocusData'
import { DazzPointerContext } from '../foundation/Pointer'

let tabScopeID: number = 0

class TabsPulseFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  onIdle(_timeLeft: number): void { this.action() }
}

/** Native button focus scopes keep keyboard traversal and selection separate. */
@Component
export struct DazzSegmentedControl {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop items: DazzItem[] = []
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop controlSize: DazzSize = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop activeStyle: string = 'raised'
  @Prop iconOnly: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: string = ''
  @State hoveredValue: string = ''
  private focusedIndex: number = 0
  private focusScope: string = 'dazz-tabs-' + String(++tabScopeID)
  onValueChange: (value: string) => void = () => {}
  aboutToAppear(): void {
    this.localValue = this.defaultValue || this.items.find((item: DazzItem) => !item.disabled)?.value || ''
  }
  selected(item: DazzItem): boolean { return item.value === (this.value ?? this.localValue) }
  radius(): number {
    return this.controlSize === 'medium' ? this.tokens.metric('radius.medium') : this.tokens.size(this.controlSize, 'radius')
  }
  ink(item: DazzItem): string {
    return this.selected(item) && this.activeStyle === 'filled'
      ? this.tokens.paint('filled', 'content', 'normal', this.tint)
      : this.tokens.color(this.selected(item) || (this.hoveredValue === item.value && !item.disabled) ? 'content' : 'content-secondary')
  }
  glyphSize(): number {
    return this.tokens.size(this.controlSize, 'icon-size') -
      (this.iconOnly ? 0 : this.tokens.metric('control.label-glyph.shrink'))
  }
  itemWidth(item: DazzItem): number {
    const label = this.iconOnly && item.icon ? 0 : this.getUIContext().px2vp(Number(
      this.getUIContext().getMeasureUtils().measureTextSize({
        textContent: item.label,
        fontSize: this.tokens.fontSize(this.tokens.label(this.controlSize).size),
        fontWeight: this.tokens.type(this.selected(item) ? 'labelMedium' : 'labelQuiet').weight,
      }).width
    ))
    return label + 2 * this.tokens.metric('spacing.3') +
      (item.icon ? this.glyphSize() + (this.iconOnly ? 0 : this.tokens.size(this.controlSize, 'gap') - this.tokens.metric('control.label-glyph.inset')) : 0)
  }
  innerWidth(): number { return this.items.reduce((width: number, item: DazzItem) => width + this.itemWidth(item), 0) }
  elevation(item: DazzItem): ShadowOptions {
    const value = this.selected(item) ? this.tokens.shadow('shadow.2xs')[0] : undefined
    return shadowOptions(value, this.getUIContext().vp2px(1))
  }
  activate(index: number): void {
    const item = this.items[index]
    if (!item || item.disabled) return
    this.focusedIndex = index
    this.getUIContext().getFocusController().requestFocus(this.focusScope + '-' + String(index))
    if (this.selected(item)) return
    if (this.value === undefined) this.localValue = item.value
    this.onValueChange(item.value)
  }
  handleKey(event: KeyEvent): void {
    // ArkUI treats a Button with a custom label as a focus scope. Confirm
    // its choice here, while native buttons still own pointer activation.
    const enter = event.keyCode === 2054 || event.keyCode === 2119
    const space = event.keyCode === 2050
    if (enter || space) {
      this.getUIContext().getFocusController().activate(true)
      if ((enter && event.type === KeyType.Down) || (space && event.type === KeyType.Up)) this.activate(this.focusedIndex)
      event.stopPropagation()
      return
    }
    if (event.type !== KeyType.Down) return
    const target = nextTabFocus(this.items, this.focusedIndex, event.keyCode)
    if (target < 0) return
    this.getUIContext().getFocusController().activate(true)
    this.focusedIndex = target
    this.getUIContext().getFocusController().requestFocus(this.focusScope + '-' + String(target))
    event.stopPropagation()
  }
  @LocalBuilder tab(item: DazzItem, index: number) {
    Button({ type: ButtonType.Normal }) {
      Row({ space: this.tokens.size(this.controlSize, 'gap') }) {
        if (item.icon) { DazzIcon({ name: item.icon, iconSize: this.glyphSize(), color: this.ink(item) }) }
        if (!this.iconOnly || !item.icon) {
          Text(item.label)
            .fontSize(this.tokens.fontSize(this.tokens.label(this.controlSize).size))
            .fontWeight(this.tokens.type(this.selected(item) ? 'labelMedium' : 'labelQuiet').weight)
            .fontColor(this.ink(item))
        }
      }.justifyContent(FlexAlign.Center)
    }.padding({ left: this.tokens.metric('spacing.3') - (item.icon && !this.iconOnly ? this.tokens.metric('control.label-glyph.inset') : 0), right: this.tokens.metric('spacing.3'), top: 0, bottom: 0 })
      .width(this.itemWidth(item))
      .height(this.tokens.size(this.controlSize) - 2 * this.tokens.metric('segmented-control.inset'))
      .borderRadius(Math.max(0, this.radius() - this.tokens.metric('segmented-control.inset')))
      .backgroundColor(this.selected(item) ? this.activeStyle === 'filled'
        ? this.tokens.paint('filled', 'surface', 'normal', this.tint)
        : this.tokens.color('surface-raised') : Color.Transparent)
      .shadow(this.elevation(item))
      .stateEffect(false)
      .focusBox({ margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
        strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')),
        strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
      // Retain the native implicit focus scope for this custom label.
      .focusOnTouch(true)
      .id(this.focusScope + '-' + String(index)).enabled(!item.disabled)
      .focusScopePriority(this.focusScope, this.selected(item) ? FocusPriority.PREVIOUS : FocusPriority.AUTO)
      .onFocus(() => { this.focusedIndex = index })
      .onClick(() => this.activate(index))
      .onHover((hovered: boolean) => { this.hoveredValue = hovered ? item.value : '' })
      .opacity(item.disabled ? 0.5 : 1).accessibilityText(item.label)
      .accessibilitySelected(this.selected(item))
  }
  build() {
    Row() {
      ForEach(this.items, (item: DazzItem, index: number) => {
        this.tab(item, index)
      }, (item: DazzItem) => item.value)
    }.padding(this.tokens.metric('segmented-control.inset'))
      .width(this.innerWidth() + 2 * this.tokens.metric('segmented-control.inset'))
      .height(this.tokens.size(this.controlSize))
      .backgroundColor(this.tokens.color('surface-inset')).borderRadius(this.radius())
      .id(this.focusScope + '-group').focusScopeId(this.focusScope, true, false)
      .accessibilityText(this.accessibilityLabel)
      .onKeyEvent((event: KeyEvent) => this.handleKey(event))
  }
}

@Component
export struct DazzPillTabs {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop items: DazzItem[] = []
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop tint: DazzTint = 'primary'
  @Prop accessibilityLabel: string = ''
  @State localValue: string = ''
  @State hoveredValue: string = ''
  private focusedIndex: number = 0
  private focusScope: string = 'dazz-tabs-' + String(++tabScopeID)
  onValueChange: (value: string) => void = () => {}
  aboutToAppear(): void { this.localValue = this.defaultValue || this.items.find((item: DazzItem) => !item.disabled)?.value || '' }
  selected(item: DazzItem): boolean { return item.value === (this.value ?? this.localValue) }
  @LocalBuilder label(value: string, item: DazzItem) {
    Text(value).fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
      .fontWeight(this.tokens.type(this.selected(item) ? 'labelStrong' : 'labelSmall').weight)
      .fontColor(this.selected(item) ? this.tokens.paint('filled', 'content', 'normal', this.tint) : this.tokens.color(this.hoveredValue === item.value && !item.disabled ? 'content' : 'content-nav'))
  }
  widthForTabs(): number {
    return this.items.reduce((width: number, item: DazzItem) => {
      const measure = (value: string): number => this.getUIContext().px2vp(Number(
        this.getUIContext().getMeasureUtils().measureTextSize({ textContent: value,
          fontSize: this.tokens.fontSize(this.tokens.type('labelSmall').size),
          fontWeight: this.tokens.type(this.selected(item) ? 'labelStrong' : 'labelSmall').weight }).width))
      return width + measure(item.label) + 2 * this.tokens.metric('spacing.3') +
        (item.count === undefined ? 0 : measure(String(item.count)) + this.tokens.size('tiny', 'gap'))
    }, 0) + Math.max(0, this.items.length - 1) * this.tokens.metric('spacing.2')
  }
  activate(index: number): void {
    const item = this.items[index]
    if (!item || item.disabled) return
    this.focusedIndex = index
    this.getUIContext().getFocusController().requestFocus(this.focusScope + '-' + String(index))
    if (this.selected(item)) return
    if (this.value === undefined) this.localValue = item.value
    this.onValueChange(item.value)
  }
  handleKey(event: KeyEvent): void {
    // ArkUI treats a Button with a custom label as a focus scope. Confirm
    // its choice here, while native buttons still own pointer activation.
    const enter = event.keyCode === 2054 || event.keyCode === 2119
    const space = event.keyCode === 2050
    if (enter || space) {
      this.getUIContext().getFocusController().activate(true)
      if ((enter && event.type === KeyType.Down) || (space && event.type === KeyType.Up)) this.activate(this.focusedIndex)
      event.stopPropagation()
      return
    }
    if (event.type !== KeyType.Down) return
    const target = nextTabFocus(this.items, this.focusedIndex, event.keyCode)
    if (target < 0) return
    this.getUIContext().getFocusController().activate(true)
    this.focusedIndex = target
    this.getUIContext().getFocusController().requestFocus(this.focusScope + '-' + String(target))
    event.stopPropagation()
  }
  @LocalBuilder tab(item: DazzItem, index: number) {
    Button({ type: ButtonType.Normal }) {
      Row({ space: this.tokens.size('tiny', 'gap') }) {
        this.label(item.label, item)
        if (item.count !== undefined) { this.label(String(item.count), item) }
      }.height(this.tokens.size('tiny'))
    }.height(this.tokens.size('tiny'))
      .padding({ left: this.tokens.metric('spacing.3'), right: this.tokens.metric('spacing.3'), top: 0, bottom: 0 })
      .backgroundColor(this.selected(item) ? this.tokens.paint('filled', 'surface', 'normal', this.tint) : this.tokens.color(this.hoveredValue === item.value && !item.disabled ? 'surface-sunken' : 'surface-inset'))
      .opacity(item.disabled ? 0.5 : 1)
      .accessibilitySelected(this.selected(item))
      .stateEffect(false)
      .focusBox({ margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
        strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')),
        strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
      // Retain the native implicit focus scope for this custom label.
      .focusOnTouch(true)
      .id(this.focusScope + '-' + String(index)).enabled(!item.disabled)
      .focusScopePriority(this.focusScope, this.selected(item) ? FocusPriority.PREVIOUS : FocusPriority.AUTO)
      .onFocus(() => { this.focusedIndex = index })
      .onClick(() => this.activate(index))
      .onHover((hovered: boolean) => { this.hoveredValue = hovered ? item.value : '' })
      .borderRadius(this.tokens.metric('radius.full'))
  }
  build() {
    Row({ space: this.tokens.metric('spacing.2') }) {
      ForEach(this.items, (item: DazzItem, index: number) => {
        this.tab(item, index)
      }, (item: DazzItem) => item.value)
    }.width(this.widthForTabs()).constraintSize({ maxWidth: '100%' })
      .height(this.tokens.size('tiny')).accessibilityRole(AccessibilityRoleType.TAB_BAR)
      .id(this.focusScope + '-group').focusScopeId(this.focusScope, true, false)
      .accessibilityText(this.accessibilityLabel)
      .onKeyEvent((event: KeyEvent) => this.handleKey(event))
  }
}

@Component
struct NavigationStatus {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop status: string = 'neutral'
  @Prop @Watch('sync') busy: boolean = false
  @Prop controlID: string = ''
  @State dimmed: boolean = false
  private alive: boolean = false
  private pulse: DazzTabsPulse = new DazzTabsPulse()
  aboutToAppear(): void {
    this.alive = true
    this.pulse.animator = {
      reset: () => this.getUIContext().animateTo({ duration: 0 }, () => { this.dimmed = false }),
      half: (dimmed: boolean, finished: () => void) => {
        this.getUIContext().animateTo({ duration: this.tokens.metric('tabs.pulse-duration') / 2,
          curve: curves.cubicBezierCurve(this.tokens.metric('motion.easing.x1'), this.tokens.metric('motion.easing.y1'), this.tokens.metric('motion.easing.x2'), this.tokens.metric('motion.easing.y2')),
          iterations: 1, playMode: PlayMode.Normal, onFinish: finished }, () => { this.dimmed = dimmed })
      },
    }
    this.getUIContext().postFrameCallback(new TabsPulseFrame(() => this.sync()))
  }
  aboutToDisappear(): void { this.alive = false; this.pulse.dispose() }
  sync(): void {
    if (!this.alive) return
    this.pulse.start(this.busy)
  }
  color(): string {
    return this.status === 'neutral' ? this.tokens.color('border-strong')
      : this.tokens.color(this.status + (this.status === 'warning' ? '.400' : '.500'))
  }
  build() {
    Circle().width(this.tokens.metric('spacing.1.5')).height(this.tokens.metric('spacing.1.5')).fill(this.color())
      .id(this.controlID)
      .opacity(this.dimmed ? this.tokens.metric('tabs.pulse-opacity') : 1)
      .accessibilityLevel('no')
  }
}

@Component
struct NavigationGlyph {
  @Consume('dazzui.theme') tokens: DazzTokens
  @ObjectLink interaction: NavigationGlyphState
  @Prop icon: string = ''
  @Prop iconSize: number = 0
  build() {
    DazzIcon({ name: this.icon, iconSize: this.iconSize,
      color: this.tokens.color(this.interaction.hovered || this.interaction.touched ? 'content' : 'content-subtle') })
  }
}

@Observed
class NavigationGlyphState { hovered: boolean = false; touched: boolean = false }

@Component
struct NavigationGlyphButton {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.activation') activation: DazzActivationContext = new DazzActivationContext()
  @Consume('dazzui.pointer') pointer: DazzPointerContext = new DazzPointerContext()
  @Prop label: string = ''
  @Prop icon: string = ''
  @Prop iconSize: number = 0
  @Prop controlID: string = ''
  @State interaction: NavigationGlyphState = new NavigationGlyphState()
  @State pressed: boolean = false
  @State private mousePressed: boolean = false
  @State private spaceArmed: boolean = false
  onAction: () => void = () => {}
  onFocusChange: (focused: boolean) => void = () => {}
  private mouseReleased: () => void = () => { this.mousePressed = false; this.pointer.offRelease(this.mouseReleased) }
  private resetTouchHover: () => void = () => { this.interaction.touched = false }
  private outsidePress: (x: number, y: number) => void = (x: number, y: number) => {
    if (!this.interaction.touched) return
    const ui = this.getUIContext(), node = ui.getAttachedFrameNodeById(this.controlID)
    if (!node) { this.resetTouchHover(); return }
    const origin = node.getPositionToWindow(), size = node.getMeasuredSize()
    if (x < origin.x || y < origin.y || x >= origin.x + ui.px2vp(size.width) || y >= origin.y + ui.px2vp(size.height)) this.resetTouchHover()
  }
  hovered(): boolean { return this.interaction.hovered || this.interaction.touched }
  state(): DazzState { return this.pressed || this.mousePressed || this.spaceArmed ? 'pressed' : this.hovered() ? 'hovered' : 'normal' }
  aboutToAppear(): void {
    this.pointer.on(this.outsidePress); this.pointer.onMove(this.resetTouchHover); this.pointer.onTouchHoverReset(this.resetTouchHover)
  }
  aboutToDisappear(): void {
    this.mouseReleased()
    this.pointer.off(this.outsidePress); this.pointer.offMove(this.resetTouchHover); this.pointer.offTouchHoverReset(this.resetTouchHover)
    forgetFocus(this.getUIContext().getWindowName() ?? '', this.controlID)
  }
  @LocalBuilder button() {
    Button({ type: ButtonType.Normal }) {
      // Forwarded builders can retain a primitive child prop. Share the live
      // interaction object so the glyph updates with the native button face.
      NavigationGlyph({ icon: this.icon, iconSize: this.iconSize, interaction: this.interaction })
    }.id(this.controlID).width(this.tokens.size('tiny')).height(this.tokens.size('tiny')).padding(0)
      .backgroundColor(this.hovered() ? this.tokens.color('surface-subtle') : this.tokens.paint('plain', 'surface', this.state(), 'neutral'))
      .border({ width: this.tokens.metric('stroke.hairline'), radius: this.tokens.size('tiny', 'radius'), color: this.tokens.paint('plain', 'border', this.state(), 'neutral') })
      .focusBox({ margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
        strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')), strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
      .focusOnTouch(true)
      .stateEffect(false).accessibilityText(this.label)
      .onHover((hovered: boolean) => { this.interaction.hovered = hovered })
      .onTouch((event: TouchEvent) => {
        if (event.source === SourceType.Mouse) {
          if (event.type === TouchType.Up) this.mouseReleased()
          return
        }
        if (event.type === TouchType.Down) this.pressed = true
        else if (event.type === TouchType.Up || event.type === TouchType.Cancel) this.pressed = false
        if (event.type === TouchType.Down && event.source === SourceType.TouchScreen) this.interaction.touched = true
      })
      .onMouse((event: MouseEvent) => {
        if (event.action === MouseAction.Press && event.button === MouseButton.Left) {
          this.mousePressed = true
          this.pointer.onRelease(this.mouseReleased)
          this.getUIContext().getFocusController().requestFocus(this.controlID)
        } else if (event.action === MouseAction.Release && event.button === MouseButton.Left) this.mouseReleased()
      })
      .onFocus(() => { rememberFocus(this.getUIContext().getWindowName() ?? '', this.controlID); this.onFocusChange(true) })
      .onBlur(() => { forgetFocus(this.getUIContext().getWindowName() ?? '', this.controlID); this.pressed = false; this.spaceArmed = false; this.mouseReleased(); this.onFocusChange(false) })
      .onClick(() => { this.onAction(); this.activation.activate() })
      .onKeyEvent((event: KeyEvent): boolean => {
        if (event.keyCode !== 2050) return false
        const armed = this.spaceArmed
        this.spaceArmed = event.type === KeyType.Down
        if (event.type === KeyType.Up && armed) { this.onAction(); this.activation.activate() }
        return true
      })
      .onKeyPreIme((event: KeyEvent): boolean => {
        if (event.type === KeyType.Down && event.keyCode === 2049) this.getUIContext().getFocusController().activate(true)
        return false
      })
  }
  build() { DazzTooltip({ label: this.label, content: this.button }) }
}

@Observed
class NavigationTabRecord {
  item: DazzTabItem
  constructor(item: DazzTabItem) { this.item = item }
}

@Component
struct NavigationTab {
  @Consume('dazzui.theme') tokens: DazzTokens
  @ObjectLink record: NavigationTabRecord
  @Prop selected: boolean = false
  @Prop compact: boolean = false
  @Prop canClose: boolean = false
  @Prop contentHeight: number = 0
  @Prop contentWidth: number = 0
  @Prop focusID: string = ''
  @State hovered: boolean = false
  @State closeFocused: boolean = false
  onSelect: () => void = () => {}
  onClose: () => void = () => {}
  aboutToDisappear(): void { forgetFocus(this.getUIContext().getWindowName() ?? '', this.focusID) }
  ink(): string { return this.tokens.color(this.selected || (this.hovered && !this.record.item.disabled) ? 'content' : 'content-subtle') }
  labelVisible(): boolean { return !this.compact || !this.record.item.icon }
  build() {
    Stack({ alignContent: Alignment.Bottom }) {
      Row() {
        Button({ type: ButtonType.Normal }) {
          Row({ space: this.tokens.metric('spacing.1.5') }) {
            if (this.record.item.status) { NavigationStatus({ status: this.record.item.status, busy: this.record.item.busy ?? false, controlID: this.focusID + '-status' }) }
            if (this.record.item.icon) { DazzIcon({ name: this.record.item.icon, iconSize: this.tokens.metric('tabs.icon-size'), color: this.ink() }) }
            if (this.labelVisible()) {
              Text(this.record.item.label).fontSize(this.tokens.fontSize(this.tokens.type('labelMedium').size))
                .fontWeight(this.tokens.type(this.selected ? 'labelStrong' : 'labelMedium').weight)
                .lineHeight(this.tokens.fontSize(this.tokens.type('labelMedium').size)).fontColor(this.ink()).maxLines(1)
              if (this.record.item.detail) {
                Text(this.record.item.detail).fontSize(this.tokens.fontSize(this.tokens.type('labelMedium').size))
                  .fontWeight(this.tokens.type('labelMedium').weight).fontColor(this.tokens.color('content-subtle')).maxLines(1)
              }
            }
            if (this.record.item.count !== undefined) {
              Text(String(this.record.item.count)).fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
                .fontWeight(this.tokens.type('labelMedium').weight).fontColor(this.tokens.color('content-subtle')).maxLines(1)
            }
          }
        }.height(this.contentHeight).padding({ left: this.tokens.metric('spacing.2.5'), right: this.tokens.metric('spacing.2.5'), top: 0, bottom: 0 })
          .backgroundColor(Color.Transparent).border({ width: 0 }).stateEffect(false)
          .enabled(!this.record.item.disabled).opacity(this.record.item.disabled ? 0.5 : 1)
          .id(this.focusID).tabIndex(this.selected ? 0 : -1)
          .bindTips(this.record.item.title ?? (this.compact ? this.record.item.label : ''))
          .focusOnTouch(true)
          .accessibilityText(this.record.item.label).accessibilityDescription('Tab').accessibilitySelected(this.selected)
          .focusBox({ margin: LengthMetrics.vp(-this.tokens.metric('focus.offset')), strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')), strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
          .onClick(() => this.onSelect())
          .onFocus(() => rememberFocus(this.getUIContext().getWindowName() ?? '', this.focusID))
          .onBlur(() => forgetFocus(this.getUIContext().getWindowName() ?? '', this.focusID))
          .onKeyPreIme((event: KeyEvent): boolean => {
            // Activate before native traversal decides whether this first Tab
            // only reveals focus or also advances to the next control.
            if (event.type === KeyType.Down && event.keyCode === 2049) this.getUIContext().getFocusController().activate(true)
            return false
          })
        if (this.record.item.closable && this.canClose && !this.compact) {
          NavigationGlyphButton({ label: 'Close ' + this.record.item.label, icon: 'DismissRegular', controlID: this.focusID + '-close', iconSize: this.tokens.metric('tabs.close-icon-size'),
            onAction: this.onClose, onFocusChange: (focused: boolean) => { this.closeFocused = focused } })
            .visibility(this.selected || this.hovered || this.closeFocused ? Visibility.Visible : Visibility.Hidden)
            .margin({ left: -this.tokens.metric('spacing.1.5'), right: this.tokens.metric('spacing.1') })
        }
      }.height(this.contentHeight)
      if (this.selected) {
        Row() {}.height(this.tokens.metric('stroke.control')).width(Math.max(0, this.contentWidth - 2 * this.tokens.metric('spacing.2.5')))
          .borderRadius(this.tokens.metric('radius.full')).backgroundColor(this.tokens.paint('filled', 'surface', 'normal', 'primary'))
          .hitTestBehavior(HitTestMode.None)
      }
    }.height(this.contentHeight).width(this.contentWidth)
      .onHover((hovered: boolean) => { this.hovered = hovered })
  }
}

/** Navigation tabs: independent width measurements keep resizing stable. */
@Component
export struct DazzTabs {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide({ allowOverride: 'dazzui.activation' }) activation: DazzActivationContext = new DazzActivationContext()
  @Prop @Watch('syncItems') items: DazzTabItem[] = []
  @Prop value: string | null = null
  @Prop contentHeight: number | undefined = undefined
  @Prop accessibilityLabel: string = ''
  @BuilderParam trailing?: () => void
  onValueChange: (value: string) => void = () => {}
  onClose: ((value: string) => void) | undefined = undefined
  @State available: number = -1
  @State tailWidth: number = 0
  @State private records: NavigationTabRecord[] = []
  aboutToAppear(): void { this.syncItems() }
  syncItems(): void {
    const focused = focusTarget(this.getUIContext().getWindowName() ?? '')
    const removedFocus = this.records.some((record: NavigationTabRecord): boolean =>
      !this.items.some((item: DazzTabItem): boolean => item.value === record.item.value) &&
      (focused === this.owner + '-' + record.item.value || focused === this.owner + '-' + record.item.value + '-close'))
    // Match DOM removal: drop the removed control's focus before ArkUI chooses
    // an unrelated surviving tab. Refused controlled removal keeps its focus.
    if (removedFocus) this.getUIContext().getFocusController().clearFocus()
    const next: NavigationTabRecord[] = []
    for (const item of this.items) {
      const record = this.records.find((record: NavigationTabRecord): boolean => record.item.value === item.value)
      if (record) { record.item = item; next.push(record) }
      else next.push(new NavigationTabRecord(item))
    }
    this.records = next
  }
  visibleRecords(): NavigationTabRecord[] {
    const shown: NavigationTabRecord[] = []
    for (const item of this.visible()) {
      const record = this.records.find((record: NavigationTabRecord): boolean => record.item.value === item.value)
      if (record) shown.push(record)
    }
    return shown
  }
  private owner: string = 'dazz-navigation-tabs-' + String(++tabScopeID)
  textWidth(label: string, role: string, strong: boolean = false): number {
    const type = this.tokens.type(role)
    return this.getUIContext().px2vp(Number(this.getUIContext().getMeasureUtils().measureTextSize({
      textContent: label, fontSize: this.tokens.fontSize(type.size), fontWeight: this.tokens.type(strong ? 'labelStrong' : 'labelMedium').weight,
    }).width))
  }
  itemWidth(item: DazzTabItem, compact: boolean): number {
    const parts: number[] = []
    if (item.status) parts.push(this.tokens.metric('spacing.1.5'))
    if (item.icon) parts.push(this.tokens.metric('tabs.icon-size'))
    if (!compact || !item.icon) {
      parts.push(this.textWidth(item.label, 'labelMedium', item.value === this.value))
      if (item.detail) parts.push(this.textWidth(item.detail, 'labelMedium'))
    }
    if (item.count !== undefined) parts.push(this.textWidth(String(item.count), 'captionSmall'))
    const close = !compact && item.closable && this.onClose
      ? this.tokens.size('tiny') - this.tokens.metric('spacing.1.5') + this.tokens.metric('spacing.1') : 0
    return Math.round(2 * this.tokens.metric('spacing.2.5') + parts.reduce((sum: number, width: number): number => sum + width, 0) +
      Math.max(0, parts.length - 1) * this.tokens.metric('spacing.1.5') + close)
  }
  fit(): DazzTabsFit {
    return fitTabs(this.items.map((item: DazzTabItem): number => this.itemWidth(item, false)),
      this.items.map((item: DazzTabItem): number => this.itemWidth(item, true)), this.items.findIndex((item: DazzTabItem): boolean => item.value === this.value),
      this.available < 0 ? Infinity : this.available, this.trailing ? this.tailWidth + this.tokens.metric('spacing.1') : 0,
      this.tokens.size('tiny') + this.tokens.metric('spacing.1'))
  }
  ordered(): DazzTabItem[] {
    const fit = this.fit()
    return tabsOrder(this.items.length, fit.shown, this.items.findIndex((item: DazzTabItem): boolean => item.value === this.value))
      .map((index: number): DazzTabItem => this.items[index])
  }
  visible(): DazzTabItem[] { return this.ordered().slice(0, this.fit().shown) }
  hidden(): DazzMenuItem[] {
    return this.ordered().slice(this.fit().shown).map((item: DazzTabItem): DazzMenuItem => ({
      label: item.label + (item.count === undefined ? '' : ' · ' + String(item.count)), icon: item.icon, disabled: item.disabled,
      onSelect: () => this.onValueChange(item.value),
    }))
  }
  stripHeight(): number { return Math.max(this.tokens.size('small'), this.contentHeight ?? 0) }
  onMeasureSize(_self: GeometryInfo, children: Measurable[], constraint: ConstraintSizeOptions): SizeResult {
    const given = Number(constraint.maxWidth)
    const maximum = Number.isFinite(given) ? given : Infinity
    if (this.available !== maximum) this.available = maximum
    const fit = this.fit()
    const width = Math.max(fit.floor, Math.min(fit.basis, maximum))
    const height = Math.max(Number(constraint.minHeight ?? 0), this.stripHeight())
    for (const child of children) child.measure({ minWidth: width, maxWidth: width, minHeight: height, maxHeight: height })
    return { width, height }
  }
  onPlaceChildren(_self: GeometryInfo, children: Layoutable[], _constraint: ConstraintSizeOptions): void {
    for (const child of children) child.layout({ x: 0, y: 0 })
  }
  handleKey(event: KeyEvent): boolean {
    const key = event.keyCode === 2014 ? 'ArrowLeft' : event.keyCode === 2015 ? 'ArrowRight' : event.keyCode === 2081 ? 'Home' : event.keyCode === 2082 ? 'End' : ''
    if (!key || event.type !== KeyType.Down) return false
    const target = tabsNavigate(this.visible(), this.value, key)
    if (!target) return false
    this.onValueChange(target)
    this.getUIContext().getFocusController().activate(true)
    this.getUIContext().getFocusController().requestFocus(this.owner + '-' + target)
    event.stopPropagation()
    return true
  }
  @LocalBuilder more() {
    NavigationGlyphButton({ label: String(this.hidden().length) + ' more', controlID: this.owner + '-more', icon: 'MoreHorizontalRegular', iconSize: this.tokens.metric('tabs.icon-size') })
  }
  build() {
    Row() {
      Row() {
        ForEach(this.visibleRecords(), (record: NavigationTabRecord) => {
          NavigationTab({ record, selected: record.item.value === this.value, compact: this.fit().compact && record.item.value !== this.value,
            canClose: this.onClose !== undefined, contentHeight: this.stripHeight(), contentWidth: this.itemWidth(record.item, this.fit().compact && record.item.value !== this.value), focusID: this.owner + '-' + record.item.value,
            onSelect: () => this.onValueChange(record.item.value), onClose: () => this.onClose?.(record.item.value) })
        }, (record: NavigationTabRecord): string => record.item.value)
      }.height(this.stripHeight()).accessibilityRole(AccessibilityRoleType.TAB_BAR).accessibilityText(this.accessibilityLabel)
        .onKeyEvent((event: KeyEvent): boolean => this.handleKey(event))
      if (this.hidden().length) { DazzMenu({ items: this.hidden(), contentAlign: 'end', trigger: this.more }) }
      if (this.trailing) {
        Row() { this.trailing() }.margin({ left: this.tokens.metric('spacing.1') })
          .onAreaChange((_old: Area, area: Area) => { this.tailWidth = Number(area.width) })
      }
    }.height(this.stripHeight()).width('100%').clip(true)
  }
}
