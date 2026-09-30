import { Progress as Base } from '@base-ui/react/progress'
import type { CSSProperties } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface ProgressProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  /** A ring rather than a bar. */
  circular?: boolean
  /** The ring's diameter; a bar fills its row. */
  size?: 'small' | 'medium' | 'large'
  className?: string
}

/**
 * UIProgressView and Material Expressive's progress indicators. A `value`
 * of `null` is indeterminate: Material sweeps its bar and spins its ring;
 * iOS, whose progress view is always determinate, shows the same sweep.
 * Base UI carries the value and the `aria-*`.
 */
export function Progress({
  circular = false,
  size = 'medium',
  className,
  style,
  ...props
}: ProgressProps) {
  const platform = usePlatform()
  const { value, min = 0, max = 100 } = props
  const fraction =
    value === null || value === undefined
      ? 0
      : Math.min(1, Math.max(0, (value - min) / (max - min)))
  return (
    <Base.Root
      data-platform={platform}
      data-circular={circular || undefined}
      data-size={size}
      className={cx('dzm-progress', className)}
      style={{ ...style, '--dzm-progress-fraction': fraction } as CSSProperties}
      {...props}
    >
      {circular ? (
        <svg className="dzm-progress__ring" viewBox="0 0 48 48" aria-hidden>
          <circle className="dzm-progress__ring-track" cx="24" cy="24" r="21" />
          <circle className="dzm-progress__ring-fill" cx="24" cy="24" r="21" />
        </svg>
      ) : (
        <Base.Track className="dzm-progress__track">
          <Base.Indicator className="dzm-progress__fill" />
          <span className="dzm-progress__stop" aria-hidden />
        </Base.Track>
      )}
    </Base.Root>
  )
}
