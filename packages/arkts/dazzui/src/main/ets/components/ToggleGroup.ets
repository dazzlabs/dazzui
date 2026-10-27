import { DazzTokens } from '../foundation/Tokens'
import { DazzItem, DazzSize, DazzTint } from '../foundation/TokenTypes'
import { toggleSelection } from '../foundation/Selection'
import { DazzToggle } from './Button'
import { DazzIcon } from './Icon'

@Component
export struct DazzToggleGroup {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop items: DazzItem[] = []
  @Prop value: string[] | undefined = undefined
  @Prop defaultValue: string[] = []
  @Prop multiple: boolean = false
  @Prop controlSize: DazzSize = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop accessibilityLabel: string = ''
  @State localValue: string[] = []
  onValueChange: (value: string[]) => void = () => {}
  aboutToAppear(): void { this.localValue = this.defaultValue.slice() }
  @LocalBuilder glyph(item: DazzItem) {
    if (item.icon) { DazzIcon({ name: item.icon, iconSize: this.tokens.size(this.controlSize, 'icon-size'), color: this.tokens.paint((this.value ?? this.localValue).includes(item.value) ? 'tinted' : 'normal', 'content', item.disabled ? 'disabled' : 'normal', this.tint) }) }
  }
  change(item: DazzItem): void {
    if (item.disabled) return
    const next = toggleSelection(this.value ?? this.localValue, item.value, this.multiple)
    if (this.value === undefined) this.localValue = next
    this.onValueChange(next)
  }
  build() {
    Row() {
      ForEach(this.items, (item: DazzItem, index: number) => {
        DazzToggle({ text: item.label, label: item.hint ?? item.label, pressed: (this.value ?? this.localValue).includes(item.value),
          controlSize: this.controlSize, tint: this.tint, disabled: item.disabled ?? false,
          groupPosition: this.items.length === 1 ? 'only' : index === 0 ? 'first' : index === this.items.length - 1 ? 'last' : 'middle',
          content: () => { this.glyph(item) }, onPressedChange: () => this.change(item) })
          .margin({ left: index ? -this.tokens.metric('spacing.px') : 0 })
      }, (item: DazzItem) => item.value)
    }.accessibilityText(this.accessibilityLabel)
  }
}
