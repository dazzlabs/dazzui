import { LengthMetrics } from '@kit.ArkUI'
import { DazzActivationContext } from '../foundation/Activation'
import { DazzFormContext } from '../foundation/FormData'
import { DazzTokens } from '../foundation/Tokens'
import { DazzShadowLayer } from './ShadowLayer'
import { DazzButtonPaint } from './ButtonPaint'
import { focusID, rememberFocus, forgetFocus } from '../foundation/FocusData'
import { DazzIcon } from './Icon'
import { DazzTooltip } from './Tooltip'
import { DazzPointerContext } from '../foundation/Pointer'
import {
  DazzTint,
  DazzVariant,
  DazzSize,
  DazzState,
} from '../foundation/TokenTypes'

interface DazzButtonCorners {
  topLeft: number
  topRight: number
  bottomLeft: number
  bottomRight: number
}

type DazzButtonRole = 'button' | 'icon-button' | 'toggle'

@Component
export struct DazzButton {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.tabular') tabular: boolean = false
  @Consume('dazzui.activation') activation: DazzActivationContext = new DazzActivationContext()
  @Consume('dazzui.pointer') pointer: DazzPointerContext = new DazzPointerContext()
  @Consume('dazzui.form') form: DazzFormContext = new DazzFormContext()
  @Prop label: string = ''
  @Prop icon: string = ''
  @Prop trailingIcon: string = ''
  @Prop chrome: boolean = false
  // Shared native mechanics do not make IconButton and Toggle inherit the
  // filled Button's lift or emphasis. Their wrappers identify the recipe.
  @Prop controlRole: DazzButtonRole = 'button'
  @Prop latched: boolean = false
  @Prop buttonType: string = 'button'
  @Prop variant: DazzVariant = 'recessed'
  @Prop tint: DazzTint = 'primary'
  @Prop controlSize: DazzSize = 'small'
  @Prop @Watch('cancelPress') disabled: boolean = false
  @Prop fullWidth: boolean = false
  @Prop shortcut: string = ''
  @Prop square: boolean = false
  @Prop groupPosition: string = 'only'
  @Prop accessibilityLabel: string = ''
  @Prop hint: string = ''
  @Prop focusControlID: string = ''
  @Prop initialFocus: boolean = false
  @BuilderParam content: () => void = this.empty
  onAction: () => void = () => {}
  @State hovered: boolean = false
  @State pressed: boolean = false
  @State private mousePressed: boolean = false
  @State private spaceArmed: boolean = false
  @State focused: boolean = false
  @State private paintWidth: number = 0
  @State private paintHeight: number = 0
  private nativeFocusID: string = focusID('button')
  focusID(): string { return this.focusControlID || this.nativeFocusID }
  private mouseReleased: () => void = () => { this.mousePressed = false; this.pointer.offRelease(this.mouseReleased) }
  cancelPress(): void { this.pressed = false; this.spaceArmed = false; this.mouseReleased() }
  performAction(): void {
    if (this.disabled) return
    this.onAction()
    if (this.buttonType === 'submit') this.form.submit()
    else if (this.buttonType === 'reset') this.form.reset()
    this.activation.activate()
  }
  aboutToDisappear(): void { this.cancelPress(); forgetFocus(this.getUIContext().getWindowName() ?? '', this.focusID()) }
  @LocalBuilder empty() {}
  state(): DazzState {
    return this.disabled
      ? 'disabled'
      : this.pressed || this.mousePressed || this.spaceArmed
        ? 'pressed'
        : this.hovered
          ? 'hovered'
          : 'normal'
  }

  corners(): DazzButtonCorners {
    const radius = this.tokens.size(this.controlSize, 'radius')
    const start = this.groupPosition === 'only' || this.groupPosition === 'first' ? radius : 0
    const end = this.groupPosition === 'only' || this.groupPosition === 'last' ? radius : 0
    return { topLeft: start, bottomLeft: start, topRight: end, bottomRight: end }
  }
  surface(): string {
    return this.chrome && this.variant === 'plain' && this.tint === 'neutral' && this.hovered && !this.disabled
      ? this.tokens.color('surface-subtle') : this.tokens.paint(this.variant, 'surface', this.state(), this.tint)
  }
  edge(): string { return this.tokens.paint(this.variant, 'border', this.state(), this.tint) }
  paintDescriptor(): string { return JSON.stringify({ surface: this.surface(), border: this.edge(), edgeWidth: this.tokens.metric('stroke.hairline') }) }
  ink(): string {
    return this.chrome && this.variant === 'plain' && this.tint === 'neutral' && !this.disabled
      ? this.tokens.color(this.hovered ? 'content' : 'content-subtle') : this.tokens.paint(this.variant, 'content', this.state(), this.tint)
  }
  glyphSize(): number {
    if (this.square && this.controlSize === 'tiny') return this.tokens.metric('icon-button.tiny.icon-size')
    return this.tokens.size(this.controlSize, 'icon-size') - (this.label ? this.tokens.metric('control.label-glyph.shrink') : 0)
  }
  strongLabel(): boolean {
    return !this.disabled && (this.controlRole === 'button' && this.variant === 'filled' ||
      this.controlRole === 'toggle' && this.latched)
  }
  elevated(): boolean { return this.controlRole === 'button' && this.variant === 'filled' && !this.disabled }
  build() {
    Stack({ alignContent: Alignment.TopStart }) {
      if (this.elevated() && this.paintWidth > 0 && this.paintHeight > 0) {
        DazzShadowLayer({ paintWidth: this.paintWidth, paintHeight: this.paintHeight,
          corners: [this.corners().topLeft, this.corners().topRight, this.corners().bottomRight, this.corners().bottomLeft],
          shadowLayers: this.tokens.shadow('shadow.2xs') })
          .position({ x: 0, y: 0 }).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      }
      if (this.paintWidth > 0 && this.paintHeight > 0) {
        DazzButtonPaint({ paintWidth: this.paintWidth, paintHeight: this.paintHeight, anchorID: this.focusID(),
          corners: [this.corners().topLeft, this.corners().topRight, this.corners().bottomRight, this.corners().bottomLeft],
          surfaceColor: this.surface(), edgeColor: this.edge(), edgeWidth: this.tokens.metric('stroke.hairline'), hasFocus: this.focused && !this.disabled })
          .position({ x: 0, y: 0 }).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      }
      Button({ type: ButtonType.Normal }) {
        Row({ space: this.tokens.size(this.controlSize, 'gap') }) {
          if (this.icon) { DazzIcon({ name: this.icon, iconSize: this.glyphSize(), color: this.ink() }) }
          this.content()
          if (this.label) {
            Text(this.label)
              .id(this.focusID() + '-label')
              .fontFeature(this.tabular ? '"tnum" on' : 'normal')
              .fontSize(this.tokens.fontSize(this.tokens.label(this.controlSize).size))
              .fontWeight(
                this.strongLabel()
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
          if (this.trailingIcon) { DazzIcon({ name: this.trailingIcon, iconSize: this.glyphSize(), color: this.ink() }) }
          if (this.shortcut) {
            Text(this.shortcut)
              .fontSize(this.tokens.fontSize(this.tokens.label(this.controlSize).size))
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
        .id(this.focusID())
        .onAreaChange((_old: Area, area: Area) => {
          this.paintWidth = Number(area.width); this.paintHeight = Number(area.height)
        })
        .defaultFocus(this.initialFocus)
        .focusOnTouch(true)
        .height(this.tokens.size(this.controlSize))
        .width(
          this.square
            ? this.tokens.size(this.controlSize)
            : this.fullWidth
              ? '100%'
              : LayoutPolicy.wrapContent
        )
        .padding({
          left: this.square ? 0 : this.tokens.size(this.controlSize, 'padding.inline') - (this.label && this.icon ? this.tokens.metric('control.label-glyph.inset') : 0),
          right: this.square ? 0 : this.tokens.size(this.controlSize, 'padding.inline') - (this.label && this.trailingIcon ? this.tokens.metric('control.label-glyph.inset') : 0),
          top: 0,
          bottom: 0,
        })
        .backgroundColor(Color.Transparent)
        .customProperty('dazzui.paint', this.paintDescriptor())
        .shadow({ radius: 0, color: Color.Transparent })
        .border({
          radius: this.corners(),
          width: this.tokens.metric('stroke.hairline'),
          color: Color.Transparent,
        })
        // The ds layer owns focus paint; the default system box
        // otherwise adds an opaque blue ring over its translucent stroke.
        .focusBox({ strokeWidth: LengthMetrics.vp(0) })
        .enabled(!this.disabled)
        .stateEffect(false)
        .accessibilityText(this.accessibilityLabel || this.label)
        .accessibilityDescription(this.hint)
        .onHover((hover: boolean) => {
          this.hovered = hover
        })
        .onTouch((event: TouchEvent) => {
          if (event.source === SourceType.Mouse) {
            if (event.type === TouchType.Up) this.mouseReleased()
            return
          }
          if (event.type === TouchType.Down) this.pressed = true
          else if (event.type === TouchType.Up || event.type === TouchType.Cancel) this.pressed = false
        })
        .onMouse((event: MouseEvent) => {
          if (event.action === MouseAction.Press && event.button === MouseButton.Left) {
            this.mousePressed = true
            this.pointer.onRelease(this.mouseReleased)
            this.getUIContext().getFocusController().requestFocus(this.focusID())
          } else if (event.action === MouseAction.Release && event.button === MouseButton.Left) this.mouseReleased()
        })
        .onKeyPreIme((event: KeyEvent): boolean => {
          // Native traversal reads keyboard mode before onKeyEvent runs.
          // Activate it here so the first Tab after touch advances once.
          if (event.type === KeyType.Down && event.keyCode === 2049)
            this.getUIContext().getFocusController().activate(true)
          return false
        })
        .onKeyEvent((event: KeyEvent) => {
          if (event.type === KeyType.Down) {
            this.getUIContext().getFocusController().activate(true)
            this.focused = true
          }
          // HTML buttons arm Space on down and activate once on release.
          // Consume native down activation, including repeat events. Enter
          // keeps native down/repeat activation without a held Space face.
          if (event.keyCode === 2050) {
            const armed = this.spaceArmed
            this.spaceArmed = event.type === KeyType.Down
            if (event.type === KeyType.Up && armed) this.performAction()
            return true
          }
          return false
        })
        .onFocus(() => {
          rememberFocus(this.getUIContext().getWindowName() ?? '', this.focusID())
          this.focused = this.getUIContext().getFocusController().isActive()
        })
        .onBlur(() => {
          forgetFocus(this.getUIContext().getWindowName() ?? '', this.focusID())
          this.focused = false
          this.cancelPress()
        })
        .onClick(() => this.performAction())
    }.width(this.square ? this.tokens.size(this.controlSize) : this.fullWidth ? '100%' : LayoutPolicy.wrapContent)
      .height(this.tokens.size(this.controlSize)).clip(false)
  }
}

@Component
export struct DazzIconButton {
  @Prop icon: string = ''
  @Prop label: string = ''
  @Prop hint: string = ''
  @Prop variant: DazzVariant = 'plain'
  @Prop tint: DazzTint = 'neutral'
  @Prop controlSize: DazzSize = 'tiny'
  @Prop disabled: boolean = false
  @Prop focusControlID: string = ''
  @Prop initialFocus: boolean = false
  @BuilderParam content: () => void = this.empty
  onAction: () => void = () => {}
  @LocalBuilder empty() {}
  @LocalBuilder button() {
    DazzButton({
      controlRole: 'icon-button',
      variant: this.variant,
      tint: this.tint,
      controlSize: this.controlSize,
      disabled: this.disabled,
      focusControlID: this.focusControlID,
      initialFocus: this.initialFocus,
      square: true,
      icon: this.icon,
      chrome: true,
      accessibilityLabel: this.label,
      hint: this.hint,
      content: this.forwardContent,
      onAction: this.onAction,
    })
  }

  build() { DazzTooltip({ label: this.label, disabled: !this.label, content: this.button }) }

  @LocalBuilder forwardContent() {
    this.content()
  }
}

@Component
export struct DazzToggle {
  @Prop pressed: boolean | undefined = undefined
  @Prop defaultPressed: boolean = false
  @Prop label: string = ''
  @Prop text: string = ''
  @Prop groupPosition: string = 'only'
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
      controlRole: 'toggle',
      latched: this.pressed ?? this.localPressed,
      label: this.text,
      accessibilityLabel: this.label || this.text,
      square: !this.text,
      groupPosition: this.groupPosition,
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
