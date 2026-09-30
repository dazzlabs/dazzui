import { Separator } from '@base-ui/react/separator'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface DividerProps extends Omit<
  Separator.Props,
  'className' | 'render'
> {
  /** Starts the line at the list's text inset rather than the edge. */
  inset?: boolean
  className?: string
}

/** iOS's hairline separator; Material's 1dp divider in the outline variant. */
export function Divider({ inset = false, className, ...props }: DividerProps) {
  const platform = usePlatform()
  return (
    <Separator
      data-platform={platform}
      data-inset={inset || undefined}
      className={cx('dzm-divider', className)}
      {...props}
    />
  )
}
