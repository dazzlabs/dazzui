import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzCheckbox {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop checked: boolean | undefined = undefined
  @Prop defaultChecked: boolean = false
  @Prop disabled: boolean = false
  @Prop label: string = ''
  @Prop note: string = ''
  @Prop tint: DazzTint = 'primary'
  @Prop controlSize: string = 'medium'
  @State localChecked: boolean = false
  onCheckedChange: (checked: boolean) => void = () => {}
  aboutToAppear(): void {
    this.localChecked = this.defaultChecked
  }
  change(value: boolean): void {
    if (this.disabled) return
    if (this.checked === undefined) this.localChecked = value
    this.onCheckedChange(value)
  }
  build() {
    Row({ space: this.tokens.metric('spacing.2.5') }) {
      Checkbox()
        .select(this.checked ?? this.localChecked)
        .shape(CheckBoxShape.ROUNDED_SQUARE)
        .width(this.tokens.metric('checkbox.' + this.controlSize + '.box'))
        .height(this.tokens.metric('checkbox.' + this.controlSize + '.box'))
        .margin(0)
        .padding(0)
        .selectedColor(
          this.tokens.paint(
            'filled',
            'surface',
            this.disabled ? 'disabled' : 'normal',
            this.tint
          )
        )
        .unselectedColor(this.tokens.color('border-muted'))
        .mark({
          strokeColor: this.tokens.paint(
            'filled',
            'content',
            'normal',
            this.tint
          ),
          strokeWidth: this.tokens.metric('checkbox.thickness'),
        })
        .enabled(!this.disabled)
        .accessibilityText(this.label)
        .onChange((checked: boolean) => this.change(checked))
      if (this.label) {
        Text(this.label + (this.note ? ' ' + this.note : ''))
          .fontSize(this.tokens.type('labelSmall').size)
          .fontWeight(
            this.tokens.type(
              (this.checked ?? this.localChecked) ? 'labelSmall' : 'labelQuiet'
            ).weight
          )
          .fontColor(
            this.tokens.color(
              this.disabled
                ? 'content-faint'
                : (this.checked ?? this.localChecked)
                  ? 'content'
                  : 'content-muted'
            )
          )
          .onClick(() => this.change(!(this.checked ?? this.localChecked)))
      }
    }
  }
}
