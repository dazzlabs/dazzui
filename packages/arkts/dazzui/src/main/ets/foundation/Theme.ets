import { DazzTokens } from './Tokens'

@Component
export struct DazzTheme {
  @Prop @Watch('themeChanged') name: string = 'studio-light'
  @Provide('dazzui.theme') tokens: DazzTokens = new DazzTokens()
  @BuilderParam content: () => void = this.empty

  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.themeChanged()
  }
  themeChanged(): void {
    this.tokens = new DazzTokens(this.name)
  }

  build() {
    Column() {
      this.content()
    }
      .alignItems(HorizontalAlign.Start)
      .backgroundColor(this.tokens.color('surface'))
      .width('100%')
      .height('100%')
  }
}
