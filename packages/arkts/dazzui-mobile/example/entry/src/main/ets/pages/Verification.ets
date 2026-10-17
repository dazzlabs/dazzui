import { CustomColors, Theme, ColorMetrics } from '@kit.ArkUI'
import { ConfigurationConstant, common } from '@kit.AbilityKit'
import { hilog } from '@kit.PerformanceAnalysisKit'
import { DazzMobileTheme, dazzTheme, DazzThemeFamily, DAZZ_THEME_FAMILIES } from '@dazzlabs/dazzui-mobile'

function colors(value: CustomColors): string[] {
  return [
    value.brand ?? Color.Transparent,
    value.warning ?? Color.Transparent,
    value.alert ?? Color.Transparent,
    value.confirm ?? Color.Transparent,
    value.fontPrimary ?? Color.Transparent,
    value.fontSecondary ?? Color.Transparent,
    value.fontTertiary ?? Color.Transparent,
    value.fontFourth ?? Color.Transparent,
    value.fontEmphasize ?? Color.Transparent,
    value.fontOnPrimary ?? Color.Transparent,
    value.fontOnSecondary ?? Color.Transparent,
    value.fontOnTertiary ?? Color.Transparent,
    value.fontOnFourth ?? Color.Transparent,
    value.iconPrimary ?? Color.Transparent,
    value.iconSecondary ?? Color.Transparent,
    value.iconTertiary ?? Color.Transparent,
    value.iconFourth ?? Color.Transparent,
    value.iconEmphasize ?? Color.Transparent,
    value.iconSubEmphasize ?? Color.Transparent,
    value.iconOnPrimary ?? Color.Transparent,
    value.iconOnSecondary ?? Color.Transparent,
    value.iconOnTertiary ?? Color.Transparent,
    value.iconOnFourth ?? Color.Transparent,
    value.backgroundPrimary ?? Color.Transparent,
    value.backgroundSecondary ?? Color.Transparent,
    value.backgroundTertiary ?? Color.Transparent,
    value.backgroundFourth ?? Color.Transparent,
    value.backgroundEmphasize ?? Color.Transparent,
    value.compForegroundPrimary ?? Color.Transparent,
    value.compBackgroundPrimary ?? Color.Transparent,
    value.compBackgroundPrimaryTran ?? Color.Transparent,
    value.compBackgroundPrimaryContrary ?? Color.Transparent,
    value.compBackgroundGray ?? Color.Transparent,
    value.compBackgroundSecondary ?? Color.Transparent,
    value.compBackgroundTertiary ?? Color.Transparent,
    value.compBackgroundEmphasize ?? Color.Transparent,
    value.compBackgroundNeutral ?? Color.Transparent,
    value.compEmphasizeSecondary ?? Color.Transparent,
    value.compEmphasizeTertiary ?? Color.Transparent,
    value.compDivider ?? Color.Transparent,
    value.compCommonContrary ?? Color.Transparent,
    value.compBackgroundFocus ?? Color.Transparent,
    value.compFocusedPrimary ?? Color.Transparent,
    value.compFocusedSecondary ?? Color.Transparent,
    value.compFocusedTertiary ?? Color.Transparent
  ].map((color: ResourceColor): string => ColorMetrics.resourceColor(color).color)
}

@Component
struct Probe {
  onTheme: (value: string[]) => void = () => {}
  onWillApplyTheme(theme: Theme): void {
    this.onTheme(colors(theme.colors))
  }
  build() {
    Column() {
      Text('Native theme probe')
      Button('Continue')
      Toggle({ type: ToggleType.Switch, isOn: true })
      TextInput({ placeholder: 'Name' })
    }
  }
}

@Entry({ useSharedStorage: true })
@Component
struct Verification {
  @LocalStorageProp('runId') runId: string = ''
  @State index: number = 0
  @State report: string = 'Running'
  @State family: DazzThemeFamily = DazzThemeFamily.Studio
  @State appearance: ThemeColorMode = ThemeColorMode.LIGHT
  private outer: string[] = []
  private inner: string[] = []
  private sibling: string[] = []
  private failures: string[] = []
  private timer: number = -1
  private checked: number = 0
  private failed: number = 0
  private app?: common.ApplicationContext

  aboutToAppear(): void {
    this.app = this.getUIContext().getHostContext()?.getApplicationContext()
    this.timer = setTimeout(() => this.verify(), 500)
  }
  aboutToDisappear(): void {
    clearTimeout(this.timer)
    this.app?.setColorMode(ConfigurationConstant.ColorMode.COLOR_MODE_NOT_SET)
  }
  verify(): void {
    const explicit = DAZZ_THEME_FAMILIES.length * 2
    const dark = this.index < explicit ? this.index % 2 === 1 : this.index === explicit + 1
    const expected = dazzTheme(this.family)
    const expectedColors = colors((dark ? expected.darkColors : expected.colors)!)
    const isolated = colors(dazzTheme(DazzThemeFamily.Frost).darkColors!)
    if (JSON.stringify(this.outer) !== JSON.stringify(expectedColors) ||
      JSON.stringify(this.sibling) !== JSON.stringify(expectedColors) ||
      JSON.stringify(this.inner) !== JSON.stringify(isolated)) {
      this.failed++
      if (this.failures.length < 3) {
        this.failures.push(this.family + '/' + (dark ? 'dark' : 'light') + '/index-' + this.index +
          '/outer-' + this.outer[0] + '/inner-' + this.inner[0] + '/sibling-' + this.sibling[0])
      }
    }
    this.checked++
    if (this.index === explicit + 1) {
      this.app?.setColorMode(ConfigurationConstant.ColorMode.COLOR_MODE_NOT_SET)
      this.report = 'Checked ' + this.checked + ' scopes; failures: ' + this.failed
      hilog.info(0x0000, 'DazzMobileTest', 'DAZZ_MOBILE_VERIFY %{public}s %{public}s %{public}s', this.runId, this.report, JSON.stringify(this.failures))
      return
    }
    this.index++
    if (this.index < explicit) {
      this.family = DAZZ_THEME_FAMILIES[Math.floor(this.index / 2)]
      this.appearance = this.index % 2 === 1 ? ThemeColorMode.DARK : ThemeColorMode.LIGHT
    } else {
      this.family = DazzThemeFamily.Studio
      this.appearance = ThemeColorMode.SYSTEM
      this.app?.setColorMode(this.index === explicit ?
        ConfigurationConstant.ColorMode.COLOR_MODE_LIGHT : ConfigurationConstant.ColorMode.COLOR_MODE_DARK)
    }
    this.timer = setTimeout(() => this.verify(), 400)
  }

  @LocalBuilder
  isolated() {
    Probe({ onTheme: value => { this.inner = value } })
  }
  @LocalBuilder
  content() {
    Column({ space: 8 }) {
      Text(this.family + ' ' + this.index).id('verification-family')
      Text(this.report).id('verification-report')
      Probe({ onTheme: value => { this.outer = value } })
      DazzMobileTheme({ family: DazzThemeFamily.Frost, appearance: ThemeColorMode.DARK, content: this.isolated })
      Probe({ onTheme: value => { this.sibling = value } })
    }.width('100%')
  }
  build() {
    Column() {
      DazzMobileTheme({ family: this.family, appearance: this.appearance, content: this.content })
    }.width('100%').height('100%')
  }
}
