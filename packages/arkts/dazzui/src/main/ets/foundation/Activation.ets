/** A slot control forwards native activation to the boundary that owns it. */
export class DazzActivationContext {
  activate: () => void = () => {}
  focusVisible: boolean = false
}

@Component
export struct DazzActivationBoundary {
  @Prop focusKey: string = ''
  @Provide({ allowOverride: 'dazzui.activation' }) activation: DazzActivationContext = new DazzActivationContext()
  @BuilderParam content: () => void = this.empty
  onAction: (focusVisible: boolean) => void = (_focusVisible: boolean) => {}
  @LocalBuilder empty() {}
  aboutToAppear(): void { this.activation.activate = () => this.onAction(this.activation.focusVisible) }
  build() {
    Row() { this.content() }.id(this.focusKey)
      // Touching an already focused trigger does not create another focus
      // event. Preserve that appearance while its menu temporarily owns focus.
      .onFocus(() => { this.activation.focusVisible = this.getUIContext().getFocusController().isActive() })
      .onBlur(() => { this.activation.focusVisible = false })
  }
}
