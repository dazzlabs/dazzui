import { DazzTokens } from '../foundation/Tokens'
import { rememberFocus, forgetFocus } from '../foundation/FocusData'
import { LengthMetrics, ColorMetrics, FrameNode } from '@kit.ArkUI'
import { DazzSize } from '../foundation/TokenTypes'
import { DazzSelectOption, optionLabel, flattenOptions } from '../foundation/Options'
import { DazzMenuRow, DazzMenuRowData, menuWidth, menuAnchorOptions } from './MenuRow'
import { DazzMenuShadow, DazzMenuViewport } from './MenuShadow'
import { DazzMenuWindow } from '../foundation/MenuWindow'
import { DazzMenuChoice, DazzMenuTypeahead, menuCharacter } from '../foundation/MenuData'
let selectSequence: number = 0

/** Native menus retain selection, type-ahead, focus return and group headings. */
@Component
export struct DazzSelect {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.tabular') tabular: boolean = false
  @Prop options: DazzSelectOption[] = []
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop placeholder: string = 'Choose'
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: string = ''
  @State open: boolean = false
  @State focused: boolean = false
  @State triggerWidth: number = 0
  private owner: string = 'dazz-select-' + String(++selectSequence)
  private alive: boolean = false
  private activeID: string = ''
  private viewport: DazzMenuViewport = new DazzMenuViewport()
  private menuWindow: DazzMenuWindow = new DazzMenuWindow()
  private touchOpened: boolean = false
  private keyboardOpening: boolean = false
  private resumeFocus: string = ''
  private consumedKey: number = -1
  private returnFocus: boolean = false
  private typeahead: DazzMenuTypeahead = new DazzMenuTypeahead()
  onValueChange: (value: string) => void = () => {}
  aboutToAppear(): void { this.alive = true; this.localValue = this.defaultValue }
  aboutToDisappear(): void {
    this.alive = false; this.menuWindow.cancel()
    forgetFocus(this.getUIContext().getWindowName() ?? '', this.owner + '-trigger')
  }
  optionID(item: DazzSelectOption): string { return this.owner + '-option-' + encodeURIComponent(item.value ?? item.label) }
  focusSelected(): boolean {
    const selected = flattenOptions(this.options)
      .find((item: DazzSelectOption): boolean => !item.disabled && item.value === (this.value ?? this.localValue))
    if (!selected) return false
    this.focusOption(selected)
    return true
  }
  focusOption(item: DazzSelectOption): void {
    if (this.alive && this.open) {
      const focus = this.getUIContext().getFocusController()
      focus.activate(true)
      focus.requestFocus(this.optionID(item))
    }
  }
  handleMenuKey(event: KeyEvent): boolean {
    // Native Menu owns arrows and confirmation; add endpoints and prefix search.
    if (this.disabled) return false
    if (event.type === KeyType.Up) {
      if (event.keyCode !== this.consumedKey) return false
      this.consumedKey = -1
    } else if (event.type === KeyType.Down) {
      // The native item can confirm before this bubbling key callback. Do
      // not mistake that same Enter/Space for a new closed-trigger command.
      if (!this.open && this.returnFocus && (event.keyCode === 2054 || event.keyCode === 2050)) {
        this.consumedKey = event.keyCode
        event.stopPropagation()
        return true
      }
      if (this.open && event.keyCode === 2070) {
        this.returnFocus = true
        return false
      }
      if (this.open && (event.keyCode === 2054 || event.keyCode === 2050 && !this.typeahead.typing(Date.now()))) {
        // Let the native item confirm on key-down, then contain its key-up
        // if focus has already returned to the trigger. It must not reopen.
        this.consumedKey = event.keyCode
        return false
      }
      const key = menuCharacter(event)
      const enabled = flattenOptions(this.options).filter((item: DazzSelectOption): boolean => !item.disabled)
      if (!this.open && (event.keyCode === 2054 || event.keyCode === 2012 || event.keyCode === 2013 ||
        event.keyCode === 2050 && !this.typeahead.typing(Date.now()))) {
        this.touchOpened = false
        this.keyboardOpening = true
        this.open = true
      } else if (this.open && (event.keyCode === 2081 || event.keyCode === 2082 || !this.activeID && (event.keyCode === 2012 || event.keyCode === 2013))) {
        const target = event.keyCode === 2081 || event.keyCode === 2013 ? enabled[0] : enabled[enabled.length - 1]
        if (target) this.focusOption(target)
      } else {
        if (!key || key === ' ' && !this.typeahead.typing(Date.now())) return false
        const choices = flattenOptions(this.options).map((item: DazzSelectOption): DazzMenuChoice => ({
          id: this.optionID(item), label: item.label, disabled: item.disabled
        }))
        const selected = flattenOptions(this.options).find((item: DazzSelectOption): boolean => item.value === (this.value ?? this.localValue))
        const target = this.typeahead.match(choices, key,
          this.open ? this.activeID : selected ? this.optionID(selected) : '', Date.now())
        const item = flattenOptions(this.options).find((choice: DazzSelectOption): boolean => this.optionID(choice) === target)
        if (item) {
          if (this.open) this.focusOption(item)
          else {
            if (this.value === undefined) this.localValue = item.value ?? ''
            this.onValueChange(item.value ?? '')
          }
        }
      }
      this.consumedKey = event.keyCode
    } else { return false }
    event.stopPropagation()
    return true
  }
  @LocalBuilder groupHeader(label: string) {
    Text(label).fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
      .fontWeight(this.tokens.type('labelSmall').weight)
      .lineHeight(this.tokens.fontSize(this.tokens.type('captionSmall').size))
      .fontColor(this.tokens.color('content-subtle'))
      .width('100%')
      .padding({ top: this.tokens.metric('spacing.1.5'), bottom: this.tokens.metric('spacing.1'),
        left: this.tokens.metric('spacing.2'), right: this.tokens.metric('spacing.2') })
  }
  @LocalBuilder option(item: DazzSelectOption) {
    DazzMenuRow({ label: item.label, checked: item.value === (this.value ?? this.localValue),
      itemID: this.optionID(item), disabled: item.disabled ?? false,
      onHighlight: (node: FrameNode | null) => { this.activeID = this.optionID(item); this.viewport.reveal(node) },
      onMenuKey: (event: KeyEvent): boolean => this.handleMenuKey(event), onSelect: () => {
        if (item.disabled) return
        this.returnFocus = true
        const next = item.value ?? ''
        if (this.value === undefined) this.localValue = next
        this.open = false
        this.onValueChange(next)
      } })
  }
  menuWidth(): number {
    return menuWidth(this.getUIContext(), this.tokens,
      flattenOptions(this.options).map((item: DazzSelectOption): DazzMenuRowData => ({ label: item.label, checked: false })),
      this.triggerWidth)
  }
  @LocalBuilder menu() { DazzMenuShadow({ anchorID: this.owner + '-trigger', viewport: this.viewport, content: this.sheet,
    onMenuKey: (event: KeyEvent): boolean => this.handleMenuKey(event) }) }
  @LocalBuilder sheet() {
    Menu() {
      ForEach(this.options, (item: DazzSelectOption, index: number) => {
        if (item.options) {
          // A builder header has ArkUI's mandatory native row minimum. A
          // noninteractive custom row keeps the heading inside the native
          // group without becoming a keyboard or pointer option.
          MenuItemGroup() {
            MenuItem(() => { this.groupHeader(item.label) })
              .height(this.tokens.type('captionSmall').size + this.tokens.metric('spacing.1.5') + this.tokens.metric('spacing.1'))
              .width('100%').padding(0).margin(0).focusable(false)
              .hitTestBehavior(HitTestMode.None)
              .accessibilityRole(AccessibilityRoleType.TEXT).accessibilityText(item.label)
              .id(this.owner + '-group-' + String(index))
            ForEach(item.options, (child: DazzSelectOption) => { this.option(child) }, (child: DazzSelectOption) => child.value ?? child.label)
          }
            .margin({ top: index > 0 && this.options[index - 1].options ? this.tokens.metric('spacing.1') : 0 })
            .accessibilityText(item.label)
        } else { this.option(item) }
      }, (item: DazzSelectOption) => item.value ?? item.label)
    }.backgroundColor(Color.Transparent)
      .onKeyEvent((event: KeyEvent): boolean => this.handleMenuKey(event))
      .backgroundBlurStyle(BlurStyle.NONE)
      .padding(this.tokens.metric('spacing.1') + this.tokens.metric('stroke.hairline'))
      .border({ width: this.tokens.metric('stroke.hairline'), color: Color.Transparent,
        radius: this.tokens.metric('radius.large') })
      .radius(this.tokens.metric('radius.large'))
      .menuItemGroupDivider({ strokeWidth: LengthMetrics.vp(0), color: Color.Transparent })
      .width(this.menuWidth())
      .id(this.owner + '-menu')
      .zIndex(1)
  }
  active(): boolean { return this.focused || this.open }
  labelColor(): string {
    if (this.disabled) return this.tokens.paint('normal', 'content', 'disabled', 'neutral')
    return this.tokens.color(optionLabel(this.options, this.value ?? this.localValue) ? 'content' : 'content-faint')
  }
  @LocalBuilder face() {
    Row({ space: this.tokens.metric('spacing.2') }) {
      Text(optionLabel(this.options, this.value ?? this.localValue) || this.placeholder)
        .id(this.owner + '-value')
        .fontFeature(this.tabular ? '"tnum" on' : 'normal')
        .fontSize(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size))
        .fontWeight(this.tokens.fieldWeight())
        .fontColor(this.labelColor())
        .lineHeight(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).leading))
        .maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis }).layoutWeight(1)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      Image($rawfile('dazzui/icons/ChevronDown12Regular.svg'))
        .width(this.tokens.metric('icon.small')).height(this.tokens.metric('icon.small'))
        .fillColor(this.tokens.color('content-subtle')).objectFit(ImageFit.Contain)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no')
    }.height('100%').width('100%').id(this.owner + '-surface')
      .padding({ left: this.tokens.fieldPadding(), right: this.tokens.fieldPadding(), top: 0, bottom: 0 })
      .backgroundColor(this.disabled ? this.tokens.paint('normal', 'surface', 'disabled', 'neutral') :
        this.tokens.color(this.active() ? 'surface' : 'surface-muted'))
      .border({ width: this.tokens.metric('stroke.hairline'),
        color: this.tokens.color(this.active() ? 'primary.600' : 'border-strong'),
        radius: this.tokens.metric('control.field.radius') })
      .outline({ width: this.active() ? this.tokens.metric('focus.width') : 0,
        color: this.tokens.focusGlow(), radius: this.tokens.metric('control.field.radius') })
      .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      .accessibilityLevel('no-hide-descendants')
  }
  build() {
    Stack({ alignContent: Alignment.TopStart }) {
      // ArkUI fades a disabled Button after applying its attributes. Keep the
      // native control and its semantic label, and draw the shared recipe in
      // a noninteractive sibling so the native fade cannot wash it out twice.
      Button(optionLabel(this.options, this.value ?? this.localValue) || this.placeholder, { type: ButtonType.Normal })
      .height('100%').width('100%').id(this.owner + '-trigger')
      .padding(0).backgroundColor(Color.Transparent).fontColor(Color.Transparent)
      .fontSize(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size))
      .fontWeight(this.tokens.fieldWeight())
      .border({ width: 0, radius: this.tokens.metric('control.field.radius') })
      .shadow({ radius: 0, color: Color.Transparent })
      .outline({ width: 0, color: Color.Transparent })
      .focusBox({ strokeWidth: LengthMetrics.vp(0), strokeColor: ColorMetrics.resourceColor(Color.Transparent) })
      .enabled(!this.disabled).stateEffect(false)
      .accessibilityRole(AccessibilityRoleType.SELECT)
      .accessibilityText(this.accessibilityLabel || optionLabel(this.options, this.value ?? this.localValue) || this.placeholder)
      .accessibilityDescription((optionLabel(this.options, this.value ?? this.localValue) || this.placeholder) + (this.open ? ', expanded' : ', collapsed'))
      .onFocus(() => {
        rememberFocus(this.getUIContext().getWindowName() ?? '', this.owner + '-trigger')
        this.focused = true
      }).onBlur(() => {
        forgetFocus(this.getUIContext().getWindowName() ?? '', this.owner + '-trigger')
        this.focused = false
        if (!this.open) this.typeahead.reset()
      })
      .onClick((event: ClickEvent) => { this.keyboardOpening = false; this.touchOpened = event.source === SourceType.TouchScreen; this.open = true })
      .onKeyEvent((event: KeyEvent): boolean => this.handleMenuKey(event))
      .onAreaChange((_old: Area, area: Area) => { this.triggerWidth = Number(area.width) })
      .bindMenu(this.open, this.menu, menuAnchorOptions(this.tokens, { placement: Placement.BottomLeft, showInSubWindow: false,
        backgroundBlurStyle: BlurStyle.NONE, backgroundColor: Color.Transparent, borderRadius: this.tokens.metric('radius.large'),
        onWillAppear: () => { this.menuWindow.opened(this.getUIContext()); this.returnFocus = false; this.activeID = ''; this.typeahead.reset() },
        onDidAppear: () => {
          // An appearance animation may finish after a fast confirmation or
          // dismissal. Its callback must not reopen a closing native scope.
          if (!this.alive || !this.open) return
          const resume = flattenOptions(this.options).find((item: DazzSelectOption): boolean =>
            !item.disabled && this.optionID(item) === this.resumeFocus)
          this.resumeFocus = ''
          if (resume) this.focusOption(resume)
          else if (!this.activeID && !this.focusSelected() && this.keyboardOpening) {
            const first = flattenOptions(this.options).find((item: DazzSelectOption): boolean => !item.disabled)
            if (first) this.focusOption(first)
          }
          this.keyboardOpening = false
        },
        onDisappear: () => {
          if (this.menuWindow.closing(this.getUIContext(), this.open && !this.returnFocus && this.touchOpened)) {
            this.resumeFocus = this.activeID
            this.viewport.restoreOffset = this.viewport.scrollOffset
          }
          this.open = false; this.activeID = ''; this.typeahead.reset()
        },
        onDidDisappear: () => {
          if (this.returnFocus && this.alive && !this.open && !this.disabled) {
            this.getUIContext().getFocusController().requestFocus(this.owner + '-trigger')
          }
          this.returnFocus = false
          this.menuWindow.afterClosed(this.getUIContext(), () => {
            if (this.alive && !this.disabled && !this.open) this.open = true
          })
        } }))
      this.face()
    }.height(this.tokens.fieldSize(this.controlSize)).width('100%').clip(false)
  }
}
