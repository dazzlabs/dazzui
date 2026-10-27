import { DazzTypography } from './TokenTypes'

export type DazzSizing = 'design' | 'system'

/** A runtime snapshot of this machine's style, independent of any ds palette. */
export class DazzSystemStyle {
  fontId: string = ''
  fontFamily: string = ''
  fontFamilies: string[] = []
  fontScale: number = 1
  fontWeightScale: number = 1
  typography: Record<string, DazzTypography> = {}
  buttonHeight: number = 0
  smallButtonHeight: number = 0
  buttonPadding: number = 0
  smallButtonPadding: number = 0
  fieldHeight: number = 0
  fieldPadding: number = 0
  buttonFontSize: number = 0
  smallButtonFontSize: number = 0
  fieldFontSize: number = 0
  fieldLineHeight: number = 0
  buttonFontWeight: number = 0
  fieldFontWeight: number = 0
  lengthScale: number = 1

  copy(): DazzSystemStyle {
    const result = new DazzSystemStyle()
    result.fontId = this.fontId
    result.fontFamily = this.fontFamily
    result.fontScale = this.fontScale
    result.fontWeightScale = this.fontWeightScale
    result.buttonHeight = this.buttonHeight
    result.smallButtonHeight = this.smallButtonHeight
    result.buttonPadding = this.buttonPadding
    result.smallButtonPadding = this.smallButtonPadding
    result.fieldHeight = this.fieldHeight
    result.fieldPadding = this.fieldPadding
    result.buttonFontSize = this.buttonFontSize
    result.smallButtonFontSize = this.smallButtonFontSize
    result.fieldFontSize = this.fieldFontSize
    result.fieldLineHeight = this.fieldLineHeight
    result.buttonFontWeight = this.buttonFontWeight
    result.fieldFontWeight = this.fieldFontWeight
    result.lengthScale = this.lengthScale
    result.fontFamilies = this.fontFamilies.slice()
    for (const id of Object.keys(this.typography)) {
      const type = this.typography[id]
      result.typography[id] = {
        size: type.size,
        leading: type.leading,
        weight: type.weight,
      }
    }
    return result
  }

  metric(id: string, base: number): number {
    const calendar = /^calendar\.(small|medium)\.cell-size$/.exec(id)
    if (calendar) return calendar[1] === 'small' ? this.smallButtonHeight : this.buttonHeight
    const height = /^control\.(tiny|small|medium|large)\.size$/.exec(id)
    if (height)
      return height[1] === 'tiny' || height[1] === 'small'
        ? this.smallButtonHeight
        : this.buttonHeight
    const padding =
      /^control\.(tiny|small|medium|large)\.padding\.inline$/.exec(id)
    if (padding)
      return padding[1] === 'tiny' || padding[1] === 'small'
        ? this.smallButtonPadding
        : this.buttonPadding
    // Like Omarchy: type, spacing, glyphs and available space grow together.
    // Corners, strokes, alpha, weights and durations do not scale.
    if (
      /^(spacing\.|icon\.)/.test(id) ||
      /^tabs\.(icon-size|close-icon-size)$/.test(id) || id === 'search-field.input-width' ||
      /^frame\..*(width|size|gap)$/.test(id) ||
      /^(icon-button\.[^.]+\.icon-size|checkbox\.[^.]+\.box|radio\.[^.]+\.(box|dot)|slider\.[^.]+\.(thumb|track)|calendar\.(cell-gap|[^.]+\.cell-size)|date-range-picker\.(side-width|trigger-min-width|time-min-width)|preview-card\.width|preferences\.[^.]+-(gap|padding))$/.test(
        id
      ) ||
      /^control\.[^.]+\.(gap|icon-size|padding\.block)$/.test(id) ||
      /^(dialog\.width|drawer\.size|menu\.(min-width|item-padding)|preferences\.(width|row-gap)|switch\.[^.]+\.(width|height|thumb)|segmented-control\.inset|shortcut-recorder\.width|toast\.max-width)$/.test(
        id
      )
    ) {
      return base * this.lengthScale
    }
    return base
  }
}
