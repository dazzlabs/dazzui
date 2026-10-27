import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzSwitch {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop checked: boolean | undefined = undefined
  @Prop defaultChecked: boolean = false
  @Prop controlSize: string = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop disabled: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localChecked: boolean = false
  onCheckedChange: (value: boolean) => void = () => {}
  aboutToAppear(): void {
    this.localChecked = this.defaultChecked
  }
  build() {
    Toggle({ type: ToggleType.Switch, isOn: this.checked ?? this.localChecked })
      .width(this.tokens.metric('switch.' + this.controlSize + '.width'))
      .height(this.tokens.metric('switch.' + this.controlSize + '.height'))
      .selectedColor(
        this.tokens.paint(
          'filled',
          'surface',
          'normal',
          this.tint
        )
      )
      .switchPointColor(
        (this.checked ?? this.localChecked)
          ? this.tokens.paint('filled', 'content', 'normal', this.tint)
          : this.tokens.color('content-faint')
      )
      .switchStyle({
        pointRadius: this.tokens.metric('switch.' + this.controlSize + '.thumb') / 2,
        unselectedColor: this.tokens.color('surface-sunken'),
      })
      .padding(0)
      .margin(0)
      .enabled(!this.disabled)
      .accessibilityText(this.accessibilityLabel)
      .onChange((value: boolean) => {
        if (this.checked === undefined) this.localChecked = value
        this.onCheckedChange(value)
      })
  }
}
