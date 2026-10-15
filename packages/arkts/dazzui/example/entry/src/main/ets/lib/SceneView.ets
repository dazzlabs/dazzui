import { DazzTokens } from '@dazzlabs/dazzui'
import { Scene } from './SceneTypes'
import { SceneState } from './SceneState'
import { SceneNode } from './SceneNode'

@Component
export struct SceneView {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop scene: Scene
  @State revision: number = 0
  @State state: SceneState = new SceneState()
  aboutToAppear(): void {
    this.state = new SceneState(this.scene.initial)
  }
  build() {
    if (this.scene.gaps.length) {
      Column({ space: this.tokens.metric('spacing.2') }) {
        Text('Mapping gaps')
          .fontSize(this.tokens.type('labelSmall').size)
          .fontWeight(this.tokens.type('labelStrong').weight)
          .fontColor(this.tokens.color('warning.700'))
        ForEach(
          this.scene.gaps,
          (gap: string) => {
            Text(gap)
              .fontSize(this.tokens.type('bodySmall').size)
              .fontColor(this.tokens.color('content-muted'))
          },
          (gap: string) => gap
        )
      }
        .alignItems(HorizontalAlign.Start)
        .padding(this.tokens.metric('spacing.4'))
        .width('100%')
        .backgroundColor(this.tokens.color('surface-muted'))
        .borderRadius(this.tokens.metric('control.container.radius'))
    } else {
      SceneNode({
        node: this.scene.content,
        state: this.state,
        revision: $revision,
        sceneId: this.scene.id,
      })
    }
  }
}
