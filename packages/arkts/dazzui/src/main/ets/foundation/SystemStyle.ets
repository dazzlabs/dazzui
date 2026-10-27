import { UIContext, font } from '@kit.ArkUI'
import { common } from '@kit.AbilityKit'
import { DazzSystemStyle } from './SystemStyleData'

/** Read actual system resources; do not infer a selected face from installed fonts. */
export function readSystemStyle(ui: UIContext): DazzSystemStyle {
  const context = ui.getHostContext() as common.UIAbilityContext
  if (!context)
    throw new Error('System sizing requires a live UIAbility context')
  const manager = context.resourceManager
  const readNumber = (resource: Resource): number => {
    const value = manager.getNumber(resource.id)
    if (!Number.isFinite(value) || value < 0)
      throw new Error('Invalid system metric: ' + resource.id)
    return value
  }
  // ResourceManager returns dimension resources in physical pixels. Recover
  // the logical resource value before applying the UI's font conversion.
  const read = (resource: Resource): number => ui.px2vp(readNumber(resource))
  const toVp = (fp: number): number => ui.px2vp(ui.fp2px(fp))
  const configuration = context.config
  const style = new DazzSystemStyle()
  style.fontId = configuration.fontId ?? ''
  style.fontScale = configuration.fontSizeScale ?? 1
  style.fontWeightScale = configuration.fontWeightScale ?? 1
  style.fontFamily = manager.getStringSync(
    $r('sys.string.ohos_id_text_font_family_regular').id
  )
  ui.runScopedTask(() => {
    const config = font.getUIFontConfig()
    style.fontFamilies =
      config?.generic.map(
        (entry: font.UIFontGenericInfo): string => entry.family
      ) ?? []
  })
  const regular = FontWeight.Normal
  const buttonWeight = readNumber($r('sys.float.ohos_id_button_font_weight'))
  const fieldWeight = readNumber($r('sys.float.ohos_id_textfield_font_weight'))
  // Native resource weights use the engine enum (W100 = 0), while ArkTS
  // fontWeight takes CSS weights. The text engine applies fontWeightScale.
  const weight = (value: number): number => {
    const native = [
      100, 200, 300, 400, 500, 600, 700, 800, 900, 700, 400, 900, 100, 500, 400,
    ]
    return value < native.length ? native[value] : value
  }
  const lineHeight = (size: number, fontWeight: number): number => {
    const measured = ui.getMeasureUtils().measureTextSize({
      textContent: 'Ag国',
      fontSize: size + 'vp',
      fontWeight,
    })
    return typeof measured.height === 'number'
      ? ui.px2vp(measured.height)
      : size
  }
  const roles: Record<string, Resource> = {
    'headlineSmall': $r('sys.float.ohos_id_text_size_headline6'),
    'headlineMedium': $r('sys.float.ohos_id_text_size_headline5'),
    'headlineLarge': $r('sys.float.ohos_id_text_size_headline4'),
    'titleSmall': $r('sys.float.ohos_id_text_size_headline9'),
    'titleMedium': $r('sys.float.ohos_id_text_size_headline8'),
    'titleLarge': $r('sys.float.ohos_id_text_size_headline7'),
    'bodySmall': $r('sys.float.ohos_id_text_size_body2'),
    'bodyMedium': $r('sys.float.ohos_id_text_size_body1'),
    'bodyLarge': $r('sys.float.ohos_id_text_size_sub_title1'),
    'labelSmall': $r('sys.float.ohos_id_text_size_button3'),
    'labelMedium': $r('sys.float.ohos_id_text_size_button2'),
    'labelLarge': $r('sys.float.ohos_id_text_size_button1'),
    'labelQuiet': $r('sys.float.ohos_id_button_font_size'),
    'labelStrong': $r('sys.float.ohos_id_button_font_size'),
    'captionSmall': $r('sys.float.ohos_id_text_size_caption'),
    'captionMedium': $r('sys.float.ohos_id_text_size_body3'),
    'captionLarge': $r('sys.float.ohos_id_text_size_body2'),
  }
  for (const id of Object.keys(roles)) {
    const size = toVp(read(roles[id]))
    const fontWeight = id.startsWith('label') ? weight(buttonWeight) : regular
    style.typography[id] = {
      size,
      leading: lineHeight(size, fontWeight),
      weight: fontWeight,
    }
  }
  style.buttonFontSize = toVp(read($r('sys.float.ohos_id_button_font_size')))
  style.smallButtonFontSize = toVp(
    read($r('sys.float.ohos_id_text_size_button2'))
  )
  style.fieldFontSize = toVp(read($r('sys.float.ohos_id_text_size_body1')))
  style.buttonFontWeight = weight(buttonWeight)
  style.fieldFontWeight = weight(fieldWeight)
  style.fieldLineHeight = lineHeight(style.fieldFontSize, style.fieldFontWeight)
  const aging = [
    readNumber($r('sys.float.ohos_id_big_font_size_scale')),
    readNumber($r('sys.float.ohos_id_large_font_size_scale')),
    readNumber($r('sys.float.ohos_id_max_font_size_scale')),
  ].some((value: number): boolean => Math.abs(style.fontScale - value) < 0.001)
  const buttonVertical = read(
    aging
      ? $r('sys.float.button_aging_normal_padding')
      : $r('sys.float.ohos_id_button_vertical_padding')
  )
  const smallButtonVertical = read(
    aging
      ? $r('sys.float.button_aging_small_padding')
      : $r('sys.float.ohos_id_button_vertical_padding')
  )
  style.buttonHeight = Math.max(
    read($r('sys.float.ohos_id_button_height')),
    lineHeight(style.buttonFontSize, style.buttonFontWeight) +
      2 * buttonVertical
  )
  style.smallButtonHeight = Math.max(
    read($r('sys.float.control_button_height_small')),
    lineHeight(style.smallButtonFontSize, style.buttonFontWeight) +
      2 * smallButtonVertical
  )
  style.buttonPadding = read($r('sys.float.ohos_id_button_horizontal_padding'))
  // ArkUI exposes a compact height and text role; its private small-button
  // horizontal padding is not a public resource, so retain the public padding.
  style.smallButtonPadding = style.buttonPadding
  style.fieldPadding = read(
    $r('sys.float.ohos_id_textfield_padding_horizontal')
  )
  style.fieldHeight = Math.max(
    read($r('sys.float.ohos_id_textfield_height')),
    style.fieldLineHeight +
      2 * read($r('sys.float.ohos_id_textfield_padding_vertical'))
  )
  style.lengthScale =
    toVp(read($r('sys.float.ohos_id_text_size_body2'))) /
    read($r('sys.float.ohos_id_text_size_body2'))
  return style
}
