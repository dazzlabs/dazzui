import { DazzTokens } from '../foundation/Tokens'
import { DazzFormContext } from '../foundation/FormData'

@Component
export struct DazzForm {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide({ allowOverride: 'dazzui.form' }) form: DazzFormContext = new DazzFormContext()
  @BuilderParam content: () => void = this.empty
  onSubmit: () => void = () => {}
  @LocalBuilder empty() {}
  aboutToAppear(): void { this.form.active = true; this.form.onSubmit = () => this.onSubmit() }
  aboutToDisappear(): void { this.form.active = false }
  build() {
    Column({ space: this.tokens.metric('spacing.4') }) { this.content() }
      .alignItems(HorizontalAlign.Start).width('100%')
  }
}
