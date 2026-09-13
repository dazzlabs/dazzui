export type SceneValue =
  | string
  | number
  | boolean
  | { state: string }
  | { trim: SceneValue }
  | { concat: SceneValue[] }
  | { eq: SceneValue[] }
  | { add: SceneValue[] }
  | { multiply: SceneValue[] }
  | { gte: SceneValue[] }
  | { if: SceneValue; then: SceneValue; else: SceneValue }
export function valueType(
  value: unknown,
  state: Record<string, unknown>,
  depth?: number
): string
export function evaluateValue(
  value: SceneValue,
  state: Record<string, unknown>
): string | number | boolean

export const TRIM_CHARACTERS: string
