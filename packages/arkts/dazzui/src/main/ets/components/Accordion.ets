import { DazzTokens } from '../foundation/Tokens'
import { DazzAccordionContext, toggleSelection } from '../foundation/Selection'
import { DazzIcon } from './Icon'

let scopeID: number = 0

@Component
export struct DazzAccordion {
  @Prop @Watch('syncValue') value: string[] | undefined = undefined
  @Prop defaultValue: string[] = []
  @Prop @Watch('syncValue') divided: boolean = false
  @Prop multiple: boolean = false
  @Provide({ allowOverride: 'dazzui.accordion' }) group: DazzAccordionContext = new DazzAccordionContext()
  @BuilderParam content: () => void = this.empty
  onValueChange: (value: string[]) => void = () => {}
  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.group.focusPrefix = 'accordion-' + String(++scopeID) + '-'
    this.group.values = (this.value ?? this.defaultValue).slice()
    this.group.divided = this.divided
    this.group.toggle = (value: string) => {
      const next = toggleSelection(this.value ?? this.group.values, value, this.multiple)
      if (this.value === undefined) this.group.values = next
      this.onValueChange(next)
    }
  }
  syncValue(): void {
    if (this.value !== undefined) this.group.values = this.value.slice()
    this.group.divided = this.divided
  }
  build() { Column() { this.content() }.alignItems(HorizontalAlign.Start).width('100%') }
}

@Component
export struct DazzAccordionItem {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.accordion') group: DazzAccordionContext = new DazzAccordionContext()
  @Prop value: string = ''
  @Prop title: string = ''
  @Prop disabled: boolean = false
  @State hovered: boolean = false
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  aboutToAppear(): void { if (!this.disabled) this.group.register(this.value) }
  aboutToDisappear(): void { this.group.unregister(this.value) }
  open(): boolean { return this.group.values.includes(this.value) }
  handleKey(event: KeyEvent): void {
    if (event.type !== KeyType.Down) return
    const code = event.keyCode
    const target = code === 2012 ? this.group.next(this.value, -1) :
      code === 2013 ? this.group.next(this.value, 1) :
      code === 2081 ? this.group.headers[0] :
      code === 2082 ? this.group.headers[this.group.headers.length - 1] : ''
    if (target) {
      this.getUIContext().getFocusController().requestFocus(this.group.focusPrefix + target)
      event.stopPropagation()
    }
  }
  build() {
    Column() {
      if (this.group.divided && this.group.headers.indexOf(this.value) > 0) {
        Divider().color(this.tokens.color('border')).strokeWidth(this.tokens.metric('stroke.hairline'))
      }
      Button({ type: ButtonType.Normal }) {
        Row({ space: this.tokens.metric('spacing.2') }) {
          DazzIcon({ name: 'ChevronRight12Regular', iconSize: this.tokens.metric('icon.small'), color: this.tokens.color('content-subtle') })
            .rotate({ angle: this.open() ? 90 : 0 })
          Text(this.title).fontSize(this.tokens.fontSize(this.tokens.type('labelMedium').size))
            .lineHeight(this.tokens.fontSize(this.tokens.type('labelMedium').leading))
            .fontWeight(this.tokens.type('labelMedium').weight).fontColor(this.tokens.color(this.disabled ? 'content-faint' : 'content'))
            .layoutWeight(1).maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis })
        }.width('100%')
      }.width('100%').padding(this.tokens.metric('spacing.2'))
        .backgroundColor(this.hovered ? this.tokens.paint('normal', 'surface', 'hovered', 'neutral') : Color.Transparent)
        .borderRadius(this.tokens.metric('control.container.radius')).stateEffect(false)
        .enabled(!this.disabled).accessibilityText(this.title)
        .accessibilityDescription(this.open() ? 'Expanded' : 'Collapsed')
        .id(this.group.focusPrefix + this.value)
        .onHover((value: boolean) => { this.hovered = value })
        .onKeyEvent((event: KeyEvent) => this.handleKey(event))
        .onClick(() => this.group.toggle(this.value))
      if (this.open()) {
        Column() { this.content() }.alignItems(HorizontalAlign.Start).width('100%')
          .padding({ top: this.tokens.metric('spacing.1'), bottom: this.tokens.metric('spacing.3'),
            left: this.tokens.metric('spacing.2') * 2 + this.tokens.metric('icon.small'), right: this.tokens.metric('spacing.2') })
      }
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
}
