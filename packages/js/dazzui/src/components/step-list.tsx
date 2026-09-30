import { Checkmark12Filled } from '@fluentui/react-icons'
import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { Spinner } from './spinner'

export type StepStatus = 'done' | 'active' | 'idle'

export interface StepProps extends Omit<
  ComponentPropsWithoutRef<'li'>,
  'children'
> {
  status: StepStatus
  label: ReactNode
  /** Trailing detail — a count, a duration. */
  meta?: ReactNode
  tint?: Tint
}

/** One stage of a pipeline, carrying its own status. */
export function Step({
  status,
  label,
  meta,
  tint = 'primary',
  className,
  ...props
}: StepProps) {
  return (
    <li
      data-status={status}
      data-tint={tint}
      className={cx('dz-step', className)}
      {...props}
    >
      {status === 'active' ? (
        <Spinner size="medium" tint={tint} />
      ) : (
        <span aria-hidden className="dz-step__marker">
          {status === 'done' ? <Checkmark12Filled /> : null}
        </span>
      )}
      <span className="dz-step__label">{label}</span>
      {meta ? <span className="dz-step__meta">{meta}</span> : null}
    </li>
  )
}

export function StepList({
  className,
  ...props
}: ComponentPropsWithoutRef<'ol'>) {
  return <ol className={cx('dz-step-list', className)} {...props} />
}
