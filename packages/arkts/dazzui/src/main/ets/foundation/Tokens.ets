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

  constructor(name: string = 'studio-light') {
    if (THEME_NAMES.indexOf(name) < 0)
      throw new Error('Unknown DazzUI theme: ' + name)
    this.name = name
    const data = THEMES[name]
    this.colors = merge(BASE.colors, data.colors)
    this.metrics = merge(BASE.metrics, data.metrics)
    this.typography = merge(BASE.typography, data.typography)
    this.recipes = merge(BASE.recipes, data.recipes)
    this.shadows = merge(BASE.shadows, data.shadows)
  }

  color(role: string): string {
    const key = role.startsWith('color.') ? role : 'color.' + role
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
}
