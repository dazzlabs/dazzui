import { DazzTokens } from '../foundation/Tokens'
import { DazzSize } from '../foundation/TokenTypes'
import { DazzButton } from './Button'
import { DazzTextField } from './TextField'

export function clampNumber(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value))
}

@Component
export struct DazzNumberField {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop @Watch('syncValue') value: number | undefined = undefined
  @Prop defaultValue: number = 0
  @Prop min: number = -Infinity
  @Prop max: number = Infinity
  @Prop step: number = 1
  @Prop unit: string = ''
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: number = 0
  @State draft: string = ''
  onValueChange: (value: number) => void = () => {}
  aboutToAppear(): void {
    this.localValue = clampNumber(this.defaultValue, this.min, this.max)
    this.syncValue()
  }
  syncValue(): void {
    this.draft = String(this.value ?? this.localValue)
  }
  commit(): void {
    if (!this.draft.trim() || !Number.isFinite(Number(this.draft))) {
      this.syncValue()
      return
    }
    this.change(Number(this.draft))
  }
  change(next: number): void {
    if (!Number.isFinite(next) || this.disabled) return
    const clamped = clampNumber(next, this.min, this.max)
    if (this.value === undefined) this.localValue = clamped
    this.draft = String(clamped)
    this.onValueChange(clamped)
  }
  build() {
    Row({ space: this.tokens.metric('spacing.1') }) {
      DazzTextField({
        value: this.draft,
        controlSize: this.controlSize,
        disabled: this.disabled,
        accessibilityLabel: this.accessibilityLabel,
        onValueChange: (text: string) => {
          // Preserve intermediate '-' and '0.' until native editing commits.
          this.draft = text
        },
        onCommit: () => this.commit(),
      }).layoutWeight(1)
      if (this.unit) {
        Text(this.unit)
          .fontSize(this.tokens.type('captionSmall').size)
          .fontColor(this.tokens.color('content-muted'))
      }
      DazzButton({
        label: '−',
        controlSize: this.controlSize,
        square: true,
        disabled:
          this.disabled || (this.value ?? this.localValue) <= this.min,
        accessibilityLabel: 'Decrease ' + this.accessibilityLabel,
        onAction: () =>
          this.change((this.value ?? this.localValue) - this.step),
      })
      DazzButton({
        label: '+',
        controlSize: this.controlSize,
        square: true,
        disabled:
          this.disabled || (this.value ?? this.localValue) >= this.max,
        accessibilityLabel: 'Increase ' + this.accessibilityLabel,
        onAction: () =>
          this.change((this.value ?? this.localValue) + this.step),
      })
    }
      .width('100%')
  }
}
