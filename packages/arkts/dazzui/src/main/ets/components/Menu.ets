import { DazzActivationBoundary } from '../foundation/Activation'
import { DazzTokens } from '../foundation/Tokens'
import { DazzPointerContext } from '../foundation/Pointer'

import { DazzButton } from './Button'
import { DazzMenuRow, menuWidth, menuAnchorOptions } from './MenuRow'
import { DazzMenuShadow, DazzMenuViewport } from './MenuShadow'
import { DazzMenuWindow } from '../foundation/MenuWindow'
import { DazzMenuChoice, DazzMenuTypeahead, menuCharacter } from '../foundation/MenuData'
import { DazzMenubarContext, DazzMenubarChoice, DazzMenubarRect, insideMenubar, menubarDestination } from '../foundation/MenuData'
import { focusID, rememberFocus, forgetFocus } from '../foundation/FocusData'
import { DazzActivationContext } from '../foundation/Activation'
import { LengthMetrics, ColorMetrics, FrameNode, FrameCallback } from '@kit.ArkUI'

class AfterMenuFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  onIdle(_timeLeftInNano: number): void { this.action() }
}

export interface DazzMenuItem {
  label: string
  disabled?: boolean
  icon?: string
  shortcut?: string
  checked?: boolean
  onSelect?: () => void
}

let menuSequence: number = 0

@Component
export struct DazzMenu {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.pointer') pointer: DazzPointerContext = new DazzPointerContext()
  @Consume('dazzui.menubar') @Watch('attachBarPointerHost') bar: DazzMenubarContext = new DazzMenubarContext()
  @Prop items: DazzMenuItem[] = []
  @Prop contextMenu: boolean = false
  @Prop contentAlign: string = 'start'
  @Prop barIndex: number = -1
  @State open: boolean = false
  private bodyRect: DazzMenubarRect | undefined = undefined
  private pointerHost: FrameNode | null = null
  private surfaceID: string = 'dazzui-menu-surface-' + String(++menuSequence)
  private typeahead: DazzMenuTypeahead = new DazzMenuTypeahead(500)
  private activeID: string = ''
  private viewport: DazzMenuViewport = new DazzMenuViewport()
  private menuWindow: DazzMenuWindow = new DazzMenuWindow()
  private lastItemID: string = ''
  private resumeFocus: string = ''
  private consumedKey: number = -1
  private triggerKey: string = focusID('menu-trigger')
  private returnFocus: boolean = false
  private returnFocusVisible: boolean = false
  private alive: boolean = false
  private dismissed: boolean = true
  private resume: boolean = false
  private resumeQueued: boolean = false
  aboutToAppear(): void { this.alive = true }
  aboutToDisappear(): void { this.alive = false; this.menuWindow.cancel(); this.detachBarPointerHost() }
  @BuilderParam trigger: () => void = this.defaultTrigger
  @LocalBuilder defaultTrigger() {
    DazzButton({ label: 'Menu' })
  }
  isOpen(): boolean { return this.barIndex >= 0 ? this.bar.activeIndex === this.barIndex : this.open }
  changeOpen(open: boolean): void {
    if (!this.alive) return
    if (this.barIndex >= 0) this.bar.onOpen(this.barIndex, open)
    else this.open = open
  }
  presented(): void {
    this.dismissed = false
    this.attachBarPointerHost()
    if (this.isOpen() && this.resumeFocus) {
      const target = this.resumeFocus
      this.resumeFocus = ''
      if (this.choices().some((item: DazzMenuChoice): boolean => item.id === target && !item.disabled))
        this.getUIContext().getFocusController().requestFocus(target)
      return
    }
    if (this.barIndex < 0 || !this.isOpen() || !this.bar.focusEdge) return
    const enabled = this.choices().filter((item: DazzMenuChoice): boolean => !item.disabled)
    const target = this.bar.focusEdge < 0 ? enabled[enabled.length - 1] : enabled[0]
    if (target) {
      this.getUIContext().getFocusController().activate(true)
      this.getUIContext().getFocusController().requestFocus(target.id)
    }
  }
  attachBarPointerHost(): void {
    if (this.barIndex < 0 || !this.isOpen()) return
    const ui = this.getUIContext()
    const node = ui.getAttachedFrameNodeById(this.surfaceID)
    const bar = this.bar.bounds()
    if (!node || !bar) return
    const position = node.getPositionToWindow()
    const size = node.getMeasuredSize()
    if (size.width <= 0 || size.height <= 0) return
    this.bodyRect = { x: position.x, y: position.y, width: ui.px2vp(size.width), height: ui.px2vp(size.height) }
    let host = node.getParent()
    while (host && host.getNodeType() !== 'MenuWrapper') host = host.getParent()
    if (!host || host.getUniqueId() === this.pointerHost?.getUniqueId()) return
    this.detachBarPointerHost()
    this.pointerHost = host
    // The native scroll clip prevents a sheet's own event region from reaching
    // its bar. Observe the owning modal host through the public FrameNode API;
    // leave its outside-dismissal behavior intact everywhere outside this bar.
    host.commonEvent.setOnTouch((event: TouchEvent) => {
      const point = event.changedTouches[0] ?? event.touches[0]
      if (!point || !this.overBar(point.windowX, point.windowY)) return
      // Mouse presses also arrive here before the native host disappears;
      // its later mouse callback may already have been detached by then.
      if (event.type === TouchType.Down) this.barPointer(point.windowX, point.windowY, true)
      event.stopPropagation()
    })
    host.commonEvent.setOnMouse((event: MouseEvent) => {
      if (!this.overBar(event.windowX, event.windowY)) return
      if (event.action === MouseAction.Move && this.isOpen()) this.barPointer(event.windowX, event.windowY, false)
      if (event.action === MouseAction.Press && event.button === MouseButton.Left) this.barPointer(event.windowX, event.windowY, true)
      event.stopPropagation()
    })
  }
  detachBarPointerHost(): void {
    const previous = this.pointerHost
    this.pointerHost = null
    if (!previous) return
    setTimeout(() => {
      if (previous.getUniqueId() === this.pointerHost?.getUniqueId()) return
      previous.commonEvent.setOnTouch(undefined)
      previous.commonEvent.setOnMouse(undefined)
    }, 0)
  }
  overBar(x: number, y: number): boolean {
    return this.barIndex >= 0 && this.alive && !insideMenubar(this.bodyRect, x, y) && insideMenubar(this.bar.bounds(), x, y)
  }
  barPointer(x: number, y: number, press: boolean): void {
    const index = this.bar.entryAt(x, y)
    if (press) {
      if (index < 0 || !this.bar.enabled[index]) {
        // The host's built-in outside handler runs before common events. A
        // disabled label or bar gap belongs to the bar, so resume this menu
        // after the native dismissal and its bound false state have rendered.
        this.resume = true
        this.resumeFocus = this.activeID || this.lastItemID
        if (this.dismissed) this.resumeAfterDismiss()
        return
      }
      this.resume = false
      this.returnFocus = index === this.barIndex
      this.bar.onOpen(index, index !== this.barIndex)
      if (index === this.barIndex) this.bar.returnFocus(index)
    } else if (index >= 0) this.bar.onHover(index)
  }
  resumeAfterDismiss(): void {
    if (this.resumeQueued) return
    this.resumeQueued = true
    this.getUIContext().postFrameCallback(new AfterMenuFrame(() => {
      this.resumeQueued = false
      if (!this.alive || !this.resume || this.bar.activeIndex >= 0) { this.resume = false; this.resumeFocus = ''; return }
      this.resume = false
      this.bar.onOpen(this.barIndex, true)
    }))
  }
  checked(index: number): boolean | undefined {
    return this.items[index].checked === undefined ? undefined :
      index === this.items.findIndex((item: DazzMenuItem): boolean => item.checked === true)
  }
  itemID(index: number): string { return this.surfaceID + '-item-' + String(index) }
  choices(): DazzMenuChoice[] {
    return this.items.map((item: DazzMenuItem, index: number): DazzMenuChoice => ({
      id: this.itemID(index), label: item.label, disabled: item.disabled
    }))
  }
  menuKey(event: KeyEvent): boolean {
    if (this.barIndex >= 0 && this.bar.onKey(this.barIndex, event, true)) return true
    if (event.type === KeyType.Up) {
      if (event.keyCode !== this.consumedKey) return false
      this.consumedKey = -1
    } else if (event.type === KeyType.Down) {
      if (event.keyCode === 2070) { this.returnFocus = true; return false }
      const choices = this.choices()
      let target: string | undefined = undefined
      if (event.keyCode === 2081 || event.keyCode === 2082 || !this.activeID && (event.keyCode === 2012 || event.keyCode === 2013)) {
        const enabled = choices.filter((item: DazzMenuChoice): boolean => !item.disabled)
        target = (event.keyCode === 2081 || event.keyCode === 2013 ? enabled[0] : enabled[enabled.length - 1])?.id
      } else {
        const key = menuCharacter(event)
        if (!key || key === ' ' && !this.typeahead.typing(Date.now())) return false
        target = this.typeahead.match(choices, key, this.activeID, Date.now())
      }
      if (target) {
        this.getUIContext().getFocusController().activate(true)
        this.getUIContext().getFocusController().requestFocus(target)
      }
      this.consumedKey = event.keyCode
    } else { return false }
    event.stopPropagation()
    return true
  }
  @LocalBuilder menu() { DazzMenuShadow({ anchorID: this.triggerKey, viewport: this.viewport, content: this.sheet,
    onMenuKey: (event: KeyEvent): boolean => this.menuKey(event) }) }
  @LocalBuilder sheet() {
    Menu() {
      ForEach(
        this.items,
        (item: DazzMenuItem, index: number) => {
          DazzMenuRow({ label: item.label, icon: item.icon ?? '', shortcut: item.shortcut ?? '',
            itemID: this.itemID(index), checked: this.checked(index), disabled: item.disabled ?? false,
            onHighlight: (node: FrameNode | null) => {
              this.activeID = this.itemID(index); this.lastItemID = this.activeID; this.viewport.reveal(node)
            },
            onMenuKey: (event: KeyEvent): boolean => this.menuKey(event), onSelect: () => {
              this.returnFocus = true
              this.changeOpen(false)
              item.onSelect?.()
            } })
        },
        (item: DazzMenuItem, index: number) => String(index)
      )
    }
      .backgroundColor(Color.Transparent)
      .backgroundBlurStyle(BlurStyle.NONE)
      // Menu's layout subtracts padding alone; its border is an overlay.
      .padding(this.tokens.metric('spacing.1') + this.tokens.metric('stroke.hairline'))
      .border({ width: this.tokens.metric('stroke.hairline'), color: Color.Transparent,
        radius: this.tokens.metric('radius.large') })
      .radius(this.tokens.metric('radius.large'))
      .width(menuWidth(this.getUIContext(), this.tokens, this.items))
      .id(this.surfaceID)
      .onTouchIntercept((event: TouchEvent): HitTestMode => {
        if (event.type === TouchType.Down) this.pointer.resetTouchHover()
        return HitTestMode.Default
      })
      .onMouse((event: MouseEvent) => { if (event.action === MouseAction.Move) this.pointer.resetTouchHover() })
      .zIndex(1)
      .onKeyEvent((event: KeyEvent): boolean => this.menuKey(event))
      .onAreaChange(() => this.attachBarPointerHost())
  }
  @LocalBuilder forwardTrigger() { this.trigger() }
  build() {
    if (this.contextMenu) {
      Row() {
        this.trigger()
      }
        .id(this.triggerKey)
        .bindContextMenu(this.menu, ResponseType.RightClick, { backgroundBlurStyle: BlurStyle.NONE, backgroundColor: Color.Transparent,
          borderRadius: this.tokens.metric('radius.large'), onAppear: () => { this.typeahead.reset(); this.activeID = '' },
          onDisappear: () => { this.typeahead.reset(); this.activeID = '' } })
    } else {
      DazzActivationBoundary({ focusKey: this.triggerKey, content: this.forwardTrigger, onAction: (focusVisible: boolean) => {
        this.returnFocusVisible = focusVisible
        if (this.barIndex >= 0) this.bar.onClick(this.barIndex)
        else this.changeOpen(true)
      } })
        .bindMenu(this.isOpen(), this.menu, menuAnchorOptions(this.tokens, { placement: this.contentAlign === 'end' ? Placement.BottomRight : Placement.BottomLeft,
          backgroundBlurStyle: BlurStyle.NONE, backgroundColor: Color.Transparent, borderRadius: this.tokens.metric('radius.large'),
          showInSubWindow: this.barIndex >= 0 && !this.bar.modal,
          modalMode: this.barIndex >= 0 && this.bar.modal ? ModalMode.TARGET_WINDOW : ModalMode.NONE,
          transition: this.barIndex >= 0 ? TransitionEffect.IDENTITY : undefined,
          onWillAppear: () => { this.menuWindow.opened(this.getUIContext()); this.returnFocus = false; this.typeahead.reset(); this.activeID = ''; if (!this.resumeFocus) this.lastItemID = '' },
          onDidAppear: () => this.presented(),
          onDisappear: () => {
            if (this.menuWindow.closing(this.getUIContext(), this.isOpen() && !this.returnFocus)) {
              this.resumeFocus = this.activeID
              this.viewport.restoreOffset = this.viewport.scrollOffset
            }
            this.changeOpen(false); this.typeahead.reset(); this.activeID = ''
          },
          onDidDisappear: () => {
            this.dismissed = true
            this.detachBarPointerHost()
            if (this.resume && this.alive) this.resumeAfterDismiss()
            if (this.barIndex >= 0 && this.alive) this.bar.onClosed(this.barIndex)
            if (this.returnFocus && this.alive && !this.isOpen()) {
              if (this.barIndex >= 0) this.bar.returnFocus(this.barIndex)
              else {
                const focus = this.getUIContext().getFocusController()
                if (this.returnFocusVisible) focus.activate(true)
                focus.requestFocus(this.triggerKey)
              }
            }
            this.returnFocus = false
            this.menuWindow.afterClosed(this.getUIContext(), () => {
              if (this.alive && !this.isOpen()) this.changeOpen(true)
            })
          } }))
    }
  }
}

export interface DazzMenubarEntry { label: string; items: DazzMenuItem[]; disabled?: boolean }

@Component
struct MenubarTrigger {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.menubar') bar: DazzMenubarContext
  @Consume('dazzui.activation') activation: DazzActivationContext
  @Prop label: string = ''
  @Prop index: number = 0
  @Prop disabled: boolean = false
  @State hovered: boolean = false
  controlID(): string { return this.bar.scope + '-trigger-' + String(this.index) }
  aboutToDisappear(): void { forgetFocus(this.getUIContext().getWindowName() ?? '', this.controlID()) }
  intrinsicWidth(): number {
    const label = this.tokens.label('small')
    const size = this.getUIContext().getMeasureUtils().measureTextSize({ textContent: this.label,
      fontSize: this.tokens.fontSize(label.size), fontWeight: label.weight })
    return (typeof size.width === 'number' ? this.getUIContext().px2vp(size.width) : 0) + 2 * this.tokens.metric('spacing.2')
  }
  build() {
    Stack() {
      Button(this.label, { type: ButtonType.Normal })
        .id(this.controlID()).width('100%').height('100%').padding(0)
        .fontSize(this.tokens.fontSize(this.tokens.label('small').size))
        .fontWeight(this.tokens.label('small').weight).fontColor(Color.Transparent)
        .backgroundColor(Color.Transparent).border({ width: 0, radius: this.tokens.metric('radius.small') })
        .shadow({ radius: 0, color: Color.Transparent }).stateEffect(false).enabled(!this.disabled)
        .focusScopePriority(this.bar.scope, this.bar.focusedIndex === this.index ? FocusPriority.PREVIOUS : FocusPriority.AUTO)
        .focusBox({ margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
          strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')), strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
        .accessibilityRole(AccessibilityRoleType.MENU_ITEM).accessibilityText(this.label)
        .accessibilityDescription(this.bar.activeIndex === this.index ? 'Menu, expanded' : 'Menu, collapsed')
        .onHover((hovered: boolean) => {
          this.hovered = hovered
        })
        .onMouse((event: MouseEvent) => {
          if (event.action === MouseAction.Move && !this.disabled) this.bar.onHover(this.index)
        })
        .onFocus(() => {
          rememberFocus(this.getUIContext().getWindowName() ?? '', this.controlID())
          this.bar.onFocus(this.index)
        })
        .onClick(() => this.activation.activate())
        .onKeyEvent((event: KeyEvent): boolean => this.bar.onKey(this.index, event, false))
      Text(this.label).width('100%').height('100%').textAlign(TextAlign.Center)
        .fontSize(this.tokens.fontSize(this.tokens.label('small').size))
        .lineHeight(this.tokens.fontSize(this.tokens.label('small').size))
        .fontWeight(this.tokens.label('small').weight)
        .fontColor(this.disabled ? this.tokens.paint('normal', 'content', 'disabled', 'neutral') : this.tokens.color('content'))
        .backgroundColor(!this.disabled && (this.hovered || this.bar.activeIndex === this.index)
          ? this.tokens.paint('normal', 'surface', 'hovered', 'neutral') : Color.Transparent)
        .borderRadius(this.tokens.metric('radius.small'))
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false).accessibilityLevel('no-hide-descendants')
        .id(this.controlID() + '-face')
    }.width(this.intrinsicWidth()).height(this.tokens.size('small')).clip(false)
  }
}

@Component
export struct DazzMenubar {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop @Watch('menusChanged') menus: DazzMenubarEntry[] = []
  @Prop @Watch('menusChanged') disabled: boolean = false
  @Prop orientation: string = 'horizontal'
  @Prop @Watch('menusChanged') modal: boolean = true
  @Provide({ allowOverride: 'dazzui.menubar' }) bar: DazzMenubarContext = new DazzMenubarContext()
  private scope: string = focusID('menubar')
  private active: number = -1
  private focused: number = 0
  private edge: number = 0
  private consumed: number = -1
  private pendingTab: KeyEvent | undefined = undefined
  private pendingIndex: number = -1
  aboutToAppear(): void { this.menusChanged() }
  choices(): DazzMenubarChoice[] { return this.menus.map((menu: DazzMenubarEntry): DazzMenubarChoice => ({ label: menu.label, disabled: this.disabled || menu.disabled })) }
  menusChanged(): void {
    const choices = this.choices()
    if (!choices[this.focused] || choices[this.focused].disabled) this.focused = menubarDestination(choices, this.focused, 2081)
    if (!choices[this.active] || choices[this.active].disabled) this.active = -1
    this.publish()
  }
  publish(): void {
    const next = new DazzMenubarContext()
    next.scope = this.scope
    next.activeIndex = this.active
    next.focusedIndex = this.focused
    next.focusEdge = this.edge
    next.modal = this.modal
    next.enabled = this.choices().map((item: DazzMenubarChoice): boolean => !item.disabled)
    next.onOpen = (index: number, open: boolean) => {
      if (open) this.show(index, 0)
      else if (this.active === index) { this.active = -1; this.publish() }
    }
    next.onHover = (index: number) => { if (this.active >= 0 && this.active !== index) this.show(index, 0) }
    next.onClick = (index: number) => {
      if (this.active === index) { this.active = -1; this.publish() }
      else this.show(index, 0)
    }
    next.onFocus = (index: number) => {
      if (this.focused === index) return
      this.focused = index
      this.publish()
    }
    next.onKey = (index: number, event: KeyEvent, popup: boolean): boolean => this.handleBarKey(index, event, popup)
    next.returnFocus = (index: number) => this.focus(index)
    next.onClosed = (index: number) => this.continueTab(index)
    next.bounds = (): DazzMenubarRect | undefined => {
      const rects = this.menus.map((_menu: DazzMenubarEntry, index: number): DazzMenubarRect | undefined => this.triggerRect(index))
        .filter((rect: DazzMenubarRect | undefined): boolean => rect !== undefined) as DazzMenubarRect[]
      if (!rects.length) return undefined
      const left = Math.min(...rects.map((rect: DazzMenubarRect): number => rect.x))
      const top = Math.min(...rects.map((rect: DazzMenubarRect): number => rect.y))
      return { x: left, y: top,
        width: Math.max(...rects.map((rect: DazzMenubarRect): number => rect.x + rect.width)) - left,
        height: Math.max(...rects.map((rect: DazzMenubarRect): number => rect.y + rect.height)) - top }
    }
    next.entryAt = (x: number, y: number): number => this.menus.findIndex((_menu: DazzMenubarEntry, index: number): boolean => insideMenubar(this.triggerRect(index), x, y))
    this.bar = next
  }
  focus(index: number): void {
    if (index < 0 || this.choices()[index]?.disabled) return
    if (this.focused !== index) { this.focused = index; this.publish() }
    this.getUIContext().getFocusController().requestFocus(this.scope + '-trigger-' + String(index))
  }
  show(index: number, edge: number): void {
    if (index < 0 || !this.menus[index] || this.disabled || this.menus[index].disabled) return
    this.pendingTab = undefined
    this.pendingIndex = -1
    this.active = index
    this.focused = index
    this.edge = edge
    this.publish()
  }
  triggerRect(index: number): DazzMenubarRect | undefined {
    const ui = this.getUIContext()
    const node = ui.getAttachedFrameNodeById(this.scope + '-trigger-' + String(index))
    if (!node) return undefined
    const position = node.getPositionToWindow()
    const size = node.getMeasuredSize()
    return { x: position.x, y: position.y, width: ui.px2vp(size.width), height: ui.px2vp(size.height) }
  }
  continueTab(index: number): void {
    if (this.pendingIndex !== index || this.active >= 0) return
    const event = this.pendingTab
    this.pendingTab = undefined
    this.pendingIndex = -1
    if (!event) return
    this.focus(index)
    const ui = this.getUIContext()
    let root = ui.getAttachedFrameNodeById(this.scope + '-group')
    if (!root) return
    let parent = root.getParent()
    while (parent) { root = parent; parent = root.getParent() }
    ui.dispatchKeyEvent(root.getUniqueId(), event)
  }
  handleBarKey(index: number, event: KeyEvent, popup: boolean): boolean {
    if (event.type === KeyType.Up && event.keyCode === 2050 && !popup && this.consumed === 2050) {
      this.consumed = -1
      this.show(index, 1)
      event.stopPropagation()
      return true
    }
    if (event.type === KeyType.Up && this.consumed === event.keyCode) {
      this.consumed = -1; event.stopPropagation(); return true
    }
    if (event.type !== KeyType.Down && !(event.type === KeyType.Up && event.keyCode === 2050 && !popup)) return false
    if (popup && event.keyCode === 2049) {
      this.pendingTab = event
      this.pendingIndex = index
      this.active = -1
      this.publish()
      return true
    }
    const destination = menubarDestination(this.choices(), index, event.keyCode,
      !popup && this.orientation === 'vertical')
    if (destination >= 0 && (!popup || event.keyCode === 2014 || event.keyCode === 2015)) {
      if (popup || this.active >= 0) this.show(destination, 1)
      else { this.focused = destination; this.publish(); this.focus(destination) }
    } else if (!popup && (event.keyCode === 2054 || event.keyCode === 2119 || event.keyCode === 2050 ||
      this.orientation !== 'vertical' && (event.keyCode === 2012 || event.keyCode === 2013))) {
      if (event.keyCode !== 2050 || event.type === KeyType.Up) this.show(index, event.keyCode === 2012 ? -1 : 1)
    } else { return false }
    this.getUIContext().getFocusController().activate(true)
    this.consumed = event.keyCode
    event.stopPropagation()
    return true
  }
  @LocalBuilder trigger(entry: DazzMenubarEntry, index: number) {
    MenubarTrigger({ label: entry.label, index, disabled: this.disabled || (entry.disabled ?? false) })
  }
  @LocalBuilder entries() {
    // Recreate the native anchor when its window/input policy changes.
    // Index keys also permit repeated labels.
    ForEach(this.menus, (entry: DazzMenubarEntry, index: number) => {
        DazzMenu({ items: entry.items, barIndex: index, trigger: () => { this.trigger(entry, index) } })
      }, (_entry: DazzMenubarEntry, index: number) => String(index) + ':' + String(this.modal))
  }
  build() {
    if (this.orientation === 'vertical') {
      Column({ space: this.tokens.metric('spacing.0.5') }) { this.entries() }
        .id(this.scope + '-group').focusScopeId(this.scope, true, false).accessibilityRole(AccessibilityRoleType.MENU)
        .onAreaChange(() => this.publish())
    } else {
      Row({ space: this.tokens.metric('spacing.0.5') }) { this.entries() }
        .id(this.scope + '-group').focusScopeId(this.scope, true, false).accessibilityRole(AccessibilityRoleType.MENU)
        .onAreaChange(() => this.publish())
    }
  }
}
