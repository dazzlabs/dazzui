import { hilog } from '@kit.PerformanceAnalysisKit'
import {
  DazzTheme, DazzTokens, DazzButton, DazzTextField, readSystemStyle,
  DazzSidebarProvider, DazzSidebar, DazzSidebarContent, DazzSidebarGroup,
  DazzSidebarMenu, DazzSidebarMenuButton,
} from '@dazzlabs/dazzui'

@Component
struct Specimen {
  @Consume('dazzui.theme') tokens: DazzTokens
  @LocalStorageProp('runId') runId: string = ''
  @Prop specimenId: string = 'adapted'
  @Prop native: boolean = false
  private sequence: number = 0
  private timer: number = -1
  aboutToAppear(): void {
    this.timer = setTimeout(() => this.verify(), 400)
  }
  aboutToDisappear(): void { clearTimeout(this.timer) }
  verify(): void {
    const style = readSystemStyle(this.getUIContext())
    const system = this.tokens.systemStyle !== undefined
    if (system) {
      const native = this.getUIContext().getComponentUtils().getRectangleById('native-button')
      const adapted = this.getUIContext().getComponentUtils().getRectangleById('adapted-button')
      const design = this.getUIContext().getComponentUtils().getRectangleById('design-button')
      const text = this.getUIContext().getComponentUtils().getRectangleById('adapted-text')
      const designText = this.getUIContext().getComponentUtils().getRectangleById('design-text')
      const side = this.getUIContext().getComponentUtils().getRectangleById('adapted-sidebar')
      const row = this.getUIContext().getComponentUtils().getRectangleById('adapted-sidebar-row')
      const designSide = this.getUIContext().getComponentUtils().getRectangleById('design-sidebar')
      const expected = style.buttonHeight
      const vp = this.getUIContext().px2vp(adapted.size.height)
      const nativeVp = this.getUIContext().px2vp(native.size.height)
      hilog.info(0x0000, 'DazzSystemStyleTest', 'SNAPSHOT %{public}s', JSON.stringify({
        runId: this.runId, sequence: ++this.sequence, designVp: this.getUIContext().px2vp(design.size.height),
        textVp: this.getUIContext().px2vp(text.size.height),
        designTextVp: this.getUIContext().px2vp(designText.size.height),
        family: style.fontFamily, fontFamilies: style.fontFamilies, fontId: style.fontId,
        follows: this.getUIContext().isFollowingSystemFontScale(), scale: style.fontScale,
        weightScale: style.fontWeightScale, label: this.tokens.label('medium'),
        vp, nativeVp, expected, fieldHeight: style.fieldHeight, buttonFont: style.buttonFontSize,
        design: new DazzTokens().size('medium'), surface: this.tokens.color('surface'),
        sidebarVp: this.getUIContext().px2vp(side.size.width),
        sidebarRowVp: this.getUIContext().px2vp(row.size.width),
        sidebarRowHeight: this.getUIContext().px2vp(row.size.height),
        sidebarRequiredVp: this.tokens.size('medium') + 2 * this.tokens.metric('spacing.2.5'),
        designSidebarVp: this.getUIContext().px2vp(designSide.size.width)
      }))
    }
    this.timer = setTimeout(() => this.verify(), 500)
  }
  @LocalBuilder sidebarRow() {
    DazzSidebarMenuButton({ label: 'Documents', icon: 'Document20Regular' })
      .id(this.specimenId + '-sidebar-row')
  }
  @LocalBuilder sidebarMenu() { DazzSidebarMenu({ content: this.sidebarRow }) }
  @LocalBuilder sidebarGroup() { DazzSidebarGroup({ content: this.sidebarMenu }) }
  @LocalBuilder sidebarBody() { DazzSidebarContent({ content: this.sidebarGroup }) }
  @LocalBuilder sidebarColumn() {
    DazzSidebar({ collapsible: 'icon', content: this.sidebarBody })
      .id(this.specimenId + '-sidebar')
  }
  build() {
    Column({ space: 16 }) {
      Text(this.specimenId).fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size))
        .fontColor(this.tokens.color('content'))
      if (this.native) { Button('Native button').id('native-button') }
      DazzButton({ label: 'System-sized button', controlSize: 'medium' }).id(this.specimenId + '-button')
      DazzTextField({ placeholder: 'Name', controlSize: 'medium' }).id(this.specimenId + '-field')
      Text('Ag国').fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size))
        .fontColor(this.tokens.color('content')).id(this.specimenId + '-text')
      DazzSidebarProvider({ defaultOpen: false, content: this.sidebarColumn })
        .width(360).height(140)
    }.width(360)
  }
}

@Component
struct FontComparison {
  @Consume('dazzui.theme') tokens: DazzTokens
  @LocalBuilder design() { Specimen({ specimenId: 'design' }) }
  build() {
    Row({ space: 24 }) {
      Specimen({ specimenId: 'adapted', native: true })
      // Consume the live palette here; a forwarded slot alone does not refresh
      // a captured entry-state value when only its outer provider updates.
      DazzTheme({ name: this.tokens.name, sizing: 'design', content: this.design })
        .width(400).height(700)
    }
  }
}

@Entry({ useSharedStorage: true })
@Component
struct SystemStyle {
  @State theme: string = 'bright-light'
  @LocalBuilder adapted() { FontComparison() }
  build() {
    Column({ space: 16 }) {
      Row({ space: 16 }) {
        Button('Scale 1').id('scale-1').onClick(() => {
          this.getUIContext().getHostContext()?.getApplicationContext().setFontSizeScale(1)
        })
        Button('Scale 2').id('scale-2').onClick(() => {
          this.getUIContext().getHostContext()?.getApplicationContext().setFontSizeScale(2)
        })
        Button('Change theme').id('change-theme').onClick(() => {
          this.theme = this.theme === 'bright-light' ? 'bright-dark' : 'bright-light'
        })
      }
      DazzTheme({ name: this.theme, sizing: 'system', content: this.adapted })
        .width(824).height(700)
    }
  }
}
