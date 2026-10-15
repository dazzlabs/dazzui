@Component
export struct DazzSplitter {
  @Prop orientation: string = 'horizontal'
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    if (this.orientation === 'vertical') {
      ColumnSplit() {
        this.content()
      }
        .resizeable(true)
        .width('100%')
        .height('100%')
    } else {
      RowSplit() {
        this.content()
      }
        .resizeable(true)
        .width('100%')
        .height('100%')
    }
  }
}

@Component
export struct DazzSplitterPanel {
  @Prop orientation: string = 'horizontal'
  @Prop defaultSize: number = 50
  @Prop minSize: number = 0
  @Prop maxSize: number = 100
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    if (this.orientation === 'vertical') {
      Column() {
        this.content()
      }
        .width('100%')
        .height(this.defaultSize + '%')
        .constraintSize({
          minHeight: this.minSize + '%',
          maxHeight: this.maxSize + '%',
        })
    } else {
      Column() {
        this.content()
      }
        .height('100%')
        .width(this.defaultSize + '%')
        .constraintSize({
          minWidth: this.minSize + '%',
          maxWidth: this.maxSize + '%',
        })
    }
  }
}
