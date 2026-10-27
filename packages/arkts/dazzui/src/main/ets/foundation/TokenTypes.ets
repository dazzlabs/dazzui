export type DazzTint =
  | 'primary'
  | 'neutral'
  | 'info'
  | 'success'
  | 'warning'
  | 'danger'
export type DazzVariant =
  | 'filled'
  | 'tinted'
  | 'outlined'
  | 'normal'
  | 'recessed'
  | 'plain'
export type DazzSize = 'tiny' | 'small' | 'medium' | 'large'
export type DazzState = 'normal' | 'hovered' | 'pressed' | 'disabled'
export type DazzPart = 'surface' | 'content' | 'border'

export interface DazzTypography {
  size: number
  weight: number
  leading: number
}
export interface DazzPaint {
  shade: number
  color: string
  alpha: number
}
export interface DazzShadow {
  x: number
  y: number
  // The ds/CSS blur length in logical pixels, not ShadowOptions.radius.
  radius: number
  color: string
}
export interface ThemeData {
  colors: Record<string, string>
  metrics: Record<string, number>
  typography: Record<string, DazzTypography>
  recipes: Record<string, DazzPaint>
  shadows: Record<string, DazzShadow[]>
}
export interface DazzItem {
  value: string
  label: string
  icon?: string
  count?: number
  hint?: string
  disabled?: boolean
}
