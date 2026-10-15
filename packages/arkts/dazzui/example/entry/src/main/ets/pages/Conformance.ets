import { common } from '@kit.AbilityKit'
import { fileIo } from '@kit.CoreFileKit'
import { image } from '@kit.ImageKit'
import { DazzTheme, DazzTokens, DazzButton } from '@dazzlabs/dazzui'
import { STORIES } from '../generated/Stories'
import { Scene } from '../lib/SceneTypes'
import { SceneView } from '../lib/SceneView'
import { captureBoxes, capturePixels } from '../lib/Conformance'

@Entry({ useSharedStorage: true })
@Component
struct Conformance {
  @LocalStorageProp('themeName') themeName: string = 'studio-light'
  @LocalStorageProp('sceneId') sceneId: string = 'button/preview'
  @LocalBuilder specimen() {
    ConformanceScene({ sceneId: this.sceneId, themeName: this.themeName })
  }
  build() {
    Column() {
      DazzTheme({ name: this.themeName, content: this.specimen })
    }.width('100%').height('100%')
  }
}

@Component
struct ConformanceScene {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop sceneId: string = 'button/preview'
  @Prop themeName: string = 'studio-light'
  @State status: string = ''
  scene(): Scene | undefined {
    for (const story of STORIES) {
      if (story.preview.id === this.sceneId) return story.preview
      for (const scene of story.examples)
        if (scene.id === this.sceneId) return scene
    }
    return undefined
  }
  write(path: string, data: string | ArrayBuffer): void {
    const file = fileIo.openSync(
      path,
      fileIo.OpenMode.CREATE |
        fileIo.OpenMode.WRITE_ONLY |
        fileIo.OpenMode.TRUNC
    )
    try {
      fileIo.writeSync(file.fd, data)
    } finally {
      fileIo.closeSync(file)
    }
  }
  async capture(): Promise<void> {
    const scene = this.scene()
    if (!scene) return
    const ui = this.getUIContext()
    const context = ui.getHostContext() as common.UIAbilityContext
    const stem =
      context.filesDir +
      '/harmony-' +
      this.themeName +
      '-' +
      this.sceneId.replaceAll('/', '-')
    try {
      const boxes = captureBoxes(ui, scene, this.themeName)
      this.write(stem + '.json', JSON.stringify(boxes, null, 2))
      if (!scene.gaps.length) {
        const pixels = await capturePixels(ui)
        const packer = image.createImagePacker()
        try {
          this.write(
            stem + '.png',
            await packer.packing(pixels, { format: 'image/png', quality: 100 })
          )
        } finally {
          await packer.release()
          await pixels.release()
        }
      }
      this.status = stem
    } catch (error) {
      this.status = 'Capture failed: ' + String(error)
    }
  }
  build() {
    Column({ space: this.tokens.metric('spacing.4') }) {
      Column() {
        if (this.scene()) {
          ForEach(
            [this.scene() as Scene],
            (scene: Scene) => {
              SceneView({ scene: scene })
            },
            (scene: Scene) => scene.id
          )
        } else {
          Text('Unknown scene: ' + this.sceneId)
        }
      }
        .width(this.tokens.metric('spacing.16') * 10)
        .padding(this.tokens.metric('spacing.6'))
        .backgroundColor(this.tokens.color('surface'))
        .id('dazz-scene-root')
      DazzButton({
        label: 'Capture geometry and image',
        onAction: () => {
          this.capture()
        },
      })
      Text(this.status)
        .fontSize(this.tokens.type('captionSmall').size)
        .fontColor(this.tokens.color('content-muted'))
        .copyOption(CopyOptions.LocalDevice)
    }
      .padding(this.tokens.metric('spacing.6'))
      .alignItems(HorizontalAlign.Start)
  }
}
