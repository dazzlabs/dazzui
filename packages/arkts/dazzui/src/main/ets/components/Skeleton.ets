import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzSkeleton {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop contentWidth: Length = '100%'
  @Prop contentHeight: Length = '100%'
  @Prop shape: string = 'rect'
  build() {
    Row()
      .width(this.contentWidth)
      .height(this.contentHeight)
      .backgroundColor(this.tokens.color('surface-sunken'))
      .borderRadius(
        this.tokens.metric(
          this.shape === 'circle' ? 'radius.full' : 'radius.small'
        )
      )
      .accessibilityLevel('no')
  }
}

@Component
export struct DazzSkeletonText {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop lines: number = 3
  lengths(): number[] {
    const indices: number[] = []
    for (let i = 0; i < Math.max(1, this.lines); i++) indices.push(i)
    return indices
  }
  build() {
    Column({ space: this.tokens.metric('spacing.2') }) {
      ForEach(
        this.lengths(),
        (value: number, index: number) => {
          DazzSkeleton({
            contentHeight: this.tokens.type('bodySmall').size,
            contentWidth: index === this.lines - 1 ? '75%' : '100%',
          })
        },
        (value: number, index: number) => String(index)
      )
    }
      .width('100%')
  }
}
