import { DazzTokens } from '../foundation/Tokens'
import { DazzEditingContext } from '../foundation/Editing'
import { DazzSize } from '../foundation/TokenTypes'
import { DazzSelectOption, flattenOptions, filterOptions, optionLabel, nextOption } from '../foundation/Options'
import { DazzIcon } from './Icon'

let querySequence: number = 0

/** ArkUI owns editing/IME and popup placement; query and highlight stay scoped. */
@Component
export struct DazzCombobox {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.editing') editing: DazzEditingContext = new DazzEditingContext()
  @Prop options: DazzSelectOption[] = []
  @Prop value: string | null | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop placeholder: string = 'Search…'
  @Prop empty: string = 'No matches'
  @Prop clearable: boolean = true
  @Prop freeText: boolean = false
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: string = ''
  @State query: string = ''
  @State filtering: boolean = false
  @State open: boolean = false
  @State focused: boolean = false
  @State highlighted: number = -1
  @State fieldWidth: number = 0
  private inputID: string = 'dazzui-query-' + String(++querySequence)
  private openTimer: number = -1
  private scroller: Scroller = new Scroller()
  onValueChange: (value: string | null) => void = () => {}
  aboutToAppear(): void { this.localValue = this.defaultValue }
  aboutToDisappear(): void { clearTimeout(this.openTimer); this.editing.clear(this.inputID) }
  requestOpen(): void {
    clearTimeout(this.openTimer)
    this.openTimer = setTimeout(() => {
      this.highlighted = flattenOptions(this.choices()).findIndex((item: DazzSelectOption) => item.value === this.selectedValue())
      this.open = true
    }, 0)
  }
  selectedValue(): string { return this.value === undefined ? this.localValue : this.value ?? '' }
  displayValue(): string {
    return this.freeText ? this.selectedValue() : this.filtering ? this.query : optionLabel(this.options, this.selectedValue())
  }
  choices(): DazzSelectOption[] { return filterOptions(this.options, this.filtering ? this.query : '') }
  suggestionDescription(): string {
    if (!this.open) return ''
    const choices = flattenOptions(this.choices())
    return String(choices.length) + ' suggestions' +
      (choices[this.highlighted] ? '. ' + choices[this.highlighted].label : '')
  }
  rowHeight(): number { return this.tokens.type('labelQuiet').size + 2 * this.tokens.metric('menu.item-padding') }
  menuHeight(options: DazzSelectOption[] = this.choices()): number {
    return options.reduce((height: number, option: DazzSelectOption) => height + (option.options
      ? this.tokens.type('captionSmall').size + this.tokens.metric('spacing.1.5') + this.tokens.metric('spacing.1') + this.menuHeight(option.options)
      : this.rowHeight()), 0)
  }
  change(query: string): void {
    this.query = query
    this.filtering = true
    this.open = true
    this.highlighted = -1
    if (this.freeText) {
      if (this.value === undefined) this.localValue = query
      this.onValueChange(query)
    }
  }
  close(): void { clearTimeout(this.openTimer); this.open = false; this.filtering = false; this.highlighted = -1 }
  choose(option: DazzSelectOption): void {
    if (option.disabled) return
    const next = option.value ?? ''
    if (this.value === undefined) this.localValue = next
    this.query = this.freeText ? next : option.label
    this.close()
    this.focusInput()
    this.onValueChange(next)
  }
  clear(): void {
    if (this.value === undefined) this.localValue = ''
    this.query = ''; this.filtering = true; this.highlighted = -1
    this.open = true
    this.focusInput()
    this.onValueChange(null)
  }
  focusInput(): void {
    if (this.focused) return
    // Selection still succeeds if an overlay loses its focus scope while closing.
    try { this.getUIContext().getFocusController().requestFocus(this.inputID) } catch (_) {}
  }
  handleKey(event: KeyEvent): void {
    if (event.type !== KeyType.Down) return
    if (event.keyCode === 2012 || event.keyCode === 2013) {
      this.open = true
      this.highlighted = nextOption(this.choices(), this.highlighted, event.keyCode === 2012 ? -1 : 1)
      if (this.highlighted >= 0) {
        try { this.scroller.scrollTo({ xOffset: 0, yOffset: this.highlighted * this.rowHeight() }) } catch (_) {}
      }
      event.stopPropagation()
    } else if (event.keyCode === 2070 && this.open) {
      this.close(); event.stopPropagation()
    } else if (event.keyCode === 2049) { this.close() }
  }
  submit(): void {
    const option = flattenOptions(this.choices())[this.highlighted]
    if (this.open && option) this.choose(option)
    else this.close()
  }
  @LocalBuilder option(item: DazzSelectOption) {
    Button({ type: ButtonType.Normal }) {
      Row({ space: this.tokens.metric('spacing.2') }) {
        if (!this.freeText) {
          DazzIcon({ name: 'Checkmark12Filled', iconSize: this.tokens.metric('icon.small') })
            .opacity(item.value === this.selectedValue() ? 1 : 0)
        }
        Text(item.label).fontSize(this.tokens.fontSize(this.tokens.type('labelQuiet').size))
          .fontWeight(this.tokens.type('labelQuiet').weight)
          .fontColor(this.tokens.color(item.disabled ? 'content-faint' : 'content'))
          .maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis }).layoutWeight(1)
      }.width('100%')
    }.height(this.rowHeight()).width('100%')
      .padding({ left: this.tokens.metric('spacing.2'), right: this.tokens.metric('spacing.2'), top: 0, bottom: 0 })
      .borderRadius(this.tokens.metric('radius.large') - this.tokens.metric('spacing.1'))
      .backgroundColor(flattenOptions(this.choices())[this.highlighted]?.value === item.value
        ? this.tokens.paint('normal', 'surface', 'hovered') : Color.Transparent)
      .enabled(!item.disabled).stateEffect(false).accessibilityText(item.label)
      .accessibilitySelected(item.value === this.selectedValue())
      .onHover((hover: boolean) => {
        if (hover && !item.disabled) this.highlighted = flattenOptions(this.choices()).findIndex((choice: DazzSelectOption) => choice.value === item.value)
      }).onClick(() => this.choose(item))
  }
  @LocalBuilder sheet() {
    Scroll(this.scroller) {
      Column() {
        if (!this.choices().length) {
          Text(this.empty).fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size))
            .fontColor(this.tokens.color('content-faint'))
            .padding({ top: this.tokens.metric('menu.item-padding'), bottom: this.tokens.metric('menu.item-padding'), left: this.tokens.metric('spacing.2'), right: this.tokens.metric('spacing.2') })
        }
        ForEach(this.choices(), (item: DazzSelectOption) => {
          if (item.options) {
            Text(item.label).fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
              .fontWeight(this.tokens.type('labelSmall').weight).fontColor(this.tokens.color('content-subtle'))
              .width('100%').textAlign(TextAlign.Start)
              .padding({ top: this.tokens.metric('spacing.1.5'), bottom: this.tokens.metric('spacing.1'), left: this.tokens.metric('spacing.2'), right: this.tokens.metric('spacing.2') })
            ForEach(item.options, (child: DazzSelectOption) => { this.option(child) }, (child: DazzSelectOption) => child.value ?? child.label)
          } else { this.option(item) }
        }, (item: DazzSelectOption) => item.value ?? item.label)
      }.width('100%').alignItems(HorizontalAlign.Start)
    }.height((this.menuHeight() || this.tokens.type('bodySmall').leading + 2 * this.tokens.metric('menu.item-padding')) + 2 * this.tokens.metric('spacing.1') + 2 * this.tokens.metric('stroke.hairline'))
      .constraintSize({ maxHeight: '90%' }).scrollBar(BarState.Auto)
      .width(Math.max(this.fieldWidth, this.tokens.metric('menu.min-width')))
      .padding(this.tokens.metric('spacing.1'))
      .backgroundColor(this.tokens.color('surface-overlay'))
      .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color('border-strong'), radius: this.tokens.metric('radius.large') })
      .accessibilityDescription(String(flattenOptions(this.choices()).length) + ' suggestions')
  }
  @LocalBuilder action(name: string, label: string, clear: boolean) {
    Button({ type: ButtonType.Normal }) {
      DazzIcon({ name, iconSize: this.tokens.metric('icon.small'), color: this.tokens.color('content-subtle') })
    }.width(this.tokens.metric('spacing.5')).height(this.tokens.metric('spacing.5')).padding(0)
      .backgroundColor(Color.Transparent).borderRadius(this.tokens.metric('radius.small'))
      .enabled(!this.disabled).accessibilityText(label)
      .onClick(() => { if (clear) this.clear(); else if (this.open) this.close(); else { this.filtering = false; this.requestOpen() } })
  }
  build() {
    Row({ space: this.tokens.metric('spacing.1') }) {
      TextInput({ text: this.displayValue(), placeholder: this.placeholder })
        .id(this.inputID).fontSize(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size))
        .fontWeight(this.tokens.fieldWeight()).fontColor(this.tokens.color('content'))
        .placeholderColor(this.tokens.color('content-faint'))
        .placeholderFont({ size: this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size), weight: this.tokens.fieldWeight() })
        .backgroundColor(Color.Transparent).borderRadius(0).padding(0).layoutWeight(1)
        .height('100%').enabled(!this.disabled)
        .accessibilityText(this.accessibilityLabel || this.placeholder)
        .accessibilityDescription(this.suggestionDescription())
        .onChange((value: string, preview?: PreviewText) => { this.editing.preview(this.inputID, preview?.value ?? ''); this.change(value) })
        .onFocus(() => { this.focused = true })
        .onBlur(() => { this.focused = false; this.editing.clear(this.inputID) })
        .onClick(() => { this.requestOpen() })
        .onKeyEvent((event: KeyEvent) => this.handleKey(event)).onSubmit(() => this.submit())
      if (!this.freeText) {
        Row() {
          if (this.clearable && this.selectedValue()) { this.action('Dismiss12Regular', 'Clear', true) }
          this.action('ChevronDown12Regular', 'Open', false)
        }
      }
    }.height(this.tokens.fieldSize(this.controlSize)).width('100%')
      .padding({ left: this.tokens.fieldPadding(), right: this.freeText ? this.tokens.fieldPadding() : this.tokens.metric('spacing.1') })
      .backgroundColor(this.tokens.color(this.disabled ? 'surface-sunken' : this.focused ? 'surface' : 'surface-muted'))
      .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color(this.focused ? 'primary.600' : 'border-strong'), radius: this.tokens.metric('control.field.radius') })
      .onAreaChange((_previous: Area, next: Area) => { this.fieldWidth = Number(next.width) })
      .bindPopup(this.open, { builder: this.sheet, placement: Placement.BottomLeft,
        enableArrow: false, focusable: false, autoCancel: true,
        targetSpace: this.tokens.metric('spacing.1'), radius: this.tokens.metric('radius.large'),
        popupColor: this.tokens.color('surface-overlay'), backgroundBlurStyle: BlurStyle.NONE,
        onStateChange: (event: PopupStateChangeParam) => { if (!event.isVisible) this.close() } })
  }
}

@Component
export struct DazzAutocomplete {
  @Prop items: string[] = []
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop placeholder: string = ''
  @Prop empty: string = 'No matches'
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  @Prop accessibilityLabel: string = ''
  onValueChange: (value: string) => void = () => {}
  options(): DazzSelectOption[] { return this.items.map((label: string): DazzSelectOption => ({ value: label, label })) }
  build() {
    DazzCombobox({ options: this.options(), value: this.value, defaultValue: this.defaultValue,
      placeholder: this.placeholder, empty: this.empty, controlSize: this.controlSize,
      disabled: this.disabled, accessibilityLabel: this.accessibilityLabel,
      freeText: true, clearable: false, onValueChange: (value: string | null) => this.onValueChange(value ?? '') })
  }
}
