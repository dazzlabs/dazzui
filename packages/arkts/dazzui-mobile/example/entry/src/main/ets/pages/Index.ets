import { Theme, ColorMetrics, SymbolGlyphModifier, window } from '@kit.ArkUI'
import { DazzMobileTheme, DazzThemeFamily, DAZZ_THEME_FAMILIES } from '@dazzlabs/dazzui-mobile'

function systemBarColor(value: ResourceColor): string {
  const color = ColorMetrics.resourceColor(value)
  return '#' + [color.red, color.green, color.blue]
    .map((channel: number): string => channel.toString(16).padStart(2, '0')).join('')
}

@Entry({ useSharedStorage: true })
@Component
struct Index {
  @LocalStorageProp('family') family: DazzThemeFamily = DazzThemeFamily.Bright
  @LocalStorageProp('appearance') appearance: ThemeColorMode = ThemeColorMode.SYSTEM

  @LocalBuilder
  gallery() {
    Gallery({ family: this.$family, appearance: this.$appearance })
  }

  build() {
    Column() {
      DazzMobileTheme({ family: this.family, appearance: this.appearance, content: this.gallery })
    }.width('100%').height('100%')
  }
}

@Component
struct Gallery {
  @Link family: DazzThemeFamily
  @Link appearance: ThemeColorMode
  @State pageBackground: ResourceColor = Color.Transparent
  @State secondary: ResourceColor = Color.Transparent
  @State accent: ResourceColor = Color.Transparent
  @State count: number = 0
  @State name: string = ''
  @State notes: string = ''
  @State notifications: boolean = true
  @State accepted: boolean = false
  @State delivery: string = 'standard'
  @State volume: number = 40
  @State plan: number = 0
  @State sheet: boolean = false
  @State saved: boolean = false

  onWillApplyTheme(theme: Theme): void {
    this.pageBackground = theme.colors.backgroundPrimary
    this.secondary = theme.colors.fontSecondary
    this.accent = theme.colors.fontEmphasize
    const context = this.getUIContext().getHostContext()
    if (context) {
      window.getLastWindow(context).then((appWindow: window.Window) =>
        appWindow.setWindowSystemBarProperties({
          statusBarColor: systemBarColor(theme.colors.backgroundPrimary),
          statusBarContentColor: systemBarColor(theme.colors.fontPrimary),
          navigationBarColor: systemBarColor(theme.colors.backgroundPrimary),
          navigationBarContentColor: systemBarColor(theme.colors.fontPrimary)
        })
      ).catch(() => { console.error('Cannot update gallery system bar colors') })
    }
  }

  @LocalBuilder
  heading(title: string) {
    Text(title).fontSize($r('sys.float.ohos_id_text_size_headline8'))
      .fontWeight(FontWeight.Medium).width('100%')
  }

  @LocalBuilder
  actions() {
    Scroll() {
      Column({ space: 20 }) {
        this.heading('Buttons')
        Button('Continue').buttonStyle(ButtonStyleMode.EMPHASIZED)
          .width('100%').onClick(() => this.count++)
        Button('Secondary action').buttonStyle(ButtonStyleMode.NORMAL).width('100%')
          .onClick(() => this.count++)
        Button('Text action').buttonStyle(ButtonStyleMode.TEXTUAL).width('100%')
          .onClick(() => this.count++)
        Button('Unavailable').enabled(false).width('100%')
        Text('Actions taken: ' + this.count).id('action-count')
        Divider()
        this.heading('Dialogs & sheets')
        Button('Show dialog').width('100%').onClick(() => {
          this.getUIContext().showAlertDialog({
            title: 'Save changes?', message: 'Your preferences will be saved on this device.',
            primaryButton: { value: 'Cancel', action: () => {} },
            secondaryButton: { value: 'Save', action: () => { this.saved = true } }
          })
        })
        Text(this.saved ? 'Changes saved' : 'No changes saved').id('save-status')
        Button('Open sheet').width('100%').onClick(() => { this.sheet = true })
        Button('More actions').width('100%').bindMenu([
          { value: 'Copy', action: () => { this.count++ } },
          { value: 'Share', action: () => { this.count++ } }
        ])
      }.padding(24).width('100%').alignItems(HorizontalAlign.Start)
    }.width('100%').height('100%')
  }

  @LocalBuilder
  sheetContent() {
    Column({ space: 20 }) {
      this.heading('Make room for what matters')
      Text('A native HarmonyOS sheet, using the selected DazzUI palette.')
      Button('Done').width('100%').onClick(() => { this.sheet = false })
    }.padding(24).width('100%')
  }

  @LocalBuilder
  inputs() {
    Scroll() {
      Column({ space: 20 }) {
        this.heading('Text input')
        Search({ placeholder: 'Search the library' }).searchButton('Search')
        TextInput({ text: this.name, placeholder: 'Your name' })
          .id('name-input').onChange(value => { this.name = value })
        Text(this.name.length ? 'Hello, ' + this.name : 'Add your name to say hello').id('name-status')
        TextInput({ placeholder: 'Password' }).type(InputType.Password)
        TextArea({ text: this.notes, placeholder: 'Add a note' })
          .onChange(value => { this.notes = value })
        Divider()
        this.heading('Selection')
        Row() {
          Text('Notifications').layoutWeight(1)
          Toggle({ type: ToggleType.Switch, isOn: this.notifications })
            .id('notification-toggle').onChange(value => { this.notifications = value })
        }.width('100%')
        Text(this.notifications ? 'Notifications on' : 'Notifications off').id('notification-status')
        Row({ space: 8 }) {
          Checkbox({ name: 'terms', group: 'preferences' }).select(this.accepted)
            .id('terms-checkbox').onChange(value => { this.accepted = value })
          Text('I agree to the terms')
        }
        Row({ space: 8 }) {
          Radio({ value: 'standard', group: 'delivery' }).checked(this.delivery === 'standard')
            .onChange(checked => { if (checked) this.delivery = 'standard' })
          Text('Standard')
          Radio({ value: 'express', group: 'delivery' }).checked(this.delivery === 'express')
            .onChange(checked => { if (checked) this.delivery = 'express' })
          Text('Express')
        }
        Select([{ value: 'Personal' }, { value: 'Team' }, { value: 'Enterprise' }])
          .arrowModifier(new SymbolGlyphModifier($r('sys.symbol.chevron_down')).fontColor([this.secondary]))
          .selected(this.plan).value(['Personal', 'Team', 'Enterprise'][this.plan])
          .onSelect(index => { this.plan = index })
        Text('Volume: ' + this.volume)
        Slider({ value: this.volume, min: 0, max: 100, step: 1 })
          .width('100%').onChange(value => { this.volume = value })
        Toggle({ type: ToggleType.Button, isOn: false }) { Text('Favorite') }
      }.padding(24).width('100%').alignItems(HorizontalAlign.Start)
    }.width('100%').height('100%')
  }

  @LocalBuilder
  feedback() {
    Scroll() {
      Column({ space: 20 }) {
        this.heading('Progress')
        Progress({ value: this.volume, total: 100, type: ProgressType.Linear }).width('100%')
        Row({ space: 32 }) {
          Progress({ value: this.volume, total: 100, type: ProgressType.Ring })
          LoadingProgress().width(32).height(32)
        }
        Slider({ value: this.volume, min: 0, max: 100 }).width('100%')
          .onChange(value => { this.volume = Math.round(value) })
        Text('Completed ' + this.volume + '%')
        Divider()
        this.heading('Date & time')
        DatePicker({ selected: new Date('2026-10-17') }).width('100%')
        TimePicker({ selected: new Date('2026-10-17T09:30:00') }).width('100%')
        Divider()
        this.heading('Content')
        Text('Native controls, familiar interactions')
          .fontSize($r('sys.float.ohos_id_text_size_headline8'))
        Text('Buttons, fields, selection, pickers and overlays keep the HarmonyOS experience. Choose a family above to explore the palette.')
          .fontColor(this.secondary)
      }.padding(24).width('100%').alignItems(HorizontalAlign.Start)
    }.width('100%').height('100%')
  }

  build() {
    Navigation() {
      Column({ space: 12 }) {
        Row({ space: 12 }) {
          Select(DAZZ_THEME_FAMILIES.map((family: DazzThemeFamily): SelectOption => ({ value: family })))
            .arrowModifier(new SymbolGlyphModifier($r('sys.symbol.chevron_down')).fontColor([this.secondary]))
            .value(this.family).id('family-select').layoutWeight(1)
            .selected(DAZZ_THEME_FAMILIES.indexOf(this.family))
            .onSelect(index => { this.family = DAZZ_THEME_FAMILIES[index] })
          Select([{ value: 'System' }, { value: 'Light' }, { value: 'Dark' }])
            .arrowModifier(new SymbolGlyphModifier($r('sys.symbol.chevron_down')).fontColor([this.secondary]))
            .value(this.appearance === ThemeColorMode.SYSTEM ? 'System' :
              this.appearance === ThemeColorMode.LIGHT ? 'Light' : 'Dark')
            .id('appearance-select')
            .onSelect(index => {
              this.appearance = [ThemeColorMode.SYSTEM, ThemeColorMode.LIGHT, ThemeColorMode.DARK][index]
            })
        }.padding({ left: 24, right: 24 }).width('100%')
        Tabs() {
          TabContent() { this.actions() }.tabBar(SubTabBarStyle.of('Actions')
            .labelStyle({ selectedColor: this.accent, unselectedColor: this.secondary })
            .indicator({ color: this.accent }))
          TabContent() { this.inputs() }.tabBar(SubTabBarStyle.of('Inputs')
            .labelStyle({ selectedColor: this.accent, unselectedColor: this.secondary })
            .indicator({ color: this.accent }))
          TabContent() { this.feedback() }.tabBar(SubTabBarStyle.of('Explore')
            .labelStyle({ selectedColor: this.accent, unselectedColor: this.secondary })
            .indicator({ color: this.accent }))
        }.barPosition(BarPosition.Start).layoutWeight(1).width('100%')
      }.width('100%').height('100%').backgroundColor(this.pageBackground)
    }.title('DazzUI Mobile').titleMode(NavigationTitleMode.Mini)
      .hideBackButton(true).width('100%').height('100%')
      .backgroundColor(this.pageBackground)
      .bindSheet(this.sheet, this.sheetContent(), {
        height: SheetSize.MEDIUM, dragBar: true, showClose: true,
        onDisappear: () => { this.sheet = false }
      })
  }
}
