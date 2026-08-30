import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export interface SectionLabelProps extends ComponentPropsWithoutRef<'span'> {
  /** Leave unset for the muted default. */
  tint?: Tint
}

export function SectionLabel({ tint, className, ...props }: SectionLabelProps) {
  return (
    <span
      data-tint={tint}
      className={cx('dz-section-label', className)}
      {...props}
    />
  )
}
