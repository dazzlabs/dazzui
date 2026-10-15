import {
  DazzTheme,
  DazzTokens,
  DazzButton,
  DazzSearchField,
  DazzSelect,
  DazzItem,
  DazzDivider,
} from '@dazzlabs/dazzui'
import { STORIES } from '../generated/Stories'
import { THEME_GROUPS } from '../generated/Themes'
import { Scene, Story, ThemeFamily, ThemeGroup } from '../lib/SceneTypes'
import { SceneView } from '../lib/SceneView'

@Entry
@Component
struct Index {
  @State theme: string = 'studio-light'
  @LocalBuilder gallery() {
    Gallery({ theme: $theme })
  }
  build() {
    Column() {
      DazzTheme({ name: this.theme, content: this.gallery })
    }.width('100%').height('100%')
  }
}

@Component
struct Gallery {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Link theme: string
  @State selected: string = ''
  @State query: string = ''
  @State showThemes: boolean = true

  matches(): Story[] {
    const query = this.query.trim().toLowerCase()
    return STORIES.filter(
      (story: Story) =>
        !query ||
        (story.name + ' ' + story.summary).toLowerCase().includes(query)
    )
  }
  selectedStory(): Story | undefined {
    return STORIES.find((story: Story) => story.id === this.selected)
  }
  family(): string {
    return this.theme.replace(/-(light|dark)$/, '')
  }
  dark(): boolean {
    return this.theme.endsWith('-dark')
  }
  familyItems(group: ThemeGroup): DazzItem[] {
    return group.families.map((family: ThemeFamily): DazzItem => {
      return { value: family.value, label: family.title }
    })
  }
  @LocalBuilder navigation() {
    Column({ space: this.tokens.metric('spacing.3') }) {
      Text('DazzUI')
        .fontSize(this.tokens.type('labelStrong').size)
        .fontWeight(this.tokens.type('labelStrong').weight)
        .fontColor(this.tokens.color('content'))
      DazzSearchField({
        value: this.query,
        placeholder: 'Find a component…',
        onValueChange: (value: string) => {
          this.query = value
        },
      })
      DazzButton({
        label: 'Overview',
        fullWidth: true,
        variant: this.selected ? 'plain' : 'tinted',
        onAction: () => {
          this.selected = ''
        },
      })
      Scroll() {
        Column({ space: this.tokens.metric('frame.nav-gap') }) {
          ForEach(
            this.matches(),
            (story: Story) => {
              DazzButton({
                label: story.name,
                fullWidth: true,
                variant: this.selected === story.id ? 'tinted' : 'plain',
                tint: 'neutral',
                onAction: () => {
                  this.selected = story.id
                },
              })
            },
            (story: Story) => story.id
          )
        }
          .width('100%')
      }
        .layoutWeight(1)
        .width('100%')
      Text('ArkUI · HarmonyOS Desktop')
        .fontSize(this.tokens.type('captionSmall').size)
        .fontColor(this.tokens.color('content-faint'))
    }
      .padding(this.tokens.metric('spacing.3'))
      .width(this.tokens.metric('frame.sidebar.width'))
      .height('100%')
      .backgroundColor(this.tokens.color('surface-column'))
  }
  @LocalBuilder overview() {
    Column({ space: this.tokens.metric('spacing.5') }) {
      Text('Components')
        .fontSize(this.tokens.type('labelStrong').size)
        .fontWeight(this.tokens.type('labelStrong').weight)
        .fontColor(this.tokens.color('content'))
      ForEach(
        this.matches(),
        (story: Story) => {
          Column({ space: this.tokens.metric('spacing.3') }) {
            DazzButton({
              label: story.name,
              variant: 'plain',
              tint: 'neutral',
              onAction: () => {
                this.selected = story.id
              },
            })
            Text(story.summary)
              .fontSize(this.tokens.type('bodySmall').size)
              .fontColor(this.tokens.color('content-muted'))
            ForEach(
              [story.preview],
              (scene: Scene) => {
                SceneView({ scene: scene })
              },
              (scene: Scene) => scene.id
            )
          }
            .alignItems(HorizontalAlign.Start)
            .padding(this.tokens.metric('spacing.4'))
            .width('100%')
            .border({
              width: this.tokens.metric('stroke.hairline'),
              color: this.tokens.color('border'),
              radius: this.tokens.metric('radius.large'),
            })
        },
        (story: Story) => story.id
      )
    }
      .alignItems(HorizontalAlign.Start)
      .width('100%')
  }
  @LocalBuilder document(story: Story) {
    Column({ space: this.tokens.metric('spacing.6') }) {
      Text(story.summary)
        .fontSize(this.tokens.type('bodyMedium').size)
        .fontColor(this.tokens.color('content-muted'))
      ForEach(
        story.examples,
        (scene: Scene) => {
          Column({ space: this.tokens.metric('spacing.3') }) {
            Text(scene.title)
              .fontSize(this.tokens.type('labelStrong').size)
              .fontWeight(this.tokens.type('labelStrong').weight)
              .fontColor(this.tokens.color('content'))
            if (scene.note) {
              Text(scene.note)
                .fontSize(this.tokens.type('bodySmall').size)
                .fontColor(this.tokens.color('content-muted'))
            }
            SceneView({ scene: scene })
          }
            .alignItems(HorizontalAlign.Start)
            .width('100%')
        },
        (scene: Scene) => scene.id
      )
    }
      .alignItems(HorizontalAlign.Start)
      .width('100%')
  }
  @LocalBuilder themePanel() {
    Scroll() {
      Column({ space: this.tokens.metric('spacing.4') }) {
        Text('Theme')
          .fontSize(this.tokens.type('labelStrong').size)
          .fontWeight(this.tokens.type('labelStrong').weight)
          .fontColor(this.tokens.color('content'))
        Row({ space: this.tokens.metric('spacing.2') }) {
          DazzButton({
            label: 'Light',
            variant: this.dark() ? 'plain' : 'tinted',
            onAction: () => {
              this.theme = this.family() + '-light'
            },
          })
          DazzButton({
            label: 'Dark',
            variant: this.dark() ? 'tinted' : 'plain',
            onAction: () => {
              this.theme = this.family() + '-dark'
            },
          })
        }
        ForEach(
          THEME_GROUPS,
          (group: ThemeGroup) => {
            Column({ space: this.tokens.metric('spacing.2') }) {
              Text(group.title)
                .fontSize(this.tokens.type('captionSmall').size)
                .fontColor(this.tokens.color('content-muted'))
              DazzSelect({
                options: this.familyItems(group),
                value: this.family(),
                placeholder: 'Choose a style',
                onValueChange: (value: string) => {
                  this.theme = value + (this.dark() ? '-dark' : '-light')
                },
              })
            }
              .alignItems(HorizontalAlign.Start)
              .width('100%')
          },
          (group: ThemeGroup) => group.title
        )
      }
        .alignItems(HorizontalAlign.Start)
        .padding(this.tokens.metric('spacing.4'))
        .width('100%')
    }
      .width(this.tokens.metric('frame.aside.width'))
      .height('100%')
      .backgroundColor(this.tokens.color('surface-column'))
  }
  build() {
    Row() {
      this.navigation()
      DazzDivider({ orientation: 'vertical' }).height('100%')
      Column() {
        Row({ space: this.tokens.metric('spacing.3') }) {
          Text(this.selectedStory()?.name ?? 'Overview')
            .fontSize(this.tokens.type('labelStrong').size)
            .fontWeight(this.tokens.type('labelStrong').weight)
            .fontColor(this.tokens.color('content'))
            .layoutWeight(1)
          DazzButton({
            label: 'Theme',
            variant: this.showThemes ? 'tinted' : 'plain',
            onAction: () => {
              this.showThemes = !this.showThemes
            },
          })
        }
          .padding(this.tokens.metric('spacing.3'))
          .height(this.tokens.metric('frame.titlebar.size'))
          .width('100%')
        DazzDivider()
        Scroll() {
          Column() {
            if (this.selectedStory()) {
              ForEach(
                STORIES.filter(
                  (story: Story) => story.id === this.selected
                ),
                (story: Story) => {
                  this.document(story)
                },
                (story: Story) => story.id
              )
            } else {
              this.overview()
            }
          }
            .padding(this.tokens.metric('spacing.6'))
            .width('100%')
        }
          .layoutWeight(1)
          .width('100%')
          .align(Alignment.TopStart)
      }
        .layoutWeight(1)
        .height('100%')
      if (this.showThemes) {
        DazzDivider({ orientation: 'vertical' }).height('100%')
        this.themePanel()
      }
    }
      .width('100%')
      .height('100%')
      .backgroundColor(this.tokens.color('surface'))
  }
}
