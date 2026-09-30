import { Progress as Base } from '@base-ui/react/progress'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type ProgressSize = 'small' | 'medium' | 'large'

export interface ProgressProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children' | 'value'
> {
  /** 0–100. Omit for an indeterminate bar. */
  value?: number
  size?: ProgressSize
  tint?: Tint
  /** Paint the fill with the theme's two-stop accent ramp instead of a flat tint. */
  gradient?: boolean
  className?: string
}

export function Progress({
  value,
  size = 'small',
  tint = 'primary',
  gradient = false,
  className,
  ...props
}: ProgressProps) {
  const clamped = value === undefined ? null : Math.min(100, Math.max(0, value))

  return (
    <Base.Root
      value={clamped}
      data-gradient={gradient || undefined}
      data-size={size}
      data-tint={tint}
      className={cx('dz-progress', className)}
      {...props}
    >
      <Base.Indicator className="dz-progress__fill" />
    </Base.Root>
  )
}
