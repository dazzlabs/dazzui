import { DazzTokens } from '@dazzlabs/dazzui'

export class SceneHeaderContext { height?: number }

/** A source-authored React header style, independent of the tab contents. */
@Component
export struct SceneChrome {
  @Consume('dazzui.theme') @Watch('sync') tokens: DazzTokens
  @Prop contentWidth: number = 0
  @Prop @Watch('sync') contentHeight: number = 0
  @Prop paddingToken: string = ''
  @Prop backgroundRole: string = ''
  @Prop @Watch('sync') borderWidthToken: string = ''
  @Prop borderColorRole: string = ''
  @Provide({ allowOverride: 'dazzui.example.header' }) header: SceneHeaderContext = new SceneHeaderContext()
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  aboutToAppear(): void { this.sync() }
  headerHeight(): number {
    return this.tokens.systemStyle ? Math.max(this.contentHeight, this.tokens.size('small') + this.tokens.metric(this.borderWidthToken)) : this.contentHeight
  }
  sync(): void {
    if (!this.borderWidthToken) return
    const next = new SceneHeaderContext()
    next.height = Math.max(0, this.headerHeight() - this.tokens.metric(this.borderWidthToken))
    this.header = next
  }
  build() {
    Row() { this.content() }
      .width(this.contentWidth).constraintSize({ maxWidth: '100%' }).height(this.headerHeight())
      .padding({ left: this.tokens.metric(this.paddingToken), right: this.tokens.metric(this.paddingToken) })
      .backgroundColor(this.tokens.color(this.backgroundRole))
      .border({ width: { bottom: this.tokens.metric(this.borderWidthToken) }, color: this.tokens.color(this.borderColorRole) })
  }
}
