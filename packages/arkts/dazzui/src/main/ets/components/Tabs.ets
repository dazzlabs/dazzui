import { DazzTokens } from '../foundation/Tokens'
import { DazzTint, DazzSize, DazzItem } from '../foundation/TokenTypes'

@Component
export struct DazzSegmentedControl {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop items: DazzItem[] = []
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop controlSize: DazzSize = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop activeStyle: string = 'normal'
  @Prop accessibilityLabel: string = ''
  @State localValue: string = ''
  onValueChange: (value: string) => void = () => {}
  aboutToAppear(): void {
    this.localValue = this.defaultValue || this.items[0]?.value || ''
  }
  index(): number {
    return Math.max(
      0,
      this.items.findIndex(
        (item: DazzItem) => item.value === (this.value ?? this.localValue)
      )
    )
  }
  @LocalBuilder tab(item: DazzItem) {
    Text(item.label)
      .fontSize(this.tokens.label(this.controlSize).size)
      .fontWeight(this.tokens.label(this.controlSize).weight)
      .fontColor(
        this.tokens.paint(
          (this.value ?? this.localValue) === item.value
            ? this.activeStyle === 'filled'
              ? 'filled'
              : 'normal'
            : 'plain',
          'content',
          'normal',
          this.tint
        )
      )
      .padding({
        left: this.tokens.size(this.controlSize, 'padding.inline'),
        right: this.tokens.size(this.controlSize, 'padding.inline'),
      })
      .height(
        this.tokens.size(this.controlSize) -
          this.tokens.metric('segmented-control.inset') * 2
      )
      .borderRadius(this.tokens.size(this.controlSize, 'radius'))
      .backgroundColor(
        (this.value ?? this.localValue) === item.value
          ? this.tokens.paint(
              this.activeStyle === 'filled' ? 'filled' : 'normal',
              'surface',
              'normal',
              this.tint
            )
          : Color.Transparent
      )
  }
  build() {
    Tabs({ index: this.index() }) {
      ForEach(
        this.items,
        (item: DazzItem) => {
          TabContent() {}
            .tabBar(this.tab(item))
        },
        (item: DazzItem) => item.value
      )
    }
      .barMode(BarMode.Scrollable)
      .barHeight(this.tokens.size(this.controlSize))
      .height(this.tokens.size(this.controlSize))
      .backgroundColor(this.tokens.color('surface-sunken'))
      .borderRadius(this.tokens.size(this.controlSize, 'radius'))
      .scrollable(false)
      .accessibilityText(this.accessibilityLabel)
      .onChange((index: number) => {
        if (index < 0 || index >= this.items.length) return
        if (this.value === undefined) this.localValue = this.items[index].value
        this.onValueChange(this.items[index].value)
      })
  }
}
