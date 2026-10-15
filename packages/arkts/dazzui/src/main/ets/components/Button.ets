import { DazzTokens } from '../foundation/Tokens'
import {
  DazzTint,
  DazzVariant,
  DazzSize,
  DazzState,
} from '../foundation/TokenTypes'

@Component
export struct DazzButton {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop variant: DazzVariant = 'recessed'
  @Prop tint: DazzTint = 'primary'
  @Prop controlSize: DazzSize = 'small'
  @Prop disabled: boolean = false
  @Prop fullWidth: boolean = false
  @Prop shortcut: string = ''
  @Prop square: boolean = false
  @Prop accessibilityLabel: string = ''
  @BuilderParam content: () => void = this.empty
  onAction: () => void = () => {}
  @State hovered: boolean = false
  @State pressed: boolean = false
  @State focused: boolean = false
  @LocalBuilder empty() {}
  state(): DazzState {
    return this.disabled
      ? 'disabled'
      : this.pressed
        ? 'pressed'
        : this.hovered
          ? 'hovered'
          : 'normal'
  }

  build() {
    Button({ type: ButtonType.Normal }) {
      Row({ space: this.tokens.size(this.controlSize, 'gap') }) {
        this.content()
        if (this.label) {
          Text(this.label)
            .fontSize(this.tokens.label(this.controlSize).size)
            .fontWeight(
              this.variant === 'filled' && !this.disabled
                ? this.tokens.type('labelStrong').weight
                : this.tokens.label(this.controlSize).weight
            )
            .fontColor(
              this.tokens.paint(
                this.variant,
                'content',
                this.state(),
                this.tint
              )
            )
            .maxLines(1)
        }
        if (this.shortcut) {
          Text(this.shortcut)
            .fontSize(this.tokens.label(this.controlSize).size)
            .fontColor(
              this.tokens.paint(
                this.variant,
                'content',
                this.state(),
                this.tint
              )
            )
            .opacity(0.7)
        }
      }
    }
      .height(this.tokens.size(this.controlSize))
      .width(
        this.square
          ? this.tokens.size(this.controlSize)
          : this.fullWidth
            ? '100%'
            : 'auto'
      )
      .padding({
        left: this.square ? 0 : this.tokens.size(this.controlSize, 'padding.inline'),
        right: this.square ? 0 : this.tokens.size(this.controlSize, 'padding.inline'),
        top: 0,
        bottom: 0,
      })
      .backgroundColor(
        this.tokens.paint(this.variant, 'surface', this.state(), this.tint)
      )
      .border({
        radius: this.tokens.size(this.controlSize, 'radius'),
        width: this.tokens.metric('stroke.hairline'),
        color: this.tokens.paint(
          this.variant,
          'border',
          this.state(),
          this.tint
        ),
      })
      .outline({
        width:
          this.focused && !this.disabled
            ? this.tokens.metric('focus.width')
            : 0,
        color: this.tokens.focus(),
        radius: this.tokens.size(this.controlSize, 'radius'),
      })
      .enabled(!this.disabled)
      .stateEffect(false)
      .accessibilityText(this.accessibilityLabel || this.label)
      .onHover((hover: boolean) => {
        this.hovered = hover
        if (!hover) this.pressed = false
      })
      .onTouch((event: TouchEvent) => {
        this.pressed = event.type === TouchType.Down
      })
      .onKeyEvent((event: KeyEvent) => {
        if (event.keyCode === 2050 || event.keyCode === 2054)
          this.pressed = event.type === KeyType.Down
        return false
      })
      .onFocus(() => {
        this.focused = true
      })
      .onBlur(() => {
        this.focused = false
        this.pressed = false
      })
      .onClick(() => {
        if (!this.disabled) this.onAction()
      })
  }
}

@Component
export struct DazzIconButton {
  @Prop label: string = ''
  @Prop variant: DazzVariant = 'plain'
  @Prop tint: DazzTint = 'neutral'
  @Prop controlSize: DazzSize = 'tiny'
  @Prop disabled: boolean = false
  @BuilderParam content: () => void = this.empty
  onAction: () => void = () => {}
  @LocalBuilder empty() {}
  build() {
    DazzButton({
      variant: this.variant,
      tint: this.tint,
      controlSize: this.controlSize,
      disabled: this.disabled,
      square: true,
      accessibilityLabel: this.label,
      content: this.forwardContent,
      onAction: this.onAction,
    })
  }

  @LocalBuilder forwardContent() {
    this.content()
  }
}

@Component
export struct DazzToggle {
  @Prop pressed: boolean | undefined = undefined
  @Prop defaultPressed: boolean = false
  @Prop label: string = ''
  @Prop controlSize: DazzSize = 'medium'
  @Prop variant: DazzVariant = 'normal'
  @Prop tint: DazzTint = 'primary'
  @Prop disabled: boolean = false
  @State localPressed: boolean = false
  @BuilderParam content: () => void = this.empty
  onPressedChange: (value: boolean) => void = () => {}
  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.localPressed = this.defaultPressed
  }
  build() {
    DazzButton({
      label: this.label,
      controlSize: this.controlSize,
      variant: (this.pressed ?? this.localPressed) ? 'tinted' : this.variant,
      tint: this.tint,
      disabled: this.disabled,
      content: this.forwardContent,
      onAction: () => {
        const next = !(this.pressed ?? this.localPressed)
        if (this.pressed === undefined) this.localPressed = next
        this.onPressedChange(next)
      },
    }).accessibilitySelected(this.pressed ?? this.localPressed)
  }

  @LocalBuilder forwardContent() {
    this.content()
  }
}
