/**
 * The ramps a control can be tinted with — one per semantic role in
 * `tokens/global/color.json`.
 *
 * Interactive colors are stored as recipes (a shade off a ramp plus an alpha)
 * rather than finished colors, so a control does not know its own hue until it
 * is tinted. `styles/tint.css` binds the chosen ramp onto `--tint-*`, which is
 * what component CSS reads.
 */
export const TINTS = [
  'primary',
  'neutral',
  'info',
  'success',
  'warning',
  'danger',
] as const

export type Tint = (typeof TINTS)[number]
