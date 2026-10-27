import { ColorMetrics, LengthMetrics } from '@kit.ArkUI'
import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'
import {
  DazzTreeNode, DazzTreeRow, flattenTree, treeNavigation, treeTypeahead,
} from '../foundation/TreeData'
import { DazzIcon } from './Icon'
import { DazzBadge } from './Badge'

let treeSequence: number = 0

/** Native focusable rows; hierarchy navigation follows the React tree. */
@Component
export struct DazzTree {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop @Watch('invalidateRows') items: DazzTreeNode[] = []
  @Prop @Watch('invalidateRows') expanded: string[] | undefined = undefined
  @Prop defaultExpanded: string[] = []
  @Prop selected: string | undefined = undefined
  @Prop controlSize: string = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop accessibilityLabel: string = ''
  @State @Watch('invalidateRows') localExpanded: string[] = []
  @State activeID: string | undefined = undefined
  @State hoveredID: string = ''
  onSelectedChange: (id: string) => void = () => {}
  onExpandedChange: (expanded: string[]) => void = () => {}
  private treeID: string = 'dazzui-tree-' + String(++treeSequence)
  private cacheDirty: boolean = true
  private cachedRows: DazzTreeRow[] = []
  private rowIndex: Map<string, DazzTreeRow> = new Map()
  private typedPrefix: string = ''
  private typedAt: number = 0
  private consumedKey: number = -1

  aboutToAppear(): void {
    this.localExpanded = this.defaultExpanded.slice()
    this.activeID = this.selected ?? this.items[0]?.id
  }
  invalidateRows(): void { this.cacheDirty = true }
  visibleRows(): DazzTreeRow[] {
    // Read both observable sources even when cached: row decorations must
    // subscribe to updates as well as the ForEach that mounts visible IDs.
    const items = this.items
    const open = this.expanded ?? this.localExpanded
    if (this.cacheDirty) {
      this.cachedRows = flattenTree(items, open)
      this.rowIndex = new Map()
      this.cachedRows.forEach(row => this.rowIndex.set(row.node.id, row))
      this.cacheDirty = false
    }
    return this.cachedRows
  }
  rowFor(id: string): DazzTreeRow {
    this.visibleRows()
    return this.rowIndex.get(id) ?? {
      node: { id, label: '' }, level: 1, index: 0,
      siblings: 0, branch: false, open: false,
    }
  }
  rowID(id: string): string { return this.treeID + '-' + id }
  stopID(): string | undefined {
    const rows = this.visibleRows()
    return rows.some(row => row.node.id === this.activeID)
      ? this.activeID : rows[0]?.node.id
  }
  focusRow(id: string): void {
    this.activeID = id
    this.getUIContext().getFocusController().requestFocus(this.rowID(id))
  }
  setOpen(id: string, next: boolean): void {
    const values = new Set(this.expanded ?? this.localExpanded)
    if (next) values.add(id)
    else values.delete(id)
    const result = Array.from(values)
    if (this.expanded === undefined) this.localExpanded = result
    this.onExpandedChange(result)
  }
  activateRow(id: string): void {
    const row = this.rowFor(id)
    if (row.node.disabled) return
    this.onSelectedChange(id)
    if (row.branch) this.setOpen(id, !row.open)
  }
  clickRow(id: string, event: ClickEvent): void {
    const row = this.rowFor(id)
    this.focusRow(id)
    if (row.node.disabled) return
    const inset = this.tokens.metric('spacing.1') +
      (row.level - 1) * this.tokens.metric('spacing.4')
    // ClickEvent has no propagation cancellation in API 20. One native row
    // hit-tests its disclosure column so opening it cannot also select it.
    if (row.branch && event.x >= inset &&
      event.x < inset + this.tokens.metric('spacing.4')) {
      this.setOpen(id, !row.open)
    } else this.activateRow(id)
  }
  handleKey(id: string, event: KeyEvent): boolean {
    if (event.type === KeyType.Up) {
      if (this.consumedKey !== event.keyCode) return false
      this.consumedKey = -1
      event.stopPropagation()
      return true
    }
    if (event.type !== KeyType.Down) return false
    const rows = this.visibleRows()
    const action = treeNavigation(rows, id, event.keyCode)
    if (action.handled) {
      this.getUIContext().getFocusController().activate(true)
      if (action.focus !== undefined) this.focusRow(action.focus)
      if (action.select !== undefined) this.onSelectedChange(action.select)
      if (action.expand !== undefined) this.setOpen(id, action.expand)
    } else {
      const control = event.getModifierKeyState?.(['Ctrl']) ?? false
      const character = event.unicode && event.unicode >= 0x21 &&
        event.unicode <= 0x7e ? String.fromCharCode(event.unicode) : ''
      if (control || event.metaKey !== 0 || !character) return false
      const now = Date.now()
      this.typedPrefix = now - this.typedAt <
        this.tokens.metric('tree.typeahead-window')
        ? this.typedPrefix + character.toLowerCase() : character.toLowerCase()
      this.typedAt = now
      const hit = treeTypeahead(rows, id, this.typedPrefix)
      if (hit !== undefined) {
        this.getUIContext().getFocusController().activate(true)
        this.focusRow(hit)
      }
    }
    this.consumedKey = event.keyCode
    event.stopPropagation()
    return true
  }
  rowInk(id: string): string {
    return this.rowFor(id).node.disabled ? this.tokens.color('content-faint')
      : this.selected === id
        ? this.tokens.paint('tinted', 'content', 'normal', this.tint)
        : this.tokens.color(this.hoveredID === id ? 'content' : 'content-nav')
  }
  rowSurface(id: string): ResourceColor {
    return this.selected === id
      ? this.tokens.paint('tinted', 'surface', 'normal', this.tint)
      : this.hoveredID === id && !this.rowFor(id).node.disabled
        ? this.tokens.paint('normal', 'surface', 'hovered') : Color.Transparent
  }
  description(id: string): string {
    const row = this.rowFor(id)
    return 'Level ' + String(row.level) + ', ' + String(row.index + 1) +
      ' of ' + String(row.siblings) +
      (row.branch ? row.open ? ', expanded' : ', collapsed' : '') +
      (row.node.disabled ? ', unavailable' : '')
  }
  @LocalBuilder item(id: string) {
    Button({ type: ButtonType.Normal }) {
      Row({ space: this.tokens.metric('spacing.1') }) {
        Row() {
          if (this.rowFor(id).branch) {
            DazzIcon({ name: 'ChevronRight12Regular',
              iconSize: this.tokens.metric('icon.small'),
              color: this.tokens.color('content-faint') })
              .rotate({ angle: this.rowFor(id).open ? 90 : 0 })
          }
        }.width(this.tokens.metric('spacing.4'))
          .height(this.tokens.metric('spacing.4'))
          .justifyContent(FlexAlign.Center).focusable(false)
          .accessibilityLevel('no').id(this.rowID(id) + '-twisty')
        if (this.rowFor(id).node.icon) {
          DazzIcon({ name: this.rowFor(id).node.icon ?? '',
            iconSize: this.rowFor(id).node.iconSize ?? this.tokens.metric('icon.medium'),
            color: this.selected === id ? this.rowInk(id)
              : this.tokens.color('content-subtle') })
        }
        Text(this.rowFor(id).node.label)
          .fontSize(this.tokens.fontSize(this.tokens.type(
            this.controlSize === 'small' ? 'labelSmall' : 'labelQuiet'
          ).size))
          .fontWeight(this.tokens.type(
            this.selected === id ? 'labelMedium' : 'labelQuiet'
          ).weight)
          .fontColor(this.rowInk(id)).maxLines(1)
          .textOverflow({ overflow: TextOverflow.Ellipsis })
          .layoutWeight(1)
        if (this.rowFor(id).node.meta || this.rowFor(id).node.badge) {
          Row() {
            if (this.rowFor(id).node.badge) {
              DazzBadge({ label: this.rowFor(id).node.badge?.label ?? '',
                tint: this.rowFor(id).node.badge?.tint ?? 'primary',
                variant: this.rowFor(id).node.badge?.variant ?? 'tinted',
                controlSize: this.rowFor(id).node.badge?.controlSize ?? 'medium' })
            } else {
              Text(this.rowFor(id).node.meta ?? '')
                .fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
                .fontWeight(this.tokens.type('captionSmall').weight)
                .fontColor(this.selected === id ? this.rowInk(id)
                  : this.tokens.color('content-faint'))
            }
          }.padding({ left: this.tokens.metric('spacing.2') })
        }
      }.width('100%')
    }
      .height(this.tokens.metric(this.controlSize === 'small' ? 'spacing.6' : 'spacing.7'))
      .width('100%').stateEffect(false)
      .padding({ left: this.tokens.metric('spacing.1') +
        (this.rowFor(id).level - 1) * this.tokens.metric('spacing.4'),
        right: this.tokens.metric('spacing.2'), top: 0, bottom: 0 })
      .borderRadius(this.tokens.metric('radius.small'))
      .backgroundColor(this.rowSurface(id)).id(this.rowID(id))
      .focusOnTouch(true)
      .focusScopePriority(this.treeID, id === this.stopID() ? FocusPriority.PREVIOUS : FocusPriority.AUTO)
      .focusBox({
        margin: LengthMetrics.vp(this.tokens.metric('focus.offset') - this.tokens.metric('spacing.px')),
        strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')),
        strokeColor: ColorMetrics.resourceColor(this.tokens.focus()),
      })
      .accessibilityRole(AccessibilityRoleType.LIST_ITEM)
      .accessibilityGroup(true).accessibilityText(this.rowFor(id).node.label)
      .accessibilityDescription(this.description(id))
      .accessibilitySelected(this.selected === id)
      .onFocus(() => { this.activeID = id })
      .onHover((hover: boolean) => { this.hoveredID = hover ? id : '' })
      .onClick((event: ClickEvent) => this.clickRow(id, event))
  }
  build() {
    Column() {
      ForEach(this.visibleRows().map(row => row.node.id),
        (id: string) => { this.item(id) }, (id: string) => id)
    }.width('100%').alignItems(HorizontalAlign.Start)
      .focusScopeId(this.treeID, true, false)
      // Changing a row's native priority can leave the old Button scope as
      // the key recipient until traversal. Dispatch at the group boundary
      // against the roving row, before a stale child receives the key.
      .onKeyEventDispatch((event: KeyEvent): boolean =>
        this.handleKey(this.stopID() ?? '', event))
      .accessibilityRole(AccessibilityRoleType.LIST)
      .accessibilityText(this.accessibilityLabel)
  }
}
