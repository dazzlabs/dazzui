import { BASE, THEMES, THEME_NAMES } from '../generated/ThemeData'
import {
  DazzTint,
  DazzVariant,
  DazzPart,
  DazzState,
  DazzSize,
  DazzTypography,
  DazzShadow,
  DazzPaint,
} from './TokenTypes'
import { DazzSystemStyle } from './SystemStyleData'

export { THEME_NAMES }

// Copy each map: nested theme scopes and host overrides never mutate the
// generated registry or another window's theme.
function merge<T>(
  base: Record<string, T>,
  overrides: Record<string, T>
): Record<string, T> {
  const result: Record<string, T> = {}
  for (const key of Object.keys(base)) result[key] = base[key]
  for (const key of Object.keys(overrides)) result[key] = overrides[key]
  return result
}

export function withAlpha(color: string, alpha: number): string {
  const byte: number = Math.round(
    parseInt(color.slice(1, 3), 16) * Math.max(0, Math.min(1, alpha))
  )
  return '#' + byte.toString(16).padStart(2, '0').toUpperCase() + color.slice(3)
}

export class DazzTokens {
  readonly name: string
  private colors: Record<string, string>
  private metrics: Record<string, number>
  private typography: Record<string, DazzTypography>
  private recipes: Record<string, DazzPaint>
  private shadows: Record<string, DazzShadow[]>
  private system?: DazzSystemStyle

  get systemStyle(): DazzSystemStyle | undefined {
    return this.system?.copy()
  }

  constructor(name: string = 'bright-light', systemStyle?: DazzSystemStyle) {
    if (THEME_NAMES.indexOf(name) < 0)
      throw new Error('Unknown DazzUI theme: ' + name)
    this.name = name
    const data = THEMES[name]
    this.colors = merge(BASE.colors, data.colors)
    this.metrics = merge(BASE.metrics, data.metrics)
    this.typography = merge(BASE.typography, data.typography)
    this.recipes = merge(BASE.recipes, data.recipes)
    this.shadows = merge(BASE.shadows, data.shadows)
    this.system = systemStyle?.copy()
    const style = this.system
    if (style) {
      this.typography = merge(this.typography, style.typography)
      for (const size of ['tiny', 'small', 'medium', 'large']) {
        this.typography['control.' + size + '.content'] = {
          size:
            size === 'tiny' || size === 'small'
              ? style.smallButtonFontSize
              : style.buttonFontSize,
          leading:
            size === 'tiny' || size === 'small'
              ? style.smallButtonFontSize
              : style.buttonFontSize,
          weight: style.buttonFontWeight,
        }
      }
      for (const id of Object.keys(this.metrics))
        this.metrics[id] = style.metric(id, this.metrics[id])
    }
  }

  color(role: string): string {
    const key = this.colors[role] !== undefined ? role : 'color.' + role
    if (this.colors[key] === undefined)
      throw new Error('Unknown DazzUI color: ' + role)
    return this.colors[key]
  }
  metric(id: string): number {
    if (this.metrics[id] === undefined)
      throw new Error('Unknown DazzUI metric: ' + id)
    return this.metrics[id]
  }
  type(id: string): DazzTypography {
    if (this.typography[id] === undefined)
      throw new Error('Unknown DazzUI type: ' + id)
    const type = this.typography[id]
    return { size: type.size, weight: type.weight, leading: type.leading }
  }
  paint(
    variant: DazzVariant,
    part: DazzPart,
    state: DazzState = 'normal',
    tint: DazzTint = 'primary'
  ): string {
    const recipe = this.recipes[variant + '.' + part + '.' + state]
    if (recipe === undefined) throw new Error('Unknown DazzUI recipe')
    return withAlpha(
      recipe.shade === 0 ? recipe.color : this.color(tint + '.' + recipe.shade),
      recipe.alpha
    )
  }
  focus(): string {
    return this.colors['focus.ring']
  }
  focusGlow(tint: DazzTint = 'primary'): string {
    return this.colors['focus.glow.' + tint]
  }
  scrim(): string {
    return this.colors['dialog.scrim']
  }
  shadow(id: string): DazzShadow[] {
    if (this.shadows[id] === undefined)
      throw new Error('Unknown DazzUI shadow: ' + id)
    return this.shadows[id].map((shadow: DazzShadow): DazzShadow => {
      return {
        x: shadow.x,
        y: shadow.y,
        radius: shadow.radius,
        color: shadow.color,
      }
    })
  }
  size(size: DazzSize, field: string = 'size'): number {
    return this.metric('control.' + size + '.' + field)
  }
  label(size: DazzSize): DazzTypography {
    return this.type('control.' + size + '.content')
  }
  /** Sizes are already resolved to vp. Never let ArkUI scale them a second time. */
  fontSize(value: number): string {
    return value + 'vp'
  }
  fieldSize(size: DazzSize): number {
    return this.system?.fieldHeight ?? this.size(size)
  }
  fieldLabel(size: DazzSize): DazzTypography {
    const style = this.system
    return style
      ? {
          size: style.fieldFontSize,
          leading: style.fieldLineHeight,
          weight: style.fieldFontWeight,
        }
      : this.label(size)
  }
  fieldLineHeight(): number {
    return this.system?.fieldLineHeight ?? this.type('bodySmall').leading
  }
  fieldWeight(): number {
    return this.system?.fieldFontWeight ?? this.type('bodySmall').weight
  }
  fieldPadding(): number {
    return this.system?.fieldPadding ?? this.metric('spacing.3')
  }
}
