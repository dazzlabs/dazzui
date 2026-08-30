import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export interface SectionLabelProps extends ComponentPropsWithoutRef<'span'> {
  /** Leave unset for the labelling ink. */
  tint?: Tint
  /** One step quieter — a nav group's heading rather than a pane's. */
  faint?: boolean
}

export function SectionLabel({
  tint,
  faint = false,
  className,
  ...props
}: SectionLabelProps) {
  return (
    <span
      data-tint={tint}
      data-tone={faint ? 'faint' : undefined}
      className={cx('dz-section-label', className)}
      {...props}
    />
  )
}
